// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Container routing map: maps effective partition keys to partition key ranges.
//!
//! The routing map stores a sorted list of partition key ranges (by `min_inclusive`)
//! and uses binary search to find which range owns a given effective partition key.

use crate::models::effective_partition_key::EffectivePartitionKey;
use crate::models::partition_key_range::{PartitionKeyRange, PartitionKeyRangeStatus};
use azure_core::http::Etag;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

/// Error returned when partition key range validation fails.
#[derive(Debug)]
#[non_exhaustive]
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

/// A sorted routing map for a single container.
///
/// Holds all partition key ranges for a container, sorted by `min_inclusive`,
/// enabling O(log n) lookup of which range owns a given effective partition key.
#[derive(Debug, Clone)]
pub(crate) struct ContainerRoutingMap {
    /// O(1) lookup by range ID.
    range_by_id: HashMap<String, PartitionKeyRange>,
    /// Sorted by `min_inclusive` for binary search.
    ordered_ranges: Vec<PartitionKeyRange>,
    /// Set of partition key range IDs that have been split (gone).
    gone_ranges: HashSet<String>,
    /// Highest non-offline partition key range ID (used for split detection).
    highest_non_offline_pk_range_id: i32,
    /// ETag for incremental change feed refresh.
    pub etag: Option<Etag>,
    /// Continuation token for incremental change feed fetches.
    pub change_feed_next_if_none_match: Option<String>,
}

/// Sentinel value for invalid/un-parseable partition key range IDs.
const INVALID_PK_RANGE_ID: i32 = -1;

impl ContainerRoutingMap {
    /// Creates an empty routing map that will fail all lookups.
    ///
    /// Used as a fallback when the service is unreachable or returns invalid data.
    pub fn empty() -> Self {
        Self {
            range_by_id: HashMap::new(),
            ordered_ranges: Vec::new(),
            gone_ranges: HashSet::new(),
            highest_non_offline_pk_range_id: INVALID_PK_RANGE_ID,
            etag: None,
            change_feed_next_if_none_match: None,
        }
    }

    /// Creates a routing map from a list of partition key ranges with an
    /// optional change feed continuation token.
    ///
    /// Returns `Ok(Some(...))` on success.
    /// Returns `Ok(None)` if the input is empty.
    /// Returns `Err(RoutingMapError::OverlappingRanges)` if ranges overlap.
    /// Returns `Err(RoutingMapError::IncompleteRanges)` if ranges have gaps
    /// or do not cover the full [`""`, `"FF"`) EPK space.
    pub fn try_create(
        ranges: Vec<PartitionKeyRange>,
        etag: Option<Etag>,
        change_feed_next_if_none_match: Option<String>,
    ) -> Result<Option<Self>, RoutingMapError> {
        if ranges.is_empty() {
            return Ok(None);
        }

        // Filter out "gone" (parent) ranges that were split.
        let gone: HashSet<String> = ranges
            .iter()
            .filter_map(|r| r.parents.as_ref())
            .flat_map(|parents| parents.iter().cloned())
            .collect();

        let mut filtered: Vec<PartitionKeyRange> = ranges
            .into_iter()
            .filter(|r| !gone.contains(&r.id))
            .collect();

        if filtered.is_empty() {
            return Ok(None);
        }

        // Sort by min_inclusive (uses Ord implementation on PartitionKeyRange).
        filtered.sort();
        let (highest_non_offline_pk_range_id, range_by_id) =
            Self::validate_and_build_index(&filtered)?;

        Ok(Some(Self {
            range_by_id,
            ordered_ranges: filtered,
            gone_ranges: gone,
            highest_non_offline_pk_range_id,
            etag,
            change_feed_next_if_none_match,
        }))
    }

    /// Finds the partition key range that contains the given effective partition key.
    ///
    /// Returns `None` if no range is found (should not happen for a valid routing map).
    pub fn get_range_by_effective_partition_key(
        &self,
        epk: &EffectivePartitionKey,
    ) -> Option<&PartitionKeyRange> {
        if self.ordered_ranges.is_empty() {
            return None;
        }

        // Special case: the minimum EPK is always in the first range.
        if epk.as_bytes().is_empty() {
            return Some(&self.ordered_ranges[0]);
        }

        let idx = self.find_range_index(epk);

        let range = &self.ordered_ranges[idx];
        let min_ok = range.min_inclusive <= *epk;
        let max_ok = *epk < range.max_exclusive;
        if min_ok && max_ok {
            Some(range)
        } else {
            None
        }
    }

