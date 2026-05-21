// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Represents an Effective Partition Key range with generic bounds that implement comparison.
///
/// Named `EpkRange` to avoid conflict with `std::ops::Range`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EpkRange<T>
where
    T: Ord + Clone,
{
    /// Minimum value of the range
    #[serde(rename = "min")]
    pub min: T,

    /// Maximum value of the range
    #[serde(rename = "max")]
    pub max: T,

    /// Whether the minimum bound is inclusive
    #[serde(rename = "isMinInclusive")]
    pub is_min_inclusive: bool,

    /// Whether the maximum bound is inclusive
    #[serde(rename = "isMaxInclusive")]
    pub is_max_inclusive: bool,
}

impl<T> EpkRange<T>
where
    T: Ord + Clone,
{
    /// Creates a new Range with the specified bounds
    pub fn new(min: T, max: T, is_min_inclusive: bool, is_max_inclusive: bool) -> Self {
        Self {
            min,
            max,
            is_min_inclusive,
            is_max_inclusive,
        }
    }

    /// Creates a point range (single value)
    pub fn from_point(value: T) -> Self {
        Self {
            min: value.clone(),
            max: value,
            is_min_inclusive: true,
            is_max_inclusive: true,
        }
    }

    /// Creates an empty range at the specified value
    pub fn from_empty(value: T) -> Self {
        Self {
            min: value.clone(),
            max: value,
            is_min_inclusive: true,
            is_max_inclusive: false,
        }
    }

    /// Checks if this range contains a single value
    pub fn is_single_value(&self) -> bool {
        self.is_min_inclusive && self.is_max_inclusive && self.min == self.max
    }

    /// Checks if this range is empty
    pub fn is_empty(&self) -> bool {
        self.min == self.max && !(self.is_min_inclusive && self.is_max_inclusive)
    }

    /// Checks if the range contains the specified value
    pub fn contains(&self, value: &T) -> bool {
        let min_satisfied = if self.is_min_inclusive {
            &self.min <= value
        } else {
            &self.min < value
        };

        let max_satisfied = if self.is_max_inclusive {
            &self.max >= value
        } else {
            &self.max > value
        };

        min_satisfied && max_satisfied
    }

    /// Checks if two ranges overlap
    pub fn check_overlapping(range1: &EpkRange<T>, range2: &EpkRange<T>) -> bool {
        if range1.is_empty() || range2.is_empty() {
            return false;
        }

        let cmp1 = range1.min.cmp(&range2.max);
        let cmp2 = range2.min.cmp(&range1.max);

        if cmp1 <= Ordering::Equal && cmp2 <= Ordering::Equal {
            if (cmp1 == Ordering::Equal && !(range1.is_min_inclusive && range2.is_max_inclusive))
                || (cmp2 == Ordering::Equal
                    && !(range2.is_min_inclusive && range1.is_max_inclusive))
            {
                return false;
            }

            return true;
        }

        false
    }
}

impl<T> PartialEq for EpkRange<T>
where
    T: Ord + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min
            && self.max == other.max
            && self.is_min_inclusive == other.is_min_inclusive
            && self.is_max_inclusive == other.is_max_inclusive
    }
}

impl<T> Eq for EpkRange<T> where T: Ord + Clone {}

impl<T> Hash for EpkRange<T>
where
    T: Ord + Clone + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.min.hash(state);
        self.max.hash(state);
        self.is_min_inclusive.hash(state);
        self.is_max_inclusive.hash(state);
    }
}

impl<T> fmt::Display for EpkRange<T>
where
    T: Ord + Clone + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{},{}{}",
            if self.is_min_inclusive { "[" } else { "(" },
            self.min,
            self.max,
            if self.is_max_inclusive { "]" } else { ")" }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_creation() {
        let range = EpkRange::new(10, 20, true, false);
        assert_eq!(range.min, 10);
        assert_eq!(range.max, 20);
        assert!(range.is_min_inclusive);
        assert!(!range.is_max_inclusive);
    }

    #[test]
    fn point_range() {
        let range = EpkRange::from_point(5);
        assert_eq!(range.min, 5);
        assert_eq!(range.max, 5);
        assert!(range.is_single_value());
    }

    #[test]
    fn empty_range() {
        let range = EpkRange::from_empty(10);
        assert!(range.is_empty());
        assert_eq!(range.min, 10);
        assert_eq!(range.max, 10);
    }

    #[test]
    fn contains() {
        let range = EpkRange::new(10, 20, true, false);
        assert!(range.contains(&10)); // min inclusive
        assert!(range.contains(&15)); // middle
        assert!(!range.contains(&20)); // max exclusive
        assert!(!range.contains(&5)); // below min
        assert!(!range.contains(&25)); // above max
    }

    #[test]
    fn contains_inclusive() {
        let range = EpkRange::new(10, 20, true, true);
        assert!(range.contains(&10));
        assert!(range.contains(&20)); // max inclusive
    }

    #[test]
    fn check_overlapping() {
        let range1 = EpkRange::new(10, 20, true, false);
        let range2 = EpkRange::new(15, 25, true, false);
        assert!(EpkRange::check_overlapping(&range1, &range2));

        let range3 = EpkRange::new(25, 30, true, false);
        assert!(!EpkRange::check_overlapping(&range1, &range3));
    }

    #[test]
    fn check_overlapping_edge_cases() {
        // Touching at boundary, one inclusive one exclusive
        let range1 = EpkRange::new(10, 20, true, false);
        let range2 = EpkRange::new(20, 30, true, false);
        assert!(!EpkRange::check_overlapping(&range1, &range2));

        // Both inclusive at boundary
        let range3 = EpkRange::new(10, 20, true, true);
        let range4 = EpkRange::new(20, 30, true, false);
        assert!(EpkRange::check_overlapping(&range3, &range4));
    }

    #[test]
    fn check_overlapping_with_empty() {
        let range1 = EpkRange::new(10, 20, true, false);
        let empty = EpkRange::from_empty(15);
        assert!(!EpkRange::check_overlapping(&range1, &empty));
    }

    #[test]
    fn equality() {
        let range1 = EpkRange::new(10, 20, true, false);
        let range2 = EpkRange::new(10, 20, true, false);
        let range3 = EpkRange::new(10, 20, true, true);

        assert_eq!(range1, range2);
        assert_ne!(range1, range3);
    }

    #[test]
    fn display() {
        let range1 = EpkRange::new(10, 20, true, false);
        assert_eq!(format!("{}", range1), "[10,20)");

        let range2 = EpkRange::new(5, 15, false, true);
        assert_eq!(format!("{}", range2), "(5,15]");
    }

    #[test]
    fn string_ranges() {
        let range = EpkRange::new("AA".to_string(), "FF".to_string(), true, false);
        assert!(range.contains(&"BB".to_string()));
        assert!(range.contains(&"AA".to_string()));
        assert!(!range.contains(&"FF".to_string()));
        assert!(!range.contains(&"ZZ".to_string()));
    }

    #[test]
    fn serialization() {
        let range = EpkRange::new("00".to_string(), "FF".to_string(), true, false);
        let json = serde_json::to_string(&range).unwrap();
        let deserialized: EpkRange<String> = serde_json::from_str(&json).unwrap();

        assert_eq!(range, deserialized);
        assert!(json.contains("\"min\":\"00\""));
        assert!(json.contains("\"max\":\"FF\""));
        assert!(json.contains("\"isMinInclusive\":true"));
        assert!(json.contains("\"isMaxInclusive\":false"));
    }
}
