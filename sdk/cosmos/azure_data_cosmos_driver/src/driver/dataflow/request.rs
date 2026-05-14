// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request leaf node for the dataflow pipeline.

use std::sync::Arc;

use async_trait::async_trait;
use azure_core::http::StatusCode;

use crate::models::{CosmosOperation, CosmosResponse, FeedRange, PartitionKey, SubStatusCode};

use super::{
    PageResult, PartitionRoutingRefresh, PipelineContext, PipelineNode, PipelineNodeState,
    ResolvedRange,
};

/// The target of a request node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RequestTarget {
    /// The request is to a non-partitioned resource (databases, containers, offers, etc.)
    NonPartitioned,

    /// A single logical partition key.
    LogicalPartitionKey(PartitionKey),

    /// An EPK slice that must be queried inside a broader physical partition key range
    /// (assuming the cached topology remains valid).
    EffectivePartitionKeyRange {
        /// EPK range scoped by this request.
        range: FeedRange,
        /// Partition key range ID containing `range`.
        partition_key_range_id: String,
    },
}

impl RequestTarget {
    /// Returns the EPK slice owned by this request target, if any.
    fn owned_range(&self) -> Option<&FeedRange> {
        match self {
            RequestTarget::EffectivePartitionKeyRange { range, .. } => Some(range),
            _ => None,
        }
    }

    /// Returns `true` if this target's EPK range starts at the same point as `parent_range`.
    fn covers_start_of(&self, parent_range: &FeedRange) -> bool {
        self.owned_range()
            .is_some_and(|range| range.min_inclusive() == parent_range.min_inclusive())
    }
}

fn intersect_feed_ranges(left: &FeedRange, right: &FeedRange) -> Option<FeedRange> {
    let min = if left.min_inclusive() >= right.min_inclusive() {
        left.min_inclusive().clone()
    } else {
        right.min_inclusive().clone()
    };
    let max = if left.max_exclusive() <= right.max_exclusive() {
        left.max_exclusive().clone()
    } else {
        right.max_exclusive().clone()
    };

    (min < max).then(|| FeedRange::new(min, max))
}

#[derive(Debug, PartialEq, Eq)]
enum RequestState {
    /// No request has been sent yet. The next page will trigger the initial request.
    Initial,

    /// A request has been sent and a server continuation token has been received, but not all pages have been drained yet. The next page will trigger a request with the continuation token.
    Continuing { continuation: String },

    /// All pages have been drained. No further requests will be sent.
    Drained,
}

/// Leaf node that executes one Cosmos DB request per page.
///
/// The `operation` is held as an `Arc<CosmosOperation>` so the same logical
/// operation can be shared across many `Request` nodes (e.g. in a fan-out
/// `SequentialDrain` over multiple partitions) without paying for one full
/// `CosmosOperation` copy per node. Per-request differences are applied at
/// execution time via [`OperationOverrides`](crate::pipeline::OperationOverrides),
/// not by mutating the shared operation.
pub(crate) struct Request {
    operation: Arc<CosmosOperation>,
    target: RequestTarget,
    state: RequestState,
}

impl Request {
    /// Creates a request node.
    pub(crate) fn new(
        operation: Arc<CosmosOperation>,
        target: RequestTarget,
        initial_continuation: Option<String>,
    ) -> Self {
        let initial_state = if let Some(token) = initial_continuation {
            RequestState::Continuing {
                continuation: token,
            }
        } else {
            RequestState::Initial
        };
        Self {
            operation,
            target,
            state: initial_state,
        }
    }

    #[cfg(test)]
    /// Returns the operation this request node executes.
    pub(crate) fn operation(&self) -> &CosmosOperation {
        &self.operation
    }

    #[cfg(test)]
    /// Returns the target this request node uses for routing.
    pub(crate) fn target(&self) -> &RequestTarget {
        &self.target
    }
}

#[async_trait]
impl PipelineNode for Request {
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> azure_core::Result<PageResult> {
        tracing::trace!(
            target = ?self.target,
            state = ?self.state,
            "executing request node"
        );

        let continuation = match &self.state {
            RequestState::Initial => None,
            RequestState::Continuing { continuation } => Some(continuation.clone()),
            RequestState::Drained => return Ok(PageResult::Drained),
        };

        match context
            .execute_request(
                &self.operation,
                self.target.clone(),
                PartitionRoutingRefresh::UseCached,
                continuation.clone(),
            )
            .await
        {
            Ok(response) => Ok(self.handle_response(response)),
            Err(error) if is_partition_topology_change(&error) => {
                self.handle_partition_topology_change(context, error, continuation)
                    .await
            }
            Err(error) => Err(error),
        }
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        Vec::new()
    }

