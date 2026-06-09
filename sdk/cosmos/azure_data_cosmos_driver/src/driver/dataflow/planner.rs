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
    DrainedLeaf, PartitionRoutingRefresh, Pipeline, PipelineNode, PipelineNodeState, Request,
    RequestTarget, SequentialDrain, TopologyProvider,
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
pub(crate) async fn build_sequential_drain(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
    resume: Option<PipelineNodeState>,
) -> crate::error::Result<Pipeline> {
    validate_query_plan(query_plan)?;

    let saved_children = match resume {
        None => None,
        Some(PipelineNodeState::Drained) => {
            return Ok(Pipeline::new(Box::new(DrainedLeaf)));
        }
        Some(PipelineNodeState::SequentialDrain { children }) => {
            Some(validate_saved_children(children)?)
        }
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

    let request_nodes = if let Some(saved) = saved_children.as_ref() {
        plan_resume_from_saved_children(query_plan, topology_provider, operation, saved).await?
    } else {
        plan_fresh(query_plan, topology_provider, operation).await?
    };

    // TODO: enforce max fan-out (default 100, configurable). See FEED_OPERATIONS_REQS.md §3.

    if request_nodes.is_empty() {
        // If we resumed from saved children that were all `Drained`, the
        // pipeline is fully drained. Otherwise the plan / topology yielded
        // nothing to query — that's a service contract violation.
        if saved_children.is_some() {
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

/// Builds the request leaves for a fresh (non-resumed) cross-partition plan.
async fn plan_fresh(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
) -> crate::error::Result<Vec<Box<dyn PipelineNode>>> {
    let mut nodes: Vec<Box<dyn PipelineNode>> = Vec::new();
    for query_range in &query_plan.query_ranges {
        let feed_range = query_range_to_feed_range(query_range)?;
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
/// saved children list as the authoritative remaining-work ledger.
///
/// For each saved child, intersects its range with the current topology and
/// emits one [`Request`] leaf per intersection (carrying the saved server
/// continuation when present; fresh otherwise; or nothing when the saved
/// state is `Drained`). The intersected union is tracked per saved child so
/// any non-`Drained` saved range that can't be fully covered is reported as
/// a continuation-token error.
async fn plan_resume_from_saved_children(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
    saved: &[SavedChild],
) -> crate::error::Result<Vec<Box<dyn PipelineNode>>> {
    let mut nodes: Vec<Box<dyn PipelineNode>> = Vec::new();

    // Track per-saved-child intersection coverage so we can detect missing
    // topology coverage at the end.
    let mut coverage: Vec<Vec<FeedRange>> = vec![Vec::new(); saved.len()];

    for query_range in &query_plan.query_ranges {
        let feed_range = query_range_to_feed_range(query_range)?;
        let resolved = topology_provider
            .resolve_ranges(&feed_range, PartitionRoutingRefresh::UseCached)
            .await?;

        for resolved_range in resolved {
            // The leaf's range is the resolved range clipped to the query range.
            let leaf_scope = intersect_feed_ranges(&resolved_range.range, &feed_range).expect(
                "topology provider must return ranges that overlap the query plan EPK range",
            );

            for (idx, saved_child) in saved.iter().enumerate() {
                let Some(intersection) = intersect_feed_ranges(&leaf_scope, &saved_child.range)
                else {
                    continue;
                };
                coverage[idx].push(intersection.clone());

                let token = match &saved_child.state {
                    SavedChildState::Drained => continue,
                    SavedChildState::Request {
                        server_continuation,
                    } => server_continuation.clone(),
                };

                let target = RequestTarget::effective_partition_key_range(
                    intersection,
                    resolved_range.partition_key_range_id.clone(),
                    resolved_range.range.clone(),
                );
                nodes.push(Box::new(Request::new(Arc::clone(operation), target, token)));
            }
        }
    }

    // Verify every non-drained saved range was fully covered by the current
    // topology. If not, the planner cannot honor the saved continuation
    // without risking duplicate emission or data loss — fail loudly.
    for (idx, saved_child) in saved.iter().enumerate() {
        if matches!(saved_child.state, SavedChildState::Drained) {
            continue;
        }
        if !range_fully_covered(&saved_child.range, &coverage[idx]) {
            let coverage_summary = if coverage[idx].is_empty() {
                "(no overlapping topology ranges)".to_string()
            } else {
                let mut sorted = coverage[idx].clone();
                sorted.sort_by(|a, b| a.min_inclusive().cmp(b.min_inclusive()));
                sorted
                    .iter()
                    .map(|r| {
                        format!(
                            "[{}, {})",
                            r.min_inclusive().as_str(),
                            r.max_exclusive().as_str()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" + ")
            };
            return Err(crate::error::CosmosError::builder()
                .with_status(
                    crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED,
                )
                .with_message(format!(
                    "continuation token saved range [{}, {}) could not be fully covered \
                     by the current topology (covered: {}); the query cannot be safely resumed",
                    saved_child.range.min_inclusive().as_str(),
                    saved_child.range.max_exclusive().as_str(),
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
            piece.min_inclusive().as_str(),
            piece.max_exclusive().as_str(),
            range.min_inclusive().as_str(),
            range.max_exclusive().as_str(),
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

/// Validated saved child range + its state.
#[derive(Debug)]
struct SavedChild {
    range: FeedRange,
    state: SavedChildState,
}

#[derive(Debug)]
enum SavedChildState {
    Drained,
    Request { server_continuation: Option<String> },
}

/// Validates a saved children list from a continuation token: each range has
/// `min <= max`, the list is strictly sorted ascending by `min`, and ranges
/// are non-overlapping. Returns the parsed `Vec<SavedChild>` on success or a
/// continuation-token shape error on failure.
fn validate_saved_children(
    children: Vec<super::RangedChildState>,
) -> crate::error::Result<Vec<SavedChild>> {
    let mut parsed: Vec<SavedChild> = Vec::with_capacity(children.len());
    for child in children {
        let min = EffectivePartitionKey::from(child.min_epk);
        let max = EffectivePartitionKey::from(child.max_epk);
        if min > max {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE)
                .with_message(format!(
                    "continuation token has invalid SequentialDrain child range (min `{}` > max `{}`)",
                    min.as_str(),
                    max.as_str(),
                ))
                .build());
        }
        if min == max {
            // A zero-width child is structurally well-formed but cannot
            // contribute coverage; reject explicitly so the caller sees a
            // diagnostic message that points at the entry itself rather
            // than at a downstream "could not be fully covered" failure.
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE)
                .with_message(format!(
                    "continuation token has zero-width SequentialDrain child range (min == max == `{}`); \
                     zero-width entries cannot carry remaining work",
                    min.as_str(),
                ))
                .build());
        }
        let range = FeedRange::new(min, max)?;
        let state = match child.state {
            PipelineNodeState::Drained => SavedChildState::Drained,
            PipelineNodeState::Request {
                server_continuation,
            } => SavedChildState::Request {
                server_continuation,
            },
            other => {
                return Err(crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_UNEXPECTED_NESTED_SHAPE)
                    .with_message(format!(
                        "continuation token has unsupported nested shape inside SequentialDrain child: {}",
                        snapshot_kind(&other)
                    ))
                    .build());
            }
        };
        if let Some(prev) = parsed.last() {
            if range.min_inclusive() < prev.range.max_exclusive() {
                return Err(crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_CHILDREN,
                    )
                    .with_message(format!(
                        "continuation token SequentialDrain children must be sorted and non-overlapping; \
                         entry [{}, {}) is out of order or overlaps the previous entry [{}, {})",
                        range.min_inclusive().as_str(),
                        range.max_exclusive().as_str(),
                        prev.range.min_inclusive().as_str(),
                        prev.range.max_exclusive().as_str(),
                    ))
                    .build());
            }
        }
        parsed.push(SavedChild { range, state });
    }
    Ok(parsed)
}

fn snapshot_kind(state: &PipelineNodeState) -> &'static str {
    match state {
        PipelineNodeState::Drained => "Drained",
        PipelineNodeState::Request { .. } => "Request",
        PipelineNodeState::SequentialDrain { .. } => "SequentialDrain",
    }
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
        driver::dataflow::{mocks::*, query_plan::QueryRange, RangedChildState, ResolvedRange},
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
            assert_eq!(request.snapshot_state(), expected_state);
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

    /// Builds a `SequentialDrain` resume state from `(min, max, state)` triples.
    fn saved_drain(children: Vec<(&str, &str, PipelineNodeState)>) -> PipelineNodeState {
        PipelineNodeState::SequentialDrain {
            children: children
                .into_iter()
                .map(|(min, max, state)| RangedChildState {
                    min_epk: min.to_owned(),
                    max_epk: max.to_owned(),
                    state,
                })
                .collect(),
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
            pipeline.snapshot_state(),
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
    async fn resume_with_all_saved_children_drained_yields_drained_pipeline() {
        // If every saved child is drained, the pipeline is fully drained
        // regardless of topology.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-a")])]);

        let resume = saved_drain(vec![
            ("", "55", PipelineNodeState::Drained),
            ("55", "FF", PipelineNodeState::Drained),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert!(matches!(
            pipeline.snapshot_state(),
            PipelineNodeState::Drained
        ));
    }

    #[tokio::test]
    async fn resume_with_empty_saved_children_yields_drained_pipeline() {
        // An empty children list means every range has been drained.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-a")])]);

        let resume = saved_drain(vec![]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert!(matches!(
            pipeline.snapshot_state(),
            PipelineNodeState::Drained
        ));
    }

    #[tokio::test]
    async fn resume_rejects_nested_sequential_drain_inside_child() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-a")])]);

        let resume = saved_drain(vec![(
            "00",
            "80",
            saved_drain(vec![("00", "80", saved_request(None))]),
        )]);

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap_err();
        assert!(
            err.to_string().contains("unsupported nested shape"),
            "unexpected error message: {err}",
        );
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
        // Out-of-order saved children: [55, AA) then [00, 55) violates
        // strict ascending order.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-a")])]);

        let resume = saved_drain(vec![
            ("55", "AA", saved_request(None)),
            ("00", "55", saved_request(None)),
        ]);

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap_err();
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_INVALID_CHILDREN),
            "expected invalid-children sub-status, got: {err}",
        );
    }

    #[tokio::test]
    async fn resume_validates_saved_children_no_overlap() {
        // Overlapping saved children: [00, 80) and [55, FF) overlap on
        // [55, 80).
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-a")])]);

        let resume = saved_drain(vec![
            ("00", "80", saved_request(None)),
            ("55", "FF", saved_request(None)),
        ]);

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap_err();
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_CONTINUATION_TOKEN_INVALID_CHILDREN),
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
    async fn resume_drained_saved_child_emits_nothing_in_its_span() {
        // Drained saved children short-circuit to zero leaves in their span,
        // even when the topology has intersecting ranges.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "55", "pk-a"),
            rr("55", "AA", "pk-b"),
            rr("AA", "FF", "pk-c"),
        ])]);

        let resume = saved_drain(vec![
            ("", "55", PipelineNodeState::Drained),
            ("55", "AA", saved_request(Some("server-token-xyz"))),
            ("AA", "FF", PipelineNodeState::Drained),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[("55", "AA", "pk-b", "55", "AA", Some("server-token-xyz"))],
        );
    }

    #[tokio::test]
    async fn resume_multiple_saved_children_in_one_resolved_range_no_duplicate_leaves() {
        // The topology has merged the saved children into one wide range.
        // Each saved child produces exactly one leaf scoped to its own
        // range — no overlap, no duplicates.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-merged")])]);

        let resume = saved_drain(vec![
            ("10", "30", saved_request(Some("tok-a"))),
            ("30", "60", saved_request(Some("tok-b"))),
            ("60", "90", saved_request(None)),
        ]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[
                ("10", "30", "pk-merged", "", "FF", Some("tok-a")),
                ("30", "60", "pk-merged", "", "FF", Some("tok-b")),
                ("60", "90", "pk-merged", "", "FF", None),
            ],
        );
    }

    #[tokio::test]
    async fn resume_does_not_requery_topology_gaps_outside_saved() {
        // Regression guard: the planner must not emit fresh leaves for
        // topology ranges that fall outside every saved child's scope.
        // Anything outside the saved-children union has already been drained
        // on a prior page; re-querying it would duplicate that page's items.
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "20", "pk-a"),
            rr("20", "40", "pk-b"),
            rr("40", "60", "pk-c"),
            rr("60", "80", "pk-d"),
            rr("80", "FF", "pk-e"),
        ])]);

        // Only [40, 60) is still pending; everything else has been drained.
        let resume = saved_drain(vec![("40", "60", saved_request(Some("tok")))]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests_with_partitions_and_continuation(
            pipeline,
            &[("40", "60", "pk-c", "40", "60", Some("tok"))],
        );
    }

    /// 0.3.0 could mint a top-level bare `Request` continuation shape for
    /// single-physical-partition queries — at the time the planner treated
    /// that as a full-range cursor. The 0.4.0 planner rejects it. The
    /// CHANGELOG documents both shapes as breaking. This test guards the
    /// CHANGELOG promise: 0.3.0 bare-`Request` tokens MUST fail on resume
    /// (with any error), not silently succeed.
    #[tokio::test]
    async fn legacy_0_3_0_top_level_request_shape_fails_to_resume() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-0")])]);

        let legacy = PipelineNodeState::Request {
            server_continuation: Some("OLD".to_owned()),
        };

        let result =
            build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(legacy)).await;
        let err = result.expect_err("0.3.0 bare-Request shape must be rejected on resume");
        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH,
            "expected SHAPE_MISMATCH for top-level bare Request shape; got {err:?}",
        );
    }

    /// L4: zero-width child entries are well-formed JSON but cannot carry
    /// remaining work. They must be rejected with a message that points
    /// at the entry itself rather than at a downstream "could not be
    /// fully covered" error.
    #[tokio::test]
    async fn rejects_zero_width_saved_child_entry_with_clear_message() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-0")])]);

        let resume = PipelineNodeState::SequentialDrain {
            children: vec![RangedChildState {
                min_epk: "40".to_owned(),
                max_epk: "40".to_owned(),
                state: PipelineNodeState::Request {
                    server_continuation: Some("tok".to_owned()),
                },
            }],
        };

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .expect_err("zero-width saved child must be rejected");
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
}
