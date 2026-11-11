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
pub mod cursor;
pub mod cursor_request;
pub mod database_ref;
pub mod diagnostics;
pub mod driver;
pub mod driver_options;
pub mod error;
pub mod fault_injection;
pub mod feed_range;
pub mod op_request;
pub mod options_snapshot;
pub mod partition_key;
pub mod read_many;
pub mod response_header;
pub mod runtime;
pub mod runtime_builder;
pub(crate) mod safety;
pub mod submit;
#[cfg(feature = "test-abi")]
mod test_abi;

// We want this value to be present as a string in the compiled binary so that
// build provenance can be recovered from a stripped library. Exposing it as a
// non-mangled static prevents the compiler from optimizing it away. The symbol
// carries the `COSMOS_` prefix like every other export so it cannot collide
// with a host's own symbols when this crate is linked as a static library.
//
// cbindgen:ignore
#[no_mangle]
pub static COSMOS_BUILD_IDENTIFIER: &CStr = c_str!(env!("BUILD_IDENTIFIER"));

// This crate is the final native artifact, so it owns the global allocator
// choice; library crates such as `azure_data_cosmos_driver` leave it to their
// consumers. mimalloc's `override` feature stays off, so host `malloc`/`free`
// remain on the host allocator. Every pointer this library returns is released
// through a `cosmos_*_free` export, so the two allocators never free each
// other's memory.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

const VERSION: &CStr = c_str!(env!("CARGO_PKG_VERSION"));

/// Returns a constant, NUL-terminated UTF-8 string containing the version of
/// the `azurecosmosdriver` library. The returned pointer is statically
/// allocated; callers **must not** free it.
#[no_mangle]
pub extern "C" fn cosmos_version() -> *const c_char {
    VERSION.as_ptr()
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "mimalloc")]
    #[test]
    fn rust_allocations_use_mimalloc() {
        use std::ffi::{c_int, c_void};

        // Declared locally because mimalloc's sys crate only exposes them
        // behind its `extended` feature.
        extern "C" {
            fn mi_is_in_heap_region(p: *const c_void) -> bool;
            fn mi_version() -> c_int;
        }

        let small = Box::new([0u8; 64]);
        let large = vec![0u8; 1 << 20];
        unsafe {
            assert!(mi_version() > 0);
            assert!(mi_is_in_heap_region(small.as_ptr().cast()));
            assert!(mi_is_in_heap_region(large.as_ptr().cast()));
        }
    }
}
