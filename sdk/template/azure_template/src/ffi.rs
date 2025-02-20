// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Foreign Function Interface (FFI) module
//!
//! This module provides FFI declarations and utilities for interacting with
//! C-compatible code. It demonstrates proper FFI integration patterns and
//! safety handling in Rust.

// Compute the absolute value of an integer using C's stdlib
//
// This function demonstrates FFI integration by wrapping C's abs function.
// Note that this is unsafe as it calls external C code.
//
// # Safety
//
// This function is unsafe because it calls into C code. The caller must ensure:
// * The input value is a valid i32
// * The resulting absolute value can be represented as an i32
//
// # Examples
//
// ```
// # use azure_template::ffi::abs;
// unsafe {
//     assert_eq!(abs(-42), 42);
//     assert_eq!(abs(0), 0);
// }
// ```
extern "C" {
    pub fn abs(input: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        unsafe {
            assert_eq!(abs(-42), 42);
            assert_eq!(abs(42), 42);
            assert_eq!(abs(0), 0);
        }
    }
}
