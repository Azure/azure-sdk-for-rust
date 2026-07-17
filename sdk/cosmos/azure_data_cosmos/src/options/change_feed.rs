// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Change feed options: start position and paging.

use azure_data_cosmos_driver::models::{MaxItemCountHint, SessionToken};
use azure_data_cosmos_driver::options::OperationOptions;

use crate::feed::ContinuationToken;
use crate::options::FeedOptions;

/// Determines where the change feed starts reading from.
///
/// There is no default: callers pass this explicitly to
/// [`ContainerClient::query_change_feed()`](crate::clients::ContainerClient::query_change_feed).
/// It is only consulted when no continuation token is provided. If a
/// continuation token is set, it carries its own position and this value is
/// ignored.
///
/// Re-exported from the driver, which owns the mapping from each start position
/// to its wire header and persists the position in the continuation token.
pub use azure_data_cosmos_driver::models::ChangeFeedStartFrom;

/// Selects which change feed mode to read.
///
/// Only [`LatestVersion`](Self::LatestVersion) is supported today. The enum is
/// `#[non_exhaustive]`, so additional modes can be added in a future release
/// without breaking callers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChangeFeedMode {
    /// Returns the latest version of each changed item ("incremental" feed).
    #[default]
    LatestVersion,
}

/// Options for change feed operations.
///
/// Used by [`ContainerClient::query_change_feed()`](crate::clients::ContainerClient::query_change_feed).
///
/// General-purpose settings such as custom headers and excluded regions are
/// configured via [`with_operation_options`](Self::with_operation_options).
/// See [`OperationOptions`] for details.
///
/// Paging-related settings (`max_item_count`, `continuation_token`) are
/// configured via the [`feed`](Self::feed) field — see [`FeedOptions`]. The
/// convenience setters [`with_max_item_count`](Self::with_max_item_count) and
/// [`with_continuation_token`](Self::with_continuation_token) delegate to the
/// inner [`FeedOptions`].
///
/// The start position is **not** part of these options: it is a required
/// argument of
/// [`ContainerClient::query_change_feed()`](crate::clients::ContainerClient::query_change_feed).
/// When a continuation token is set, that start position is ignored because the
/// token carries its own.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ChangeFeedOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Feed-paging options (max item count, continuation token) for this change feed read.
    /// See [`FeedOptions`].
    pub feed: FeedOptions,

    /// Session token for session-consistent reads.
    pub session_token: Option<SessionToken>,

    /// Which change feed mode to read. Defaults to [`ChangeFeedMode::LatestVersion`].
    pub mode: ChangeFeedMode,
}

impl ChangeFeedOptions {
    /// Sets which change feed mode to read.
    pub fn with_mode(mut self, mode: ChangeFeedMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the session token for this request.
    pub fn with_session_token(mut self, token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(token.into());
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }

    /// Sets the [`FeedOptions`] (max item count, continuation token) for this request.
    pub fn with_feed_options(mut self, feed: FeedOptions) -> Self {
        self.feed = feed;
        self
    }

    /// Sets the maximum number of items the service should return per page.
    ///
    /// Delegates to [`FeedOptions::with_max_item_count`] on the inner
    /// [`feed`](Self::feed).
    pub fn with_max_item_count(mut self, max_item_count: MaxItemCountHint) -> Self {
        self.feed = self.feed.with_max_item_count(max_item_count);
        self
    }

    /// Sets a continuation token to resume the change feed at a previous position.
    ///
    /// When continuation is set, the `start_from` argument to
    /// [`query_change_feed`](crate::clients::ContainerClient::query_change_feed)
    /// is ignored because the token carries its own position.
    pub fn with_continuation_token(mut self, token: ContinuationToken) -> Self {
        self.feed = self.feed.with_continuation_token(token);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mode_is_latest_version() {
        assert_eq!(ChangeFeedMode::default(), ChangeFeedMode::LatestVersion);
        assert_eq!(
            ChangeFeedOptions::default().mode,
            ChangeFeedMode::LatestVersion
        );
    }

    #[test]
    fn options_builder_chain() {
        let opts = ChangeFeedOptions::default().with_mode(ChangeFeedMode::LatestVersion);

        assert_eq!(opts.mode, ChangeFeedMode::LatestVersion);
    }
}
