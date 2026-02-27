// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Composite continuation token for change feed.

use serde::{Deserialize, Serialize};

use crate::routing::range::Range;

/// A continuation token paired with the feed range it applies to.
///
/// Each token tracks the progress of reading a specific sub-range of the change feed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeContinuationToken {
    /// The continuation token (etag) from the server.
    #[serde(rename = "token")]
    pub token: Option<String>,

    /// The feed range this token applies to.
    #[serde(rename = "range")]
    pub range: Range<String>,
}

impl CompositeContinuationToken {
    /// Creates a new composite continuation token.
    pub fn new(range: Range<String>, token: Option<String>) -> Self {
        Self { token, range }
    }

    /// Updates the token with a new etag from the server.
    pub fn update_token(&mut self, etag: String) {
        self.token = Some(etag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composite_token_creation() {
        let range = Range::new("00".to_string(), "FF".to_string(), true, false);
        let token = CompositeContinuationToken::new(range.clone(), None);

        assert!(token.token.is_none());
        assert_eq!(token.range, range);
    }

    #[test]
    fn composite_token_update() {
        let range = Range::new("00".to_string(), "FF".to_string(), true, false);
        let mut token = CompositeContinuationToken::new(range, None);

        token.update_token("\"etag123\"".to_string());
        assert_eq!(token.token, Some("\"etag123\"".to_string()));
    }

    #[test]
    fn composite_token_serialization() {
        let range = Range::new("00".to_string(), "FF".to_string(), true, false);
        let token = CompositeContinuationToken::new(range, Some("etag123".to_string()));

        let json = serde_json::to_string(&token).unwrap();
        assert!(json.contains("\"token\""));
        assert!(json.contains("\"range\""));
        assert!(json.contains("etag123"));

        let deserialized: CompositeContinuationToken = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.token, token.token);
        assert_eq!(deserialized.range, token.range);
    }
}
