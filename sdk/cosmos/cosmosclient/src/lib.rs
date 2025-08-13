// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::ffi::{c_char, CStr, CString};

#[macro_use]
mod macros;

#[no_mangle] // Necessary to prevent the compiler from stripping it when optimizing
/// cbindgen:ignore
pub static BUILD_IDENTIFIER: &CStr = c_str!(env!("BUILD_IDENTIFIER"));

const VERSION: &CStr = c_str!(env!("CARGO_PKG_VERSION"));

#[no_mangle]
/// Returns a constant C string containing the version of the Cosmos Client library.
pub extern "C" fn cosmosclient_version() -> *const c_char {
    VERSION.as_ptr()
}
