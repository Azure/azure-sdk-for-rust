// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Feed range type for the Cosmos DB driver.
//!
//! A [`FeedRange`] represents a contiguous range of the effective partition key (EPK) space.
//! It is used by the dataflow pipeline to target operations at one or more physical partitions.
//!
//! Feed ranges can also be serialized to base64-encoded JSON for cross-SDK storage and transport.

use azure_core::fmt::SafeDebug;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::models::{effective_partition_key::EffectivePartitionKey, ItemReference, PartitionKey};
use crate::models::{partition_key_range::PartitionKeyRange, PartitionKeyDefinition};

/// A contiguous range of the effective partition key space.
///
/// Defined by `[min_inclusive, max_exclusive)` EPK boundaries. A `FeedRange` may
/// map to one or several physical partitions depending on the current partition
/// topology.
///
/// Use [`FeedRange::full()`] for the entire key space (`""..FF`).
#[derive(Clone, SafeDebug, PartialEq, Eq, Hash)]
#[safe(true)]
pub struct FeedRange(FeedRangeRepr);

#[derive(Clone, SafeDebug, PartialEq, Eq, Hash)]
#[safe(true)]
enum FeedRangeRepr {
    /// The range represents a logical partition key prefix.
    ///
    /// If the number of keys in [`FeedRangeRepr::LogicalPartition::partition_key`]
    /// is less than the number of levels in the container's partition key definition,
    /// the feed range represents all logical partitions that share that prefix
    /// (i.e. `min_inclusive < max_exclusive`).
    /// Otherwise, if the number of keys matches the number of levels, the feed
    /// range represents exactly one logical partition (i.e.
    /// `min_inclusive == max_exclusive`).
    ///
    /// This variant exists to preserve the logical partition key semantics for
    /// feed ranges that target a single logical partition or prefix, which is
    /// important for certain operations (e.g. session-token merging treats both
    /// shapes conservatively).
    LogicalPartition {
        partition_key: PartitionKey,
        min_inclusive: EffectivePartitionKey,
        max_exclusive: EffectivePartitionKey,
    },