    /// Looks up a range by its ID.
    pub fn range(&self, id: &str) -> Option<&PartitionKeyRange> {
        self.range_by_id.get(id)
    }

    /// Returns all partition key ranges, sorted by `min_inclusive`.
    pub fn ranges(&self) -> &[PartitionKeyRange] {
        &self.ordered_ranges
    }

    /// Returns true if the given partition key range ID has been split (gone).
    pub fn is_gone(&self, partition_key_range_id: &str) -> bool {
        self.gone_ranges.contains(partition_key_range_id)
    }

    /// Returns all partition key ranges that overlap with the given EPK range.
    ///
    /// The input range is `[start, end)` (inclusive start, exclusive end),
    /// matching the semantics of [`std::ops::Range`].
    pub fn get_overlapping_ranges(
        &self,
        epk_range: Range<&EffectivePartitionKey>,
    ) -> Vec<&PartitionKeyRange> {
        let Some((start_idx, end_idx)) = self.overlapping_range_bounds(epk_range) else {
            return Vec::new();
        };
        self.ordered_ranges[start_idx..end_idx].iter().collect()
    }

    /// Returns the ID of the single partition key range that overlaps the given
    /// EPK range, or `None` when the range maps to zero or more than one
    /// physical partition.
    ///
    /// This is a cheaper alternative to [`get_overlapping_ranges`](Self::get_overlapping_ranges)
    /// for callers that only need to know whether a feed range is owned by
    /// exactly one physical partition (e.g. PPCB/PPAF first-attempt
    /// attribution). It reuses the same O(log n) binary-search bounds but
    /// clones at most a single ID instead of every overlapping range.
    ///
    /// A multi-partition overlap is unexpected for callers on the operation
    /// pipeline path: the dataflow pipeline splits multi-partition feed ranges
    /// into one sub-operation per physical partition before execution. If it
    /// happens here it signals a stale routing map / partition-split race, so
    /// we surface it via `warn!` + `debug_assert!` and still return `None` so
    /// the caller degrades gracefully.
    pub fn single_overlapping_range_id(
        &self,
        epk_range: Range<&EffectivePartitionKey>,
    ) -> Option<String> {
        let (start_idx, end_idx) = self.overlapping_range_bounds(epk_range)?;
        match end_idx - start_idx {
            0 => None,
            1 => Some(self.ordered_ranges[start_idx].id.clone()),
            count => {
                debug_assert!(
                    false,
                    "feed range mapped to {count} physical partitions; expected \
                     exactly one at the operation pipeline (stale routing map / \
                     partition-split race)"
                );
                tracing::warn!(
                    overlapping_partition_count = count,
                    "feed range mapped to multiple physical partitions during \
                     single-owner resolution; treating as no single owner (stale \
                     routing map / partition-split race)"
                );
                None
            }
        }
    }

