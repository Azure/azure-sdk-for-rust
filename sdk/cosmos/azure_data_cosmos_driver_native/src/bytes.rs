// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Byte buffer marshalling helpers for the C ABI boundary.
//!
//! Per section 3.3 of the spec, the wrapper distinguishes two byte-buffer shapes:
//!
//! - **`cosmos_bytes_view_t`** — a caller-owned by-value view, layout
//!   published. Used as **input** to the library. Defined in section 3.3; not yet
//!   implemented because no entry point consumes one yet.
//! - **`cosmos_bytes_t`** — a library-owned opaque handle backing
//!   heap-allocated bytes that the library returns. The internal
//!   representation is intentionally NOT published so the storage can evolve
//!   (`Box<Vec<u8>>` today; could become `bytes::Bytes` or mmap-backed in the
//!   future) without an ABI break.
//!
//! This module ships only the opaque output type and its three accessors:
//! [`cosmos_bytes_data`], [`cosmos_bytes_len`], and [`cosmos_bytes_free`].
//! The first caller of [`cosmos_bytes_data`] is the response/body surface.

use std::os::raw::c_void;

/// Opaque byte-buffer handle owned by the library.
///
/// Internal representation: boxed `Vec<u8>`. The struct is intentionally
/// near-empty — cbindgen emits an opaque forward declaration so consumers
/// never see the storage shape.
#[repr(C)]
pub struct CosmosBytes {
    // A zero-length array keeps cbindgen from emitting a zero-sized
    // struct body, which some C compilers warn on.
    _opaque: [u8; 0],
}

/// Allocates a new `cosmos_bytes_t` from a `Vec<u8>` and returns a raw
/// pointer suitable for handing back across the C ABI.
///
/// This is an internal-only helper — it is not exposed via `#[no_mangle]`.
/// The response body surface uses it.
#[allow(dead_code, reason = "first caller is the response body surface")]
pub(crate) fn into_raw_bytes(buf: Vec<u8>) -> *mut CosmosBytes {
    let boxed = Box::new(buf);
    Box::into_raw(boxed) as *mut CosmosBytes
}

/// Returns a borrowed pointer to the start of the byte buffer's payload.
///
/// The returned pointer is valid until [`cosmos_bytes_free`] is called on the
/// same handle. If `b` is null, returns null.
///
/// # Safety
///
/// `b` must have been obtained from a library API that returns
/// `cosmos_bytes_t *`. Reading more than [`cosmos_bytes_len`] bytes is
/// undefined behavior.
#[no_mangle]
pub extern "C" fn cosmos_bytes_data(b: *const CosmosBytes) -> *const u8 {
    if b.is_null() {
        return std::ptr::null();
    }
    // SAFETY: callers must satisfy the safety contract above.
    let vec: &Vec<u8> = unsafe { &*(b as *const Vec<u8>) };
    vec.as_ptr()
}

/// Returns the number of payload bytes in `b`. Returns 0 if `b` is null.
///
/// # Safety
///
/// `b` must have been obtained from a library API that returns
/// `cosmos_bytes_t *`.
#[no_mangle]
pub extern "C" fn cosmos_bytes_len(b: *const CosmosBytes) -> usize {
    if b.is_null() {
        return 0;
    }
    // SAFETY: see above.
    let vec: &Vec<u8> = unsafe { &*(b as *const Vec<u8>) };
    vec.len()
}

/// Releases a `cosmos_bytes_t` previously obtained from a library API.
///
/// Safe to call with a null pointer (no-op). Calling on a non-library handle,
/// or calling twice on the same handle, is undefined behavior.
#[no_mangle]
pub extern "C" fn cosmos_bytes_free(b: *mut CosmosBytes) {
    if b.is_null() {
        return;
    }
    tracing::trace!(ptr = ?(b as *const c_void), "freeing bytes");
    // SAFETY: caller guarantees `b` was obtained from `into_raw_bytes`.
    unsafe {
        drop(Box::from_raw(b as *mut Vec<u8>));
    }
}
