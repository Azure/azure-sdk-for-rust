// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Error types for the FFI boundary.
//!
//! Mirrors §3.5 of `docs/NATIVE_WRAPPER_SPEC.md`. The internal [`Error`] type
//! is rich (carries an optional `Box<dyn std::error::Error>` source); the
//! external [`CosmosError`] type is `#[repr(C)]` and contains only raw
//! pointers + the discriminant code.

use std::ffi::{c_char, CStr, CString};

/// Collection of static C strings for common error messages.
pub mod messages {
    use std::ffi::CStr;

    pub static OPERATION_SUCCEEDED: &CStr = c"Operation completed successfully";
    pub static INVALID_UTF8: &CStr = c"String is not valid UTF-8";
    pub static STRING_CONTAINS_NUL: &CStr = c"String contains NUL bytes";
    pub static NULL_OUTPUT_POINTER: &CStr = c"Output pointer is null";
    pub static INVALID_ENDPOINT: &CStr = c"Invalid endpoint string";
    pub static INVALID_KEY: &CStr = c"Invalid key string";
    pub static INVALID_DATABASE_ID: &CStr = c"Invalid database ID string";
    pub static INVALID_CONTAINER_ID: &CStr = c"Invalid container ID string";
    pub static INVALID_ITEM_ID: &CStr = c"Invalid item ID string";
    pub static INVALID_PARTITION_KEY: &CStr = c"Invalid partition key";
    pub static INVALID_ACCOUNT_REFERENCE: &CStr = c"Invalid account reference";
    pub static INVALID_HANDLE: &CStr = c"Invalid handle";
    pub static OPERATION_CONSUMED: &CStr = c"Operation handle has already been consumed";
    pub static RESPONSE_CONSUMED: &CStr = c"Response handle has already been consumed";
}

/// Discriminant codes returned across the FFI boundary.
///
/// Layout is `#[repr(i32)]` so any 32-bit signed integer is a valid C-side
/// representation. Unknown values from the FFI side are coerced to
/// [`CosmosErrorCode::UnknownError`] via [`From<i32>`].
#[repr(i32)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CosmosErrorCode {
    #[default]
    Success = 0,

    // ── Generic FFI / argument errors ─────────────────────────────────────
    InvalidArgument = 1,
    ConnectionFailed = 2,
    UnknownError = 999,

    // ── HTTP-mapped (informational, used by diagnostics) ───────────────────
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

    // ── Cosmos-specific ────────────────────────────────────────────────────
    AuthenticationFailed = 1001,
    DataConversion = 1002,

    // ── FFI-layer programmer errors (30xx) ────────────────────────────────
    InternalError = 3001,
    InvalidUtf8 = 3002,
    InvalidHandle = 3003,
    MemoryError = 3004,
    MarshalingError = 3005,
    CallContextMissing = 3006,
    RuntimeContextMissing = 3007,
    InvalidCString = 3008,

    // ── Driver-wrapper-specific (40xx) — new in this crate ────────────────
    RuntimeAlreadyInitialized = 4001,
    DriverNotInitialized = 4002,
    InvalidAccountReference = 4003,
    InvalidPartitionKey = 4004,
    OperationConsumed = 4005,
    ResponseConsumed = 4006,
}

/// Rich internal error type. NOT exposed across the FFI boundary directly.
///
/// cbindgen:ignore
#[derive(Debug)]
pub struct Error {
    code: CosmosErrorCode,
    message: &'static CStr,
    detail: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Error {
    pub const SUCCESS: Self = Self {
        code: CosmosErrorCode::Success,
        message: messages::OPERATION_SUCCEEDED,
        detail: None,
    };

    pub fn new(code: CosmosErrorCode, message: &'static CStr) -> Self {
        Self {
            code,
            message,
            detail: None,
        }
    }

    pub fn with_detail(
        code: CosmosErrorCode,
        message: &'static CStr,
        detail: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            code,
            message,
            detail: Some(Box::new(detail)),
        }
    }

    pub fn code(&self) -> CosmosErrorCode {
        self.code
    }

    /// Lowers this rich error to the C-visible [`CosmosError`]. When
    /// `include_detail` is true and a detail source is attached, its
    /// `Display` representation is allocated as a `CString` whose ownership
    /// transfers to the caller (release via `cosmos_string_free`).
    pub fn into_ffi(self, include_detail: bool) -> CosmosError {
        let detail_ptr = if include_detail {
            self.detail
                .as_ref()
                .and_then(|d| CString::new(d.to_string()).ok())
                .map(|c| c.into_raw() as *const c_char)
                .unwrap_or(std::ptr::null())
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
            write!(f, ": {detail}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

impl From<azure_core::Error> for Error {
    fn from(e: azure_core::Error) -> Self {
        // TODO(phase-6): refine into structured Cosmos error codes once
        // execute_operation lands. For now, all driver errors map to a single
        // "unknown" code with the original error attached as detail.
        Error::with_detail(CosmosErrorCode::UnknownError, c"Driver error", e)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(_: std::ffi::NulError) -> Self {
        Error::new(
            CosmosErrorCode::InvalidCString,
            messages::STRING_CONTAINS_NUL,
        )
    }
}

/// External, C-visible error representation.
#[repr(C)]
pub struct CosmosError {
    pub code: CosmosErrorCode,
    /// Static C-string (does NOT need to be freed).
    pub message: *const c_char,
    /// Optional, dynamically allocated detail string. If non-null, MUST be
    /// released via [`crate::string::cosmos_string_free`].
    pub detail: *const c_char,
}

impl Default for CosmosError {
    fn default() -> Self {
        Self {
            code: CosmosErrorCode::default(),
            message: std::ptr::null(),
            detail: std::ptr::null(),
        }
    }
}

impl CosmosError {
    /// cbindgen:ignore
    pub const SUCCESS: Self = Self {
        code: CosmosErrorCode::Success,
        message: messages::OPERATION_SUCCEEDED.as_ptr(),
        detail: std::ptr::null(),
    };
}
