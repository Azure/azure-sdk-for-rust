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
//! [`CosmosStringView`] (counted UTF-8). The library copies inputs before
//! returning; outputs retain their existing NUL-terminated representation.
//!
//! [`cosmos_version`]: crate::cosmos_version

use crate::error::CosmosErrorCode;
use std::ffi::CString;
use std::os::raw::c_char;

/// Borrowed counted UTF-8 input, copied before the FFI call returns.
///
/// `len` counts bytes, not characters, and must not exceed `isize::MAX`.
/// Nonempty data must occupy one readable allocation valid throughout the call.
/// NULL/0 means unset for optional fields; non-NULL/0 means explicitly empty.
/// Required fields reject NULL, except partition-key strings where NULL/0 is empty.
/// NULL/nonzero is always invalid. Only partition-key strings permit embedded NUL.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CosmosStringView {
    /// Borrowed UTF-8 bytes, without a terminator requirement.
    pub data: *const u8,
    /// Number of readable bytes.
    pub len: usize,
}

impl Default for CosmosStringView {
    fn default() -> Self {
        Self {
            data: std::ptr::null(),
            len: 0,
        }
    }
}

impl CosmosStringView {
    pub(crate) fn is_unset(self) -> bool {
        self.data.is_null() && self.len == 0
    }
}

#[cfg(test)]
pub(crate) fn view(bytes: &[u8]) -> CosmosStringView {
    CosmosStringView {
        data: bytes.as_ptr(),
        len: bytes.len(),
    }
}

/// Validates slice metadata before any pointer arithmetic or allocation.
pub(crate) fn validate_array<T>(data: *const T, len: usize) -> Result<(), CosmosErrorCode> {
    if (data.is_null() && len != 0) || len > (isize::MAX as usize) / std::mem::size_of::<T>().max(1)
    {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    Ok(())
}

/// Copies all bytes, validating UTF-8 before field-specific validation.
///
/// # Safety
/// Nonempty `view` must describe readable bytes in one allocation for this call.
pub(crate) unsafe fn copy_utf8(view: CosmosStringView) -> Result<String, CosmosErrorCode> {
    validate_array(view.data, view.len)?;
    if view.len == 0 {
        return Ok(String::new());
    }
    // SAFETY: bounds/null checked above; allocation validity is the caller's contract.
    let bytes = unsafe { std::slice::from_raw_parts(view.data, view.len) };
    std::str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
}

/// Copies required text and rejects NUL with the field's typed error.
///
/// # Safety
/// The view must satisfy [`copy_utf8`]'s allocation contract.
pub(crate) unsafe fn required_text(
    view: CosmosStringView,
    invalid: CosmosErrorCode,
) -> Result<String, CosmosErrorCode> {
    if view.data.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: forwarded allocation contract.
    let text = unsafe { copy_utf8(view) }?;
    if text.contains('\0') {
        return Err(invalid);
    }
    Ok(text)
}

/// Copies optional text, preserving unset versus explicitly empty.
///
/// # Safety
/// The view must satisfy [`copy_utf8`]'s allocation contract.
pub(crate) unsafe fn optional_text(
    view: CosmosStringView,
    invalid: CosmosErrorCode,
) -> Result<Option<String>, CosmosErrorCode> {
    if view.is_unset() {
        return Ok(None);
    }
    // SAFETY: forwarded allocation contract.
    unsafe { required_text(view, invalid) }.map(Some)
}

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
    // SAFETY: caller obtained this allocation from a library API that transfers ownership.
    unsafe {
        drop(CString::from_raw(s as *mut c_char));
    }
}
#[cfg(test)]
mod tests {
    use super::{copy_utf8, optional_text, required_text, view, CosmosStringView};
    use crate::error::CosmosErrorCode;

    #[test]
    fn complete_utf8_and_null_contract() {
        let invalid = CosmosErrorCode::CosmosErrorCodeInvalidOptionValue;
        // SAFETY: literals remain live; invalid metadata is rejected before reading.
        unsafe {
            assert_eq!(
                copy_utf8(view("é水\0tail".as_bytes())).unwrap(),
                "é水\0tail"
            );
            assert_eq!(
                copy_utf8(view(b"ok\0\xff")),
                Err(CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
            );
            assert_eq!(required_text(view(b"ok\0tail"), invalid), Err(invalid));
            assert_eq!(
                required_text(view(b"ok\0\xff"), invalid),
                Err(CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
            );
            assert_eq!(copy_utf8(CosmosStringView::default()).unwrap(), "");
            assert_eq!(
                optional_text(CosmosStringView::default(), invalid).unwrap(),
                None
            );
            assert_eq!(
                optional_text(view(b""), invalid).unwrap(),
                Some(String::new())
            );
            assert_eq!(
                required_text(CosmosStringView::default(), invalid),
                Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument)
            );
            for input in [
                CosmosStringView {
                    data: std::ptr::null(),
                    len: 1,
                },
                CosmosStringView {
                    data: b"x".as_ptr(),
                    len: isize::MAX as usize + 1,
                },
            ] {
                assert_eq!(
                    copy_utf8(input),
                    Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument)
                );
                assert_eq!(
                    optional_text(input, invalid),
                    Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument)
                );
            }
        }
    }

    #[test]
    fn copied_value_survives_input_release() {
        let owned = {
            let input = "水\0tail".as_bytes().to_vec();
            // SAFETY: input remains live until copying finishes.
            unsafe { copy_utf8(view(&input)) }.unwrap()
        };
        assert_eq!(owned, "水\0tail");
    }
}