    /// The range is defined by explicit EPK bounds.
    Range {
        min_inclusive: EffectivePartitionKey,
        max_exclusive: EffectivePartitionKey,
    },
}

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
    /// Creates a feed range from explicit EPK bounds.
    pub fn new(
        min_inclusive: EffectivePartitionKey,
        max_exclusive: EffectivePartitionKey,
    ) -> crate::error::Result<Self> {
        if min_inclusive > max_exclusive {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(
                    "feed range min_inclusive must be less than or equal to max_exclusive",
                )
                .build());
        }

        Ok(Self(FeedRangeRepr::Range {
            min_inclusive,
            max_exclusive,
        }))
    }

    /// Creates a feed range covering the entire partition key space (`""..FF`).
    pub fn full() -> Self {
        Self(FeedRangeRepr::Range {
            min_inclusive: EffectivePartitionKey::MIN.clone(),
            max_exclusive: EffectivePartitionKey::MAX.clone(),
        })
    }

    /// Creates a feed range for the logical partition key of the given item.
    pub(crate) fn for_item(item: &ItemReference) -> Self {
        Self::for_partition(
            item.partition_key().clone(),
            item.container().partition_key_definition(),
        )
    }

    /// Creates a feed range for the given logical partition key or prefix.
    ///
    /// Because the version of the partition hashing scheme must be known to compute the effective partition key,
    /// the caller must provide a reference to the partition key definition.
    ///
    /// For full keys (component count == path count) and non-MultiHash containers
    /// this yields a point range (`min == max`); for partial HPK keys (component
    /// count < path count on a MultiHash container) it yields a real prefix range
    /// (`min < max`) so the thin-client proxy can scope the per-pkrange request
    /// down to just the prefix subrange instead of returning every row in the
    /// pkrange.
    pub fn for_partition(partition_key: PartitionKey, definition: &PartitionKeyDefinition) -> Self {
        // `compute_range` returns the right shape for both full and partial keys.
        // Fall back to a point range built from `compute` if the inputs are
        // malformed (e.g. empty values) — preserves prior behavior for that
        // edge case rather than panicking.
        let (min_inclusive, max_exclusive) =
            match EffectivePartitionKey::compute_range(partition_key.values(), definition) {
                Ok(range) => (range.start, range.end),
                Err(_) => {
                    let epk = EffectivePartitionKey::compute(
                        partition_key.values(),
                        definition.kind,
                        definition.version,
                    );
                    (epk.clone(), epk)
                }
            };
        Self(FeedRangeRepr::LogicalPartition {
            partition_key,
            min_inclusive,
            max_exclusive,
        })
    }

    /// Returns the logical partition key if this feed range represents a single logical partition or prefix.
    ///
    /// It is the caller's responsibility to determine whether the returned partition key represents a full logical partition (i.e. has values for all levels of the hierarchy)
    /// or a prefix that covers multiple logical partitions (i.e. has values for only a subset of the levels).
    pub(crate) fn partition_key(&self) -> Option<&PartitionKey> {
        match &self.0 {
            FeedRangeRepr::LogicalPartition { partition_key, .. } => Some(partition_key),
            FeedRangeRepr::Range { .. } => None,
        }
    }

    /// Returns `true` if this feed range represents a single logical partition (or
    /// a hierarchical-partition-key prefix), as opposed to an explicit `[min, max)`
    /// EPK range.
    ///
    /// Logical-partition feed ranges have implicit single-partition targeting semantics
    /// that are lost when combined with arbitrary EPK ranges, so callers that wish to
    /// merge feed ranges (for example, session-token coalescing) typically exclude this
    /// variant before doing so.
    pub fn is_logical_partition(&self) -> bool {
        matches!(self.0, FeedRangeRepr::LogicalPartition { .. })
    }

    /// Returns the inclusive lower bound of this range.
    pub fn min_inclusive(&self) -> &EffectivePartitionKey {
        match &self.0 {
            FeedRangeRepr::LogicalPartition { min_inclusive, .. } => min_inclusive,
            FeedRangeRepr::Range { min_inclusive, .. } => min_inclusive,
        }
    }

    /// Returns the exclusive upper bound of this range.
    ///
    /// NOTE: The [`min_inclusive`](FeedRange::min_inclusive) value overrides this limit. Thus, a range with
    /// `min_inclusive == max_exclusive` is valid and represents exactly one EPK value, not an empty range.
    pub fn max_exclusive(&self) -> &EffectivePartitionKey {
        match &self.0 {
            FeedRangeRepr::LogicalPartition { max_exclusive, .. } => max_exclusive,
            FeedRangeRepr::Range { max_exclusive, .. } => max_exclusive,
        }
    }

    /// Returns `true` if this feed range is entirely contained within `other`.
    pub fn is_subset_of(&self, other: &FeedRange) -> bool {
        other.min_inclusive() <= self.min_inclusive()
            && other.max_exclusive() >= self.max_exclusive()
    }

    /// Returns `true` if this feed range and `other` share any portion of the EPK space.
    ///
    /// Two feed ranges overlap when one starts before the other ends and vice versa.
    pub fn overlaps(&self, other: &FeedRange) -> bool {
        self.min_inclusive() < other.max_exclusive() && other.min_inclusive() < self.max_exclusive()
    }

    fn to_json(&self) -> FeedRangeJson {
        FeedRangeJson {
            range: RangeJson {
                min: self.min_inclusive().to_hex().to_owned(),
                max: self.max_exclusive().to_hex().to_owned(),
                is_min_inclusive: true,
                is_max_inclusive: false,
            },
        }
    }

    fn from_json(json: FeedRangeJson) -> crate::error::Result<Self> {
        if !json.range.is_min_inclusive || json.range.is_max_inclusive {
            return Err(crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::new(azure_core::http::StatusCode::BadRequest)).with_message("feed range must have [min, max) semantics (isMinInclusive=true, isMaxInclusive=false)").build());
        }

        let min = EffectivePartitionKey::from(json.range.min);
        let max = EffectivePartitionKey::from(json.range.max);

        if min > max {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("feed range min must be less than or equal to max")
                .build());
        }

        Ok(Self(FeedRangeRepr::Range {
            min_inclusive: min,
            max_exclusive: max,
        }))
    }
}

impl TryFrom<&PartitionKeyRange> for FeedRange {
    type Error = crate::error::CosmosError;

    /// Creates a `FeedRange` from a driver `PartitionKeyRange`.
    ///
    /// Partition key ranges from the service always use `[min, max)` semantics
    /// (min inclusive, max exclusive). Returns an error if the range is inverted.
    fn try_from(pkr: &PartitionKeyRange) -> Result<Self, Self::Error> {
        if pkr.min_inclusive > pkr.max_exclusive {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("partition key range min_inclusive must be <= max_exclusive")
                .build());
        }