    /// Computes the `[start_idx, end_idx)` slice of `ordered_ranges` that
    /// overlaps the given EPK range. Returns `None` when the map is empty.
    ///
    /// Because `ordered_ranges` is sorted AND contiguous (no gaps/overlaps),
    /// the overlapping ranges form a contiguous slice, found via two binary
    /// searches for O(log n) total.
    fn overlapping_range_bounds(
        &self,
        epk_range: Range<&EffectivePartitionKey>,
    ) -> Option<(usize, usize)> {
        if self.ordered_ranges.is_empty() {
            return None;
        }

        let min_epk = epk_range.start;
        let max_epk = epk_range.end;

        // Start: rightmost range whose min_inclusive <= query min.
        let start_idx = self.find_range_index(min_epk);

        // End: first range whose min_inclusive >= query max (all ranges from
        // start_idx up to but not including this index overlap the query).
        let end_idx = self.ordered_ranges[start_idx..]
            .partition_point(|r| r.min_inclusive < *max_epk)
            + start_idx;

        Some((start_idx, end_idx))
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
        new_ranges: Vec<PartitionKeyRange>,
        change_feed_next_if_none_match: Option<String>,
    ) -> Result<Option<Self>, RoutingMapError> {
        // Accumulate all gone (parent) range IDs.
        let mut combined_gone: HashSet<String> = new_ranges
            .iter()
            .filter_map(|r| r.parents.as_ref())
            .flat_map(|parents| parents.iter().cloned())
            .collect();
        combined_gone.extend(self.gone_ranges.iter().cloned());

        // Merge range maps: start from existing (excluding gone), then add new (excluding gone).
        let mut merged: HashMap<String, PartitionKeyRange> = self
            .range_by_id
            .iter()
            .filter(|(id, _)| !combined_gone.contains(*id))
            .map(|(id, r)| (id.clone(), r.clone()))
            .collect();

        for range in new_ranges {
            if !combined_gone.contains(&range.id) {
                merged.insert(range.id.clone(), range);
            }
        }

        // Sort by min_inclusive (uses Ord implementation on PartitionKeyRange).
        let mut sorted: Vec<PartitionKeyRange> = merged.into_values().collect();
        sorted.sort();

        // Validate contiguity: gaps mean we need a full refresh (Ok(None)),
        // overlaps are always an error.
        if sorted.is_empty() {
            return Ok(None);
        }
        let (highest_non_offline_pk_range_id, range_by_id) =
            match Self::validate_and_build_index(&sorted) {
                Ok(result) => result,
                Err(RoutingMapError::IncompleteRanges) => return Ok(None),
                Err(e) => return Err(e),
            };

        Ok(Some(Self {
            range_by_id,
            ordered_ranges: sorted,
            gone_ranges: combined_gone,
            highest_non_offline_pk_range_id,
            etag: self.etag.clone(),
            change_feed_next_if_none_match,
        }))
    }

    /// Binary-searches `ordered_ranges` for the rightmost range whose
    /// `min_inclusive <= epk`.
    ///
    /// Callers must ensure `ordered_ranges` is non-empty and `epk` is non-empty.
    fn find_range_index(&self, epk: &EffectivePartitionKey) -> usize {
        match self
            .ordered_ranges
            .binary_search_by(|r| r.min_inclusive.cmp(epk))
        {
            Ok(i) => i,               // Exact match on min_inclusive.
            Err(i) if i > 0 => i - 1, // epk falls between ranges[i-1] and ranges[i].
            Err(_) => unreachable!("EPK before first range; constructor guarantees full coverage"),
        }
    }

    /// Validates that a non-empty sorted slice of ranges forms a contiguous,
    /// complete partition of the EPK space `["", "FF")`, then builds the
    /// by-ID index and computes the highest non-offline range ID.
    ///
    /// Returns `Ok((highest_id, range_by_id))` on success.
    /// Returns `Err(OverlappingRanges)` if any range's min is less than the
    /// previous range's max.
    /// Returns `Err(IncompleteRanges)` if there are gaps or the ranges don't
    /// cover the full `["", "FF")` interval.
    fn validate_and_build_index(
        sorted: &[PartitionKeyRange],
    ) -> Result<(i32, HashMap<String, PartitionKeyRange>), RoutingMapError> {
        let max_epk = EffectivePartitionKey::MAX.clone();
        let mut expected_min = EffectivePartitionKey::MIN.clone();
        for range in sorted {
            match range.min_inclusive.cmp(&expected_min) {
                std::cmp::Ordering::Greater => return Err(RoutingMapError::IncompleteRanges),
                std::cmp::Ordering::Less => return Err(RoutingMapError::OverlappingRanges),
                std::cmp::Ordering::Equal => {}
            }
            expected_min = range.max_exclusive.clone();
        }
        if expected_min != max_epk {
            return Err(RoutingMapError::IncompleteRanges);
        }

        let range_by_id: HashMap<String, PartitionKeyRange> =
            sorted.iter().map(|r| (r.id.clone(), r.clone())).collect();

        let highest_non_offline_pk_range_id = sorted
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

        Ok((highest_non_offline_pk_range_id, range_by_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    fn epk(s: &str) -> EffectivePartitionKey {
        EffectivePartitionKey::from(s.to_string())
    }

    fn make_range(
        id: &str,
        min_inclusive: &str,
        max_exclusive: &str,
        parents: Option<Vec<String>>,
    ) -> PartitionKeyRange {
        let mut r = PartitionKeyRange::new(id.into(), min_inclusive, max_exclusive);
        r.parents = parents;
        r
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
        let map = ContainerRoutingMap::try_create(single_range(), None, None)
            .unwrap()
            .unwrap();
        let ranges = map.ranges();
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].id, "0");
        assert_eq!(ranges[0].min_inclusive, "");
        assert_eq!(ranges[0].max_exclusive, "FF");
    }

    #[test]
    fn create_three_ranges() {
        let map = ContainerRoutingMap::try_create(three_ranges(), None, None)
            .unwrap()
            .unwrap();
        let ids: Vec<&str> = map.ranges().iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["1", "2", "3"]);
        assert_eq!(map.ranges()[0].min_inclusive, "");
        assert_eq!(map.ranges()[0].max_exclusive, "3F");
        assert_eq!(map.ranges()[1].min_inclusive, "3F");
        assert_eq!(map.ranges()[1].max_exclusive, "7F");
        assert_eq!(map.ranges()[2].min_inclusive, "7F");
        assert_eq!(map.ranges()[2].max_exclusive, "FF");
    }

