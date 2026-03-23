// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![allow(dead_code)]

//! Collection routing map: maps effective partition keys to partition key ranges.
//!
//! The routing map stores a sorted list of partition key ranges (by `min_inclusive`)
//! and uses binary search to find which range owns a given effective partition key.

use crate::models::partition_key_range::{PartitionKeyRange, PartitionKeyRangeStatus};
use crate::models::service_identity::ServiceIdentity;
use std::collections::{HashMap, HashSet};

/// Error returned when partition key range validation fails.
#[derive(Debug)]
pub(crate) enum RoutingMapError {
    /// The ranges overlap, indicating data corruption.
    OverlappingRanges,
    /// The ranges have gaps and do not cover the full EPK space.
    IncompleteRanges,
}

impl std::fmt::Display for RoutingMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoutingMapError::OverlappingRanges => write!(f, "Partition key ranges overlap"),
            RoutingMapError::IncompleteRanges => {
                write!(f, "Partition key ranges do not cover the full EPK space")
            }
        }
    }
}

impl std::error::Error for RoutingMapError {}

/// A sorted routing map for a single collection.
///
/// Holds all partition key ranges for a collection, sorted by `min_inclusive`,
/// enabling O(log n) lookup of which range owns a given effective partition key.
#[derive(Debug, Clone)]
pub(crate) struct CollectionRoutingMap {
    /// O(1) lookup by range ID, with optional service identity for direct-mode routing.
    range_by_id: HashMap<String, (PartitionKeyRange, Option<ServiceIdentity>)>,
    /// Sorted by `min_inclusive` for binary search.
    ordered_ranges: Vec<PartitionKeyRange>,
    /// Set of partition key range IDs that have been split (gone).
    gone_ranges: HashSet<String>,
    /// Highest non-offline partition key range ID (used for split detection).
    highest_non_offline_pk_range_id: i32,
    /// Continuation token for incremental change feed fetches.
    pub change_feed_next_if_none_match: Option<String>,
}

/// Sentinel value for invalid/un-parseable partition key range IDs.
const INVALID_PK_RANGE_ID: i32 = -1;

/// Minimum inclusive effective partition key (empty string).
const MIN_EPK: &str = "";
/// Maximum exclusive effective partition key.
const MAX_EPK: &str = "FF";

impl CollectionRoutingMap {
    /// Creates an empty routing map that will fail all lookups.
    ///
    /// Used as a fallback when the service is unreachable or returns invalid data.
    pub fn empty() -> Self {
        Self {
            range_by_id: HashMap::new(),
            ordered_ranges: Vec::new(),
            gone_ranges: HashSet::new(),
            highest_non_offline_pk_range_id: INVALID_PK_RANGE_ID,
            change_feed_next_if_none_match: None,
        }
    }

    /// Creates a routing map from a list of partition key ranges (without service identity).
    ///
    /// Returns `Ok(Some(...))` on success.
    /// Returns `Ok(None)` if the input is empty.
    /// Returns `Err(RoutingMapError::OverlappingRanges)` if ranges overlap.
    /// Returns `Err(RoutingMapError::IncompleteRanges)` if ranges have gaps
    /// or do not cover the full [`""`, `"FF"`) EPK space.
    pub fn try_create(
        ranges: Vec<PartitionKeyRange>,
    ) -> Result<Option<Self>, RoutingMapError> {
        let tuples = ranges.into_iter().map(|r| (r, None)).collect();
        Self::try_create_with_continuation(tuples, None)
    }

