use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::error::{CosmosError, CosmosErrorCode};

// Safe CString conversion helper that handles NUL bytes gracefully
pub fn safe_cstring_new(s: &str) -> CString {
    CString::new(s).expect("FFI boundary strings must not contain NUL bytes")
}

// Safe CString conversion that returns raw pointer and error code
pub fn safe_cstring_into_raw(
    s: &str,
    out_ptr: &mut *mut c_char,
    out_error: &mut CosmosError,
) -> CosmosErrorCode {
    let c_string = safe_cstring_new(s);
    *out_ptr = c_string.into_raw();
    *out_error = CosmosError::success();
    CosmosErrorCode::Success
}

pub fn parse_cstr<'a>(
    ptr: *const c_char,
    error_msg: &'static CStr,
) -> Result<&'a str, CosmosError> {
    if ptr.is_null() {
        return Err(CosmosError::from_static_cstr(
            CosmosErrorCode::InvalidArgument,
            error_msg,
        ));
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|_| CosmosError::from_static_cstr(CosmosErrorCode::InvalidArgument, error_msg))
}

/// Releases the memory associated with a C string obtained from Rust.
#[no_mangle]
pub extern "C" fn cosmos_string_free(ptr: *const c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr as *mut c_char);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cosmos_version;
    use crate::error::CSTR_INVALID_JSON;

    use super::*;
    use std::ffi::CStr;
    use std::ptr;

    #[test]
    fn test_cosmos_version() {
        let version_ptr = cosmos_version();
        assert!(!version_ptr.is_null());

        let version_str = unsafe { CStr::from_ptr(version_ptr).to_str().unwrap() };

        assert!(version_str.contains("cosmos-cpp-wrapper"));
        assert!(version_str.contains("v0.1.0"));

        cosmos_string_free(version_ptr);
    }

    #[test]
    fn test_cosmos_string_free_null_safety() {
        cosmos_string_free(ptr::null());
    }

    #[test]
    fn test_safe_cstring_new() {
        let result = safe_cstring_new("hello world");
        assert_eq!(result.to_str().unwrap(), "hello world");

        let panic_result = std::panic::catch_unwind(|| {
            safe_cstring_new("hello\0world");
        });
        assert!(panic_result.is_err());
    }

    #[test]
    fn test_safe_cstring_into_raw() {
        let mut ptr: *mut c_char = ptr::null_mut();
        let mut error = CosmosError::success();
        let code = safe_cstring_into_raw("test", &mut ptr, &mut error);
        assert_eq!(code, CosmosErrorCode::Success);
        assert!(!ptr.is_null());
        assert_eq!(error.code, CosmosErrorCode::Success);

        cosmos_string_free(ptr);

        let panic_result = std::panic::catch_unwind(|| {
            let mut ptr2: *mut c_char = ptr::null_mut();
            let mut error2 = CosmosError::success();
            safe_cstring_into_raw("test\0fail", &mut ptr2, &mut error2);
        });
        assert!(panic_result.is_err());
    }

    #[test]
    fn test_static_vs_owned_error_messages() {
        let static_error =
            CosmosError::from_static_cstr(CosmosErrorCode::DataConversion, CSTR_INVALID_JSON);
        assert_eq!(static_error.code, CosmosErrorCode::DataConversion);
        assert!(!static_error.message.is_null());
        assert_eq!(
            static_error.message,
            CSTR_INVALID_JSON.as_ptr() as *mut c_char
        );

        let owned_error = CosmosError::new(
            CosmosErrorCode::BadRequest,
            "Dynamic error message".to_string(),
        );
        assert_eq!(owned_error.code, CosmosErrorCode::BadRequest);
        assert!(!owned_error.message.is_null());
        assert_ne!(
            owned_error.message,
            CSTR_INVALID_JSON.as_ptr() as *mut c_char
        );
    }
}
