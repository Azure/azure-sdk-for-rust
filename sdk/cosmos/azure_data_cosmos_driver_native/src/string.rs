// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C string helpers and the `c_str!` compile-time builder.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::error::{CosmosErrorCode, Error};

/// Builds a `&'static CStr` from a `&str` literal at compile time.
///
/// This is the same pattern used by the previous wrapper crate. It works in
/// `const` contexts so the resulting NUL-terminated bytes live in the binary's
/// read-only data section.
#[macro_export]
macro_rules! c_str {
    ($s:expr) => {{
        const STR: &str = $s;
        const BYTES: [u8; STR.len() + 1] = {
            let mut buf: [u8; STR.len() + 1] = [0; STR.len() + 1];
            let mut i = 0;
            while i < STR.len() {
                buf[i] = STR.as_bytes()[i];
                i += 1;
            }
            buf
        };
        match ::std::ffi::CStr::from_bytes_with_nul(&BYTES) {
            Ok(cstr) => cstr,
            Err(_) => panic!("c_str! input contains an embedded NUL"),
        }
    }};
}

/// Parses a caller-provided C string into a borrowed `&str`, mapping null
/// pointers and invalid UTF-8 to a structured [`Error`].
pub fn parse_cstr<'a>(ptr: *const c_char, msg: &'static CStr) -> Result<&'a str, Error> {
    if ptr.is_null() {
        return Err(Error::new(CosmosErrorCode::InvalidArgument, msg));
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|_| Error::new(CosmosErrorCode::InvalidUtf8, msg))
}

/// Releases a C string previously returned to the caller through `out_*`.
///
/// Calling this with a null pointer is a no-op.
///
/// # Safety
/// `s` must be either null or a pointer previously produced by this library
/// via `CString::into_raw`. Calling with any other pointer is undefined
/// behavior.
#[no_mangle]
pub unsafe extern "C" fn cosmos_string_free(s: *const c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s as *mut c_char));
    }
}
