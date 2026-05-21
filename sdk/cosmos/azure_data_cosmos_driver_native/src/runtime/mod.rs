// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Runtime context — owns the async executor and the shared
//! [`azure_data_cosmos_driver::CosmosDriverRuntime`].
//!
//! Currently only Tokio is supported. The runtime selection is an
//! implementation detail; the C ABI never names it.

#[cfg(feature = "tokio")]
mod tokio;

#[cfg(feature = "tokio")]
pub use tokio::RuntimeContext;

use crate::error::CosmosError;

/// Caller-supplied runtime configuration.
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct RuntimeOptions {
    /// Number of worker threads. `0` means "auto-detect" (Tokio default).
    pub worker_threads: u32,
}

/// Creates a new [`RuntimeContext`]. On failure, returns null and (if
/// `out_error` is non-null) writes the error there. The error's detail string,
/// if any, must be released by the caller via
/// [`crate::string::cosmos_string_free`].
#[no_mangle]
pub extern "C" fn cosmos_runtime_create(
    options: *const RuntimeOptions,
    out_error: *mut CosmosError,
) -> *mut RuntimeContext {
    let opts = if options.is_null() {
        RuntimeOptions::default()
    } else {
        unsafe { *options }
    };
    match RuntimeContext::new(opts) {
        Ok(rt) => Box::into_raw(Box::new(rt)),
        Err(e) => {
            if !out_error.is_null() {
                unsafe { *out_error = e.into_ffi(true) };
            }
            std::ptr::null_mut()
        }
    }
}

/// Releases a [`RuntimeContext`] created by [`cosmos_runtime_create`].
///
/// # Safety
/// `rt` must be either null or a pointer previously returned by
/// `cosmos_runtime_create`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_runtime_free(rt: *mut RuntimeContext) {
    if !rt.is_null() {
        drop(Box::from_raw(rt));
    }
}
