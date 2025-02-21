// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

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

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
