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
    /// Creates a new change feed state for initial processing.
    pub fn new(
        container_rid: String,
        feed_range: FeedRangeInternal,
        start_from: &ChangeFeedStartFrom,
        mode: ChangeFeedMode,
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
            continuation: FeedRangeCompositeContinuation::new(container_rid, feed_range),
        }
    }

    /// Creates a state for the full partition range of a container.
    pub fn for_full_range(
        container_rid: String,
        start_from: &ChangeFeedStartFrom,
        mode: ChangeFeedMode,
    ) -> Self {
        // Full range: "" to "FF" (covers entire EPK space)
        let range = Range::new("".to_string(), "FF".to_string(), true, false);
        let feed_range = FeedRangeInternal::from_epk_range(range);
        Self::new(container_rid, feed_range, start_from, mode)
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

    /// Returns true if the mode is AllVersionsAndDeletes.
    pub fn is_full_fidelity(&self) -> bool {
        self.mode == "AllVersionsAndDeletes"
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

/// Builder for constructing request headers from change feed state.
pub struct ChangeFeedRequestHeaders {
    /// The A-IM header value for incremental or full fidelity feed.
    pub a_im: &'static str,

    /// The If-None-Match header value (etag or "*" for "now").
    pub if_none_match: Option<String>,

    /// The If-Modified-Since header value for point-in-time queries.
    pub if_modified_since: Option<String>,

    /// The partition key range ID to target.
    pub partition_key_range_id: Option<String>,

    /// The start EPK for sub-range queries.
    pub start_epk: Option<String>,

    /// The end EPK for sub-range queries.
    pub end_epk: Option<String>,

    /// The wire format version for full fidelity mode.
    pub wire_format_version: Option<&'static str>,
}

impl ChangeFeedRequestHeaders {
    /// Creates headers for a change feed request based on the current state.
    pub fn from_state(state: &ChangeFeedState, pk_range_id: Option<&str>) -> Self {
        let a_im = if state.is_full_fidelity() {
            "FullFidelityFeed"
        } else {
            "Incremental feed"
        };

        let if_none_match = match &state.start_from.start_type {
            super::start_from::ChangeFeedStartFromType::Now => {
                // If we have an etag, use it; otherwise use "*" for "now"
                state
                    .current_etag()
                    .map(|s| s.to_string())
                    .or_else(|| Some("*".to_string()))
            }
            super::start_from::ChangeFeedStartFromType::Lease => {
                // Use the etag from the lease/continuation
                state.current_etag().map(|s| s.to_string())
            }
            _ => {
                // For Beginning or PointInTime, only use etag if we have one from previous request
                state.current_etag().map(|s| s.to_string())
            }
        };

        let if_modified_since = state.get_start_time().map(|dt| {
            // Format as HTTP date: "Mon, 15 Jan 2024 12:00:00 GMT"
            azure_core::time::to_rfc7231(&dt)
        });

        let wire_format_version = if state.is_full_fidelity() {
            Some("2021-09-15")
        } else {
            None
        };

        Self {
            a_im,
            if_none_match,
            if_modified_since,
            partition_key_range_id: pk_range_id.map(|s| s.to_string()),
            start_epk: None,
            end_epk: None,
            wire_format_version,
        }
    }

    /// Sets the EPK range for sub-partition queries.
    pub fn with_epk_range(mut self, range: &Range<String>) -> Self {
        self.start_epk = Some(range.min.clone());
        self.end_epk = Some(range.max.clone());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state_for_full_range() {
        let state = ChangeFeedState::for_full_range(
            "test-rid".to_string(),
            &ChangeFeedStartFrom::Beginning,
            ChangeFeedMode::LatestVersion,
        );

        assert_eq!(state.version, "v2");
        assert_eq!(state.container_rid, "test-rid");
        assert_eq!(state.mode, "LatestVersion");
        assert!(!state.is_full_fidelity());
    }

    #[test]
    fn full_fidelity_mode() {
        let state = ChangeFeedState::for_full_range(
            "test-rid".to_string(),
            &ChangeFeedStartFrom::Beginning,
            ChangeFeedMode::AllVersionsAndDeletes,
        );

        assert_eq!(state.mode, "AllVersionsAndDeletes");
        assert!(state.is_full_fidelity());
    }

    #[test]
    fn continuation_token_roundtrip() {
        let state = ChangeFeedState::for_full_range(
            "test-rid".to_string(),
            &ChangeFeedStartFrom::Beginning,
            ChangeFeedMode::LatestVersion,
        );

        let token = state.to_continuation_token().unwrap();
        let restored = ChangeFeedState::from_continuation_token(&token).unwrap();

        assert_eq!(restored.version, state.version);
        assert_eq!(restored.container_rid, state.container_rid);
        assert_eq!(restored.mode, state.mode);
    }

    #[test]
    fn apply_server_response() {
        let mut state = ChangeFeedState::for_full_range(
            "test-rid".to_string(),
            &ChangeFeedStartFrom::Beginning,
            ChangeFeedMode::LatestVersion,
        );

        let token = state.apply_server_response("\"etag123\"".to_string(), true);

        // Token should be valid base64
        assert!(!token.is_empty());
        assert!(BASE64_STANDARD.decode(&token).is_ok());

        // State should have the etag
        assert_eq!(state.current_etag(), Some("\"etag123\""));
    }

    #[test]
    fn request_headers_for_beginning() {
        let state = ChangeFeedState::for_full_range(
            "test-rid".to_string(),
            &ChangeFeedStartFrom::Beginning,
            ChangeFeedMode::LatestVersion,
        );

        let headers = ChangeFeedRequestHeaders::from_state(&state, Some("0"));

        assert_eq!(headers.a_im, "Incremental feed");
        assert!(headers.if_none_match.is_none()); // No etag yet
        assert!(headers.if_modified_since.is_none());
        assert_eq!(headers.partition_key_range_id, Some("0".to_string()));
    }

    #[test]
    fn request_headers_for_now() {
        let state = ChangeFeedState::for_full_range(
            "test-rid".to_string(),
            &ChangeFeedStartFrom::Now,
            ChangeFeedMode::LatestVersion,
        );

        let headers = ChangeFeedRequestHeaders::from_state(&state, None);

        assert_eq!(headers.if_none_match, Some("*".to_string()));
    }

    #[test]
    fn request_headers_for_full_fidelity() {
        let state = ChangeFeedState::for_full_range(
            "test-rid".to_string(),
            &ChangeFeedStartFrom::Beginning,
            ChangeFeedMode::AllVersionsAndDeletes,
        );

        let headers = ChangeFeedRequestHeaders::from_state(&state, None);

        assert_eq!(headers.a_im, "FullFidelityFeed");
        assert_eq!(headers.wire_format_version, Some("2021-09-15"));
    }
}
