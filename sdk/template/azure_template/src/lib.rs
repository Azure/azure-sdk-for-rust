//! Azure Template crate demonstrating Rust patterns

pub use azure_template_core::NumericCore;
pub use azure_template_macros::numeric_operation;

pub mod data; // Enums and variants
pub mod ffi;
pub mod numeric;
pub mod module_example; // Traits and generics // FFI bindings

pub use data::{DataProcessor, Status, StatusVariant};
pub use numeric::{IntOrFloat, NumericOps, Vector3};

/// Basic function example
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
