// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request leaf node for the dataflow pipeline.

use async_trait::async_trait;
use azure_core::http::StatusCode;

use crate::models::{CosmosOperation, CosmosResponse, FeedRange, PartitionKey, SubStatusCode};

use super::{PartitionRoutingRefresh, PipelineContext, PipelineNode};

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
    ) -> azure_core::Result<Option<CosmosResponse>> {
        match context
            .execute_request(
                &self.operation,
                self.target.clone(),
                PartitionRoutingRefresh::UseCached,
                self.latest_server_continuation.clone(),
            )
            .await
        {
            Ok(response) => Ok(Some(self.record_response_continuation(response))),
            Err(error) if is_partition_topology_change(&error) => {
                self.handle_partition_topology_change(context, error).await
            }
            Err(error) => Err(error),
        }
    }

    fn children(&self) -> &[Box<dyn PipelineNode>] {
        &[]
    }
}

impl Request {
    async fn handle_partition_topology_change(
        &mut self,
        context: &mut PipelineContext<'_>,
        error: azure_core::Error,
    ) -> azure_core::Result<Option<CosmosResponse>> {
        match &self.target {
            RequestTarget::NonPartitioned => {
                // Non-partitioned resources don't have partition topology changes.
                Err(error)
            }
            RequestTarget::LogicalPartitionKey(_) => {
                if self.logical_partition_topology_retry_used {
                    return Err(error);
                }

                self.logical_partition_topology_retry_used = true;
                context
                    .execute_request(
                        &self.operation,
                        self.target.clone(),
                        PartitionRoutingRefresh::ForceRefresh,
                        self.latest_server_continuation.clone(),
                    )
                    .await
                    .map(|response| self.record_response_continuation(response))
                    .map(Some)
            }
            RequestTarget::EffectivePartitionKeyRange { .. } => {
                panic!(
                    "EPK range request encountered a partition topology change; pipeline repair is not implemented"
                );
            }
        }
    }

    fn record_response_continuation(&mut self, response: CosmosResponse) -> CosmosResponse {
        self.latest_server_continuation = response.headers().continuation.clone();
        response
    }
}

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
    use std::{collections::VecDeque, sync::Arc};

    use azure_core::error::ErrorKind;
    use futures::future::BoxFuture;

    use super::*;
    use crate::{
        diagnostics::DiagnosticsContextBuilder,
        driver::dataflow::RequestExecutor,
        models::{
            effective_partition_key::EffectivePartitionKey, AccountReference, ActivityId,
            CosmosResponseHeaders, CosmosStatus, DatabaseReference,
        },
        options::DiagnosticsOptions,
    };

    struct MockRequestExecutor {
        responses: VecDeque<azure_core::Result<CosmosResponse>>,
        refresh_calls: Vec<PartitionRoutingRefresh>,
        continuation_calls: Vec<Option<String>>,
    }

    impl MockRequestExecutor {
        fn new(responses: Vec<azure_core::Result<CosmosResponse>>) -> Self {
            Self {
                responses: responses.into(),
                refresh_calls: Vec::new(),
                continuation_calls: Vec::new(),
            }
        }
    }

    impl RequestExecutor for MockRequestExecutor {
        fn execute_request<'a>(
            &'a mut self,
            _operation: &'a CosmosOperation,
            _target: RequestTarget,
            partition_routing_refresh: PartitionRoutingRefresh,
            continuation: Option<String>,
        ) -> BoxFuture<'a, azure_core::Result<CosmosResponse>> {
            self.refresh_calls.push(partition_routing_refresh);
            self.continuation_calls.push(continuation);
            let response = self.responses.pop_front().expect("mock request response");
            Box::pin(async move { response })
        }
    }

    fn operation() -> CosmosOperation {
        let account = AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let database = DatabaseReference::from_name(account, "db".to_owned());
        CosmosOperation::read_database(database)
    }

    fn logical_partition_target() -> RequestTarget {
        RequestTarget::LogicalPartitionKey(PartitionKey::from("pk"))
    }

    fn epk_range_target() -> RequestTarget {
        RequestTarget::EffectivePartitionKeyRange {
            range: FeedRange::new(
                EffectivePartitionKey::min(),
                EffectivePartitionKey::from("80"),
            ),
            partition_key_range_id: "0".to_string(),
        }
    }

    fn response(body: &[u8]) -> CosmosResponse {
        response_with_continuation(body, None)
    }

    fn response_with_continuation(body: &[u8], continuation: Option<&str>) -> CosmosResponse {
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::new_uuid(),
            Arc::new(DiagnosticsOptions::default()),
        );
        diagnostics.set_operation_status(StatusCode::Ok, None);
        let mut headers = CosmosResponseHeaders::new();
        headers.continuation = continuation.map(str::to_owned);
        CosmosResponse::new(
            body.to_vec(),
            headers,
            CosmosStatus::new(StatusCode::Ok),
            Arc::new(diagnostics.complete()),
        )
    }

    fn gone_error() -> azure_core::Error {
        azure_core::Error::new(
            ErrorKind::HttpResponse {
                status: StatusCode::Gone,
                error_code: Some(SubStatusCode::PARTITION_KEY_RANGE_GONE.value().to_string()),
                raw_response: None,
            },
            "partition topology changed",
        )
    }

    fn non_topology_gone_error() -> azure_core::Error {
        azure_core::Error::new(
            ErrorKind::HttpResponse {
                status: StatusCode::Gone,
                error_code: Some(SubStatusCode::NAME_CACHE_STALE.value().to_string()),
                raw_response: None,
            },
            "name cache is stale",
        )
    }

    #[tokio::test]
    async fn request_retries_logical_partition_key_topology_change_once() {
        let mut request = Request::new(operation(), logical_partition_target());
        let mut executor = MockRequestExecutor::new(vec![Err(gone_error()), Ok(response(b"ok"))]);
        let mut context = PipelineContext::new(&mut executor);

        let page = request.next_page(&mut context).await.unwrap().unwrap();

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
        let mut context = PipelineContext::new(&mut executor);

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
        let mut context = PipelineContext::new(&mut executor);

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
        let mut context = PipelineContext::new(&mut executor);

        let page1 = request.next_page(&mut context).await.unwrap().unwrap();
        let page2 = request.next_page(&mut context).await.unwrap().unwrap();

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
        let mut context = PipelineContext::new(&mut executor);

        let page = request.next_page(&mut context).await.unwrap().unwrap();

        assert_eq!(page.body(), b"page");
        assert_eq!(
            executor.continuation_calls,
            vec![Some("restored-token".to_string())]
        );
        assert_eq!(request.latest_server_continuation(), None);
    }
}
