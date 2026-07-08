// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

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
    models::{effective_partition_key::EffectivePartitionKey, CosmosOperation, FeedRange},
};

use super::{
    intersect_feed_ranges,
    query_plan::{QueryInfo, QueryPlan},
    DrainedLeaf, PartitionRoutingRefresh, Pipeline, PipelineNode, PipelineNodeState, RangedToken,
    Request, RequestTarget, ResolvedRange, SequentialDrain, TopologyProvider, UnorderedMerge,
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
    validate_query_plan(query_plan)?;

    let saved_snapshot = match resume {
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
        plan_resume_from_saved_snapshot(query_plan, topology_provider, operation, saved).await?
    } else {
        plan_fresh(query_plan, topology_provider, operation).await?
    };

    // TODO: enforce max fan-out (default 100, configurable). See FEED_OPERATIONS_REQS.md §3.

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
    let root = Box::new(SequentialDrain::new(request_nodes));
    Ok(Pipeline::new(root))
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

    let root = Box::new(UnorderedMerge::new(request_nodes).with_start_marker(start_marker));
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
    // For partition-scoped queries (e.g. `FeedScope::partition(partial_hpk)`)
    // the operation carries a FeedRange that bounds the partition-key prefix.
    // The server-supplied `query_ranges` always cover the full container, so
    // we intersect each one with the operation scope to keep the fan-out (and
    // the per-pkrange wire EPK bounds) scoped to that prefix.
    let scope_range = operation.target();
    for query_range in &query_plan.query_ranges {
        let plan_range = query_range_to_feed_range(query_range)?;
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
            let range = intersect_feed_ranges(&resolved_range.range, &feed_range).expect(
                "topology provider must return ranges that overlap the query plan EPK range",
            );
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
    // See `plan_fresh` for rationale on intersecting with the operation scope.
    let scope_range = operation.target();

    for query_range in &query_plan.query_ranges {
        let plan_range = query_range_to_feed_range(query_range)?;
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
            // The leaf's range is the resolved range clipped to the query range.
            let leaf_scope = intersect_feed_ranges(&resolved_range.range, &feed_range).expect(
                "topology provider must return ranges that overlap the query plan EPK range",
            );

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
fn query_range_to_feed_range(
    query_range: &super::query_plan::QueryRange,
) -> crate::error::Result<FeedRange> {
    let min = EffectivePartitionKey::from(query_range.min.as_str());
    let max = EffectivePartitionKey::from(query_range.max.as_str());
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
    if info.top.is_some() {
        return Err(unsupported_feature("TOP clause in cross-partition queries"));
    }
    if info.limit.is_some() {
        return Err(unsupported_feature(
            "LIMIT clause in cross-partition queries",
        ));
    }
    if !info.order_by.is_empty() {
        return Err(unsupported_feature("ORDER BY in cross-partition queries"));
    }
    if !info.aggregates.is_empty() {
        return Err(unsupported_feature("aggregates in cross-partition queries"));
    }
    if !info.group_by_expressions.is_empty() {
        return Err(unsupported_feature("GROUP BY in cross-partition queries"));
    }
    if info.distinct_type != DistinctType::None {
        return Err(unsupported_feature("DISTINCT in cross-partition queries"));
    }
    Ok(())
}

fn unsupported_feature(feature: &str) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::CLIENT_UNSUPPORTED_QUERY_FEATURE)
        .with_message(format!("unsupported query feature: {feature}"))
        .build()
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use crate::{
        driver::dataflow::{mocks::*, query_plan::QueryRange, RangedToken, ResolvedRange},
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
        serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap()
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
    async fn rejects_query_plan_with_top() {
        let plan = QueryPlan {
            query_info: Some(QueryInfo {
                top: Some(10),
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
            rendered.ends_with("unsupported query feature: TOP clause in cross-partition queries"),
            "unexpected: {rendered}"
        );
    }

    #[tokio::test]
    async fn rejects_query_plan_with_limit() {
        let plan = QueryPlan {
            query_info: Some(QueryInfo {
                limit: Some(20),
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
            rendered
                .ends_with("unsupported query feature: LIMIT clause in cross-partition queries"),
            "unexpected: {rendered}"
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
}
