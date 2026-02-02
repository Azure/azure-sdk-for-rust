// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal feed range types used for change feed state management.

use serde::{Deserialize, Serialize};

use crate::routing::range::Range;

/// Internal representation of a feed range for state management.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FeedRangeInternal {
    /// Feed range based on effective partition key (EPK) range.
    Epk(FeedRangeInternalEpk),
    /// Feed range based on a specific partition key value.
    PartitionKey(FeedRangeInternalPartitionKey),
}

impl FeedRangeInternal {
    /// Gets the normalized EPK range for this feed range.
    pub fn get_normalized_range(&self) -> &Range<String> {
        match self {
            FeedRangeInternal::Epk(epk) => &epk.range,
            FeedRangeInternal::PartitionKey(pk) => &pk.range,
        }
    }

    /// Creates an EPK-based feed range.
    pub fn from_epk_range(range: Range<String>) -> Self {
        FeedRangeInternal::Epk(FeedRangeInternalEpk { range })
    }
}

/// EPK-based feed range.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedRangeInternalEpk {
    #[serde(rename = "Range")]
    pub range: Range<String>,
}

/// Partition key-based feed range.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedRangeInternalPartitionKey {
    #[serde(rename = "PK")]
    pub partition_key: serde_json::Value,

    /// The EPK range for this partition key (computed at runtime).
    #[serde(skip, default = "default_empty_range")]
    pub range: Range<String>,
}

fn default_empty_range() -> Range<String> {
    Range::new("".to_string(), "".to_string(), true, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epk_feed_range_serialization() {
        let range = Range::new("00".to_string(), "FF".to_string(), true, false);
        let feed_range = FeedRangeInternal::from_epk_range(range.clone());

        let json = serde_json::to_string(&feed_range).unwrap();
        assert!(json.contains("\"Range\""));

        let deserialized: FeedRangeInternal = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.get_normalized_range(), &range);
    }
}
