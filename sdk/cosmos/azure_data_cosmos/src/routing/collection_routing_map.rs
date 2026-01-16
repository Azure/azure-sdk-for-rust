// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![allow(dead_code)]
use crate::routing::partition_key_range::{PartitionKeyRange, PartitionKeyRangeStatus};
use crate::routing::range::Range;
use crate::routing::service_identity::ServiceIdentity;
use azure_core::Error;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

/// Stores partition key ranges efficiently with some additional information and provides
/// convenience methods for working with a set of ranges.
#[derive(Debug, Clone)]
pub struct CollectionRoutingMap {
    /// Partition key range id to partition address and range.
    range_by_id: HashMap<String, (PartitionKeyRange, Option<ServiceIdentity>)>,

    /// Ordered list of partition key ranges
    ordered_partition_key_ranges: Vec<PartitionKeyRange>,

    /// Ordered list of ranges for efficient binary search
    ordered_ranges: Vec<Range<String>>,

    /// Set of partition key range IDs that have been split (gone)
    gone_ranges: HashSet<String>,

    /// Highest non-offline partition key range ID
    highest_non_offline_pk_range_id: i32,

    /// Unique identifier for the collection
    collection_unique_id: String,

    /// ETag for change feed continuation
    pub change_feed_next_if_none_match: Option<String>,
}

const INVALID_PK_RANGE_ID: i32 = -1;
pub const MINIMUM_INCLUSIVE_EFFECTIVE_PARTITION_KEY: &str = "";
pub const MAXIMUM_EXCLUSIVE_EFFECTIVE_PARTITION_KEY: &str = "FF";

impl CollectionRoutingMap {
    /// Creates a new CollectionRoutingMap from raw components
    fn new(
        range_by_id: HashMap<String, (PartitionKeyRange, Option<ServiceIdentity>)>,
        ordered_partition_key_ranges: Vec<PartitionKeyRange>,
        collection_unique_id: String,
        change_feed_next_if_none_match: Option<String>,
    ) -> Result<Self, Error> {
        // Build ordered ranges from partition key ranges
        let ordered_ranges: Vec<Range<String>> = ordered_partition_key_ranges
            .iter()
            .map(|range| range.to_range())
            .collect();

        // Build gone ranges set from parents
        let gone_ranges: HashSet<String> = ordered_partition_key_ranges
            .iter()
            .filter_map(|r| r.parents.as_ref())
            .flat_map(|parents| parents.iter().cloned())
            .collect();

        // Calculate highest non-offline partition key range ID
        let highest_non_offline_pk_range_id = ordered_partition_key_ranges
            .iter()
            .filter_map(|range| match range.id.parse::<i32>() {
                Ok(pk_id) => {
                    if range.status != PartitionKeyRangeStatus::Offline {
                        Some(pk_id)
                    } else {
                        Some(INVALID_PK_RANGE_ID)
                    }
                }
                Err(_) => {
                    tracing::error!(
                        "Could not parse partition key range Id as int {} for collectionRid {}",
                        range.id,
                        collection_unique_id
                    );
                    None
                }
            })
            .max()
            .unwrap_or(INVALID_PK_RANGE_ID);

        Ok(Self {
            range_by_id,
            ordered_partition_key_ranges,
            ordered_ranges,
            gone_ranges,
            highest_non_offline_pk_range_id,
            collection_unique_id,
            change_feed_next_if_none_match,
        })
    }

    /// Tries to create a complete routing map from a set of ranges
    pub fn try_create_complete_routing_map(
        ranges: Vec<(PartitionKeyRange, Option<ServiceIdentity>)>,
        collection_unique_id: String,
        change_feed_next_if_none_match: Option<String>,
    ) -> Result<Option<Self>, Error> {
        let mut range_by_id: HashMap<String, (PartitionKeyRange, Option<ServiceIdentity>)> =
            HashMap::new();

        for (range, service_identity) in ranges {
            range_by_id.insert(range.id.clone(), (range, service_identity));
        }

        // Sort ranges by MinInclusive
        let mut sorted_ranges: Vec<(PartitionKeyRange, Option<ServiceIdentity>)> =
            range_by_id.values().cloned().collect();
        sorted_ranges.sort_by(|a, b| a.0.min_inclusive.cmp(&b.0.min_inclusive));

        let ordered_ranges: Vec<PartitionKeyRange> = sorted_ranges
            .iter()
            .map(|(range, _)| range.clone())
            .collect();

        if !Self::is_complete_set_of_ranges(&ordered_ranges)? {
            return Ok(None);
        }

        Ok(Some(Self::new(
            range_by_id,
            ordered_ranges,
            collection_unique_id,
            change_feed_next_if_none_match,
        )?))
    }

