// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Byte buffer marshalling helpers for the C ABI boundary.
//!
//! The wrapper distinguishes two byte-buffer shapes:
//!
//! - **Request-body input** — a borrowed `{ ptr, len }` pair passed inline as
//!   the `body` / `body_len` fields of `cosmos_operation_request_t`. The
//!   wrapper copies the bytes into a driver-owned `Vec<u8>` before returning,
//!   so the host may free the buffer immediately after submit.
//! - **`cosmos_bytes_t`** — a library-owned buffer returned **by value** as a
//!   `{ ptr, len }` pair the caller reads directly (no accessor calls). The
//!   caller hands the struct back to [`cosmos_bytes_free`] to release the
//!   backing allocation.

/// A library-owned byte buffer returned by value across the C ABI.
///
/// The caller reads `ptr` and `len` directly — there are no accessor
/// functions. Ownership of the backing allocation transfers to the caller, who
/// must return the struct to [`cosmos_bytes_free`] exactly once. An empty
/// buffer is represented as a NULL `ptr` with `len == 0`.
#[repr(C)]
pub struct CosmosBytes {
    /// Pointer to the first byte, or NULL when `len` is `0`.
    pub ptr: *const u8,
    /// Number of bytes addressable from `ptr`.
    pub len: usize,
}

impl CosmosBytes {
    /// An empty buffer: NULL pointer, zero length.
    pub(crate) fn empty() -> Self {
        Self {
            ptr: std::ptr::null(),
            len: 0,
        }
    }
}

/// Builds an owned `cosmos_bytes_t` from a `Vec<u8>`, transferring ownership of
/// the allocation to the caller. The buffer must be released via
/// [`cosmos_bytes_free`]. An empty input yields [`CosmosBytes::empty`].
#[allow(dead_code, reason = "first caller is the response body take surface")]
pub(crate) fn into_cosmos_bytes(buf: Vec<u8>) -> CosmosBytes {
    if buf.is_empty() {
        return CosmosBytes::empty();
    }
    let boxed: Box<[u8]> = buf.into_boxed_slice();
    let len = boxed.len();
    // Cast the fat slice pointer down to a thin data pointer; `len` carries the
    // length so `cosmos_bytes_free` can reconstitute the boxed slice.
    let ptr = Box::into_raw(boxed) as *const u8;
    CosmosBytes { ptr, len }
}

/// Releases a `cosmos_bytes_t` previously returned by a library API.
///
/// Passing an empty buffer (NULL `ptr` / zero `len`) is a no-op. Passing a
/// buffer not obtained from this library, or freeing the same buffer twice, is
/// undefined behavior.
#[no_mangle]
pub extern "C" fn cosmos_bytes_free(bytes: CosmosBytes) {
    if bytes.ptr.is_null() || bytes.len == 0 {
        return;
    }
    tracing::trace!(ptr = ?bytes.ptr, len = bytes.len, "freeing bytes");
    // SAFETY: `bytes` was produced by `into_cosmos_bytes`, which boxed a slice
    // of exactly `len` bytes whose data pointer is `ptr`.
    unsafe {
        let slice = std::slice::from_raw_parts_mut(bytes.ptr as *mut u8, bytes.len);
        drop(Box::from_raw(slice as *mut [u8]));
    }
}
