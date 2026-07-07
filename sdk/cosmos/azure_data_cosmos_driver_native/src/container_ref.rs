// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_container_ref_t` — wraps the driver's
//! [`azure_data_cosmos_driver::models::ContainerReference`].
//!
//! ## How container references are obtained
//!
//! The driver's `ContainerReference::new` is `pub(crate)`-only because
//! a fully-resolved container needs both the name-based and RID-based
//! identifiers + the partition-key definition. Resolution happens via
//! `CosmosDriver::resolve_container_by_name(db, container).await`,
//! which the wrapper exposes through two entry points:
//!
//! - [`cosmos_driver_resolve_container_blocking`] — runs the async
//!   resolution on the wrapper's Tokio runtime via `block_on`.
//!   Suitable for startup-time initialization.
//! - `cosmos_driver_resolve_container_submit` — fires the resolution
//!   asynchronously through the standard submit pipeline; the result
//!   arrives on the supplied completion queue as a degenerate response
//!   from which `cosmos_response_take_container` extracts the
//!   container handle.
//!
//! Once obtained, `cosmos_container_ref_t` is the value-type currency
//! the container/item-scope operation factories (`cosmos_operation_read_item`,
//! `cosmos_operation_create_item`, etc.) consume.
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] section 4.3 + section 4.4 + section 4.7.
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::ffi::{c_char, CStr};
use std::sync::Arc;

use azure_data_cosmos_driver::models::ContainerReference as DriverContainerReference;

use crate::driver::DriverHandle;
use crate::error::{CosmosErrorCode, CosmosErrorHandle};
use crate::runtime::RuntimeContext;

/// The C ABI handle for a container reference (`cosmos_container_ref_t`).
///
/// Wraps the driver's container reference; the C side holds it as an opaque
/// handle and releases it with `cosmos_container_ref_free`.
pub struct ContainerRefHandle {
    pub(crate) inner: DriverContainerReference,
}

impl ContainerRefHandle {
    pub(crate) fn into_raw(inner: DriverContainerReference) -> *mut Self {
        Box::into_raw(Box::new(ContainerRefHandle { inner }))
    }

    /// Borrows the handle for the duration of an FFI call without taking
    /// ownership. Returns `None` for a NULL pointer.
    pub(crate) fn from_ptr<'a>(p: *const ContainerRefHandle) -> Option<&'a ContainerRefHandle> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and is
        // not freed for the duration of the borrow.
        Some(unsafe { &*p })
    }

    fn drop_raw(p: *mut ContainerRefHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and has
        // not already been freed.
        unsafe {
            drop(Box::from_raw(p));
        }
    }
}

fn try_cstr_to_str<'a>(p: *const c_char) -> Result<&'a str, CosmosErrorCode> {
    if p.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: caller contract on every entry point.
    let cstr = unsafe { CStr::from_ptr(p) };
    cstr.to_str()
        .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: lifecycle
// ─────────────────────────────────────────────────────────────────────────────

/// Frees a container-reference handle. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_container_ref_free(container: *mut ContainerRefHandle) {
    if container.is_null() {
        return;
    }
    tracing::trace!(?container, "freeing cosmos_container_ref_t");
    ContainerRefHandle::drop_raw(container);
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: resolve_container — blocking convenience
// ─────────────────────────────────────────────────────────────────────────────

/// Synchronously resolves a container reference. Bridges
/// [`azure_data_cosmos_driver::driver::CosmosDriver::resolve_container_by_name`]
/// through the wrapper's Tokio runtime via `block_on`.
///
/// Touches the network on cache miss (reads container metadata from
/// the gateway). On cache hit returns immediately without I/O.
///
/// # Parameters
///
/// - `runtime` — non-NULL.
/// - `driver` — non-NULL; the driver whose container cache to consult.
/// - `database_id` — NUL-terminated UTF-8.
/// - `container_id` — NUL-terminated UTF-8.
/// - `out_container` — non-NULL slot for the resolved handle.
/// - `out_error` — optional rich error on failure. NULL silently drops.
///
/// # Returns
///
/// `SUCCESS` (0) on success, the standard coarse codes on failure
/// derived from the driver-side error.
#[no_mangle]
pub extern "C" fn cosmos_driver_resolve_container_blocking(
    runtime: *const RuntimeContext,
    driver: *const DriverHandle,
    database_id: *const c_char,
    container_id: *const c_char,
    out_container: *mut *mut ContainerRefHandle,
    out_error: *mut *mut CosmosErrorHandle,
) -> i32 {
    if out_container.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(runtime_inner) = RuntimeContext::inner_arc(runtime) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let Some(driver_inner) = DriverHandle::inner_arc(driver) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let db_id = match try_cstr_to_str(database_id) {
        Ok(s) => s.to_owned(),
        Err(code) => return code.as_i32(),
    };
    let container_id = match try_cstr_to_str(container_id) {
        Ok(s) => s.to_owned(),
        Err(code) => return code.as_i32(),
    };

    let driver_arc = Arc::clone(&driver_inner.inner);
    let result = runtime_inner.tokio.block_on(async move {
        driver_arc
            .resolve_container_by_name(&db_id, &container_id)
            .await
    });

    match result {
        Ok(container_ref) => {
            let handle = ContainerRefHandle::into_raw(container_ref);
            // SAFETY: caller guarantees `out_container` is writable.
            unsafe {
                *out_container = handle;
            }
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        }
        Err(err) => {
            let coarse = CosmosErrorCode::from_driver_error(&err);
            if !out_error.is_null() {
                // SAFETY: caller guarantees `out_error` is writable.
                unsafe {
                    *out_error = CosmosErrorHandle::into_raw(err);
                }
            }
            coarse.as_i32()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn free_handles_null() {
        cosmos_container_ref_free(ptr::null_mut());
    }

    #[test]
    fn resolve_rejects_null_arguments() {
        let mut out: *mut ContainerRefHandle = ptr::null_mut();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();

        // NULL runtime / driver / out_container all rejected pre-flight.
        assert_eq!(
            cosmos_driver_resolve_container_blocking(
                ptr::null(),
                ptr::null(),
                std::ffi::CString::new("db").unwrap().as_ptr(),
                std::ffi::CString::new("c").unwrap().as_ptr(),
                &mut out,
                &mut err,
            ),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert!(out.is_null());
        assert!(err.is_null());
    }
}
