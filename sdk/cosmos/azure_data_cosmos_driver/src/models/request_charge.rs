// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Newtype wrapper for Request Units (RU) charges.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::Sum;
use std::ops::Add;

/// Request charge measured in Request Units (RU).
///
/// All Cosmos DB operations consume Request Units (RU), which represent
/// the compute, memory, and I/O resources consumed by the operation.
/// This newtype wraps `f64` to provide type safety and clarity.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::models::RequestCharge;
///
/// let charge = RequestCharge::new(3.5);
/// assert_eq!(charge.value(), 3.5);
///
/// // Supports addition
/// let total = charge + RequestCharge::new(2.0);
/// assert_eq!(total.value(), 5.5);
///
/// // Supports summing iterators
/// let charges = vec![RequestCharge::new(1.0), RequestCharge::new(2.0), RequestCharge::new(3.0)];
/// let sum: RequestCharge = charges.into_iter().sum();
/// assert_eq!(sum.value(), 6.0);
/// ```
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequestCharge(f64);

impl RequestCharge {
    /// Creates a new `RequestCharge` from a raw `f64` value.
    ///
    /// NaN is normalized to `0.0` and negative zero is normalized to positive
    /// zero so that `RequestCharge` can implement [`Eq`] and [`Hash`].
    pub fn new(value: f64) -> Self {
        Self(Self::normalize(value))
    }

    /// Returns the raw `f64` value of this request charge.
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Normalizes an `f64` value: NaN becomes `0.0`, and `-0.0` becomes `+0.0`.
    fn normalize(value: f64) -> f64 {
        if value.is_nan() {
            0.0
        } else if value == 0.0 {
            // Handles both +0.0 and -0.0; always store +0.0.
            0.0
        } else {
            value
        }
    }

    /// Returns canonical bits for hashing.
    ///
    /// After normalization, NaN and -0.0 are impossible, so `to_bits()` is
    /// consistent with our [`PartialEq`] implementation.
    fn canonical_bits(self) -> u64 {
        self.0.to_bits()
    }
}

impl fmt::Display for RequestCharge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for RequestCharge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for RequestCharge {}

impl PartialOrd for RequestCharge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RequestCharge {
    fn cmp(&self, other: &Self) -> Ordering {
        // After normalization NaN is impossible, so total_cmp is safe.
        self.0.total_cmp(&other.0)
    }
}

impl Hash for RequestCharge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonical_bits().hash(state);
    }
}

impl Add for RequestCharge {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl Sum for RequestCharge {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, x| acc + x)
    }
}

impl From<f64> for RequestCharge {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<RequestCharge> for f64 {
    fn from(charge: RequestCharge) -> Self {
        charge.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_zero() {
        let charge = RequestCharge::default();
        assert_eq!(charge.value(), 0.0);
    }

    #[test]
    fn new_and_value() {
        let charge = RequestCharge::new(3.5);
        assert_eq!(charge.value(), 3.5);
    }

    #[test]
    fn add() {
        let a = RequestCharge::new(1.5);
        let b = RequestCharge::new(2.5);
        assert_eq!((a + b).value(), 4.0);
    }

    #[test]
    fn sum_iterator() {
        let charges = vec![
            RequestCharge::new(1.0),
            RequestCharge::new(2.0),
            RequestCharge::new(3.0),
        ];
        let total: RequestCharge = charges.into_iter().sum();
        assert_eq!(total.value(), 6.0);
    }

    #[test]
    fn sum_empty_iterator() {
        let charges: Vec<RequestCharge> = vec![];
        let total: RequestCharge = charges.into_iter().sum();
        assert_eq!(total.value(), 0.0);
    }

    #[test]
    fn display() {
        let charge = RequestCharge::new(5.5);
        assert_eq!(format!("{}", charge), "5.5");
    }

    #[test]
    fn from_f64() {
        let charge: RequestCharge = 3.5.into();
        assert_eq!(charge.value(), 3.5);
    }

    #[test]
    fn into_f64() {
        let charge = RequestCharge::new(3.5);
        let val: f64 = charge.into();
        assert_eq!(val, 3.5);
    }

    #[test]
    fn partial_ord() {
        let a = RequestCharge::new(1.0);
        let b = RequestCharge::new(2.0);
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn serialization() {
        let charge = RequestCharge::new(3.5);
        let json = serde_json::to_string(&charge).unwrap();
        assert_eq!(json, "3.5");
    }

    #[test]
    fn deserialization() {
        let charge: RequestCharge = serde_json::from_str("3.5").unwrap();
        assert_eq!(charge.value(), 3.5);
    }

    #[test]
    fn copy_semantics() {
        let a = RequestCharge::new(1.0);
        let b = a;
        assert_eq!(a.value(), b.value()); // `a` is still usable (Copy)
    }

    #[test]
    fn nan_normalized_to_zero() {
        let charge = RequestCharge::new(f64::NAN);
        assert_eq!(charge.value(), 0.0);
    }

    #[test]
    fn negative_zero_normalized() {
        let charge = RequestCharge::new(-0.0);
        assert_eq!(charge, RequestCharge::new(0.0));
        assert_eq!(charge.value().to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn eq_and_hash_consistent() {
        use std::collections::HashSet;

        let a = RequestCharge::new(3.5);
        let b = RequestCharge::new(3.5);
        assert_eq!(a, b);

        let mut set = HashSet::new();
        set.insert(a);
        assert!(set.contains(&b));
    }

    #[test]
    fn nan_from_f64() {
        let charge: RequestCharge = f64::NAN.into();
        assert_eq!(charge.value(), 0.0);
    }

    #[test]
    fn nan_add_normalized() {
        let a = RequestCharge::new(f64::NAN);
        let b = RequestCharge::new(1.0);
        // NaN is normalized to 0.0 during construction, so 0.0 + 1.0 = 1.0
        let result = a + b;
        assert_eq!(result.value(), 1.0);
    }

    #[test]
    fn ord() {
        let a = RequestCharge::new(1.0);
        let b = RequestCharge::new(2.0);
        let c = RequestCharge::new(2.0);
        assert!(a < b);
        assert_eq!(b, c);
        assert_eq!(b.cmp(&c), std::cmp::Ordering::Equal);
    }
}
