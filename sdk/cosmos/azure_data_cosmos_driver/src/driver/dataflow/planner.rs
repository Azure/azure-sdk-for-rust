// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore unemitted checkpointed

//! Pipeline planner for Cosmos DB operations.
//!
//! The planner validates an operation's target against its resource type and
//! constructs the appropriate dataflow [`Pipeline`].
//!
//! For cross-partition queries, [`build_sequential_drain`] consumes a backend
//! [`QueryPlan`](super::query_plan::QueryPlan) and resolves the query's EPK
//! ranges against the current topology to produce a fan-out pipeline.

use std::sync::Arc;

use crate::{
    driver::dataflow::query_plan::DistinctType,
    models::{
        effective_partition_key::{normalized_epk_len, EffectivePartitionKey},
        CosmosOperation, FeedRange,
    },
    options::PlanOptions,
};

use super::{
    distinct_hash::Hash128,
    intersect_feed_ranges,
    query_plan::{QueryInfo, QueryPlan, SortOrder},
    query_response,
    snapshot::{OrderByRangeToken, ValueBoundary},
    streaming_ordered_merge, Distinct, DrainedLeaf, NonStreamingOrderedMerge, OperationPlan,
    PartitionRoutingRefresh, Pipeline, PipelineNode, PipelineNodeState, RangedToken, Request,
    RequestTarget, ResolvedRange, SequentialDrain, SkipTake, StreamingOrderedMerge,
    TopologyProvider, UnorderedMerge,
};

/// Builds a single-node [`Pipeline`] for a trivial operation.
///
/// Trivial operations are those that can be satisfied by a single request to
/// one partition (point reads, single-partition queries, metadata operations).
/// Use [`CosmosOperation::is_trivial`] to check eligibility before calling.
///
/// `operation` is shared with the resulting [`Request`] node via `Arc`; the
/// caller passes ownership in (cheap because the underlying allocation is
/// shared with any other nodes that need the same operation).
///
/// `resume` is an optional [`PipelineNodeState`] from a continuation token
/// that augments planning. Only `Request` and `Drained` shapes are accepted
/// for trivial operations; any other shape returns a `DataConversion` error.
///
/// # Panics (debug builds)
///
/// Debug-asserts that the operation is indeed trivial. In release builds,
/// returns an error if a non-trivial operation (e.g. a cross-partition query)
/// is passed.
pub(crate) fn build_trivial_pipeline(
    operation: Arc<CosmosOperation>,
    resume: Option<PipelineNodeState>,
) -> crate::error::Result<Pipeline> {
    debug_assert!(
        operation.is_trivial(),
        "build_trivial_pipeline called with non-trivial operation: {:?} targeting {:?}",
        operation.operation_type(),
        operation.target(),
    );

    let target = operation.target();

    let initial_continuation = match resume {
        None => None,
        Some(PipelineNodeState::Request {
            server_continuation,
        }) => server_continuation,
        Some(PipelineNodeState::Drained) => {
            return Ok(Pipeline::new(Box::new(DrainedLeaf)));
        }
        Some(other) => {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH)
                .with_message(format!(
                    "continuation token shape {} does not match a trivial operation",
                    snapshot_kind(&other)
                ))
                .build());
        }
    };

    // We should only have been called when is_trivial() is true, which guarantees that the target is either None (non-partitioned)
    // or it holds a specific, complete, logical partition key.

    let request_target = match target {
        None => RequestTarget::NonPartitioned,
        Some(f) => {
            if let Some(pk) = f.partition_key() {
                RequestTarget::LogicalPartitionKey(pk.clone())
            } else {
                return Err(crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_FEED_RANGE_REQUIRES_FANOUT_PIPELINE,
                    )
                    .with_message(
                        "FeedRange targeting requires a fan-out pipeline; \
                 use plan_operation for cross-partition queries",
                    )
                    .build());
            }
        }
    };

    let root = Request::new(operation, request_target, initial_continuation);
    Ok(Pipeline::new(Box::new(root)))
}

/// Wraps a built pipeline into an [`OperationPlan`], enforcing the maximum
/// fan-out on fresh plans.
///
/// Every planning branch funnels through here so the fan-out limit is enforced
/// uniformly regardless of the pipeline's shape. The check counts leaf request
/// nodes via [`Pipeline::fan_out_width`] — each parent node contributes its own
/// accounting, so this scales to any future pipeline shape.
///
/// The limit is enforced **only at initial plan time**. The check is skipped on
/// resume (`is_fresh == false`), because a resumed plan already passed it when
/// it was first created. It is also not a runtime cap: a partition that splits
/// mid-execution may push the effective fan-out above the limit, and the
/// operation keeps running rather than aborting.
///
/// Returns a [`CosmosStatus::CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED`](crate::error::CosmosStatus::CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED)
/// error when a fresh plan exceeds [`PlanOptions::max_fan_out`].
pub(crate) fn finalize_plan(
    pipeline: Pipeline,
    operation: Arc<CosmosOperation>,
    is_fresh: bool,
    plan_options: &PlanOptions,
) -> crate::error::Result<OperationPlan> {
    if is_fresh {
        let width = pipeline.fan_out_width();
        if width > plan_options.max_fan_out as usize {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED)
                .with_message(format!(
                    "operation fans out to {width} partitions, exceeding the maximum of {}; \
                     raise max_fan_out (via FeedOptions) to run a broader cross-partition query",
                    plan_options.max_fan_out
                ))
                .build());
        }
    }
    Ok(OperationPlan::new(
        pipeline,
        operation,
        plan_options.clone(),
        !is_fresh,
    ))
}

/// Builds a fan-out [`Pipeline`] from a backend query plan as a sequential drain.
///
/// Produces a [`SequentialDrain`] over one [`Request`] per resolved range.
/// Other cross-partition strategies (streaming `ORDER BY`, hybrid search,
/// read-many, etc.) will live as sibling functions.
///
/// `operation` is the underlying logical operation shared across every
/// resulting [`Request`] node via `Arc::clone`; per-partition differences
/// (e.g. partition-key-range targeting) are layered on at execution time via
/// [`OperationOverrides`](crate::pipeline::OperationOverrides) and the
/// per-node [`RequestTarget`], not by cloning the operation itself.
///
/// This function:
/// 1. Validates that the query plan contains no unsupported features (no
///    top/limit, no ordering, no hybrid search, no aggregates).
/// 2. Converts the plan's `queryRanges` to [`FeedRange`]s and resolves them
///    against the current partition topology.
/// 3. Creates a [`Request`] node per resolved range (per saved child range
///    on resume) and bundles them in a [`SequentialDrain`].
///
/// `resume` is an optional [`PipelineNodeState`] from a continuation token.
/// On resume, the `SequentialDrain { children }` list is the authoritative
/// remaining-work ledger: every still-pending range and its server
/// continuation. The planner intersects each saved range with the current
/// topology and emits one [`Request`] leaf per intersection (carrying the
/// saved server continuation, if any); saved ranges marked `Drained` emit
/// nothing. Topology gaps that fall outside every saved range have already
/// been drained and are not re-queried. If a non-`Drained` saved range can't
/// be fully covered by the current topology, the resume fails with a
/// continuation-token error rather than silently dropping work.
/// `resume` is an optional [`PipelineNodeState`] from a continuation token.
/// On resume, the `SequentialDrain { left_most_undrained_epk, active_tokens }`
/// pair describes the remaining work sparsely: anything strictly below the
/// cursor has already been drained; ranges at or above the cursor with no
/// matching `active_tokens` entry are implicitly fresh-start; ranges that
/// overlap an entry inherit that entry's server continuation (cloned across
/// every overlapping topology leaf, which transparently handles partition
/// splits since the saved snapshot was taken). If a non-empty `active_tokens`
/// entry can't be fully covered by the current topology above the cursor,
/// the resume fails with a continuation-token error rather than silently
/// dropping work.
pub(crate) async fn build_sequential_drain(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
    resume: Option<PipelineNodeState>,
) -> crate::error::Result<Pipeline> {
    build_sequential_drain_inner(query_plan, topology_provider, operation, resume).await
}

async fn build_sequential_drain_inner(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
    resume: Option<PipelineNodeState>,
) -> crate::error::Result<Pipeline> {
    validate_query_plan(query_plan)?;

    // Global OFFSET / LIMIT / TOP window derived from the query plan.
    let query_info = query_plan.query_info.as_ref();
    let mut skip = query_info.and_then(|info| info.offset).unwrap_or(0);
    let mut take = query_info.and_then(combine_take);
    // Whether the *query plan* itself carries a skip/take window, i.e. whether
    // the resumed pipeline will contain a `SkipTake` node. Used to validate a
    // resumed continuation's pipeline shape below.
    let plan_has_window = skip > 0 || take.is_some();

    // A `SkipTake` continuation wraps the fan-out snapshot; peel it so the saved
    // remaining window overrides the plan and the inner child drives the
    // fan-out resume below.
    let inner_resume = match resume {
        Some(PipelineNodeState::SkipTake {
            remaining_skip,
            remaining_take,
            child,
        }) => {
            // Resume validates the *pipeline* shape, not the query shape: `TOP n`
            // and `OFFSET x LIMIT y` build an identical global skip/take
            // pipeline, so a token minted by one legitimately resumes the other.
            // We only reject when the resumed query has no skip/take window at
            // all — then the pipeline has no `SkipTake` node to resume into.
            if !plan_has_window {
                return Err(crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH,
                    )
                    .with_message(
                        "continuation token carries a skip/take (OFFSET/LIMIT/TOP) window but \
                         the resumed query has no such window",
                    )
                    .build());
            }
            skip = remaining_skip;
            take = remaining_take;
            Some(*child)
        }
        other => other,
    };

    // `DISTINCT` sits *inside* the skip/take window (SQL applies OFFSET /
    // LIMIT / TOP to the deduplicated stream), so its token state nests one
    // level below `SkipTake`'s and is peeled second.
    let distinct_type = plan_distinct_type(query_plan);
    let (inner_resume, last_hash) = peel_distinct_resume(inner_resume, distinct_type)?;
    let resumed_drained = matches!(inner_resume, Some(PipelineNodeState::Drained));

    let needs_skip_take = skip > 0 || take.is_some();

    // Per-partition requests must use the plan's `rewrittenQuery` so
    // OFFSET / LIMIT / TOP are applied once, globally, by the `SkipTake` node
    // here rather than being re-applied inside every partition.
    let effective_operation = rewritten_operation(operation, query_plan)?;

    let saved_snapshot = match inner_resume {
        None => None,
        Some(PipelineNodeState::Drained) => {
            return Ok(Pipeline::new(Box::new(DrainedLeaf)));
        }
        Some(PipelineNodeState::SequentialDrain {
            left_most_undrained_epk,
            active_tokens,
        }) => Some(validate_saved_snapshot(
            left_most_undrained_epk,
            active_tokens,
        )?),
        Some(other) => {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH)
                .with_message(format!(
                    "continuation token shape {} does not match a cross-partition operation",
                    snapshot_kind(&other)
                ))
                .build());
        }
    };

    let request_nodes = if let Some(saved) = saved_snapshot.as_ref() {
        plan_resume_from_saved_snapshot(query_plan, topology_provider, &effective_operation, saved)
            .await?
    } else {
        plan_fresh(query_plan, topology_provider, &effective_operation).await?
    };

    // The max fan-out limit is enforced centrally in
    // `CosmosDriver::plan_operation` via `Pipeline::fan_out_width`, so it
    // applies uniformly to every pipeline shape and is not duplicated here.

    if request_nodes.is_empty() {
        // Resumed past every range that still has work: the pipeline is
        // fully drained. Otherwise the plan / topology yielded nothing to
        // query — that's a service contract violation.
        if saved_snapshot.is_some() {
            return Ok(Pipeline::new(Box::new(DrainedLeaf)));
        }
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES)
            .with_message("query plan produced no partition ranges to query")
            .build());
    }

    // Even when there's only one request node, we still need to wrap it in
    // a SequentialDrain so the pipeline can react to splits by replacing
    // the single Request with multiple Requests.
    let fanout: Box<dyn PipelineNode> = Box::new(SequentialDrain::new(request_nodes));

    // `DISTINCT` deduplicates the fan-out stream first; the global skip/take
    // window then counts deduplicated rows, matching SQL semantics.
    let deduped = apply_distinct(
        fanout,
        distinct_type,
        last_hash,
        resumed_drained,
        operation.emits_binary_payload(),
    );

    // Cross-partition OFFSET / LIMIT / TOP applies a global skip/take over the
    // fan-out's EPK-ordered stream. When none is present the fan-out is the
    // pipeline root directly.
    let root: Box<dyn PipelineNode> = if needs_skip_take {
        Box::new(SkipTake::new(
            deduped,
            skip,
            take,
            operation.emits_binary_payload(),
        ))
    } else {
        deduped
    };
    Ok(Pipeline::new(root))
}

/// `true` if `query_info` selects the streaming `ORDER BY` pipeline (one or
/// more `ORDER BY` columns not requiring the non-streaming buffered sort).
pub(crate) fn is_streaming_order_by(info: &QueryInfo) -> bool {
    !info.order_by.is_empty() && !info.has_non_streaming_order_by
}

/// `true` if `query_info` selects the fully buffered non-streaming ORDER BY pipeline.
pub(crate) fn is_non_streaming_order_by(info: &QueryInfo) -> bool {
    info.has_non_streaming_order_by
}

/// Builds a bounded, fully buffered merge for a finite non-streaming ORDER BY query.
pub(crate) async fn build_non_streaming_ordered_merge(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
    resume: Option<PipelineNodeState>,
) -> crate::error::Result<Pipeline> {
    if resume.is_some() {
        return Err(crate::error::CosmosError::builder()
            .with_status(
                crate::error::CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_CONTINUATION_UNSUPPORTED,
            )
            .with_message(
                "cross-partition non-streaming ORDER BY queries cannot be resumed from a continuation token",
            )
            .build());
    }

    if query_plan.hybrid_search_query_info.is_some() {
        return Err(unsupported_feature(
            "hybrid search combined with non-streaming ORDER BY",
        ));
    }

    let info = query_plan.query_info.as_ref().ok_or_else(|| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_UNSUPPORTED_QUERY_FEATURE)
            .with_message(
                "internal error: non-streaming ORDER BY path selected with no queryInfo present",
            )
            .build()
    })?;
    if !info.has_non_streaming_order_by {
        return Err(unsupported_feature(
            "non-streaming ORDER BY path selected for a streaming query",
        ));
    }
    if info.distinct_type != DistinctType::None {
        return Err(unsupported_feature(
            "DISTINCT combined with non-streaming ORDER BY",
        ));
    }
    if !info.aggregates.is_empty() {
        return Err(unsupported_feature(
            "aggregates combined with non-streaming ORDER BY",
        ));
    }
    if !info.group_by_expressions.is_empty() {
        return Err(unsupported_feature(
            "GROUP BY combined with non-streaming ORDER BY",
        ));
    }
    if info.order_by.is_empty() {
        return Err(unsupported_feature(
            "non-streaming ORDER BY requires at least one sort key",
        ));
    }
    if info
        .rewritten_query
        .as_deref()
        .is_none_or(|query| query.is_empty())
    {
        return Err(crate::error::CosmosError::builder()
            .with_status(
                crate::error::CosmosStatus::SERVICE_QUERY_PLAN_ORDER_BY_MISSING_REWRITTEN_QUERY,
            )
            .with_message(
                "query plan reported non-streaming ORDER BY but did not supply a non-empty rewrittenQuery",
            )
            .build());
    }

    let skip = info.offset.unwrap_or(0);
    let take = combine_take(info).ok_or_else(|| {
        crate::error::CosmosError::builder()
            .with_status(
                crate::error::CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_REQUIRES_FINITE_WINDOW,
            )
            .with_message(
                "cross-partition non-streaming ORDER BY requires a finite TOP or OFFSET/LIMIT window",
            )
            .build()
    })?;
    let retention_limit = skip.checked_add(take).ok_or_else(|| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_WINDOW_TOO_LARGE)
            .with_message("non-streaming ORDER BY OFFSET plus take overflows the supported window")
            .build()
    })?;
    let retention_limit = usize::try_from(retention_limit).map_err(|_| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_WINDOW_TOO_LARGE)
            .with_message("non-streaming ORDER BY candidate window does not fit in memory")
            .build()
    })?;
    let skip = usize::try_from(skip).map_err(|_| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_WINDOW_TOO_LARGE)
            .with_message("non-streaming ORDER BY OFFSET does not fit in memory")
            .build()
    })?;
    let take = usize::try_from(take).map_err(|_| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_WINDOW_TOO_LARGE)
            .with_message("non-streaming ORDER BY take does not fit in memory")
            .build()
    })?;

    let effective_operation = rewritten_operation(operation, query_plan)?;
    let request_nodes = plan_fresh(query_plan, topology_provider, &effective_operation).await?;
    if request_nodes.is_empty() {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES)
            .with_message("query plan produced no partition ranges to query")
            .build());
    }

    let fanout: Box<dyn PipelineNode> = Box::new(SequentialDrain::new(request_nodes));
    let root = Box::new(NonStreamingOrderedMerge::new(
        fanout,
        info.order_by.clone(),
        retention_limit,
        skip,
        take,
        operation.request_headers().max_item_count,
        operation.emits_binary_payload(),
    ));
    Ok(Pipeline::new(root))
}

/// Builds a [`streaming_ordered_merge::StreamingOrderedMerge`] pipeline
/// from a backend query plan whose `queryInfo.orderBy` is non-empty.
/// Mirrors [`build_sequential_drain`]'s shape, but snapshots every
/// still-active range explicitly since global ordering means any range
/// may still have unemitted rows.
///
/// `resume` re-resolves each saved range against current topology and
/// rebuilds it via [`streaming_ordered_merge::build_children`], the same
/// path a live split uses.
pub(crate) async fn build_streaming_ordered_merge(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
    resume: Option<PipelineNodeState>,
) -> crate::error::Result<Pipeline> {
    build_streaming_ordered_merge_inner(query_plan, topology_provider, operation, resume).await
}

