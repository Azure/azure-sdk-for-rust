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
//! let pk_ranges = container.feed_range_from_partition_key("my_partition_key", None).await?;
//! for range in &ranges {
//!     if range.contains(&pk_ranges[0]) {
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

use azure_data_cosmos_driver::models::partition_key_range::PartitionKeyRange;

use crate::hash::EffectivePartitionKey;
use crate::hash::{MAX_EXCLUSIVE_EFFECTIVE_PARTITION_KEY, MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY};
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
/// # Serialization Formats
///
/// `FeedRange` supports two distinct serialization formats:
///
/// - **[`Display`](std::fmt::Display)/[`FromStr`]** — base64-encoded JSON, intended for string storage and cross-SDK transfer.
/// - **[`Serialize`]/[`Deserialize`]** — structured JSON (`{"Range": {...}}`), intended for embedding in JSON documents.
///
/// These formats are **not interchangeable**: a value serialized with one cannot be deserialized with the other.
///
/// # Comparison Methods
///
/// Feed ranges support containment and overlap checks:
/// - [`contains()`](FeedRange::contains) — checks if another feed range is entirely within this one
/// - [`overlaps()`](FeedRange::overlaps) — checks if two feed ranges share any portion of the EPK space
#[derive(Clone, SafeDebug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct FeedRange {
    pub(crate) min_inclusive: EffectivePartitionKey,
    pub(crate) max_exclusive: EffectivePartitionKey,
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
            min_inclusive: EffectivePartitionKey::from(MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY),
            max_exclusive: EffectivePartitionKey::from(MAX_EXCLUSIVE_EFFECTIVE_PARTITION_KEY),
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
    pub(crate) fn from_range(range: &Range<String>) -> azure_core::Result<Self> {
        if !range.is_min_inclusive || range.is_max_inclusive {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                "FeedRange requires [min, max) semantics (isMinInclusive=true, isMaxInclusive=false)",
            ));
        }
        Ok(Self {
            min_inclusive: EffectivePartitionKey::from(range.min.as_str()),
            max_exclusive: EffectivePartitionKey::from(range.max.as_str()),
        })
    }

    /// Converts this `FeedRange` to an internal `Range<String>`.
    #[allow(
        dead_code,
        reason = "will be used when query/change-feed gain FeedRange support"
    )]
    pub(crate) fn to_range(&self) -> Range<String> {
        Range::new(
            self.min_inclusive.as_str().to_owned(),
            self.max_exclusive.as_str().to_owned(),
            true,
            false,
        )
    }

    /// Creates a `FeedRange` from a driver `PartitionKeyRange`.
    ///
    /// Partition key ranges from the service always use `[min, max)` semantics
    /// (min inclusive, max exclusive). Returns an error if the range is inverted.
    #[allow(
        dead_code,
        reason = "will be used when feed range methods route through the driver's routing map"
    )]
    pub(crate) fn from_partition_key_range(pkr: &PartitionKeyRange) -> azure_core::Result<Self> {
        if pkr.min_inclusive > pkr.max_exclusive {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                "partition key range min_inclusive must be <= max_exclusive",
            ));
        }
        Ok(Self {
            min_inclusive: EffectivePartitionKey::from(pkr.min_inclusive.as_str()),
            max_exclusive: EffectivePartitionKey::from(pkr.max_exclusive.as_str()),
        })
    }

    /// Creates a `FeedRange` from the SDK's internal `PartitionKeyRange`.
    ///
    /// This uses the SDK-side routing map type (with `String` EPK fields).
    pub(crate) fn from_sdk_partition_key_range(
        pkr: &crate::routing::partition_key_range::PartitionKeyRange,
    ) -> Self {
        debug_assert!(
            pkr.min_inclusive.as_str() <= pkr.max_exclusive.as_str(),
            "partition key range min_inclusive must be <= max_exclusive"
        );
        Self {
            min_inclusive: EffectivePartitionKey::from(pkr.min_inclusive.as_str()),
            max_exclusive: EffectivePartitionKey::from(pkr.max_exclusive.as_str()),
        }
    }

    /// Builds the JSON wire-format representation for serialization.
    fn to_json(&self) -> FeedRangeJson {
        FeedRangeJson {
            range: RangeJson {
                min: self.min_inclusive.as_str().to_owned(),
                max: self.max_exclusive.as_str().to_owned(),
                is_min_inclusive: true,
                is_max_inclusive: false,
            },
        }
    }

    /// Validates and constructs a `FeedRange` from deserialized JSON fields.
    ///
    /// Checks inclusivity flags and min ≤ max ordering.
    fn from_json(json: FeedRangeJson) -> azure_core::Result<Self> {
        if !json.range.is_min_inclusive || json.range.is_max_inclusive {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                "feed range must have [min, max) semantics (isMinInclusive=true, isMaxInclusive=false)",
            ));
        }

        let min = EffectivePartitionKey::from(json.range.min);
        let max = EffectivePartitionKey::from(json.range.max);

        if min > max {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                "feed range min must be less than or equal to max",
            ));
        }

        Ok(Self {
            min_inclusive: min,
            max_exclusive: max,
        })
    }
}