    fn snapshot_state(&self) -> PipelineNodeState {
        match &self.state {
            RequestState::Initial => PipelineNodeState::Request {
                server_continuation: None,
            },
            RequestState::Continuing { continuation } => PipelineNodeState::Request {
                server_continuation: Some(continuation.clone()),
            },
            RequestState::Drained => PipelineNodeState::Drained,
        }
    }

    fn feed_range(&self) -> Option<&FeedRange> {
        self.target.owned_range()
    }
}
impl Request {
    fn handle_response(&mut self, response: CosmosResponse) -> PageResult {
        let continuation = response.headers().continuation.clone();
        tracing::trace!(
            target = ?self.target,
            status = ?response.status(),
            output_continuation = ?continuation,
            "request completed"
        );
        self.state = if let Some(token) = continuation {
            RequestState::Continuing {
                continuation: token,
            }
        } else {
            RequestState::Drained
        };
        tracing::trace!(target = ?self.target, state = ?self.state, "updated request state after response");
        let is_terminal = matches!(self.state, RequestState::Drained);
        PageResult::Page {
            response,
            is_terminal,
        }
    }

    async fn handle_partition_topology_change(
        &mut self,
        context: &mut PipelineContext<'_>,
        error: azure_core::Error,
        continuation: Option<String>,
    ) -> azure_core::Result<PageResult> {
        match &self.target {
            RequestTarget::NonPartitioned => {
                // Non-partitioned resources don't have partition topology changes.
                Err(error)
            }
            RequestTarget::LogicalPartitionKey(_) => {
                // This shouldn't really happen, but it's been observed.
                // Since the original request had a logical partition key,
                // the gateway should have been able to route the request
                // to the correct partition even if it has split.
                // But we can do a single retry without forcing a topology refresh to see if it succeeds.
                context
                    .execute_request(
                        &self.operation,
                        self.target.clone(),
                        PartitionRoutingRefresh::ForceRefresh,
                        continuation,
                    )
                    .await
                    .map(|response| {
                        tracing::trace!(
                            target = ?self.target,
                            status = ?response.status(),
                            "retry after logical partition key topology change succeeded"
                        );
                        self.handle_response(response)
                    })
            }
            RequestTarget::EffectivePartitionKeyRange { range, .. } => {
                let range = range.clone();
                self.split_for_topology_change(context, &range).await
            }
        }
    }

    /// Resolves the current topology for this node's EPK range and returns
    /// a `SplitRequired` result with replacement nodes for each sub-range.
    async fn split_for_topology_change(
        &self,
        context: &mut PipelineContext<'_>,
        range: &FeedRange,
    ) -> azure_core::Result<PageResult> {
        let resolved = context
            .resolve_ranges(range, PartitionRoutingRefresh::ForceRefresh)
            .await?;

        let replacement_nodes: Vec<Box<dyn PipelineNode>> = resolved
            .into_iter()
            .map(|resolved_range| {
                let ResolvedRange {
                    partition_key_range_id,
                    range: resolved_range,
                } = resolved_range;
                let owned_range = intersect_feed_ranges(&resolved_range, range).expect(
                    "topology provider must return ranges that overlap the request's owned EPK range",
                );

                let target = RequestTarget::EffectivePartitionKeyRange {
                    range: owned_range,
                    partition_key_range_id,
                };

                // Carry over the server continuation to the first replacement that
                // covers the same starting EPK. For a split, only the left-most child
                // inherits the continuation since it resumes where this node left off.
                let continuation = match (target.covers_start_of(range), &self.state) {
                    (
                        true,
                        RequestState::Continuing {
                            continuation: latest_server_continuation,
                        },
                    ) => Some(latest_server_continuation.clone()),
                    _ => None,
                };
                Box::new(Request::new(self.operation.clone(), target, continuation))
                    as Box<dyn PipelineNode>
            })
            .collect();

        Ok(PageResult::SplitRequired { replacement_nodes })
    }
}

// Partition topology changes are a specific subset of `Gone` substatus codes.
// Other substatus mappings live in `pipeline::retry_evaluation`; this one stays
// here because it drives pipeline-level repair (splitting a node into
// replacements) rather than per-attempt retry.
fn is_partition_topology_change(error: &azure_core::Error) -> bool {
    match error.kind() {
        azure_core::error::ErrorKind::HttpResponse {
            status, error_code, ..
        } if *status == StatusCode::Gone => error_code
            .as_deref()
            .and_then(|code| code.parse::<u32>().ok())
            .is_some_and(is_partition_topology_change_substatus),
        _ => false,
    }
}