    /// Gets the collection unique identifier
    pub fn collection_unique_id(&self) -> &str {
        &self.collection_unique_id
    }

    /// Gets the change feed continuation token
    pub fn change_feed_next_if_none_match(&self) -> Option<&str> {
        self.change_feed_next_if_none_match.as_deref()
    }

    /// Gets the highest non-offline partition key range ID
    pub fn highest_non_offline_pk_range_id(&self) -> i32 {
        self.highest_non_offline_pk_range_id
    }

    /// Gets the ordered list of partition key ranges
    pub fn ordered_partition_key_ranges(&self) -> &[PartitionKeyRange] {
        &self.ordered_partition_key_ranges
    }

    /// Gets overlapping ranges for a single range
    pub fn get_overlapping_ranges(&self, range: &Range<String>) -> Vec<PartitionKeyRange> {
        self.get_overlapping_ranges_multi(std::slice::from_ref(range))
    }

    /// Gets overlapping ranges for multiple provided ranges
    pub fn get_overlapping_ranges_multi(
        &self,
        provided_partition_key_ranges: &[Range<String>],
    ) -> Vec<PartitionKeyRange> {
        let mut partition_ranges: std::collections::BTreeMap<String, PartitionKeyRange> =
            std::collections::BTreeMap::new();

        // Algorithm: Use binary search to find the positions of the min key and max key in the routing map
        // Then within those two positions, check for overlapping partition key ranges
        for provided_range in provided_partition_key_ranges {
            let min_index = self
                .ordered_ranges
                .binary_search_by(|probe| Self::compare_range_min(probe, provided_range))
                .unwrap_or_else(|idx| if idx > 0 { idx - 1 } else { 0 });

            let max_index = match self
                .ordered_ranges
                .binary_search_by(|probe| Self::compare_range_max(probe, provided_range))
            {
                Ok(idx) => idx,
                Err(idx) => std::cmp::min(self.ordered_partition_key_ranges.len() - 1, idx),
            };

            for i in min_index..=max_index {
                if Range::check_overlapping(&self.ordered_ranges[i], provided_range) {
                    partition_ranges.insert(
                        self.ordered_partition_key_ranges[i].min_inclusive.clone(),
                        self.ordered_partition_key_ranges[i].clone(),
                    );
                }
            }
        }

        partition_ranges.into_values().collect()
    }

    /// Gets a range by effective partition key value
    pub fn get_range_by_effective_partition_key(
        &self,
        effective_partition_key_value: &str,
    ) -> Result<&PartitionKeyRange, Error> {
        if effective_partition_key_value >= MAXIMUM_EXCLUSIVE_EFFECTIVE_PARTITION_KEY {
            return Err(Error::with_message(
                azure_core::error::ErrorKind::Other,
                "effectivePartitionKeyValue out of range",
            ));
        }

        if effective_partition_key_value == MINIMUM_INCLUSIVE_EFFECTIVE_PARTITION_KEY {
            return Ok(&self.ordered_partition_key_ranges[0]);
        }

        let search_range = Range::new(
            effective_partition_key_value.to_string(),
            effective_partition_key_value.to_string(),
            true,
            true,
        );

        let index = match self
            .ordered_ranges
            .binary_search_by(|probe| Self::compare_range_min(probe, &search_range))
        {
            Ok(idx) => idx,
            Err(idx) => {
                debug_assert!(idx > 0);
                let adjusted_idx = idx - 1;
                debug_assert!(self.ordered_ranges[adjusted_idx]
                    .contains(&effective_partition_key_value.to_string()));
                adjusted_idx
            }
        };

        Ok(&self.ordered_partition_key_ranges[index])
    }