        Ok(Self(FeedRangeRepr::Range {
            min_inclusive: pkr.min_inclusive.clone(),
            max_exclusive: pkr.max_exclusive.clone(),
        }))
    }
}

impl fmt::Display for FeedRange {
    /// Formats this feed range as a base64-encoded JSON string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json_str = serde_json::to_string(&self.to_json()).map_err(|_| fmt::Error)?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(json_str.as_bytes());
        f.write_str(&encoded)
    }
}

impl FromStr for FeedRange {
    type Err = crate::error::CosmosError;

    /// Parses a feed range from a base64-encoded JSON string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded_bytes = base64::engine::general_purpose::STANDARD
            .decode(s)
            .map_err(|e| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(
                        azure_core::http::StatusCode::BadRequest,
                    ))
                    .with_message("feed range is not valid base64")
                    .with_source(e)
                    .build()
            })?;

        let json: FeedRangeJson = serde_json::from_slice(&decoded_bytes).map_err(|e| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("feed range JSON is invalid")
                .with_source(e)
                .build()
        })?;

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
        assert_eq!(full.min_inclusive().to_hex(), "");
        assert_eq!(full.max_exclusive().to_hex(), "FF");
    }

    #[test]
    fn is_subset_of_full() {
        let full = FeedRange::full();
        let sub = FeedRange::new(
            EffectivePartitionKey::from("00"),
            EffectivePartitionKey::from("80"),
        )
        .unwrap();
        assert!(sub.is_subset_of(&full));
        assert!(!full.is_subset_of(&sub));
    }

    #[test]
    fn is_subset_of_self() {
        let range = FeedRange::new(
            EffectivePartitionKey::from("20"),
            EffectivePartitionKey::from("80"),
        )
        .unwrap();
        assert!(range.is_subset_of(&range));
    }

    #[test]
    fn overlaps_basic() {
        let a = FeedRange::new(
            EffectivePartitionKey::from("00"),
            EffectivePartitionKey::from("50"),
        )
        .unwrap();
        let b = FeedRange::new(
            EffectivePartitionKey::from("30"),
            EffectivePartitionKey::from("80"),
        )
        .unwrap();
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn overlaps_adjacent_no_overlap() {
        let a = FeedRange::new(
            EffectivePartitionKey::from("00"),
            EffectivePartitionKey::from("50"),
        )
        .unwrap();
        let b = FeedRange::new(
            EffectivePartitionKey::from("50"),
            EffectivePartitionKey::from("FF"),
        )
        .unwrap();
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn overlaps_disjoint() {
        let a = FeedRange::new(
            EffectivePartitionKey::from("00"),
            EffectivePartitionKey::from("30"),
        )
        .unwrap();
        let b = FeedRange::new(
            EffectivePartitionKey::from("50"),
            EffectivePartitionKey::from("FF"),
        )
        .unwrap();
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn display_round_trip() {
        let range = FeedRange::new(
            EffectivePartitionKey::from("3FFFFFFFFFFF"),
            EffectivePartitionKey::from("7FFFFFFFFFFF"),
        )
        .unwrap();

        let serialized = range.to_string();
        let parsed: FeedRange = serialized.parse().unwrap();

        assert_eq!(parsed, range);
    }

    #[test]
    fn serde_json_round_trip() {
        let range = FeedRange::new(
            EffectivePartitionKey::from(""),
            EffectivePartitionKey::from("FF"),
        )
        .unwrap();

        let json = serde_json::to_string(&range).unwrap();
        let parsed: FeedRange = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed, range);
    }

    #[test]
    fn try_from_partition_key_range() {
        let pkr = PartitionKeyRange::new("0".to_string(), "".to_string(), "FF".to_string());
        let feed_range = FeedRange::try_from(&pkr).unwrap();

        assert_eq!(feed_range.min_inclusive().to_hex(), "");
        assert_eq!(feed_range.max_exclusive().to_hex(), "FF");
    }

    #[test]
    fn from_str_invalid_base64() {
        assert!("not-valid-base64!!!".parse::<FeedRange>().is_err());
    }

    #[test]
    fn from_str_invalid_json() {
        let encoded = base64::engine::general_purpose::STANDARD.encode(b"not json");
        assert!(encoded.parse::<FeedRange>().is_err());
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
}