fn is_partition_topology_change_substatus(substatus: u32) -> bool {
    matches!(
        SubStatusCode::new(substatus),
        SubStatusCode::PARTITION_KEY_RANGE_GONE
            | SubStatusCode::COMPLETING_SPLIT
            | SubStatusCode::COMPLETING_PARTITION_MIGRATION
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::dataflow::{mocks::*, RequestExecutor, ResolvedRange, TopologyProvider};
    use crate::models::{effective_partition_key::EffectivePartitionKey, FeedRange};

    #[derive(Clone, Debug)]
    struct PhysicalPartitionSpec {
        partition_key_range_id: String,
        range: FeedRange,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct RequestSpec {
        target: RequestTarget,
        continuation: Option<String>,
    }

    struct ScenarioTopologyProvider {
        resolved_ranges: Vec<ResolvedRange>,
    }

    impl ScenarioTopologyProvider {
        fn new(partitions: &[PhysicalPartitionSpec]) -> Self {
            Self {
                resolved_ranges: partitions
                    .iter()
                    .map(|partition| ResolvedRange {
                        partition_key_range_id: partition.partition_key_range_id.clone(),
                        range: partition.range.clone(),
                    })
                    .collect(),
            }
        }
    }

    impl TopologyProvider for ScenarioTopologyProvider {
        fn resolve_ranges<'a>(
            &'a mut self,
            range: &'a FeedRange,
            _refresh: PartitionRoutingRefresh,
        ) -> futures::future::BoxFuture<'a, azure_core::Result<Vec<ResolvedRange>>> {
            let resolved = self
                .resolved_ranges
                .iter()
                .filter(|candidate| {
                    candidate.range.min_inclusive() < range.max_exclusive()
                        && candidate.range.max_exclusive() > range.min_inclusive()
                })
                .cloned()
                .collect::<Vec<_>>();

            Box::pin(async move {
                if resolved.is_empty() {
                    Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        "scenario topology produced no overlapping ranges",
                    ))
                } else {
                    Ok(resolved)
                }
            })
        }
    }

    struct AlwaysGoneRequestExecutor;

    impl RequestExecutor for AlwaysGoneRequestExecutor {
        fn execute_request<'a>(
            &'a mut self,
            _operation: &'a CosmosOperation,
            _target: RequestTarget,
            _partition_routing_refresh: PartitionRoutingRefresh,
            _continuation: Option<String>,
        ) -> futures::future::BoxFuture<'a, azure_core::Result<CosmosResponse>> {
            Box::pin(async { Err(gone_error()) })
        }
    }

    fn physical_partition(
        min: &str,
        max: &str,
        partition_key_range_id: &str,
    ) -> PhysicalPartitionSpec {
        PhysicalPartitionSpec {
            partition_key_range_id: partition_key_range_id.to_string(),
            range: FeedRange::new(
                EffectivePartitionKey::from(min),
                EffectivePartitionKey::from(max),
            ),
        }
    }

    fn request_spec(target: RequestTarget, continuation: Option<&str>) -> RequestSpec {
        RequestSpec {
            target,
            continuation: continuation.map(str::to_owned),
        }
    }

    fn partition_key_request(
        min: &str,
        max: &str,
        partition_key_range_id: &str,
        continuation: Option<&str>,
    ) -> RequestSpec {
        request_spec(
            effective_partition_key_range_target(min, max, partition_key_range_id),
            continuation,
        )
    }

    fn effective_partition_key_request(
        min: &str,
        max: &str,
        partition_key_range_id: &str,
        continuation: Option<&str>,
    ) -> RequestSpec {
        request_spec(
            effective_partition_key_range_target(min, max, partition_key_range_id),
            continuation,
        )
    }

    fn build_request(spec: RequestSpec) -> Request {
        Request::new(Arc::new(operation()), spec.target, spec.continuation)
    }

    fn snapshot_request(request: &Request) -> RequestSpec {
        let continuation = match &request.state {
            RequestState::Initial => None,
            RequestState::Continuing { continuation } => Some(continuation.clone()),
            RequestState::Drained => panic!("scenario helper should not produce drained requests"),
        };

        RequestSpec {
            target: request.target.clone(),
            continuation,
        }
    }

    async fn apply_topology_round(
        requests: Vec<Request>,
        partitions: &[PhysicalPartitionSpec],
    ) -> Vec<Request> {
        let mut executor = AlwaysGoneRequestExecutor;
        let mut topology = ScenarioTopologyProvider::new(partitions);
        let mut rewritten = Vec::new();

        for mut request in requests {
            let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
            match request.next_page(&mut context).await.unwrap() {
                PageResult::SplitRequired { replacement_nodes } => {
                    rewritten.extend(replacement_nodes.into_iter().map(|node| {
                        *node
                            .downcast::<Request>()
                            .expect("scenario helper should only produce request nodes")
                    }));
                }
                other => panic!("expected SplitRequired during topology rewrite, got {other:?}"),
            }
        }

        rewritten
    }

    async fn assert_topology_rewrite(
        initial_requests: Vec<RequestSpec>,
        topology_rounds: Vec<Vec<PhysicalPartitionSpec>>,
        expected_requests: Vec<RequestSpec>,
    ) {
        let mut current = initial_requests
            .into_iter()
            .map(build_request)
            .collect::<Vec<_>>();

        // Each round applies a new physical partition layout to the current request list.
        // We intentionally do not try to coalesce adjacent requests after repeated topology
        // changes; these tests care about correctness of ownership, not optimality.
        for partitions in topology_rounds {
            current = apply_topology_round(current, &partitions).await;
        }

        let actual = current.iter().map(snapshot_request).collect::<Vec<_>>();
        assert_eq!(actual, expected_requests);
    }

    fn effective_partition_key_range_target(
        min: &str,
        max: &str,
        partition_key_range_id: &str,
    ) -> RequestTarget {
        RequestTarget::EffectivePartitionKeyRange {
            range: FeedRange::new(
                EffectivePartitionKey::from(min),
                EffectivePartitionKey::from(max),
            ),
            partition_key_range_id: partition_key_range_id.to_string(),
        }
    }

    #[tokio::test]
    async fn request_retries_logical_partition_key_topology_change_once() {
        let mut request = Request::new(Arc::new(operation()), logical_partition_target(), None);
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error()), Ok(response(b"ok"))]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let page = unwrap_page(request.next_page(&mut context).await);

        assert_eq!(page.body(), b"ok");
        assert_eq!(
            executor.refresh_calls,
            vec![
                PartitionRoutingRefresh::UseCached,
                PartitionRoutingRefresh::ForceRefresh
            ]
        );
        assert_eq!(executor.continuation_calls, vec![None, None]);
    }

    #[tokio::test]
    async fn request_returns_second_logical_partition_key_topology_change() {
        let mut request = Request::new(Arc::new(operation()), logical_partition_target(), None);
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error()), Err(gone_error())]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let error = request.next_page(&mut context).await.unwrap_err();

        assert!(is_partition_topology_change(&error));
        assert_eq!(
            executor.refresh_calls,
            vec![
                PartitionRoutingRefresh::UseCached,
                PartitionRoutingRefresh::ForceRefresh
            ]
        );
        assert_eq!(executor.continuation_calls, vec![None, None]);
    }

    #[tokio::test]
    async fn request_does_not_retry_non_topology_gone() {
        let mut request = Request::new(Arc::new(operation()), logical_partition_target(), None);
        let mut executor = MockRequestExecutor::new(vec![Err(non_topology_gone_error())]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let error = request.next_page(&mut context).await.unwrap_err();

        assert!(!is_partition_topology_change(&error));
        assert_eq!(
            executor.refresh_calls,
            vec![PartitionRoutingRefresh::UseCached]
        );
        assert_eq!(executor.continuation_calls, vec![None]);
    }

    #[tokio::test]
    async fn request_tracks_server_continuation_for_next_page() {
        let mut request = Request::new(Arc::new(operation()), logical_partition_target(), None);
        let mut executor = MockRequestExecutor::new(vec![
            Ok(response_with_continuation(b"page1", Some("token-1"))),
            Ok(response_with_continuation(b"page2", Some("token-2"))),
        ]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let page1 = unwrap_page(request.next_page(&mut context).await);
        let page2 = unwrap_page(request.next_page(&mut context).await);

        assert_eq!(page1.body(), b"page1");
        assert_eq!(page2.body(), b"page2");
        assert_eq!(
            executor.continuation_calls,
            vec![None, Some("token-1".to_string())]
        );
        assert_eq!(
            request.state,
            RequestState::Continuing {
                continuation: "token-2".to_string()
            }
        );
    }

    #[tokio::test]
    async fn request_uses_restored_continuation_on_first_page() {
        let mut request = Request::new(
            Arc::new(operation()),
            logical_partition_target(),
            Some("restored-token".to_string()),
        );
        let mut executor = MockRequestExecutor::new(vec![Ok(response(b"page"))]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let page = unwrap_page(request.next_page(&mut context).await);

        assert_eq!(page.body(), b"page");
        assert_eq!(
            executor.continuation_calls,
            vec![Some("restored-token".to_string())]
        );
        assert_eq!(request.state, RequestState::Drained);
    }

    // ── Topology rewrite scenarios ───────────────────────────────────────

    #[tokio::test]
    async fn topology_rewrite_handles_simple_split() {
        assert_topology_rewrite(
            vec![partition_key_request("", "80", "0", Some("server-token"))],
            vec![vec![
                physical_partition("", "40", "1"),
                physical_partition("40", "80", "2"),
            ]],
            vec![
                partition_key_request("", "40", "1", Some("server-token")),
                partition_key_request("40", "80", "2", None),
            ],
        )
        .await;
    }

    #[tokio::test]
    async fn topology_rewrite_handles_simple_merge() {
        assert_topology_rewrite(
            vec![
                partition_key_request("", "40", "left", Some("merge-token")),
                partition_key_request("40", "80", "right", None),
            ],
            vec![vec![physical_partition("", "80", "merged")]],
            vec![
                effective_partition_key_request("", "40", "merged", Some("merge-token")),
                effective_partition_key_request("40", "80", "merged", None),
            ],
        )
        .await;
    }

    #[tokio::test]
    async fn topology_rewrite_leaves_unchanged_neighbors_alone() {
        assert_topology_rewrite(
            vec![
                partition_key_request("", "40", "left", Some("ct")),
                partition_key_request("40", "80", "right", None),
            ],
            vec![vec![
                physical_partition("", "40", "left"),
                physical_partition("40", "60", "right-a"),
                physical_partition("60", "80", "right-b"),
            ]],
            vec![
                partition_key_request("", "40", "left", Some("ct")),
                partition_key_request("40", "60", "right-a", None),
                partition_key_request("60", "80", "right-b", None),
            ],
        )
        .await;
    }

    #[tokio::test]
    async fn topology_rewrite_can_return_from_merged_epk_slices_to_exact_pk_ranges() {
        assert_topology_rewrite(
            vec![
                effective_partition_key_request("", "40", "merged", Some("ct")),
                effective_partition_key_request("40", "80", "merged", None),
            ],
            vec![vec![
                physical_partition("", "40", "left"),
                physical_partition("40", "80", "right"),
            ]],
            vec![
                partition_key_request("", "40", "left", Some("ct")),
                partition_key_request("40", "80", "right", None),
            ],
        )
        .await;
    }

    #[tokio::test]
    async fn topology_rewrite_handles_merge_then_different_split_mid_pipeline() {
        assert_topology_rewrite(
            vec![
                partition_key_request("00", "20", "a", Some("ct")),
                partition_key_request("20", "40", "b", None),
                partition_key_request("40", "80", "c", None),
            ],
            vec![
                vec![
                    physical_partition("00", "40", "merged-left"),
                    physical_partition("40", "80", "c"),
                ],
                vec![
                    physical_partition("00", "10", "split-a"),
                    physical_partition("10", "30", "split-b"),
                    physical_partition("30", "50", "split-c"),
                    physical_partition("50", "80", "split-d"),
                ],
            ],
            vec![
                partition_key_request("00", "10", "split-a", Some("ct")),
                effective_partition_key_request("10", "20", "split-b", None),
                effective_partition_key_request("20", "30", "split-b", None),
                effective_partition_key_request("30", "40", "split-c", None),
                effective_partition_key_request("40", "50", "split-c", None),
                partition_key_request("50", "80", "split-d", None),
            ],
        )
        .await;
    }

    #[tokio::test]
    async fn topology_provider_error_propagates() {
        let mut request = Request::new(Arc::new(operation()), epk_range_target(), None);
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error())]);
        let mut topology = MockTopologyProvider::new(vec![Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "topology fetch failed",
        ))]);
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let err = request.next_page(&mut context).await.unwrap_err();
        assert_eq!(err.to_string(), "topology fetch failed");
    }

    #[tokio::test]
    async fn non_partitioned_topology_change_not_retried() {
        let mut request = Request::new(Arc::new(operation()), RequestTarget::NonPartitioned, None);
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error())]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let err = request.next_page(&mut context).await.unwrap_err();
        assert!(is_partition_topology_change(&err));
    }
}
