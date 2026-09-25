// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI wrapper for the Azure Cosmos DB driver crate
//! ([`azure_data_cosmos_driver`]).
//!
//! This crate exposes a schema-agnostic completion-queue-style FFI suitable
//! for cross-language SDK reuse (.NET, Java, Go, Python, native C/C++). The
//! design uses a completion queue and flat C-compatible data structures.
//!
//! [`azure_data_cosmos_driver`]: https://docs.rs/azure_data_cosmos_driver

// We routinely dereference C-supplied pointers at the FFI boundary; that is
// the entire point of the crate.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::{c_char, CStr};

#[macro_use]
pub mod string;
pub mod account_ref;
pub mod bytes;
pub mod completion;
pub mod container_ref;
pub mod credential;
pub mod database_ref;
pub mod diagnostics;
pub mod driver;
pub mod driver_options;
pub mod error;
pub mod feed_range;
pub mod op_request;
pub mod partition_key;
pub mod response_header;
pub mod runtime;
pub mod runtime_builder;
pub(crate) mod safety;
pub mod submit;

// We want this value to be present as a string in the compiled binary so that
// build provenance can be recovered from a stripped library. Exposing it as a
// non-mangled static prevents the compiler from optimizing it away. The symbol
// carries the `COSMOS_` prefix like every other export so it cannot collide
// with a host's own symbols when this crate is linked as a static library.
//
// cbindgen:ignore
#[no_mangle]
pub static COSMOS_BUILD_IDENTIFIER: &CStr = c_str!(env!("BUILD_IDENTIFIER"));

const VERSION: &CStr = c_str!(env!("CARGO_PKG_VERSION"));
const ABI_VERSION_MAJOR: u16 = 1;
const ABI_VERSION_MINOR: u16 = 1;

/// Returns a constant, NUL-terminated UTF-8 string containing the version of
/// the `azurecosmosdriver` library. The returned pointer is statically
/// allocated; callers **must not** free it.
#[no_mangle]
pub extern "C" fn cosmos_version() -> *const c_char {
    VERSION.as_ptr()
}

/// Returns the native ABI version as `(major << 16) | minor`.
///
/// Consumers must require an equal major version and a minor version greater
/// than or equal to the minimum they use.
#[no_mangle]
pub extern "C" fn cosmos_abi_version() -> u32 {
    u32::from(ABI_VERSION_MAJOR) << 16 | u32::from(ABI_VERSION_MINOR)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abi_version_is_major_minor_encoded() {
        assert_eq!(cosmos_abi_version(), 0x0001_0001);
    }
}
