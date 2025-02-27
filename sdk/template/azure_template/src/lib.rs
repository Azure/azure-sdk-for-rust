// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use azure_template_core::NumericCore;

// Example modules organized by Rust concepts
pub mod struct_example;
pub mod modules_example;
pub mod trait_example;
pub mod enum_example;
pub mod function_example;
pub mod constant_example;
pub mod static_example;
pub mod use_example;
pub mod struct_fields_example;

// Re-export key examples for easier access
pub use struct_example::{Person, Point2D};
pub use enum_example::{Status, Message};
pub use trait_example::Shape;
pub use function_example::add;
pub use constant_example::MAX_CONNECTIONS;
pub use static_example::PROGRAM_NAME;

/// Basic function example
pub fn add_numbers(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add_numbers(2, 2);
        assert_eq!(result, 4);
    }
}
