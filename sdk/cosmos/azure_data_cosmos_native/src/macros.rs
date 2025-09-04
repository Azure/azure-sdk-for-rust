// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

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
