// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/// Core numeric traits and types
pub mod numeric {
    /// Core trait for numeric operations
    ///
    /// Provides basic numeric functionality that should be
    /// implemented by all numeric types in the SDK.
    pub trait NumericCore {
        /// Validates the numeric value
        fn is_valid(&self) -> bool;

        /// Converts the value to a string representation
        fn to_string(&self) -> String;
    }
}

pub use numeric::NumericCore;

/// Adds two `u64` values and returns the result.
///
/// # Examples
/// ```
/// # use azure_template_core::add;
/// let sum = add(5, 10);
/// assert_eq!(sum, 15);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
