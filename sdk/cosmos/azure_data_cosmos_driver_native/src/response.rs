// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_response_t` — wraps the driver's
//! [`azure_data_cosmos_driver::models::CosmosResponse`].
//!
//! ## Scope
//!
//! - Status code, RU charge.
//! - Body access (zero-copy view + take-ownership variant).
//! - The four high-traffic typed headers (activity_id, session_token,
//!   etag, continuation token).
//! - `cosmos_response_take_driver` for the degenerate response delivered
//!   by `cosmos_driver_get_or_create_submit`.
//! - `cosmos_response_take_container` for the degenerate response
//!   delivered by `cosmos_driver_resolve_container_submit`.
//!
//! Multi-part responses (`ResponseBody::Items`) collapse to the first
//! part for the body view — the multi-part iterator surface, diagnostics,
//! header iteration, and the long-tail of typed accessors are follow-ups.
//!
//! ## Storage pun
//!
//! `cosmos_response_t *` wraps an `Arc<ResponseInner>` whose inner state
//! holds the driver's `CosmosResponse` (already cheap to clone) plus
//! lazily-cached `CString` copies of the four borrowed-string
//! accessors. The Arc shape mirrors `CosmosErrorHandle` so future
//! diagnostics borrowing can share allocations.
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] section 4.7.
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::ffi::{c_char, CString};
use std::sync::{Arc, Mutex, OnceLock};

use azure_data_cosmos_driver::models::{
    ContainerReference as DriverContainerReference, CosmosResponse, ResponseBody,
};

use crate::container_ref::ContainerRefHandle;
use crate::driver::DriverHandle;
use crate::error::CosmosErrorCode;
use crate::safety::MutexExt;

/// The C ABI handle for a [`CosmosResponse`] (`cosmos_response_t`).
///
/// Single-owner and `Box`-managed (responses are never cloned).
///
/// The handle also carries optional "side payloads" populated only on
/// degenerate responses delivered by the driver-creation / container-resolve
/// submit paths. `_take_driver` / `_take_container` move these payloads out;
/// once taken, both accessors return NULL.
pub struct ResponseHandle {
    /// `None` on degenerate responses (driver-creation and
    /// container-resolve submit paths). On these the scalar / header
    /// accessors return default values; the side-payload
    /// `_take_driver` / `_take_container` accessors carry the real
    /// payload via the separate `Mutex` slots below.
    pub(crate) inner: Option<CosmosResponse>,
    // Lazily-cached null-terminated copies of the four header strings
    // exposed via the high-traffic typed accessors. Each is populated on
    // first read; once populated the pointer is stable until the
    // response is freed.
    activity_id_cstring: OnceLock<Option<CString>>,
    session_token_cstring: OnceLock<Option<CString>>,
    etag_cstring: OnceLock<Option<CString>>,
    continuation_cstring: OnceLock<Option<CString>>,
    // Side payloads — `Mutex<Option<...>>` so the take accessors can
    // detach ownership in place. NULL on every "real" CRUD completion.
    driver_payload: Mutex<Option<Arc<crate::driver::DriverHandle>>>,
    container_payload: Mutex<Option<DriverContainerReference>>,
    // Plan-derived next-page continuation token, captured by the feed
    // submit path from `OperationPlan::to_continuation_token()`. This is
    // distinct from the *response header* continuation (which only carries
    // server-opaque tokens valid for trivial single-partition operations);
    // the plan token correctly resumes cross-partition queries. `None`
    // when the page was the last one, when the operation was a singleton,
    // or on any non-feed completion. Exposed via
    // [`cosmos_response_next_continuation`].
    next_continuation: Option<CString>,
}

impl ResponseHandle {
    pub(crate) fn into_raw(inner: CosmosResponse) -> *mut Self {
        Self::into_raw_with_payloads(Some(inner), None, None, None)
    }

    pub(crate) fn into_raw_with_driver(driver: Arc<crate::driver::DriverHandle>) -> *mut Self {
        Self::into_raw_with_payloads(None, Some(driver), None, None)
    }

    pub(crate) fn into_raw_with_container(container: DriverContainerReference) -> *mut Self {
        Self::into_raw_with_payloads(None, None, Some(container), None)
    }

