// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Feed/query options: paging, query metrics, and continuation tokens.

use azure_data_cosmos_driver::models::{MaxItemCountHint, SessionToken};
use azure_data_cosmos_driver::options::OperationOptions;

use crate::feed::ContinuationToken;

/// Options that apply to feed-style operations (paged reads, queries, etc.).
///
/// These settings control paging behavior — how many items the service should
/// return per page and where to resume from. They are surfaced as a separate
/// struct so other feed-style APIs can adopt them without re-declaring the
/// same fields.
///
/// Today, `FeedOptions` is composed into [`QueryOptions`] via its
/// [`feed`](QueryOptions::feed) field; [`QueryOptions`] also exposes
/// [`with_max_item_count`](QueryOptions::with_max_item_count) and
/// [`with_continuation_token`](QueryOptions::with_continuation_token)
/// shortcuts that delegate to the inner [`FeedOptions`].
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct FeedOptions {
    /// Maximum number of items the service should return per page
    /// (`x-ms-max-item-count`).
    ///
    /// `None` omits the header so the SDK / service defaults apply. See
    /// [`MaxItemCountHint`] for the two explicit values.
    ///
    /// This is a _hint_ to the server, not a client-side guarantee of the
    /// maximum returned page size. In a cross-partition query, each partition
    /// may return up to this many items, so the total page size could be up
    /// to this value times the number of partitions involved.
    pub max_item_count: Option<MaxItemCountHint>,

    /// Continuation token from a prior page iterator, used to resume the feed.
    ///
    /// See [`QueryPageIterator::to_continuation_token`](crate::feed::QueryPageIterator::to_continuation_token).
    pub continuation_token: Option<ContinuationToken>,

    /// Maximum number of physical partitions a cross-partition query may fan
    /// out to.
    ///
    /// When `None`, the SDK uses its built-in default
    /// (`QueryOptions::DEFAULT_MAX_FAN_OUT` = 100). Setting this to a higher
    /// value allows queries that span very large containers, though such
    /// queries are typically expensive and should be avoided in
    /// latency-sensitive paths.
    ///
    /// If a query would target more physical partitions than this limit,
    /// [`plan_operation`](azure_data_cosmos_driver::driver::CosmosDriver::plan_operation)
    /// returns an error with status 400 / sub-status 20307.
    pub max_fan_out: Option<usize>,
}

impl FeedOptions {
    /// Sets the maximum number of items the service should return per page.
    ///
    /// Pass [`MaxItemCountHint::Limit`] with a concrete page size, or
    /// [`MaxItemCountHint::ServerDecides`] to let the service choose.
    pub fn with_max_item_count(mut self, max_item_count: MaxItemCountHint) -> Self {
        self.max_item_count = Some(max_item_count);
        self
    }

    /// Sets a continuation token to resume the feed at a previous position.
    pub fn with_continuation_token(mut self, continuation_token: ContinuationToken) -> Self {
        self.continuation_token = Some(continuation_token);
        self
    }

    /// Sets the maximum number of physical partitions a cross-partition query
    /// may fan out to.
    ///
    /// Overrides the built-in default (100). Pass a larger value only when
    /// you understand the performance implications of a wide fan-out query.
    pub fn with_max_fan_out(mut self, max_fan_out: usize) -> Self {
        self.max_fan_out = Some(max_fan_out);
        self
    }
}

/// Options for query operations.
///
/// Used by [`ContainerClient::query_items()`](crate::clients::ContainerClient::query_items()).
///
/// General-purpose settings such as custom headers and excluded regions are configured
/// via the [`with_operation_options`](Self::with_operation_options) setter. See [`OperationOptions`] for details.
///
/// Paging-related settings (`max_item_count`, `continuation_token`) are configured via
/// the [`feed`](Self::feed) field — see [`FeedOptions`]. The convenience setters
/// [`with_max_item_count`](Self::with_max_item_count) and
/// [`with_continuation_token`](Self::with_continuation_token) delegate to the inner
/// [`FeedOptions`].
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Feed-paging options (max item count, continuation token) for this query.
    /// See [`FeedOptions`].
    pub feed: FeedOptions,

    /// Session token for session-consistent queries.
    pub session_token: Option<SessionToken>,

    /// When `true`, request that the service include index utilization metrics
    /// in the response (`x-ms-cosmos-populateindexmetrics`). The decoded JSON is
    /// surfaced via `QueryFeedPage::index_metrics()`.
    pub populate_index_metrics: Option<bool>,

    /// When `true`, request that the service include per-query metrics in the
    /// response (`x-ms-documentdb-populatequerymetrics`). Surfaced via
    /// `QueryFeedPage::query_metrics()`.
    pub populate_query_metrics: Option<bool>,
}

impl QueryOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }

    /// Sets the [`FeedOptions`] (max item count, continuation token) for this query.
    pub fn with_feed_options(mut self, feed: FeedOptions) -> Self {
        self.feed = feed;
        self
    }

    /// Enables or disables index-utilization metric collection for this query.
    pub fn with_populate_index_metrics(mut self, enable: bool) -> Self {
        self.populate_index_metrics = Some(enable);
        self
    }

    /// Enables or disables per-query metric collection for this query.
    pub fn with_populate_query_metrics(mut self, enable: bool) -> Self {
        self.populate_query_metrics = Some(enable);
        self
    }

    /// Sets the maximum number of items the service should return per page.
    ///
    /// Delegates to [`FeedOptions::with_max_item_count`] on the inner
    /// [`feed`](Self::feed). Pass [`MaxItemCountHint::Limit`] with a concrete
    /// page size, or [`MaxItemCountHint::ServerDecides`] to let the service
    /// choose.
    pub fn with_max_item_count(mut self, max_item_count: MaxItemCountHint) -> Self {
        self.feed = self.feed.with_max_item_count(max_item_count);
        self
    }

    /// Sets a continuation token to resume the query at a previous position.
    ///
    /// Delegates to [`FeedOptions::with_continuation_token`] on the inner
    /// [`feed`](Self::feed).
    pub fn with_continuation_token(mut self, continuation_token: ContinuationToken) -> Self {
        self.feed = self.feed.with_continuation_token(continuation_token);
        self
    }

    /// Sets the maximum number of physical partitions a cross-partition query
    /// may fan out to.
    ///
    /// Delegates to [`FeedOptions::with_max_fan_out`] on the inner
    /// [`feed`](Self::feed). Pass a larger value only when you understand the
    /// performance implications of a wide fan-out query.
    ///
    /// The default is 100. Queries that target more physical partitions than
    /// this limit fail with a 400 error.
    pub fn with_max_fan_out(mut self, max_fan_out: usize) -> Self {
        self.feed = self.feed.with_max_fan_out(max_fan_out);
        self
    }
}