    #[test]
    fn lookup_in_single_range() {
        let map = ContainerRoutingMap::try_create(single_range(), None, None)
            .unwrap()
            .unwrap();
        let r = map
            .get_range_by_effective_partition_key(&epk("7A"))
            .unwrap();
        assert_eq!(r.id, "0");
    }

    #[test]
    fn lookup_in_three_ranges() {
        let map = ContainerRoutingMap::try_create(three_ranges(), None, None)
            .unwrap()
            .unwrap();

        // epk "" → range 1
        let r = map.get_range_by_effective_partition_key(&epk("")).unwrap();
        assert_eq!(r.id, "1");

        // epk "20" → range 1
        let r = map
            .get_range_by_effective_partition_key(&epk("20"))
            .unwrap();
        assert_eq!(r.id, "1");

        // epk "3F" → range 2 (min_inclusive of range 2)
        let r = map
            .get_range_by_effective_partition_key(&epk("3F"))
            .unwrap();
        assert_eq!(r.id, "2");

        // epk "50" → range 2
        let r = map
            .get_range_by_effective_partition_key(&epk("50"))
            .unwrap();
        assert_eq!(r.id, "2");

        // epk "7F" → range 3
        let r = map
            .get_range_by_effective_partition_key(&epk("7F"))
            .unwrap();
        assert_eq!(r.id, "3");

        // epk "A0" → range 3
        let r = map
            .get_range_by_effective_partition_key(&epk("A0"))
            .unwrap();
        assert_eq!(r.id, "3");
    }

    #[test]
    fn lookup_by_id() {
        let map = ContainerRoutingMap::try_create(three_ranges(), None, None)
            .unwrap()
            .unwrap();
        let r = map.range("2").unwrap();
        assert_eq!(r.id, "2");
        assert_eq!(r.min_inclusive, "3F");
        assert_eq!(r.max_exclusive, "7F");
        assert!(map.range("0").is_none()); // gone parent
    }

    #[test]
    fn incomplete_range_returns_error() {
        let ranges = vec![make_range("0", "", "7F", None)];
        let result = ContainerRoutingMap::try_create(ranges, None, None);
        assert!(matches!(result, Err(RoutingMapError::IncompleteRanges)));
    }

    #[test]
    fn overlapping_ranges_returns_error() {
        let ranges = vec![
            make_range("0", "", "80", None),
            make_range("1", "7F", "FF", None), // Overlaps with range 0
        ];
        let result = ContainerRoutingMap::try_create(ranges, None, None);
        assert!(matches!(result, Err(RoutingMapError::OverlappingRanges)));
    }

