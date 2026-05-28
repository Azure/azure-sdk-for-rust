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
/// Produces either a single [`Request`] leaf (when planning resolves to a
/// single physical partition) or a [`SequentialDrain`] over one `Request` per
/// resolved range. Other cross-partition strategies (streaming `ORDER BY`,
/// hybrid search, read-many, etc.) will live as sibling functions.
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
/// 3. Creates a [`Request`] node for each resolved range and bundles them in a
///    [`SequentialDrain`].
///
/// `resume` is an optional [`PipelineNodeState`] from a continuation token.
/// When present, ranges whose `max_exclusive <= current_min_epk` are skipped
/// and the server continuation from `left_most` is propagated to the front
/// (resumed) leaf only. The cursor also preserves the active child's original
/// max EPK so resume can split a merged physical range into
/// `[current_min_epk, current_max_epk)` (continued) and
/// `[current_max_epk, merged_max)` (fresh).
pub(crate) async fn build_sequential_drain(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
    resume: Option<PipelineNodeState>,
) -> crate::error::Result<Pipeline> {
    validate_query_plan(query_plan)?;

    let resume = match resume {
        None => None,
        Some(PipelineNodeState::Drained) => {
            return Ok(Pipeline::new(Box::new(DrainedLeaf)));
        }
        Some(PipelineNodeState::SequentialDrain {
            current_min_epk,
            current_max_epk,
            left_most,
        }) => {
            let server_continuation = match *left_most {
                PipelineNodeState::Request {
                    server_continuation,
                } => server_continuation,
                PipelineNodeState::Drained => None,
                other => {
                    return Err(crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_UNEXPECTED_NESTED_SHAPE).with_message(format!(
                            "continuation token has unsupported nested shape inside SequentialDrain: {}",
                            snapshot_kind(&other)
                        )).build());
                }
            };
            let current_min_epk = EffectivePartitionKey::from(current_min_epk);
            let current_max_epk = EffectivePartitionKey::from(current_max_epk);
            if current_min_epk > current_max_epk {
                return Err(crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE,
                    )
                    .with_message(
                        "continuation token has invalid SequentialDrain range (min > max)",
                    )
                    .build());
            }
            Some(ResumeCursor {
                current_min_epk,
                current_max_epk,
                server_continuation,
            })
        }
        Some(PipelineNodeState::Request {
            server_continuation,
        }) => {
            // A bare Request snapshot means the cross-partition query had only
            // a single child — apply it as a cursor at the minimum EPK.
            Some(ResumeCursor {
                current_min_epk: EffectivePartitionKey::MIN.clone(),
                current_max_epk: EffectivePartitionKey::MAX.clone(),
                server_continuation,
            })
        }
    };

    // Convert query ranges to FeedRanges and resolve against topology.
    let mut request_nodes: Vec<Box<dyn PipelineNode>> = Vec::new();
    let mut resume = resume;
    for query_range in &query_plan.query_ranges {
        let min = EffectivePartitionKey::from(query_range.min.as_str());
        let max = EffectivePartitionKey::from(query_range.max.as_str());
        let feed_range = FeedRange::new(min, max)?;
        let resolved = topology_provider
            .resolve_ranges(&feed_range, PartitionRoutingRefresh::UseCached)
            .await?;

        for resolved_range in resolved {
            // Skip ranges that are entirely below the resume cursor.
            if let Some(cursor) = resume.as_ref() {
                if resolved_range.range.max_exclusive() <= &cursor.current_min_epk {
                    continue;
                }
            }

            // If we resumed inside a range that later merged with neighbors,
            // keep the continuation scoped to the original child range and
            // enqueue the remaining tail as a fresh request.
            if let Some(cursor) = resume.as_mut() {
                if cursor.server_continuation.is_some()
                    && resolved_range.range.min_inclusive() <= &cursor.current_min_epk
                    && &cursor.current_max_epk < resolved_range.range.max_exclusive()
                {
                    let resumed_range = FeedRange::new(
                        cursor.current_min_epk.clone(),
                        cursor.current_max_epk.clone(),
                    )?;
                    let resumed_target = RequestTarget::effective_partition_key_range(
                        resumed_range,
                        resolved_range.partition_key_range_id.clone(),
                        resolved_range.range.clone(),
                    );
                    let resumed_continuation = cursor.server_continuation.take();
                    request_nodes.push(Box::new(Request::new(
                        Arc::clone(operation),
                        resumed_target,
                        resumed_continuation,
                    )));

                    let tail_range = FeedRange::new(
                        cursor.current_max_epk.clone(),
                        resolved_range.range.max_exclusive().clone(),
                    )?;
                    let tail_target = RequestTarget::effective_partition_key_range(
                        tail_range,
                        resolved_range.partition_key_range_id,
                        resolved_range.range,
                    );
                    request_nodes.push(Box::new(Request::new(
                        Arc::clone(operation),
                        tail_target,
                        None,
                    )));
                    continue;
                }
            }

            // Carry the server continuation to every leaf node, because
            // we don't know which ones have been drained. The service does, and will
            // just return empty pages for those if needed.
            let initial_continuation = resume.as_mut().and_then(|c| c.server_continuation.take());
            let range = intersect_feed_ranges(&resolved_range.range, &feed_range).expect(
                "topology provider must return ranges that overlap the query plan EPK range",
            );
            let target = RequestTarget::effective_partition_key_range(
                range,
                resolved_range.partition_key_range_id,
                resolved_range.range,
            );
            request_nodes.push(Box::new(Request::new(
                Arc::clone(operation),
                target,
                initial_continuation,
            )));
        }
    }

    // TODO: enforce max fan-out (default 100, configurable). See FEED_OPERATIONS_REQS.md §3.

    if request_nodes.is_empty() {
        // Either the plan had no ranges or everything was below the cursor.
        // The latter is a normal "fully drained" outcome — emit a drained leaf.
        if resume.is_some() {
            return Ok(Pipeline::new(Box::new(DrainedLeaf)));
        }
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES)
            .with_message("query plan produced no partition ranges to query")
            .build());
    }

    // Even when there's only one request node, we still need to wrap it in a SequentialDrain
    // so that the pipeline can react to splits by replacing the single Request with multiple Requests.
    let root = Box::new(SequentialDrain::new(request_nodes));

    Ok(Pipeline::new(root))
}

