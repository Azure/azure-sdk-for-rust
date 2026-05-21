// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI bindings for [`azure_data_cosmos_driver`].
//!
//! See `docs/NATIVE_WRAPPER_SPEC.md` in the driver crate for the full design.
//!
//! All `extern "C"` functions exported here use the `cosmos_` prefix and follow
//! the conventions described in §3 of the spec:
//!
//! - Fallible calls return [`error::CosmosErrorCode`] and write outputs through
//!   `out_*` pointers.
//! - Allocated outputs (handles, byte buffers, strings) must be released via
//!   the matching `cosmos_*_free` function.
//! - `options == NULL` always means "use defaults".

#![allow(
    clippy::not_unsafe_ptr_arg_deref,
    reason = "Raw pointers are pervasive at the FFI boundary; safety contracts are documented per-function."
)]
#![allow(
    clippy::missing_safety_doc,
    reason = "FFI safety contracts are documented in the C header and in `docs/NATIVE_WRAPPER_SPEC.md`."
)]

use std::ffi::{c_char, CStr};

#[macro_use]
pub mod string;
pub mod bytes;
#[macro_use]
pub mod context;
pub mod error;
pub mod handles;
pub mod options;
pub mod runtime;

// Re-export the most common handle types at the crate root for convenience.
pub use context::{CallContext, CallContextOptions};
pub use error::{CosmosError, CosmosErrorCode};
pub use runtime::RuntimeContext;

/// Build identifier baked into the binary. Exposed as a non-mangled static so
/// the linker cannot strip it; tooling can grep the produced library for the
/// `$Id: …$` marker to identify exact builds.
///
/// cbindgen:ignore
#[no_mangle]
pub static BUILD_IDENTIFIER: &CStr = c_str!(env!("BUILD_IDENTIFIER"));

const VERSION: &CStr = c_str!(env!("CARGO_PKG_VERSION"));

/// Returns a static C string containing the version of `azurecosmosdriver`.
///
/// The returned pointer is valid for the lifetime of the loaded library and
/// must NOT be passed to [`crate::string::cosmos_string_free`].
#[no_mangle]
pub extern "C" fn cosmos_version() -> *const c_char {
    VERSION.as_ptr()
}

/// Installs tracing listeners writing to stderr, filtered by the `COSMOS_LOG`
/// environment variable. See
/// <https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html>
/// for the filter syntax. Has no effect unless the `tracing` feature is enabled.
#[cfg(feature = "tracing")]
#[no_mangle]
pub extern "C" fn cosmos_enable_tracing() {
    use tracing_subscriber::EnvFilter;

    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_env("COSMOS_LOG").unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_thread_ids(true)
        .with_thread_names(true)
        .try_init();
}

/// Helper that converts a non-null `*const T` to a `&T`, returning an
/// `InvalidArgument` error otherwise.
pub(crate) fn unwrap_required_ptr<'a, T>(
    ptr: *const T,
    msg: &'static CStr,
) -> Result<&'a T, error::Error> {
    if ptr.is_null() {
        Err(error::Error::new(
            error::CosmosErrorCode::InvalidArgument,
            msg,
        ))
    } else {
        Ok(unsafe { &*ptr })
    }
}