    /// Tries to get a range by partition key range ID
    pub fn try_get_range_by_partition_key_range_id(
        &self,
        partition_key_range_id: &str,
    ) -> Option<PartitionKeyRange> {
        self.range_by_id
            .get(partition_key_range_id)
            .map(|(range, _)| range.clone())
    }

    /// Tries to get service identity by partition key range ID
    pub fn try_get_info_by_partition_key_range_id(
        &self,
        partition_key_range_id: &str,
    ) -> Option<ServiceIdentity> {
        self.range_by_id
            .get(partition_key_range_id)
            .and_then(|(_, service_identity)| service_identity.clone())
    }

    /// Tries to combine this routing map with new ranges
    pub fn try_combine(
        &self,
        ranges: Vec<(PartitionKeyRange, Option<ServiceIdentity>)>,
        change_feed_next_if_none_match: Option<String>,
    ) -> Result<Option<Self>, Error> {
        // Build new gone ranges set
        let mut new_gone_ranges: HashSet<String> = ranges
            .iter()
            .filter_map(|(range, _)| range.parents.as_ref())
            .flat_map(|parents| parents.iter().cloned())
            .collect();
        new_gone_ranges.extend(self.gone_ranges.iter().cloned());

        // Build new range_by_id, excluding gone ranges
        let mut new_range_by_id: HashMap<String, (PartitionKeyRange, Option<ServiceIdentity>)> =
            self.range_by_id
                .values()
                .filter(|(range, _)| !new_gone_ranges.contains(&range.id))
                .map(|(range, si)| (range.id.clone(), (range.clone(), si.clone())))
                .collect();

        // Add new ranges (excluding gone ranges)
        for (range, service_identity) in ranges {
            if !new_gone_ranges.contains(&range.id) {
                tracing::info!(
                    "CollectionRoutingMap.TryCombine newRangeById[{}] = {:?}",
                    range.id,
                    (range.clone(), service_identity.clone())
                );
                new_range_by_id.insert(range.id.clone(), (range, service_identity));
            }
        }

        // Sort ranges by MinInclusive
        let mut sorted_ranges: Vec<(PartitionKeyRange, Option<ServiceIdentity>)> =
            new_range_by_id.values().cloned().collect();
        sorted_ranges.sort_by(|a, b| a.0.min_inclusive.cmp(&b.0.min_inclusive));

        let new_ordered_ranges: Vec<PartitionKeyRange> = sorted_ranges
            .iter()
            .map(|(range, _)| range.clone())
            .collect();

        if !Self::is_complete_set_of_ranges(&new_ordered_ranges)? {
            return Ok(None);
        }

        Ok(Some(Self::new(
            new_range_by_id,
            new_ordered_ranges,
            self.collection_unique_id.clone(),
            change_feed_next_if_none_match,
        )?))
    }

    /// Checks if a partition key range ID is gone (has been split)
    pub fn is_gone(&self, partition_key_range_id: &str) -> bool {
        self.gone_ranges.contains(partition_key_range_id)
    }

    /// Validates that the provided ranges form a complete set covering the entire key space
    fn is_complete_set_of_ranges(ordered_ranges: &[PartitionKeyRange]) -> Result<bool, Error> {
        if ordered_ranges.is_empty() {
            return Ok(false);
        }

        let first_range = &ordered_ranges[0];
        let last_range = &ordered_ranges[ordered_ranges.len() - 1];

        let mut is_complete =
            first_range.min_inclusive == MINIMUM_INCLUSIVE_EFFECTIVE_PARTITION_KEY;
        is_complete &= last_range.max_exclusive == MAXIMUM_EXCLUSIVE_EFFECTIVE_PARTITION_KEY;

        for i in 1..ordered_ranges.len() {
            let previous_range = &ordered_ranges[i - 1];
            let current_range = &ordered_ranges[i];
            is_complete &= previous_range.max_exclusive == current_range.min_inclusive;

            if !is_complete {
                if previous_range.max_exclusive > current_range.min_inclusive {
                    return Err(Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        "Ranges overlap",
                    ));
                }
                break;
            }
        }

