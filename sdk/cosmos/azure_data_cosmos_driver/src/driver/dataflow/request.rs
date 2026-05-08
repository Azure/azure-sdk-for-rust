// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request leaf node for the dataflow pipeline.

use async_trait::async_trait;
use azure_core::http::StatusCode;

use crate::models::{CosmosOperation, CosmosResponse, FeedRange, PartitionKey, SubStatusCode};

use super::{ChildNodes, PageResult, PartitionRoutingRefresh, PipelineContext, PipelineNode};

/// The target of a request node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RequestTarget {
    /// The request is to a non-partitioned resource (databases, containers, offers, etc.)
    NonPartitioned,

    /// A single logical partition key.
    LogicalPartitionKey(PartitionKey),

    /// An effective partition key range believed to be in one physical partition.
    EffectivePartitionKeyRange {
        /// EPK range scoped by this request.
        range: FeedRange,
        /// Partition key range ID believed to contain `range`.
        partition_key_range_id: String,
    },
}

impl RequestTarget {
    /// Returns `true` if this target's EPK range starts at the same point as `parent_range`.
    fn covers_start_of(&self, parent_range: &FeedRange) -> bool {
        match self {
            RequestTarget::EffectivePartitionKeyRange { range, .. } => {
                range.min_inclusive() == parent_range.min_inclusive()
            }
            _ => false,
        }
    }
}

/// Leaf node that executes one Cosmos DB request per page.
pub(crate) struct Request {
    operation: CosmosOperation,
    target: RequestTarget,
    latest_server_continuation: Option<String>,
    logical_partition_topology_retry_used: bool,
}

impl Request {
    /// Creates a request node.
    pub(crate) fn new(operation: CosmosOperation, target: RequestTarget) -> Self {
        Self::with_continuation(operation, target, None)
    }

    /// Creates a request node restored with the latest server-issued continuation.
    pub(crate) fn with_continuation(
        operation: CosmosOperation,
        target: RequestTarget,
        latest_server_continuation: Option<String>,
    ) -> Self {
        Self {
            operation,
            target,
            latest_server_continuation,
            logical_partition_topology_retry_used: false,
        }
    }

    /// Returns the operation this request node executes.
    pub(crate) fn operation(&self) -> &CosmosOperation {
        &self.operation
    }

    /// Returns the target this request node uses for routing.
    pub(crate) fn target(&self) -> &RequestTarget {
        &self.target
    }

    /// Returns the latest server-issued continuation for this request's partition.
    pub(crate) fn latest_server_continuation(&self) -> Option<&str> {
        self.latest_server_continuation.as_deref()
    }
}

#[async_trait]
impl PipelineNode for Request {
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> azure_core::Result<PageResult> {
        match context
            .execute_request(
                &self.operation,
                self.target.clone(),
                PartitionRoutingRefresh::UseCached,
                self.latest_server_continuation.clone(),
            )
            .await
        {
            Ok(response) => Ok(PageResult::Page(
                self.record_response_continuation(response),
            )),
            Err(error) if is_partition_topology_change(&error) => {
                self.handle_partition_topology_change(context, error).await
            }
            Err(error) => Err(error),
        }
    }

    fn children(&self) -> ChildNodes<'_> {
        ChildNodes::None
    }

    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        Vec::new()
    }
}
impl Request {
    async fn handle_partition_topology_change(
        &mut self,
        context: &mut PipelineContext<'_>,
        error: azure_core::Error,
    ) -> azure_core::Result<PageResult> {
        match &self.target {
            RequestTarget::NonPartitioned => {
                // Non-partitioned resources don't have partition topology changes.
                Err(error)
            }
            RequestTarget::LogicalPartitionKey(_) => {
                if self.logical_partition_topology_retry_used {
                    return Err(error);
                }

                // This shouldn't really happen, but it's been observed.
                // Since the original request had a logical partition key,
                // the gateway should have been able to route the request
                // to the correct partition even if it has split.
                // But we can do a single retry without forcing a topology refresh to see if it succeeds.
                self.logical_partition_topology_retry_used = true;
                context
                    .execute_request(
                        &self.operation,
                        self.target.clone(),
                        PartitionRoutingRefresh::ForceRefresh,
                        self.latest_server_continuation.clone(),
                    )
                    .await
                    .map(|response| PageResult::Page(self.record_response_continuation(response)))
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
                let target = RequestTarget::EffectivePartitionKeyRange {
                    range: resolved_range.range,
                    partition_key_range_id: resolved_range.partition_key_range_id,
                };
                // Carry over the server continuation to the first replacement that
                // covers the same starting EPK. For a split, only the left-most child
                // inherits the continuation since it resumes where this node left off.
                // TODO: When we support streaming ordered merges, we'll need to augment this a bit.
                let continuation = if target.covers_start_of(range) {
                    self.latest_server_continuation.clone()
                } else {
                    None
                };
                Box::new(Request::with_continuation(
                    self.operation.clone(),
                    target,
                    continuation,
                )) as Box<dyn PipelineNode>
            })
            .collect();

        Ok(PageResult::SplitRequired { replacement_nodes })
    }