impl fmt::Display for FeedRange {
    /// Formats this feed range as a base64-encoded JSON string.
    ///
    /// The output is compatible with other Azure Cosmos DB SDKs and can be
    /// parsed back using [`std::str::FromStr`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json_str = serde_json::to_string(&self.to_json()).map_err(|_| fmt::Error)?;
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

        Self::from_json(json)
    }
}

impl Serialize for FeedRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_json().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FeedRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let json = FeedRangeJson::deserialize(deserializer)?;
        Self::from_json(json).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_range() {
        let full = FeedRange::full();
        assert_eq!(full.min_inclusive.as_str(), "");
        assert_eq!(full.max_exclusive.as_str(), "FF");
    }

    #[test]
    fn contains_full_contains_sub() {
        let full = FeedRange::full();
        let sub = FeedRange {
            min_inclusive: EffectivePartitionKey::from("00"),
            max_exclusive: EffectivePartitionKey::from("80"),
        };
        assert!(full.contains(&sub));
        assert!(!sub.contains(&full));
    }

    #[test]
    fn contains_self() {
        let range = FeedRange {
            min_inclusive: EffectivePartitionKey::from("20"),
            max_exclusive: EffectivePartitionKey::from("80"),
        };
        assert!(range.contains(&range));
    }

    #[test]
    fn overlaps_basic() {
        let a = FeedRange {
            min_inclusive: EffectivePartitionKey::from("00"),
            max_exclusive: EffectivePartitionKey::from("50"),
        };
        let b = FeedRange {
            min_inclusive: EffectivePartitionKey::from("30"),
            max_exclusive: EffectivePartitionKey::from("80"),
        };
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn overlaps_adjacent_no_overlap() {
        let a = FeedRange {
            min_inclusive: EffectivePartitionKey::from("00"),
            max_exclusive: EffectivePartitionKey::from("50"),
        };
        let b = FeedRange {
            min_inclusive: EffectivePartitionKey::from("50"),
            max_exclusive: EffectivePartitionKey::from("FF"),
        };
        // Adjacent ranges (a's max == b's min) do NOT overlap because max is exclusive
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn overlaps_disjoint() {
        let a = FeedRange {
            min_inclusive: EffectivePartitionKey::from("00"),
            max_exclusive: EffectivePartitionKey::from("30"),
        };
        let b = FeedRange {
            min_inclusive: EffectivePartitionKey::from("50"),
            max_exclusive: EffectivePartitionKey::from("FF"),
        };
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn display_produces_expected_base64_full_range() {
        let range = FeedRange {
            min_inclusive: EffectivePartitionKey::from(""),
            max_exclusive: EffectivePartitionKey::from("FF"),
        };
        assert_eq!(
            range.to_string(),
            "eyJSYW5nZSI6eyJtaW4iOiIiLCJtYXgiOiJGRiIsImlzTWluSW5jbHVzaXZlIjp0cnVlLCJpc01heEluY2x1c2l2ZSI6ZmFsc2V9fQ=="
        );
    }

    #[test]
    fn display_produces_expected_base64_sub_range() {
        let range = FeedRange {
            min_inclusive: EffectivePartitionKey::from("3FFFFFFFFFFF"),
            max_exclusive: EffectivePartitionKey::from("7FFFFFFFFFFF"),
        };
        assert_eq!(
            range.to_string(),
            "eyJSYW5nZSI6eyJtaW4iOiIzRkZGRkZGRkZGRkYiLCJtYXgiOiI3RkZGRkZGRkZGRkYiLCJpc01pbkluY2x1c2l2ZSI6dHJ1ZSwiaXNNYXhJbmNsdXNpdmUiOmZhbHNlfX0="
        );
    }

    #[test]
    fn from_str_parses_full_range() {
        let input = "eyJSYW5nZSI6eyJtaW4iOiIiLCJtYXgiOiJGRiIsImlzTWluSW5jbHVzaXZlIjp0cnVlLCJpc01heEluY2x1c2l2ZSI6ZmFsc2V9fQ==";
        let range: FeedRange = input.parse().unwrap();
        assert_eq!(range.min_inclusive.as_str(), "");
        assert_eq!(range.max_exclusive.as_str(), "FF");
    }

    #[test]
    fn from_str_parses_sub_range() {
        let input = "eyJSYW5nZSI6eyJtaW4iOiIzRkZGRkZGRkZGRkYiLCJtYXgiOiI3RkZGRkZGRkZGRkYiLCJpc01pbkluY2x1c2l2ZSI6dHJ1ZSwiaXNNYXhJbmNsdXNpdmUiOmZhbHNlfX0=";
        let range: FeedRange = input.parse().unwrap();
        assert_eq!(range.min_inclusive.as_str(), "3FFFFFFFFFFF");
        assert_eq!(range.max_exclusive.as_str(), "7FFFFFFFFFFF");
    }

    #[test]
    fn serde_json_serializes_to_cross_sdk_format() {
        let range = FeedRange {
            min_inclusive: EffectivePartitionKey::from(""),
            max_exclusive: EffectivePartitionKey::from("FF"),
        };
        let json = serde_json::to_string(&range).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let inner = value.get("Range").expect("expected 'Range' key");
        assert_eq!(inner.get("min").unwrap().as_str().unwrap(), "");
        assert_eq!(inner.get("max").unwrap().as_str().unwrap(), "FF");
        assert!(inner.get("isMinInclusive").unwrap().as_bool().unwrap());
        assert!(!inner.get("isMaxInclusive").unwrap().as_bool().unwrap());
    }

    #[test]
    fn serde_json_deserializes_cross_sdk_format() {
        let json =
            r#"{"Range":{"min":"","max":"FF","isMinInclusive":true,"isMaxInclusive":false}}"#;
        let range: FeedRange = serde_json::from_str(json).unwrap();
        assert_eq!(range.min_inclusive.as_str(), "");
        assert_eq!(range.max_exclusive.as_str(), "FF");
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
        let feed_range = FeedRange::from_partition_key_range(&pkr).unwrap();
        assert_eq!(feed_range.min_inclusive.as_str(), "");
        assert_eq!(feed_range.max_exclusive.as_str(), "FF");
    }

    #[test]
    fn to_range_produces_expected_fields() {
        let feed_range = FeedRange {
            min_inclusive: EffectivePartitionKey::from("20"),
            max_exclusive: EffectivePartitionKey::from("80"),
        };
        let range = feed_range.to_range();
        assert_eq!(range.min, "20");
        assert_eq!(range.max, "80");
        assert!(range.is_min_inclusive);
        assert!(!range.is_max_inclusive);
    }

    #[test]
    fn from_range_parses_expected_fields() {
        let range = Range::new("20".to_owned(), "80".to_owned(), true, false);
        let feed_range = FeedRange::from_range(&range).unwrap();
        assert_eq!(feed_range.min_inclusive.as_str(), "20");
        assert_eq!(feed_range.max_exclusive.as_str(), "80");
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

    #[test]
    fn from_str_rejects_max_inclusive() {
        let json = r#"{"Range":{"min":"","max":"FF","isMinInclusive":true,"isMaxInclusive":true}}"#;
        let encoded = base64::engine::general_purpose::STANDARD.encode(json.as_bytes());
        assert!(encoded.parse::<FeedRange>().is_err());
    }

    #[test]
    fn serde_rejects_min_not_inclusive() {
        let json =
            r#"{"Range":{"min":"","max":"FF","isMinInclusive":false,"isMaxInclusive":false}}"#;
        assert!(serde_json::from_str::<FeedRange>(json).is_err());
    }

    #[test]
    fn from_str_rejects_inverted_range() {
        let json =
            r#"{"Range":{"min":"FF","max":"","isMinInclusive":true,"isMaxInclusive":false}}"#;
        let encoded = base64::engine::general_purpose::STANDARD.encode(json.as_bytes());
        assert!(encoded.parse::<FeedRange>().is_err());
    }

    #[test]
    fn serde_rejects_inverted_range() {
        let json =
            r#"{"Range":{"min":"FF","max":"","isMinInclusive":true,"isMaxInclusive":false}}"#;
        assert!(serde_json::from_str::<FeedRange>(json).is_err());
    }

    #[test]
    fn from_range_rejects_wrong_inclusivity() {
        let range = Range::new("".to_string(), "FF".to_string(), false, true);
        assert!(FeedRange::from_range(&range).is_err());
    }
}
