// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::error::ErrorKind;
use std::ffi::{CStr, CString, NulError};

/// Collection of static C strings for error messages.
pub mod messages {
    use std::ffi::CStr;

    pub static INVALID_UTF8: &CStr = c"String is not valid UTF-8";
    pub static OPERATION_SUCCEEDED: &CStr = c"Operation completed successfully";
    pub static NULL_OUTPUT_POINTER: &CStr = c"Output pointer is null";
    pub static INVALID_JSON: &CStr = c"Invalid JSON data";
    pub static INVALID_ENDPOINT: &CStr = c"Invalid endpoint string";
    pub static INVALID_KEY: &CStr = c"Invalid key string";
    pub static INVALID_DATABASE_ID: &CStr = c"Invalid database ID string";
    pub static INVALID_CONTAINER_ID: &CStr = c"Invalid container ID string";
    pub static INVALID_PARTITION_KEY: &CStr = c"Invalid partition key string";
    pub static INVALID_ITEM_ID: &CStr = c"Invalid item ID string";
    pub static INVALID_QUERY: &CStr = c"Invalid query string";
    pub static INVALID_CLIENT_POINTER: &CStr = c"Invalid client pointer";
    pub static INVALID_DATABASE_POINTER: &CStr = c"Invalid database client pointer";
    pub static INVALID_CONTAINER_POINTER: &CStr = c"Invalid container client pointer";
}

#[repr(i32)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CosmosErrorCode {
    #[default]
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

    InternalError = 3001,         // Internal error within the FFI layer
    InvalidUTF8 = 3002,           // Invalid UTF-8 in string parameters crossing FFI
    InvalidHandle = 3003,         // Corrupted/invalid handle passed across FFI
    MemoryError = 3004,           // Memory allocation/deallocation issues at FFI boundary
    MarshalingError = 3005,       // Data marshaling/unmarshaling failed at FFI boundary
    CallContextMissing = 3006,    // CallContext not provided where required
    RuntimeContextMissing = 3007, // RuntimeContext not provided where required
    InvalidCString = 3008,        // Invalid C string (not null-terminated or malformed)
}

/// Internal structure for representing errors.
///
/// This structure is not exposed across the FFI boundary directly.
/// Instead, the [`CallContext`](crate::context::CallContext) receives this error and then marshals it
/// to an appropriate representation for the caller.
/// cbindgen:ignore
#[derive(Debug)]
pub struct Error {
    /// The error code representing the type of error.
    code: CosmosErrorCode,

    /// A static C string message describing the error. This value does not need to be freed.
    message: &'static CStr,

    /// An optional error detail object that can provide additional context about the error.
    /// This is held as a boxed trait so that it only allocates the string if the user requested detailed errors.
    detail: Option<Box<dyn std::error::Error>>,
}

impl Error {
    /// Creates a success [`CosmosError`] with a static message and no detail.
    pub const SUCCESS: Self = Self {
        code: CosmosErrorCode::Success,
        message: messages::OPERATION_SUCCEEDED,
        detail: None,
    };

    /// Creates a new [`CosmosError`] with a static C string message that does not need to be freed.
    pub fn new(code: CosmosErrorCode, message: &'static CStr) -> Self {
        Self {
            code,
            message,
            detail: None,
        }
    }

    /// Creates a new [`CosmosError`] with both a static message, and a detailed dynamic message that must be freed with [`cosmos_string_free`](crate::string::cosmos_string_free).
    pub fn with_detail(
        code: CosmosErrorCode,
        message: &'static CStr,
        detail: impl std::error::Error + 'static,
    ) -> Self {
        Self {
            code,
            message,
            detail: Some(Box::new(detail)),
        }
    }

