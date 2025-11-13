use azure_core::error::ErrorKind;
use std::ffi::CStr;
use std::os::raw::c_char;

pub static CSTR_NUL_BYTES_ERROR: &CStr = c"String contains NUL bytes";
pub static CSTR_INVALID_CHARS_ERROR: &CStr = c"Error message contains invalid characters";
pub static CSTR_UNKNOWN_ERROR: &CStr = c"Unknown error";
pub static CSTR_INVALID_JSON: &CStr = c"Invalid JSON data";
pub static CSTR_CLIENT_CREATION_FAILED: &CStr = c"Failed to create Azure Cosmos client";

pub static CSTR_INVALID_ENDPOINT: &CStr = c"Invalid endpoint string";
pub static CSTR_INVALID_KEY: &CStr = c"Invalid key string";
pub static CSTR_INVALID_DATABASE_ID: &CStr = c"Invalid database ID string";
pub static CSTR_INVALID_CONTAINER_ID: &CStr = c"Invalid container ID string";
pub static CSTR_INVALID_PARTITION_KEY: &CStr = c"Invalid partition key string";
pub static CSTR_INVALID_ITEM_ID: &CStr = c"Invalid item ID string";
pub static CSTR_INVALID_JSON_DATA: &CStr = c"Invalid JSON data string";
pub static CSTR_INVALID_QUERY: &CStr = c"Invalid query string";

pub static CSTR_QUERY_NOT_IMPLEMENTED: &CStr =
    c"Query operations not yet implemented - requires stream handling";