    #[test]
    fn filters_gone_parent_ranges() {
        let mut ranges = three_ranges();
        // Add the parent range "0" which should be filtered out.
        ranges.push(make_range("0", "", "FF", None));
        let map = ContainerRoutingMap::try_create(ranges, None, None)
            .unwrap()
            .unwrap();
        // Parent "0" should be filtered out, leaving 3 child ranges.
        let ids: Vec<&str> = map.ranges().iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["1", "2", "3"]);
        assert!(map.range("0").is_none());
    }

    #[test]
    fn is_gone_tracks_parent_ranges() {
        let map = ContainerRoutingMap::try_create(three_ranges(), None, None)
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
        let map = ContainerRoutingMap::try_create(three_ranges(), None, None)
            .unwrap()
            .unwrap();
        // Query the full EPK space — should return all ranges.
        let overlapping = map.get_overlapping_ranges(&epk("")..&epk("FF"));
        let ids: Vec<&str> = overlapping.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["1", "2", "3"]);
    }

    #[test]
    fn get_overlapping_ranges_partial() {
        let map = ContainerRoutingMap::try_create(three_ranges(), None, None)
            .unwrap()
            .unwrap();
        // Query [30, 50) — overlaps range 1 (max "3F" > "30") and range 2 (min "3F" < "50").
        let overlapping = map.get_overlapping_ranges(&epk("30")..&epk("50"));
        let ids: Vec<&str> = overlapping.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["1", "2"]);
    }

    #[test]
    fn get_overlapping_ranges_single() {
        let map = ContainerRoutingMap::try_create(three_ranges(), None, None)
            .unwrap()
            .unwrap();
        // Query [40, 50) — only range 2 [3F, 7F).
        let overlapping = map.get_overlapping_ranges(&epk("40")..&epk("50"));
        let ids: Vec<&str> = overlapping.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["2"]);
    }

    #[test]
    fn single_overlapping_range_id_one_partition_returns_id() {
        let map = ContainerRoutingMap::try_create(three_ranges(), None, None)
            .unwrap()
            .unwrap();
        // Query [40, 50) — owned entirely by range 2 [3F, 7F).
        let id = map.single_overlapping_range_id(&epk("40")..&epk("50"));
        assert_eq!(id.as_deref(), Some("2"));
    }

    #[test]
    #[should_panic(expected = "physical partitions")]
    fn single_overlapping_range_id_multiple_partitions_panics_in_debug() {
        let map = ContainerRoutingMap::try_create(three_ranges(), None, None)
            .unwrap()
            .unwrap();
        // A multi-partition overlap is an invariant violation for this helper
        // (the dataflow pipeline should have split the range first), so it trips
        // the `debug_assert!`. In release builds it returns `None` instead.
        let _ = map.single_overlapping_range_id(&epk("")..&epk("FF"));
    }

    #[test]
    fn single_overlapping_range_id_single_partition_map_returns_id() {
        let map = ContainerRoutingMap::try_create(single_range(), None, None)
            .unwrap()
            .unwrap();
        // Whole space against a one-partition container → that partition owns it.
        let id = map.single_overlapping_range_id(&epk("")..&epk("FF"));
        assert_eq!(id.as_deref(), Some("0"));
    }

    #[test]
    fn empty_input_returns_none() {
        let result = ContainerRoutingMap::try_create(vec![], None, None).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn try_combine_split_produces_valid_map() {
        // Start with a single range covering the full EPK space.
        let map = ContainerRoutingMap::try_create(single_range(), None, None)
            .unwrap()
            .unwrap();

        // Simulate a split: range "0" splits into "1" [, 7F) and "2" [7F, FF).
        let new_ranges = vec![
            make_range("1", "", "7F", Some(vec!["0".into()])),
            make_range("2", "7F", "FF", Some(vec!["0".into()])),
        ];

        let merged = map
            .try_combine(new_ranges, Some("new-etag".into()))
            .unwrap()
            .unwrap();

        // Parent "0" should be gone, two child ranges remain.
        let ids: Vec<&str> = merged.ranges().iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["1", "2"]);
        assert!(merged.is_gone("0"));
        // EPK lookup should work on the merged map.
        assert_eq!(
            merged
                .get_range_by_effective_partition_key(&epk("30"))
                .unwrap()
                .id,
            "1"
        );
        assert_eq!(
            merged
                .get_range_by_effective_partition_key(&epk("A0"))
                .unwrap()
                .id,
            "2"
        );
    }

    #[test]
    fn try_combine_resolves_cascading_splits_from_one_page() {
        let map = ContainerRoutingMap::try_create(single_range(), None, None)
            .unwrap()
            .unwrap();

        // 0 -> B + C, then B -> D + E. Children of B intentionally precede B.
        let new_ranges = vec![
            make_range("D", "", "33", Some(vec!["B".into()])),
            make_range("E", "33", "55", Some(vec!["B".into()])),
            make_range("B", "", "55", Some(vec!["0".into()])),
            make_range("C", "55", "FF", Some(vec!["0".into()])),
        ];

        let merged = map
            .try_combine(new_ranges, Some("new-etag".into()))
            .unwrap()
            .unwrap();

        let ids = merged
            .ranges()
            .iter()
            .map(|range| range.id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(ids, ["D", "E", "C"]);
        assert!(merged.is_gone("0"));
        assert!(merged.is_gone("B"));
    }

    #[test]
    fn try_combine_incomplete_returns_none() {
        let map = ContainerRoutingMap::try_create(single_range(), None, None)
            .unwrap()
            .unwrap();

        // Only one child range — the merged set has a gap [7F, FF).
        let new_ranges = vec![make_range("1", "", "7F", Some(vec!["0".into()]))];

        let result = map.try_combine(new_ranges, Some("etag".into())).unwrap();
        assert!(result.is_none(), "Incomplete merge should return None");
    }

    #[test]
    fn try_combine_overlapping_returns_error() {
        let map = ContainerRoutingMap::try_create(single_range(), None, None)
            .unwrap()
            .unwrap();

        // Two children that overlap: [, 80) and [7F, FF) — "80" > "7F".
        let new_ranges = vec![
            make_range("1", "", "80", Some(vec!["0".into()])),
            make_range("2", "7F", "FF", Some(vec!["0".into()])),
        ];

        let result = map.try_combine(new_ranges, Some("etag".into()));
        assert!(matches!(result, Err(RoutingMapError::OverlappingRanges)));
    }

    /// Builds a range with an explicit lifecycle status. Used by the
    /// `highest_non_offline_pk_range_id` regression tests below; not exposed
    /// through the slim `PartitionKeyRange::new` API because production
    /// callers should not be hand-stamping `status`.
    fn make_range_with_status(
        id: &str,
        min_inclusive: &str,
        max_exclusive: &str,
        parents: Option<Vec<String>>,
        status: PartitionKeyRangeStatus,
    ) -> PartitionKeyRange {
        let mut r = PartitionKeyRange::new(id.into(), min_inclusive, max_exclusive);
        r.parents = parents;
        r.status = status;
        r
    }

    #[test]
    fn highest_non_offline_pk_range_id_picks_max_online_id() {
        // Three online ranges, ids "1", "2", "3" — highest should be 3.
        let map = ContainerRoutingMap::try_create(three_ranges(), None, None)
            .unwrap()
            .unwrap();
        assert_eq!(map.highest_non_offline_pk_range_id(), 3);
    }

    /// Regression guard: when an incremental change-feed merge re-publishes
    /// an existing range with `status = Offline`, the cached
    /// `highest_non_offline_pk_range_id` must be recomputed from the merged
    /// view (not stay pinned at the pre-merge value). This is the exact
    /// path that would silently break if we ever stripped `status` without
    /// a replacement plumbed through `try_combine`.
    #[test]
    fn try_combine_online_to_offline_recomputes_highest_non_offline() {
        let map = ContainerRoutingMap::try_create(three_ranges(), None, None)
            .unwrap()
            .unwrap();
        assert_eq!(map.highest_non_offline_pk_range_id(), 3);

        // Service marks range "3" as Offline. Same id/EPK extents, no
        // parents — this is a status flip, not a split.
        let new_ranges = vec![make_range_with_status(
            "3",
            "7F",
            "FF",
            None,
            PartitionKeyRangeStatus::Offline,
        )];

        let merged = map
            .try_combine(new_ranges, Some("etag".into()))
            .unwrap()
            .unwrap();

        // Range "3" is still in the routing map (offline ranges are NOT
        // gone — they're in a transient state and may flip back).
        assert!(
            merged.range_by_id.contains_key("3"),
            "Offline range should remain in the routing map"
        );
        // But it must no longer count toward the highest non-offline id.
        assert_eq!(
            merged.highest_non_offline_pk_range_id(),
            2,
            "Highest non-offline id should drop from 3 to 2 after the status flip"
        );

        // Recovery path: the same range comes back Online (e.g., after a
        // planned failover completes). The highest-non-offline id must
        // bump back to 3 — `validate_and_build_index` recomputes from
        // scratch on every merge, so this should "just work", but a
        // future caching-of-highest optimization could silently break it.
        let recovered_ranges = vec![make_range_with_status(
            "3",
            "7F",
            "FF",
            None,
            PartitionKeyRangeStatus::Online,
        )];
        let recovered = merged
            .try_combine(recovered_ranges, Some("etag2".into()))
            .unwrap()
            .unwrap();
        assert_eq!(
            recovered.highest_non_offline_pk_range_id(),
            3,
            "Highest non-offline id should bump back to 3 after the range recovers"
        );
    }

    // -- Length-aware EPK ordering tests --

    #[test]
    fn epk_cmp_equal_strings() {
        assert_eq!(epk("06AB34CF").cmp(&epk("06AB34CF")), Ordering::Equal);
    }

    #[test]
    fn epk_cmp_shorter_less_than_longer_nonzero_suffix() {
        assert_eq!(
            epk("06AB34CF").cmp(&epk("06AB34CF11223344")),
            Ordering::Less
        );
    }

    #[test]
    fn epk_cmp_prefix_with_partial_zero_suffix_is_equal() {
        assert_eq!(
            epk("06AB34CF").cmp(&epk("06AB34CF00000000")),
            Ordering::Equal
        );
    }

    #[test]
    fn epk_cmp_prefix_with_zero_suffix_is_equal() {
        assert_eq!(
            epk("06AB34CFE4E482236BCACBBF50E234AB").cmp(&epk(
                "06AB34CFE4E482236BCACBBF50E234AB00000000000000000000000000000000"
            )),
            Ordering::Equal
        );
    }

    #[test]
    fn epk_cmp_zero_padded_first_arg() {
        assert_eq!(
            epk("06AB34CFE4E482236BCACBBF50E234AB00000000000000000000000000000000")
                .cmp(&epk("06AB34CFE4E482236BCACBBF50E234AB")),
            Ordering::Equal
        );
    }

    #[test]
    fn epk_cmp_different_prefixes() {
        assert_eq!(epk("06AB34CF").cmp(&epk("07AB34CF")), Ordering::Less);
        assert_eq!(epk("07AB34CF").cmp(&epk("06AB34CF")), Ordering::Greater);
    }

    // -- HPK mixed-length boundary tests (ported from .NET PR #5260) --

    /// Builds a routing map with mixed-length EPK boundaries typical of
    /// HPK containers, matching the .NET test GenerateRoutingMap.
    fn hpk_ranges() -> Vec<PartitionKeyRange> {
        vec![
            make_range("0", "", "03559A67F2724111B5E565DFA8711A00", None),
            make_range(
                "1",
                "03559A67F2724111B5E565DFA8711A00",
                "06AB34CFE4E482236BCACBBF50E234AB00000000000000000000000000000000",
                None,
            ),
            make_range(
                "2",
                "06AB34CFE4E482236BCACBBF50E234AB00000000000000000000000000000000",
                "0BD3FBE846AF75790CE63F78B1A81620",
                None,
            ),
            make_range(
                "3",
                "0BD3FBE846AF75790CE63F78B1A81620",
                "0BD3FBE846AF75790CE63F78B1A8163100000000000000000000000000000000",
                None,
            ),
            make_range(
                "11",
                "0BD3FBE846AF75790CE63F78B1A8163100000000000000000000000000000000",
                "0BD3FBE846AF75790CE63F78B1A81631FF",
                None,
            ),
            make_range(
                "12",
                "0BD3FBE846AF75790CE63F78B1A81631FF",
                "0D4DC2CD8F49C65A8E0C5306B61B4343",
                None,
            ),
            make_range(
                "4",
                "0D4DC2CD8F49C65A8E0C5306B61B4343",
                "0D4EC2CD8F49C65A8E0C5306B61B4343",
                None,
            ),
            make_range(
                "44",
                "0D4EC2CD8F49C65A8E0C5306B61B4343",
                "0D5DC2CD8F49C65A8E0C5306B61B4343",
                None,
            ),
            make_range(
                "24",
                "0D5DC2CD8F49C65A8E0C5306B61B4343",
                "0DCEB8CE51C6BFE84F4BD9409F69B9BB2164DEBD78C50C850E0C1E3E3F0579ED",
                None,
            ),
            make_range(
                "5",
                "0DCEB8CE51C6BFE84F4BD9409F69B9BB2164DEBD78C50C850E0C1E3E3F0579ED",
                "FF",
                None,
            ),
        ]
    }

    /// .NET scenario 1.1: Partial input EPK on boundary between ranges 1 and 2.
    /// The partial EPK "06AB...AB" matches the fully-specified boundary
    /// "06AB...AB00000000000000000000000000000000" — should resolve to range 2 only.
    #[test]
    fn hpk_partial_epk_on_boundary_returns_correct_range() {
        let map = ContainerRoutingMap::try_create(hpk_ranges(), None, None)
            .unwrap()
            .unwrap();

        let overlapping = map.get_overlapping_ranges(
            &epk("06AB34CFE4E482236BCACBBF50E234AB")..&epk("06AB34CFE4E482236BCACBBF50E234ABFF"),
        );
        let ids: Vec<&str> = overlapping.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["2"]);
    }

    /// .NET scenario 1.2: Partial EPK on another boundary.
    #[test]
    fn hpk_partial_epk_boundary_second_split() {
        let map = ContainerRoutingMap::try_create(hpk_ranges(), None, None)
            .unwrap()
            .unwrap();

        let overlapping = map.get_overlapping_ranges(
            &epk("0BD3FBE846AF75790CE63F78B1A81631")..&epk("0BD3FBE846AF75790CE63F78B1A81631FF"),
        );
        let ids: Vec<&str> = overlapping.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["11"]);
    }

    /// .NET scenario 1.2 (continued): Fully-specified input within a single range.
    #[test]
    fn hpk_full_epk_within_single_range() {
        let map = ContainerRoutingMap::try_create(hpk_ranges(), None, None)
            .unwrap()
            .unwrap();

        let overlapping = map.get_overlapping_ranges(
            &epk("0D4DC2CD8F49C65A8E0C5306B61B43440D4DC2CD8F49C65A8E0C5306B61B4343")
                ..&epk("0D4DC2CD8F49C65A8E0C5306B61B43440D4DC2CD8F49C65A8E0C5306B61B4344"),
        );
        let ids: Vec<&str> = overlapping.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["4"]);
    }

    /// .NET scenario 1.2 (continued): Range that falls inside range 3.
    #[test]
    fn hpk_range_inside_range_3() {
        let map = ContainerRoutingMap::try_create(hpk_ranges(), None, None)
            .unwrap()
            .unwrap();

        let overlapping = map.get_overlapping_ranges(
            &epk("0BD3FBE846AF75790CE63F78B1A81620")..&epk("0BD3FBE846AF75790CE63F78B1A81631"),
        );
        let ids: Vec<&str> = overlapping.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["3"]);
    }

    /// .NET scenario 1.3: Partial EPK spans two overlapping ranges.
    #[test]
    fn hpk_partial_epk_spans_two_ranges() {
        let map = ContainerRoutingMap::try_create(hpk_ranges(), None, None)
            .unwrap()
            .unwrap();

        let overlapping = map.get_overlapping_ranges(
            &epk("0DCEB8CE51C6BFE84F4BD9409F69B9BB")..&epk("0DCEB8CE51C6BFE84F4BD9409F69B9BBFF"),
        );
        let ids: Vec<&str> = overlapping.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["24", "5"]);
    }

    /// .NET scenario 1.4: Partial point EPK in the middle.
    #[test]
    fn hpk_partial_point_epk_in_middle() {
        let map = ContainerRoutingMap::try_create(hpk_ranges(), None, None)
            .unwrap()
            .unwrap();

        let r = map
            .get_range_by_effective_partition_key(&epk("02559A67F2724111B5E565DFA8711A00"))
            .unwrap();
        assert_eq!(r.id, "0");
    }

    /// .NET scenario 1.5: Partial point EPK where range has partial boundaries.
    #[test]
    fn hpk_partial_point_epk_in_partial_range() {
        let map = ContainerRoutingMap::try_create(hpk_ranges(), None, None)
            .unwrap()
            .unwrap();

        let r = map
            .get_range_by_effective_partition_key(&epk("0D4DC2CD8F49C65A8E0C5306B61B4345"))
            .unwrap();
        assert_eq!(r.id, "4");
    }

    /// .NET scenario 1.6: Fully-specified input against partially-specified backend range.
    #[test]
    fn hpk_full_epk_against_partial_backend_range() {
        let map = ContainerRoutingMap::try_create(hpk_ranges(), None, None)
            .unwrap()
            .unwrap();

        let overlapping = map.get_overlapping_ranges(
            &epk("0D4DC2CD8F49C65A8E0C5306B61B434300000000000000000000000000000000")
                ..&epk("0D4EC2CD8F49C65A8E0C5306B61B434300000000000000000000000000000000"),
        );
        let ids: Vec<&str> = overlapping.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["4"]);
    }
}
