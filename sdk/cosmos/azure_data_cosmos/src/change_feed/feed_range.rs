// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};

use crate::routing::range::Range;

/// Represents a range of partition key values in a Cosmos DB container.
///
/// Feed ranges are used to parallelize change feed processing by dividing the container's
/// partition key space into independent ranges that can be processed concurrently.
///
/// Obtain feed ranges using [`ContainerClient::read_feed_ranges()`](crate::clients::ContainerClient::read_feed_ranges).
///
/// # Examples
///
/// ```rust,no_run
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// # let container_client: azure_data_cosmos::clients::ContainerClient = panic!("this is a non-running example");
/// // Get all feed ranges for parallel processing
/// let feed_ranges = container_client.read_feed_ranges(None).await?;
///
/// // Process each range independently (could be on different threads/processes)
/// for range in feed_ranges {
///     println!("Processing range: {}", range);
///     // Use range with query_items_change_feed()
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedRange {
    pub(crate) inner: FeedRangeInner,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum FeedRangeInner {
    Epk(EpkFeedRange),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct EpkFeedRange {
    #[serde(rename = "Range")]
    pub range: Range<String>,
}

impl FeedRange {
    /// Creates a new `FeedRange` from an effective partition key (EPK) range.
    pub(crate) fn from_epk_range(range: Range<String>) -> Self {
        Self {
            inner: FeedRangeInner::Epk(EpkFeedRange { range }),
        }
    }

    /// Gets the underlying EPK range if this is an EPK-based feed range.
    #[allow(dead_code)]
    pub(crate) fn as_epk_range(&self) -> Option<&Range<String>> {
        match &self.inner {
            FeedRangeInner::Epk(epk) => Some(&epk.range),
        }
    }

    /// Serializes the feed range to a base64-encoded JSON string.
    ///
    /// This string representation can be stored and later used to resume processing.
    pub fn to_string_representation(&self) -> azure_core::Result<String> {
        let json = serde_json::to_string(&self.inner).map_err(azure_core::Error::from)?;
        Ok(BASE64_STANDARD.encode(json.as_bytes()))
    }

    /// Deserializes a feed range from a base64-encoded JSON string.
    ///
    /// Use this to restore a feed range from a previously saved string representation.
    pub fn from_string_representation(s: &str) -> azure_core::Result<Self> {
        let bytes = BASE64_STANDARD
            .decode(s)
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
        let json = String::from_utf8(bytes)
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
        let inner: FeedRangeInner = serde_json::from_str(&json).map_err(azure_core::Error::from)?;
        Ok(Self { inner })
    }
}

impl std::fmt::Display for FeedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            FeedRangeInner::Epk(epk) => write!(f, "{}", epk.range),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feed_range_display() {
        let range = Range::new("00".to_string(), "FF".to_string(), true, false);
        let feed_range = FeedRange::from_epk_range(range);
        assert_eq!(format!("{}", feed_range), "[00,FF)");
    }

    #[test]
    fn feed_range_roundtrip() {
        let range = Range::new("00".to_string(), "FF".to_string(), true, false);
        let feed_range = FeedRange::from_epk_range(range);

        let encoded = feed_range.to_string_representation().unwrap();
        let decoded = FeedRange::from_string_representation(&encoded).unwrap();

        assert_eq!(feed_range, decoded);
    }

    #[test]
    fn feed_range_serialization() {
        let range = Range::new("".to_string(), "FF".to_string(), true, false);
        let feed_range = FeedRange::from_epk_range(range);

        let encoded = feed_range.to_string_representation().unwrap();
        // Should be valid base64
        assert!(BASE64_STANDARD.decode(&encoded).is_ok());

        // Decode and verify JSON structure
        let bytes = BASE64_STANDARD.decode(&encoded).unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert!(json.get("Range").is_some());
    }
}