async fn build_streaming_ordered_merge_inner(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
    resume: Option<PipelineNodeState>,
) -> crate::error::Result<Pipeline> {
    validate_query_plan_for_streaming_order_by(query_plan)?;
    let info = query_plan
        .query_info
        .as_ref()
        .expect("is_streaming_order_by requires query_info to be Some");
    let rewritten_query = info
        .rewritten_query
        .as_deref()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(
                    crate::error::CosmosStatus::SERVICE_QUERY_PLAN_ORDER_BY_MISSING_REWRITTEN_QUERY,
                )
                .with_message(
                    "query plan reported one or more ORDER BY columns but did not supply a \
                     non-empty rewrittenQuery",
                )
                .build()
        })?;
    let directions = info.order_by.clone();

    // Global OFFSET / LIMIT / TOP window (mirrors `build_sequential_drain`): an
    // ORDER BY query may also carry a skip/take, which is applied *globally* on
    // top of the ordered merge by a `SkipTake` root rather than per partition.
    let mut skip = info.offset.unwrap_or(0);
    let mut take = combine_take(info);
    let plan_has_window = skip > 0 || take.is_some();

    // A combined ORDER BY + OFFSET/LIMIT/TOP continuation nests the ordered-merge
    // snapshot inside a `SkipTake`; peel it so the saved remaining window
    // overrides the plan and the inner snapshot drives the ordered-merge resume
    // below. Mirrors the peel in `build_sequential_drain`.
    let resume = match resume {
        Some(PipelineNodeState::SkipTake {
            remaining_skip,
            remaining_take,
            child,
        }) => {
            // Resume validates pipeline shape, not query shape (see
            // `build_sequential_drain`): `TOP` and `OFFSET`/`LIMIT` build the
            // same skip/take node, so either token resumes the other. Only a
            // token whose query has lost its window entirely is rejected.
            if !plan_has_window {
                return Err(crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH,
                    )
                    .with_message(
                        "continuation token carries a skip/take (OFFSET/LIMIT/TOP) window but \
                         the resumed query has no such window",
                    )
                    .build());
            }
            skip = remaining_skip;
            take = remaining_take;
            Some(*child)
        }
        other => other,
    };

    // `DISTINCT` sits *inside* the skip/take window (SQL applies OFFSET /
    // LIMIT / TOP to the deduplicated stream), so its token state nests one
    // level below `SkipTake`'s and is peeled second.
    let distinct_type = plan_distinct_type(query_plan);
    let (resume, last_hash) = peel_distinct_resume(resume, distinct_type)?;
    let resumed_drained = matches!(resume, Some(PipelineNodeState::Drained));

    let query_from_beginning = query_response::rewritten_query_from_beginning(rewritten_query)?;
    let plain_body = query_response::rewrite_query_body(operation.body(), &query_from_beginning)?;
    let plain_operation = Arc::new((**operation).clone().with_body(plain_body));

    let is_resume = resume.is_some();
    // The feed scope is folded into the fingerprint (see
    // `streaming_ordered_merge::query_fingerprint`) because nothing else binds
    // a token to it: a resumed node treats its saved ranges as authoritative,
    // and `is_valid_for_operation` checks only the operation kind and RID.
    let scope_range = operation.target();
    let query_fingerprint =
        streaming_ordered_merge::query_fingerprint(operation.body(), scope_range);
    let saved_ranges = match resume {
        None => None,
        Some(PipelineNodeState::Drained) => {
            return Ok(Pipeline::new(Box::new(DrainedLeaf)));
        }
        Some(PipelineNodeState::StreamingOrderedMerge {
            directions: saved_directions,
            query_fingerprint: saved_fingerprint,
            ranges,
        }) => Some(validate_streaming_order_by_snapshot(
            &directions,
            &saved_directions,
            &query_fingerprint,
            saved_fingerprint.as_deref(),
            ranges,
        )?),
        Some(other) => {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH)
                .with_message(format!(
                    "continuation token shape {} does not match a streaming ORDER BY operation",
                    snapshot_kind(&other)
                ))
                .build());
        }
    };

    let mut children = Vec::new();

    if let Some(saved_ranges) = saved_ranges {
        for saved in saved_ranges {
            // A matching fingerprint already proves the scope is unchanged, so
            // this only fires for a token that carries no fingerprint. Reject
            // rather than clip: `build_children` decides whether a saved server
            // continuation is safe to replay by comparing the resolved topology
            // against this range, so narrowing it would make a stale pre-split
            // continuation look replayable instead of taking the rebuild path.
            if let Some(scope) = scope_range {
                if !saved.range.is_subset_of(scope) {
                    return Err(crate::error::CosmosError::builder()
                        .with_status(
                            crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID,
                        )
                        .with_message(format!(
                            "continuation token covers {}-{}, which is not contained in the requested feed scope {}-{}",
                            saved.range.min_inclusive().to_hex(),
                            saved.range.max_exclusive().to_hex(),
                            scope.min_inclusive().to_hex(),
                            scope.max_exclusive().to_hex(),
                        ))
                        .build());
                }
            }
            let resolved = topology_provider
                .resolve_ranges(&saved.range, PartitionRoutingRefresh::UseCached)
                .await?;
            let mut range_children = streaming_ordered_merge::build_children(
                &resolved,
                &saved.range,
                &plain_operation,
                &directions,
                saved.server_continuation,
                saved.boundary.as_ref(),
            )?;
            children.append(&mut range_children);
        }
    } else {
        // See `plan_fresh` for rationale on intersecting with the operation scope.
        let normalized_len = operation
            .container()
            .and_then(|c| normalized_epk_len(c.partition_key_definition()));
        for query_range in &query_plan.query_ranges {
            let plan_range = query_range_to_feed_range(query_range, normalized_len)?;
            let feed_range = match scope_range {
                Some(scope) => match intersect_feed_ranges(scope, &plan_range) {
                    Some(r) => r,
                    None => continue,
                },
                None => plan_range,
            };
            let resolved = topology_provider
                .resolve_ranges(&feed_range, PartitionRoutingRefresh::UseCached)
                .await?;
            let mut range_children = streaming_ordered_merge::build_children(
                &resolved,
                &feed_range,
                &plain_operation,
                &directions,
                None,
                None,
            )?;
            children.append(&mut range_children);
        }
    }

    if children.is_empty() {
        if is_resume {
            return Ok(Pipeline::new(Box::new(DrainedLeaf)));
        }
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES)
            .with_message("query plan produced no partition ranges to query")
            .build());
    }

    let emit_binary = plain_operation.emits_binary_payload();
    let ordered_root: Box<dyn PipelineNode> = Box::new(StreamingOrderedMerge::new(
        plain_operation,
        directions,
        children,
        query_fingerprint,
    ));

    // `DISTINCT` deduplicates the ordered stream first; the global skip/take
    // window then counts deduplicated rows, matching SQL semantics.
    let deduped = apply_distinct(
        ordered_root,
        distinct_type,
        last_hash,
        resumed_drained,
        emit_binary,
    );

    // Apply the global OFFSET / LIMIT / TOP window over the ordered stream. When
    // the query carries none, the ordered merge is the pipeline root directly.
    let needs_skip_take = skip > 0 || take.is_some();
    let root: Box<dyn PipelineNode> = if needs_skip_take {
        Box::new(SkipTake::new(deduped, skip, take, emit_binary))
    } else {
        deduped
    };
    Ok(Pipeline::new(root))
}

/// A saved [`OrderByRangeToken`], parsed and validated into planner-ready
/// types.
struct ParsedOrderByRange {
    range: FeedRange,
    server_continuation: Option<String>,
    boundary: Option<ValueBoundary>,
}

/// Validates a resumed `StreamingOrderedMerge` continuation's `ORDER BY`
/// direction and query-fingerprint discriminators against the current query,
/// then validates and parses every saved range: well-formed, sorted,
/// non-overlapping bounds, and a boundary whose resume-value count matches the
/// columns and whose RID is a decodable document RID.
fn validate_streaming_order_by_snapshot(
    directions: &[SortOrder],
    saved_directions: &[SortOrder],
    query_fingerprint: &str,
    saved_fingerprint: Option<&str>,
    ranges: Vec<OrderByRangeToken>,
) -> crate::error::Result<Vec<ParsedOrderByRange>> {
    if saved_directions != directions {
        return Err(order_by_state_invalid(format!(
            "continuation token has ORDER BY direction(s) {saved_directions:?} but the current \
             query has {directions:?}"
        )));
    }
    // Tokens minted before the fingerprint existed carry `None` and can only
    // be checked on `directions`.
    if let Some(saved_fingerprint) = saved_fingerprint {
        if saved_fingerprint != query_fingerprint {
            return Err(order_by_state_invalid(
                "continuation token was produced by a different query (query text, parameters, or \
                 feed scope changed); a streaming ORDER BY token can only resume the query and \
                 scope that minted it",
            ));
        }
    }
    if ranges.is_empty() {
        return Err(order_by_state_invalid(
            "continuation token has an empty StreamingOrderedMerge range list; a fully-drained \
             operation must use the Drained shape instead",
        ));
    }

    let mut parsed = Vec::with_capacity(ranges.len());
    let mut prev_max: Option<EffectivePartitionKey> = None;
    for entry in ranges {
        let min = EffectivePartitionKey::from(entry.min_epk);
        let max = EffectivePartitionKey::from(entry.max_epk);
        if min >= max {
            return Err(order_by_state_invalid(format!(
                "continuation token has an invalid range (min `{}` >= max `{}`)",
                min.to_hex(),
                max.to_hex(),
            )));
        }
        if let Some(prev) = &prev_max {
            if &min < prev {
                return Err(order_by_state_invalid(
                    "continuation token ranges must be sorted ascending and non-overlapping",
                ));
            }
        }
        prev_max = Some(max.clone());

        if let Some(boundary) = &entry.boundary {
            if boundary.resume_values.len() != directions.len() {
                return Err(order_by_state_invalid(format!(
                    "continuation token range boundary has {} resume value(s) but the query has \
                     {} ORDER BY column(s)",
                    boundary.resume_values.len(),
                    directions.len(),
                )));
            }
            // A non-empty RID isn't enough, and neither is a decodable one:
            // the boundary RID is compared against real backend `_rid`s by
            // `compare_document_rids`, so it must be a *document* RID. A
            // sibling 16-byte RID (partition key range, stored procedure, ...)
            // would yield an arbitrary ordinal, and an undecodable one would
            // silently degrade to raw-string ordering, which is not monotonic
            // in document ordinal — either way the discard pass would drop or
            // keep the wrong rows inside the boundary tie group. Reject it
            // here, as .NET/Java do when `ResourceId.TryParse` fails.
            if crate::models::resource_id::document_ordinal(&boundary.last_rid).is_none() {
                return Err(order_by_state_invalid(format!(
                    "continuation token range boundary RID `{}` is not a decodable Cosmos \
                     document RID",
                    boundary.last_rid,
                )));
            }
            // Rust's versioned client-token boundary counts at least its own
            // boundary row, so `skip_count` is always >= 1. This is not a
            // restriction on the .NET-compatible resumeFilter wire contract.
            // An explicit 0 is corrupt (a legacy token that omits the field is
            // read back as 1 by serde default, not 0).
            if boundary.skip_count == 0 {
                return Err(order_by_state_invalid(
                    "continuation token range boundary has a skip count of 0 (must be >= 1)",
                ));
            }
        }

        parsed.push(ParsedOrderByRange {
            range: FeedRange::new(min, max)?,
            server_continuation: entry.server_continuation,
            boundary: entry.boundary,
        });
    }

    Ok(parsed)
}

fn order_by_state_invalid(
    message: impl Into<std::borrow::Cow<'static, str>>,
) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID)
        .with_message(message)
        .build()
}

/// Builds an [`UnorderedMerge`] pipeline for change feed operations.
///
/// Unlike [`build_sequential_drain`], this does not require a query plan.
/// The operation's target [`FeedRange`] is resolved against the current
/// partition topology to produce one [`Request`] leaf per physical
/// partition. All leaves are wrapped in an [`UnorderedMerge`] that polls
/// them round-robin without evicting children on 304.
///
/// `resume` is an optional [`PipelineNodeState`] from a continuation token.
/// On resume, `UnorderedMerge { active_tokens, start_from }` carries per-
/// EPK-range server continuations plus the feed's original start position.
/// Each physical range is rebuilt by sweeping the saved tokens that overlap it
/// left to right: every saved sub-range becomes its own EPK-scoped leaf
/// resuming from that sub-range's continuation, and any slice with no saved
/// token re-applies `start_from`. A split therefore fans one parent token out
/// to its children, while a merge reads each saved sub-range independently
/// without dropping a continuation — matching the per-EPK-range change feed
/// resume used by the other Cosmos SDKs (.NET, Java, Python).
pub(crate) async fn build_unordered_merge(
    feed_range: &FeedRange,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
    resume: Option<PipelineNodeState>,
) -> crate::error::Result<Pipeline> {
    let (saved_tokens, resume_start) = match resume {
        None => (None, None),
        Some(PipelineNodeState::Drained) => {
            return Ok(Pipeline::new(Box::new(DrainedLeaf)));
        }
        Some(PipelineNodeState::UnorderedMerge {
            active_tokens,
            start_from,
        }) => (
            Some(validate_unordered_merge_tokens(active_tokens)?),
            start_from,
        ),
        Some(other) => {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH)
                .with_message(format!(
                    "continuation token shape {} does not match a change feed operation",
                    snapshot_kind(&other)
                ))
                .build());
        }
    };

    // The start marker is carried so every checkpoint re-persists it. On a
    // fresh start it comes from the operation; on resume the token's persisted
    // marker wins, because the caller only hands back the token and does not
    // repeat the original start position.
    let is_resume = saved_tokens.is_some();
    let start_marker = if is_resume {
        resume_start
    } else {
        operation.change_feed_start().cloned()
    };

    // Full-fidelity (AllVersionsAndDeletes) feeds must pin every range to a
    // concrete starting continuation before the first checkpoint, so a range
    // that is never polled can't resume from a stale `Now` and drop the
    // versions/deletes in the gap. Priming is needed whenever no range has yet
    // recorded a continuation: on a fresh start, and also on resume from a
    // checkpoint taken *before* the first page was pulled (an empty token set).
    // A fully drained resume arrives as `PipelineNodeState::Drained` and is
    // handled earlier, so an empty `UnorderedMerge` token set here only ever
    // means "nothing has been polled yet". A resume that already carries saved
    // continuations must NOT prime: those ranges would re-poll from their real
    // ETags and the discarded primed page would lose real data.
    let prime_on_first_drain = operation.request_headers().full_fidelity_feed
        && saved_tokens.as_ref().is_none_or(|tokens| tokens.is_empty());

    // On resume the operation rebuilt by the SDK no longer carries the original
    // start headers (the caller only passed the continuation token). Re-derive
    // them from the persisted marker so partitions with no saved continuation
    // (never polled before the checkpoint) honor the original start position
    // instead of silently reading from the beginning. Partitions that do have a
    // saved continuation still take precedence via their `If-None-Match` ETag.
    let operation: Arc<CosmosOperation> = match (is_resume, &start_marker) {
        (true, Some(marker)) => {
            Arc::new((**operation).clone().with_change_feed_start(marker.clone()))
        }
        _ => Arc::clone(operation),
    };

    let resolved = topology_provider
        .resolve_ranges(feed_range, PartitionRoutingRefresh::UseCached)
        .await?;

    let mut request_nodes: Vec<Box<dyn PipelineNode>> = Vec::new();

    for resolved_range in resolved {
        let range = intersect_feed_ranges(&resolved_range.range, feed_range)
            .expect("topology provider must return ranges that overlap the feed range");

        // Rebuild this physical range's leaves by sweeping the saved tokens
        // that overlap it, left to right. Each saved sub-range resumes from its
        // own `server_continuation`; any slice with no saved token (a
        // never-polled sub-range, or a brand-new range) emits a fresh-start
        // leaf that re-applies `start_from`.
        //
        // A split appears here as one saved token spanning several physical
        // children: each child is fully covered, so it yields a single leaf
        // carrying the parent continuation (the server accepts a parent token
        // against a post-split child). A merge appears as several saved tokens
        // inside one physical range: each saved sub-range is read independently
        // from its own continuation, EPK-scoped via `x-ms-start/end-epk`, so no
        // saved continuation is dropped. This mirrors the per-EPK-range change
        // feed resume used by the other Cosmos SDKs (.NET, Java, Python).
        let mut cursor = range.min_inclusive().clone();
        let range_max = range.max_exclusive().clone();

        if let Some(tokens) = saved_tokens.as_ref() {
            // `saved_tokens` is sorted ascending and non-overlapping, so the
            // overlapping slices are produced in order with no backtracking.
            for token in tokens {
                let Some(slice) = intersect_feed_ranges(&token.range, &range) else {
                    continue;
                };
                if &cursor < slice.min_inclusive() {
                    let gap = FeedRange::new(cursor.clone(), slice.min_inclusive().clone())?;
                    push_change_feed_leaf(
                        &mut request_nodes,
                        &operation,
                        gap,
                        &resolved_range,
                        None,
                    );
                }
                cursor = slice.max_exclusive().clone();
                push_change_feed_leaf(
                    &mut request_nodes,
                    &operation,
                    slice,
                    &resolved_range,
                    Some(token.server_continuation.clone()),
                );
            }
        }

        if cursor < range_max {
            // Trailing slice with no saved continuation, or the whole range on
            // a fresh (non-resumed) start.
            let tail = FeedRange::new(cursor, range_max)?;
            push_change_feed_leaf(&mut request_nodes, &operation, tail, &resolved_range, None);
        }
    }

    if request_nodes.is_empty() {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES)
            .with_message("change feed produced no partition ranges to query")
            .build());
    }

    let root = Box::new(
        UnorderedMerge::new(request_nodes)
            .with_start_marker(start_marker)
            .with_prime_on_first_drain(prime_on_first_drain),
    );
    Ok(Pipeline::new(root))
}

/// Pushes one change feed [`Request`] leaf scoped to `leaf_range` within the
/// given physical partition, optionally resuming from `continuation`.
///
/// When `leaf_range` covers the whole physical partition the EPK scoping
/// collapses away (`x-ms-start/end-epk` are omitted); a narrower slice — as
/// produced after a merge — carries explicit EPK bounds.
fn push_change_feed_leaf(
    request_nodes: &mut Vec<Box<dyn PipelineNode>>,
    operation: &Arc<CosmosOperation>,
    leaf_range: FeedRange,
    resolved_range: &ResolvedRange,
    continuation: Option<String>,
) {
    let target = RequestTarget::effective_partition_key_range(
        leaf_range,
        resolved_range.partition_key_range_id.clone(),
        resolved_range.range.clone(),
    );
    request_nodes.push(Box::new(Request::new(
        Arc::clone(operation),
        target,
        continuation,
    )));
}

/// Builds the request leaves for a fresh (non-resumed) cross-partition plan.
async fn plan_fresh(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
) -> crate::error::Result<Vec<Box<dyn PipelineNode>>> {
    let mut nodes: Vec<Box<dyn PipelineNode>> = Vec::new();
    // Clip each server-supplied query range to the operation scope (e.g.
    // `FeedScope::partition(partial_hpk)`), which bounds the partition-key
    // prefix. The `query_ranges` always cover the full container, so we
    // intersect to keep the fan-out (and per-pkrange wire EPK bounds) scoped.
    //
    // An equality / `IN` predicate yields a point plan range `[X, X]`, which
    // `query_range_to_feed_range` normalizes to the half-open window
    // `[X, successor(X))` so it routes like any other range (#4574 / #4638).
    let scope_range = operation.target();
    // Full EPK width for this container, used to zero-extend a closed range's
    // (or point's) inclusive upper bound to full width before making it
    // exclusive (#4574).
    let normalized_len = operation
        .container()
        .and_then(|c| normalized_epk_len(c.partition_key_definition()));
    for query_range in &query_plan.query_ranges {
        let plan_range = query_range_to_feed_range(query_range, normalized_len)?;
        let feed_range = match scope_range {
            Some(scope) => match intersect_feed_ranges(scope, &plan_range) {
                Some(r) => r,
                None => continue,
            },
            None => plan_range,
        };
        let resolved = topology_provider
            .resolve_ranges(&feed_range, PartitionRoutingRefresh::UseCached)
            .await?;
        for resolved_range in resolved {
            // Clip the resolved partition to the query range (for an equality
            // point this is the narrow `[X, successor(X))` window, emitted as a
            // `start`/`end-epk` pair alongside `partitionkeyrangeid`).
            let range =
                intersect_feed_ranges(&resolved_range.range, &feed_range).ok_or_else(|| {
                    topology_range_not_overlapping_error(&resolved_range.range, &feed_range)
                })?;

            let target = RequestTarget::effective_partition_key_range(
                range,
                resolved_range.partition_key_range_id,
                resolved_range.range,
            );
            nodes.push(Box::new(Request::new(Arc::clone(operation), target, None)));
        }
    }
    Ok(nodes)
}