    /// Creates a routing map from a list of partition key ranges with optional
    /// service identities and an optional change feed continuation token.
    pub fn try_create_with_continuation(
        ranges: Vec<(PartitionKeyRange, Option<ServiceIdentity>)>,
        change_feed_next_if_none_match: Option<String>,
    ) -> Result<Option<Self>, RoutingMapError> {
        if ranges.is_empty() {
            return Ok(None);
        }

        // Filter out "gone" (parent) ranges that were split.
        let gone: HashSet<String> = ranges
            .iter()
            .filter_map(|(r, _)| r.parents.as_ref())
            .flat_map(|parents| parents.iter().cloned())
            .collect();

        let mut filtered: Vec<(PartitionKeyRange, Option<ServiceIdentity>)> = ranges
            .into_iter()
            .filter(|(r, _)| !gone.contains(&r.id))
            .collect();

        // Sort by min_inclusive.
        filtered.sort_by(|a, b| a.0.min_inclusive.cmp(&b.0.min_inclusive));

        // Validate completeness: first range starts at "" and last ends at "FF",
        // and each range's max_exclusive == next range's min_inclusive.
        match filtered.first() {
            Some((r, _)) if r.min_inclusive != MIN_EPK => {
                return Err(RoutingMapError::IncompleteRanges);
            }
            None => return Ok(None),
            _ => {}
        }
        match filtered.last() {
            Some((r, _)) if r.max_exclusive != MAX_EPK => {
                return Err(RoutingMapError::IncompleteRanges);
            }
            _ => {}
        }
        for i in 0..filtered.len() - 1 {
            let prev_max = &filtered[i].0.max_exclusive;
            let next_min = &filtered[i + 1].0.min_inclusive;
            match prev_max.cmp(next_min) {
                std::cmp::Ordering::Greater => return Err(RoutingMapError::OverlappingRanges),
                std::cmp::Ordering::Less => return Err(RoutingMapError::IncompleteRanges),
                std::cmp::Ordering::Equal => {}
            }
        }

        let range_by_id: HashMap<String, (PartitionKeyRange, Option<ServiceIdentity>)> = filtered
            .iter()
            .map(|(r, si)| (r.id.clone(), (r.clone(), si.clone())))
            .collect();

        let ordered_ranges: Vec<PartitionKeyRange> = filtered.into_iter().map(|(r, _)| r).collect();

        let highest_non_offline_pk_range_id = ordered_ranges
            .iter()
            .filter_map(|r| {
                if r.status != PartitionKeyRangeStatus::Offline {
                    r.id.parse::<i32>().ok()
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(INVALID_PK_RANGE_ID);

        Ok(Some(Self {
            range_by_id,
            ordered_ranges,
            gone_ranges: gone,
            highest_non_offline_pk_range_id,
            change_feed_next_if_none_match,
        }))
    }

    /// Finds the partition key range that contains the given effective partition key.
    ///
    /// Returns `None` if no range is found (should not happen for a valid routing map).
    pub fn get_range_by_effective_partition_key(&self, epk: &str) -> Option<&PartitionKeyRange> {
        if self.ordered_ranges.is_empty() {
            return None;
        }

        // Special case: the minimum EPK is always in the first range.
        if epk == MIN_EPK {
            return Some(&self.ordered_ranges[0]);
        }

        // Binary search: find the rightmost range whose min_inclusive <= epk.
        // We search for the insertion point of epk among the min_inclusive values.
        let idx = match self
            .ordered_ranges
            .binary_search_by(|r| r.min_inclusive.as_str().cmp(epk))
        {
            Ok(i) => i,               // Exact match on min_inclusive.
            Err(i) if i > 0 => i - 1, // epk falls between ranges[i-1] and ranges[i].
            Err(_) => 0,              // Before the first range (shouldn't happen).
        };

        let range = &self.ordered_ranges[idx];
        // Inline [min_inclusive, max_exclusive) containment check to avoid
        // allocating a Range<String> and a new String on every lookup.
        if range.min_inclusive.as_str() <= epk && range.max_exclusive.as_str() > epk {
            Some(range)
        } else {
            None
        }
    }

    /// Looks up a range by its ID.
    pub fn get_range_by_id(&self, id: &str) -> Option<&PartitionKeyRange> {
        self.range_by_id.get(id).map(|(r, _)| r)
    }

    /// Looks up the service identity for a partition key range by its ID.
    pub fn get_service_identity_by_id(&self, id: &str) -> Option<&ServiceIdentity> {
        self.range_by_id.get(id).and_then(|(_, si)| si.as_ref())
    }

    /// Returns all ordered partition key ranges.
    pub fn ordered_ranges(&self) -> &[PartitionKeyRange] {
        &self.ordered_ranges
    }

    /// Returns true if the given partition key range ID has been split (gone).
    pub fn is_gone(&self, partition_key_range_id: &str) -> bool {
        self.gone_ranges.contains(partition_key_range_id)
    }

    /// Returns all partition key ranges that overlap with the given EPK range.
    ///
    /// The range is `[min_inclusive, max_exclusive)`.
    pub fn get_overlapping_ranges(
        &self,
        min_inclusive: &str,
        max_exclusive: &str,
    ) -> Vec<&PartitionKeyRange> {
        if self.ordered_ranges.is_empty() {
            return Vec::new();
        }

        // Binary search for the first range that could overlap.
        // We need the rightmost range whose min_inclusive <= min_inclusive of the query.
        let start_idx = match self
            .ordered_ranges
            .binary_search_by(|r| r.min_inclusive.as_str().cmp(min_inclusive))
        {
            Ok(i) => i,
            Err(i) if i > 0 => i - 1,
            Err(_) => 0,
        };

        let mut result = Vec::new();
        for range in &self.ordered_ranges[start_idx..] {
            // If this range's min_inclusive >= max_exclusive, no more overlaps possible.
            if range.min_inclusive.as_str() >= max_exclusive {
                break;
            }
            // A range overlaps if its max_exclusive > query min AND its min_inclusive < query max.
            if range.max_exclusive.as_str() > min_inclusive
                && range.min_inclusive.as_str() < max_exclusive
            {
                result.push(range);
            }
        }
        result
    }

    /// Returns the highest partition key range ID that is not offline.
    ///
    /// Returns `INVALID_PK_RANGE_ID` (-1) if no valid ranges exist.
    /// Used for detecting whether a split has occurred: if a range ID is higher
    /// than this value, it was produced by a split after the routing map was built.
    pub fn highest_non_offline_pk_range_id(&self) -> i32 {
        self.highest_non_offline_pk_range_id
    }

    /// Merges a set of incrementally-fetched ranges into this routing map.
    ///
    /// This is used for change-feed-based incremental refresh: the caller fetches
    /// only the ranges that changed since `self.change_feed_next_if_none_match`
    /// and passes them here. The method:
    ///
    /// 1. Collects parent IDs from the new ranges into the gone set.
    /// 2. Filters out gone ranges from both old and new sets.
    /// 3. Validates the combined set forms a complete routing map.
    ///
    /// Returns `Ok(Some(new_map))` on success, `Ok(None)` if the combined set is
    /// incomplete (caller should do a full refresh), or `Err` on overlap.
    pub fn try_combine(
        &self,
        new_ranges: Vec<(PartitionKeyRange, Option<ServiceIdentity>)>,
        change_feed_next_if_none_match: Option<String>,
    ) -> Result<Option<Self>, RoutingMapError> {
        // Accumulate all gone (parent) range IDs.
        let mut combined_gone: HashSet<String> = new_ranges
            .iter()
            .filter_map(|(r, _)| r.parents.as_ref())
            .flat_map(|parents| parents.iter().cloned())
            .collect();
        combined_gone.extend(self.gone_ranges.iter().cloned());

        // Merge range maps: start from existing (excluding gone), then add new (excluding gone).
        let mut merged: HashMap<String, (PartitionKeyRange, Option<ServiceIdentity>)> = self
            .range_by_id
            .iter()
            .filter(|(id, _)| !combined_gone.contains(*id))
            .map(|(id, (r, si))| (id.clone(), (r.clone(), si.clone())))
            .collect();

        for (range, service_identity) in new_ranges {
            if !combined_gone.contains(&range.id) {
                merged.insert(range.id.clone(), (range, service_identity));
            }
        }

        // Sort by min_inclusive.
        let mut sorted: Vec<(PartitionKeyRange, Option<ServiceIdentity>)> =
            merged.into_values().collect();
        sorted.sort_by(|a, b| a.0.min_inclusive.cmp(&b.0.min_inclusive));

        let ordered_ranges: Vec<PartitionKeyRange> =
            sorted.iter().map(|(r, _)| r.clone()).collect();

        // Validate completeness.
        if ordered_ranges.is_empty() || !Self::is_complete_range_set(&ordered_ranges) {
            return Ok(None);
        }

        // Check for overlaps and gaps.
        for i in 0..ordered_ranges.len() - 1 {
            let prev_max = &ordered_ranges[i].max_exclusive;
            let next_min = &ordered_ranges[i + 1].min_inclusive;
            match prev_max.cmp(next_min) {
                std::cmp::Ordering::Greater => return Err(RoutingMapError::OverlappingRanges),
                std::cmp::Ordering::Less => return Ok(None), // Incomplete — need full refresh
                std::cmp::Ordering::Equal => {}
            }
        }

        let range_by_id: HashMap<String, (PartitionKeyRange, Option<ServiceIdentity>)> = sorted
            .into_iter()
            .map(|(r, si)| (r.id.clone(), (r, si)))
            .collect();
        let highest = ordered_ranges
            .iter()
            .filter_map(|r| {
                if r.status != PartitionKeyRangeStatus::Offline {
                    r.id.parse::<i32>().ok()
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(INVALID_PK_RANGE_ID);

        Ok(Some(Self {
            range_by_id,
            ordered_ranges,
            gone_ranges: combined_gone,
            highest_non_offline_pk_range_id: highest,
            change_feed_next_if_none_match,
        }))
    }

    /// Checks whether a sorted slice of ranges covers the full EPK space.
    fn is_complete_range_set(sorted: &[PartitionKeyRange]) -> bool {
        match (sorted.first(), sorted.last()) {
            (Some(first), Some(last)) => {
                first.min_inclusive == MIN_EPK && last.max_exclusive == MAX_EPK
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_range(
        id: &str,
        min_inclusive: &str,
        max_exclusive: &str,
        parents: Option<Vec<String>>,
    ) -> PartitionKeyRange {
        PartitionKeyRange {
            id: id.into(),
            resource_id: None,
            self_link: None,
            etag: None,
            timestamp: None,
            min_inclusive: min_inclusive.into(),
            max_exclusive: max_exclusive.into(),
            rid_prefix: None,
            throughput_fraction: 0.0,
            target_throughput: None,
            status: Default::default(),
            lsn: 0,
            parents,
            owned_archival_pk_range_ids: None,
        }
    }

    fn single_range() -> Vec<PartitionKeyRange> {
        vec![make_range("0", "", "FF", None)]
    }

    fn three_ranges() -> Vec<PartitionKeyRange> {
        vec![
            make_range("1", "", "3F", Some(vec!["0".into()])),
            make_range("2", "3F", "7F", Some(vec!["0".into()])),
            make_range("3", "7F", "FF", Some(vec!["0".into()])),
        ]
    }

    #[test]
    fn create_single_range() {
        let map = CollectionRoutingMap::try_create(single_range())
            .unwrap()
            .unwrap();
        assert_eq!(map.ordered_ranges().len(), 1);
    }

    #[test]
    fn create_three_ranges() {
        let map = CollectionRoutingMap::try_create(three_ranges())
            .unwrap()
            .unwrap();
        assert_eq!(map.ordered_ranges().len(), 3);
    }

    #[test]
    fn lookup_in_single_range() {
        let map = CollectionRoutingMap::try_create(single_range())
            .unwrap()
            .unwrap();
        let r = map.get_range_by_effective_partition_key("7A").unwrap();
        assert_eq!(r.id, "0");
    }

    #[test]
    fn lookup_in_three_ranges() {
        let map = CollectionRoutingMap::try_create(three_ranges())
            .unwrap()
            .unwrap();

        // epk "" → range 1
        let r = map.get_range_by_effective_partition_key("").unwrap();
        assert_eq!(r.id, "1");

        // epk "20" → range 1
        let r = map.get_range_by_effective_partition_key("20").unwrap();
        assert_eq!(r.id, "1");

        // epk "3F" → range 2 (min_inclusive of range 2)
        let r = map.get_range_by_effective_partition_key("3F").unwrap();
        assert_eq!(r.id, "2");

        // epk "50" → range 2
        let r = map.get_range_by_effective_partition_key("50").unwrap();
        assert_eq!(r.id, "2");

        // epk "7F" → range 3
        let r = map.get_range_by_effective_partition_key("7F").unwrap();
        assert_eq!(r.id, "3");

        // epk "A0" → range 3
        let r = map.get_range_by_effective_partition_key("A0").unwrap();
        assert_eq!(r.id, "3");
    }

    #[test]
    fn lookup_by_id() {
        let map = CollectionRoutingMap::try_create(three_ranges())
            .unwrap()
            .unwrap();
        assert!(map.get_range_by_id("1").is_some());
        assert!(map.get_range_by_id("2").is_some());
        assert!(map.get_range_by_id("3").is_some());
        assert!(map.get_range_by_id("0").is_none()); // gone parent
    }

    #[test]
    fn incomplete_range_returns_error() {
        let ranges = vec![make_range("0", "", "7F", None)];
        let result = CollectionRoutingMap::try_create(ranges);
        assert!(matches!(result, Err(RoutingMapError::IncompleteRanges)));
    }

    #[test]
    fn overlapping_ranges_returns_error() {
        let ranges = vec![
            make_range("0", "", "80", None),
            make_range("1", "7F", "FF", None), // Overlaps with range 0
        ];
        let result = CollectionRoutingMap::try_create(ranges);
        assert!(matches!(result, Err(RoutingMapError::OverlappingRanges)));
    }

    #[test]
    fn filters_gone_parent_ranges() {
        let mut ranges = three_ranges();
        // Add the parent range "0" which should be filtered out.
        ranges.push(make_range("0", "", "FF", None));
        let map = CollectionRoutingMap::try_create(ranges)
            .unwrap()
            .unwrap();
        // Parent "0" should be filtered out, leaving 3 child ranges.
        assert_eq!(map.ordered_ranges().len(), 3);
        assert!(map.get_range_by_id("0").is_none());
    }

    #[test]
    fn is_gone_tracks_parent_ranges() {
        let map = CollectionRoutingMap::try_create(three_ranges())
            .unwrap()
            .unwrap();
        // "0" is listed as a parent in all three child ranges.
        assert!(map.is_gone("0"));
        assert!(!map.is_gone("1"));
        assert!(!map.is_gone("2"));
        assert!(!map.is_gone("3"));
    }

    #[test]
    fn get_overlapping_ranges_full_span() {
        let map = CollectionRoutingMap::try_create(three_ranges())
            .unwrap()
            .unwrap();
        // Query the full EPK space — should return all ranges.
        let overlapping = map.get_overlapping_ranges("", "FF");
        assert_eq!(overlapping.len(), 3);
    }

    #[test]
    fn get_overlapping_ranges_partial() {
        let map = CollectionRoutingMap::try_create(three_ranges())
            .unwrap()
            .unwrap();
        // Query [30, 50) — overlaps range 1 (max "3F" > "30") and range 2 (min "3F" < "50").
        let overlapping = map.get_overlapping_ranges("30", "50");
        assert_eq!(overlapping.len(), 2);
        assert_eq!(overlapping[0].id, "1");
        assert_eq!(overlapping[1].id, "2");
    }

    #[test]
    fn get_overlapping_ranges_single() {
        let map = CollectionRoutingMap::try_create(three_ranges())
            .unwrap()
            .unwrap();
        // Query [40, 50) — only range 2 [3F, 7F).
        let overlapping = map.get_overlapping_ranges("40", "50");
        assert_eq!(overlapping.len(), 1);
        assert_eq!(overlapping[0].id, "2");
    }

    #[test]
    fn empty_input_returns_none() {
        let result = CollectionRoutingMap::try_create(vec![]).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn try_combine_split_produces_valid_map() {
        // Start with a single range covering the full EPK space.
        let map = CollectionRoutingMap::try_create(single_range())
            .unwrap()
            .unwrap();

        // Simulate a split: range "0" splits into "1" [, 7F) and "2" [7F, FF).
        let new_ranges = vec![
            (
                make_range("1", "", "7F", Some(vec!["0".into()])),
                None,
            ),
            (
                make_range("2", "7F", "FF", Some(vec!["0".into()])),
                None,
            ),
        ];

        let merged = map
            .try_combine(new_ranges, Some("new-etag".into()))
            .unwrap()
            .unwrap();

        // Parent "0" should be gone, two child ranges remain.
        assert_eq!(merged.ordered_ranges().len(), 2);
        assert!(merged.is_gone("0"));
        assert!(!merged.is_gone("1"));
        assert!(!merged.is_gone("2"));
        // EPK lookup should work on the merged map.
        assert_eq!(
            merged
                .get_range_by_effective_partition_key("30")
                .unwrap()
                .id,
            "1"
        );
        assert_eq!(
            merged
                .get_range_by_effective_partition_key("A0")
                .unwrap()
                .id,
            "2"
        );
    }

    #[test]
    fn try_combine_incomplete_returns_none() {
        let map = CollectionRoutingMap::try_create(single_range())
            .unwrap()
            .unwrap();

        // Only one child range — the merged set has a gap [7F, FF).
        let new_ranges = vec![(
            make_range("1", "", "7F", Some(vec!["0".into()])),
            None,
        )];

        let result = map.try_combine(new_ranges, Some("etag".into())).unwrap();
        assert!(result.is_none(), "Incomplete merge should return None");
    }

    #[test]
    fn try_combine_overlapping_returns_error() {
        let map = CollectionRoutingMap::try_create(single_range())
            .unwrap()
            .unwrap();

        // Two children that overlap: [, 80) and [7F, FF) — "80" > "7F".
        let new_ranges = vec![
            (
                make_range("1", "", "80", Some(vec!["0".into()])),
                None,
            ),
            (
                make_range("2", "7F", "FF", Some(vec!["0".into()])),
                None,
            ),
        ];

        let result = map.try_combine(new_ranges, Some("etag".into()));
        assert!(matches!(result, Err(RoutingMapError::OverlappingRanges)));
    }
}
