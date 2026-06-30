// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request leaf node for the dataflow pipeline.

use std::sync::Arc;

use async_trait::async_trait;

use crate::models::{CosmosOperation, CosmosResponse, FeedRange, PartitionKey};

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
        /// EPK range scoped by this request, when narrower than `partition_key_range`.
        range: Option<FeedRange>,
        /// Partition key range ID containing `range`.
        partition_key_range_id: String,
        /// EPK range owned by the physical partition key range ID.
        partition_key_range: FeedRange,
    },
}

impl RequestTarget {
    /// Creates a target for an effective partition key range inside a physical partition.
    pub(crate) fn effective_partition_key_range(
        range: FeedRange,
        partition_key_range_id: String,
        partition_key_range: FeedRange,
    ) -> Self {
        let range = if range == partition_key_range {
            None
        } else {
            Some(range)
        };

        Self::EffectivePartitionKeyRange {
            range,
            partition_key_range_id,
            partition_key_range,
        }
    }

    /// Returns the EPK slice owned by this request target, if any.
    fn owned_range(&self) -> Option<&FeedRange> {
        match self {
            RequestTarget::EffectivePartitionKeyRange {
                range,
                partition_key_range,
                ..
            } => Some(range.as_ref().unwrap_or(partition_key_range)),
            _ => None,
        }
    }
}

