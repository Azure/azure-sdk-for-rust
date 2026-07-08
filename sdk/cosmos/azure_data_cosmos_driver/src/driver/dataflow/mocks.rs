// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared test mocks for dataflow pipeline testing.

use std::{collections::VecDeque, sync::Arc};

use azure_core::http::StatusCode;
use futures::future::BoxFuture;

use super::{
    PageResult, PartitionRoutingRefresh, PipelineContext, PipelineNode, PipelineNodeState,
    RequestExecutor, RequestTarget, ResolvedRange, TopologyProvider,
};
use crate::{
    diagnostics::DiagnosticsContextBuilder,
    models::{
        effective_partition_key::EffectivePartitionKey, AccountReference, ActivityId,
        CosmosOperation, CosmosResponse, CosmosResponseHeaders, CosmosStatus, DatabaseReference,
        FeedRange, PartitionKey, SubStatusCode,
    },
    options::DiagnosticsOptions,
};

// ── Mock pipeline node ──────────────────────────────────────────────────────

/// A mock leaf node that returns pre-configured page results.
pub(crate) struct MockLeaf {
    pages: VecDeque<crate::error::Result<PageResult>>,
    feed_range: Option<FeedRange>,
    snapshot: Option<PipelineNodeState>,
}

impl MockLeaf {
    /// Creates a mock leaf with a sequence of results to return from `next_page`.
    pub fn with_pages(pages: Vec<crate::error::Result<PageResult>>) -> Self {
        Self {
            pages: pages.into(),
            feed_range: None,
            snapshot: None,
        }
    }

    /// Sets the feed range reported by [`PipelineNode::feed_range`].
    pub fn with_feed_range(mut self, range: FeedRange) -> Self {
        self.feed_range = Some(range);
        self
    }

    /// Overrides the state returned by [`PipelineNode::snapshot_state`].
    /// Defaults to [`PipelineNodeState::Drained`] when unset.
    pub fn with_snapshot(mut self, state: PipelineNodeState) -> Self {
        self.snapshot = Some(state);
        self
    }
}

#[async_trait::async_trait]
impl PipelineNode for MockLeaf {
    async fn next_page(
        &mut self,
        _context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<PageResult> {
        self.pages
            .pop_front()
            .expect("MockLeaf: no more page results")
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        vec![]
    }

    fn snapshot_state(&self) -> crate::error::Result<PipelineNodeState> {
        Ok(self.snapshot.clone().unwrap_or(PipelineNodeState::Drained))
    }

    fn feed_range(&self) -> Option<&FeedRange> {
        self.feed_range.as_ref()
    }

    fn topology_can_change(&self) -> bool {
        // A MockLeaf is just a test stub and doesn't represent a real request, so it can't be the target of a topology change error that requires splitting or merging.
        false
    }
}

// ── Request executors ───────────────────────────────────────────────────────

/// A request executor that should never be called.
pub(crate) struct NoopRequestExecutor;

impl RequestExecutor for NoopRequestExecutor {
    fn execute_request<'a>(
        &'a mut self,
        _operation: &'a CosmosOperation,
        _target: RequestTarget,
        _partition_routing_refresh: PartitionRoutingRefresh,
        _continuation: Option<String>,
    ) -> BoxFuture<'a, crate::error::Result<CosmosResponse>> {
        Box::pin(async {
            Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("noop executor should not be called")
                .build())
        })
    }
}

/// A mock request executor that records calls and returns pre-configured responses.
pub(crate) struct MockRequestExecutor {
    pub responses: VecDeque<crate::error::Result<CosmosResponse>>,
    pub refresh_calls: Vec<PartitionRoutingRefresh>,
    pub continuation_calls: Vec<Option<String>>,
    pub target_calls: Vec<RequestTarget>,
}

impl MockRequestExecutor {
    pub fn new(responses: Vec<crate::error::Result<CosmosResponse>>) -> Self {
        Self {
            responses: responses.into(),
            refresh_calls: Vec::new(),
            continuation_calls: Vec::new(),
            target_calls: Vec::new(),
        }
    }
}

impl RequestExecutor for MockRequestExecutor {
    fn execute_request<'a>(
        &'a mut self,
        _operation: &'a CosmosOperation,
        target: RequestTarget,
        partition_routing_refresh: PartitionRoutingRefresh,
        continuation: Option<String>,
    ) -> BoxFuture<'a, crate::error::Result<CosmosResponse>> {
        self.refresh_calls.push(partition_routing_refresh);
        self.continuation_calls.push(continuation);
        self.target_calls.push(target);
        let response = self.responses.pop_front().expect("mock request response");
        Box::pin(async move { response })
    }
}

