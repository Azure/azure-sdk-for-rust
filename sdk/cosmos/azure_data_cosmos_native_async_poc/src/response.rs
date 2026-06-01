// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Successful (or service-error) response handle exposed to the host.
//!
//! A response carries the HTTP status code and the raw response body bytes.
//! The spike does not surface headers, diagnostics, RU charge, etag,
//! activity-id, etc. — that surface is the production crate's job and is
//! orthogonal to the async-FFI feasibility question.
//!
//! ## Ownership
//!
//! `*out_response` from [`cosmos_cq_wait`](crate::cq::cosmos_cq_wait) hands
//! the host a `*mut CosmosResponseHandle`. The host **must** free it via
//! [`cosmos_response_free`]; the spike-side allocator is the global Rust
//! allocator, which on Windows is incompatible with the C runtime's `free`.
//!
//! Body bytes are exposed via [`cosmos_response_body`], which writes a
//! pointer into the Rust-owned `Vec<u8>` and the byte length. The pointer
//! is valid until the next call to `cosmos_response_free` on the same
//! handle. Hosts that need to retain the bytes longer must copy them.

use std::ffi::c_void;

use crate::error::CosmosStatusCode;
use crate::ffi_guard;

/// Opaque response handle.
pub struct CosmosResponseHandle {
    pub(crate) status: u16,
    pub(crate) body: Vec<u8>,
}

/// Returns the HTTP status code (or 0 for non-service errors).
///
/// # Safety
///
/// `response` must be a non-null pointer returned by `cosmos_cq_wait`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_response_status(response: *const CosmosResponseHandle) -> u16 {
    if response.is_null() {
        return 0;
    }
    ffi_guard!(0, { (*response).status })
}

/// Writes the body pointer (into Rust-owned memory) and length into the
/// supplied out-params. Pointer validity ends at `cosmos_response_free`.
///
/// Returns `Ok` on success, `InvalidArg` on null arguments.
///
/// # Safety
///
/// `response`, `out_ptr`, and `out_len` must all be non-null and point to
/// caller-owned memory.
#[no_mangle]
pub unsafe extern "C" fn cosmos_response_body(
    response: *const CosmosResponseHandle,
    out_ptr: *mut *const c_void,
    out_len: *mut usize,
) -> i32 {
    ffi_guard!(CosmosStatusCode::InternalError as i32, {
        if response.is_null() || out_ptr.is_null() || out_len.is_null() {
            return CosmosStatusCode::InvalidArg as i32;
        }
        let body = &(*response).body;
        *out_ptr = body.as_ptr() as *const c_void;
        *out_len = body.len();
        CosmosStatusCode::Ok as i32
    })
}

/// Releases the response handle and the body buffer it owns.
///
/// # Safety
///
/// `response` must have been written by `cosmos_cq_wait` and not yet
/// freed. `null` is a no-op.
#[no_mangle]
pub unsafe extern "C" fn cosmos_response_free(response: *mut CosmosResponseHandle) {
    if response.is_null() {
        return;
    }
    ffi_guard!((), {
        let _ = Box::from_raw(response);
    })
}