    fn record_response_continuation(&mut self, response: CosmosResponse) -> CosmosResponse {
        self.latest_server_continuation = response.headers().continuation.clone();
        response
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
    use crate::driver::dataflow::{mocks::*, ResolvedRange};
    use crate::models::{effective_partition_key::EffectivePartitionKey, FeedRange};

    #[tokio::test]
    async fn request_retries_logical_partition_key_topology_change_once() {
        let mut request = Request::new(operation(), logical_partition_target());
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error()), Ok(response(b"ok"))]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

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
        let mut request = Request::new(operation(), logical_partition_target());
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error()), Err(gone_error())]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

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
        let mut request = Request::new(operation(), logical_partition_target());
        let mut executor = MockRequestExecutor::new(vec![Err(non_topology_gone_error())]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

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
        let mut request = Request::new(operation(), logical_partition_target());
        let mut executor = MockRequestExecutor::new(vec![
            Ok(response_with_continuation(b"page1", Some("token-1"))),
            Ok(response_with_continuation(b"page2", Some("token-2"))),
        ]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        let page1 = unwrap_page(request.next_page(&mut context).await);
        let page2 = unwrap_page(request.next_page(&mut context).await);

        assert_eq!(page1.body(), b"page1");
        assert_eq!(page2.body(), b"page2");
        assert_eq!(
            executor.continuation_calls,
            vec![None, Some("token-1".to_string())]
        );
        assert_eq!(request.latest_server_continuation(), Some("token-2"));
    }

    #[tokio::test]
    async fn request_uses_restored_continuation_on_first_page() {
        let mut request = Request::with_continuation(
            operation(),
            logical_partition_target(),
            Some("restored-token".to_string()),
        );
        let mut executor = MockRequestExecutor::new(vec![Ok(response(b"page"))]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        let page = unwrap_page(request.next_page(&mut context).await);

        assert_eq!(page.body(), b"page");
        assert_eq!(
            executor.continuation_calls,
            vec![Some("restored-token".to_string())]
        );
        assert_eq!(request.latest_server_continuation(), None);
    }

    // ── Split recovery tests ──────────────────────────────────────────────

    #[tokio::test]
    async fn epk_range_topology_change_returns_split_required() {
        let mut request = Request::new(operation(), epk_range_target());
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error())]);
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            ResolvedRange {
                partition_key_range_id: "1".to_string(),
                range: FeedRange::new(
                    EffectivePartitionKey::min(),
                    EffectivePartitionKey::from("40"),
                ),
            },
            ResolvedRange {
                partition_key_range_id: "2".to_string(),
                range: FeedRange::new(
                    EffectivePartitionKey::from("40"),
                    EffectivePartitionKey::from("80"),
                ),
            },
        ])]);
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        let result = request.next_page(&mut context).await.unwrap();
        match result {
            PageResult::SplitRequired { replacement_nodes } => {
                assert_eq!(replacement_nodes.len(), 2);

                let r0 = replacement_nodes[0].downcast_ref::<Request>().unwrap();
                assert_eq!(
                    r0.target(),
                    &RequestTarget::EffectivePartitionKeyRange {
                        range: FeedRange::new(
                            EffectivePartitionKey::min(),
                            EffectivePartitionKey::from("40"),
                        ),
                        partition_key_range_id: "1".to_string(),
                    }
                );

                let r1 = replacement_nodes[1].downcast_ref::<Request>().unwrap();
                assert_eq!(
                    r1.target(),
                    &RequestTarget::EffectivePartitionKeyRange {
                        range: FeedRange::new(
                            EffectivePartitionKey::from("40"),
                            EffectivePartitionKey::from("80"),
                        ),
                        partition_key_range_id: "2".to_string(),
                    }
                );
            }
            other => panic!("expected SplitRequired, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn split_left_child_inherits_continuation() {
        let mut request = Request::with_continuation(
            operation(),
            epk_range_target(),
            Some("server-token".to_string()),
        );
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error())]);
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            ResolvedRange {
                partition_key_range_id: "1".to_string(),
                range: FeedRange::new(
                    EffectivePartitionKey::min(),
                    EffectivePartitionKey::from("40"),
                ),
            },
            ResolvedRange {
                partition_key_range_id: "2".to_string(),
                range: FeedRange::new(
                    EffectivePartitionKey::from("40"),
                    EffectivePartitionKey::from("80"),
                ),
            },
        ])]);
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        let result = request.next_page(&mut context).await.unwrap();
        match result {
            PageResult::SplitRequired { replacement_nodes } => {
                let left = replacement_nodes[0].downcast_ref::<Request>().unwrap();
                assert_eq!(
                    left.latest_server_continuation(),
                    Some("server-token"),
                    "left-most child should inherit the server continuation"
                );

                let right = replacement_nodes[1].downcast_ref::<Request>().unwrap();
                assert_eq!(
                    right.latest_server_continuation(),
                    None,
                    "non-left children should have no continuation"
                );
            }
            other => panic!("expected SplitRequired, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn split_three_way_only_left_inherits_continuation() {
        let range = FeedRange::new(
            EffectivePartitionKey::from("10"),
            EffectivePartitionKey::from("90"),
        );
        let mut request = Request::with_continuation(
            operation(),
            RequestTarget::EffectivePartitionKeyRange {
                range: range.clone(),
                partition_key_range_id: "0".to_string(),
            },
            Some("ct".to_string()),
        );
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error())]);
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            ResolvedRange {
                partition_key_range_id: "1".to_string(),
                range: FeedRange::new(
                    EffectivePartitionKey::from("10"),
                    EffectivePartitionKey::from("40"),
                ),
            },
            ResolvedRange {
                partition_key_range_id: "2".to_string(),
                range: FeedRange::new(
                    EffectivePartitionKey::from("40"),
                    EffectivePartitionKey::from("70"),
                ),
            },
            ResolvedRange {
                partition_key_range_id: "3".to_string(),
                range: FeedRange::new(
                    EffectivePartitionKey::from("70"),
                    EffectivePartitionKey::from("90"),
                ),
            },
        ])]);
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        let result = request.next_page(&mut context).await.unwrap();
        match result {
            PageResult::SplitRequired { replacement_nodes } => {
                assert_eq!(replacement_nodes.len(), 3);
                let left = replacement_nodes[0].downcast_ref::<Request>().unwrap();
                assert_eq!(left.latest_server_continuation(), Some("ct"));
                let mid = replacement_nodes[1].downcast_ref::<Request>().unwrap();
                assert_eq!(mid.latest_server_continuation(), None);
                let right = replacement_nodes[2].downcast_ref::<Request>().unwrap();
                assert_eq!(right.latest_server_continuation(), None);
            }
            other => panic!("expected SplitRequired, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn topology_provider_error_propagates() {
        let mut request = Request::new(operation(), epk_range_target());
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error())]);
        let mut topology = MockTopologyProvider::new(vec![Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "topology fetch failed",
        ))]);
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        let err = request.next_page(&mut context).await.unwrap_err();
        assert_eq!(err.to_string(), "topology fetch failed");
    }

    #[tokio::test]
    async fn non_partitioned_topology_change_not_retried() {
        let mut request = Request::new(operation(), RequestTarget::NonPartitioned);
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error())]);
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        let err = request.next_page(&mut context).await.unwrap_err();
        assert!(is_partition_topology_change(&err));
    }
}
