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

use crate::models::{
    effective_partition_key::EffectivePartitionKey, CosmosOperation, FeedRange, OperationTarget,
};

use super::{
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
) -> azure_core::Result<Pipeline> {
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
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!(
                    "continuation token shape {} does not match a trivial operation",
                    snapshot_kind(&other)
                ),
            ));
        }
    };

    let request_target = match target {
        OperationTarget::None => RequestTarget::NonPartitioned,
        OperationTarget::PartitionKey(pk) => RequestTarget::LogicalPartitionKey(pk.clone()),
        OperationTarget::FeedRange(_) => {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "FeedRange targeting requires a fan-out pipeline; \
                 use plan_operation for cross-partition queries",
            ));
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
/// (resumed) leaf only.
pub(crate) async fn build_sequential_drain(
    query_plan: &QueryPlan,
    topology_provider: &mut dyn TopologyProvider,
    operation: &Arc<CosmosOperation>,
    resume: Option<PipelineNodeState>,
) -> azure_core::Result<Pipeline> {
    validate_query_plan(query_plan)?;

    let resume = match resume {
        None => None,
        Some(PipelineNodeState::Drained) => {
            return Ok(Pipeline::new(Box::new(DrainedLeaf)));
        }
        Some(PipelineNodeState::SequentialDrain {
            current_min_epk,
            left_most,
        }) => {
            let server_continuation = match *left_most {
                PipelineNodeState::Request {
                    server_continuation,
                } => server_continuation,
                PipelineNodeState::Drained => None,
                other => {
                    return Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        format!(
                            "continuation token has unsupported nested shape inside SequentialDrain: {}",
                            snapshot_kind(&other)
                        ),
                    ));
                }
            };
            Some(ResumeCursor {
                current_min_epk: EffectivePartitionKey::from(current_min_epk),
                server_continuation,
            })
        }
        Some(PipelineNodeState::Request {
            server_continuation,
        }) => {
            // A bare Request snapshot means the cross-partition query had only
            // a single child — apply it as a cursor at the minimum EPK.
            Some(ResumeCursor {
                current_min_epk: EffectivePartitionKey::min(),
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
        let feed_range = FeedRange::new(min, max);
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

            // Carry the server continuation onto the first surviving leaf,
            // then clear it so subsequent leaves start fresh.
            let initial_continuation = resume.as_mut().and_then(|c| c.server_continuation.take());
            let target = RequestTarget::EffectivePartitionKeyRange {
                range: resolved_range.range,
                partition_key_range_id: resolved_range.partition_key_range_id,
            };
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
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "query plan produced no partition ranges to query",
        ));
    }

    let root: Box<dyn PipelineNode> = if request_nodes.len() == 1 {
        request_nodes.into_iter().next().unwrap()
    } else {
        Box::new(SequentialDrain::new(request_nodes))
    };

    Ok(Pipeline::new(root))
}

/// Resume cursor extracted from a `SequentialDrain` continuation snapshot.
struct ResumeCursor {
    current_min_epk: EffectivePartitionKey,
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
fn validate_query_plan(plan: &QueryPlan) -> azure_core::Result<()> {
    if plan.hybrid_search_query_info.is_some() {
        return Err(unsupported_feature("hybrid search queries"));
    }

    if let Some(info) = &plan.query_info {
        validate_query_info(info)?;
    }

    Ok(())
}

fn validate_query_info(info: &QueryInfo) -> azure_core::Result<()> {
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
    Ok(())
}

fn unsupported_feature(feature: &str) -> azure_core::Error {
    azure_core::Error::with_message(
        azure_core::error::ErrorKind::Other,
        format!("unsupported query feature: {feature}"),
    )
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
        CosmosOperation::query_items(
            test_container(),
            OperationTarget::FeedRange(FeedRange::full()),
        )
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
                assert_eq!(
                    err.to_string(),
                    "FeedRange targeting requires a fan-out pipeline; \
                     use plan_operation for cross-partition queries"
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
            ),
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

    /// Asserts that the pipeline is a single `Request` targeting the expected EPK range.
    fn assert_single_request(
        pipeline: &Pipeline,
        expected_min: &str,
        expected_max: &str,
        expected_pk_range_id: &str,
    ) {
        let request = pipeline
            .root()
            .downcast_ref::<Request>()
            .expect("expected single Request root");
        assert_eq!(
            *request.target(),
            RequestTarget::EffectivePartitionKeyRange {
                range: FeedRange::new(
                    EffectivePartitionKey::from(expected_min),
                    EffectivePartitionKey::from(expected_max),
                ),
                partition_key_range_id: expected_pk_range_id.to_string(),
            }
        );
    }

    /// Asserts that the pipeline is a `SequentialDrain` containing `Request` nodes
    /// targeting the given EPK ranges (in order).
    fn assert_drain_requests(pipeline: Pipeline, expected: &[(&str, &str, &str)]) {
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
        for (child, &(min, max, pk_range_id)) in children.into_iter().zip(expected) {
            let request = child
                .downcast::<Request>()
                .expect("expected Request child node");
            assert_eq!(
                *request.target(),
                RequestTarget::EffectivePartitionKeyRange {
                    range: FeedRange::new(
                        EffectivePartitionKey::from(min),
                        EffectivePartitionKey::from(max),
                    ),
                    partition_key_range_id: pk_range_id.to_string(),
                },
                "mismatch for pk range {pk_range_id}"
            );
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
        assert_single_request(&pipeline, "", "FF", "pkrange-0");
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
        // The resolved range matches the topology, not the query range.
        let plan = plan_with_ranges(vec![qr("20", "80")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![rr("", "FF", "pkrange-wide")])]);

        let pipeline = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap();
        assert_single_request(&pipeline, "", "FF", "pkrange-wide");
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
        assert_eq!(
            err.to_string(),
            "unsupported query feature: TOP clause in cross-partition queries"
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
        assert_eq!(
            err.to_string(),
            "unsupported query feature: LIMIT clause in cross-partition queries"
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
        assert_eq!(
            err.to_string(),
            "unsupported query feature: ORDER BY in cross-partition queries"
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
        assert_eq!(
            err.to_string(),
            "unsupported query feature: aggregates in cross-partition queries"
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
        assert_eq!(
            err.to_string(),
            "unsupported query feature: GROUP BY in cross-partition queries"
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
        assert_eq!(
            err.to_string(),
            "unsupported query feature: hybrid search queries"
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
        assert_single_request(&pipeline, "", "FF", "pkrange-0");
    }

    #[tokio::test]
    async fn rejects_empty_query_ranges() {
        let plan = plan_with_ranges(vec![]);
        let op = cross_partition_query_operation();
        let mut topology = NoopTopologyProvider;

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap_err();
        assert_eq!(
            err.to_string(),
            "query plan produced no partition ranges to query"
        );
    }

    #[tokio::test]
    async fn propagates_topology_resolution_error() {
        let plan = plan_with_ranges(vec![qr("", "FF")]);
        let op = cross_partition_query_operation();
        let mut topology = MockTopologyProvider::new(vec![Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "topology resolution failed",
        ))]);

        let err = build_sequential_drain(&plan, &mut topology, &Arc::new(op), None)
            .await
            .unwrap_err();
        assert_eq!(err.to_string(), "topology resolution failed");
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
            left_most: Box::new(PipelineNodeState::SequentialDrain {
                current_min_epk: "00".to_owned(),
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
}
