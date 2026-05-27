// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Canonical finite `f64` value for hashable/equatable newtypes.

use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// A finite `f64` value with canonicalized zero and optional NaN handling.
///
/// Guarantees:
/// - `-0.0` is normalized to `+0.0` for stable hashing.
/// - `new_strict` rejects non-finite values (NaN, ±∞).
/// - `new_lossy` maps non-finite values to `0.0`.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct FiniteF64(f64);

impl FiniteF64 {
    /// Creates a finite value and panics if input is not finite.
    pub(crate) fn new_strict(value: f64) -> Self {
        assert!(value.is_finite(), "FiniteF64 value must be finite");
        Self::new_lossy(value)
    }

    /// Creates a finite value, mapping non-finite values to `0.0`.
    pub(crate) fn new_lossy(value: f64) -> Self {
        if !value.is_finite() || value == 0.0 {
            Self(0.0)
        } else {
            Self(value)
        }
    }

    /// Returns the underlying `f64` value.
    pub(crate) const fn value(self) -> f64 {
        self.0
    }
}

impl PartialEq for FiniteF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for FiniteF64 {}

impl std::fmt::Display for FiniteF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Hash for FiniteF64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::FiniteF64;
    use std::hash::{Hash, Hasher};

    #[test]
    fn new_lossy_maps_non_finite_to_zero() {
        assert_eq!(FiniteF64::new_lossy(f64::NAN).value(), 0.0);
        assert_eq!(FiniteF64::new_lossy(f64::INFINITY).value(), 0.0);
        assert_eq!(FiniteF64::new_lossy(f64::NEG_INFINITY).value(), 0.0);
    }

    #[test]
    fn new_lossy_normalizes_negative_zero() {
        let value = FiniteF64::new_lossy(-0.0).value();
        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn positive_and_negative_zero_are_equal_and_hash_equal() {
        let positive_zero = FiniteF64::new_lossy(0.0);
        let negative_zero = FiniteF64::new_lossy(-0.0);

        assert_eq!(positive_zero, negative_zero);

        let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
        positive_zero.hash(&mut hasher1);

        let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
        negative_zero.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn new_strict_accepts_finite_values() {
        assert_eq!(FiniteF64::new_strict(1.5).value(), 1.5);
    }

    #[test]
    #[should_panic(expected = "FiniteF64 value must be finite")]
    fn new_strict_rejects_infinity() {
        let _ = FiniteF64::new_strict(f64::INFINITY);
    }
}
