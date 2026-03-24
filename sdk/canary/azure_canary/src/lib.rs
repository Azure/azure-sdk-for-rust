// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use azure_canary_core::NumericCore;

// Example modules organized by Rust concepts
pub mod constant_example;
pub mod enum_example;
pub mod function_example;
pub mod modules_example;
pub mod static_example;
pub mod struct_example;
pub mod struct_fields_example;
pub mod trait_example;
pub mod use_example;

// Re-export key examples for easier access
pub use constant_example::MAX_CONNECTIONS;
pub use enum_example::{Message, Status};
pub use static_example::PROGRAM_NAME;
pub use struct_example::{Person, Point2D};
pub use trait_example::Shape;

/// Basic function example
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