pub(crate) fn intersect_feed_ranges(left: &FeedRange, right: &FeedRange) -> Option<FeedRange> {
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

    // Panicking is acceptable here since we know that left and right should be valid ranges, and thus the intersection must be too.
    (min < max).then(|| FeedRange::new(min, max).expect("we just computed valid range bounds"))
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
    ) -> crate::error::Result<PageResult> {
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
            Err(error) if error.status().is_partition_topology_change() => {
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

    fn snapshot_state(&self) -> crate::error::Result<PipelineNodeState> {
        Ok(match &self.state {
            RequestState::Initial => PipelineNodeState::Request {
                server_continuation: None,
            },
            RequestState::Continuing { continuation } => PipelineNodeState::Request {
                server_continuation: Some(continuation.clone()),
            },
            RequestState::Drained => PipelineNodeState::Drained,
        })
    }

    fn feed_range(&self) -> Option<&FeedRange> {
        self.target.owned_range()
    }

    fn topology_can_change(&self) -> bool {
        // Only requests targeting EPK ranges can be affected by partition topology changes that require splitting or merging.
        // A non-partitioned request can't be affected because it doesn't rely on partition routing.
        // A logical partition key request shouldn't be affected because the gateway should route it to the correct partition even if splits have occurred,
        // but in the unlikely event it is, we retry once inline and have no need to split since a given LPK can never span multiple physical partitions.
        matches!(
            self.target,
            RequestTarget::EffectivePartitionKeyRange { .. }
        )
    }
}

impl Request {
    fn handle_response(&mut self, response: CosmosResponse) -> PageResult {
        if self.operation.is_change_feed() {
            return self.handle_change_feed_response(response);
        }

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

    /// Handles a change feed response.
    ///
    /// Change feed is an unbounded stream: the consumer polls indefinitely and
    /// the service signals "no new changes" with `304 Not Modified` rather than
    /// ending the feed. Two behaviors differ from a normal feed read:
    ///
    /// 1. The continuation token is carried by the **ETag** header (re-sent as
    ///    `If-None-Match` on the next poll), not `x-ms-continuation`.
    /// 2. The request must **never** transition to `Drained`. Even a `304` with
    ///    no body advances the ETag, and the next poll resumes from there.
    fn handle_change_feed_response(&mut self, response: CosmosResponse) -> PageResult {
        let etag = response.headers().etag.as_ref().map(|e| e.to_string());
        tracing::trace!(
            target = ?self.target,
            status = ?response.status(),
            output_etag = ?etag,
            "change feed request completed"
        );
        if let Some(token) = etag {
            self.state = RequestState::Continuing {
                continuation: token,
            };
        }
        // If the response carried no ETag (unexpected for a change feed read),
        // keep the prior state so the next poll can retry rather than ending
        // the stream prematurely. The change feed is never terminal.
        PageResult::Page {
            response,
            is_terminal: false,
        }
    }

    async fn handle_partition_topology_change(
        &mut self,
        context: &mut PipelineContext<'_>,
        error: crate::error::CosmosError,
        continuation: Option<String>,
    ) -> crate::error::Result<PageResult> {
        // Capture the failed attempt's diagnostics before consuming the
        // error. The per-operation pipeline that produced this error
        // owns its own `DiagnosticsContext`; the dataflow retry below
        // will spin up another full pipeline invocation with a fresh
        // context. Without splicing the prior context onto the
        // retry's response, callers reading
        // `response.diagnostics().request_count()` would only see the
        // final successful attempt — violating the
        // "one operation = one `DiagnosticsContext` capturing every
        // attempt" contract. Always capture, regardless of branch, so
        // the splice happens uniformly on every successful retry path.
        let prior_diagnostics = error.diagnostics();
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
                        // Splice the prior failed attempt's diagnostics
                        // onto the retry's diagnostics so the surfaced
                        // `CosmosResponse` reflects every attempt the
                        // operation made (see `prior_diagnostics`
                        // capture above for rationale).
                        let response = if let Some(prior) = prior_diagnostics {
                            response.with_aggregated_prior_diagnostics(&[prior])
                        } else {
                            response
                        };
                        self.handle_response(response)
                    })
            }
            RequestTarget::EffectivePartitionKeyRange { .. } => {
                let range = self
                    .target
                    .owned_range()
                    .expect("effective partition key range target must have an owned range")
                    .clone();
                // TODO(diagnostics-aggregation): the split path replaces
                // this node with one or more sub-range `Request` nodes
                // that each execute independently in subsequent
                // `next_page` calls. Splicing `prior_diagnostics` into
                // every sub-node's first response would require
                // threading the prior context through the replacement
                // nodes; tracked as a follow-up. For now, prior
                // attempts on the EPK-range split path are still
                // captured by the replacement node when it triggers
                // its own dataflow retry, but not aggregated onto the
                // first successful sub-range response.
                let _ = prior_diagnostics;
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
    ) -> crate::error::Result<PageResult> {
        let resolved = context
            .resolve_ranges(range, PartitionRoutingRefresh::ForceRefresh)
            .await?;

        let continuation = match &self.state {
            RequestState::Continuing { continuation } => Some(continuation.clone()),
            _ => None,
        };

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

                let target = RequestTarget::effective_partition_key_range(
                    owned_range,
                    partition_key_range_id,
                    resolved_range,
                );

                Box::new(Request::new(
                    self.operation.clone(),
                    target,
                    continuation.clone(),
                ))
                    as Box<dyn PipelineNode>
            })
            .collect();

        Ok(PageResult::SplitRequired { replacement_nodes })
    }
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
        ) -> futures::future::BoxFuture<'a, crate::error::Result<Vec<ResolvedRange>>> {
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
                    Err(crate::error::CosmosError::builder()
                        .with_status(crate::error::CosmosStatus::new(
                            azure_core::http::StatusCode::BadRequest,
                        ))
                        .with_message("scenario topology produced no overlapping ranges")
                        .build())
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
        ) -> futures::future::BoxFuture<'a, crate::error::Result<CosmosResponse>> {
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
            )
            .unwrap(),
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
            effective_partition_key_range_target(min, max, partition_key_range_id, min, max),
            continuation,
        )
    }

    fn effective_partition_key_request(
        min: &str,
        max: &str,
        partition_key_range_id: &str,
        partition_min: &str,
        partition_max: &str,
        continuation: Option<&str>,
    ) -> RequestSpec {
        request_spec(
            effective_partition_key_range_target(
                min,
                max,
                partition_key_range_id,
                partition_min,
                partition_max,
            ),
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
        partition_min: &str,
        partition_max: &str,
    ) -> RequestTarget {
        RequestTarget::effective_partition_key_range(
            FeedRange::new(
                EffectivePartitionKey::from(min),
                EffectivePartitionKey::from(max),
            )
            .unwrap(),
            partition_key_range_id.to_string(),
            FeedRange::new(
                EffectivePartitionKey::from(partition_min),
                EffectivePartitionKey::from(partition_max),
            )
            .unwrap(),
        )
    }

    #[tokio::test]
    async fn request_retries_logical_partition_key_topology_change_once() {
        let mut request = Request::new(Arc::new(operation()), logical_partition_target(), None);
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error()), Ok(response(b"ok"))]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let page = unwrap_page(request.next_page(&mut context).await);

        assert_eq!(page.body_bytes(), b"ok");
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

        assert!(error.status().is_partition_topology_change());
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

        assert!(!error.status().is_partition_topology_change());
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

        assert_eq!(page1.body_bytes(), b"page1");
        assert_eq!(page2.body_bytes(), b"page2");
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

        assert_eq!(page.body_bytes(), b"page");
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
                partition_key_request("40", "80", "2", Some("server-token")),
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
                effective_partition_key_request("", "40", "merged", "", "80", Some("merge-token")),
                effective_partition_key_request("40", "80", "merged", "", "80", None),
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
                effective_partition_key_request("", "40", "merged", "", "80", Some("ct")),
                effective_partition_key_request("40", "80", "merged", "", "80", None),
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
                effective_partition_key_request("10", "20", "split-b", "10", "30", Some("ct")),
                effective_partition_key_request("20", "30", "split-b", "10", "30", None),
                effective_partition_key_request("30", "40", "split-c", "30", "50", None),
                effective_partition_key_request("40", "50", "split-c", "30", "50", None),
                partition_key_request("50", "80", "split-d", None),
            ],
        )
        .await;
    }

    #[tokio::test]
    async fn topology_provider_error_propagates() {
        let mut request = Request::new(Arc::new(operation()), epk_range_target(), None);
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error())]);
        let mut topology =
            MockTopologyProvider::new(vec![Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("topology fetch failed")
                .build())]);
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let err = request.next_page(&mut context).await.unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.ends_with("topology fetch failed"),
            "unexpected: {rendered}"
        );
    }

    #[tokio::test]
    async fn non_partitioned_topology_change_not_retried() {
        let mut request = Request::new(Arc::new(operation()), RequestTarget::NonPartitioned, None);
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error())]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));

        let err = request.next_page(&mut context).await.unwrap_err();
        assert!(err.status().is_partition_topology_change());
    }
}
