// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Change feed state management for tracking progress across requests.

use azure_core::time::OffsetDateTime;
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};

use crate::routing::range::Range;

use super::{
    feed_range_composite_continuation::FeedRangeCompositeContinuation,
    feed_range_internal::FeedRangeInternal, mode::ChangeFeedMode, start_from::ChangeFeedStartFrom,
    start_from::ChangeFeedStartFromInternal,
};

/// The complete state of a change feed query, including position and configuration.
///
/// This state is serialized to/from continuation tokens to enable resuming change feed
/// processing across requests or even across application restarts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeFeedState {
    /// Version identifier for the state format.
    #[serde(rename = "v")]
    pub version: String,

    /// The container resource ID.
    #[serde(rename = "containerRid")]
    pub container_rid: String,

    /// The change feed mode.
    #[serde(rename = "mode")]
    pub mode: String,

    /// The starting point configuration.
    #[serde(rename = "startFrom")]
    pub start_from: ChangeFeedStartFromInternal,

    /// The composite continuation tracking progress across sub-ranges.
    #[serde(rename = "continuation")]
    pub continuation: FeedRangeCompositeContinuation,
}

impl ChangeFeedState {
    /// Creates a state with pre-resolved sub-ranges for each physical partition.
    ///
    /// This avoids an initial "full range → split" cycle by starting with the known partition layout.
    pub fn with_sub_ranges(
        container_rid: String,
        feed_range: FeedRangeInternal,
        start_from: &ChangeFeedStartFrom,
        mode: ChangeFeedMode,
        sub_ranges: Vec<Range<String>>,
    ) -> Self {
        let mode_str = match mode {
            ChangeFeedMode::LatestVersion => "LatestVersion",
            ChangeFeedMode::AllVersionsAndDeletes => "AllVersionsAndDeletes",
        };

        Self {
            version: "v2".to_string(),
            container_rid: container_rid.clone(),
            mode: mode_str.to_string(),
            start_from: ChangeFeedStartFromInternal::from_public(start_from),
            continuation: FeedRangeCompositeContinuation::with_sub_ranges(
                container_rid,
                feed_range,
                sub_ranges,
            ),
        }
    }

    /// Encodes the state to a base64 continuation token string.
    pub fn to_continuation_token(&self) -> azure_core::Result<String> {
        let json = serde_json::to_string(self).map_err(azure_core::Error::from)?;
        Ok(BASE64_STANDARD.encode(json.as_bytes()))
    }

    /// Decodes a continuation token string to a state object.
    pub fn from_continuation_token(token: &str) -> azure_core::Result<Self> {
        let bytes = BASE64_STANDARD
            .decode(token)
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
        let json = String::from_utf8(bytes)
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
        let state: Self = serde_json::from_str(&json).map_err(azure_core::Error::from)?;
        Ok(state)
    }

    /// Gets the current feed range being processed.
    pub fn current_feed_range(&self) -> &Range<String> {
        &self.continuation.current_token().range
    }

    /// Gets the current continuation token (etag) if any.
    pub fn current_etag(&self) -> Option<&str> {
        self.continuation.current_token().token.as_deref()
    }

    /// Applies the server response and returns the new continuation token.
    pub fn apply_server_response(&mut self, etag: String, has_results: bool) -> String {
        self.continuation
            .apply_server_response_continuation(etag, has_results);

        // Return the updated continuation token
        self.to_continuation_token()
            .unwrap_or_else(|_| String::new())
    }

    /// Moves to the next sub-range in the feed.
    pub fn move_to_next_range(&mut self) {
        self.continuation.move_to_next_token();
    }

    /// Determines if we should retry after getting no results.
    pub fn should_retry_on_not_modified(&self) -> bool {
        self.continuation.should_retry_on_not_modified()
    }

    /// Gets the start time for point-in-time queries.
    pub fn get_start_time(&self) -> Option<OffsetDateTime> {
        self.start_from.point_in_time_ms.map(|ms| {
            OffsetDateTime::from_unix_timestamp(ms / 1000).unwrap_or(OffsetDateTime::UNIX_EPOCH)
        })
    }
}

/// Request headers derived from change feed state.
pub struct ChangeFeedRequestHeaders {
    /// The If-None-Match header value (etag or "*" for "now").
    pub if_none_match: Option<String>,

