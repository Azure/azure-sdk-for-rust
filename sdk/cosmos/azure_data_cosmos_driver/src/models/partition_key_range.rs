// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::effective_partition_key::EffectivePartitionKey;
use crate::models::range::EpkRange;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

/// Represents a partition key range in the Azure Cosmos DB service.
///
/// This is the routing-cache view of a partition key range, not a full
/// mirror of the `/pkranges` REST resource. The service returns several
/// metadata fields (`_rid`, `_self`, `_etag`, `_ts`, `ridPrefix`,
/// `targetThroughput`, `_lsn`, `ownedArchivalPKRangeIds`) that the routing
/// layer never consults. They are intentionally absent from this struct;
/// serde silently ignores the unknown JSON fields on deserialization, so
/// service responses keep parsing without modification while each cached
/// entry stays small — important when many containers or `CosmosClient`
/// instances are alive at once.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionKeyRange {
    /// Gets or sets the Id of the resource
    #[serde(rename = "id")]
    pub id: String,

    /// Represents the minimum possible value of a PartitionKeyRange (inclusive)
    #[serde(rename = "minInclusive")]
    pub min_inclusive: EffectivePartitionKey,

    /// Represents maximum exclusive value of a PartitionKeyRange
    #[serde(rename = "maxExclusive")]
    pub max_exclusive: EffectivePartitionKey,

    /// Status of the partition key range.
    ///
    /// Not part of the public API surface; uses a crate-internal enum type.
    /// Retained on the cached struct (1 byte) because the routing map uses
    /// it to compute `highest_non_offline_pk_range_id` for split detection.
    #[serde(rename = "status", default)]
    pub(crate) status: PartitionKeyRangeStatus,

    /// Fraction of the container's provisioned throughput allocated to this
    /// partition key range. Not consulted by the routing layer itself, but
    /// kept on the cached struct so consumers that read it directly continue
    /// to work after the slim-down.
    #[serde(rename = "throughputFraction", default)]
    pub throughput_fraction: f64,

    /// Contains ids of parent ranges.
    /// For example if range with id '1' splits into '2' and '3',
    /// then Parents for ranges '2' and '3' will be ['1'].
    /// If range '3' splits into '4' and '5', then parents for ranges '4' and '5'
    /// will be ['1', '3'].
    #[serde(rename = "parents", skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,
}

/// Status of a partition key range
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub(crate) enum PartitionKeyRangeStatus {
    #[default]
    Online,
    Splitting,
    Offline,
    Split,
}

impl PartitionKeyRange {
    /// Creates a new PartitionKeyRange with required fields
    pub fn new(
        id: String,
        min_inclusive: impl Into<EffectivePartitionKey>,
        max_exclusive: impl Into<EffectivePartitionKey>,
    ) -> Self {
        Self {
            id,
            min_inclusive: min_inclusive.into(),
            max_exclusive: max_exclusive.into(),
            status: PartitionKeyRangeStatus::default(),
            throughput_fraction: 0.0,
            parents: None,
        }
    }

    /// Returns a view of this partition key range as an `EpkRange<&EffectivePartitionKey>`.
    pub(crate) fn as_range(&self) -> EpkRange<&EffectivePartitionKey> {
        EpkRange {
            min: &self.min_inclusive,
            max: &self.max_exclusive,
            is_min_inclusive: true,
            is_max_inclusive: false,
        }
    }

    /// Gets the parent IDs as a HashSet, or empty set if none
    pub fn get_parent_ids(&self) -> HashSet<String> {
        self.parents
            .as_ref()
            .map(|parents| parents.iter().cloned().collect())
            .unwrap_or_default()
    }
}

// Implement PartialEq for PartitionKeyRange.
//
// Equality compares only the routing-relevant identity fields. The
// service-side `_rid` (resource_id) is no longer stored on the cached
// struct, so it cannot participate. Two ranges with the same `id` and
// EPK extents are treated as equal — which matches how the routing map
// addresses them (`range_by_id` keys by `id` only).
impl PartialEq for PartitionKeyRange {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.min_inclusive == other.min_inclusive
            && self.max_exclusive == other.max_exclusive
    }
}

impl Eq for PartitionKeyRange {}

/// Orders partition key ranges by `min_inclusive`.
///
/// This ordering determines the position of a range in the sorted routing map,
/// enabling binary search over partition key ranges.
impl PartialOrd for PartitionKeyRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PartitionKeyRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min_inclusive.cmp(&other.min_inclusive)
    }
}

// Implement a manual Hash for PartitionKeyRange consistent with PartialEq:
// only the identity fields contribute.
impl Hash for PartitionKeyRange {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.min_inclusive.hash(state);
        self.max_exclusive.hash(state);
    }
}

/// Response from the `/pkranges` REST endpoint.
#[derive(Debug, Deserialize)]
pub(crate) struct PkRangesResponse {
    /// The partition key ranges returned by the service.
    #[serde(rename = "PartitionKeyRanges")]
    pub partition_key_ranges: Vec<PartitionKeyRange>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_key_range_creation() {
        let pkr = PartitionKeyRange::new("1".to_string(), "", "FF");

        assert_eq!(pkr.id, "1");
        assert_eq!(pkr.min_inclusive.to_hex(), "");
        assert_eq!(pkr.max_exclusive.to_hex(), "FF");
    }