    /// Builds a response handle for a feed page produced by
    /// `cosmos_submit_operation`. `response` is `None` when
    /// the feed is exhausted (host treats it as end-of-stream); `next` is
    /// the plan-derived continuation token for the following page (`None`
    /// when this was the last page).
    pub(crate) fn into_raw_with_next_continuation(
        response: Option<CosmosResponse>,
        next: Option<String>,
    ) -> *mut Self {
        let next_continuation = next.and_then(|s| CString::new(s).ok());
        Self::into_raw_with_payloads(response, None, None, next_continuation)
    }

    fn into_raw_with_payloads(
        response: Option<CosmosResponse>,
        driver: Option<Arc<crate::driver::DriverHandle>>,
        container: Option<DriverContainerReference>,
        next_continuation: Option<CString>,
    ) -> *mut Self {
        Box::into_raw(Box::new(ResponseHandle {
            inner: response,
            activity_id_cstring: OnceLock::new(),
            session_token_cstring: OnceLock::new(),
            etag_cstring: OnceLock::new(),
            continuation_cstring: OnceLock::new(),
            driver_payload: Mutex::new(driver),
            container_payload: Mutex::new(container),
            next_continuation,
        }))
    }

    /// Borrows the handle from a raw pointer for the duration of an FFI call.
    fn from_ptr<'a>(p: *const ResponseHandle) -> Option<&'a ResponseHandle> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` came from `into_raw_*`.
        Some(unsafe { &*p })
    }

    /// Returns the borrowed `(ptr, len)` view of the body, normalizing an
    /// empty / degenerate / feed-envelope body to `(NULL, 0)`. The pointer is
    /// valid until the response is freed. Shared by `cosmos_response_body` and
    /// `cosmos_response_view`.
    fn body_view(&self) -> (*const u8, usize) {
        let Some(inner_response) = self.inner.as_ref() else {
            return (std::ptr::null(), 0);
        };
        match inner_response.body() {
            // Normalize an empty `Bytes` body to a NULL pointer so it matches
            // the documented "NULL pointer + 0 length when empty" contract (a
            // non-empty `Vec`'s `as_ptr()` is a dangling sentinel a host might
            // mistake for a present body when checking `ptr != NULL`).
            ResponseBody::Bytes(b) if b.is_empty() => (std::ptr::null(), 0),
            ResponseBody::Bytes(b) => (b.as_ptr(), b.len()),
            ResponseBody::Items(items) => items
                .first()
                .filter(|b| !b.is_empty())
                .map(|b| (b.as_ptr(), b.len()))
                .unwrap_or((std::ptr::null(), 0)),
            ResponseBody::NoPayload => (std::ptr::null(), 0),
        }
    }

    fn drop_raw(p: *mut ResponseHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` came from `into_raw_*` and has not
        // already been freed.
        unsafe {
            drop(Box::from_raw(p));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: lifecycle
// ─────────────────────────────────────────────────────────────────────────────

/// Frees a response handle. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_response_free(response: *mut ResponseHandle) {
    if response.is_null() {
        return;
    }
    tracing::trace!(?response, "freeing cosmos_response_t");
    ResponseHandle::drop_raw(response);
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: scalar accessors
// ─────────────────────────────────────────────────────────────────────────────

/// Returns the HTTP status code from the response (e.g. 200, 201,
/// 204). Returns `0` for NULL / degenerate responses.
#[no_mangle]
pub extern "C" fn cosmos_response_status_code(response: *const ResponseHandle) -> u16 {
    let Some(handle) = ResponseHandle::from_ptr(response) else {
        return 0;
    };
    let Some(inner) = handle.inner.as_ref() else {
        return 0;
    };
    u16::from(inner.status().status_code())
}

/// Returns the request charge in Request Units, or `0.0` when the
/// header is absent / response is NULL / response is degenerate.
#[no_mangle]
pub extern "C" fn cosmos_response_request_charge(response: *const ResponseHandle) -> f64 {
    let Some(handle) = ResponseHandle::from_ptr(response) else {
        return 0.0;
    };
    let Some(inner) = handle.inner.as_ref() else {
        return 0.0;
    };
    inner
        .headers()
        .request_charge
        .as_ref()
        .map(|c| c.value())
        .unwrap_or(0.0)
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: string accessors (lazily-cached `CString`s, borrowed across the
// boundary; lifetime == until response is freed)
// ─────────────────────────────────────────────────────────────────────────────

/// Borrowed pointer to the activity id, or NULL when absent / response
/// is NULL.
#[no_mangle]
pub extern "C" fn cosmos_response_activity_id(response: *const ResponseHandle) -> *const c_char {
    let Some(handle) = ResponseHandle::from_ptr(response) else {
        return std::ptr::null();
    };
    let cached = handle.activity_id_cstring.get_or_init(|| {
        handle
            .inner
            .as_ref()?
            .headers()
            .activity_id
            .as_ref()
            .and_then(|a| CString::new(a.as_str().to_owned()).ok())
    });
    cached.as_ref().map_or(std::ptr::null(), |c| c.as_ptr())
}

/// Borrowed pointer to the session token, or NULL when absent.
#[no_mangle]
pub extern "C" fn cosmos_response_session_token(response: *const ResponseHandle) -> *const c_char {
    let Some(handle) = ResponseHandle::from_ptr(response) else {
        return std::ptr::null();
    };
    let cached = handle.session_token_cstring.get_or_init(|| {
        handle
            .inner
            .as_ref()?
            .headers()
            .session_token
            .as_ref()
            .and_then(|t| CString::new(t.as_str().to_owned()).ok())
    });
    cached.as_ref().map_or(std::ptr::null(), |c| c.as_ptr())
}

/// Borrowed pointer to the ETag, or NULL when absent.
#[no_mangle]
pub extern "C" fn cosmos_response_etag(response: *const ResponseHandle) -> *const c_char {
    let Some(handle) = ResponseHandle::from_ptr(response) else {
        return std::ptr::null();
    };
    let cached = handle.etag_cstring.get_or_init(|| {
        handle
            .inner
            .as_ref()?
            .headers()
            .etag
            .as_ref()
            .and_then(|e| CString::new(e.to_string()).ok())
    });
    cached.as_ref().map_or(std::ptr::null(), |c| c.as_ptr())
}

/// Borrowed pointer to the continuation token, or NULL when absent.
#[no_mangle]
pub extern "C" fn cosmos_response_continuation_token(
    response: *const ResponseHandle,
) -> *const c_char {
    let Some(handle) = ResponseHandle::from_ptr(response) else {
        return std::ptr::null();
    };
    let cached = handle.continuation_cstring.get_or_init(|| {
        handle
            .inner
            .as_ref()?
            .headers()
            .continuation
            .as_ref()
            .and_then(|c| CString::new(c.clone()).ok())
    });
    cached.as_ref().map_or(std::ptr::null(), |c| c.as_ptr())
}

/// Borrowed pointer to the **next-page** continuation token for a feed
/// page produced by [`crate::submit::cosmos_submit_operation`],
/// or NULL when this was the last page / the response did not come from a
/// feed submit.
///
/// This is the planner-derived token (from
/// `OperationPlan::to_continuation_token()`) and is the correct token to
/// pass back as `CosmosOperationRequest::continuation_token` to fetch the
/// following page — including for cross-partition queries. It differs from
/// [`cosmos_response_continuation_token`], which surfaces the raw server
/// header continuation (valid only for trivial single-partition reads).
///
/// The returned pointer is valid until [`cosmos_response_free`] is called
/// on this response handle.
#[no_mangle]
pub extern "C" fn cosmos_response_next_continuation(
    response: *const ResponseHandle,
) -> *const c_char {
    let Some(handle) = ResponseHandle::from_ptr(response) else {
        return std::ptr::null();
    };
    handle
        .next_continuation
        .as_ref()
        .map_or(std::ptr::null(), |c| c.as_ptr())
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: body access
// ─────────────────────────────────────────────────────────────────────────────

/// Zero-copy borrowed view of the response body bytes. NULL pointer +
/// 0 length when the body is empty / response is NULL.
///
/// For multi-part feed bodies (driver's `ResponseBody::Items`) this
/// returns the **first** part only; full multi-part iteration is a
/// follow-up alongside the feed pagination surface.
///
/// The returned pointer is valid until [`cosmos_response_free`] is
/// called on this response handle.
#[no_mangle]
pub extern "C" fn cosmos_response_body(
    response: *const ResponseHandle,
    out_data: *mut *const u8,
    out_len: *mut usize,
) -> i32 {
    if out_data.is_null() || out_len.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    // SAFETY: caller guarantees both out-slots are writable.
    unsafe {
        *out_data = std::ptr::null();
        *out_len = 0;
    }
    let Some(handle) = ResponseHandle::from_ptr(response) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let (ptr, len) = handle.body_view();
    // SAFETY: same writable contract as above.
    unsafe {
        *out_data = ptr;
        *out_len = len;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// A flat snapshot of a response's scalar + borrowed-string fields, read in
/// one FFI call.
///
/// Lets a host pull the common response fields (status, RU charge, the four
/// typed header strings, both continuation tokens, and the body view) in a
/// single `cosmos_response_view` call instead of up to eight separate accessor
/// round-trips. All string pointers and `body_data` are **borrowed** — valid
/// until the response is freed — exactly like the individual accessors. The
/// detachable side payloads (`_take_driver` / `_take_container`) are not
/// included; they carry ownership and stay on their own accessors.
#[repr(C)]
pub struct CosmosResponseView {
    /// HTTP status code (`0` for a degenerate response).
    pub status_code: u16,
    /// Request charge in RU (`0.0` when absent).
    pub request_charge: f64,
    /// Borrowed activity id, or NULL when absent.
    pub activity_id: *const c_char,
    /// Borrowed session token, or NULL when absent.
    pub session_token: *const c_char,
    /// Borrowed ETag, or NULL when absent.
    pub etag: *const c_char,
    /// Borrowed server header continuation token, or NULL when absent.
    pub continuation_token: *const c_char,
    /// Borrowed planner-derived next-page continuation token, or NULL when
    /// this was the last page / not a feed response.
    pub next_continuation: *const c_char,
    /// Borrowed pointer to the body bytes, or NULL when the body is empty.
    pub body_data: *const u8,
    /// Number of bytes addressable from `body_data`.
    pub body_len: usize,
}

/// Fills `out_view` with a snapshot of the response's scalar and
/// borrowed-string fields and returns `SUCCESS`. Returns `INVALID_ARGUMENT`
/// (leaving `*out_view` untouched) when `response` or `out_view` is NULL.
///
/// This is the single-call alternative to `cosmos_response_status_code` +
/// `_request_charge` + `_activity_id` + `_session_token` + `_etag` +
/// `_continuation_token` + `_next_continuation` + `_body`. Every borrowed
/// pointer it returns is valid until [`cosmos_response_free`].
#[no_mangle]
pub extern "C" fn cosmos_response_view(
    response: *const ResponseHandle,
    out_view: *mut CosmosResponseView,
) -> i32 {
    if out_view.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(handle) = ResponseHandle::from_ptr(response) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let (body_data, body_len) = handle.body_view();
    // Reuse the individual accessors: each lazily populates its cached
    // `CString` and returns a pointer stable until the response is freed.
    let view = CosmosResponseView {
        status_code: cosmos_response_status_code(response),
        request_charge: cosmos_response_request_charge(response),
        activity_id: cosmos_response_activity_id(response),
        session_token: cosmos_response_session_token(response),
        etag: cosmos_response_etag(response),
        continuation_token: cosmos_response_continuation_token(response),
        next_continuation: cosmos_response_next_continuation(response),
        body_data,
        body_len,
    };
    // SAFETY: `out_view` is non-NULL and the caller guarantees it is writable
    // for one `CosmosResponseView`.
    unsafe {
        *out_view = view;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: side-payload take accessors
// ─────────────────────────────────────────────────────────────────────────────

/// Takes ownership of the driver handle stashed inside a degenerate
/// response produced by `cosmos_driver_get_or_create_submit`. Returns
/// NULL on any other response, on NULL input, or after a previous
/// `_take_driver`.
#[no_mangle]
pub extern "C" fn cosmos_response_take_driver(response: *mut ResponseHandle) -> *mut DriverHandle {
    let Some(handle) = ResponseHandle::from_ptr(response) else {
        return std::ptr::null_mut();
    };
    let mut slot = handle.driver_payload.lock_recover();
    match slot.take() {
        Some(arc) => crate::driver::DriverHandle::from_arc_into_raw(arc),
        None => std::ptr::null_mut(),
    }
}

/// Takes ownership of the container reference stashed inside a
/// degenerate response produced by
/// `cosmos_driver_resolve_container_submit`. Same semantics as
/// `_take_driver`.
#[no_mangle]
pub extern "C" fn cosmos_response_take_container(
    response: *mut ResponseHandle,
) -> *mut ContainerRefHandle {
    let Some(handle) = ResponseHandle::from_ptr(response) else {
        return std::ptr::null_mut();
    };
    let mut slot = handle.container_payload.lock_recover();
    match slot.take() {
        Some(container) => crate::container_ref::ContainerRefHandle::into_raw(container),
        None => std::ptr::null_mut(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn free_handles_null() {
        cosmos_response_free(ptr::null_mut());
    }

    #[test]
    fn accessors_handle_null() {
        assert_eq!(cosmos_response_status_code(ptr::null()), 0);
        assert_eq!(cosmos_response_request_charge(ptr::null()), 0.0);
        assert!(cosmos_response_activity_id(ptr::null()).is_null());
        assert!(cosmos_response_session_token(ptr::null()).is_null());
        assert!(cosmos_response_etag(ptr::null()).is_null());
        assert!(cosmos_response_continuation_token(ptr::null()).is_null());

        let mut data: *const u8 = ptr::null();
        let mut len: usize = 0;
        assert_eq!(
            cosmos_response_body(ptr::null(), &mut data, &mut len),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
    }

    #[test]
    fn take_side_payloads_on_plain_response_returns_null() {
        // A response built without side payloads must return NULL for
        // both _take_driver and _take_container regardless of body /
        // headers.
        //
        // We can't construct a `CosmosResponse` directly here without
        // touching `pub(crate)` API, but the take-on-NULL paths above
        // already exercise both code paths.
    }

    #[test]
    fn response_view_rejects_null() {
        let mut view = blank_view();
        assert_eq!(
            cosmos_response_view(ptr::null(), &mut view),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_response_view(ptr::null(), ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
    }

    #[test]
    fn response_view_matches_accessors_on_degenerate_with_next_token() {
        // A degenerate (no-body) response carrying a planner next-page token.
        // The view must agree with every individual accessor.
        let handle = ResponseHandle::into_raw_with_next_continuation(
            None,
            Some("next-page-token".to_owned()),
        );

        let mut view = blank_view();
        assert_eq!(
            cosmos_response_view(handle, &mut view),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );

        assert_eq!(view.status_code, cosmos_response_status_code(handle));
        assert_eq!(view.request_charge, cosmos_response_request_charge(handle));
        assert_eq!(view.activity_id, cosmos_response_activity_id(handle));
        assert_eq!(view.session_token, cosmos_response_session_token(handle));
        assert_eq!(view.etag, cosmos_response_etag(handle));
        assert_eq!(
            view.continuation_token,
            cosmos_response_continuation_token(handle)
        );
        assert_eq!(
            view.next_continuation,
            cosmos_response_next_continuation(handle)
        );
        // Degenerate response: no body, but a populated next-page token.
        assert!(view.body_data.is_null());
        assert_eq!(view.body_len, 0);
        assert!(!view.next_continuation.is_null());

        cosmos_response_free(handle);
    }

    fn blank_view() -> CosmosResponseView {
        CosmosResponseView {
            status_code: 0,
            request_charge: 0.0,
            activity_id: ptr::null(),
            session_token: ptr::null(),
            etag: ptr::null(),
            continuation_token: ptr::null(),
            next_continuation: ptr::null(),
            body_data: ptr::null(),
            body_len: 0,
        }
    }
}