    /// The If-Modified-Since header value for point-in-time queries.
    pub if_modified_since: Option<String>,
}

impl ChangeFeedRequestHeaders {
    /// Creates headers for a change feed request based on the current state.
    pub fn from_state(state: &ChangeFeedState, _pk_range_id: Option<&str>) -> Self {
        let if_none_match = match &state.start_from.start_type {
            super::start_from::ChangeFeedStartFromType::Now => {
                // If we have an etag, use it; otherwise use "*" for "now"
                state
                    .current_etag()
                    .map(|s| s.to_string())
                    .or_else(|| Some("*".to_string()))
            }
            super::start_from::ChangeFeedStartFromType::Lease => {
                state.current_etag().map(|s| s.to_string())
            }
            _ => {
                // For Beginning or PointInTime, only use etag if we have one from previous request
                state.current_etag().map(|s| s.to_string())
            }
        };

        let if_modified_since = state.get_start_time().map(|dt| {
            azure_core::time::to_rfc7231(&dt)
        });

        Self {
            if_none_match,
            if_modified_since,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::change_feed::feed_range_internal::FeedRangeInternal;

    fn create_test_state(
        start_from: &ChangeFeedStartFrom,
        mode: ChangeFeedMode,
    ) -> ChangeFeedState {
        let range = Range::new("".to_string(), "FF".to_string(), true, false);
        let feed_range = FeedRangeInternal::from_epk_range(range.clone());
        ChangeFeedState::with_sub_ranges(
            "test-rid".to_string(),
            feed_range,
            start_from,
            mode,
            vec![range],
        )
    }

    #[test]
    fn new_state_for_full_range() {
        let state = create_test_state(&ChangeFeedStartFrom::Beginning, ChangeFeedMode::LatestVersion);

        assert_eq!(state.version, "v2");
        assert_eq!(state.container_rid, "test-rid");
        assert_eq!(state.mode, "LatestVersion");
    }

    #[test]
    fn full_fidelity_mode() {
        let state = create_test_state(
            &ChangeFeedStartFrom::Beginning,
            ChangeFeedMode::AllVersionsAndDeletes,
        );

        assert_eq!(state.mode, "AllVersionsAndDeletes");
    }

    #[test]
    fn continuation_token_roundtrip() {
        let state = create_test_state(&ChangeFeedStartFrom::Beginning, ChangeFeedMode::LatestVersion);

        let token = state.to_continuation_token().unwrap();
        let restored = ChangeFeedState::from_continuation_token(&token).unwrap();

        assert_eq!(restored.version, state.version);
        assert_eq!(restored.container_rid, state.container_rid);
        assert_eq!(restored.mode, state.mode);
    }

    #[test]
    fn apply_server_response() {
        let mut state = create_test_state(&ChangeFeedStartFrom::Beginning, ChangeFeedMode::LatestVersion);

        let token = state.apply_server_response("\"etag123\"".to_string(), true);

        assert!(!token.is_empty());
        assert!(BASE64_STANDARD.decode(&token).is_ok());
        assert_eq!(state.current_etag(), Some("\"etag123\""));
    }

    #[test]
    fn request_headers_for_beginning() {
        let state = create_test_state(&ChangeFeedStartFrom::Beginning, ChangeFeedMode::LatestVersion);

        let headers = ChangeFeedRequestHeaders::from_state(&state, Some("0"));

        assert!(headers.if_none_match.is_none());
        assert!(headers.if_modified_since.is_none());
    }

    #[test]
    fn request_headers_for_now() {
        let state = create_test_state(&ChangeFeedStartFrom::Now, ChangeFeedMode::LatestVersion);

        let headers = ChangeFeedRequestHeaders::from_state(&state, None);

        assert_eq!(headers.if_none_match, Some("*".to_string()));
    }

    #[test]
    fn request_headers_for_full_fidelity() {
        let state = create_test_state(
            &ChangeFeedStartFrom::Beginning,
            ChangeFeedMode::AllVersionsAndDeletes,
        );

        let headers = ChangeFeedRequestHeaders::from_state(&state, None);

        // from_state only returns if_none_match and if_modified_since;
        // mode headers (A-IM, wire format) are set by the container_client stream.
        assert!(headers.if_none_match.is_none());
    }
}
