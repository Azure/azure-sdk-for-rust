// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Byte-buffer marshalling across the FFI boundary.
//!
//! The driver is schema-agnostic — request/response bodies are raw `Vec<u8>` /
//! `&[u8]`, not C strings. NUL-terminated `const char*` cannot represent
//! arbitrary bodies (Cosmos binary encoding contains `0x00` bytes), so we use
//! length-prefixed views/buffers instead.
//!
//! See `docs/NATIVE_WRAPPER_SPEC.md` §3.3.

use std::ffi::c_void;

/// Borrowed view over a byte slice owned by the caller.
///
/// The caller must keep the underlying memory live for the duration of the
/// receiving call.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmosBytesView {
    pub data: *const u8,
    pub len: usize,
}

impl Default for CosmosBytesView {
    fn default() -> Self {
        Self {
            data: std::ptr::null(),
            len: 0,
        }
    }
}

impl CosmosBytesView {
    /// Returns the view as a `&[u8]`. Returns `&[]` for a null/empty view.
    ///
    /// # Safety
    /// `data` must either be null (in which case `len` is treated as 0) or
    /// point to a region of at least `len` bytes that remains valid for the
    /// lifetime `'a`.
    pub unsafe fn as_slice<'a>(self) -> &'a [u8] {
        if self.data.is_null() || self.len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(self.data, self.len)
        }
    }
}

/// SDK-owned byte buffer returned to the caller. Must be released via
/// [`cosmos_bytes_free`].
///
/// `handle` is an opaque pointer to the underlying `Box<Vec<u8>>`; the
/// `data`/`len` pair points into that vector. Treat `handle` as a black box.
#[repr(C)]
pub struct CosmosBytes {
    pub data: *const u8,
    pub len: usize,
    /// Opaque allocator handle. Always non-null for a buffer returned by this
    /// library. Set to null if the slot is uninitialized.
    pub handle: *mut c_void,
}

impl Default for CosmosBytes {
    fn default() -> Self {
        Self {
            data: std::ptr::null(),
            len: 0,
            handle: std::ptr::null_mut(),
        }
    }
}

impl CosmosBytes {
    /// Wraps a `Vec<u8>` into a `CosmosBytes`, transferring ownership to the
    /// caller. The caller must release the buffer via [`cosmos_bytes_free`].
    pub fn from_vec(v: Vec<u8>) -> Self {
        // Box the vec so its address is stable, then expose its pointer / len.
        let boxed = Box::new(v);
        let data = boxed.as_ptr();
        let len = boxed.len();
        let handle = Box::into_raw(boxed) as *mut c_void;
        Self { data, len, handle }
    }
}

/// Releases a `cosmos_bytes_t` returned by this library.
///
/// Calling with a `handle == NULL` value is a no-op (so it is safe to call on
/// a default-initialized struct).
///
/// # Safety
/// `bytes.handle` must be either null or a pointer previously produced by
/// [`CosmosBytes::from_vec`] (i.e. by any `cosmos_*` API that returns a
/// `cosmos_bytes_t`).
#[no_mangle]
pub unsafe extern "C" fn cosmos_bytes_free(bytes: CosmosBytes) {
    if !bytes.handle.is_null() {
        drop(Box::from_raw(bytes.handle as *mut Vec<u8>));
    }
}
