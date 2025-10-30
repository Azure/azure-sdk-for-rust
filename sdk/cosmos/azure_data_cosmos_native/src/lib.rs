// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::ffi::{c_char, CStr};

#[macro_use]
mod macros;

/// cbindgen:ignore
#[no_mangle] // Necessary to prevent the compiler from stripping it when optimizing
pub static BUILD_IDENTIFIER: &CStr = c_str!(env!("BUILD_IDENTIFIER"));

const VERSION: &CStr = c_str!(env!("CARGO_PKG_VERSION"));

/// Returns a constant C string containing the version of the Cosmos Client library.
#[no_mangle]
pub extern "C" fn cosmosclient_version() -> *const c_char {
    VERSION.as_ptr()
}