/// Resume cursor extracted from a `SequentialDrain` continuation snapshot.
struct ResumeCursor {
    current_min_epk: EffectivePartitionKey,
    current_max_epk: EffectivePartitionKey,
    server_continuation: Option<String>,
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
        driver::dataflow::{mocks::*, query_plan::QueryRange, ResolvedRange},
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

        // The drained pipeline immediately yields no pages.
        assert!(matches!(
            pipeline.snapshot_state(),
            PipelineNodeState::Drained
        ));
    }

    #[tokio::test]
    async fn resume_skips_ranges_below_cursor() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "55", "pk-a"),
            rr("55", "AA", "pk-b"),
            rr("AA", "FF", "pk-c"),
        ])]);

        // Cursor sitting at the first byte of the second range — the first
        // range (max_exclusive == "55") must be skipped, the others kept.
        let resume = PipelineNodeState::SequentialDrain {
            current_min_epk: "55".to_owned(),
            current_max_epk: "AA".to_owned(),
            left_most: Box::new(PipelineNodeState::Request {
                server_continuation: None,
            }),
        };

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert_drain_requests(pipeline, &[("55", "AA", "pk-b"), ("AA", "FF", "pk-c")]);
    }

    #[tokio::test]
    async fn resume_propagates_server_continuation_to_first_surviving_leaf_only() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            rr("", "55", "pk-a"),
            rr("55", "AA", "pk-b"),
            rr("AA", "FF", "pk-c"),
        ])]);

        let resume = PipelineNodeState::SequentialDrain {
            current_min_epk: "55".to_owned(),
            current_max_epk: "AA".to_owned(),
            left_most: Box::new(PipelineNodeState::Request {
                server_continuation: Some("server-token-xyz".to_owned()),
            }),
        };

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        let snapshot = pipeline.snapshot_state();
        let PipelineNodeState::SequentialDrain { left_most, .. } = snapshot else {
            panic!("expected SequentialDrain snapshot, got {snapshot:?}");
        };
        assert_eq!(
            *left_most,
            PipelineNodeState::Request {
                server_continuation: Some("server-token-xyz".to_owned()),
            },
            "front leaf must carry the resumed server continuation",
        );
    }

    #[tokio::test]
    async fn resume_with_cursor_past_all_ranges_yields_drained_pipeline() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "55", "pk-a")])]);

        let resume = PipelineNodeState::SequentialDrain {
            current_min_epk: "FF".to_owned(),
            current_max_epk: "FF".to_owned(),
            left_most: Box::new(PipelineNodeState::Drained),
        };

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), Some(resume))
            .await
            .unwrap();
        assert!(matches!(
            pipeline.snapshot_state(),
            PipelineNodeState::Drained
        ));
    }

    #[tokio::test]
    async fn resume_rejects_nested_sequential_drain_inside_left_most() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-a")])]);

        let resume = PipelineNodeState::SequentialDrain {
            current_min_epk: "00".to_owned(),
            current_max_epk: "80".to_owned(),
            left_most: Box::new(PipelineNodeState::SequentialDrain {
                current_min_epk: "00".to_owned(),
                current_max_epk: "80".to_owned(),
                left_most: Box::new(PipelineNodeState::Request {
                    server_continuation: None,
                }),
            }),
        };

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
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pk-merged")])]);

        let resume = PipelineNodeState::SequentialDrain {
            current_min_epk: "55".to_owned(),
            current_max_epk: "AA".to_owned(),
            left_most: Box::new(PipelineNodeState::Request {
                server_continuation: Some("server-token-xyz".to_owned()),
            }),
        };

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
}
