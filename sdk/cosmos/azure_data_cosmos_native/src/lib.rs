// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(clippy::not_unsafe_ptr_arg_deref, reason = "We do that a lot here.")]

use std::ffi::{c_char, CStr};

#[macro_use]
pub mod string;
#[macro_use]
pub mod context;
pub mod blocking;
pub mod clients;
pub mod error;
pub mod runtime;

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

/// Installs tracing listeners that output to stdout/stderr based on the `COSMOS_LOG` environment variable.
///
/// Just calling this function isn't sufficient to get logging output. You must also set the `COSMOS_LOG` environment variable
/// to specify the desired log level and targets. See <https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html>
/// for details on the syntax for this variable.
#[no_mangle]
#[cfg(feature = "tracing")]
pub extern "C" fn cosmos_enable_tracing() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("COSMOS_LOG"))
        .init();
}
