// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types for working with feed ranges in Azure Cosmos DB.
//!
//! A [`FeedRange`] represents a contiguous range of partitions in a Cosmos DB container,
//! defined by effective partition key (EPK) boundaries. Feed ranges enable:
//!
//! - Parallel query processing by distributing ranges across workers
//! - Scoped change feed consumption for specific partitions
//! - Workload distribution across multiple consumers
//!
//! # Examples
//!
//! ```rust,no_run
//! # use azure_data_cosmos::clients::ContainerClient;
//! # async fn example(container: ContainerClient) -> azure_core::Result<()> {
//! // Get physical partition feed ranges
//! let ranges = container.read_feed_ranges(None).await?;
//! println!("Container has {} physical partitions", ranges.len());
//!
//! // Check if one range contains another
//! let pk_range = container.feed_range_from_partition_key("my_partition_key").await?;
//! for range in &ranges {
//!     if range.contains(&pk_range) {
//!         println!("Partition key falls within this feed range");
//!     }
//! }
//!
//! // Serialize/deserialize for storage or transfer
//! let serialized = ranges[0].to_string();
//! let restored: azure_data_cosmos::FeedRange = serialized.parse()?;
//! assert_eq!(ranges[0], restored);
//! # Ok(())
//! # }
//! ```

use azure_core::fmt::SafeDebug;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::routing::partition_key_range::PartitionKeyRange;
use crate::routing::range::Range;

/// An opaque representation of a contiguous range of partitions in a Cosmos DB container.
///
/// Feed ranges are defined by effective partition key (EPK) boundaries and map to one or more
/// physical partitions. They are obtained from [`ContainerClient::read_feed_ranges()`](crate::clients::ContainerClient::read_feed_ranges)
/// or [`ContainerClient::feed_range_from_partition_key()`](crate::clients::ContainerClient::feed_range_from_partition_key).
///
/// Feed ranges can be serialized to strings (via [`std::fmt::Display`]/[`std::str::FromStr`]) for storage or transfer
/// between processes. The serialization format is base64-encoded JSON, compatible with other
/// Azure Cosmos DB SDKs.
///
/// # Comparison Methods
///
/// Feed ranges support containment and overlap checks:
/// - [`contains()`](FeedRange::contains) — checks if another feed range is entirely within this one
/// - [`overlaps()`](FeedRange::overlaps) — checks if two feed ranges share any portion of the EPK space
#[derive(Clone, SafeDebug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct FeedRange {
    pub(crate) min_inclusive: String,
    pub(crate) max_exclusive: String,
}

/// JSON wire format matching the cross-SDK feed range representation.
///
/// Example:
/// ```json
/// {"Range": {"min": "", "max": "FF", "isMinInclusive": true, "isMaxInclusive": false}}
/// ```
#[derive(Serialize, Deserialize)]
struct FeedRangeJson {
    #[serde(rename = "Range")]
    range: RangeJson,
}

#[derive(Serialize, Deserialize)]
struct RangeJson {
    min: String,
    max: String,
    #[serde(rename = "isMinInclusive")]
    is_min_inclusive: bool,
    #[serde(rename = "isMaxInclusive")]
    is_max_inclusive: bool,
}

impl FeedRange {
    /// Creates a feed range covering the entire partition key space.
    ///
    /// This range spans from the minimum to maximum effective partition key values,
    /// encompassing all partitions in a container.
    pub fn full() -> Self {
        Self {
            min_inclusive: String::new(),
            max_exclusive: "FF".to_string(),
        }
    }

    /// Returns `true` if `other` is entirely contained within this feed range.
    ///
    /// A feed range A contains feed range B when A's minimum is less than or equal to B's minimum
    /// and A's maximum is greater than or equal to B's maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// # use azure_data_cosmos::FeedRange;
    /// let full = FeedRange::full();
    /// let sub: FeedRange = "eyJSYW5nZSI6eyJtaW4iOiIiLCJtYXgiOiIzRkZGRkZGRkZGRkYiLCJpc01pbkluY2x1c2l2ZSI6dHJ1ZSwiaXNNYXhJbmNsdXNpdmUiOmZhbHNlfX0=".parse().unwrap();
    /// assert!(full.contains(&sub));
    /// ```
    pub fn contains(&self, other: &FeedRange) -> bool {
        self.min_inclusive <= other.min_inclusive && self.max_exclusive >= other.max_exclusive
    }