/// Builds the request leaves for a resumed cross-partition plan, using the
/// sparse saved snapshot as the authoritative remaining-work ledger.
///
/// Iterates the current topology above the cursor. For each leaf, walks
/// through `active_tokens` overlapping that leaf and emits one [`Request`]
/// per intersection carrying the saved token; gaps between (or around)
/// overlapping tokens within the leaf emit fresh-start [`Request`]s. Each
/// `active_tokens` entry's coverage is tracked so any entry that can't be
/// fully covered by the current topology above the cursor is reported as a
/// continuation-token error.
///
/// # Cosmos server continuation semantics
///
/// When an `active_tokens` entry's range straddles multiple post-split
/// resolved leaves, this function forwards the *same* server continuation
/// token to every intersecting sub-leaf. This relies on the Cosmos backend's
/// documented behavior that a continuation token issued for a parent
/// partition remains valid against each of that partition's post-split
/// children — the server uses the EPK range carried alongside the request
/// to scope which child the token applies to.
async fn plan_resume_from_saved_snapshot(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
    saved: &SavedSnapshot,
) -> crate::error::Result<Vec<Box<dyn PipelineNode>>> {
    let mut nodes: Vec<Box<dyn PipelineNode>> = Vec::new();
    let mut coverage: Vec<Vec<FeedRange>> = vec![Vec::new(); saved.active_tokens.len()];
    // See `plan_fresh` for the scope-clip rationale. Equality / `IN` points are
    // normalized to `[X, successor(X))` windows by `query_range_to_feed_range`,
    // so they resume through the same half-open path as any other range.
    let scope_range = operation.target();
    // Full EPK width for this container (see `plan_fresh`).
    let normalized_len = operation
        .container()
        .and_then(|c| normalized_epk_len(c.partition_key_definition()));

    for query_range in &query_plan.query_ranges {
        let plan_range = query_range_to_feed_range(query_range, normalized_len)?;
        let feed_range = match scope_range {
            Some(scope) => match intersect_feed_ranges(scope, &plan_range) {
                Some(r) => r,
                None => continue,
            },
            None => plan_range,
        };
        let resolved = topology_provider
            .resolve_ranges(&feed_range, PartitionRoutingRefresh::UseCached)
            .await?;

        for resolved_range in resolved {
            // Clip the resolved partition to the query range.
            let leaf_scope =
                intersect_feed_ranges(&resolved_range.range, &feed_range).ok_or_else(|| {
                    topology_range_not_overlapping_error(&resolved_range.range, &feed_range)
                })?;

            // Clip to "at or above cursor". Drop leaves entirely below.
            if leaf_scope.max_exclusive() <= &saved.cursor {
                continue;
            }
            let effective_min = if leaf_scope.min_inclusive() < &saved.cursor {
                saved.cursor.clone()
            } else {
                leaf_scope.min_inclusive().clone()
            };
            let effective_leaf = FeedRange::new(effective_min, leaf_scope.max_exclusive().clone())?;

            // Walk active_tokens left-to-right against this leaf, emitting
            // a continued sub-leaf per intersection plus fresh-start
            // sub-leaves for any gaps.
            let mut cursor_within_leaf = effective_leaf.min_inclusive().clone();
            for (idx, entry) in saved.active_tokens.iter().enumerate() {
                if entry.range.max_exclusive() <= &cursor_within_leaf {
                    continue;
                }
                if entry.range.min_inclusive() >= effective_leaf.max_exclusive() {
                    break;
                }

                let overlap_min = if entry.range.min_inclusive() > &cursor_within_leaf {
                    entry.range.min_inclusive().clone()
                } else {
                    cursor_within_leaf.clone()
                };
                let overlap_max = if entry.range.max_exclusive() < effective_leaf.max_exclusive() {
                    entry.range.max_exclusive().clone()
                } else {
                    effective_leaf.max_exclusive().clone()
                };

                if overlap_min > cursor_within_leaf {
                    // Gap before this token entry — fresh-start sub-leaf.
                    let gap = FeedRange::new(cursor_within_leaf.clone(), overlap_min.clone())?;
                    let target = RequestTarget::effective_partition_key_range(
                        gap,
                        resolved_range.partition_key_range_id.clone(),
                        resolved_range.range.clone(),
                    );
                    nodes.push(Box::new(Request::new(Arc::clone(operation), target, None)));
                }

                let intersection = FeedRange::new(overlap_min, overlap_max.clone())?;
                coverage[idx].push(intersection.clone());
                let target = RequestTarget::effective_partition_key_range(
                    intersection,
                    resolved_range.partition_key_range_id.clone(),
                    resolved_range.range.clone(),
                );
                nodes.push(Box::new(Request::new(
                    Arc::clone(operation),
                    target,
                    Some(entry.server_continuation.clone()),
                )));

                cursor_within_leaf = overlap_max;
            }

            if cursor_within_leaf < *effective_leaf.max_exclusive() {
                // Trailing gap after the last overlapping token entry.
                let gap =
                    FeedRange::new(cursor_within_leaf, effective_leaf.max_exclusive().clone())?;
                let target = RequestTarget::effective_partition_key_range(
                    gap,
                    resolved_range.partition_key_range_id.clone(),
                    resolved_range.range.clone(),
                );
                nodes.push(Box::new(Request::new(Arc::clone(operation), target, None)));
            }
        }
    }

    // Verify every active token's range was fully covered by the current
    // topology above the cursor. If not, the planner cannot honor the
    // saved continuation without risking duplicate emission or data loss —
    // fail loudly.
    for (idx, entry) in saved.active_tokens.iter().enumerate() {
        if !range_fully_covered(&entry.range, &coverage[idx]) {
            const MAX_COVERAGE_PIECES_RENDERED: usize = 8;
            let coverage_summary = if coverage[idx].is_empty() {
                "(no overlapping topology ranges)".to_string()
            } else {
                let mut sorted = coverage[idx].clone();
                sorted.sort_by(|a, b| a.min_inclusive().cmp(b.min_inclusive()));
                let total = sorted.len();
                let rendered: Vec<String> = sorted
                    .iter()
                    .take(MAX_COVERAGE_PIECES_RENDERED)
                    .map(|r| {
                        format!(
                            "[{}, {})",
                            r.min_inclusive().to_hex(),
                            r.max_exclusive().to_hex()
                        )
                    })
                    .collect();
                if total > MAX_COVERAGE_PIECES_RENDERED {
                    format!("{} + ... ({} total ranges)", rendered.join(" + "), total)
                } else {
                    rendered.join(" + ")
                }
            };
            return Err(crate::error::CosmosError::builder()
                .with_status(
                    crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED,
                )
                .with_message(format!(
                    "continuation token active range [{}, {}) could not be fully covered \
                     by the current topology above the cursor (covered: {}); the query \
                     cannot be safely resumed",
                    entry.range.min_inclusive().to_hex(),
                    entry.range.max_exclusive().to_hex(),
                    coverage_summary,
                ))
                .build());
        }
    }

    Ok(nodes)
}

/// Converts a query-plan EPK range to a [`FeedRange`].
///
/// The gateway returns a *closed* range `[a, b]`
/// (`isMinInclusive == isMaxInclusive == true`) when a query filters on the
/// partition key with an equality / `IN` predicate (issue #4574) — a *point*
/// `[X, X]` per value — and a normal half-open `[min, max)` range otherwise.
///
/// Any closed upper bound is made exclusive by advancing it to its successor,
/// so a point `[X, X]` becomes the non-empty half-open range `[X, successor(X))`
/// and a closed range `[a, b]` becomes `[a, successor(b))`. Both then flow
/// through the normal `[min, max)` routing (scope intersection, topology
/// resolution, per-partition EPK-window emit) instead of collapsing to the empty
/// set (which panicked, #4574) or being special-cased to whole-partition
/// routing. Emitting the narrow `[X, successor(X))` window as a `start`/`end-epk`
/// pair alongside `partitionkeyrangeid` (with
/// `x-ms-read-key-type: EffectivePartitionKeyRange`, #4729) is honored by the
/// gateway and matches the normalize-to-EPK-range model the .NET SDK uses.
///
/// `normalized_len` is the container's full EPK width in bytes (see
/// [`normalized_epk_len`](crate::models::effective_partition_key::normalized_epk_len)).
/// It is applied **only to a point** (`min == max`, an equality / `IN` value,
/// which is always a *full* partition key): the trailing-zero-trimmed value is
/// zero-extended to full width before the increment, so the successor is the
/// exact one the backend expects (matching .NET's full-width HPK normalization).
/// When the width is unknown (V1), a point falls back to the width-preserving
/// [`successor`](EffectivePartitionKey::successor) so it still becomes a
/// non-empty window instead of collapsing to the empty set (the #4574 panic).
///
/// Every *other* range — including a closed **non-point** range such as an HPK
/// **prefix** upper bound — is passed through unchanged, exactly as upstream
/// does. Those bounds are produced at partition-boundary granularity by the
/// gateway/topology layer; advancing them with a successor over-extends the band
/// and routes incorrectly (it drops owning physical partitions — the
/// `hpk_tenant_prefix_where_full_scope` regression).
fn query_range_to_feed_range(
    query_range: &super::query_plan::QueryRange,
    normalized_len: Option<usize>,
) -> crate::error::Result<FeedRange> {
    let min = EffectivePartitionKey::from(query_range.min.as_str());
    let max = EffectivePartitionKey::from(query_range.max.as_str());
    // Only a closed *point* `[X, X]` (equality / `IN`, min == max) is
    // transformed — into the non-empty half-open window `[X, successor(X))`.
    // Every other range is left as-is (upstream behavior).
    let max = if query_range.is_max_inclusive && min == max {
        match normalized_len {
            // Full key: normalize to full EPK width before incrementing
            // (Option B, #4574 / #4638), matching .NET's HPK normalization.
            Some(len) => max.normalized_successor(len),
            // Unknown width (V1): width-preserving successor keeps `[X, X]` from
            // collapsing to the empty set.
            None => max.successor(),
        }
    } else {
        max
    };
    FeedRange::new(min, max)
}

/// Returns true if the union of `pieces` covers `range` end-to-end.
///
/// Assumes pieces are subsets of `range`. The check sorts pieces by
/// `min_inclusive` and walks left-to-right, requiring the running cursor to
/// reach `range.max_exclusive` with no gaps.
fn range_fully_covered(range: &FeedRange, pieces: &[FeedRange]) -> bool {
    if pieces.is_empty() {
        return false;
    }
    let mut sorted: Vec<&FeedRange> = pieces.iter().collect();
    sorted.sort_by(|a, b| a.min_inclusive().cmp(b.min_inclusive()));
    let mut cursor = range.min_inclusive().clone();
    for piece in sorted {
        debug_assert!(
            piece.min_inclusive() >= range.min_inclusive()
                && piece.max_exclusive() <= range.max_exclusive(),
            "range_fully_covered piece [{}, {}) is not a subset of range [{}, {})",
            piece.min_inclusive().to_hex(),
            piece.max_exclusive().to_hex(),
            range.min_inclusive().to_hex(),
            range.max_exclusive().to_hex(),
        );
        if piece.min_inclusive() > &cursor {
            return false;
        }
        if piece.max_exclusive() > &cursor {
            cursor = piece.max_exclusive().clone();
        }
    }
    &cursor >= range.max_exclusive()
}

/// Validated saved snapshot: cursor + per-range active tokens parsed into
/// strongly-typed [`EffectivePartitionKey`] / [`FeedRange`].
#[derive(Debug)]
struct SavedSnapshot {
    cursor: EffectivePartitionKey,
    active_tokens: Vec<SavedActiveToken>,
}

#[derive(Debug)]
struct SavedActiveToken {
    range: FeedRange,
    server_continuation: String,
}

/// Validates a sparse saved snapshot from a continuation token: each
/// `active_tokens` entry has `min < max` (and is not zero-width), the list
/// is strictly sorted ascending and non-overlapping, and the cursor is at
/// or before the first entry's `min`. Returns the parsed [`SavedSnapshot`]
/// on success or a continuation-token shape error on failure.
fn validate_saved_snapshot(
    left_most_undrained_epk: String,
    active_tokens: Vec<RangedToken>,
) -> crate::error::Result<SavedSnapshot> {
    let cursor = EffectivePartitionKey::from(left_most_undrained_epk);

    let mut parsed: Vec<SavedActiveToken> = Vec::with_capacity(active_tokens.len());
    for entry in active_tokens {
        let min = EffectivePartitionKey::from(entry.min_epk);
        let max = EffectivePartitionKey::from(entry.max_epk);
        if min > max {
            return Err(crate::error::CosmosError::builder()
                .with_status(
                    crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE,
                )
                .with_message(format!(
                    "continuation token has invalid active_tokens entry (min `{}` > max `{}`)",
                    min.to_hex(),
                    max.to_hex(),
                ))
                .build());
        }
        if min == max {
            // A zero-width entry is structurally well-formed but cannot
            // carry remaining work; reject explicitly so the caller sees
            // a diagnostic message that points at the entry itself.
            return Err(crate::error::CosmosError::builder()
                .with_status(
                    crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE,
                )
                .with_message(format!(
                    "continuation token has zero-width active_tokens entry (min == max == `{}`); \
                     zero-width entries cannot carry remaining work",
                    min.to_hex(),
                ))
                .build());
        }
        let range = FeedRange::new(min, max)?;
        if let Some(prev) = parsed.last() {
            if range.min_inclusive() < prev.range.max_exclusive() {
                return Err(crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE,
                    )
                    .with_message(format!(
                        "continuation token active_tokens must be sorted and non-overlapping; \
                         entry [{}, {}) is out of order or overlaps the previous entry [{}, {})",
                        range.min_inclusive().to_hex(),
                        range.max_exclusive().to_hex(),
                        prev.range.min_inclusive().to_hex(),
                        prev.range.max_exclusive().to_hex(),
                    ))
                    .build());
            }
        }
        parsed.push(SavedActiveToken {
            range,
            server_continuation: entry.server_continuation,
        });
    }

    // Cursor cannot leapfrog past a still-active token entry: anything
    // strictly below the cursor is implicitly drained, but `active_tokens`
    // entries are by definition not drained.
    if let Some(first) = parsed.first() {
        if &cursor > first.range.min_inclusive() {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE)
                .with_message(format!(
                    "continuation token cursor `{}` is past the first active_tokens entry [{}, {}); \
                     cursor must be at or before every active range",
                    cursor.to_hex(),
                    first.range.min_inclusive().to_hex(),
                    first.range.max_exclusive().to_hex(),
                ))
                .build());
        }
    }

    Ok(SavedSnapshot {
        cursor,
        active_tokens: parsed,
    })
}

fn snapshot_kind(state: &PipelineNodeState) -> &'static str {
    match state {
        PipelineNodeState::Drained => "Drained",
        PipelineNodeState::Request { .. } => "Request",
        PipelineNodeState::SequentialDrain { .. } => "SequentialDrain",
        PipelineNodeState::UnorderedMerge { .. } => "UnorderedMerge",
        PipelineNodeState::SkipTake { .. } => "SkipTake",
        PipelineNodeState::StreamingOrderedMerge { .. } => "StreamingOrderedMerge",
        PipelineNodeState::Distinct { .. } => "Distinct",
    }
}

/// Validates the `active_tokens` from an `UnorderedMerge` continuation token.
///
/// Each entry must have `min < max` and be non-zero-width. The list must be
/// sorted ascending by `min_epk` and non-overlapping.
fn validate_unordered_merge_tokens(
    active_tokens: Vec<RangedToken>,
) -> crate::error::Result<Vec<SavedActiveToken>> {
    let mut parsed: Vec<SavedActiveToken> = Vec::with_capacity(active_tokens.len());
    for entry in active_tokens {
        let min = EffectivePartitionKey::from(entry.min_epk);
        let max = EffectivePartitionKey::from(entry.max_epk);
        if min >= max {
            return Err(crate::error::CosmosError::builder()
                .with_status(
                    crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE,
                )
                .with_message(format!(
                    "continuation token has invalid active_tokens entry \
                     (min `{}` >= max `{}`)",
                    min.to_hex(),
                    max.to_hex(),
                ))
                .build());
        }
        let range = FeedRange::new(min, max)?;
        if let Some(prev) = parsed.last() {
            if range.min_inclusive() < prev.range.max_exclusive() {
                return Err(crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE,
                    )
                    .with_message(format!(
                        "continuation token active_tokens must be sorted and non-overlapping; \
                         entry [{}, {}) overlaps [{}, {})",
                        range.min_inclusive().to_hex(),
                        range.max_exclusive().to_hex(),
                        prev.range.min_inclusive().to_hex(),
                        prev.range.max_exclusive().to_hex(),
                    ))
                    .build());
            }
        }
        parsed.push(SavedActiveToken {
            range,
            server_continuation: entry.server_continuation,
        });
    }
    Ok(parsed)
}

/// The kind of deduplication this query plan asks for, or
/// [`DistinctType::None`] when it asks for none.
fn plan_distinct_type(plan: &QueryPlan) -> DistinctType {
    plan.query_info
        .as_ref()
        .map(|info| info.distinct_type)
        .unwrap_or_default()
}

/// Wraps `pipeline`'s root in a [`Distinct`] stage when the plan calls for
/// deduplication, leaving it untouched otherwise.
///
/// `DISTINCT` composes *above* the fan-out root, matching .NET's
/// `PipelineFactory` and Java's `PipelinedDocumentQueryExecutionContext`:
/// merge/`ORDER BY` -> aggregate -> **DISTINCT** -> `GROUP BY` ->
/// `OFFSET`/`LIMIT`/`TOP`.
/// Wraps `node` in a [`Distinct`] stage when the plan calls for one.
///
/// `DISTINCT` deduplicates *before* any `OFFSET` / `LIMIT` / `TOP` window is
/// applied, so callers must wrap the fan-out with this first and only then
/// apply [`SkipTake`]; otherwise the window would count duplicate rows.
fn apply_distinct(
    node: Box<dyn PipelineNode>,
    distinct_type: DistinctType,
    last_hash: Option<Hash128>,
    resumed_drained: bool,
    emit_binary: bool,
) -> Box<dyn PipelineNode> {
    if distinct_type == DistinctType::None {
        return node;
    }
    // A fully-drained resume needs no deduplication stage: the inner pipeline
    // is a `DrainedLeaf` and will emit nothing. Wrapping it would leave a
    // `Distinct` whose `exhausted` is still `false`, so an unordered query
    // would refuse to re-snapshot a token it had just accepted.
    if resumed_drained {
        return node;
    }
    Box::new(Distinct::with_last_hash(
        node,
        distinct_type,
        last_hash,
        emit_binary,
    ))
}

/// Splits a resume state into the inner (fan-out) state and the `DISTINCT`
/// stage's saved `last_hash`, rejecting any token whose shape or distinct kind
/// does not match the current query plan.
///
/// A token minted before DISTINCT support (or for a non-DISTINCT query) has no
/// `Distinct` layer; resuming it into a DISTINCT plan would silently skip
/// deduplication, so it is rejected rather than reinterpreted. The mirror case
/// — a `Distinct` token resumed into a non-DISTINCT plan — falls through to the
/// inner builders, which reject the unexpected shape.
fn peel_distinct_resume(
    resume: Option<PipelineNodeState>,
    distinct_type: DistinctType,
) -> crate::error::Result<(Option<PipelineNodeState>, Option<Hash128>)> {
    match resume {
        Some(PipelineNodeState::Distinct {
            distinct_type: saved,
            last_hash,
            child,
        }) => {
            if saved != distinct_type {
                return Err(distinct_token_mismatch(saved, distinct_type));
            }
            if saved != DistinctType::Ordered {
                // We never mint one, so this is a hand-crafted or corrupted
                // token. Resuming would re-emit every value seen before the
                // checkpoint.
                return Err(crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED,
                    )
                    .with_message(
                        "continuation token carries unordered DISTINCT state, which cannot be \
                         resumed; add a matching ORDER BY to make the query resumable",
                    )
                    .build());
            }
            Ok((Some(*child), last_hash))
        }
        // `Drained` is shape-agnostic: the whole pipeline, DISTINCT included,
        // finished.
        Some(PipelineNodeState::Drained) => Ok((Some(PipelineNodeState::Drained), None)),
        Some(other) if distinct_type != DistinctType::None => {
            Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH)
                .with_message(format!(
                    "continuation token shape {} does not match a DISTINCT query",
                    snapshot_kind(&other)
                ))
                .build())
        }
        other => Ok((other, None)),
    }
}