    #[test]
    fn as_range() {
        let pkr = PartitionKeyRange::new("1".to_string(), "00", "FF");

        let range = pkr.as_range();
        assert_eq!(range.min.to_hex(), "00");
        assert_eq!(range.max.to_hex(), "FF");
        assert!(range.is_min_inclusive);
        assert!(!range.is_max_inclusive);
    }

    #[test]
    fn equality_check() {
        let pkr1 = PartitionKeyRange::new("1".to_string(), "00", "FF");

        let mut pkr2 = PartitionKeyRange::new("1".to_string(), "00", "FF");

        assert_eq!(pkr1, pkr2);

        pkr2.id = "2".to_string();
        assert_ne!(pkr1, pkr2);
    }

    /// Service responses commonly include metadata fields (`_rid`, `_self`,
    /// `_etag`, `_ts`, `ridPrefix`, `targetThroughput`, `_lsn`,
    /// `ownedArchivalPKRangeIds`) that this struct deliberately does not
    /// retain. Serde must silently ignore them so existing service payloads
    /// keep parsing.
    #[test]
    fn deserialization_ignores_stripped_metadata_fields() {
        let json = r#"{
            "id": "1",
            "_rid": "rid123",
            "_self": "self/1",
            "_etag": "\"etag\"",
            "_ts": 1234567890,
            "minInclusive": "",
            "maxExclusive": "FF",
            "ridPrefix": 42,
            "throughputFraction": 0.5,
            "targetThroughput": 1000.0,
            "status": "online",
            "_lsn": 100,
            "parents": ["0"],
            "ownedArchivalPKRangeIds": ["arch-0"]
        }"#;

        let pkr: PartitionKeyRange = serde_json::from_str(json).unwrap();

        assert_eq!(pkr.id, "1");
        assert_eq!(pkr.min_inclusive.to_hex(), "");
        assert_eq!(pkr.max_exclusive.to_hex(), "FF");
        assert_eq!(pkr.status, PartitionKeyRangeStatus::Online);
        assert_eq!(pkr.throughput_fraction, 0.5);
        assert_eq!(pkr.parents.as_deref(), Some(&["0".to_string()][..]));
    }

    /// The whole point of slimming the cached struct is to keep its
    /// in-memory footprint small. This guard catches accidental re-bloat
    /// (a new field, a `#[serde(flatten)]` extension, etc.) before it
    /// regresses memory at scale.
    ///
    /// The exact size depends on the `EffectivePartitionKey` and `String`
    /// representations on the target platform; the bound is set just above
    /// the current 64-bit value (112 B) so that adding back a 24-byte
    /// `String` field would trip this check.
    ///
    /// We assert two things:
    /// 1. A hard upper bound to catch re-bloat (re-adding a stripped field).
    /// 2. An exact-size pin so a silent layout regression (e.g., a future
    ///    Rust release tweaks `String`/`Vec` representation, niche
    ///    optimization for `Option<Vec<T>>`, or alignment) is also caught
    ///    rather than absorbed by the slack.
    #[test]
    fn cached_size_stays_small() {
        // 6 fields:
        //   id:                   String                    (24 bytes on 64-bit)
        //   min:                  EffectivePartitionKey     (24 bytes — wraps String)
        //   max:                  EffectivePartitionKey     (24 bytes)
        //   status:               PartitionKeyRangeStatus   (1 byte, packed)
        //   throughput_fraction:  f64                       (8 bytes, 8-aligned)
        //   parents:              Option<Vec<String>>       (24 bytes — Vec is 24, niche fills the Option)
        //
        // Plus alignment padding ⇒ 112 bytes today. The cap is intentionally
        // tight (one stripped-then-re-added `String` would push it past 120)
        // so re-bloat is caught at PR time rather than at runtime.
        const MAX_SIZE: usize = 120;
        let actual = std::mem::size_of::<PartitionKeyRange>();
        assert!(
            actual <= MAX_SIZE,
            "PartitionKeyRange grew to {actual} bytes (cap {MAX_SIZE}). \
             Cached pkrange entries should stay small — re-evaluate the field set."
        );

        // Layout-regression pin: the exact size on 64-bit. If a future Rust
        // release tweaks `String`/`Vec` representation, niche optimization
        // for `Option<Vec<T>>`, or alignment, this fires before the slack
        // absorbs it. 32-bit targets have a different (smaller) expected
        // size, so the pin is gated to 64-bit; the cap above still catches
        // re-bloat on any arch.
        #[cfg(target_pointer_width = "64")]
        {
            const EXPECTED_SIZE: usize = 112;
            assert_eq!(
                actual, EXPECTED_SIZE,
                "PartitionKeyRange size shifted from {EXPECTED_SIZE} to {actual} bytes \
                 without a field change. A toolchain or stdlib layout assumption may \
                 have moved (`String`/`Vec` representation, `Option<Vec<T>>` niche, \
                 alignment). Re-confirm the per-cache-entry footprint expectation, \
                 then bump `EXPECTED_SIZE` if the new layout is acceptable."
            );
        }
    }

    #[test]
    fn range_overlap() {
        let range1: EpkRange<String> =
            EpkRange::new("00".to_string(), "50".to_string(), true, false);
        let range2 = EpkRange::new("40".to_string(), "80".to_string(), true, false);
        let range3 = EpkRange::new("60".to_string(), "90".to_string(), true, false);
        assert!(EpkRange::check_overlapping(&range1, &range2));
        assert!(!EpkRange::check_overlapping(&range1, &range3));
    }
}
