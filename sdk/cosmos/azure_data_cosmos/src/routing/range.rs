// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// range.rs

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Represents a range with generic bounds that implement comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range<T>
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

impl<T> Range<T>
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
    pub fn get_point_range(value: T) -> Self {
        Self {
            min: value.clone(),
            max: value,
            is_min_inclusive: true,
            is_max_inclusive: true,
        }
    }

    /// Creates an empty range at the specified value
    pub fn get_empty_range(value: T) -> Self {
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
    pub fn check_overlapping(range1: &Range<T>, range2: &Range<T>) -> bool {
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

impl<T> PartialEq for Range<T>
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

impl<T> Eq for Range<T> where T: Ord + Clone {}

impl<T> Hash for Range<T>
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

impl<T> fmt::Display for Range<T>
where
    T: Ord + Clone + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            if self.is_min_inclusive { "[" } else { "(" },
            self.min,
            ",",
            self.max,
            if self.is_max_inclusive { "]" } else { ")" }
        )
    }
}

/// Comparer for Range that compares by minimum value
#[derive(Debug, Clone, Copy)]
pub struct MinComparer;

impl MinComparer {
    /// Compares two ranges by their minimum bounds
    pub fn compare<T>(left: &Range<T>, right: &Range<T>) -> Ordering
    where
        T: Ord + Clone,
    {
        let result = left.min.cmp(&right.min);
        if result != Ordering::Equal || left.is_min_inclusive == right.is_min_inclusive {
            return result;
        }

        if left.is_min_inclusive {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

/// Comparer for Range that compares by maximum value
#[derive(Debug, Clone, Copy)]
pub struct MaxComparer;

impl MaxComparer {
    /// Compares two ranges by their maximum bounds
    pub fn compare<T>(left: &Range<T>, right: &Range<T>) -> Ordering
    where
        T: Ord + Clone,
    {
        let result = left.max.cmp(&right.max);

        if result != Ordering::Equal || left.is_max_inclusive == right.is_max_inclusive {
            return result;
        }

        if left.is_max_inclusive {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_creation() {
        let range = Range::new(10, 20, true, false);
        assert_eq!(range.min, 10);
        assert_eq!(range.max, 20);
        assert!(range.is_min_inclusive);
        assert!(!range.is_max_inclusive);
    }

    #[test]
    fn test_point_range() {
        let range = Range::get_point_range(5);
        assert_eq!(range.min, 5);
        assert_eq!(range.max, 5);
        assert!(range.is_single_value());
    }

    #[test]
    fn test_empty_range() {
        let range = Range::get_empty_range(10);
        assert!(range.is_empty());
        assert_eq!(range.min, 10);
        assert_eq!(range.max, 10);
    }

    #[test]
    fn test_contains() {
        let range = Range::new(10, 20, true, false);
        assert!(range.contains(&10)); // min inclusive
        assert!(range.contains(&15)); // middle
        assert!(!range.contains(&20)); // max exclusive
        assert!(!range.contains(&5)); // below min
        assert!(!range.contains(&25)); // above max
    }

    #[test]
    fn test_contains_inclusive() {
        let range = Range::new(10, 20, true, true);
        assert!(range.contains(&10));
        assert!(range.contains(&20)); // max inclusive
    }

    #[test]
    fn test_check_overlapping() {
        let range1 = Range::new(10, 20, true, false);
        let range2 = Range::new(15, 25, true, false);
        assert!(Range::check_overlapping(&range1, &range2));

        let range3 = Range::new(25, 30, true, false);
        assert!(!Range::check_overlapping(&range1, &range3));
    }

    #[test]
    fn test_check_overlapping_edge_cases() {
        // Touching at boundary, one inclusive one exclusive
        let range1 = Range::new(10, 20, true, false);
        let range2 = Range::new(20, 30, true, false);
        assert!(!Range::check_overlapping(&range1, &range2));

        // Both inclusive at boundary
        let range3 = Range::new(10, 20, true, true);
        let range4 = Range::new(20, 30, true, false);
        assert!(Range::check_overlapping(&range3, &range4));
    }

    #[test]
    fn test_check_overlapping_with_empty() {
        let range1 = Range::new(10, 20, true, false);
        let empty = Range::get_empty_range(15);
        assert!(!Range::check_overlapping(&range1, &empty));
    }

    #[test]
    fn test_equality() {
        let range1 = Range::new(10, 20, true, false);
        let range2 = Range::new(10, 20, true, false);
        let range3 = Range::new(10, 20, true, true);

        assert_eq!(range1, range2);
        assert_ne!(range1, range3);
    }

    #[test]
    fn test_display() {
        let range1 = Range::new(10, 20, true, false);
        assert_eq!(format!("{}", range1), "[10,20)");

        let range2 = Range::new(5, 15, false, true);
        assert_eq!(format!("{}", range2), "(5,15]");
    }

    #[test]
    fn test_min_comparer() {
        let range1 = Range::new(10, 20, true, false);
        let range2 = Range::new(15, 25, true, false);
        let range3 = Range::new(10, 30, false, false);

        assert_eq!(MinComparer::compare(&range1, &range2), Ordering::Less);
        assert_eq!(MinComparer::compare(&range2, &range1), Ordering::Greater);

        // Same min, but different inclusiveness
        assert_eq!(MinComparer::compare(&range1, &range3), Ordering::Less);
    }

    #[test]
    fn test_max_comparer() {
        let range1 = Range::new(10, 20, true, false);
        let range2 = Range::new(15, 25, true, false);
        let range3 = Range::new(5, 20, true, true);

        assert_eq!(MaxComparer::compare(&range1, &range2), Ordering::Less);
        assert_eq!(MaxComparer::compare(&range2, &range1), Ordering::Greater);

        // Same max, but different inclusiveness
        assert_eq!(MaxComparer::compare(&range1, &range3), Ordering::Less);
        assert_eq!(MaxComparer::compare(&range3, &range1), Ordering::Greater);
    }

    #[test]
    fn test_string_ranges() {
        let range = Range::new("AA".to_string(), "FF".to_string(), true, false);
        assert!(range.contains(&"BB".to_string()));
        assert!(range.contains(&"AA".to_string()));
        assert!(!range.contains(&"FF".to_string()));
        assert!(!range.contains(&"ZZ".to_string()));
    }

    #[test]
    fn test_serialization() {
        let range = Range::new("00".to_string(), "FF".to_string(), true, false);
        let json = serde_json::to_string(&range).unwrap();
        let deserialized: Range<String> = serde_json::from_str(&json).unwrap();

        assert_eq!(range, deserialized);
        assert!(json.contains("\"min\":\"00\""));
        assert!(json.contains("\"max\":\"FF\""));
        assert!(json.contains("\"isMinInclusive\":true"));
        assert!(json.contains("\"isMaxInclusive\":false"));
    }
}