fn distinct_token_mismatch(
    saved: DistinctType,
    expected: DistinctType,
) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH)
        .with_message(format!(
            "continuation token was minted for {saved:?} DISTINCT but the query plan reports \
             {expected:?}"
        ))
        .build()
}

/// Validates that the query plan does not require features we don't yet support.
fn validate_query_plan(plan: &QueryPlan) -> crate::error::Result<()> {
    if plan.hybrid_search_query_info.is_some() {
        return Err(unsupported_feature("hybrid search queries"));
    }

    if let Some(info) = &plan.query_info {
        validate_query_info(info)?;
    }

    Ok(())
}

fn validate_query_info(info: &QueryInfo) -> crate::error::Result<()> {
    if !info.order_by.is_empty() {
        return Err(unsupported_feature("ORDER BY in cross-partition queries"));
    }
    if !info.aggregates.is_empty() {
        return Err(unsupported_feature("aggregates in cross-partition queries"));
    }
    if !info.group_by_expressions.is_empty() {
        return Err(unsupported_feature("GROUP BY in cross-partition queries"));
    }
    Ok(())
}

/// Combines a query plan's `TOP` and `LIMIT` into a single global take bound.
///
/// Both clauses cap the number of documents returned, so the effective take is
/// the tighter of the two (`min`), treating an absent clause as unbounded.
fn combine_take(info: &QueryInfo) -> Option<u64> {
    match (info.top, info.limit) {
        (Some(top), Some(limit)) => Some(top.min(limit)),
        (Some(top), None) => Some(top),
        (None, Some(limit)) => Some(limit),
        (None, None) => None,
    }
}

/// Returns `operation` with its query text replaced by the plan's
/// `rewrittenQuery`, if the plan provides a non-empty one.
///
/// The gateway rewrites `OFFSET x LIMIT y` (and `TOP n`) into a per-partition
/// bound (e.g. `OFFSET 0 LIMIT x + y`) so each partition returns enough
/// documents for the client to apply the *global* skip/take in
/// [`SkipTake`](super::SkipTake). Sending the original query to each partition
/// would double-apply the clause, so this substitution is required for
/// correctness — not an optimization. When the plan carries no rewritten query
/// the operation is returned unchanged (a cheap `Arc` clone).
fn rewritten_operation(
    operation: &Arc<CosmosOperation>,
    query_plan: &QueryPlan,
) -> crate::error::Result<Arc<CosmosOperation>> {
    let Some(rewritten) = query_plan
        .query_info
        .as_ref()
        .and_then(|info| info.rewritten_query.as_deref())
        .filter(|query| !query.is_empty())
    else {
        return Ok(Arc::clone(operation));
    };
    let rewritten = query_response::rewritten_query_from_beginning(rewritten)?;

    let mut body: serde_json::Value = match operation.body() {
        Some(bytes) if !bytes.is_empty() => serde_json::from_slice(bytes).map_err(|e| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_QUERY_REWRITE_BODY_INVALID)
                .with_message(
                    "cross-partition query request body is not valid JSON; \
                     cannot apply the plan's rewritten query",
                )
                .with_source(e)
                .build()
        })?,
        _ => serde_json::Value::Object(serde_json::Map::new()),
    };

    let serde_json::Value::Object(map) = &mut body else {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_QUERY_REWRITE_BODY_INVALID)
            .with_message(
                "cross-partition query request body must be a JSON object; \
                 cannot apply the plan's rewritten query",
            )
            .build());
    };
    map.insert("query".to_owned(), serde_json::Value::String(rewritten));

    let new_body = serde_json::to_vec(&body).map_err(|e| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_QUERY_REWRITE_BODY_INVALID)
            .with_message("failed to serialize rewritten cross-partition query body")
            .with_source(e)
            .build()
    })?;

    Ok(Arc::new((**operation).clone().with_body(new_body)))
}

/// Validates a query plan for [`build_streaming_ordered_merge`]: `ORDER BY`
/// is expected, but every other unsupported feature (TOP, non-streaming
/// `ORDER BY`, DISTINCT/GROUP BY/aggregates/OFFSET/LIMIT/hybrid-search) is
/// still rejected the same way [`validate_query_info`] rejects it.
fn validate_query_plan_for_streaming_order_by(plan: &QueryPlan) -> crate::error::Result<()> {
    if plan.hybrid_search_query_info.is_some() {
        return Err(unsupported_feature("hybrid search queries"));
    }
    let info = plan.query_info.as_ref().ok_or_else(|| {
        // Precondition of `is_streaming_order_by`; an internal planner bug if violated.
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_UNSUPPORTED_QUERY_FEATURE)
            .with_message(
                "internal error: streaming ORDER BY path selected with no queryInfo present",
            )
            .build()
    })?;
    if info.has_non_streaming_order_by {
        return Err(unsupported_feature(
            "non-streaming ORDER BY in cross-partition queries",
        ));
    }
    // `TOP` and `OFFSET`/`LIMIT` are supported combined with streaming ORDER BY:
    // the ordered merge streams the globally-sorted rows and a `SkipTake` root
    // (composed in `build_streaming_ordered_merge`) applies the window on top.
    if !info.aggregates.is_empty() {
        return Err(unsupported_feature(
            "aggregates combined with ORDER BY in cross-partition queries",
        ));
    }
    if !info.group_by_expressions.is_empty() {
        return Err(unsupported_feature(
            "GROUP BY combined with ORDER BY in cross-partition queries",
        ));
    }
    Ok(())
}

fn unsupported_feature(feature: &str) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::CLIENT_UNSUPPORTED_QUERY_FEATURE)
        .with_message(format!("unsupported query feature: {feature}"))
        .build()
}

/// Builds the error returned when a topology range resolved for a query-plan
/// EPK range does not actually overlap that range.
///
/// This is a contract violation: [`TopologyProvider::resolve_ranges`] is
/// expected to return only ranges that overlap the requested feed range. It
/// should be unreachable in practice, but returning a structured error rather
/// than panicking keeps a plan that cannot be served from taking down the
/// worker thread (and deadlocking the caller) — see issue #4574.
fn topology_range_not_overlapping_error(
    resolved: &FeedRange,
    query: &FeedRange,
) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::CLIENT_QUERY_PLAN_RANGE_NOT_COVERED_BY_TOPOLOGY)
        .with_message(format!(
            "resolved topology range {} does not overlap query plan EPK {}",
            render_feed_range_for_error(resolved),
            render_feed_range_for_error(query),
        ))
        .build()
}