    pub fn into_ffi(self, include_details: bool) -> CosmosError {
        let detail_ptr = if include_details {
            if let Some(detail) = self.detail {
                let detail_string = detail.to_string();
                CString::new(detail_string)
                    .map(|c| c.into_raw() as *const _)
                    .unwrap_or_else(|_| std::ptr::null())
            } else {
                std::ptr::null()
            }
        } else {
            std::ptr::null()
        };

        CosmosError {
            code: self.code,
            message: self.message.as_ptr(),
            detail: detail_ptr,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (code: {:?})",
            self.message.to_string_lossy(),
            self.code
        )?;
        if let Some(detail) = &self.detail {
            write!(f, ": {}", detail)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

/// External representation of an error across the FFI boundary.
#[repr(C)]
#[derive(Default)]
pub struct CosmosError {
    /// The error code representing the type of error.
    pub code: CosmosErrorCode,

    /// A static C string message describing the error. This value does not need to be freed.
    pub message: *const std::ffi::c_char,

    /// An optional detailed C string message providing additional context about the error.
    /// This is only set if [`include_error_details`](crate::context::CallContext::include_error_details) is true.
    /// If this pointer is non-null, it must be freed by the caller using [`cosmos_string_free`](crate::string::cosmos_string_free).
    pub detail: *const std::ffi::c_char,
}

impl CosmosError {
    // /// cbindgen:ignore
    // pub static SUCCESS: Self = Self {
    //     code: CosmosErrorCode::Success,
    //     message: messages::OPERATION_SUCCEEDED.as_ptr(),
    //     detail: std::ptr::null(),
    // };
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
fn extract_cosmos_db_error_info(error_message: &str) -> (CosmosErrorCode, &'static CStr) {
    if error_message.contains("PartitionKeyMismatch")
        || error_message.contains("partition key mismatch")
    {
        (
            CosmosErrorCode::PartitionKeyMismatch,
            c"Partition key mismatch",
        )
    } else if error_message.contains("Resource quota exceeded")
        || error_message.contains("Request rate is large")
    {
        (
            CosmosErrorCode::ResourceQuotaExceeded,
            c"Resource quota exceeded",
        )
    } else if error_message.contains("429") && error_message.contains("Request rate is large") {
        (
            CosmosErrorCode::RequestRateTooLarge,
            c"Request rate too large",
        )
    } else if error_message.contains("Entity is too large")
        || error_message.contains("Request entity too large")
    {
        (CosmosErrorCode::ItemSizeTooLarge, c"Item size too large")
    } else if error_message.contains("Partition key") && error_message.contains("not found") {
        (
            CosmosErrorCode::PartitionKeyNotFound,
            c"Partition key not found",
        )
    } else {
        (CosmosErrorCode::UnknownError, c"Unknown error")
    }
}

// Native Azure SDK error conversion using structured error data
pub fn convert_azure_error_native(azure_error: azure_core::Error) -> Error {
    let error_string = azure_error.to_string();

    if let Some(status_code) = azure_error.http_status() {
        let (cosmos_error_code, message) = extract_cosmos_db_error_info(&error_string);

        if cosmos_error_code != CosmosErrorCode::UnknownError {
            Error::with_detail(cosmos_error_code, message, azure_error)
        } else {
            let error_code = http_status_to_error_code(u16::from(status_code));
            Error::with_detail(error_code, c"HTTP error", azure_error)
        }
    } else {
        match azure_error.kind() {
            ErrorKind::Credential => Error::with_detail(
                CosmosErrorCode::AuthenticationFailed,
                c"Authentication failed",
                azure_error,
            ),
            ErrorKind::Io => Error::with_detail(
                CosmosErrorCode::ConnectionFailed,
                c"Connection failed",
                azure_error,
            ),
            ErrorKind::DataConversion => {
                if error_string.contains("Not Found") || error_string.contains("not found") {
                    Error::with_detail(
                        CosmosErrorCode::NotFound,
                        c"Resource not found",
                        azure_error,
                    )
                } else {
                    Error::with_detail(
                        CosmosErrorCode::DataConversion,
                        c"Data conversion failed",
                        azure_error,
                    )
                }
            }
            _ => Error::with_detail(CosmosErrorCode::UnknownError, c"Unknown error", azure_error),
        }
    }
}

impl From<azure_core::Error> for Error {
    fn from(error: azure_core::Error) -> Self {
        convert_azure_error_native(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::with_detail(
            CosmosErrorCode::DataConversion,
            c"JSON serialization/deserialization error",
            error,
        )
    }
}

impl From<NulError> for Error {
    fn from(_error: NulError) -> Self {
        Error::new(
            CosmosErrorCode::InvalidCString,
            c"String contains NUL bytes",
        )
    }
}
