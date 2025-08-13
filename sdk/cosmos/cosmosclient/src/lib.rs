// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::ffi::{c_char, CStr, CString};

#[no_mangle] // Necessary to prevent the compiler from stripping it when optimizing
/// cbindgen:ignore
pub static BUILD_IDENTIFIER: &CStr = const {
    // This does a few funky things to make sure we can stay in a const context
    // Which ensures the string is generated as a c-str at compile time
    // and thus appears properly if you run `strings [lib] | grep "\$Id:"`
    const BUILD_IDENTIFIER_STR: &str = env!("BUILD_IDENTIFIER");
    const BUILD_IDENTIFIER_BYTES: [u8; BUILD_IDENTIFIER_STR.len() + 1] = const {
        let mut cstrbuf: [u8; BUILD_IDENTIFIER_STR.len() + 1] = [0; BUILD_IDENTIFIER_STR.len() + 1];
        let mut i = 0;
        // For loops over ranges don't really work in const contexts.
        while i < BUILD_IDENTIFIER_STR.len() {
            cstrbuf[i] = BUILD_IDENTIFIER_STR.as_bytes()[i];
            i += 1;
        }
        cstrbuf
    };
    match CStr::from_bytes_with_nul(&BUILD_IDENTIFIER_BYTES) {
        Ok(cstr) => cstr,
        Err(_) => panic!("BUILD_IDENTIFIER is not a valid C string"),
    }
};

#[no_mangle]
/// Returns a constant C string containing the version of the Cosmos Client library.
pub extern "C" fn cosmosclient_version() -> *const c_char {
    let version = env!("CARGO_PKG_VERSION");
    CString::new(version)
        .expect("failed to create CString from version")
        .into_raw()
}
