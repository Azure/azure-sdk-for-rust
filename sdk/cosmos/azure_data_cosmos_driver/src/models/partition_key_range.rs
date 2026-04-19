// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::effective_partition_key::EffectivePartitionKey;
use crate::models::range::EpkRange;
use crate::models::ETag;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

/// Represents a partition key range in the Azure Cosmos DB service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionKeyRange {
    /// Gets or sets the Id of the resource
    #[serde(rename = "id")]
    pub id: String,

    /// Gets or sets the Resource Id associated with the resource
    #[serde(rename = "_rid", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,

    /// Gets the self-link associated with the resource
    #[serde(rename = "_self", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// Gets the entity tag associated with the resource
    #[serde(rename = "_etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<ETag>,

    /// Gets the last modified timestamp associated with the resource
    #[serde(rename = "_ts", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,

    /// Represents the minimum possible value of a PartitionKeyRange (inclusive)
    #[serde(rename = "minInclusive")]
    pub min_inclusive: EffectivePartitionKey,

    /// Represents maximum exclusive value of a PartitionKeyRange
    #[serde(rename = "maxExclusive")]
    pub max_exclusive: EffectivePartitionKey,

    /// Resource ID prefix
    #[serde(rename = "ridPrefix", skip_serializing_if = "Option::is_none")]
    pub rid_prefix: Option<i32>,

    /// Throughput fraction
    #[serde(rename = "throughputFraction", default)]
    pub throughput_fraction: f64,

    /// Target throughput
    #[serde(rename = "targetThroughput", skip_serializing_if = "Option::is_none")]
    pub target_throughput: Option<f64>,

    /// Status of the partition key range.
    ///
    /// Not part of the public API surface; uses a crate-internal enum type.
    #[serde(rename = "status", default)]
    pub(crate) status: PartitionKeyRangeStatus,

    /// Log Sequence Number
    #[serde(rename = "_lsn", default)]
    pub lsn: i64,

    /// Contains ids of parent ranges.
    /// For example if range with id '1' splits into '2' and '3',
    /// then Parents for ranges '2' and '3' will be ['1'].
    /// If range '3' splits into '4' and '5', then parents for ranges '4' and '5'
    /// will be ['1', '3'].
    #[serde(rename = "parents", skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<String>>,

    /// Contains ids of owned archival pkranges.
    /// For example, consider a range '1' owns archival reference to ['0'], to begin.
    /// If '1' splits into '2' (left) and '3' (right)
    /// '2' owns archival reference to ['0']
    /// '3' owns archival reference to ['1']
    #[serde(
        rename = "ownedArchivalPKRangeIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub owned_archival_pk_range_ids: Option<Vec<String>>,
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
            resource_id: None,
            self_link: None,
            etag: None,
            timestamp: None,
            min_inclusive: min_inclusive.into(),
            max_exclusive: max_exclusive.into(),
            rid_prefix: None,
            throughput_fraction: 0.0,
            target_throughput: None,
            status: PartitionKeyRangeStatus::default(),
            lsn: 0,
            parents: None,
            owned_archival_pk_range_ids: None,
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

// Implement PartialEq for PartitionKeyRange
// Note: Only identity fields are compared to maintain consistency with Hash.
// Floating-point fields (throughput_fraction, target_throughput) are excluded
// because f64 does not implement Hash, and Rust requires that if a == b,
// then hash(a) == hash(b).
impl PartialEq for PartitionKeyRange {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.resource_id == other.resource_id
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

// Implement a Manual Hash for PartitionKeyRange, because only the ID, RID,
// and min/max should be considered for equality/hashing.
impl Hash for PartitionKeyRange {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.resource_id.hash(state);
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
        assert_eq!(pkr.min_inclusive.as_str(), "");
        assert_eq!(pkr.max_exclusive.as_str(), "FF");
    }

    #[test]
    fn as_range() {
        let pkr = PartitionKeyRange::new("1".to_string(), "00", "FF");

        let range = pkr.as_range();
        assert_eq!(range.min.as_str(), "00");
        assert_eq!(range.max.as_str(), "FF");
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

    #[test]
    fn serialization() {
        let pkr = PartitionKeyRange {
            id: "1".to_string(),
            resource_id: Some("rid123".to_string()),
            self_link: None,
            etag: None,
            timestamp: Some(1234567890),
            min_inclusive: EffectivePartitionKey::from(""),
            max_exclusive: EffectivePartitionKey::from("FF"),
            rid_prefix: Some(42),
            throughput_fraction: 0.5,
            target_throughput: Some(1000.0),
            status: PartitionKeyRangeStatus::Online,
            lsn: 100,
            parents: Some(vec!["0".to_string()]),
            owned_archival_pk_range_ids: None,
        };

        let json = serde_json::to_string(&pkr).unwrap();
        let deserialized: PartitionKeyRange = serde_json::from_str(&json).unwrap();

        assert_eq!(pkr, deserialized);
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
