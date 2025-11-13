// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(clippy::not_unsafe_ptr_arg_deref, reason = "We do that a lot here.")]

use std::ffi::{c_char, CStr};

#[macro_use]
mod macros;

pub mod blocking;
pub mod clients;
pub mod error;
pub mod string;

pub use clients::*;

// We just want this value to be present as a string in the compiled binary.
// But in order to prevent the compiler from optimizing it away, we expose it as a non-mangled static variable.
/// cbindgen:ignore
#[no_mangle]
pub static BUILD_IDENTIFIER: &CStr = c_str!(env!("BUILD_IDENTIFIER"));

const VERSION: &CStr = c_str!(env!("CARGO_PKG_VERSION"));

/// Returns a constant C string containing the version of the Cosmos Client library.
#[no_mangle]
pub extern "C" fn cosmos_version() -> *const c_char {
    VERSION.as_ptr()
}
