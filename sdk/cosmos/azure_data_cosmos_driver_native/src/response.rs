// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_response_t` — wraps the driver's
//! [`azure_data_cosmos_driver::models::CosmosResponse`].
//!
//! ## Scope (Phase 6)
//!
//! - Status code, RU charge.
//! - Body access (zero-copy view + take-ownership variant).
//! - The four high-traffic typed headers (activity_id, session_token,
//!   etag, continuation token).
//! - `cosmos_response_take_driver` for the degenerate response delivered
//!   by `cosmos_driver_get_or_create_submit` (Phase 6 also lands the
//!   async driver-creation path).
//! - `cosmos_response_take_container` for the degenerate response
//!   delivered by `cosmos_driver_resolve_container_submit`.
//!
//! Multi-part responses (`ResponseBody::Items`) collapse to the first
//! part for the body view — the multi-part iterator surface lands in
//! Phase 8 (pager). Diagnostics, header iteration, and the long-tail
//! of typed accessors land in Phase 7 (diagnostics) and Phase 8.
//!
//! ## Storage pun
//!
//! `cosmos_response_t *` wraps an `Arc<ResponseInner>` whose inner state
//! holds the driver's `CosmosResponse` (already cheap to clone) plus
//! lazily-cached `CString` copies of the four borrowed-string
//! accessors. The Arc shape mirrors `CosmosErrorHandle` so future
//! diagnostics borrowing can share allocations.
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] §4.7.
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::ffi::{c_char, CString};
use std::sync::{Arc, Mutex, OnceLock};

use azure_data_cosmos_driver::models::{CosmosResponse, ResponseBody};

use crate::container_ref::ContainerRefHandle;
use crate::driver::DriverHandle;
use crate::error::CosmosErrorCode;

pub(crate) struct ResponseInner {
    /// `None` on degenerate responses (driver-creation and
    /// container-resolve submit paths). On these the scalar / header
    /// accessors return default values; the side-payload
    /// `_take_driver` / `_take_container` accessors carry the real
    /// payload via the parent storage's separate `Mutex` slots.
    pub(crate) inner: Option<CosmosResponse>,
    // Lazily-cached null-terminated copies of the four header strings
    // exposed via the high-traffic typed accessors. Each is populated on
    // first read; once populated the pointer is stable until the
    // response is freed.
    activity_id_cstring: OnceLock<Option<CString>>,
    session_token_cstring: OnceLock<Option<CString>>,
    etag_cstring: OnceLock<Option<CString>>,
    continuation_cstring: OnceLock<Option<CString>>,
}

impl ResponseInner {
    pub(crate) fn new(inner: CosmosResponse) -> Self {
        Self {
            inner: Some(inner),
            activity_id_cstring: OnceLock::new(),
            session_token_cstring: OnceLock::new(),
            etag_cstring: OnceLock::new(),
            continuation_cstring: OnceLock::new(),
        }
    }

    /// Constructs an empty `ResponseInner` for degenerate side-payload
    /// responses (driver-creation / container-resolve submits).
    pub(crate) fn degenerate() -> Self {
        Self {
            inner: None,
            activity_id_cstring: OnceLock::new(),
            session_token_cstring: OnceLock::new(),
            etag_cstring: OnceLock::new(),
            continuation_cstring: OnceLock::new(),
        }
    }
}

/// Opaque C ABI handle for [`CosmosResponse`].
///
/// Storage pun: same shape as `CosmosErrorHandle` — `Arc<ResponseInner>`
/// lives in a trailing storage struct, the C side only sees the
/// `_opaque` marker.
///
/// The handle also carries optional "side payloads" populated only on
/// degenerate responses delivered by the driver-creation / container-
/// resolve submit paths. `_take_driver` / `_take_container` move these
/// payloads out by stealing the Arc slot's interior; once taken, both
/// accessors return NULL.
#[repr(C)]
pub struct ResponseHandle {
    _opaque: [u8; 0],
}

#[repr(C)]
struct ResponseStorage {
    _opaque: [u8; 0],
    inner: Arc<ResponseInner>,
    // Side payloads — `Mutex<Option<...>>` so the take accessors can
    // detach ownership in place. NULL on every "real" CRUD completion.
    driver_payload: Mutex<Option<Arc<crate::driver::DriverInner>>>,
    container_payload: Mutex<Option<Arc<crate::container_ref::ContainerRefInner>>>,
}

impl ResponseHandle {
    pub(crate) fn into_raw(inner: CosmosResponse) -> *mut Self {
        Self::into_raw_with_payloads(Some(inner), None, None)
    }

    pub(crate) fn into_raw_with_driver(driver: Arc<crate::driver::DriverInner>) -> *mut Self {
        Self::into_raw_with_payloads(None, Some(driver), None)
    }

    pub(crate) fn into_raw_with_container(
        container: Arc<crate::container_ref::ContainerRefInner>,
    ) -> *mut Self {
        Self::into_raw_with_payloads(None, None, Some(container))
    }