/// Renders a feed range for diagnostics as a half-open `[min, max)` range.
fn render_feed_range_for_error(range: &FeedRange) -> String {
    format!(
        "range [{}, {})",
        range.min_inclusive().to_hex(),
        range.max_exclusive().to_hex(),
    )
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use crate::{
        driver::dataflow::{
            mocks::*, query_plan::QueryRange, PageResult, PipelineContext, RangedToken,
            ResolvedRange,
        },
        models::{
            effective_partition_key::EffectivePartitionKey, AccountReference, ContainerProperties,
            ContainerReference, DatabaseReference, ItemReference, OperationType, PartitionKey,
            PartitionKeyDefinition, ResourceType, SystemProperties,
        },
    };

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        )
    }

    fn test_database() -> DatabaseReference {
        DatabaseReference::from_name(test_account(), "db".to_owned())
    }

    fn test_partition_key_definition() -> PartitionKeyDefinition {
        // Explicit `version: 2`: this fixture models a modern V2 hash container,
        // which is the shape the EPK-window normalization path targets. An
        // absent `version` now deserializes to legacy V1 (see
        // `models::default_pk_version`), which would route these point queries
        // through the width-preserving `successor()` path instead of the
        // full-width `normalized_successor(16)` EPK window.
        serde_json::from_str(r#"{"paths":["/pk"],"version":2}"#).unwrap()
    }

    fn test_container_props() -> ContainerProperties {
        ContainerProperties {
            id: Cow::Owned("coll".into()),
            partition_key: test_partition_key_definition(),
            system_properties: SystemProperties::default(),
        }
    }

    fn test_container() -> ContainerReference {
        ContainerReference::new(
            test_account(),
            "db",
            "db_rid",
            "coll",
            "coll_rid",
            &test_container_props(),
        )
    }

    fn cross_partition_query_operation() -> CosmosOperation {
        CosmosOperation::query_items(test_container(), Some(FeedRange::full()))
            .with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec())
    }

    /// A cross-partition query scoped to an explicit EPK feed-range target,
    /// e.g. `FeedScope::range([min, max))`.
    fn query_operation_with_target(min: &str, max: &str) -> CosmosOperation {
        let target = FeedRange::new(
            EffectivePartitionKey::from(min),
            EffectivePartitionKey::from(max),
        )
        .unwrap();
        CosmosOperation::query_items(test_container(), Some(target))
            .with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec())
    }

    // --- build_trivial_pipeline tests ---

    #[test]
    fn plans_non_partitioned_pipeline_for_database_read() {
        let op = CosmosOperation::read_database(test_database());
        let pipeline = build_trivial_pipeline(Arc::new(op), None).unwrap();

        let request = pipeline.root().downcast_ref::<Request>().unwrap();
        assert_eq!(*request.target(), RequestTarget::NonPartitioned);
        assert_eq!(request.operation().operation_type(), OperationType::Read);
        assert_eq!(request.operation().resource_type(), ResourceType::Database);
    }

    #[test]
    fn plans_logical_partition_pipeline_for_item_read() {
        let pk = PartitionKey::from("pk-value");
        let item = ItemReference::from_name(&test_container(), pk.clone(), "doc1");
        let op = CosmosOperation::read_item(item);
        let pipeline = build_trivial_pipeline(Arc::new(op), None).unwrap();

        let request = pipeline.root().downcast_ref::<Request>().unwrap();
        assert_eq!(
            *request.target(),
            RequestTarget::LogicalPartitionKey(pk.clone())
        );
        assert_eq!(request.operation().operation_type(), OperationType::Read);
        assert_eq!(request.operation().resource_type(), ResourceType::Document);
    }

    #[test]
    fn plans_logical_partition_pipeline_for_partition_scoped_query() {
        // Regression for the SDK→driver differentiation (issue #4574 follow-up):
        // a query scoped to a COMPLETE partition key via `FeedScope::Partition`
        // / `cosmos_feed_range_for_partition_key` (a `LogicalPartition` feed
        // range) must still route by the logical partition key — emitting
        // `x-ms-documentdb-partitionkey` — NOT through the planner's EPK-point
        // path. This is the path that already works and must stay distinct from
        // the predicate-derived gateway point.
        let pk = PartitionKey::from("pk-value");
        let feed_range = FeedRange::for_partition(pk.clone(), &test_partition_key_definition());
        let op = CosmosOperation::query_items(test_container(), Some(feed_range))
            .with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec());

        // The operation is trivial (complete PK), so it routes through the
        // single-request trivial pipeline rather than the cross-partition planner.
        assert!(op.is_trivial());
        let pipeline = build_trivial_pipeline(Arc::new(op), None).unwrap();

        let request = pipeline.root().downcast_ref::<Request>().unwrap();
        assert_eq!(*request.target(), RequestTarget::LogicalPartitionKey(pk));
    }

    #[test]
    fn rejects_feed_range_target() {
        let op = CosmosOperation::read_all_items_cross_partition(test_container());

        // In debug builds, this panics via debug_assert; in release builds it returns Err.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            build_trivial_pipeline(Arc::new(op), None)
        }));

        match result {
            // Panicked in debug mode (expected)
            Err(_) if cfg!(debug_assertions) => {}
            // Panicked in release mode (bad)
            Err(_) => panic!("did not expect panic for FeedRange target"),
            // Returned Err in release mode (also acceptable)
            Ok(Err(err)) => {
                let rendered = err.to_string();
                assert!(
                    rendered.ends_with(
                        "FeedRange targeting requires a fan-out pipeline; \
                         use plan_operation for cross-partition queries"
                    ),
                    "unexpected: {rendered}"
                );
            }
            _ => panic!("expected error or panic for FeedRange target"),
        }
    }

    // --- build_sequential_drain tests ---

    /// Shorthand to build a `QueryRange` from hex-prefix EPK strings.
    fn qr(min: &str, max: &str) -> QueryRange {
        QueryRange {
            min: min.to_string(),
            max: max.to_string(),
            is_min_inclusive: true,
            is_max_inclusive: false,
        }
    }

    /// Shorthand to build a `ResolvedRange` from (min, max, pk_range_id).
    fn rr(min: &str, max: &str, pk_range_id: &str) -> ResolvedRange {
        ResolvedRange {
            partition_key_range_id: pk_range_id.to_string(),
            range: FeedRange::new(
                EffectivePartitionKey::from(min),
                EffectivePartitionKey::from(max),
            )
            .unwrap(),
        }
    }

    /// Builds a query plan with the given query ranges (and no query info).
    fn plan_with_ranges(ranges: Vec<QueryRange>) -> QueryPlan {
        QueryPlan {
            partitioned_query_execution_info_version: 1,
            query_info: None,
            query_ranges: ranges,
            hybrid_search_query_info: None,
        }
    }

    /// Asserts the pipeline root is a `SkipTake`, returning its
    /// `(remaining_skip, remaining_take)` window and the wrapped fan-out child.
    fn unwrap_skip_take(pipeline: Pipeline) -> (u64, Option<u64>, Box<SequentialDrain>) {
        let root = pipeline
            .into_root()
            .downcast::<SkipTake>()
            .expect("expected SkipTake root");
        let (skip, take) = match root.snapshot_state().unwrap() {
            PipelineNodeState::SkipTake {
                remaining_skip,
                remaining_take,
                ..
            } => (remaining_skip, remaining_take),
            other => panic!("expected SkipTake snapshot, got {other:?}"),
        };
        let mut children = root.into_children();
        assert_eq!(children.len(), 1, "SkipTake must wrap exactly one child");
        let child = children
            .pop()
            .unwrap()
            .downcast::<SequentialDrain>()
            .expect("expected SequentialDrain child");
        (skip, take, child)
    }

    /// Asserts that the pipeline is a `SequentialDrain` containing `Request` nodes
    /// targeting the given EPK ranges (in order).
    type ExpectedDrainRequestWithPartition<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str);
    type ExpectedDrainRequestWithContinuation<'a> =
        (&'a str, &'a str, &'a str, &'a str, &'a str, Option<&'a str>);

    fn assert_drain_requests(pipeline: Pipeline, expected: &[(&str, &str, &str)]) {
        let expected = expected
            .iter()
            .map(|&(min, max, pk_range_id)| (min, max, pk_range_id, min, max))
            .collect::<Vec<_>>();
        assert_drain_requests_with_partitions(pipeline, &expected);
    }

    fn assert_drain_requests_with_partitions(
        pipeline: Pipeline,
        expected: &[ExpectedDrainRequestWithPartition<'_>],
    ) {
        let drain = pipeline
            .into_root()
            .downcast::<SequentialDrain>()
            .expect("expected SequentialDrain root");
        let children = drain.into_children();
        assert_eq!(
            children.len(),
            expected.len(),
            "expected {} request nodes, got {}",
            expected.len(),
            children.len(),
        );
        for (child, &(min, max, pk_range_id, partition_min, partition_max)) in
            children.into_iter().zip(expected)
        {
            let request = child
                .downcast::<Request>()
                .expect("expected Request child node");
            assert_eq!(
                *request.target(),
                RequestTarget::effective_partition_key_range(
                    FeedRange::new(
                        EffectivePartitionKey::from(min),
                        EffectivePartitionKey::from(max),
                    )
                    .unwrap(),
                    pk_range_id.to_string(),
                    FeedRange::new(
                        EffectivePartitionKey::from(partition_min),
                        EffectivePartitionKey::from(partition_max),
                    )
                    .unwrap(),
                ),
                "mismatch for pk range {pk_range_id}"
            );
        }
    }

    fn assert_drain_requests_with_partitions_and_continuation(
        pipeline: Pipeline,
        expected: &[ExpectedDrainRequestWithContinuation<'_>],
    ) {
        let drain = pipeline
            .into_root()
            .downcast::<SequentialDrain>()
            .expect("expected SequentialDrain root");
        let children = drain.into_children();
        assert_eq!(
            children.len(),
            expected.len(),
            "expected {} request nodes, got {}",
            expected.len(),
            children.len(),
        );

        for (child, &(min, max, pk_range_id, partition_min, partition_max, continuation)) in
            children.into_iter().zip(expected)
        {
            let request = child
                .downcast::<Request>()
                .expect("expected Request child node");
            assert_eq!(
                *request.target(),
                RequestTarget::effective_partition_key_range(
                    FeedRange::new(
                        EffectivePartitionKey::from(min),
                        EffectivePartitionKey::from(max),
                    )
                    .unwrap(),
                    pk_range_id.to_string(),
                    FeedRange::new(
                        EffectivePartitionKey::from(partition_min),
                        EffectivePartitionKey::from(partition_max),
                    )
                    .unwrap(),
                ),
                "mismatch for pk range {pk_range_id}"
            );

            let expected_state = PipelineNodeState::Request {
                server_continuation: continuation.map(ToOwned::to_owned),
            };
            assert_eq!(request.snapshot_state().unwrap(), expected_state);
        }
    }

    #[tokio::test]
    async fn builds_single_node_pipeline_for_one_partition() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-0")])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_drain_requests(pipeline, &[("", "FF", "pkrange-0")]);
    }

    #[tokio::test]
    async fn builds_sequential_drain_for_multiple_partitions() {
        // Query targets full range, topology has two partitions split at "80".
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "80", "pkrange-left"),
            rr("80", "FF", "pkrange-right"),
        ])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_drain_requests(
            pipeline,
            &[("", "80", "pkrange-left"), ("80", "FF", "pkrange-right")],
        );
    }

    // --- fan-out limit / finalize_plan tests ---

    /// Builds a fresh two-partition sequential drain for the fan-out tests.
    async fn two_partition_drain() -> Pipeline {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "80", "pkrange-left"),
            rr("80", "FF", "pkrange-right"),
        ])]);
        build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn fan_out_width_sums_leaf_nodes() {
        let pipeline = two_partition_drain().await;
        assert_eq!(pipeline.fan_out_width(), 2);
    }

    #[tokio::test]
    async fn finalize_plan_allows_fresh_plan_within_limit() {
        let pipeline = two_partition_drain().await;
        let op = Arc::new(cross_partition_query_operation());
        let options = PlanOptions::default().with_max_fan_out(2);
        finalize_plan(pipeline, op, true, &options).expect("plan at the limit should be allowed");
    }

    #[tokio::test]
    async fn finalize_plan_rejects_fresh_plan_exceeding_limit() {
        let pipeline = two_partition_drain().await;
        let op = Arc::new(cross_partition_query_operation());
        let options = PlanOptions::default().with_max_fan_out(1);
        let err = match finalize_plan(pipeline, op, true, &options) {
            Ok(_) => panic!("plan over the limit should be rejected"),
            Err(err) => err,
        };
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED),
            "unexpected error: {err}",
        );
    }

    #[tokio::test]
    async fn finalize_plan_skips_limit_on_resume() {
        let pipeline = two_partition_drain().await;
        let op = Arc::new(cross_partition_query_operation());
        // A width of 2 exceeds the limit of 1, but resume (is_fresh = false)
        // must not re-check the fan-out.
        let options = PlanOptions::default().with_max_fan_out(1);
        finalize_plan(pipeline, op, false, &options).expect("resume must skip the fan-out check");
    }

    #[tokio::test]
    async fn builds_pipeline_for_multiple_query_ranges() {
        // Query plan specifies two disjoint query ranges; each resolves to one partition.
        let plan = plan_with_ranges(vec![qr("", "40"), qr("80", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![
            Ok(vec![rr("", "40", "pkrange-A")]),
            Ok(vec![rr("80", "FF", "pkrange-C")]),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_drain_requests(
            pipeline,
            &[("", "40", "pkrange-A"), ("80", "FF", "pkrange-C")],
        );
    }

    #[tokio::test]
    async fn query_range_spans_multiple_topology_partitions() {
        // A single query range [00, C0) spans three topology partitions.
        let plan = plan_with_ranges(vec![qr("00", "C0")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("00", "40", "pkrange-1"),
            rr("40", "80", "pkrange-2"),
            rr("80", "C0", "pkrange-3"),
        ])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_drain_requests(
            pipeline,
            &[
                ("00", "40", "pkrange-1"),
                ("40", "80", "pkrange-2"),
                ("80", "C0", "pkrange-3"),
            ],
        );
    }

    #[tokio::test]
    async fn multiple_query_ranges_each_spanning_multiple_partitions() {
        // Two query ranges, each resolving to multiple partitions. The resulting
        // pipeline should have all resolved ranges in order.
        let plan = plan_with_ranges(vec![qr("", "60"), qr("A0", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![
            // First query range [, 60) spans two partitions.
            Ok(vec![
                rr("", "30", "pkrange-alpha"),
                rr("30", "60", "pkrange-beta"),
            ]),
            // Second query range [A0, FF) spans two partitions.
            Ok(vec![
                rr("A0", "D0", "pkrange-gamma"),
                rr("D0", "FF", "pkrange-delta"),
            ]),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_drain_requests(
            pipeline,
            &[
                ("", "30", "pkrange-alpha"),
                ("30", "60", "pkrange-beta"),
                ("A0", "D0", "pkrange-gamma"),
                ("D0", "FF", "pkrange-delta"),
            ],
        );
    }

    #[tokio::test]
    async fn topology_partition_wider_than_query_range() {
        // The topology partition [, FF) is wider than query range [20, 80).
        let plan = plan_with_ranges(vec![qr("20", "80")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-wide")])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_drain_requests_with_partitions(pipeline, &[("20", "80", "pkrange-wide", "", "FF")]);
    }

    #[tokio::test]
    async fn closed_point_query_range_emits_epk_window() {
        // Regression for issues #4574 / #4638: an equality / `IN` predicate on
        // the partition key makes the gateway return a *closed* point range
        // `[X, X]` (isMinInclusive == isMaxInclusive == true). Option B: the
        // planner normalizes it to the half-open window `[X, successor(X))` and
        // emits it as a `start`/`end-epk` pair scoped to the owning partition
        // (with `x-ms-read-key-type: EffectivePartitionKeyRange`, #4729). The
        // empty-intersection panic is avoided because `min != max`.
        let point = QueryRange {
            min: "30".to_string(),
            max: "30".to_string(),
            is_min_inclusive: true,
            is_max_inclusive: true,
        };
        let plan = plan_with_ranges(vec![point]);
        let op = cross_partition_query_operation();
        // The single physical partition `["", "FF")` owns EPK "30".
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-0")])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();

        // Narrow `[30, successor(30))` EPK window over the owning partition.
        let s30 = EffectivePartitionKey::from("30")
            .normalized_successor(16)
            .to_hex();
        assert_drain_requests_with_partitions(
            pipeline,
            &[("30", s30.as_str(), "pkrange-0", "", "FF")],
        );
    }

    #[tokio::test]
    async fn in_predicate_colocated_points_emit_one_window_each() {
        // `WHERE c.pk IN (@a, @b)` where both values hash into the same
        // physical partition: the gateway returns two point ranges, both
        // resolving to `pkrange-0`. Option B emits a distinct, disjoint EPK
        // window (`[X, successor(X))`) per value — no de-duplication needed
        // because each window matches only its own value.
        let plan = plan_with_ranges(vec![
            QueryRange {
                min: "30".to_string(),
                max: "30".to_string(),
                is_min_inclusive: true,
                is_max_inclusive: true,
            },
            QueryRange {
                min: "50".to_string(),
                max: "50".to_string(),
                is_min_inclusive: true,
                is_max_inclusive: true,
            },
        ]);
        let op = cross_partition_query_operation();
        // Both points resolve to the same single partition.
        let mut topology = MockTopologyProvider::new(vec![
            Ok(vec![rr("", "FF", "pkrange-0")]),
            Ok(vec![rr("", "FF", "pkrange-0")]),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();

        let s30 = EffectivePartitionKey::from("30")
            .normalized_successor(16)
            .to_hex();
        let s50 = EffectivePartitionKey::from("50")
            .normalized_successor(16)
            .to_hex();
        assert_drain_requests_with_partitions(
            pipeline,
            &[
                ("30", s30.as_str(), "pkrange-0", "", "FF"),
                ("50", s50.as_str(), "pkrange-0", "", "FF"),
            ],
        );
    }

    #[tokio::test]
    async fn in_predicate_points_across_partitions_emit_one_window_each() {
        // `WHERE c.pk IN (@a, @b)` where the values live in different
        // partitions: each point normalizes to its own `[X, successor(X))`
        // EPK window scoped to the owning partition.
        let plan = plan_with_ranges(vec![
            QueryRange {
                min: "20".to_string(),
                max: "20".to_string(),
                is_min_inclusive: true,
                is_max_inclusive: true,
            },
            QueryRange {
                min: "C0".to_string(),
                max: "C0".to_string(),
                is_min_inclusive: true,
                is_max_inclusive: true,
            },
        ]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![
            Ok(vec![rr("", "80", "pkrange-left")]),
            Ok(vec![rr("80", "FF", "pkrange-right")]),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();

        let s20 = EffectivePartitionKey::from("20")
            .normalized_successor(16)
            .to_hex();
        let sc0 = EffectivePartitionKey::from("C0")
            .normalized_successor(16)
            .to_hex();
        assert_drain_requests_with_partitions(
            pipeline,
            &[
                ("20", s20.as_str(), "pkrange-left", "", "80"),
                ("C0", sc0.as_str(), "pkrange-right", "80", "FF"),
            ],
        );
    }

    #[test]
    fn query_range_to_feed_range_normalizes_closed_point_to_window() {
        // A closed point range `[X, X]` (isMaxInclusive=true) is normalized to
        // the half-open window `[X, successor(X))` (Option B, issues #4574 /
        // #4638) rather than kept as an empty point.
        let point = QueryRange {
            min: "30".to_string(),
            max: "30".to_string(),
            is_min_inclusive: true,
            is_max_inclusive: true,
        };
        let fr = query_range_to_feed_range(&point, None).unwrap();
        assert_eq!(fr.min_inclusive().to_hex(), "30");
        // successor("30") = "31" (no width normalization requested).
        assert_eq!(fr.max_exclusive().to_hex(), "31");
    }

    #[test]
    fn query_range_to_feed_range_preserves_half_open() {
        // A half-open range `[A, B)` (isMaxInclusive=false) is a plain range —
        // the common full-container / split case.
        let fr = query_range_to_feed_range(&qr("20", "80"), None).unwrap();
        assert_eq!(fr.min_inclusive().to_hex(), "20");
        assert_eq!(fr.max_exclusive().to_hex(), "80");
    }

    #[test]
    fn query_range_to_feed_range_closed_non_point_range_passes_through_when_len_unknown() {
        // A closed non-point range `[A, B]` (`min != max`) with no known EPK
        // width (V1) is passed through unchanged — only equality / `IN` *points*
        // are transformed.
        let closed = QueryRange {
            min: "20".to_string(),
            max: "3AFF".to_string(),
            is_min_inclusive: true,
            is_max_inclusive: true,
        };
        let fr = query_range_to_feed_range(&closed, None).unwrap();
        assert_eq!(fr.min_inclusive().to_hex(), "20");
        // Non-point range: upper bound is passed through, no successor applied.
        assert_eq!(fr.max_exclusive().to_hex(), "3AFF");
    }

    #[test]
    fn query_range_to_feed_range_closed_non_point_range_passes_through_even_with_known_len() {
        // Regression guard (#4574 / #4638): a closed *non-point* range
        // (`min != max`) is an HPK **prefix** bound, not a full key. It must be
        // passed through unchanged even when the full EPK width is known —
        // advancing it with a successor over-extends the prefix band and drops
        // owning partitions (the in-memory-emulator
        // `hpk_tenant_prefix_where_full_scope` regression: touched {5,6} instead
        // of {4,5,6}).
        let closed = QueryRange {
            min: "20".to_string(),
            max: "3A".to_string(),
            is_min_inclusive: true,
            is_max_inclusive: true,
        };
        let fr = query_range_to_feed_range(&closed, Some(16)).unwrap();
        assert_eq!(fr.min_inclusive().to_hex(), "20");
        // Non-point range: passed through, NOT normalized / incremented.
        assert_eq!(fr.max_exclusive().to_hex(), "3A");
    }

    #[test]
    fn query_range_to_feed_range_normalizes_point_to_full_width() {
        // A closed *point* (`min == max`, an equality / `IN` value — always a
        // full key) on a single-path V2 container (16-byte EPK): the
        // trailing-zero-trimmed value `3A` is zero-extended to 16 bytes, then
        // incremented at the last byte (Option B full-width normalization,
        // matching .NET).
        let point = QueryRange {
            min: "3A".to_string(),
            max: "3A".to_string(),
            is_min_inclusive: true,
            is_max_inclusive: true,
        };
        let fr = query_range_to_feed_range(&point, Some(16)).unwrap();
        assert_eq!(fr.min_inclusive().to_hex(), "3A");
        // normalized_successor("3A", 16) = "3A" zero-extended to 16 bytes, then
        // +1 at the last byte: [0x3A, 0x00 x14, 0x01].
        let mut expected = vec![0x3Au8];
        expected.resize(16, 0x00);
        expected[15] = 0x01;
        let expected_hex: String = expected.iter().map(|b| format!("{:02X}", b)).collect();
        assert_eq!(fr.max_exclusive().to_hex(), expected_hex);
    }

    #[tokio::test]
    async fn restricts_fanout_to_explicit_target_range() {
        // The reported bug: query plan spans the whole space `[, FF)` but the
        // caller scoped the query to `[00, 80)` via `FeedScope::range`. Only
        // the requested slice must be queried, not the neighbouring `[80, FF)`
        // partition. The physical topology actually contains both partitions,
        // so a planner that ignores the target would resolve and emit a leaf
        // for `[80, FF)` as well.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = query_operation_with_target("00", "80");
        let mut topology = PhysicalTopologyProvider::new(vec![
            rr("00", "80", "pkrange-left"),
            rr("80", "FF", "pkrange-right"),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_drain_requests(pipeline, &[("00", "80", "pkrange-left")]);
    }

    #[tokio::test]
    async fn drops_query_ranges_outside_target() {
        // Two disjoint query-plan ranges; the target only overlaps the first.
        // The second range must contribute no request leaves (and its topology
        // must never be resolved).
        let plan = plan_with_ranges(vec![qr("", "40"), qr("80", "FF")]);
        let op = query_operation_with_target("", "40");
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "40", "pkrange-A")])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_drain_requests(pipeline, &[("", "40", "pkrange-A")]);
    }

    #[tokio::test]
    async fn clips_query_range_to_partial_target_overlap() {
        // The target `[20, 60)` partially overlaps a single query-plan range
        // spanning several partitions. Only partitions within the target,
        // clipped to its bounds, may be queried.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = query_operation_with_target("20", "60");
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("00", "40", "pkrange-1"),
            rr("40", "80", "pkrange-2"),
        ])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_drain_requests_with_partitions(
            pipeline,
            &[
                ("20", "40", "pkrange-1", "00", "40"),
                ("40", "60", "pkrange-2", "40", "80"),
            ],
        );
    }

    #[tokio::test]
    async fn drops_query_range_touching_target_boundary() {
        // A query-plan range that only *touches* the target's exclusive upper
        // bound (target `[, 40)`, range `[40, FF)`) shares no EPKs with the
        // target and must be dropped, not queried. Exercises the exact
        // boundary case where `intersect_feed_ranges` collapses to empty.
        let plan = plan_with_ranges(vec![qr("", "40"), qr("40", "FF")]);
        let op = query_operation_with_target("", "40");
        let mut topology = PhysicalTopologyProvider::new(vec![
            rr("", "40", "pkrange-A"),
            rr("40", "FF", "pkrange-B"),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_drain_requests(pipeline, &[("", "40", "pkrange-A")]);
    }

    #[tokio::test]
    async fn resume_restricts_fanout_to_target_range() {
        // Resuming a target-scoped query must also honour the target: the
        // `[80, FF)` partition lies outside `[00, 80)` and must not be queried
        // even though the query plan spans `[, FF)` and that partition exists
        // in the physical topology. An unclipped resume would emit a second,
        // fresh-start leaf for `[80, FF)`.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = query_operation_with_target("00", "80");
        let mut topology = PhysicalTopologyProvider::new(vec![
            rr("00", "80", "pkrange-left"),
            rr("80", "FF", "pkrange-right"),
        ]);

        let resume = saved_drain(vec![("00", "80", saved_request(Some("server-token-xyz")))]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[(
                "00",
                "80",
                "pkrange-left",
                "00",
                "80",
                Some("server-token-xyz"),
            )],
        );
    }

    #[tokio::test]
    async fn rejects_query_plan_with_top() {
        let plan = QueryPlan {
            query_info: Some(QueryInfo {
                top: Some(10),
                ..Default::default()
            }),
            ..plan_with_ranges(vec![qr("", "FF")])
        };
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-a")])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        let (skip, take, _child) = unwrap_skip_take(pipeline);
        assert_eq!(skip, 0);
        assert_eq!(take, Some(10));
    }

    #[tokio::test]
    async fn wraps_fanout_in_skip_take_for_offset_limit() {
        let plan = QueryPlan {
            query_info: Some(QueryInfo {
                offset: Some(5),
                limit: Some(10),
                top: Some(7),
                ..Default::default()
            }),
            ..plan_with_ranges(vec![qr("", "FF")])
        };
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-a")])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        let (skip, take, _child) = unwrap_skip_take(pipeline);
        assert_eq!(skip, 5);
        // Effective take = min(top = 7, limit = 10) = 7.
        assert_eq!(take, Some(7));
    }

    #[tokio::test]
    async fn no_skip_take_wrapper_without_offset_limit_top() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-a")])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        // The fan-out is the pipeline root directly (no SkipTake wrapper).
        assert_drain_requests(pipeline, &[("", "FF", "pkrange-a")]);
    }

    #[tokio::test]
    async fn skip_take_continuation_accepts_cross_construct_window() {
        // The plan is an OFFSET/LIMIT query but the continuation token was minted
        // by a TOP query. Both build the identical global skip/take pipeline, so
        // resume validates the pipeline shape (a SkipTake node exists), not which
        // SQL construct minted the token — this must resume, not reject.
        let plan = QueryPlan {
            query_info: Some(QueryInfo {
                offset: Some(2),
                limit: Some(5),
                ..Default::default()
            }),
            ..plan_with_ranges(vec![qr("", "FF")])
        };
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-a")])]);

        let resume = PipelineNodeState::SkipTake {
            remaining_skip: 0,
            remaining_take: Some(3),
            child: Box::new(PipelineNodeState::Drained),
        };
        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .expect("a TOP-shaped window token should resume an OFFSET/LIMIT query");
        // The saved child was `Drained`, so the resumed pipeline is drained.
        let mut root = pipeline.into_root();
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        assert!(matches!(
            root.next_page(&mut context).await.unwrap(),
            PageResult::Drained
        ));
    }

    #[tokio::test]
    async fn skip_take_continuation_rejects_window_token_against_windowless_query() {
        // The token carries a skip/take window, but the resumed query has no
        // OFFSET/LIMIT/TOP — so the pipeline it resumes into has no SkipTake node
        // to receive it. This is a genuine pipeline-shape mismatch and is
        // rejected rather than silently applying a phantom window.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-a")])]);

        let resume = PipelineNodeState::SkipTake {
            remaining_skip: 0,
            remaining_take: Some(3),
            child: Box::new(PipelineNodeState::Drained),
        };
        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .expect_err("a skip/take token must not resume a query with no skip/take window");
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH,
            "expected SHAPE_MISMATCH for a window token against a windowless query; got {err:?}",
        );
    }

    #[tokio::test]
    async fn skip_take_continuation_accepts_matching_stage() {
        // A matching-window continuation (OFFSET/LIMIT token, OFFSET/LIMIT query)
        // resumes without error. The `Drained` child yields a drained pipeline.
        let plan = QueryPlan {
            query_info: Some(QueryInfo {
                offset: Some(2),
                limit: Some(5),
                ..Default::default()
            }),
            ..plan_with_ranges(vec![qr("", "FF")])
        };
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-a")])]);

        let resume = PipelineNodeState::SkipTake {
            remaining_skip: 1,
            remaining_take: Some(3),
            child: Box::new(PipelineNodeState::Drained),
        };
        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .expect("matching-stage continuation should resume");
        // The saved child was `Drained`, so the resumed pipeline is drained.
        let mut root = pipeline.into_root();
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        assert!(matches!(
            root.next_page(&mut context).await.unwrap(),
            PageResult::Drained
        ));
    }

    #[tokio::test]
    async fn applies_rewritten_query_to_request_bodies() {
        let plan = QueryPlan {
            query_info: Some(QueryInfo {
                offset: Some(2),
                limit: Some(3),
                rewritten_query: Some("SELECT * FROM c OFFSET 0 LIMIT 5".to_owned()),
                ..Default::default()
            }),
            ..plan_with_ranges(vec![qr("", "FF")])
        };
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-a")])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        let (_skip, _take, child) = unwrap_skip_take(pipeline);
        let requests = child.into_children();
        assert_eq!(requests.len(), 1);
        let request = requests
            .into_iter()
            .next()
            .unwrap()
            .downcast::<Request>()
            .expect("expected Request node");
        let body = request.operation().body().expect("request body");
        let parsed: serde_json::Value = serde_json::from_slice(body).unwrap();
        assert_eq!(parsed["query"], "SELECT * FROM c OFFSET 0 LIMIT 5");
    }

    #[test]
    fn rewritten_operation_replaces_formattable_order_by_placeholder() {
        let operation = Arc::new(
            cross_partition_query_operation().with_body(
                br#"{"query":"SELECT TOP 2 * FROM c ORDER BY VectorDistance(c.v, @v, false)","parameters":[{"name":"@v","value":[0.0,1.0]}]}"#
                    .to_vec(),
            ),
        );
        let plan = QueryPlan {
            query_info: Some(QueryInfo {
                rewritten_query: Some(
                    "SELECT TOP 2 c._rid, [{\"item\": VectorDistance(c.v, @v, false)}] AS orderByItems, c AS payload FROM c WHERE {documentdb-formattableorderbyquery-filter} ORDER BY VectorDistance(c.v, @v, false)"
                        .to_owned(),
                ),
                ..Default::default()
            }),
            ..Default::default()
        };

        let rewritten = rewritten_operation(&operation, &plan).unwrap();
        let body: serde_json::Value = serde_json::from_slice(rewritten.body().unwrap()).unwrap();
        let query = body["query"].as_str().unwrap();
        assert!(query.contains("WHERE true"));
        assert!(!query.contains("documentdb-formattableorderbyquery-filter"));
        assert_eq!(
            body["parameters"][0]["value"],
            serde_json::json!([0.0, 1.0])
        );
    }

    #[tokio::test]
    async fn rejects_query_plan_with_order_by() {
        use super::super::query_plan::SortOrder;
        let plan = QueryPlan {
            query_info: Some(QueryInfo {
                order_by: vec![SortOrder::Ascending],
                ..Default::default()
            }),
            ..plan_with_ranges(vec![qr("", "FF")])
        };
        let op = cross_partition_query_operation();
        let mut topology = NoopTopologyProvider;

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.ends_with("unsupported query feature: ORDER BY in cross-partition queries"),
            "unexpected: {rendered}"
        );
    }

    #[tokio::test]
    async fn rejects_query_plan_with_aggregates() {
        let plan = QueryPlan {
            query_info: Some(QueryInfo {
                aggregates: vec!["Count".to_string()],
                ..Default::default()
            }),
            ..plan_with_ranges(vec![qr("", "FF")])
        };
        let op = cross_partition_query_operation();
        let mut topology = NoopTopologyProvider;

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.ends_with("unsupported query feature: aggregates in cross-partition queries"),
            "unexpected: {rendered}"
        );
    }

    #[tokio::test]
    async fn rejects_query_plan_with_group_by() {
        let plan = QueryPlan {
            query_info: Some(QueryInfo {
                group_by_expressions: vec!["c.category".to_string()],
                ..Default::default()
            }),
            ..plan_with_ranges(vec![qr("", "FF")])
        };
        let op = cross_partition_query_operation();
        let mut topology = NoopTopologyProvider;

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.ends_with("unsupported query feature: GROUP BY in cross-partition queries"),
            "unexpected: {rendered}"
        );
    }

    #[tokio::test]
    async fn rejects_query_plan_with_hybrid_search() {
        let plan = QueryPlan {
            hybrid_search_query_info: Some(super::super::query_plan::HybridSearchQueryInfo {
                global_statistics_query: "SELECT COUNT(1) FROM c".to_string(),
                component_query_infos: vec![],
                component_weights: vec![],
                skip: None,
                take: Some(10),
                requires_global_statistics: true,
            }),
            ..plan_with_ranges(vec![qr("", "FF")])
        };
        let op = cross_partition_query_operation();
        let mut topology = NoopTopologyProvider;

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.ends_with("unsupported query feature: hybrid search queries"),
            "unexpected: {rendered}"
        );
    }

    #[tokio::test]
    async fn accepts_query_plan_with_no_query_info() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-0")])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_drain_requests(pipeline, &[("", "FF", "pkrange-0")]);
    }

    #[tokio::test]
    async fn rejects_empty_query_ranges() {
        let plan = plan_with_ranges(vec![]);
        let op = cross_partition_query_operation();
        let mut topology = NoopTopologyProvider;

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.ends_with("query plan produced no partition ranges to query"),
            "unexpected: {rendered}"
        );
    }

    #[tokio::test]
    async fn rejects_target_disjoint_from_query_ranges() {
        // A target that shares no EPKs with any query-plan range clips every
        // range away, leaving zero request leaves. On the fresh path this is
        // reported as the same hard error as an empty query plan — the clip
        // does not silently swallow the query. `NoopTopologyProvider` asserts
        // no partition is ever resolved (the clip skips before resolution).
        let plan = plan_with_ranges(vec![qr("80", "FF")]);
        let op = query_operation_with_target("00", "40");
        let mut topology = NoopTopologyProvider;

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.ends_with("query plan produced no partition ranges to query"),
            "unexpected: {rendered}"
        );
    }

    #[tokio::test]
    async fn propagates_topology_resolution_error() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology =
            MockTopologyProvider::new(vec![Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("topology resolution failed")
                .build())]);

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.ends_with("topology resolution failed"),
            "unexpected: {rendered}"
        );
    }

    // -----------------------------------------------------------------
    // Resume tests
    // -----------------------------------------------------------------

    /// Builds a sparse `SequentialDrain` resume state from the legacy
    /// `(min, max, state)` triple shape. Drained entries set / advance the
    /// cursor; `Request { Some(token) }` entries become `active_tokens`;
    /// `Request { None }` entries are skipped (sparse encoding treats them
    /// as implicit fresh-start). The triples are assumed sorted; tests
    /// that want to exercise the validator with malformed sparse shapes
    /// build `PipelineNodeState::SequentialDrain { ... }` directly.
    fn saved_drain(children: Vec<(&str, &str, PipelineNodeState)>) -> PipelineNodeState {
        let mut cursor: Option<String> = None;
        let mut active_tokens: Vec<RangedToken> = Vec::new();
        for (min, max, state) in children {
            match state {
                PipelineNodeState::Drained => {
                    debug_assert!(
                        cursor.is_none(),
                        "saved_drain helper does not support drained children after the cursor; \
                         construct the sparse shape directly for that case",
                    );
                    cursor = Some(max.to_owned());
                }
                PipelineNodeState::Request {
                    server_continuation,
                } => {
                    if cursor.is_none() {
                        cursor = Some(min.to_owned());
                    }
                    if let Some(token) = server_continuation {
                        active_tokens.push(RangedToken {
                            min_epk: min.to_owned(),
                            max_epk: max.to_owned(),
                            server_continuation: token,
                        });
                    }
                }
                other => panic!(
                    "saved_drain helper does not accept nested SequentialDrain states; \
                     construct the sparse shape directly. Got: {other:?}"
                ),
            }
        }
        PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: cursor.unwrap_or_default(),
            active_tokens,
        }
    }

    fn saved_request(server_continuation: Option<&str>) -> PipelineNodeState {
        PipelineNodeState::Request {
            server_continuation: server_continuation.map(str::to_owned),
        }
    }

    #[tokio::test]
    async fn resume_drained_state_yields_drained_pipeline() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-0")])]);

        let pipeline = build_sequential_drain(
            &plan,
            &mut topology,
            &Arc::new(op),
            Some(PipelineNodeState::Drained),
        )
        .await
        .unwrap();

        assert!(matches!(
            pipeline.snapshot_state().unwrap(),
            PipelineNodeState::Drained
        ));
    }

    #[tokio::test]
    async fn resume_skips_topology_below_first_saved_child() {
        // Saved children cover only `[55, FF)`. The topology has a range
        // `[, 55)` that falls outside every saved range — that range has
        // already been drained on a prior page and must not be re-queried.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "55", "pk-a"),
            rr("55", "AA", "pk-b"),
            rr("AA", "FF", "pk-c"),
        ])]);

        let resume = saved_drain(vec![
            ("55", "AA", saved_request(None)),
            ("AA", "FF", saved_request(None)),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests(pipeline, &[("55", "AA", "pk-b"), ("AA", "FF", "pk-c")]);
    }

    #[tokio::test]
    async fn resume_propagates_server_continuation_to_every_surviving_leaf_after_split() {
        // The saved `[55, AA)` child held a server continuation. Between
        // sessions the underlying partition split into `[55, 70)` + `[70, AA)`;
        // every surviving leaf in the saved child's scope must carry the
        // saved continuation, otherwise the continuation-less leaves execute
        // a fresh query and re-emit items the caller already consumed.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "55", "pk-a"),
            rr("55", "70", "pk-b1"),
            rr("70", "AA", "pk-b2"),
            rr("AA", "FF", "pk-c"),
        ])]);

        let resume = saved_drain(vec![
            ("55", "AA", saved_request(Some("server-token-xyz"))),
            ("AA", "FF", saved_request(None)),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[
                ("55", "70", "pk-b1", "55", "70", Some("server-token-xyz")),
                ("70", "AA", "pk-b2", "70", "AA", Some("server-token-xyz")),
                ("AA", "FF", "pk-c", "AA", "FF", None),
            ],
        );
    }

    #[tokio::test]
    async fn resume_does_not_leak_continuation_into_siblings_past_saved_scope() {
        // Saved child `[55, AA)` holds a continuation; sibling `[AA, FF)`
        // does not. Topology unchanged across sessions: each saved child
        // maps 1:1 to its leaf, and the continuation must not propagate
        // into the following sibling.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "55", "pk-a"),
            rr("55", "AA", "pk-b"),
            rr("AA", "FF", "pk-c"),
        ])]);

        let resume = saved_drain(vec![
            ("55", "AA", saved_request(Some("server-token-xyz"))),
            ("AA", "FF", saved_request(None)),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[
                ("55", "AA", "pk-b", "55", "AA", Some("server-token-xyz")),
                ("AA", "FF", "pk-c", "AA", "FF", None),
            ],
        );
    }

    #[tokio::test]
    async fn resume_does_not_leak_continuation_across_query_ranges() {
        // Two disjoint query-plan ranges. The first saved child holds the
        // continuation; every leaf in the second range must start fresh.
        let plan = plan_with_ranges(vec![qr("", "55"), qr("80", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![
            Ok(vec![rr("", "30", "pk-a"), rr("30", "55", "pk-b")]),
            Ok(vec![rr("80", "C0", "pk-c"), rr("C0", "FF", "pk-d")]),
        ]);

        let resume = saved_drain(vec![
            ("30", "55", saved_request(Some("server-token-xyz"))),
            ("80", "C0", saved_request(None)),
            ("C0", "FF", saved_request(None)),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[
                ("30", "55", "pk-b", "30", "55", Some("server-token-xyz")),
                ("80", "C0", "pk-c", "80", "C0", None),
                ("C0", "FF", "pk-d", "C0", "FF", None),
            ],
        );
    }

    #[tokio::test]
    async fn resume_with_cursor_past_topology_yields_drained_pipeline() {
        // Wire form `SequentialDrain { cursor = "FF", active_tokens = [] }`
        // means every range has been drained: the cursor is at or past
        // the last topology max, and no range above it owes a token.
        // The planner emits no leaves → pipeline is drained.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-a")])]);

        let resume = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: "FF".to_owned(),
            active_tokens: vec![],
        };

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert!(matches!(
            pipeline.snapshot_state().unwrap(),
            PipelineNodeState::Drained
        ));
    }

    #[tokio::test]
    async fn resume_on_merged_range_splits_resumed_slice_and_tail() {
        // Two saved children: `[55, AA)` with a token, `[AA, FF)` without.
        // Between sessions the topology merged into one wide `[, FF)` range;
        // each saved child intersects the merged range and produces its own
        // leaf, preserving the token/no-token distinction.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-merged")])]);

        let resume = saved_drain(vec![
            ("55", "AA", saved_request(Some("server-token-xyz"))),
            ("AA", "FF", saved_request(None)),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();

        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[
                ("55", "AA", "pk-merged", "", "FF", Some("server-token-xyz")),
                ("AA", "FF", "pk-merged", "", "FF", None),
            ],
        );
    }

    #[tokio::test]
    async fn resume_validates_saved_children_sorted_non_overlapping() {
        // Out-of-order active_tokens: [55, AA) then [00, 55) violates
        // strict ascending order.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-a")])]);

        let resume = saved_drain(vec![
            ("55", "AA", saved_request(Some("tok-a"))),
            ("00", "55", saved_request(Some("tok-b"))),
        ]);

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap_err();
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE),
            "expected invalid-children sub-status, got: {err}",
        );
    }

    #[tokio::test]
    async fn resume_validates_saved_children_no_overlap() {
        // Overlapping active_tokens: [00, 80) and [55, FF) overlap on
        // [55, 80).
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-a")])]);

        let resume = saved_drain(vec![
            ("00", "80", saved_request(Some("tok-a"))),
            ("55", "FF", saved_request(Some("tok-b"))),
        ]);

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap_err();
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE),
            "expected invalid-children sub-status, got: {err}",
        );
    }

    #[tokio::test]
    async fn resume_errors_when_non_drained_saved_range_unhonored() {
        // Saved child `[55, AA)` holds a continuation, but the topology
        // only covers `[00, 40)`. The planner cannot honor the saved
        // continuation without risking duplicate emission or data loss.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "40", "pk-a")])]);

        let resume = saved_drain(vec![("55", "AA", saved_request(Some("server-token-xyz")))]);

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap_err();
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED),
            "expected saved-range-unhonored sub-status, got: {err}",
        );
    }

    #[tokio::test]
    async fn resume_with_cursor_skips_drained_prefix_and_fresh_starts_uncovered_tail() {
        // Sparse semantics: the cursor marks the end of the drained
        // prefix. Anything above the cursor that has no active token is
        // implicitly fresh-start — there's no "drained range past the
        // cursor" in the sparse encoding. So with cursor="55", one
        // active token covering [55, AA), and a topology of three
        // resolved ranges [, 55), [55, AA), [AA, FF), the planner must:
        //   - skip [, 55)        (fully below cursor → drained prefix)
        //   - emit Request(tok)  for [55, AA)  (overlaps active token)
        //   - emit Request(None) for [AA, FF)  (above cursor, no active
        //                                       token → fresh-start)
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "55", "pk-a"),
            rr("55", "AA", "pk-b"),
            rr("AA", "FF", "pk-c"),
        ])]);

        let resume = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: "55".to_owned(),
            active_tokens: vec![RangedToken {
                min_epk: "55".to_owned(),
                max_epk: "AA".to_owned(),
                server_continuation: "server-token-xyz".to_owned(),
            }],
        };

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[
                ("55", "AA", "pk-b", "55", "AA", Some("server-token-xyz")),
                ("AA", "FF", "pk-c", "AA", "FF", None),
            ],
        );
    }

    #[tokio::test]
    async fn resume_in_predicate_drops_point_partition_below_cursor() {
        // Regression for issues #4574 / #4638 resume path (Option B): an
        // `IN (@a, @b)` whose values hash into two different partitions, resumed
        // with a cursor that has fully drained the first point's window. Each
        // point normalizes to `[X, successor(X))`. The first window lies entirely
        // at/below the cursor and is dropped; only the second point's window is
        // emitted (fresh-start, no token).
        let plan = plan_with_ranges(vec![
            QueryRange {
                min: "20".to_string(),
                max: "20".to_string(),
                is_min_inclusive: true,
                is_max_inclusive: true,
            },
            QueryRange {
                min: "C0".to_string(),
                max: "C0".to_string(),
                is_min_inclusive: true,
                is_max_inclusive: true,
            },
        ]);
        let op = cross_partition_query_operation();
        // One resolve_ranges call per query range: "20" → left, "C0" → right.
        let mut topology = MockTopologyProvider::new(vec![
            Ok(vec![rr("", "80", "pk-left")]),
            Ok(vec![rr("80", "FF", "pk-right")]),
        ]);

        // Cursor at "80": the left window `[20, successor(20))` is fully drained.
        let resume = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: "80".to_owned(),
            active_tokens: vec![],
        };
        let resume = serde_json::from_str(&serde_json::to_string(&resume).unwrap()).unwrap();

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        // Left window dropped (at/below cursor); right window emitted fresh-start.
        let sc0 = EffectivePartitionKey::from("C0")
            .normalized_successor(16)
            .to_hex();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[("C0", sc0.as_str(), "pk-right", "80", "FF", None)],
        );
    }

    #[tokio::test]
    async fn resume_in_predicate_colocated_windows_carry_their_continuations() {
        // Resume path, Option B: an `IN (@a, @b)` whose values are co-located in
        // ONE partition. Each equality value is its own `[X, successor(X))` EPK
        // window with an independent server continuation, so the saved snapshot
        // carries one token per window. On resume each window re-emits carrying
        // its own token — disjoint windows, no de-duplication.
        let plan = plan_with_ranges(vec![
            QueryRange {
                min: "20".to_string(),
                max: "20".to_string(),
                is_min_inclusive: true,
                is_max_inclusive: true,
            },
            QueryRange {
                min: "50".to_string(),
                max: "50".to_string(),
                is_min_inclusive: true,
                is_max_inclusive: true,
            },
        ]);
        let op = cross_partition_query_operation();
        // Both resumed point fragments resolve to the same single partition.
        let mut topology = MockTopologyProvider::new(vec![
            Ok(vec![rr("", "FF", "pk-0")]),
            Ok(vec![rr("", "FF", "pk-0")]),
        ]);

        let s20 = EffectivePartitionKey::from("20")
            .normalized_successor(16)
            .to_hex();
        let s50 = EffectivePartitionKey::from("50")
            .normalized_successor(16)
            .to_hex();

        // Each in-flight window has its own saved server continuation.
        let resume = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: "".to_owned(),
            active_tokens: vec![
                RangedToken {
                    min_epk: "20".to_owned(),
                    max_epk: s20.clone(),
                    server_continuation: "tok-a".to_owned(),
                },
                RangedToken {
                    min_epk: "50".to_owned(),
                    max_epk: s50.clone(),
                    server_continuation: "tok-b".to_owned(),
                },
            ],
        };

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[
                ("20", s20.as_str(), "pk-0", "", "FF", Some("tok-a")),
                ("50", s50.as_str(), "pk-0", "", "FF", Some("tok-b")),
            ],
        );
    }

    #[tokio::test]
    async fn resume_multiple_saved_children_in_one_resolved_range_no_duplicate_leaves() {
        // The topology has merged the saved children into one wide range.
        // Each active token produces exactly one leaf scoped to its own
        // range, and the trailing portion of the merged range above the
        // last active token is emitted as a single fresh-start leaf
        // covering the gap to the topology max (sparse semantics:
        // ranges above the cursor not covered by an active token are
        // implicitly fresh-start).
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-merged")])]);

        let resume = saved_drain(vec![
            ("10", "30", saved_request(Some("tok-a"))),
            ("30", "60", saved_request(Some("tok-b"))),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[
                ("10", "30", "pk-merged", "", "FF", Some("tok-a")),
                ("30", "60", "pk-merged", "", "FF", Some("tok-b")),
                ("60", "FF", "pk-merged", "", "FF", None),
            ],
        );
    }

    #[tokio::test]
    async fn resume_emits_fresh_leaves_for_topology_gaps_above_cursor() {
        // Sparse semantics: any topology range above the cursor that is
        // NOT covered by an active token is fresh-start (not drained).
        // This is the O(S) trade-off — only ranges below the cursor are
        // skipped as drained. With cursor="40", one active token at
        // [40, 60), and topology [, 20), [20, 40), [40, 60), [60, 80),
        // [80, FF), the planner emits:
        //   - skip [, 20)        (below cursor)
        //   - skip [20, 40)      (below cursor)
        //   - Request(tok)       for [40, 60)
        //   - Request(None)      for [60, 80)  (fresh-start, no token)
        //   - Request(None)      for [80, FF)  (fresh-start, no token)
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "20", "pk-a"),
            rr("20", "40", "pk-b"),
            rr("40", "60", "pk-c"),
            rr("60", "80", "pk-d"),
            rr("80", "FF", "pk-e"),
        ])]);

        let resume = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: "40".to_owned(),
            active_tokens: vec![RangedToken {
                min_epk: "40".to_owned(),
                max_epk: "60".to_owned(),
                server_continuation: "tok".to_owned(),
            }],
        };

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[
                ("40", "60", "pk-c", "40", "60", Some("tok")),
                ("60", "80", "pk-d", "60", "80", None),
                ("80", "FF", "pk-e", "80", "FF", None),
            ],
        );
    }

    /// An older serialized shape — a top-level bare `Request` continuation
    /// for what is now a `SequentialDrain` — must be rejected on resume
    /// rather than silently re-interpreted as a full-range cursor. Guards
    /// the planner's existing rejection of that shape.
    #[tokio::test]
    async fn legacy_top_level_bare_request_shape_fails_to_resume() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-0")])]);

        let legacy = PipelineNodeState::Request {
            server_continuation: Some("OLD".to_owned()),
        };

        let result =
            build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(legacy)).await;
        let err = result.expect_err("bare top-level Request shape must be rejected on resume");
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH,
            "expected SHAPE_MISMATCH for top-level bare Request shape; got {err:?}",
        );
    }

    /// Zero-width active_tokens entries are well-formed JSON but cannot
    /// carry remaining work. They must be rejected with a message that
    /// points at the entry itself rather than at a downstream "could not
    /// be fully covered" error.
    #[tokio::test]
    async fn rejects_zero_width_saved_child_entry_with_clear_message() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-0")])]);

        let resume = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: String::new(),
            active_tokens: vec![RangedToken {
                min_epk: "40".to_owned(),
                max_epk: "40".to_owned(),
                server_continuation: "tok".to_owned(),
            }],
        };

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .expect_err("zero-width active_tokens entry must be rejected");
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE,
        );
        let rendered = err.to_string();
        assert!(
            rendered.contains("zero-width"),
            "error message should describe the zero-width entry; got: {rendered}"
        );
    }

    /// The continuation-token validator must reject an `active_tokens`
    /// entry with `min >= max` regardless of how that wire payload was
    /// produced (corrupted token, hand-rolled, future-version rollback).
    /// Pins the validator behavior so a future change to the comparison
    /// semantics (e.g., `EffectivePartitionKey::Ord`) can't silently
    /// downgrade this fail-loud path to a silent re-query.
    #[tokio::test]
    async fn malformed_min_greater_than_max_child_is_rejected_by_validator() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-0")])]);

        let resume = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: String::new(),
            active_tokens: vec![RangedToken {
                min_epk: "FF".to_owned(),
                max_epk: "00".to_owned(),
                server_continuation: "tok".to_owned(),
            }],
        };

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .expect_err("malformed min>max entry must be rejected by the validator");
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE,
            "malformed min>max entry must trip the EPK-range validator path",
        );
    }

    /// Companion to the test above: when a malformed `min >= max` entry
    /// is *appended* to legitimate entries, the validator must still
    /// reject the whole payload. Guards against a future "skip invalid
    /// entries, continue with the valid ones" relaxation that would
    /// silently swallow snapshot corruption.
    #[tokio::test]
    async fn malformed_min_greater_than_max_appended_to_valid_children_still_rejects() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-0")])]);

        let resume = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: String::new(),
            active_tokens: vec![
                RangedToken {
                    min_epk: String::new(),
                    max_epk: "80".to_owned(),
                    server_continuation: "real-token".to_owned(),
                },
                RangedToken {
                    min_epk: "FF".to_owned(),
                    max_epk: "00".to_owned(),
                    server_continuation: "tok".to_owned(),
                },
            ],
        };

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .expect_err("appended malformed min>max entry must still be rejected");
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE,
        );
    }

    /// Symmetric variant of the cascading-split scenario — the FRONT
    /// sibling splits between snapshots instead of the back one. The
    /// planner's interval-join logic is symmetric in the two siblings,
    /// so this test guards against an accidental asymmetry (e.g.,
    /// assuming the "still-pending" sibling is always the back one)
    /// that would be invisible to the existing back-split test.
    #[tokio::test]
    async fn cascading_split_of_front_sibling_propagates_token_to_grand_children() {
        // Saved state: cursor at start, active_tokens has one entry for
        // [, 80) owing T1 (front sibling is in progress). The back range
        // [80, FF) is not in active_tokens, so it's implicitly fresh-start.
        // Then topology resolves the front into two grand-children
        // [, 40) + [40, 80) on top of the unchanged back [80, FF).
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "40", "pkrange-front-left"),
            rr("40", "80", "pkrange-front-right"),
            rr("80", "FF", "pkrange-back"),
        ])]);

        let resume = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: String::new(),
            active_tokens: vec![RangedToken {
                min_epk: String::new(),
                max_epk: "80".to_owned(),
                server_continuation: "T1".to_owned(),
            }],
        };

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .expect("front-sibling cascading split must plan cleanly");

        // Walk the planned children via snapshot: the two front grand-
        // children must each carry T1; the back range must be a
        // fresh-start leaf (implicit — appears in the planned children
        // but not in active_tokens). The exact structure mirrors the
        // back-split case in `query_resume_integration_tests::
        // cascading_split_..._grand_child`.
        let snap = pipeline.snapshot_state().unwrap();
        let (cursor, active_tokens) = match snap {
            PipelineNodeState::SequentialDrain {
                left_most_undrained_epk,
                active_tokens,
            } => (left_most_undrained_epk, active_tokens),
            other => panic!("expected SequentialDrain, got {other:?}"),
        };
        assert_eq!(cursor, "", "cursor must remain at start");
        assert_eq!(
            active_tokens.len(),
            2,
            "expected 2 active tokens for the front grand-children, got {active_tokens:?}",
        );
        for (idx, expected_min, expected_max) in [(0, "", "40"), (1, "40", "80")] {
            assert_eq!(
                active_tokens[idx].min_epk, expected_min,
                "active_tokens[{idx}] min_epk mismatch",
            );
            assert_eq!(
                active_tokens[idx].max_epk, expected_max,
                "active_tokens[{idx}] max_epk mismatch",
            );
            assert_eq!(
                active_tokens[idx].server_continuation, "T1",
                "front grand-child {idx} must carry T1",
            );
        }
    }

    // ── Streaming ORDER BY selection and validation ───────────────────────

    fn order_by_query_info(rewritten_query: Option<&str>) -> QueryInfo {
        QueryInfo {
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.rank".to_owned()],
            rewritten_query: rewritten_query.map(str::to_owned),
            ..Default::default()
        }
    }

    fn order_by_plan(rewritten_query: Option<&str>, ranges: Vec<QueryRange>) -> QueryPlan {
        QueryPlan {
            partitioned_query_execution_info_version: 2,
            query_info: Some(order_by_query_info(rewritten_query)),
            query_ranges: ranges,
            hybrid_search_query_info: None,
        }
    }

    fn order_by_operation() -> CosmosOperation {
        CosmosOperation::query_items(test_container(), Some(FeedRange::full()))
            .with_body(br#"{"query":"SELECT * FROM c ORDER BY c.rank","parameters":[]}"#.to_vec())
    }

    #[test]
    fn is_streaming_order_by_true_only_for_non_empty_streaming_order_by() {
        assert!(!is_streaming_order_by(&QueryInfo::default()));
        assert!(is_streaming_order_by(&order_by_query_info(Some(
            "SELECT 1"
        ))));

        let mut non_streaming = order_by_query_info(Some("SELECT 1"));
        non_streaming.has_non_streaming_order_by = true;
        assert!(!is_streaming_order_by(&non_streaming));
        assert!(is_non_streaming_order_by(&non_streaming));
    }

    fn non_streaming_order_by_operation() -> CosmosOperation {
        CosmosOperation::query_items(test_container(), Some(FeedRange::full())).with_body(
            br#"{"query":"SELECT TOP 5 c.id FROM c ORDER BY c.rank, c.tie DESC","parameters":[]}"#
                .to_vec(),
        )
    }

    fn non_streaming_order_by_plan() -> QueryPlan {
        QueryPlan {
            partitioned_query_execution_info_version: 2,
            query_info: Some(QueryInfo {
                top: Some(5),
                order_by: vec![SortOrder::Ascending, SortOrder::Descending],
                order_by_expressions: vec!["c.rank".to_owned(), "c.tie".to_owned()],
                rewritten_query: Some(
                    "SELECT c._rid, [{\"item\": c.rank}, {\"item\": c.tie}] AS orderByItems, c.id AS payload FROM c ORDER BY c.rank, c.tie DESC"
                        .to_owned(),
                ),
                has_non_streaming_order_by: true,
                ..Default::default()
            }),
            query_ranges: vec![qr("", "FF")],
            hybrid_search_query_info: None,
        }
    }

    #[tokio::test]
    async fn build_non_streaming_ordered_merge_trusts_plan_metadata() {
        let operation = Arc::new(non_streaming_order_by_operation());
        let plan = non_streaming_order_by_plan();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "80", "pk-left"),
            rr("80", "FF", "pk-right"),
        ])]);

        let pipeline = build_non_streaming_ordered_merge(&plan, &mut topology, &operation, None)
            .await
            .unwrap();
        let merge = pipeline
            .into_root()
            .downcast::<crate::driver::dataflow::NonStreamingOrderedMerge>()
            .expect("root must be a NonStreamingOrderedMerge");
        let mut children = merge.into_children();
        let drain = children
            .pop()
            .unwrap()
            .downcast::<crate::driver::dataflow::SequentialDrain>()
            .expect("buffered merge child must be a SequentialDrain");
        assert_eq!(drain.into_children().len(), 2);
    }

    #[tokio::test]
    async fn build_non_streaming_ordered_merge_requires_finite_window() {
        let operation = Arc::new(non_streaming_order_by_operation());
        let mut plan = non_streaming_order_by_plan();
        plan.query_info.as_mut().unwrap().top = None;
        let mut topology = MockTopologyProvider::new(Vec::new());

        let err = build_non_streaming_ordered_merge(&plan, &mut topology, &operation, None)
            .await
            .unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_REQUIRES_FINITE_WINDOW
        );
    }

    #[tokio::test]
    async fn build_non_streaming_ordered_merge_accepts_large_finite_window() {
        let operation = Arc::new(non_streaming_order_by_operation());
        let mut plan = non_streaming_order_by_plan();
        let info = plan.query_info.as_mut().unwrap();
        info.top = None;
        info.offset = Some(50_000);
        info.limit = Some(3);
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-range")])]);

        let pipeline = build_non_streaming_ordered_merge(&plan, &mut topology, &operation, None)
            .await
            .expect("finite windows are not capped by the client");
        assert!(pipeline
            .into_root()
            .downcast::<crate::driver::dataflow::NonStreamingOrderedMerge>()
            .is_some());
    }

    #[tokio::test]
    async fn build_non_streaming_ordered_merge_rejects_resume_and_streaming_plan() {
        let operation = Arc::new(non_streaming_order_by_operation());
        let plan = non_streaming_order_by_plan();
        let mut topology = MockTopologyProvider::new(Vec::new());
        let err = build_non_streaming_ordered_merge(
            &plan,
            &mut topology,
            &operation,
            Some(PipelineNodeState::Drained),
        )
        .await
        .unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_CONTINUATION_UNSUPPORTED
        );

        let mut streaming_plan = non_streaming_order_by_plan();
        streaming_plan
            .query_info
            .as_mut()
            .unwrap()
            .has_non_streaming_order_by = false;
        let err =
            build_non_streaming_ordered_merge(&streaming_plan, &mut topology, &operation, None)
                .await
                .unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_UNSUPPORTED_QUERY_FEATURE
        );
    }

    #[tokio::test]
    async fn build_non_streaming_ordered_merge_rejects_distinct() {
        let operation = Arc::new(non_streaming_order_by_operation());
        let mut plan = non_streaming_order_by_plan();
        plan.query_info.as_mut().unwrap().distinct_type = DistinctType::Unordered;
        let mut topology = MockTopologyProvider::new(Vec::new());

        let err = build_non_streaming_ordered_merge(&plan, &mut topology, &operation, None)
            .await
            .unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_UNSUPPORTED_QUERY_FEATURE
        );
    }

    #[test]
    fn validate_query_plan_for_streaming_order_by_accepts_plain_order_by() {
        let plan = order_by_plan(Some("SELECT 1"), vec![qr("", "FF")]);
        assert!(validate_query_plan_for_streaming_order_by(&plan).is_ok());
    }

    #[test]
    fn validate_query_plan_for_streaming_order_by_rejects_non_streaming_order_by() {
        let mut plan = order_by_plan(Some("SELECT 1"), vec![qr("", "FF")]);
        plan.query_info.as_mut().unwrap().has_non_streaming_order_by = true;
        let err = validate_query_plan_for_streaming_order_by(&plan).unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_UNSUPPORTED_QUERY_FEATURE
        );
    }

    #[test]
    fn validate_query_plan_for_streaming_order_by_accepts_top() {
        // TOP combined with streaming ORDER BY is now supported: the ordered
        // merge streams sorted rows and a `SkipTake` root applies the window.
        let mut plan = order_by_plan(Some("SELECT 1"), vec![qr("", "FF")]);
        plan.query_info.as_mut().unwrap().top = Some(5);
        assert!(validate_query_plan_for_streaming_order_by(&plan).is_ok());
    }

    #[test]
    fn validate_query_plan_for_streaming_order_by_accepts_offset_limit() {
        // OFFSET/LIMIT combined with streaming ORDER BY is now supported.
        let mut plan = order_by_plan(Some("SELECT 1"), vec![qr("", "FF")]);
        plan.query_info.as_mut().unwrap().offset = Some(1);
        assert!(validate_query_plan_for_streaming_order_by(&plan).is_ok());

        let mut plan = order_by_plan(Some("SELECT 1"), vec![qr("", "FF")]);
        plan.query_info.as_mut().unwrap().limit = Some(1);
        assert!(validate_query_plan_for_streaming_order_by(&plan).is_ok());
    }

    #[test]
    fn validate_query_plan_for_streaming_order_by_rejects_aggregates_and_group_by() {
        let mut plan = order_by_plan(Some("SELECT 1"), vec![qr("", "FF")]);
        plan.query_info.as_mut().unwrap().aggregates = vec!["Count".to_owned()];
        assert!(validate_query_plan_for_streaming_order_by(&plan).is_err());

        let mut plan = order_by_plan(Some("SELECT 1"), vec![qr("", "FF")]);
        plan.query_info.as_mut().unwrap().group_by_expressions = vec!["c.a".to_owned()];
        assert!(validate_query_plan_for_streaming_order_by(&plan).is_err());
    }

    /// DISTINCT is now composed as a stage above the merge rather than
    /// rejected, so plan validation must accept it in both forms.
    #[test]
    fn validate_query_plan_for_streaming_order_by_accepts_distinct() {
        for distinct_type in [DistinctType::Ordered, DistinctType::Unordered] {
            let mut plan = order_by_plan(Some("SELECT 1"), vec![qr("", "FF")]);
            plan.query_info.as_mut().unwrap().distinct_type = distinct_type;
            assert!(
                validate_query_plan_for_streaming_order_by(&plan).is_ok(),
                "{distinct_type:?} DISTINCT must be accepted alongside ORDER BY"
            );
        }
    }

    // ── DISTINCT continuation-token validation ───────────────────────────
    //
    // The catalog scenarios `malformed_token_rejected`,
    // `token_shape_mismatch_rejected`, and
    // `distinct_type_mismatch_on_resume_rejected` are pinned here, since they
    // are about token shape rather than page contents.

    fn distinct_state(distinct_type: DistinctType) -> PipelineNodeState {
        PipelineNodeState::Distinct {
            distinct_type,
            last_hash: None,
            child: Box::new(PipelineNodeState::SequentialDrain {
                left_most_undrained_epk: String::new(),
                active_tokens: Vec::new(),
            }),
        }
    }

    #[test]
    fn peel_distinct_resume_unwraps_a_matching_ordered_token() {
        let (inner, last_hash) = peel_distinct_resume(
            Some(distinct_state(DistinctType::Ordered)),
            DistinctType::Ordered,
        )
        .expect("a matching ordered token resumes");
        assert!(matches!(
            inner,
            Some(PipelineNodeState::SequentialDrain { .. })
        ));
        assert_eq!(last_hash, None);
    }

    /// Catalog: `distinct_type_mismatch_on_resume_rejected`. Reinterpreting an
    /// ordered token as unordered would apply adjacency deduplication to an
    /// unsorted stream.
    #[test]
    fn peel_distinct_resume_rejects_a_distinct_type_mismatch() {
        let err = peel_distinct_resume(
            Some(distinct_state(DistinctType::Ordered)),
            DistinctType::Unordered,
        )
        .expect_err("an ordered token must not resume an unordered plan");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH)
        );
        assert!(err.to_string().contains("minted for"));
    }

    /// We never mint an unordered DISTINCT token, so one can only be
    /// hand-crafted or corrupted — and resuming it would re-emit every value
    /// seen before the checkpoint.
    #[test]
    fn peel_distinct_resume_rejects_a_hand_crafted_unordered_token() {
        let err = peel_distinct_resume(
            Some(distinct_state(DistinctType::Unordered)),
            DistinctType::Unordered,
        )
        .expect_err("an unordered DISTINCT token is never resumable");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED)
        );
        assert!(err.to_string().contains("ORDER BY"));
    }

    /// Catalog: `token_shape_mismatch_rejected`. A token minted before DISTINCT
    /// support (or for a non-DISTINCT query) has no `Distinct` layer; resuming
    /// it would silently skip deduplication for every remaining page.
    #[test]
    fn peel_distinct_resume_rejects_a_token_without_a_distinct_layer() {
        let err = peel_distinct_resume(
            Some(PipelineNodeState::SequentialDrain {
                left_most_undrained_epk: String::new(),
                active_tokens: Vec::new(),
            }),
            DistinctType::Unordered,
        )
        .expect_err("a non-DISTINCT token must not resume a DISTINCT plan");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH)
        );
        assert!(err.to_string().contains("does not match a DISTINCT query"));
    }

    /// The mirror case: a `Distinct` token handed to a plan that no longer asks
    /// for deduplication is rejected rather than reinterpreted.
    #[test]
    fn peel_distinct_resume_rejects_a_distinct_token_for_a_plain_plan() {
        let err = peel_distinct_resume(
            Some(distinct_state(DistinctType::Ordered)),
            DistinctType::None,
        )
        .expect_err("a DISTINCT token cannot resume a plain plan");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH)
        );
    }

    /// `Drained` is shape-agnostic: the whole pipeline, DISTINCT included,
    /// finished, so it must resume for any plan shape.
    #[test]
    fn peel_distinct_resume_passes_drained_through_unchanged() {
        for distinct_type in [
            DistinctType::None,
            DistinctType::Ordered,
            DistinctType::Unordered,
        ] {
            let (inner, last_hash) =
                peel_distinct_resume(Some(PipelineNodeState::Drained), distinct_type)
                    .expect("a drained token resumes for any shape");
            assert!(matches!(inner, Some(PipelineNodeState::Drained)));
            assert_eq!(last_hash, None);
        }
    }

    #[test]
    fn peel_distinct_resume_passes_a_fresh_start_through() {
        let (inner, last_hash) = peel_distinct_resume(None, DistinctType::Unordered)
            .expect("a fresh start needs no token");
        assert!(inner.is_none());
        assert_eq!(last_hash, None);
    }

    /// Catalog: `malformed_token_rejected`. Wrapping the fan-out state in a
    /// `Distinct` layer must not bypass the inner state's own validation — a
    /// malformed child is still rejected rather than silently restarting the
    /// query (which would re-emit every row).
    #[tokio::test]
    async fn distinct_token_with_a_malformed_child_is_still_rejected() {
        let mut plan = order_by_plan(Some("SELECT 1"), vec![qr("", "FF")]);
        plan.query_info.as_mut().unwrap().distinct_type = DistinctType::Ordered;
        let operation = Arc::new(order_by_operation());
        let mut topology = MockTopologyProvider::new(vec![]);

        // A `SequentialDrain` child is the wrong shape under a streaming
        // ORDER BY plan, and the inner builder must say so.
        let malformed = PipelineNodeState::Distinct {
            distinct_type: DistinctType::Ordered,
            last_hash: None,
            child: Box::new(PipelineNodeState::SequentialDrain {
                left_most_undrained_epk: String::new(),
                active_tokens: Vec::new(),
            }),
        };
        let err = build_streaming_ordered_merge(&plan, &mut topology, &operation, Some(malformed))
            .await
            .expect_err("a malformed inner state must not resume");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH)
        );
    }

    #[test]
    fn validate_query_plan_for_streaming_order_by_rejects_hybrid_search() {
        let mut plan = order_by_plan(Some("SELECT 1"), vec![qr("", "FF")]);
        plan.hybrid_search_query_info =
            Some(crate::driver::dataflow::query_plan::HybridSearchQueryInfo {
                global_statistics_query: String::new(),
                component_query_infos: vec![],
                component_weights: vec![],
                skip: None,
                take: None,
                requires_global_statistics: false,
            });
        assert!(validate_query_plan_for_streaming_order_by(&plan).is_err());
    }

    #[tokio::test]
    async fn build_streaming_ordered_merge_rejects_missing_rewritten_query() {
        let op = Arc::new(order_by_operation());
        let plan = order_by_plan(None, vec![qr("", "FF")]);
        let mut topology = MockTopologyProvider::new(vec![]);
        let err = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
            .await
            .unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::SERVICE_QUERY_PLAN_ORDER_BY_MISSING_REWRITTEN_QUERY
        );
    }

    #[tokio::test]
    async fn build_streaming_ordered_merge_rejects_empty_rewritten_query() {
        let op = Arc::new(order_by_operation());
        let plan = order_by_plan(Some(""), vec![qr("", "FF")]);
        let mut topology = MockTopologyProvider::new(vec![]);
        let err = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
            .await
            .unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::SERVICE_QUERY_PLAN_ORDER_BY_MISSING_REWRITTEN_QUERY
        );
    }

    #[tokio::test]
    async fn build_streaming_ordered_merge_builds_one_child_per_resolved_range() {
        let op = Arc::new(order_by_operation());
        let plan = order_by_plan(Some("SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c ORDER BY c.rank ASC"), vec![qr("", "FF")]);
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "80", "pk-left"),
            rr("80", "FF", "pk-right"),
        ])]);

        let pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
            .await
            .unwrap();
        let root = pipeline.into_root();
        let merge = root
            .downcast::<crate::driver::dataflow::StreamingOrderedMerge>()
            .expect("root must be a StreamingOrderedMerge");
        let children = merge.into_children();
        assert_eq!(
            children.len(),
            2,
            "one child per resolved physical partition"
        );
    }

    #[tokio::test]
    async fn build_streaming_ordered_merge_wraps_skip_take_for_combined_offset_limit() {
        // ORDER BY combined with OFFSET/LIMIT must compose a `SkipTake` root
        // over the ordered merge so the window is applied once, globally.
        let op = Arc::new(order_by_operation());
        let mut plan = order_by_plan(Some("SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c ORDER BY c.rank ASC"), vec![qr("", "FF")]);
        {
            let info = plan.query_info.as_mut().unwrap();
            info.offset = Some(2);
            info.limit = Some(3);
        }
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-0")])]);
        let pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
            .await
            .expect("combined ORDER BY + OFFSET/LIMIT must build");
        let skip_take = pipeline
            .into_root()
            .downcast::<crate::driver::dataflow::SkipTake>()
            .expect("combined ORDER BY + OFFSET/LIMIT must wrap the ordered merge in a SkipTake");
        let mut children = skip_take.into_children();
        assert_eq!(
            children.len(),
            1,
            "a SkipTake wraps exactly one ordered-merge child"
        );
        children
            .pop()
            .unwrap()
            .downcast::<crate::driver::dataflow::StreamingOrderedMerge>()
            .expect("the SkipTake's child must be the StreamingOrderedMerge");
    }

    #[tokio::test]
    async fn build_streaming_ordered_merge_wraps_skip_take_for_combined_top() {
        // ORDER BY combined with TOP must also compose a `SkipTake` root.
        let op = Arc::new(order_by_operation());
        let mut plan = order_by_plan(Some("SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c ORDER BY c.rank ASC"), vec![qr("", "FF")]);
        plan.query_info.as_mut().unwrap().top = Some(4);
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-0")])]);
        let pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
            .await
            .expect("combined ORDER BY + TOP must build");
        assert!(
            pipeline
                .into_root()
                .downcast::<crate::driver::dataflow::SkipTake>()
                .is_some(),
            "combined ORDER BY + TOP must wrap the ordered merge in a SkipTake"
        );
    }

    #[tokio::test]
    async fn build_streaming_ordered_merge_no_wrap_for_plain_order_by() {
        // A plain ORDER BY (no window) leaves the ordered merge as the root.
        let op = Arc::new(order_by_operation());
        let plan = order_by_plan(Some("SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c ORDER BY c.rank ASC"), vec![qr("", "FF")]);
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-0")])]);
        let pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
            .await
            .unwrap();
        assert!(
            pipeline
                .into_root()
                .downcast::<crate::driver::dataflow::StreamingOrderedMerge>()
                .is_some(),
            "a plain ORDER BY must not be wrapped in a SkipTake"
        );
    }

    #[tokio::test]
    async fn build_streaming_ordered_merge_accepts_cross_construct_window_token() {
        // A `TOP`-shaped window token resumed against an `OFFSET`/`LIMIT` combined
        // ORDER BY query builds the identical SkipTake-over-merge pipeline, so it
        // resumes (resume validates pipeline shape, not the SQL construct).
        let op = Arc::new(order_by_operation());
        let mut plan = order_by_plan(Some("SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c ORDER BY c.rank ASC"), vec![qr("", "FF")]);
        {
            let info = plan.query_info.as_mut().unwrap();
            info.offset = Some(1);
            info.limit = Some(2);
        }
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-0")])]);
        let resume = PipelineNodeState::SkipTake {
            remaining_skip: 0,
            remaining_take: Some(2),
            child: Box::new(PipelineNodeState::Drained),
        };
        let pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, Some(resume))
            .await
            .expect("a window token should resume a combined ORDER BY window query");
        // The saved child was `Drained`, so the resumed pipeline is drained.
        let mut root = pipeline.into_root();
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        assert!(matches!(
            root.next_page(&mut context).await.unwrap(),
            PageResult::Drained
        ));
    }

    #[tokio::test]
    async fn build_streaming_ordered_merge_rejects_window_token_against_plain_order_by() {
        // A skip/take window token resumed against a *plain* ORDER BY (no
        // OFFSET/LIMIT/TOP) is a pipeline-shape mismatch: the resumed pipeline
        // has no SkipTake node to receive the window. Must be rejected.
        let op = Arc::new(order_by_operation());
        let plan = order_by_plan(Some("SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c ORDER BY c.rank ASC"), vec![qr("", "FF")]);
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-0")])]);
        let resume = PipelineNodeState::SkipTake {
            remaining_skip: 0,
            remaining_take: Some(2),
            child: Box::new(PipelineNodeState::Drained),
        };
        let err = build_streaming_ordered_merge(&plan, &mut topology, &op, Some(resume))
            .await
            .unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH
        );
    }

    #[tokio::test]
    async fn build_streaming_ordered_merge_empty_ranges_errors_on_fresh_start() {
        let op = Arc::new(order_by_operation());
        // Empty `query_ranges` exercises the "produced nothing" guard.
        let plan = order_by_plan(Some("SELECT 1"), vec![]);
        let mut topology = MockTopologyProvider::new(vec![]);
        let err = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
            .await
            .unwrap_err();
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES
        );
    }

    /// An operation whose SQL body carries `sql` as the `query` field.
    fn order_by_operation_with_query(sql: &str) -> CosmosOperation {
        let body = serde_json::json!({ "query": sql, "parameters": [] });
        CosmosOperation::query_items(test_container(), Some(FeedRange::full()))
            .with_body(serde_json::to_vec(&body).unwrap())
    }

    #[tokio::test]
    async fn build_streaming_ordered_merge_accepts_join_query() {
        // A JOIN can emit multiple rows per document `_rid`; the resume cursor
        // now tracks that with a `_rid`-tie skip count (mirroring .NET), so a
        // JOIN-shaped ORDER BY builds instead of being rejected up front.
        let op = Arc::new(order_by_operation_with_query(
            "SELECT * FROM c JOIN t IN c.tags ORDER BY c.rank",
        ));
        let plan = order_by_plan(
            Some("SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c JOIN t IN c.tags ORDER BY c.rank ASC"),
            vec![qr("", "FF")],
        );
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-0")])]);
        let pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
            .await
            .expect("a JOIN-shaped ORDER BY must build a pipeline");
        assert!(
            pipeline
                .into_root()
                .downcast::<crate::driver::dataflow::StreamingOrderedMerge>()
                .is_some(),
            "a JOIN-shaped ORDER BY builds a StreamingOrderedMerge"
        );
    }

    #[tokio::test]
    async fn build_streaming_ordered_merge_accepts_ordinary_parameterized_order_by() {
        // An ordinary single-source, parameterized ORDER BY builds normally
        // (no local SQL parsing gate stands in the way).
        let op = Arc::new(order_by_operation_with_query(
            "SELECT * FROM c WHERE c.tenant = @t ORDER BY c.rank",
        ));
        let plan = order_by_plan(
            Some("SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c ORDER BY c.rank ASC"),
            vec![qr("", "FF")],
        );
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-0")])]);
        let pipeline = build_streaming_ordered_merge(&plan, &mut topology, &op, None)
            .await
            .expect("ordinary parameterized ORDER BY must build a pipeline");
        assert!(
            pipeline
                .into_root()
                .downcast::<crate::driver::dataflow::StreamingOrderedMerge>()
                .is_some(),
            "ordinary parameterized ORDER BY builds a StreamingOrderedMerge"
        );
    }

    /// A token minted for a different query text (or different parameter
    /// values) must be rejected: the resume filter is built client-side from
    /// the saved boundary, so replaying it against another query silently
    /// returns the wrong rows rather than failing at the service.
    #[test]
    fn streaming_order_by_snapshot_rejects_mismatched_query_fingerprint() {
        let ranges = vec![OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "FF".to_owned(),
            server_continuation: None,
            boundary: None,
        }];
        let err = validate_streaming_order_by_snapshot(
            &[SortOrder::Ascending],
            &[SortOrder::Ascending],
            "current-query",
            Some("other-query"),
            ranges,
        )
        .err()
        .expect("a token minted by a different query must be rejected");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID),
            "got: {err}"
        );
    }

    /// A token minted under one feed scope must not resume under another.
    /// Nothing else binds the two: the resumed node treats its saved ranges as
    /// authoritative, and `is_valid_for_operation` checks only the operation
    /// kind and container RID, so without the scope in the fingerprint the
    /// query would read outside the caller's scope.
    #[tokio::test]
    async fn build_streaming_ordered_merge_rejects_token_from_a_different_feed_scope() {
        let body = br#"{"query":"SELECT * FROM c ORDER BY c.rank","parameters":[]}"#.to_vec();
        let scoped_op = |min: &str, max: &str| {
            Arc::new(
                CosmosOperation::query_items(
                    test_container(),
                    Some(
                        FeedRange::new(
                            EffectivePartitionKey::from(min),
                            EffectivePartitionKey::from(max),
                        )
                        .unwrap(),
                    ),
                )
                .with_body(body.clone()),
            )
        };
        let plan = order_by_plan(
            Some("SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c ORDER BY c.rank ASC"),
            vec![qr("", "FF")],
        );

        // Mint a token under the left half of the key space.
        let minted = {
            let op = scoped_op("", "80");
            let mut topology =
                PhysicalTopologyProvider::new(vec![rr("", "80", "pk-0"), rr("80", "FF", "pk-1")]);
            build_streaming_ordered_merge(&plan, &mut topology, &op, None)
                .await
                .expect("fresh scoped plan must build")
                .into_root()
                .snapshot_state()
                .expect("a fresh merge must snapshot")
        };
        assert!(
            matches!(
                &minted,
                PipelineNodeState::StreamingOrderedMerge {
                    query_fingerprint: Some(_),
                    ..
                }
            ),
            "the minted token must carry a fingerprint: {minted:?}"
        );

        // Replay it against the right half — same query text and parameters.
        // The topology resolves against the requested range, so without the
        // scope check this would succeed and happily query `..80` (the token's
        // scope) while the caller asked for `80..`.
        let op = scoped_op("80", "FF");
        let mut topology =
            PhysicalTopologyProvider::new(vec![rr("", "80", "pk-0"), rr("80", "FF", "pk-1")]);
        let err = build_streaming_ordered_merge(&plan, &mut topology, &op, Some(minted))
            .await
            .err()
            .expect("a token minted under a different feed scope must be rejected");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID),
            "got: {err}"
        );
    }

    /// A legacy token predating `query_fingerprint` carries `None`, so the
    /// fingerprint check cannot catch a scope mismatch. Each saved range must
    /// still be contained in the operation's scope, so such a token can never
    /// read outside it. Rejected rather than clipped: narrowing a saved range
    /// would make a stale pre-split server continuation look replayable to
    /// `build_children`.
    #[tokio::test]
    async fn build_streaming_ordered_merge_rejects_legacy_token_ranges_outside_scope() {
        let op = Arc::new(
            CosmosOperation::query_items(
                test_container(),
                Some(
                    FeedRange::new(
                        EffectivePartitionKey::from(""),
                        EffectivePartitionKey::from("80"),
                    )
                    .unwrap(),
                ),
            )
            .with_body(br#"{"query":"SELECT * FROM c ORDER BY c.rank","parameters":[]}"#.to_vec()),
        );
        // A legacy token spanning the whole key space: one range inside the
        // operation's scope, one entirely outside it.
        let resume = PipelineNodeState::StreamingOrderedMerge {
            directions: vec![SortOrder::Ascending],
            query_fingerprint: None,
            ranges: vec![
                OrderByRangeToken {
                    min_epk: String::new(),
                    max_epk: "80".to_owned(),
                    server_continuation: None,
                    boundary: None,
                },
                OrderByRangeToken {
                    min_epk: "80".to_owned(),
                    max_epk: "FF".to_owned(),
                    server_continuation: None,
                    boundary: None,
                },
            ],
        };
        let plan = order_by_plan(
            Some("SELECT c._rid, [{\"item\":c.rank}] AS orderByItems, c AS payload FROM c ORDER BY c.rank ASC"),
            vec![qr("", "FF")],
        );
        let mut topology =
            PhysicalTopologyProvider::new(vec![rr("", "80", "pk-0"), rr("80", "FF", "pk-1")]);

        let err = build_streaming_ordered_merge(&plan, &mut topology, &op, Some(resume))
            .await
            .expect_err("the `80..FF` saved range lies outside the operation's scope");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID),
            "got: {err}"
        );
    }

    /// A token minted before `query_fingerprint` existed carries `None` and
    /// still resumes — it is validated on `directions` alone.
    #[test]
    fn streaming_order_by_snapshot_accepts_absent_query_fingerprint() {
        let ranges = vec![OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "FF".to_owned(),
            server_continuation: None,
            boundary: None,
        }];
        let parsed = validate_streaming_order_by_snapshot(
            &[SortOrder::Ascending],
            &[SortOrder::Ascending],
            "current-query",
            None,
            ranges,
        )
        .expect("a legacy token without a fingerprint stays resumable");
        assert_eq!(parsed.len(), 1);
    }

    /// A boundary RID that isn't a decodable Cosmos document RID must be
    /// rejected: `compare_document_rids` would fall back to raw-string order,
    /// which is not monotonic in document ordinal, so the discard pass would
    /// drop or keep the wrong rows inside the boundary tie group.
    #[test]
    fn streaming_order_by_snapshot_rejects_undecodable_boundary_rid() {
        let ranges = vec![OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "FF".to_owned(),
            server_continuation: None,
            boundary: Some(ValueBoundary {
                resume_values: vec![
                    crate::driver::dataflow::order_by::OrderByResumeValue::Number {
                        value: 5.0.into(),
                    },
                ],
                last_rid: "not-a-rid".to_owned(),
                skip_count: 1,
            }),
        }];
        let err = validate_streaming_order_by_snapshot(
            &[SortOrder::Ascending],
            &[SortOrder::Ascending],
            "fingerprint",
            Some("fingerprint"),
            ranges,
        )
        .err()
        .expect("an undecodable boundary RID must be rejected");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID),
            "got: {err}"
        );
    }

    /// A well-formed 16-byte RID is *not* automatically a document RID: the
    /// child-resource type nibble distinguishes documents from partition key
    /// ranges, stored procedures, and so on. Without that check a crafted
    /// token would pass validation and enter the numeric tie-break with an
    /// arbitrary ordinal.
    #[test]
    fn streaming_order_by_snapshot_rejects_non_document_boundary_rid() {
        // Same shape as `valid_rid`, but tagged as a partition key range.
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&[0x0A, 0x0B, 0x0C, 0x0D, 0x80, 0x01, 0x02, 0x03]);
        bytes[8..16].copy_from_slice(&7u64.to_le_bytes());
        bytes[15] = 0x50;
        let pkrange_rid = crate::models::resource_id::encode_rid(&bytes);

        let ranges = vec![OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "FF".to_owned(),
            server_continuation: None,
            boundary: Some(ValueBoundary {
                resume_values: vec![
                    crate::driver::dataflow::order_by::OrderByResumeValue::Number {
                        value: 5.0.into(),
                    },
                ],
                last_rid: pkrange_rid,
                skip_count: 1,
            }),
        }];
        let err = validate_streaming_order_by_snapshot(
            &[SortOrder::Ascending],
            &[SortOrder::Ascending],
            "fingerprint",
            Some("fingerprint"),
            ranges,
        )
        .err()
        .expect("a non-document boundary RID must be rejected");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID),
            "got: {err}"
        );
    }

    /// A realistic 16-byte document `_rid`, as the backend emits — the
    /// boundary validator requires one it can decode.
    fn valid_rid(doc_id: u64) -> String {
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&[0x0A, 0x0B, 0x0C, 0x0D, 0x80, 0x01, 0x02, 0x03]);
        bytes[8..16].copy_from_slice(&doc_id.to_le_bytes());
        crate::models::resource_id::encode_rid(&bytes)
    }

    /// A boundary in a resumed `StreamingOrderedMerge` snapshot always counts
    /// at least its own boundary row, so an explicit `skip_count` of 0 is a
    /// corrupt token and must be rejected.
    #[test]
    fn streaming_order_by_snapshot_rejects_zero_skip_count() {
        let ranges = vec![OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "FF".to_owned(),
            server_continuation: None,
            boundary: Some(ValueBoundary {
                resume_values: vec![
                    crate::driver::dataflow::order_by::OrderByResumeValue::Number {
                        value: 5.0.into(),
                    },
                ],
                last_rid: valid_rid(1),
                skip_count: 0,
            }),
        }];
        let err = validate_streaming_order_by_snapshot(
            &[SortOrder::Ascending],
            &[SortOrder::Ascending],
            "fingerprint",
            Some("fingerprint"),
            ranges,
        )
        .err()
        .expect("a boundary skip_count of 0 must be rejected");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID),
            "a boundary skip_count of 0 must be rejected, got: {err}"
        );
    }

    /// The smallest well-formed boundary carries `skip_count == 1` (just the
    /// boundary row) and validates.
    #[test]
    fn streaming_order_by_snapshot_accepts_skip_count_one() {
        let ranges = vec![OrderByRangeToken {
            min_epk: String::new(),
            max_epk: "FF".to_owned(),
            server_continuation: None,
            boundary: Some(ValueBoundary {
                resume_values: vec![
                    crate::driver::dataflow::order_by::OrderByResumeValue::Number {
                        value: 5.0.into(),
                    },
                ],
                last_rid: valid_rid(1),
                skip_count: 1,
            }),
        }];
        let parsed = validate_streaming_order_by_snapshot(
            &[SortOrder::Ascending],
            &[SortOrder::Ascending],
            "fingerprint",
            Some("fingerprint"),
            ranges,
        )
        .expect("a well-formed boundary with skip_count 1 is valid");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].boundary.as_ref().unwrap().skip_count, 1);
    }

    /// A legacy token that omits `skip_count` deserializes as 1 (not 0), so it
    /// still validates — its boundary row is discarded on resume, never
    /// re-emitted.
    #[test]
    fn streaming_order_by_snapshot_defaults_missing_skip_count_to_one() {
        let token: OrderByRangeToken = serde_json::from_str(&format!(
            r#"{{"min_epk":"","max_epk":"FF","boundary":{{"resume_values":[{{"type":"number","value":5.0}}],"last_rid":"{rid}"}}}}"#,
            rid = valid_rid(1),
        ))
        .unwrap();
        assert_eq!(token.boundary.as_ref().unwrap().skip_count, 1);
        let parsed = validate_streaming_order_by_snapshot(
            &[SortOrder::Ascending],
            &[SortOrder::Ascending],
            "fingerprint",
            Some("fingerprint"),
            vec![token],
        )
        .expect("a legacy boundary missing skip_count defaults to 1 and is valid");
        assert_eq!(parsed[0].boundary.as_ref().unwrap().skip_count, 1);
    }
}
