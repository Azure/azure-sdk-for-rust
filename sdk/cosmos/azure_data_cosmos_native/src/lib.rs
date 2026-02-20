// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(clippy::not_unsafe_ptr_arg_deref, reason = "We do that a lot here.")]

use std::ffi::{c_char, CStr};

#[macro_use]
pub mod string;
#[macro_use]
pub mod context;
pub mod clients;
pub mod error;
pub mod options;
pub mod runtime;

pub use clients::*;

/// Helper function to safely unwrap a required pointer, returning an error if it's null.
///
/// # Arguments
/// * `ptr` - The pointer to check and dereference.
/// * `msg` - A static error message to use if the pointer is null.
///
/// # Returns
/// * `Ok(&T)` if the pointer is non-null.
/// * `Err(Error)` with code `InvalidArgument` if the pointer is null.
///
/// # Safety
/// This function assumes that if the pointer is non-null, it points to a valid `T`.
/// The caller must ensure the pointer was created properly and has not been freed.
pub fn unwrap_required_ptr<'a, T>(
    ptr: *const T,
    msg: &'static CStr,
) -> Result<&'a T, error::Error> {
    if ptr.is_null() {
        Err(error::Error::new(
            error::CosmosErrorCode::InvalidArgument,
            msg,
        ))
    } else {
        tracing::trace!(
            ?ptr,
            type_name = std::any::type_name::<T>(),
            "unwrapped pointer"
        );
        Ok(unsafe { &*ptr })
    }
}

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
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}