    /// Returns `true` if this feed range and `other` share any portion of the EPK space.
    ///
    /// Two feed ranges overlap when one starts before the other ends and vice versa.
    pub fn overlaps(&self, other: &FeedRange) -> bool {
        self.min_inclusive < other.max_exclusive && other.min_inclusive < self.max_exclusive
    }

    /// Creates a `FeedRange` from an internal `Range<String>`.
    ///
    /// The source range must have `[min, max)` semantics (min inclusive, max exclusive),
    /// which is the invariant for all partition key ranges from the service.
    #[allow(
        dead_code,
        reason = "will be used when query/change-feed gain FeedRange support"
    )]
    pub(crate) fn from_range(range: &Range<String>) -> Self {
        debug_assert!(
            range.is_min_inclusive && !range.is_max_inclusive,
            "FeedRange requires [min, max) semantics but got is_min_inclusive={}, is_max_inclusive={}",
            range.is_min_inclusive,
            range.is_max_inclusive
        );
        Self {
            min_inclusive: range.min.clone(),
            max_exclusive: range.max.clone(),
        }
    }

    /// Converts this `FeedRange` to an internal `Range<String>`.
    #[allow(
        dead_code,
        reason = "will be used when query/change-feed gain FeedRange support"
    )]
    pub(crate) fn to_range(&self) -> Range<String> {
        Range::new(
            self.min_inclusive.clone(),
            self.max_exclusive.clone(),
            true,
            false,
        )
    }

    /// Creates a `FeedRange` from a `PartitionKeyRange`.
    pub(crate) fn from_partition_key_range(pkr: &PartitionKeyRange) -> Self {
        Self {
            min_inclusive: pkr.min_inclusive.clone(),
            max_exclusive: pkr.max_exclusive.clone(),
        }
    }
}

impl fmt::Display for FeedRange {
    /// Formats this feed range as a base64-encoded JSON string.
    ///
    /// The output is compatible with other Azure Cosmos DB SDKs and can be
    /// parsed back using [`std::str::FromStr`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = FeedRangeJson {
            range: RangeJson {
                min: self.min_inclusive.clone(),
                max: self.max_exclusive.clone(),
                is_min_inclusive: true,
                is_max_inclusive: false,
            },
        };
        let json_str = serde_json::to_string(&json).map_err(|_| fmt::Error)?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(json_str.as_bytes());
        f.write_str(&encoded)
    }
}

impl FromStr for FeedRange {
    type Err = azure_core::Error;

    /// Parses a feed range from a base64-encoded JSON string.
    ///
    /// The input should be a string produced by [`std::fmt::Display`] or by another Azure Cosmos DB SDK.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded_bytes = base64::engine::general_purpose::STANDARD
            .decode(s)
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;

        let json: FeedRangeJson = serde_json::from_slice(&decoded_bytes)
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;

        // Cosmos DB always uses [min, max) semantics. Reject ranges with unexpected inclusivity
        // to prevent subtly incorrect containment/overlap checks.
        if !json.range.is_min_inclusive || json.range.is_max_inclusive {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                "feed range must have [min, max) semantics (isMinInclusive=true, isMaxInclusive=false)",
            ));
        }

        Ok(Self {
            min_inclusive: json.range.min,
            max_exclusive: json.range.max,
        })
    }
}

