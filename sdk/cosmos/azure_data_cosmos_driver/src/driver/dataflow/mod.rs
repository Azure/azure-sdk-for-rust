// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Dataflow pipeline nodes for paged Cosmos DB operations.

mod request;

use futures::future::BoxFuture;

use crate::models::{CosmosOperation, CosmosResponse};

pub(crate) use request::{Request, RequestTarget};

/// Request execution mode for partition routing metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PartitionRoutingRefresh {
    /// Use existing partition routing metadata.
    UseCached,
    /// Force partition routing metadata to be refreshed before executing.
    ForceRefresh,
}

/// Executes leaf request nodes through the existing operation pipeline.
pub(crate) trait RequestExecutor: Send {
    /// Executes a single request node.
    fn execute_request<'a>(
        &'a mut self,
        operation: &'a CosmosOperation,
        target: RequestTarget,
        partition_routing_refresh: PartitionRoutingRefresh,
        continuation: Option<String>,
    ) -> BoxFuture<'a, azure_core::Result<CosmosResponse>>;
}

/// Context passed through dataflow node execution.
pub(crate) struct PipelineContext<'a> {
    request_executor: &'a mut dyn RequestExecutor,
}

impl<'a> PipelineContext<'a> {
    /// Creates a new pipeline execution context.
    pub(crate) fn new(request_executor: &'a mut dyn RequestExecutor) -> Self {
        Self { request_executor }
    }

    async fn execute_request(
        &mut self,
        operation: &CosmosOperation,
        target: RequestTarget,
        partition_routing_refresh: PartitionRoutingRefresh,
        continuation: Option<String>,
    ) -> azure_core::Result<CosmosResponse> {
        self.request_executor
            .execute_request(operation, target, partition_routing_refresh, continuation)
            .await
    }
}

/// A dataflow node that emits pages and may own child nodes.
///
/// Each `next_page` call boxes a future via `async_trait`; the per-page
/// allocation is negligible compared to the multi-millisecond network I/O
/// of a Cosmos DB request.
#[async_trait::async_trait]
pub(crate) trait PipelineNode: Send {
    /// Emits the next page of results, or `None` when this node is drained.
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> azure_core::Result<Option<CosmosResponse>>;

    /// Returns the node's strongly-owned children.
    fn children(&self) -> &[Box<dyn PipelineNode>];
}

/// A pipeline root that owns the node tree.
pub(crate) struct Pipeline {
    root: Box<dyn PipelineNode>,
}

impl Pipeline {
    /// Creates a pipeline from an owned root node.
    pub(crate) fn new(root: Box<dyn PipelineNode>) -> Self {
        Self { root }
    }

    /// Emits the next page from the root node.
    pub(crate) async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> azure_core::Result<Option<CosmosResponse>> {
        self.root.next_page(context).await
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, sync::Arc};

    use futures::future::BoxFuture;

    use super::*;
    use crate::{
        diagnostics::DiagnosticsContextBuilder,
        models::{ActivityId, CosmosResponseHeaders, CosmosStatus},
        options::DiagnosticsOptions,
    };

    struct MockLeaf {
        pages: VecDeque<azure_core::Result<Option<CosmosResponse>>>,
    }

    impl MockLeaf {
        fn with_pages(pages: Vec<azure_core::Result<Option<CosmosResponse>>>) -> Self {
            Self {
                pages: pages.into(),
            }
        }
    }

    #[async_trait::async_trait]
    impl PipelineNode for MockLeaf {
        async fn next_page(
            &mut self,
            _context: &mut PipelineContext<'_>,
        ) -> azure_core::Result<Option<CosmosResponse>> {
            self.pages.pop_front().expect("mock page result")
        }

        fn children(&self) -> &[Box<dyn PipelineNode>] {
            &[]
        }
    }

    struct NoopRequestExecutor;

    impl RequestExecutor for NoopRequestExecutor {
        fn execute_request<'a>(
            &'a mut self,
            _operation: &'a CosmosOperation,
            _target: RequestTarget,
            _partition_routing_refresh: PartitionRoutingRefresh,
            _continuation: Option<String>,
        ) -> BoxFuture<'a, azure_core::Result<CosmosResponse>> {
            Box::pin(async {
                Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "noop executor should not be called",
                ))
            })
        }
    }

    fn response(body: &[u8]) -> CosmosResponse {
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::new_uuid(),
            Arc::new(DiagnosticsOptions::default()),
        );
        diagnostics.set_operation_status(azure_core::http::StatusCode::Ok, None);
        CosmosResponse::new(
            body.to_vec(),
            CosmosResponseHeaders::new(),
            CosmosStatus::new(azure_core::http::StatusCode::Ok),
            Arc::new(diagnostics.complete()),
        )
    }

    #[tokio::test]
    async fn pipeline_forwards_pages_from_root() {
        let mut pipeline = Pipeline::new(Box::new(MockLeaf::with_pages(vec![Ok(Some(response(
            b"page",
        )))])));
        let mut executor = NoopRequestExecutor;
        let mut context = PipelineContext::new(&mut executor);

        let page = pipeline.next_page(&mut context).await.unwrap().unwrap();

        assert_eq!(page.body(), b"page");
    }
}
