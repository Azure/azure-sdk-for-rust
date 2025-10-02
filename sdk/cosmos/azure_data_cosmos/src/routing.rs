use std::ops::Range;

use crate::types::{EffectivePartitionKey, PartitionKeyRangeId};
use serde::{Deserialize, Serialize};

/// Represents a partition key range within a Cosmos DB collection.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct PartitionKeyRange {
    // The ID of the partition key range.
    pub id: PartitionKeyRangeId,

    /// The minimum inclusive value of the partition key range.
    pub min_inclusive: EffectivePartitionKey,

    /// The maximum exclusive value of the partition key range.
    pub max_exclusive: EffectivePartitionKey,

    /// The parents of the partition key range, if they still exist.
    ///
    /// During a split or merge, this will contain the ID of the partition key range(s) that were split or merged to create this range.
    #[serde(default)]
    pub parents: Vec<PartitionKeyRangeId>,
}

impl PartitionKeyRange {
    /// Checks if the given effective partition key falls within this partition key range.
    pub fn contains(&self, epk: &EffectivePartitionKey) -> bool {
        self.min_inclusive <= *epk && *epk < self.max_exclusive
    }

    /// Compares the given effective partition key to this partition key range, returning an [`Ordering`](std::cmp::Ordering) that can be used for searching a sorted list of ranges for the range containing the key.
    pub fn compare_to(&self, epk: &EffectivePartitionKey) -> std::cmp::Ordering {
        if self.contains(epk) {
            std::cmp::Ordering::Equal
        } else if self.min_inclusive > *epk {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }

    pub fn overlaps(&self, range: &Range<EffectivePartitionKey>) -> bool {
        // Empty ranges don't overlap with anything
        if range.start >= range.end {
            return false;
        }
        !(self.max_exclusive <= range.start || self.min_inclusive >= range.end)
    }
}

#[derive(Debug)]
pub struct ContainerRoutingMap {
    /// The list of partition key ranges for the container, sorted by their minimum inclusive value.
    pk_ranges: Vec<PartitionKeyRange>,
}

impl ContainerRoutingMap {
    /// Creates a new `ContainerRoutingMap` from the provided partition key ranges.
    pub fn new(mut pk_ranges: Vec<PartitionKeyRange>) -> Self {
        normalize_ranges(&mut pk_ranges);
        Self { pk_ranges }
    }

    /// Replaces the partition key ranges with a new set, normalizing them in the process.
    pub fn with_ranges(mut self, mut pk_ranges: Vec<PartitionKeyRange>) -> Self {
        normalize_ranges(&mut pk_ranges);
        self.pk_ranges = pk_ranges;
        self
    }

    /// Gets the list of partition key ranges for the container, sorted by their minimum inclusive value.
    pub fn ranges(&self) -> &[PartitionKeyRange] {
        &self.pk_ranges
    }

    /// Finds the partition key range that contains the given effective partition key, if any.
    pub fn range_containing(&self, epk: &EffectivePartitionKey) -> Option<&PartitionKeyRange> {
        // It's critical that pk_ranges is sorted by min_inclusive for this to work correctly, so assert that in debug builds
        debug_assert!(self.pk_ranges.is_sorted_by_key(|pkr| &pkr.min_inclusive));
        self.pk_ranges
            .binary_search_by(|pkr| pkr.compare_to(epk))
            .ok()
            .map(|idx| &self.pk_ranges[idx])
    }

    /// Finds the partition key range with the given ID, if any.
    pub fn range(&self, id: &PartitionKeyRangeId) -> Option<&PartitionKeyRange> {
        // We don't know that IDs are sorted, so just do a linear search
        self.pk_ranges.iter().find(|pkr| &pkr.id == id)
    }

