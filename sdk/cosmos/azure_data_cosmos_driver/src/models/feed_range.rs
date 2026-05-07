// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Feed range type for the Cosmos DB driver.
//!
//! A [`FeedRange`] represents a contiguous range of the effective partition key (EPK) space.
//! It is used by the dataflow pipeline to target operations at one or more physical partitions.

use crate::models::effective_partition_key::EffectivePartitionKey;

/// A contiguous range of the effective partition key space.
///
/// Defined by `[min_inclusive, max_exclusive)` EPK boundaries. A `FeedRange` may
/// map to one or several physical partitions depending on the current partition
/// topology.
///
/// Use [`FeedRange::full()`] for the entire key space (`""..FF`).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FeedRange {
    min_inclusive: EffectivePartitionKey,
    max_exclusive: EffectivePartitionKey,
}

impl FeedRange {
    /// Creates a feed range from explicit EPK bounds.
    pub fn new(min_inclusive: EffectivePartitionKey, max_exclusive: EffectivePartitionKey) -> Self {
        Self {
            min_inclusive,
            max_exclusive,
        }
    }

    /// Creates a feed range covering the entire partition key space (`""..FF`).
    pub fn full() -> Self {
        Self {
            min_inclusive: EffectivePartitionKey::min(),
            max_exclusive: EffectivePartitionKey::max(),
        }
    }

    /// Returns the inclusive lower bound of this range.
    pub fn min_inclusive(&self) -> &EffectivePartitionKey {
        &self.min_inclusive
    }

    /// Returns the exclusive upper bound of this range.
    pub fn max_exclusive(&self) -> &EffectivePartitionKey {
        &self.max_exclusive
    }

    /// Returns `true` if this feed range is entirely contained within `other`.
    pub fn is_subset_of(&self, other: &FeedRange) -> bool {
        other.min_inclusive <= self.min_inclusive && other.max_exclusive >= self.max_exclusive
    }

    /// Returns `true` if this feed range and `other` share any portion of the EPK space.
    ///
    /// Two feed ranges overlap when one starts before the other ends and vice versa.
    pub fn overlaps(&self, other: &FeedRange) -> bool {
        self.min_inclusive < other.max_exclusive && other.min_inclusive < self.max_exclusive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_range() {
        let full = FeedRange::full();
        assert_eq!(full.min_inclusive().as_str(), "");
        assert_eq!(full.max_exclusive().as_str(), "FF");
    }

    #[test]
    fn is_subset_of_full() {
        let full = FeedRange::full();
        let sub = FeedRange::new(
            EffectivePartitionKey::from("00"),
            EffectivePartitionKey::from("80"),
        );
        assert!(sub.is_subset_of(&full));
        assert!(!full.is_subset_of(&sub));
    }

    #[test]
    fn is_subset_of_self() {
        let range = FeedRange::new(
            EffectivePartitionKey::from("20"),
            EffectivePartitionKey::from("80"),
        );
        assert!(range.is_subset_of(&range));
    }

    #[test]
    fn overlaps_basic() {
        let a = FeedRange::new(
            EffectivePartitionKey::from("00"),
            EffectivePartitionKey::from("50"),
        );
        let b = FeedRange::new(
            EffectivePartitionKey::from("30"),
            EffectivePartitionKey::from("80"),
        );
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn overlaps_adjacent_no_overlap() {
        let a = FeedRange::new(
            EffectivePartitionKey::from("00"),
            EffectivePartitionKey::from("50"),
        );
        let b = FeedRange::new(
            EffectivePartitionKey::from("50"),
            EffectivePartitionKey::from("FF"),
        );
        // Adjacent ranges (a's max == b's min) do NOT overlap because max is exclusive.
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn overlaps_disjoint() {
        let a = FeedRange::new(
            EffectivePartitionKey::from("00"),
            EffectivePartitionKey::from("30"),
        );
        let b = FeedRange::new(
            EffectivePartitionKey::from("50"),
            EffectivePartitionKey::from("FF"),
        );
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }
}