        Ok(is_complete)
    }

    /// Comparison function for binary search by min value
    fn compare_range_min(probe: &Range<String>, target: &Range<String>) -> Ordering {
        let min_cmp = probe.min.cmp(&target.min);
        if min_cmp != Ordering::Equal {
            return min_cmp;
        }

        // If mins are equal, compare inclusiveness
        match (probe.is_min_inclusive, target.is_min_inclusive) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }

    /// Comparison function for binary search by max value
    fn compare_range_max(probe: &Range<String>, target: &Range<String>) -> Ordering {
        let max_cmp = probe.max.cmp(&target.max);
        if max_cmp != Ordering::Equal {
            return max_cmp;
        }

        // If max is equal, compare inclusiveness
        match (probe.is_max_inclusive, target.is_max_inclusive) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn create_test_range(id: &str, min: &str, max: &str) -> PartitionKeyRange {
        PartitionKeyRange {
            id: id.to_string(),
            resource_id: Some(format!("rid_{}", id)),
            self_link: None,
            etag: None,
            timestamp: None,
            min_inclusive: min.to_string(),
            max_exclusive: max.to_string(),
            rid_prefix: None,
            throughput_fraction: 0.0,
            target_throughput: None,
            status: PartitionKeyRangeStatus::Online,
            lsn: 0,
            parents: None,
            owned_archival_pk_range_ids: None,
        }
    }

    #[test]
    fn create_complete_routing_map() {
        let ranges = vec![
            (create_test_range("0", "", "AA"), None),
            (create_test_range("1", "AA", "FF"), None),
        ];

        let routing_map = CollectionRoutingMap::try_create_complete_routing_map(
            ranges,
            "collection1".to_string(),
            Some("etag1".to_string()),
        )
        .unwrap();

        assert!(routing_map.is_some());
        let map = routing_map.unwrap();
        assert_eq!(map.ordered_partition_key_ranges.len(), 2);
        assert_eq!(map.collection_unique_id, "collection1");
    }

    #[test]
    fn get_overlapping_ranges() {
        let ranges = vec![
            (create_test_range("0", "", "33"), None),
            (create_test_range("1", "33", "66"), None),
            (create_test_range("2", "66", "FF"), None),
        ];

        let routing_map = CollectionRoutingMap::try_create_complete_routing_map(
            ranges,
            "collection1".to_string(),
            None,
        )
        .unwrap()
        .unwrap();

        let search_range = Range::new("30".to_string(), "70".to_string(), true, false);
        let overlapping = routing_map.get_overlapping_ranges(&search_range);

        assert_eq!(overlapping.len(), 3);
    }

    #[test]
    fn try_get_range_by_id() {
        let ranges = vec![
            (create_test_range("0", "", "50"), None),
            (create_test_range("1", "50", "FF"), None),
        ];

        let routing_map = CollectionRoutingMap::try_create_complete_routing_map(
            ranges,
            "collection1".to_string(),
            None,
        )
        .unwrap()
        .unwrap();

        let range = routing_map.try_get_range_by_partition_key_range_id("1");
        assert!(range.is_some());
        assert_eq!(range.unwrap().id, "1");

        let not_found = routing_map.try_get_range_by_partition_key_range_id("999");
        assert!(not_found.is_none());
    }

    #[test]
    fn is_gone() {
        let mut child1 = create_test_range("1", "", "80");
        let mut child2 = create_test_range("2", "80", "FF");

        // Set parent for child ranges
        child1.parents = Some(vec!["0".to_string()]);
        child2.parents = Some(vec!["0".to_string()]);

        let ranges = vec![(child1, None), (child2, None)];

        let routing_map = CollectionRoutingMap::try_create_complete_routing_map(
            ranges,
            "collection1".to_string(),
            None,
        )
        .unwrap()
        .unwrap();

        assert!(routing_map.is_gone("0"));
        assert!(!routing_map.is_gone("1"));
        assert!(!routing_map.is_gone("2"));
    }
}