// Helper function to check if a pointer is one of our static constants
fn is_static_error_message(ptr: *const c_char) -> bool {
    if ptr.is_null() {
        return true;
    }

    ptr == CSTR_INVALID_CHARS_ERROR.as_ptr()
        || ptr == CSTR_UNKNOWN_ERROR.as_ptr()
        || ptr == CSTR_NUL_BYTES_ERROR.as_ptr()
        || ptr == CSTR_INVALID_JSON.as_ptr()
        || ptr == CSTR_CLIENT_CREATION_FAILED.as_ptr()
        || ptr == CSTR_INVALID_ENDPOINT.as_ptr()
        || ptr == CSTR_INVALID_KEY.as_ptr()
        || ptr == CSTR_INVALID_DATABASE_ID.as_ptr()
        || ptr == CSTR_INVALID_CONTAINER_ID.as_ptr()
        || ptr == CSTR_INVALID_PARTITION_KEY.as_ptr()
        || ptr == CSTR_INVALID_ITEM_ID.as_ptr()
        || ptr == CSTR_INVALID_JSON_DATA.as_ptr()
        || ptr == CSTR_QUERY_NOT_IMPLEMENTED.as_ptr()
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CosmosErrorCode {
    Success = 0,
    InvalidArgument = 1,
    ConnectionFailed = 2,
    UnknownError = 999,

    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    PreconditionFailed = 412,
    RequestTimeout = 408,
    TooManyRequests = 429,
    InternalServerError = 500,
    BadGateway = 502,
    ServiceUnavailable = 503,

    // Additional Azure SDK specific error codes
    AuthenticationFailed = 1001,
    DataConversion = 1002,

    PartitionKeyMismatch = 2001,
    ResourceQuotaExceeded = 2002,
    RequestRateTooLarge = 2003,
    ItemSizeTooLarge = 2004,
    PartitionKeyNotFound = 2005,

    // FFI boundary error codes - for infrastructure issues at the boundary
    FFIInvalidUTF8 = 3001,   // Invalid UTF-8 in string parameters crossing FFI
    FFIInvalidHandle = 3002, // Corrupted/invalid handle passed across FFI
    FFIMemoryError = 3003,   // Memory allocation/deallocation issues at FFI boundary
    FFIMarshalingError = 3004, // Data marshaling/unmarshaling failed at FFI boundary
}

// CosmosError struct with hybrid memory management
//
// MEMORY MANAGEMENT STRATEGY:
// - message pointer can be either:
//   1. STATIC: Points to compile-time constants (never freed)
//   2. OWNED: Points to heap-allocated CString (must be freed)
//
// SAFETY:
// - Static messages: Created via from_static_cstr(), pointer lives forever
// - Owned messages: Created via new(), caller responsible for cleanup
// - Mixed usage: free_message() safely distinguishes between static/owned
//
// This approach follows Azure SDK pattern for zero-cost static errors
// while maintaining compatibility with dynamic error messages.
#[repr(C)]
pub struct CosmosError {
    pub code: CosmosErrorCode,
    pub message: *const c_char,
}

impl CosmosError {
    // Safe cleanup that handles both static and owned messages
    pub fn free_message(&mut self) {
        if !is_static_error_message(self.message) && !self.message.is_null() {
            unsafe {
                let _ = std::ffi::CString::from_raw(self.message as *mut c_char);
            }
        }
        self.message = std::ptr::null();
    }

    pub fn success() -> Self {
        Self {
            code: CosmosErrorCode::Success,
            message: std::ptr::null(),
        }
    }

    // Create error from static CStr (zero allocation)
    pub fn from_static_cstr(code: CosmosErrorCode, static_cstr: &'static std::ffi::CStr) -> Self {
        Self {
            code,
            message: static_cstr.as_ptr(),
        }
    }

    pub fn new(code: CosmosErrorCode, message: String) -> Self {
        let c_message = std::ffi::CString::new(message)
            .expect("SDK-generated error message should not contain NUL bytes")
            .into_raw() as *const c_char;

        Self {
            code,
            message: c_message,
        }
    }
}

pub fn http_status_to_error_code(status_code: u16) -> CosmosErrorCode {
    match status_code {
        400 => CosmosErrorCode::BadRequest,
        401 => CosmosErrorCode::Unauthorized,
        403 => CosmosErrorCode::Forbidden,
        404 => CosmosErrorCode::NotFound,
        408 => CosmosErrorCode::RequestTimeout,
        409 => CosmosErrorCode::Conflict,
        412 => CosmosErrorCode::PreconditionFailed,
        429 => CosmosErrorCode::TooManyRequests,
        500 => CosmosErrorCode::InternalServerError,
        502 => CosmosErrorCode::BadGateway,
        503 => CosmosErrorCode::ServiceUnavailable,
        _ => CosmosErrorCode::UnknownError,
    }
}

// Extract Cosmos DB specific error information from error messages
fn extract_cosmos_db_error_info(error_message: &str) -> (CosmosErrorCode, String) {
    if error_message.contains("PartitionKeyMismatch")
        || error_message.contains("partition key mismatch")
    {
        (
            CosmosErrorCode::PartitionKeyMismatch,
            error_message.to_string(),
        )
    } else if error_message.contains("Resource quota exceeded")
        || error_message.contains("Request rate is large")
    {
        (
            CosmosErrorCode::ResourceQuotaExceeded,
            error_message.to_string(),
        )
    } else if error_message.contains("429") && error_message.contains("Request rate is large") {
        (
            CosmosErrorCode::RequestRateTooLarge,
            error_message.to_string(),
        )
    } else if error_message.contains("Entity is too large")
        || error_message.contains("Request entity too large")
    {
        (CosmosErrorCode::ItemSizeTooLarge, error_message.to_string())
    } else if error_message.contains("Partition key") && error_message.contains("not found") {
        (
            CosmosErrorCode::PartitionKeyNotFound,
            error_message.to_string(),
        )
    } else {
        (CosmosErrorCode::UnknownError, error_message.to_string())
    }
}

// Native Azure SDK error conversion using structured error data
pub fn convert_azure_error_native(azure_error: &azure_core::Error) -> CosmosError {
    let error_string = azure_error.to_string();

    if let Some(status_code) = azure_error.http_status() {
        let (cosmos_error_code, refined_message) = extract_cosmos_db_error_info(&error_string);

        if cosmos_error_code != CosmosErrorCode::UnknownError {
            CosmosError::new(cosmos_error_code, refined_message)
        } else {
            let error_code = http_status_to_error_code(u16::from(status_code));
            CosmosError::new(error_code, error_string)
        }
    } else {
        match azure_error.kind() {
            ErrorKind::Credential => CosmosError::new(
                CosmosErrorCode::AuthenticationFailed,
                format!("Authentication failed: {}", azure_error),
            ),
            ErrorKind::Io => CosmosError::new(
                CosmosErrorCode::ConnectionFailed,
                format!("IO error: {}", azure_error),
            ),
            ErrorKind::DataConversion => {
                if error_string.contains("Not Found") || error_string.contains("not found") {
                    CosmosError::new(
                        CosmosErrorCode::NotFound,
                        format!("Resource not found: {}", azure_error),
                    )
                } else {
                    CosmosError::new(
                        CosmosErrorCode::DataConversion,
                        format!("Data conversion error: {}", azure_error),
                    )
                }
            }
            _ => CosmosError::new(
                CosmosErrorCode::UnknownError,
                format!("Unknown error: {}", azure_error),
            ),
        }
    }
}

impl From<azure_core::Error> for CosmosError {
    fn from(error: azure_core::Error) -> Self {
        convert_azure_error_native(&error)
    }
}

impl From<serde_json::Error> for CosmosError {
    fn from(error: serde_json::Error) -> Self {
        CosmosError::new(
            CosmosErrorCode::DataConversion,
            format!("JSON error: {}", error),
        )
    }
}

fn free_non_static_error_message(message: *const c_char) {
    if message.is_null() {
        return;
    }
    if !is_static_error_message(message) {
        unsafe {
            let _ = std::ffi::CString::from_raw(message as *mut c_char);
        }
    }
}

/// Releases the memory associated with a [`CosmosError`].
#[no_mangle]
pub extern "C" fn cosmos_error_free(error: *mut CosmosError) {
    if error.is_null() {
        return;
    }
    unsafe {
        let err = Box::from_raw(error);
        free_non_static_error_message(err.message);
    }
}

pub fn marshal_result<T, E>(
    result: Result<T, E>,
    out_error: *mut CosmosError,
    on_success: impl FnOnce(T),
) -> CosmosErrorCode
where
    E: Into<CosmosError>,
{
    match result {
        Ok(value) => {
            on_success(value);
            unsafe {
                *out_error = CosmosError::success();
            }
            CosmosErrorCode::Success
        }
        Err(err) => {
            let cosmos_error: CosmosError = err.into();
            let code = cosmos_error.code;
            unsafe {
                *out_error = cosmos_error;
            }
            code
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::{error::ErrorKind, Error};

    #[test]
    fn test_convert_azure_error_native_io_error() {
        let azure_error = Error::new(ErrorKind::Io, "Network connection failed");
        let cosmos_error = convert_azure_error_native(&azure_error);
        assert_eq!(cosmos_error.code, CosmosErrorCode::ConnectionFailed);
        free_non_static_error_message(cosmos_error.message);
    }

    #[test]
    fn test_convert_azure_error_native_credential_error() {
        let azure_error = Error::new(ErrorKind::Credential, "Invalid credentials");
        let cosmos_error = convert_azure_error_native(&azure_error);
        assert_eq!(cosmos_error.code, CosmosErrorCode::AuthenticationFailed);
        free_non_static_error_message(cosmos_error.message);
    }

    #[test]
    fn test_convert_azure_error_native_data_conversion_error() {
        let azure_error = Error::new(ErrorKind::DataConversion, "Failed to convert data");
        let cosmos_error = convert_azure_error_native(&azure_error);
        assert_eq!(cosmos_error.code, CosmosErrorCode::DataConversion);
        free_non_static_error_message(cosmos_error.message);
    }

    #[test]
    fn test_convert_azure_error_native_missing_field_id_without_404_remains_dataconversion() {
        // Missing field without explicit 404 indicator should remain DataConversion
        let azure_error = Error::new(
            ErrorKind::DataConversion,
            "missing field `id` at line 1 column 49",
        );
        let cosmos_error = convert_azure_error_native(&azure_error);
        assert_eq!(cosmos_error.code, CosmosErrorCode::DataConversion);
        free_non_static_error_message(cosmos_error.message);
    }

    #[test]
    fn test_convert_azure_error_native_missing_field_id_with_404_maps_to_notfound() {
        // Missing field WITH explicit 404 indicator should map to NotFound
        let azure_error = Error::new(
            ErrorKind::DataConversion,
            "404 Not Found: missing field `id` at line 1 column 49",
        );
        let cosmos_error = convert_azure_error_native(&azure_error);
        assert_eq!(cosmos_error.code, CosmosErrorCode::NotFound);
        free_non_static_error_message(cosmos_error.message);
    }

    #[test]
    fn test_convert_azure_error_native_invalid_value_remains_dataconversion() {
        let azure_error = Error::new(
            ErrorKind::DataConversion,
            "invalid value: integer `-2`, expected u64 at line 1 column 305",
        );
        let cosmos_error = convert_azure_error_native(&azure_error);
        assert_eq!(cosmos_error.code, CosmosErrorCode::DataConversion);
        free_non_static_error_message(cosmos_error.message);
    }

    #[test]
    fn test_convert_azure_error_native_404_in_dataconversion() {
        let azure_error = Error::new(
            ErrorKind::DataConversion,
            "404 Not Found - Resource does not exist",
        );
        let cosmos_error = convert_azure_error_native(&azure_error);
        assert_eq!(cosmos_error.code, CosmosErrorCode::NotFound);
        free_non_static_error_message(cosmos_error.message);
    }

    // Comprehensive unit tests for conservative error mapping
    #[test]
    fn test_error_mapping_explicit_http_status_codes() {
        // Test explicit HTTP status codes in DataConversion errors
        let test_cases = vec![
            ("401 Unauthorized access", CosmosErrorCode::DataConversion), // No longer http
            ("403 Forbidden resource", CosmosErrorCode::DataConversion),  // No longer http
            ("409 Conflict detected", CosmosErrorCode::DataConversion),   // No longer http
            ("404 Not Found", CosmosErrorCode::NotFound),
            ("not found resource", CosmosErrorCode::NotFound),
            ("Not Found: resource missing", CosmosErrorCode::NotFound),
        ];

        for (message, expected_code) in test_cases {
            let azure_error = Error::new(ErrorKind::DataConversion, message);
            let cosmos_error = convert_azure_error_native(&azure_error);
            assert_eq!(
                cosmos_error.code, expected_code,
                "Failed for message: {}",
                message
            );
            free_non_static_error_message(cosmos_error.message);
        }
    }

    #[test]
    fn test_error_mapping_missing_field_patterns() {
        // Test missing field patterns with and without 404 indicators
        let should_remain_dataconversion = vec![
            "missing field `id` at line 1 column 49",
            "missing field `name` at line 2 column 10",
            "missing required field `partition_key`",
            "field `id` missing from JSON", // This should stay DataConversion - no "Not Found" phrase
        ];

        for message in should_remain_dataconversion {
            let azure_error = Error::new(ErrorKind::DataConversion, message);
            let cosmos_error = convert_azure_error_native(&azure_error);
            assert_eq!(
                cosmos_error.code,
                CosmosErrorCode::DataConversion,
                "Should remain DataConversion for: {}",
                message
            );
            free_non_static_error_message(cosmos_error.message);
        }

        // Test missing field WITH explicit "Not Found" indicator (should map to NotFound)
        let should_map_to_notfound = vec![
            "Not Found - missing field `id` in response",
            "not found: missing field `name` at line 2",
        ];

        for message in should_map_to_notfound {
            let azure_error = Error::new(ErrorKind::DataConversion, message);
            let cosmos_error = convert_azure_error_native(&azure_error);
            assert_eq!(
                cosmos_error.code,
                CosmosErrorCode::NotFound,
                "Should map to NotFound for: {}",
                message
            );
            free_non_static_error_message(cosmos_error.message);
        }
    }

    #[test]
    fn test_error_mapping_json_parsing_errors() {
        // Test various JSON parsing errors that should remain DataConversion
        let json_parsing_errors = vec![
            "invalid value: integer `-2`, expected u64 at line 1 column 305",
            "expected `,` or `}` at line 1 column 15",
            "invalid type: string \"hello\", expected u64 at line 2 column 8",
            "EOF while parsing a value at line 1 column 0",
            "invalid escape sequence at line 1 column 20",
        ];

        for message in json_parsing_errors {
            let azure_error = Error::new(ErrorKind::DataConversion, message);
            let cosmos_error = convert_azure_error_native(&azure_error);
            assert_eq!(
                cosmos_error.code,
                CosmosErrorCode::DataConversion,
                "JSON parsing error should remain DataConversion for: {}",
                message
            );
            free_non_static_error_message(cosmos_error.message);
        }
    }

    #[test]
    fn test_error_mapping_http_response_errors() {
        use azure_core::http::StatusCode;

        let http_test_cases = vec![
            (
                StatusCode::NotFound,
                "Resource not found",
                CosmosErrorCode::NotFound,
            ),
            (
                StatusCode::Unauthorized,
                "Authentication failed",
                CosmosErrorCode::Unauthorized,
            ),
            (
                StatusCode::Forbidden,
                "Access denied",
                CosmosErrorCode::Forbidden,
            ),
            (
                StatusCode::Conflict,
                "Resource already exists",
                CosmosErrorCode::Conflict,
            ),
            (
                StatusCode::InternalServerError,
                "Internal server error",
                CosmosErrorCode::InternalServerError,
            ),
            (
                StatusCode::BadRequest,
                "Bad request",
                CosmosErrorCode::BadRequest,
            ),
            (
                StatusCode::RequestTimeout,
                "Request timeout",
                CosmosErrorCode::RequestTimeout,
            ),
            (
                StatusCode::TooManyRequests,
                "Too many requests",
                CosmosErrorCode::TooManyRequests,
            ),
            (
                StatusCode::BadGateway,
                "Bad gateway",
                CosmosErrorCode::BadGateway,
            ),
            (
                StatusCode::ServiceUnavailable,
                "Service unavailable",
                CosmosErrorCode::ServiceUnavailable,
            ),
        ];

        for (status_code, message, expected_code) in http_test_cases {
            let error_kind = ErrorKind::HttpResponse {
                status: status_code,
                error_code: None,
                raw_response: None,
            };
            let azure_error =
                Error::with_error(error_kind, std::io::Error::other(message), message);
            let cosmos_error = convert_azure_error_native(&azure_error);
            assert_eq!(
                cosmos_error.code,
                expected_code,
                "Failed for HTTP status {}: {}",
                u16::from(status_code),
                message
            );
            free_non_static_error_message(cosmos_error.message);
        }
    }

    #[test]
    fn test_error_mapping_fallback_without_http_status() {
        let fallback_test_cases = vec![
            (
                ErrorKind::Other,
                "Generic error without HTTP status",
                CosmosErrorCode::UnknownError,
            ),
            (
                ErrorKind::Other,
                "Some other error",
                CosmosErrorCode::UnknownError,
            ),
        ];

        for (kind, message, expected_code) in fallback_test_cases {
            let azure_error = Error::new(kind, message);
            let cosmos_error = convert_azure_error_native(&azure_error);
            assert_eq!(
                cosmos_error.code, expected_code,
                "Failed for fallback case: {}",
                message
            );
            free_non_static_error_message(cosmos_error.message);
        }
    }

    #[test]
    fn test_error_mapping_edge_cases() {
        // Test edge cases and boundary conditions
        let edge_cases = vec![
            // Case sensitivity (our logic checks for "Not Found" and "not found", but not "NOT FOUND")
            ("NOT FOUND", CosmosErrorCode::DataConversion),
            // Empty/whitespace
            ("", CosmosErrorCode::DataConversion),
            ("   ", CosmosErrorCode::DataConversion),
            // Real Azure error patterns
            (
                "Entity with the specified id does not exist",
                CosmosErrorCode::DataConversion,
            ),
            (
                "Not Found: Entity with the specified id does not exist",
                CosmosErrorCode::NotFound,
            ),
        ];

        for (message, expected_code) in edge_cases {
            let azure_error = Error::new(ErrorKind::DataConversion, message);
            let cosmos_error = convert_azure_error_native(&azure_error);
            assert_eq!(
                cosmos_error.code, expected_code,
                "Failed for edge case: {}",
                message
            );
            free_non_static_error_message(cosmos_error.message);
        }
    }

    #[test]
    fn test_http_status_to_error_code_mapping() {
        assert_eq!(http_status_to_error_code(400), CosmosErrorCode::BadRequest);
        assert_eq!(
            http_status_to_error_code(401),
            CosmosErrorCode::Unauthorized
        );
        assert_eq!(http_status_to_error_code(403), CosmosErrorCode::Forbidden);
        assert_eq!(http_status_to_error_code(404), CosmosErrorCode::NotFound);
        assert_eq!(http_status_to_error_code(409), CosmosErrorCode::Conflict);
        assert_eq!(
            http_status_to_error_code(412),
            CosmosErrorCode::PreconditionFailed
        );
        assert_eq!(
            http_status_to_error_code(408),
            CosmosErrorCode::RequestTimeout
        );
        assert_eq!(
            http_status_to_error_code(429),
            CosmosErrorCode::TooManyRequests
        );
        assert_eq!(
            http_status_to_error_code(500),
            CosmosErrorCode::InternalServerError
        );
        assert_eq!(http_status_to_error_code(502), CosmosErrorCode::BadGateway);
        assert_eq!(
            http_status_to_error_code(503),
            CosmosErrorCode::ServiceUnavailable
        );
        assert_eq!(
            http_status_to_error_code(999),
            CosmosErrorCode::UnknownError
        );
    }

    #[test]
    fn test_cosmos_error_memory_management() {
        let mut error = CosmosError::new(CosmosErrorCode::BadRequest, "Test error".to_string());

        // Message should be allocated
        assert!(!error.message.is_null());

        // Free should work without crashing
        error.free_message();
        assert!(error.message.is_null());
    }

    #[test]
    fn test_cosmos_error_static_message() {
        let error = CosmosError::from_static_cstr(
            CosmosErrorCode::InvalidArgument,
            CSTR_INVALID_CHARS_ERROR,
        );

        // Static message should be set
        assert!(!error.message.is_null());

        // Should be recognized as static
        assert!(is_static_error_message(error.message));
    }
}
