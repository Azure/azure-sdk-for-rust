// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Feed range composite continuation for managing change feed state across partition splits/merges.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::routing::{partition_key_range::PartitionKeyRange, range::Range};

use super::{
    composite_continuation_token::CompositeContinuationToken,
    feed_range_internal::FeedRangeInternal,
};

/// Manages continuation tokens across multiple sub-ranges of the change feed.
///
/// This structure handles partition splits and merges by maintaining a queue of tokens,
/// one for each sub-range. When a partition splits, the affected token is replaced with
/// tokens for the child ranges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedRangeCompositeContinuation {
    /// Version identifier for the continuation format.
    #[serde(rename = "v")]
    pub version: String,

    /// The container resource ID.
    #[serde(rename = "rid")]
    pub container_rid: String,

    /// The queue of continuation tokens, one per sub-range.
    #[serde(rename = "continuation")]
    pub continuation: Vec<CompositeContinuationToken>,

    /// The overall feed range being processed (serialized as part of the object).
    #[serde(flatten)]
    pub feed_range: FeedRangeInternal,

    /// Index of the current token being processed.
    #[serde(skip)]
    current_index: usize,

    /// The initial range with no results (for retry logic).
    #[serde(skip)]
    initial_no_result_range: Option<Range<String>>,
}

impl FeedRangeCompositeContinuation {
    /// Creates a new composite continuation for the given feed range.
    pub fn new(container_rid: String, feed_range: FeedRangeInternal) -> Self {
        let range = feed_range.get_normalized_range().clone();
        let initial_token = CompositeContinuationToken::new(range, None);

        Self {
            version: "v2".to_string(),
            container_rid,
            continuation: vec![initial_token],
            feed_range,
            current_index: 0,
            initial_no_result_range: None,
        }
    }

    /// Gets the current token being processed.
    pub fn current_token(&self) -> &CompositeContinuationToken {
        &self.continuation[self.current_index]
    }

    /// Gets a mutable reference to the current token.
    pub fn current_token_mut(&mut self) -> &mut CompositeContinuationToken {
        &mut self.continuation[self.current_index]
    }

    /// Moves to the next token in the queue (round-robin).
    pub fn move_to_next_token(&mut self) {
        if !self.continuation.is_empty() {
            self.current_index = (self.current_index + 1) % self.continuation.len();
        }
    }

    /// Applies the server response continuation token.
    pub fn apply_server_response_continuation(&mut self, etag: String, has_results: bool) {
        self.current_token_mut().update_token(etag);

        if has_results {
            self.initial_no_result_range = None;
        } else {
            self.apply_not_modified_response();
        }
    }

    /// Records that no results were returned from the current range.
    fn apply_not_modified_response(&mut self) {
        if self.initial_no_result_range.is_none() {
            self.initial_no_result_range = Some(self.current_token().range.clone());
        }
    }

    /// Determines if we should retry on a not-modified (304) response.
    ///
    /// Returns true if there are more sub-ranges to try and we haven't
    /// cycled through all of them yet.
    pub fn should_retry_on_not_modified(&self) -> bool {
        if self.continuation.len() > 1 {
            if let Some(ref initial_range) = self.initial_no_result_range {
                return &self.current_token().range != initial_range;
            }
        }
        false
    }

    /// Handles a partition split or merge by updating the continuation tokens.
    ///
    /// When a partition splits, the current token's range is replaced with tokens
    /// for each of the child ranges, preserving the continuation token for each.
    pub fn handle_feed_range_gone(&mut self, overlapping_ranges: &[PartitionKeyRange]) {
        if overlapping_ranges.len() == 1 {
            // Merge scenario: reuse existing token
            return;
        }

        // Split scenario: replace current token with tokens for child ranges
        let current_token = self.continuation.remove(self.current_index);

        // Insert new tokens for each child range
        for (i, child_range) in overlapping_ranges.iter().enumerate() {
            let range = child_range.to_range();
            let new_token = CompositeContinuationToken::new(range, current_token.token.clone());
            self.continuation.insert(self.current_index + i, new_token);
        }

        // Reset to the first new token
        // current_index already points to the first inserted token
    }

    /// Converts the continuation to a queue-based structure for processing.
    pub fn into_queue(self) -> VecDeque<CompositeContinuationToken> {
        self.continuation.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_feed_range() -> FeedRangeInternal {
        let range = Range::new("".to_string(), "FF".to_string(), true, false);
        FeedRangeInternal::from_epk_range(range)
    }

    #[test]
    fn new_composite_continuation() {
        let feed_range = create_test_feed_range();
        let continuation =
            FeedRangeCompositeContinuation::new("test-rid".to_string(), feed_range.clone());

        assert_eq!(continuation.version, "v2");
        assert_eq!(continuation.container_rid, "test-rid");
        assert_eq!(continuation.continuation.len(), 1);
        assert!(continuation.current_token().token.is_none());
    }

    #[test]
    fn apply_server_response() {
        let feed_range = create_test_feed_range();
        let mut continuation =
            FeedRangeCompositeContinuation::new("test-rid".to_string(), feed_range);

        continuation.apply_server_response_continuation("\"etag123\"".to_string(), true);

        assert_eq!(
            continuation.current_token().token,
            Some("\"etag123\"".to_string())
        );
    }

    #[test]
    fn move_to_next_token_single() {
        let feed_range = create_test_feed_range();
        let mut continuation =
            FeedRangeCompositeContinuation::new("test-rid".to_string(), feed_range);

        // With single token, should stay at index 0
        continuation.move_to_next_token();
        assert_eq!(continuation.current_index, 0);
    }

    #[test]
    fn should_retry_single_range() {
        let feed_range = create_test_feed_range();
        let mut continuation =
            FeedRangeCompositeContinuation::new("test-rid".to_string(), feed_range);

        // Apply not-modified response
        continuation.apply_server_response_continuation("\"etag\"".to_string(), false);

        // Single range should not retry
        assert!(!continuation.should_retry_on_not_modified());
    }

    #[test]
    fn handle_split() {
        let feed_range = create_test_feed_range();
        let mut continuation =
            FeedRangeCompositeContinuation::new("test-rid".to_string(), feed_range);

        // Set a token first
        continuation.apply_server_response_continuation("\"parent-etag\"".to_string(), true);

        // Simulate a split into two ranges
        let child_ranges = vec![
            PartitionKeyRange::new("1".to_string(), "".to_string(), "80".to_string()),
            PartitionKeyRange::new("2".to_string(), "80".to_string(), "FF".to_string()),
        ];

        continuation.handle_feed_range_gone(&child_ranges);

        // Should now have two tokens
        assert_eq!(continuation.continuation.len(), 2);

        // Both should inherit the parent's token
        assert_eq!(
            continuation.continuation[0].token,
            Some("\"parent-etag\"".to_string())
        );
        assert_eq!(
            continuation.continuation[1].token,
            Some("\"parent-etag\"".to_string())
        );
    }

    #[test]
    fn serialization_roundtrip() {
        let feed_range = create_test_feed_range();
        let mut continuation =
            FeedRangeCompositeContinuation::new("test-rid".to_string(), feed_range);

        continuation.apply_server_response_continuation("etag123".to_string(), true);

        let json = serde_json::to_string(&continuation).unwrap();
        assert!(json.contains("\"v\":\"v2\""));
        assert!(json.contains("\"rid\":\"test-rid\""));
        assert!(json.contains("etag123"));

        let deserialized: FeedRangeCompositeContinuation = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.version, "v2");
        assert_eq!(deserialized.container_rid, "test-rid");
    }
}