    pub fn overlapping_ranges(
        &self,
        ranges: &[Range<EffectivePartitionKey>],
    ) -> Vec<&PartitionKeyRange> {
        // TODO: Could be optimized further later
        self.pk_ranges
            .iter()
            .filter(|pkr| ranges.iter().any(|r| pkr.overlaps(r)))
            .collect()
    }
}

/// Discards any [`PartitionKeyRange`] that is the parent of another range, leaving only the leaf ranges.
/// Also sorts the ranges by their minimum inclusive value.
fn normalize_ranges(pk_ranges: &mut Vec<PartitionKeyRange>) {
    let parent_ids: std::collections::HashSet<_> = pk_ranges
        .iter()
        .flat_map(|pkr| pkr.parents.iter().cloned())
        .collect();
    pk_ranges.retain(|pkr| !parent_ids.contains(&pkr.id));
    pk_ranges.sort_by(|a, b| a.min_inclusive.cmp(&b.min_inclusive));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_range(id: &str, min: &str, max: &str, parents: Vec<&str>) -> PartitionKeyRange {
        PartitionKeyRange {
            id: id.into(),
            min_inclusive: min.into(),
            max_exclusive: max.into(),
            parents: parents.into_iter().map(|p| p.into()).collect(),
        }
    }

    #[test]
    fn partition_key_range_contains() {
        let range = create_range("1", "00", "33", vec![]);

        assert!(range.contains(&"00".into()));
        assert!(range.contains(&"15".into()));
        assert!(range.contains(&"32".into()));
        assert!(!range.contains(&"33".into()));
        assert!(!range.contains(&"34".into()));
        assert!(!range.contains(&"FF".into()));
    }

    #[test]
    fn partition_key_range_overlaps() {
        let range = create_range("1", "20", "60", vec![]);

        assert!(range.overlaps(&("30".into().."40".into())));
        assert!(range.overlaps(&("10".into().."30".into())));
        assert!(range.overlaps(&("50".into().."70".into())));
        assert!(range.overlaps(&("10".into().."70".into())));
        assert!(range.overlaps(&("20".into().."60".into())));
        assert!(!range.overlaps(&("00".into().."20".into())));
        assert!(!range.overlaps(&("60".into().."80".into())));
        assert!(!range.overlaps(&("30".into().."30".into())));
    }

    #[test]
    fn container_routing_map_construction() {
        let ranges = vec![
            create_range("2", "33", "66", vec![]),
            create_range("1", "00", "33", vec![]),
            create_range("3", "66", "FF", vec![]),
        ];

        let map = ContainerRoutingMap::new(ranges);
        assert_eq!(map.ranges().len(), 3);
        assert_eq!(map.ranges()[0].id.value(), "1");
        assert_eq!(map.ranges()[1].id.value(), "2");
        assert_eq!(map.ranges()[2].id.value(), "3");

        let new_ranges = vec![
            create_range("2", "33", "66", vec![]),
            create_range("1", "00", "33", vec![]),
        ];
        let map = map.with_ranges(new_ranges);
        assert_eq!(map.ranges().len(), 2);
        assert_eq!(map.ranges()[0].id.value(), "1");
        assert_eq!(map.ranges()[1].id.value(), "2");
    }

    #[test]
    fn container_routing_map_range_containing() {
        let ranges = vec![
            create_range("1", "00", "33", vec![]),
            create_range("2", "33", "66", vec![]),
            create_range("3", "66", "FF", vec![]),
        ];

        let map = ContainerRoutingMap::new(ranges);

        assert_eq!(map.range_containing(&"00".into()).unwrap().id.value(), "1");
        assert_eq!(map.range_containing(&"32".into()).unwrap().id.value(), "1");
        assert_eq!(map.range_containing(&"33".into()).unwrap().id.value(), "2");
        assert_eq!(map.range_containing(&"50".into()).unwrap().id.value(), "2");
        assert_eq!(map.range_containing(&"66".into()).unwrap().id.value(), "3");
        assert_eq!(map.range_containing(&"AA".into()).unwrap().id.value(), "3");

        let single_range_map =
            ContainerRoutingMap::new(vec![create_range("1", "33", "66", vec![])]);
        assert!(single_range_map.range_containing(&"00".into()).is_none());
        assert!(single_range_map.range_containing(&"99".into()).is_none());
    }

    #[test]
    fn container_routing_map_range_by_id() {
        let ranges = vec![
            create_range("1", "00", "33", vec![]),
            create_range("2", "33", "66", vec![]),
            create_range("3", "66", "FF", vec![]),
        ];

        let map = ContainerRoutingMap::new(ranges);

        assert_eq!(map.range(&"1".into()).unwrap().min_inclusive.value(), "00");
        assert_eq!(map.range(&"2".into()).unwrap().min_inclusive.value(), "33");
        assert_eq!(map.range(&"3".into()).unwrap().min_inclusive.value(), "66");
        assert!(map.range(&"99".into()).is_none());
    }

    #[test]
    fn container_routing_map_overlapping_ranges() {
        let ranges = vec![
            create_range("1", "00", "33", vec![]),
            create_range("2", "33", "66", vec![]),
            create_range("3", "66", "FF", vec![]),
        ];

        let map = ContainerRoutingMap::new(ranges);

        let overlapping = map.overlapping_ranges(&["00".into().."20".into()]);
        assert_eq!(overlapping.len(), 1);
        assert_eq!(overlapping[0].id.value(), "1");

        let overlapping = map.overlapping_ranges(&["20".into().."50".into()]);
        assert_eq!(overlapping.len(), 2);
        assert_eq!(overlapping[0].id.value(), "1");
        assert_eq!(overlapping[1].id.value(), "2");

        let overlapping = map.overlapping_ranges(&["00".into().."FF".into()]);
        assert_eq!(overlapping.len(), 3);

        let overlapping = map.overlapping_ranges(&[]);
        assert_eq!(overlapping.len(), 0);

        let overlapping =
            map.overlapping_ranges(&["10".into().."20".into(), "70".into().."80".into()]);
        assert_eq!(overlapping.len(), 2);
        assert_eq!(overlapping[0].id.value(), "1");
        assert_eq!(overlapping[1].id.value(), "3");
    }

    #[test]
    fn normalize_ranges_removes_parents() {
        let mut ranges = vec![
            create_range("parent1", "00", "50", vec![]),
            create_range("parent2", "50", "FF", vec![]),
            create_range("child1", "00", "25", vec!["parent1"]),
            create_range("child2", "25", "50", vec!["parent1"]),
            create_range("child3", "50", "FF", vec!["parent2"]),
        ];

        normalize_ranges(&mut ranges);

        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0].id.value(), "child1");
        assert_eq!(ranges[1].id.value(), "child2");
        assert_eq!(ranges[2].id.value(), "child3");
        assert!(ranges[0].min_inclusive <= ranges[1].min_inclusive);
        assert!(ranges[1].min_inclusive <= ranges[2].min_inclusive);
    }

    #[test]
    fn normalize_ranges_sorts_by_min_inclusive() {
        let mut ranges = vec![
            create_range("3", "66", "99", vec![]),
            create_range("1", "00", "33", vec![]),
            create_range("2", "33", "66", vec![]),
        ];

        normalize_ranges(&mut ranges);
        assert_eq!(ranges[0].id.value(), "1");
        assert_eq!(ranges[1].id.value(), "2");
        assert_eq!(ranges[2].id.value(), "3");
    }

    #[test]
    fn normalize_ranges_handles_empty_list() {
        let mut ranges: Vec<PartitionKeyRange> = vec![];
        normalize_ranges(&mut ranges);
        assert_eq!(ranges.len(), 0);
    }

    #[test]
    fn normalize_ranges_handles_no_parents() {
        let mut ranges = vec![
            create_range("2", "33", "66", vec![]),
            create_range("1", "00", "33", vec![]),
        ];

        normalize_ranges(&mut ranges);
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].id.value(), "1");
        assert_eq!(ranges[1].id.value(), "2");
    }

    #[test]
    fn container_routing_map_full_test() {
        let ranges = vec![
            create_range("parent1", "00", "80", vec![]),
            create_range("parent2", "80", "FF", vec![]),
            create_range("child1", "00", "40", vec!["parent1"]),
            create_range("child2", "40", "80", vec!["parent1"]),
            create_range("child3", "80", "C0", vec!["parent2"]),
            create_range("child4", "C0", "FF", vec!["parent2"]),
        ];

        let map = ContainerRoutingMap::new(ranges);

        assert_eq!(map.ranges().len(), 4);
        assert_eq!(
            map.range_containing(&"10".into()).unwrap().id.value(),
            "child1"
        );
        assert_eq!(
            map.range_containing(&"50".into()).unwrap().id.value(),
            "child2"
        );
        assert_eq!(
            map.range_containing(&"90".into()).unwrap().id.value(),
            "child3"
        );
        assert_eq!(
            map.range_containing(&"D0".into()).unwrap().id.value(),
            "child4"
        );

        let overlapping = map.overlapping_ranges(&["30".into().."70".into()]);
        assert_eq!(overlapping.len(), 2);
        assert_eq!(overlapping[0].id.value(), "child1");
        assert_eq!(overlapping[1].id.value(), "child2");
    }
}
