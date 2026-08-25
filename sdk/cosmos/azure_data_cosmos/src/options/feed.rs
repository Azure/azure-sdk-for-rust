// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Feed/query options: paging, query metrics, and continuation tokens.

use azure_data_cosmos_driver::models::{MaxItemCountHint, SessionToken};
use azure_data_cosmos_driver::options::{OperationOptions, PlanOptions, DEFAULT_MAX_FAN_OUT};

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

    /// Maximum number of physical partitions a fresh cross-partition operation
    /// may fan out to.
    ///
    /// Cross-partition queries and change feeds are expensive by design: a
    /// container can have a very large number of physical partitions, and an
    /// accidental broad query can span all of them. To guard against this, the
    /// SDK refuses to start a fresh operation that would fan out to more than
    /// this many partitions.
    ///
    /// `None` applies the default of [`DEFAULT_MAX_FAN_OUT`]. `Some(0)` is
    /// treated the same as `None` — a fan-out of zero is meaningless, so it
    /// falls back to the default rather than rejecting every operation. To run
    /// a broader cross-partition operation, set this to a larger value — there
    /// is no separate "unlimited" setting; pass a value large enough for the
    /// workload.
    ///
    /// The limit is only checked at **initial query setup**, against the
    /// partition topology at that moment. It is not a runtime cap: if a
    /// partition splits while the operation is executing and pushes the
    /// effective fan-out above this value, the operation continues to run and is
    /// not aborted. Likewise, resuming from a `continuation_token` does not
    /// re-check the limit, since the fan-out was already accepted when the
    /// operation started.
    pub max_fan_out: Option<u32>,
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

    /// Sets the maximum number of physical partitions a fresh cross-partition
    /// operation may fan out to.
    ///
    /// See [`max_fan_out`](Self::max_fan_out) for details. There is no separate
    /// "unlimited" setting; pass a value large enough for the workload.
    pub fn with_max_fan_out(mut self, max_fan_out: u32) -> Self {
        self.max_fan_out = Some(max_fan_out);
        self
    }

    /// Builds driver [`PlanOptions`] from these feed options, applying the
    /// default fan-out when the caller did not set one (or set it to `0`).
    ///
    /// Kept as a crate-private inherent method rather than a `From`
    /// implementation so the driver's `PlanOptions` type does not appear on
    /// this crate's public surface.
    pub(crate) fn to_plan_options(&self) -> PlanOptions {
        let max_fan_out = match self.max_fan_out {
            None | Some(0) => DEFAULT_MAX_FAN_OUT,
            Some(n) => n,
        };
        PlanOptions::default().with_max_fan_out(max_fan_out)
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

    pub(crate) fn to_plan_options(&self) -> PlanOptions {
        self.feed.to_plan_options()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_options_uses_default_fan_out_when_unset() {
        let plan_options = FeedOptions::default().to_plan_options();
        assert_eq!(plan_options.max_fan_out, DEFAULT_MAX_FAN_OUT);
    }

    #[test]
    fn plan_options_treats_zero_fan_out_as_default() {
        let feed = FeedOptions::default().with_max_fan_out(0);
        let plan_options = feed.to_plan_options();
        assert_eq!(plan_options.max_fan_out, DEFAULT_MAX_FAN_OUT);
    }

    #[test]
    fn plan_options_carries_explicit_fan_out() {
        let feed = FeedOptions::default().with_max_fan_out(250);
        let plan_options = feed.to_plan_options();
        assert_eq!(plan_options.max_fan_out, 250);
    }
}
