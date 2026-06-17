// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Change feed options: mode selection, start position, and paging.

use azure_data_cosmos_driver::models::{MaxItemCountHint, SessionToken};
use azure_data_cosmos_driver::options::OperationOptions;
use time::OffsetDateTime;

use crate::feed::ContinuationToken;
use crate::options::FeedOptions;

/// Determines the change feed mode, which controls the shape of the response.
///
/// - [`LatestVersion`](Self::LatestVersion): Returns the latest version of each
///   changed document (creates and replaces only; deletes are not surfaced).
/// - [`AllVersionsAndDeletes`](Self::AllVersionsAndDeletes): Returns a
///   full-fidelity envelope for every change including deletes, with optional
///   previous images.
///
/// The default mode is [`LatestVersion`](Self::LatestVersion).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChangeFeedMode {
    /// Returns the latest version of each changed document.
    ///
    /// Wire header: `A-IM: Incremental feed`.
    LatestVersion,

    /// Returns a full-fidelity envelope for every operation (create, replace, delete).
    ///
    /// Wire header: `A-IM: Full-Fidelity Feed`.
    ///
    /// **Note:** This mode is defined for forward compatibility but is not
    /// fully wired in this version of the SDK. The envelope types
    /// (`ChangeFeedItem<T>`, `ChangeFeedMetadata`) will be added in a
    /// follow-up release.
    AllVersionsAndDeletes,
}

impl Default for ChangeFeedMode {
    fn default() -> Self {
        Self::LatestVersion
    }
}

/// Determines where the change feed starts reading from.
///
/// This is only consulted when no continuation token is provided. If a
/// continuation token is set, it carries its own position and this value is
/// ignored.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ChangeFeedStartFrom {
    /// Start from the beginning of the change feed (all available changes).
    ///
    /// No additional headers are sent.
    Beginning,

    /// Start from the current point in time (only changes after the request).
    ///
    /// Wire header: `If-None-Match: *`.
    Now,

    /// Start from a specific point in time.
    ///
    /// Wire header: `If-Modified-Since: <RFC 1123 timestamp>`.
    PointInTime(OffsetDateTime),
}

impl Default for ChangeFeedStartFrom {
    fn default() -> Self {
        Self::Beginning
    }
}

/// Options for change feed operations.
///
/// Used by [`ContainerClient::read_change_feed()`](crate::clients::ContainerClient::read_change_feed).
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
/// When a continuation token is set, [`mode`](Self::mode) and
/// [`start_from`](Self::start_from) are ignored because the token carries its
/// own position and mode.
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

    /// The change feed mode. Defaults to [`ChangeFeedMode::LatestVersion`].
    pub mode: ChangeFeedMode,

    /// Where to start reading the change feed. Defaults to [`ChangeFeedStartFrom::Beginning`].
    pub start_from: ChangeFeedStartFrom,
}

impl ChangeFeedOptions {
    /// Sets the change feed mode.
    pub fn with_mode(mut self, mode: ChangeFeedMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets where to start reading the change feed.
    pub fn with_start_from(mut self, start_from: ChangeFeedStartFrom) -> Self {
        self.start_from = start_from;
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
    /// When continuation is set, [`mode`](Self::mode) and
    /// [`start_from`](Self::start_from) are ignored because the token carries
    /// its own position and mode.
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
    }

    #[test]
    fn default_start_from_is_beginning() {
        assert!(matches!(
            ChangeFeedStartFrom::default(),
            ChangeFeedStartFrom::Beginning
        ));
    }

    #[test]
    fn options_builder_chain() {
        let opts = ChangeFeedOptions::default()
            .with_mode(ChangeFeedMode::LatestVersion)
            .with_start_from(ChangeFeedStartFrom::Now);

        assert_eq!(opts.mode, ChangeFeedMode::LatestVersion);
        assert!(matches!(opts.start_from, ChangeFeedStartFrom::Now));
    }
}
