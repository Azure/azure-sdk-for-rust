// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! String marshalling helpers for the C ABI boundary.
//!
//! The wrapper distinguishes two ownership models for strings crossing the
//! FFI boundary:
//!
//! - **Statically allocated, library-owned** — e.g. [`cosmos_version`]. The
//!   returned `*const c_char` is a pointer into a `&'static CStr` and must
//!   **not** be freed.
//! - **Heap allocated, library-owned, freed by the caller** — returned by
//!   accessors that produce a fresh string (e.g. future
//!   `cosmos_diagnostics_to_json` overloads). Always paired with a matching
//!   [`cosmos_string_free`] call.
//!
//! Inputs flow the other direction: callers hand the library a borrowed
//! `*const c_char` (UTF-8, NUL-terminated). The library never takes ownership
//! of caller-supplied strings — it parses them into Rust `&str` slices via
//! `parse_cstr` for the duration of the call.
//!
//! [`cosmos_version`]: crate::cosmos_version

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Builds a `&'static CStr` from a compile-time `&'static str` literal.
///
/// This is the primary tool for constructing static C strings used in the FFI
/// surface (e.g. the value returned by [`cosmos_version`]). Doing the
/// conversion in a `const` context guarantees the bytes are baked into the
/// final binary instead of being allocated at runtime.
///
/// # Panics
///
/// Panics at compile time if the input string contains an interior NUL byte.
///
/// [`cosmos_version`]: crate::cosmos_version
#[macro_export]
macro_rules! c_str {
    ($s:expr) => {
        const {
            // Stay in a const context the whole way through so the bytes are
            // produced at compile time.
            const STR: &str = $s;
            const BYTES: [u8; STR.len() + 1] = const {
                let mut cstr_buf: [u8; STR.len() + 1] = [0; STR.len() + 1];
                let mut i = 0;
                while i < STR.len() {
                    cstr_buf[i] = STR.as_bytes()[i];
                    i += 1;
                }
                cstr_buf
            };
            match ::std::ffi::CStr::from_bytes_with_nul(&BYTES) {
                Ok(cstr) => cstr,
                Err(_) => panic!("c_str! input must not contain interior NUL bytes"),
            }
        }
    };
}

/// Borrows a caller-supplied C string as a Rust `&str` for the duration of
/// the call.
///
/// Returns `None` if `ptr` is null or if the bytes are not valid UTF-8. The
/// caller retains ownership of the underlying memory; this function does not
/// take a copy.
///
/// # Safety
///
/// `ptr` must either be null or reference a valid NUL-terminated byte
/// sequence that stays valid for at least the lifetime `'a`. Because the
/// returned slice borrows that memory with a caller-chosen `'a` that is not
/// tied to any input, picking an `'a` that outlives the underlying buffer is
/// undefined behavior — hence the `unsafe` contract.
///
/// This ships as a simple boolean-fail helper; an error-aware variant that
/// threads the rich `cosmos_error_t` payload through builders / submits can be
/// added later. For now the convention is "null or invalid
/// UTF-8 → reject the call with `COSMOS_ERROR_CODE_INVALID_ARGUMENT`."
#[allow(
    dead_code,
    reason = "first callers arrive with the builder / submit surface"
)]
pub(crate) unsafe fn parse_cstr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    // SAFETY: caller guarantees `ptr` references a valid NUL-terminated byte
    // sequence valid for at least the lifetime `'a`.
    unsafe { CStr::from_ptr(ptr) }.to_str().ok()
}

/// Releases a heap-allocated, library-owned C string.
///
/// Safe to call with a null pointer (the call is a no-op in that case). Must
/// **not** be called on statically allocated strings such as the return
/// value of [`cosmos_version`]; doing so is undefined behavior.
///
/// [`cosmos_version`]: crate::cosmos_version
#[no_mangle]
pub extern "C" fn cosmos_string_free(s: *const c_char) {
    if s.is_null() {
        return;
    }
    tracing::trace!(?s, "freeing string");
    // SAFETY: caller must have obtained `s` from a library API documented to
    // return a heap-allocated string. Casting from `*const` to `*mut` here is
    // safe because the underlying allocation was originally `*mut` (we just
    // hand out `*const` at the ABI boundary for read-only semantics).
    unsafe {
        drop(CString::from_raw(s as *mut c_char));
    }
}
