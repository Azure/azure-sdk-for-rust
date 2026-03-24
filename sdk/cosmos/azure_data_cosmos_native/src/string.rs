// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::error::{CosmosErrorCode, Error};

#[macro_export]
macro_rules! c_str {
    ($s:expr) => {
        const {
            // This does a few funky things to make sure we can stay in a const context
            // Which ensures the string is generated as a c-str at compile time
            const STR: &str = $s;
            const BYTES: [u8; STR.len() + 1] = const {
                let mut cstr_buf: [u8; STR.len() + 1] = [0; STR.len() + 1];
                let mut i = 0;
                // For loops over ranges don't work in const contexts yet.
                while i < STR.len() {
                    cstr_buf[i] = STR.as_bytes()[i];
                    i += 1;
                }
                cstr_buf
            };
            match CStr::from_bytes_with_nul(&BYTES) {
                Ok(cstr) => cstr,
                Err(_) => panic!("failed to convert value to C string"),
            }
        }
    };
}

// Safe CString conversion helper that handles NUL bytes gracefully
pub fn safe_cstring_new(s: &str) -> CString {
    CString::new(s).expect("FFI boundary strings must not contain NUL bytes")
}

pub fn parse_cstr<'a>(ptr: *const c_char, error_msg: &'static CStr) -> Result<&'a str, Error> {
    if ptr.is_null() {
        return Err(Error::new(CosmosErrorCode::InvalidArgument, error_msg));
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|_| Error::new(CosmosErrorCode::InvalidArgument, error_msg))
}

/// Releases the memory associated with a C string obtained from Rust.
#[no_mangle]
pub extern "C" fn cosmos_string_free(str: *const c_char) {
    if !str.is_null() {
        tracing::trace!(?str, "freeing string");
        unsafe {
            drop(CString::from_raw(str as *mut c_char));
        }
    }
}