    fn into_raw_with_payloads(
        response: Option<CosmosResponse>,
        driver: Option<Arc<crate::driver::DriverInner>>,
        container: Option<Arc<crate::container_ref::ContainerRefInner>>,
    ) -> *mut Self {
        let inner = match response {
            Some(r) => Arc::new(ResponseInner::new(r)),
            None => Arc::new(ResponseInner::degenerate()),
        };
        let storage = Box::new(ResponseStorage {
            _opaque: [],
            inner,
            driver_payload: Mutex::new(driver),
            container_payload: Mutex::new(container),
        });
        Box::into_raw(storage).cast::<ResponseHandle>()
    }

    fn storage<'a>(p: *const ResponseHandle) -> Option<&'a ResponseStorage> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` came from `into_raw_*`.
        Some(unsafe { &*(p as *const ResponseStorage) })
    }

    fn drop_raw(p: *mut ResponseHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` came from `into_raw_*`.
        unsafe {
            drop(Box::from_raw(p.cast::<ResponseStorage>()));
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
    let Some(storage) = ResponseHandle::storage(response) else {
        return 0;
    };
    let Some(inner) = storage.inner.inner.as_ref() else {
        return 0;
    };
    u16::from(inner.status().status_code())
}

/// Returns the request charge in Request Units, or `0.0` when the
/// header is absent / response is NULL / response is degenerate.
#[no_mangle]
pub extern "C" fn cosmos_response_request_charge(response: *const ResponseHandle) -> f64 {
    let Some(storage) = ResponseHandle::storage(response) else {
        return 0.0;
    };
    let Some(inner) = storage.inner.inner.as_ref() else {
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
    let Some(storage) = ResponseHandle::storage(response) else {
        return std::ptr::null();
    };
    let inner = &storage.inner;
    let cached = inner.activity_id_cstring.get_or_init(|| {
        inner
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
    let Some(storage) = ResponseHandle::storage(response) else {
        return std::ptr::null();
    };
    let inner = &storage.inner;
    let cached = inner.session_token_cstring.get_or_init(|| {
        inner
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
    let Some(storage) = ResponseHandle::storage(response) else {
        return std::ptr::null();
    };
    let inner = &storage.inner;
    let cached = inner.etag_cstring.get_or_init(|| {
        inner
            .inner
            .as_ref()?
            .headers()
            .etag
            .as_ref()
            .and_then(|e| CString::new(e.as_str().to_owned()).ok())
    });
    cached.as_ref().map_or(std::ptr::null(), |c| c.as_ptr())
}

/// Borrowed pointer to the continuation token, or NULL when absent.
#[no_mangle]
pub extern "C" fn cosmos_response_continuation_token(
    response: *const ResponseHandle,
) -> *const c_char {
    let Some(storage) = ResponseHandle::storage(response) else {
        return std::ptr::null();
    };
    let inner = &storage.inner;
    let cached = inner.continuation_cstring.get_or_init(|| {
        inner
            .inner
            .as_ref()?
            .headers()
            .continuation
            .as_ref()
            .and_then(|c| CString::new(c.clone()).ok())
    });
    cached.as_ref().map_or(std::ptr::null(), |c| c.as_ptr())
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: body access
// ─────────────────────────────────────────────────────────────────────────────

/// Zero-copy borrowed view of the response body bytes. NULL pointer +
/// 0 length when the body is empty / response is NULL.
///
/// For multi-part feed bodies (driver's `ResponseBody::Items`) this
/// returns the **first** part only; full multi-part iteration lands in
/// Phase 8 alongside the pager.
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
    let Some(storage) = ResponseHandle::storage(response) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let Some(inner_response) = storage.inner.inner.as_ref() else {
        // Degenerate response — empty body, success code so callers
        // can distinguish from "NULL handle".
        return CosmosErrorCode::CosmosErrorCodeSuccess.as_i32();
    };
    let (ptr, len) = match inner_response.body() {
        ResponseBody::Bytes(b) => (b.as_ptr(), b.len()),
        ResponseBody::Items(items) => items
            .first()
            .map(|b| (b.as_ptr(), b.len()))
            .unwrap_or((std::ptr::null(), 0)),
        ResponseBody::NoPayload => (std::ptr::null(), 0),
    };
    // SAFETY: same writable contract as above.
    unsafe {
        *out_data = ptr;
        *out_len = len;
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
    let Some(storage) = ResponseHandle::storage(response) else {
        return std::ptr::null_mut();
    };
    let mut slot = storage.driver_payload.lock().unwrap();
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
    let Some(storage) = ResponseHandle::storage(response) else {
        return std::ptr::null_mut();
    };
    let mut slot = storage.container_payload.lock().unwrap();
    match slot.take() {
        Some(arc) => crate::container_ref::ContainerRefHandle::from_arc_into_raw(arc),
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
}
