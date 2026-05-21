// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response handle and accessors (Phase 6 — initial set).
//!
//! `cosmos_response` wraps `azure_data_cosmos_driver::models::CosmosResponse`.
//! Accessors expose the most commonly needed fields: HTTP status code, RU
//! charge, activity id, body bytes, and (later) full header iteration and
//! diagnostics.

use std::ffi::{c_char, CString};

use azure_data_cosmos_driver::models::CosmosResponse;

use crate::bytes::{CosmosBytes, CosmosBytesView};
use crate::error::{messages, CosmosErrorCode};
use crate::unwrap_required_ptr;

/// Opaque response handle.
///
/// Borrowed accessors (`*_status_code`, `*_request_charge`, `*_body`) are
/// valid only while the response handle is alive. `*_into_body` consumes the
/// response and transfers body ownership to the caller.
///
/// cbindgen:ignore
#[allow(non_camel_case_types)]
pub struct cosmos_response(pub(crate) Option<CosmosResponse>);

impl cosmos_response {
    /// Wraps a `CosmosResponse` for FFI. The returned struct is meant to be
    /// `Box::new`'d by the caller (the `IntoRaw` impl for `Box<T>` handles
    /// the rest).
    pub(crate) fn new_inner(r: CosmosResponse) -> cosmos_response {
        cosmos_response(Some(r))
    }

    fn get(&self) -> Option<&CosmosResponse> {
        self.0.as_ref()
    }
}

/// Returns the HTTP status code from the response. Returns 0 if the response
/// has been consumed or if `r` is null.
#[no_mangle]
pub extern "C" fn cosmos_response_status_code(r: *const cosmos_response) -> u16 {
    let Ok(r) = unwrap_required_ptr(r, messages::INVALID_HANDLE) else {
        return 0;
    };
    r.get()
        .map(|resp| u16::from(resp.status().status_code()))
        .unwrap_or(0)
}

/// Returns the request charge (RU/s consumed) for the response. Returns 0.0
/// if the value is absent, the response has been consumed, or `r` is null.
#[no_mangle]
pub extern "C" fn cosmos_response_request_charge(r: *const cosmos_response) -> f64 {
    let Ok(r) = unwrap_required_ptr(r, messages::INVALID_HANDLE) else {
        return 0.0;
    };
    r.get()
        .and_then(|resp| resp.headers().request_charge.as_ref().map(|rc| rc.value()))
        .unwrap_or(0.0)
}

/// Returns the activity id as a heap-allocated C string, or null if absent.
///
/// On success, the caller MUST release the returned pointer via
/// [`crate::string::cosmos_string_free`].
///
/// # Safety
/// `r` must be null or a valid response pointer.
#[no_mangle]
pub unsafe extern "C" fn cosmos_response_activity_id(r: *const cosmos_response) -> *const c_char {
    let Ok(r) = unwrap_required_ptr(r, messages::INVALID_HANDLE) else {
        return std::ptr::null();
    };
    r.get()
        .and_then(|resp| resp.headers().activity_id.as_ref())
        .and_then(|aid| CString::new(aid.as_str()).ok())
        .map(|c| c.into_raw() as *const c_char)
        .unwrap_or(std::ptr::null())
}

/// Returns the ETag header, or null if absent. Caller must free the returned
/// string via [`crate::string::cosmos_string_free`].
#[no_mangle]
pub unsafe extern "C" fn cosmos_response_etag(r: *const cosmos_response) -> *const c_char {
    let Ok(r) = unwrap_required_ptr(r, messages::INVALID_HANDLE) else {
        return std::ptr::null();
    };
    r.get()
        .and_then(|resp| resp.headers().etag.as_ref())
        .and_then(|e| CString::new(e.as_str()).ok())
        .map(|c| c.into_raw() as *const c_char)
        .unwrap_or(std::ptr::null())
}

/// Returns the continuation token, or null if absent. Caller must free.
#[no_mangle]
pub unsafe extern "C" fn cosmos_response_continuation_token(
    r: *const cosmos_response,
) -> *const c_char {
    let Ok(r) = unwrap_required_ptr(r, messages::INVALID_HANDLE) else {
        return std::ptr::null();
    };
    r.get()
        .and_then(|resp| resp.headers().continuation.as_ref())
        .and_then(|c| CString::new(c.as_str()).ok())
        .map(|c| c.into_raw() as *const c_char)
        .unwrap_or(std::ptr::null())
}

/// Borrowed view over the response body. Valid only until the response handle
/// is freed. Returns an empty/null view if the body has been consumed.
///
/// For multi-part response bodies (e.g. paged query results), the driver
/// concatenates the parts into a single contiguous buffer; if concatenation
/// fails this returns an empty view.
#[no_mangle]
pub extern "C" fn cosmos_response_body(r: *const cosmos_response) -> CosmosBytesView {
    let default = CosmosBytesView::default();
    let Ok(r) = unwrap_required_ptr(r, messages::INVALID_HANDLE) else {
        return default;
    };
    let Some(resp) = r.get() else { return default };

    // ResponseBody::body() may be a single-part or multi-part body. The cheap
    // common case is single-part; for multi-part we'd need to allocate, which
    // doesn't fit a borrowing accessor. Callers that need multi-part should
    // use `cosmos_response_into_body`.
    match resp.body() {
        azure_data_cosmos_driver::models::ResponseBody::Bytes(b) => CosmosBytesView {
            data: b.as_ptr(),
            len: b.len(),
        },
        _ => default,
    }
}

/// Consumes the response and returns its body as an owned byte buffer. The
/// response handle is left in a "consumed" state; subsequent accessor calls
/// return zero/null. The caller still must release the handle via
/// [`cosmos_response_free`] AND release the returned buffer via
/// [`crate::bytes::cosmos_bytes_free`].
///
/// # Safety
/// `r` must be non-null and `out_body` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn cosmos_response_into_body(
    r: *mut cosmos_response,
    out_body: *mut CosmosBytes,
) -> CosmosErrorCode {
    if r.is_null() || out_body.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }
    let slot = &mut *r;
    match slot.0.take() {
        None => CosmosErrorCode::ResponseConsumed,
        Some(resp) => {
            let body = resp.into_body();
            // `body.single()` concatenates parts (or errors out if the body
            // was an empty multipart). For now we surface concatenation
            // failures as an empty buffer.
            let bytes = body.single().map(|b| b.to_vec()).unwrap_or_default();
            *out_body = CosmosBytes::from_vec(bytes);
            CosmosErrorCode::Success
        }
    }
}

/// Releases a response handle.
///
/// # Safety
/// `r` must be null or a pointer returned by `cosmos_driver_execute`.
#[no_mangle]
pub unsafe extern "C" fn cosmos_response_free(r: *mut cosmos_response) {
    if !r.is_null() {
        drop(Box::from_raw(r));
    }
}