impl Serialize for FeedRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let json = FeedRangeJson {
            range: RangeJson {
                min: self.min_inclusive.clone(),
                max: self.max_exclusive.clone(),
                is_min_inclusive: true,
                is_max_inclusive: false,
            },
        };
        json.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FeedRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let json = FeedRangeJson::deserialize(deserializer)?;

        if !json.range.is_min_inclusive || json.range.is_max_inclusive {
            return Err(serde::de::Error::custom(
                "feed range must have [min, max) semantics (isMinInclusive=true, isMaxInclusive=false)",
            ));
        }

        Ok(Self {
            min_inclusive: json.range.min,
            max_exclusive: json.range.max,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_range() {
        let full = FeedRange::full();
        assert_eq!(full.min_inclusive, "");
        assert_eq!(full.max_exclusive, "FF");
    }

    #[test]
    fn contains_full_contains_sub() {
        let full = FeedRange::full();
        let sub = FeedRange {
            min_inclusive: "00".to_string(),
            max_exclusive: "80".to_string(),
        };
        assert!(full.contains(&sub));
        assert!(!sub.contains(&full));
    }

    #[test]
    fn contains_self() {
        let range = FeedRange {
            min_inclusive: "20".to_string(),
            max_exclusive: "80".to_string(),
        };
        assert!(range.contains(&range));
    }

    #[test]
    fn overlaps_basic() {
        let a = FeedRange {
            min_inclusive: "00".to_string(),
            max_exclusive: "50".to_string(),
        };
        let b = FeedRange {
            min_inclusive: "30".to_string(),
            max_exclusive: "80".to_string(),
        };
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn overlaps_adjacent_no_overlap() {
        let a = FeedRange {
            min_inclusive: "00".to_string(),
            max_exclusive: "50".to_string(),
        };
        let b = FeedRange {
            min_inclusive: "50".to_string(),
            max_exclusive: "FF".to_string(),
        };
        // Adjacent ranges (a's max == b's min) do NOT overlap because max is exclusive
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn overlaps_disjoint() {
        let a = FeedRange {
            min_inclusive: "00".to_string(),
            max_exclusive: "30".to_string(),
        };
        let b = FeedRange {
            min_inclusive: "50".to_string(),
            max_exclusive: "FF".to_string(),
        };
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn display_and_from_str_round_trip() {
        let range = FeedRange {
            min_inclusive: "".to_string(),
            max_exclusive: "FF".to_string(),
        };
        let serialized = range.to_string();
        let restored: FeedRange = serialized.parse().unwrap();
        assert_eq!(range, restored);
    }

    #[test]
    fn display_and_from_str_sub_range() {
        let range = FeedRange {
            min_inclusive: "3FFFFFFFFFFF".to_string(),
            max_exclusive: "7FFFFFFFFFFF".to_string(),
        };
        let serialized = range.to_string();
        let restored: FeedRange = serialized.parse().unwrap();
        assert_eq!(range, restored);
    }

    #[test]
    fn serde_json_round_trip() {
        let range = FeedRange {
            min_inclusive: "".to_string(),
            max_exclusive: "FF".to_string(),
        };
        let json = serde_json::to_string(&range).unwrap();
        let restored: FeedRange = serde_json::from_str(&json).unwrap();
        assert_eq!(range, restored);

        // Verify the JSON structure matches the cross-SDK format
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(value.get("Range").is_some());
        let inner = value.get("Range").unwrap();
        assert_eq!(inner.get("min").unwrap().as_str().unwrap(), "");
        assert_eq!(inner.get("max").unwrap().as_str().unwrap(), "FF");
        assert!(inner.get("isMinInclusive").unwrap().as_bool().unwrap());
        assert!(!inner.get("isMaxInclusive").unwrap().as_bool().unwrap());
    }

    #[test]
    fn from_str_invalid_base64() {
        let result = "not-valid-base64!!!".parse::<FeedRange>();
        assert!(result.is_err());
    }

    #[test]
    fn from_str_invalid_json() {
        let encoded = base64::engine::general_purpose::STANDARD.encode(b"not json");
        let result = encoded.parse::<FeedRange>();
        assert!(result.is_err());
    }

    #[test]
    fn from_partition_key_range() {
        let pkr = PartitionKeyRange::new("0".to_string(), "".to_string(), "FF".to_string());
        let feed_range = FeedRange::from_partition_key_range(&pkr);
        assert_eq!(feed_range.min_inclusive, "");
        assert_eq!(feed_range.max_exclusive, "FF");
    }

    #[test]
    fn to_range_and_back() {
        let feed_range = FeedRange {
            min_inclusive: "20".to_string(),
            max_exclusive: "80".to_string(),
        };
        let range = feed_range.to_range();
        assert_eq!(range.min, "20");
        assert_eq!(range.max, "80");
        assert!(range.is_min_inclusive);
        assert!(!range.is_max_inclusive);

        let restored = FeedRange::from_range(&range);
        assert_eq!(feed_range, restored);
    }

    #[test]
    fn cross_sdk_compatibility() {
        // Verify that the full range serializes to the same base64 string regardless of platform
        let full = FeedRange::full();
        let serialized = full.to_string();

        // Decode and verify the JSON structure
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&serialized)
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&decoded).unwrap();

        let range = json.get("Range").unwrap();
        assert_eq!(range.get("min").unwrap().as_str().unwrap(), "");
        assert_eq!(range.get("max").unwrap().as_str().unwrap(), "FF");
        assert!(range.get("isMinInclusive").unwrap().as_bool().unwrap());
        assert!(!range.get("isMaxInclusive").unwrap().as_bool().unwrap());
    }
}
