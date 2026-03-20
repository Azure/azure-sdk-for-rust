// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition key range data model.

use serde::Deserialize;

/// Status of a partition key range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub(crate) enum PartitionKeyRangeStatus {
    #[default]
    Online,
    Splitting,
    Offline,
    Split,
}

/// A partition key range as returned by the Cosmos DB pkranges feed.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PartitionKeyRange {
    /// Range identifier (e.g. "0", "1", "2").
    #[serde(rename = "id")]
    pub id: String,

    /// Resource ID (internal identifier).
    #[serde(rename = "_rid", default)]
    pub resource_id: Option<String>,

    /// Minimum inclusive effective partition key bound (hex string, e.g. "" or "3FFF…").
    #[serde(rename = "minInclusive")]
    pub min_inclusive: String,

    /// Maximum exclusive effective partition key bound (hex string, e.g. "FF").
    #[serde(rename = "maxExclusive")]
    pub max_exclusive: String,

    /// Status of the partition key range.
    #[serde(rename = "status", default)]
    pub status: PartitionKeyRangeStatus,

    /// Log Sequence Number.
    #[serde(rename = "_lsn", default)]
    pub lsn: i64,

    /// Parents of this range (populated after splits).
    #[serde(rename = "parents", default)]
    pub parents: Option<Vec<String>>,
}

impl PartitionKeyRange {
    /// Returns true if the given effective partition key falls within this range.
    pub fn contains(&self, epk: &str) -> bool {
        epk >= self.min_inclusive.as_str() && epk < self.max_exclusive.as_str()
    }
}

/// Response envelope from the pkranges REST feed.
#[derive(Debug, Deserialize)]
pub(crate) struct PkRangesResponse {
    #[serde(rename = "PartitionKeyRanges")]
    pub partition_key_ranges: Vec<PartitionKeyRange>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_key_range_contains() {
        let range = PartitionKeyRange {
            id: "0".into(),
            resource_id: None,
            min_inclusive: "".into(),
            max_exclusive: "FF".into(),
            status: Default::default(),
            lsn: 0,
            parents: None,
        };
        assert!(range.contains("00"));
        assert!(range.contains("7F"));
        assert!(!range.contains("FF"));
    }

    #[test]
    fn deserialize_pk_ranges_response() {
        let json = r#"{
            "PartitionKeyRanges": [
                {"id": "0", "_rid": "rid0", "minInclusive": "", "maxExclusive": "FF"}
            ]
        }"#;
        let resp: PkRangesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.partition_key_ranges.len(), 1);
        assert_eq!(resp.partition_key_ranges[0].id, "0");
    }
}