// ── Topology providers ─────────────────────────────────────────────────────

/// A topology provider that should never be called.
pub(crate) struct NoopTopologyProvider;

impl TopologyProvider for NoopTopologyProvider {
    fn resolve_ranges<'a>(
        &'a mut self,
        _range: &'a FeedRange,
        _refresh: PartitionRoutingRefresh,
    ) -> BoxFuture<'a, crate::error::Result<Vec<ResolvedRange>>> {
        Box::pin(async {
            Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("noop topology provider should not be called")
                .build())
        })
    }
}

/// A mock topology provider that returns pre-configured resolved ranges.
pub(crate) struct MockTopologyProvider {
    results: VecDeque<crate::error::Result<Vec<ResolvedRange>>>,
}

impl MockTopologyProvider {
    pub fn new(results: Vec<crate::error::Result<Vec<ResolvedRange>>>) -> Self {
        Self {
            results: results.into(),
        }
    }
}

impl TopologyProvider for MockTopologyProvider {
    fn resolve_ranges<'a>(
        &'a mut self,
        _range: &'a FeedRange,
        _refresh: PartitionRoutingRefresh,
    ) -> BoxFuture<'a, crate::error::Result<Vec<ResolvedRange>>> {
        let result = self
            .results
            .pop_front()
            .expect("MockTopologyProvider: no more results");
        Box::pin(async move { result })
    }
}

// ── Test helpers ────────────────────────────────────────────────────────────

/// Extracts the `CosmosResponse` from a `PageResult::Page`, panicking otherwise.
pub(crate) fn unwrap_page(result: crate::error::Result<PageResult>) -> CosmosResponse {
    match result.expect("expected Ok result") {
        PageResult::Page { response, .. } => response,
        PageResult::Drained => panic!("expected Page, got Drained"),
        PageResult::SplitRequired { .. } => panic!("expected Page, got SplitRequired"),
    }
}

/// Asserts that a `PageResult` is `Drained`.
pub(crate) fn assert_drained(result: crate::error::Result<PageResult>) {
    match result.expect("expected Ok result") {
        PageResult::Drained => {}
        PageResult::Page { .. } => panic!("expected Drained, got Page"),
        PageResult::SplitRequired { .. } => panic!("expected Drained, got SplitRequired"),
    }
}

/// Creates a test `CosmosOperation`.
pub(crate) fn operation() -> CosmosOperation {
    let account = AccountReference::with_master_key(
        url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        "dGVzdA==",
    );
    let database = DatabaseReference::from_name(account, "db".to_owned());
    CosmosOperation::read_database(database)
}

/// Creates a `RequestTarget` for a logical partition key.
pub(crate) fn logical_partition_target() -> RequestTarget {
    RequestTarget::LogicalPartitionKey(PartitionKey::from("pk"))
}

/// Creates a `RequestTarget` for an EPK range ("" to "80", partition key range ID "0").
pub(crate) fn epk_range_target() -> RequestTarget {
    let range = FeedRange::new(
        EffectivePartitionKey::MIN,
        EffectivePartitionKey::from("80"),
    )
    .unwrap();
    RequestTarget::effective_partition_key_range(range.clone(), "0".to_string(), range)
}

/// Creates a test response with the given body.
pub(crate) fn response(body: &[u8]) -> CosmosResponse {
    response_with_continuation(body, None)
}

/// Creates a test response with the given body and optional continuation token.
pub(crate) fn response_with_continuation(
    body: &[u8],
    continuation: Option<&str>,
) -> CosmosResponse {
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

/// Creates a 410 Gone error with a partition topology change substatus.
pub(crate) fn gone_error() -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::new(
            azure_core::http::StatusCode::InternalServerError,
        ))
        .with_status(CosmosStatus::from_parts(
            StatusCode::Gone,
            Some(SubStatusCode::PARTITION_KEY_RANGE_GONE),
        ))
        .with_message("partition topology changed")
        .with_response_parts(crate::models::CosmosResponsePayload::new(
            Vec::new(),
            CosmosResponseHeaders::default(),
        ))
        .build()
}

/// Creates a 410 Gone error with a non-topology substatus.
pub(crate) fn non_topology_gone_error() -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::new(
            azure_core::http::StatusCode::InternalServerError,
        ))
        .with_status(CosmosStatus::from_parts(
            StatusCode::Gone,
            Some(SubStatusCode::NAME_CACHE_STALE),
        ))
        .with_message("name cache is stale")
        .with_response_parts(crate::models::CosmosResponsePayload::new(
            Vec::new(),
            CosmosResponseHeaders::default(),
        ))
        .build()
}
