macro_rules! c_str {
    ($s:expr) => { const {
        // This does a few funky things to make sure we can stay in a const context
        // Which ensures the string is generated as a c-str at compile time
        const STR: &str = $s
        const BYTES: [u8; STR.len() + 1] = const {
            let mut cstrbuf: [u8; STR.len() + 1] = [0; STR.len() + 1];
            let mut i = 0;
            // For loops over ranges don't really work in const contexts.
            while i < STR.len() {
                cstrbuf[i] = STR.as_bytes()[i];
                i += 1;
            }
            cstrbuf
        };
        match CStr::from_bytes_with_nul(&BYTES) {
            Ok(cstr) => cstr,
            Err(_) => panic!("failed to convert value to C string"),
        }
    } },
}
