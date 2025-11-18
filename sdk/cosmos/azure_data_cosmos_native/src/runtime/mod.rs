//! This module provides runtime abstractions and implementations for different async runtimes.
//! When compiling the C library, a feature is used to select which runtime implementation to include.
//! Currently, only the Tokio runtime is supported.
//!
//! All callers to the Cosmos DB Client API must first create a RuntimeContext object appropriate for their chosen runtime
//! using the [`cosmos_runtime_context_create`] function.
//! This object must then be passed to all other API functions, within a `CallContext` structure.

#[cfg(feature = "tokio")]
mod tokio;

#[cfg(feature = "tokio")]
pub use tokio::*;

use crate::error::CosmosError;

#[repr(C)]
pub struct RuntimeOptions {
    // Reserved for future use.
}

/// Creates a new [`RuntimeContext`] for Cosmos DB Client API calls.
///
/// This must be called before any other Cosmos DB Client API functions are used,
/// and the returned pointer must be passed within a `CallContext` structure to those functions.
///
/// When the `RuntimeContext` is no longer needed, it should be freed using the
/// [`cosmos_runtime_context_free`] function. However, if the program is terminating,
/// it is not strictly necessary to free it.
///
/// If this function fails, it will return a null pointer, and the `out_error` parameter
/// (if not null) will be set to contain the error details.
///
/// The error will contain a dynamically-allocated [`CosmosError::detail`] string that must be
/// freed by the caller using the [`cosmos_string_free`](crate::string::cosmos_string_free) function.
///
/// # Arguments
///
/// * `options` - Pointer to [`RuntimeOptions`] for runtime configuration, may be null.
/// * `out_error` - Output parameter that will receive error details if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_runtime_context_create(
    options: *const RuntimeOptions,
    out_error: *mut CosmosError,
) -> *mut RuntimeContext {
    let options = if options.is_null() {
        None
    } else {
        Some(unsafe { &*options })
    };
    let c = match RuntimeContext::new(options) {
        Ok(c) => c,
        Err(e) => {
            unsafe {
                if !out_error.is_null() {
                    *out_error = e.into_ffi(true);
                }
            }
            return std::ptr::null_mut();
        }
    };
    Box::into_raw(Box::new(c))
}

/// Destroys a [`RuntimeContext`] created by [`cosmos_runtime_context_create`].
/// This frees the memory associated with the `RuntimeContext`.
#[no_mangle]
pub extern "C" fn cosmos_runtime_context_free(ctx: *mut RuntimeContext) {
    if !ctx.is_null() {
        tracing::trace!(?ctx, "freeing runtime context");
        unsafe { drop(Box::from_raw(ctx)) }
    }
}
