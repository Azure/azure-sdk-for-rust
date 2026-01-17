// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![allow(dead_code)]

use crate::routing::range::Range;
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
    pub etag: Option<String>,

    /// Gets the last modified timestamp associated with the resource
    #[serde(rename = "_ts", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,

    /// Represents the minimum possible value of a PartitionKeyRange (inclusive)
    #[serde(rename = "minInclusive")]
    pub min_inclusive: String,

    /// Represents maximum exclusive value of a PartitionKeyRange
    #[serde(rename = "maxExclusive")]
    pub max_exclusive: String,

    /// Resource ID prefix
    #[serde(rename = "ridPrefix", skip_serializing_if = "Option::is_none")]
    pub rid_prefix: Option<i32>,

    /// Throughput fraction
    #[serde(rename = "throughputFraction", default)]
    pub throughput_fraction: f64,

    /// Target throughput
    #[serde(rename = "targetThroughput", skip_serializing_if = "Option::is_none")]
    pub target_throughput: Option<f64>,

    /// Status of the partition key range
    #[serde(rename = "status", default)]
    pub status: PartitionKeyRangeStatus,

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
pub enum PartitionKeyRangeStatus {
    #[default]
    Online,
    Splitting,
    Offline,
    Split,
}

impl PartitionKeyRange {
    /// Creates a new PartitionKeyRange with required fields
    pub fn new(id: String, min_inclusive: String, max_exclusive: String) -> Self {
        Self {
            id,
            resource_id: None,
            self_link: None,
            etag: None,
            timestamp: None,
            min_inclusive,
            max_exclusive,
            rid_prefix: None,
            throughput_fraction: 0.0,
            target_throughput: None,
            status: PartitionKeyRangeStatus::default(),
            lsn: 0,
            parents: None,
            owned_archival_pk_range_ids: None,
        }
    }

    /// Converts this PartitionKeyRange to a Range<String>
    pub fn to_range(&self) -> Range<String> {
        Range {
            min: self.min_inclusive.clone(),
            max: self.max_exclusive.clone(),
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
impl PartialEq for PartitionKeyRange {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.resource_id == other.resource_id
            && self.min_inclusive == other.min_inclusive
            && self.max_exclusive == other.max_exclusive
            && self.target_throughput == other.target_throughput
            && self.throughput_fraction == other.throughput_fraction
    }
}

impl Eq for PartitionKeyRange {}

// Implement Hash for PartitionKeyRange
impl Hash for PartitionKeyRange {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.resource_id.hash(state);
        self.min_inclusive.hash(state);
        self.max_exclusive.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_key_range_creation() {
        let pkr = PartitionKeyRange::new("1".to_string(), "".to_string(), "FF".to_string());

        assert_eq!(pkr.id, "1");
        assert_eq!(pkr.min_inclusive, "");
        assert_eq!(pkr.max_exclusive, "FF");
    }

    #[test]
    fn to_range() {
        let pkr = PartitionKeyRange::new("1".to_string(), "00".to_string(), "FF".to_string());

        let range = pkr.to_range();
        assert_eq!(range.min, "00");
        assert_eq!(range.max, "FF");
        assert!(range.is_min_inclusive);
        assert!(!range.is_max_inclusive);
    }

    #[test]
    fn equality_check() {
        let pkr1 = PartitionKeyRange::new("1".to_string(), "00".to_string(), "FF".to_string());

        let mut pkr2 = PartitionKeyRange::new("1".to_string(), "00".to_string(), "FF".to_string());

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
            min_inclusive: "".to_string(),
            max_exclusive: "FF".to_string(),
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
        let range1: Range<String> = Range::new("00".to_string(), "50".to_string(), true, false);
        let range2 = Range::new("40".to_string(), "80".to_string(), true, false);
        let range3 = Range::new("60".to_string(), "90".to_string(), true, false);
        assert!(Range::check_overlapping(&range1, &range2));
        assert!(!Range::check_overlapping(&range1, &range3));
    }
}
