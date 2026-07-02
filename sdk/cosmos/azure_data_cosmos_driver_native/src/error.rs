// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Error model for the C ABI boundary.
//!
//! Implements the spec's two-layer error contract from
//! [`docs/NATIVE_WRAPPER_SPEC.md`] section 3.5:
//!
//! - **Coarse status** ([`CosmosErrorCode`] in Rust / `cosmos_error_code_t` in
//!   C). A `#[repr(i32)]` enum whose value-range bands (`0`, `1..=999`,
//!   `1001..=1999`, `2001..=2999`, `3001..=3999`, `4001..=4999`, `5001..=5999`)
//!   mirror the old `azure_data_cosmos_native` ranges so callers that already
//!   switch on the bands keep working.
//! - **Rich payload** ([`CosmosErrorHandle`] in Rust / `cosmos_error_t *` in C).
//!   An opaque heap-allocated handle wrapping
//!   [`azure_data_cosmos_driver::error::CosmosError`]. Accessors mirror the
//!   merged driver API 1:1 (per spec section 3.5.2).
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::ffi::{c_char, CString};
use std::sync::Arc;

use azure_data_cosmos_driver::error::CosmosError as DriverCosmosError;

// ─────────────────────────────────────────────────────────────────────────────
// Coarse status code
// ─────────────────────────────────────────────────────────────────────────────

/// Coarse numeric return value for every fallible C function.
///
/// Per spec section 3.5.1, the layout retains the FFI / Cosmos-specific bands
/// established by the old wrapper:
///
/// - `0` — success.
/// - `1..=999` — FFI / argument-validation errors.
/// - `1001..=1999` — auth / conversion errors.
/// - `2001..=2999` — Cosmos-specific errors mapped from wire HTTP status.
/// - `3001..=3999` — FFI plumbing errors.
/// - `4001..=4999` — driver-wrapper-specific fatal codes (new in this crate).
/// - `5001..=5999` — non-fatal warnings (`out_*` populated; rich error advisory).
///
/// Only the codes the completion / queue / handle FFI actively produces are
/// populated today. The rest are reserved and are added as their producing
/// surfaces land. Consumers must treat unknown codes per
/// their band: `4xxx` = fatal-but-recoverable, `5xxx` = warning with
/// populated `out_*`.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CosmosErrorCode {
    /// Operation completed successfully.
    CosmosErrorCodeSuccess = 0,

    // ── 1..=999: FFI / argument-validation (reserved for incremental fill-in) ──
    /// A required pointer argument was `NULL`. Every accessor checks for this
    /// before dereferencing.
    CosmosErrorCodeInvalidArgument = 1,

    /// A `*const c_char` argument contained bytes that were not valid UTF-8.
    CosmosErrorCodeInvalidUtf8 = 2,

    // ── 1001..=1999: auth / conversion (reserved) ──

    // ── 2001..=2999: Cosmos service errors ──
    /// Mapped from a wire response with HTTP 404.
    CosmosErrorCodeNotFound = 2404,

    /// Mapped from a wire response with HTTP 409.
    CosmosErrorCodeConflict = 2409,

    /// Mapped from a wire response with HTTP 412.
    CosmosErrorCodePreconditionFailed = 2412,

    /// Mapped from a wire response with HTTP 429.
    CosmosErrorCodeThrottled = 2429,

    /// Mapped from a wire response with HTTP 410.
    CosmosErrorCodeGone = 2410,

    /// Mapped from a wire response with HTTP 408 (or synthetic
    /// `CLIENT_OPERATION_TIMEOUT` substatus 20008).
    CosmosErrorCodeTimeout = 2408,

    /// Mapped from a wire response with HTTP 401.
    CosmosErrorCodeUnauthorized = 2401,

    /// Mapped from a wire response with HTTP 403.
    CosmosErrorCodeForbidden = 2403,

    /// Mapped from a wire response with HTTP 400.
    CosmosErrorCodeBadRequest = 2400,

    /// Mapped from a wire response with HTTP 503 (excluding transport-
    /// synthesized 503, which falls under [`TransportFailure`](Self::CosmosErrorCodeTransportFailure)).
    CosmosErrorCodeServiceUnavailable = 2503,

    /// Any other wire-side error (5xx, unmapped 4xx).
    CosmosErrorCodeServiceError = 2999,

    // ── 3001..=3999: FFI plumbing ──
    /// A driver client-side / synthetic failure with no specific 2xxx mapping.
    CosmosErrorCodeClientError = 3001,

    /// A driver transport-layer failure (connection / DNS / TLS / IO).
    CosmosErrorCodeTransportFailure = 3002,

    /// A driver client-side serialization failure.
    CosmosErrorCodeSerializationFailed = 3003,

    /// A driver client-side authentication failure (e.g. token acquisition).
    CosmosErrorCodeAuthenticationFailed = 3004,

    /// A driver client-side operation timeout
    /// (`SubStatusCode::CLIENT_OPERATION_TIMEOUT` = 20008).
    CosmosErrorCodeClientOperationTimeout = 3005,

    // ── 4001..=4999: driver-wrapper-specific fatal codes (per spec section 3.5.1) ──
    //
    // Code 4001 is intentionally reserved (formerly OPTIONS_IGNORED_ON_CACHE_HIT,
    // moved to the 5xxx warning class).
    /// Operation issued before `initialize()` completed.
    CosmosErrorCodeDriverNotInitialized = 4002,

    /// Account endpoint URL or credential could not be parsed.
    CosmosErrorCodeInvalidAccountReference = 4003,

    /// `PartitionKey` builder produced an empty / inconsistent key.
    CosmosErrorCodeInvalidPartitionKey = 4004,

    /// A mutator or second submit was called after the operation handle was
    /// already consumed by an earlier successful submit.
    CosmosErrorCodeOperationConsumed = 4005,

    /// Reserved. Formerly signalled a response handle consumed twice; the
    /// response is now delivered inline on the completion, so this code is no
    /// longer produced but its numeric slot is retained for ABI stability.
    CosmosErrorCodeResponseConsumed = 4006,

    /// Single-shot submit yielded `Ok(None)` from a feed-style operation.
    CosmosErrorCodeFeedExhausted = 4007,

    /// Second precondition setter on an operation that already has one.
    CosmosErrorCodePreconditionAlreadySet = 4008,

    /// A mutator only meaningful for a specific operation kind was rejected.
    CosmosErrorCodeUnsupportedOperationForMutator = 4009,

    /// A request header (`cosmos_header_kv_t`) on the submitted operation
    /// request had a non-ASCII / control-character name or value.
    CosmosErrorCodeInvalidHeader = 4010,

    /// A submit targeted a `cosmos_completion_queue_t` that had already been
    /// shut down via
    /// `cosmos_completion_queue_shutdown`. Pre-flight rejection — no completion is posted.
    CosmosErrorCodeQueueShutdown = 4011,

    /// Surfaced via the completion's `status` field when its outcome
    /// is `CANCELLED`. Triggered by `cosmos_operation_handle_cancel` or by
    /// `cosmos_completion_queue_shutdown`.
    CosmosErrorCodeOperationCancelled = 4012,

    /// A submit targeted a `cosmos_completion_queue_t` whose hard capacity is already
    /// reached. Pre-flight rejection — no completion is posted.
    CosmosErrorCodeQueueFull = 4013,

    /// A builder setter was passed a value outside the documented range.
    CosmosErrorCodeInvalidOptionValue = 4014,

    /// `cosmos_runtime_build` could not construct the underlying
    /// `CosmosDriverRuntime`.
    CosmosErrorCodeRuntimeBuildFailed = 4015,

    // ── 5001..=5999: non-fatal warnings (per spec section 3.5.1) ──
    /// `cosmos_driver_get_or_create` called with non-NULL options while a
    /// driver for the same account endpoint was already cached. The cached
    /// instance is still delivered.
    CosmosErrorCodeOptionsIgnoredOnCacheHit = 5001,
}

impl CosmosErrorCode {
    /// Numeric `i32` representation; the value carried across the FFI boundary.
    #[inline]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Derives the coarse code from a driver `CosmosError` per spec section 6.3.
    ///
    /// The routing is top-to-bottom: more specific synthetic-substatus checks
    /// run first, then synthetic-status branches, then the HTTP-status table.
    pub fn from_driver_error(err: &DriverCosmosError) -> Self {
        let status = err.status();
        let http = u16::from(status.status_code());
        let sub = status.sub_status().map(|s| s.value());

        if !err.is_from_wire() {
            // Synthetic (transport / client / serialization / auth / config).
            // Pattern-match on sub-status first, fall back to the synthetic
            // 408 / 503 placeholders.
            match sub {
                Some(20402) => return Self::CosmosErrorCodeAuthenticationFailed,
                Some(20003) | Some(20010..=20015) => return Self::CosmosErrorCodeTransportFailure,
                Some(20008) => return Self::CosmosErrorCodeClientOperationTimeout,
                Some(20020) => return Self::CosmosErrorCodeSerializationFailed,
                _ => {}
            }
            return Self::CosmosErrorCodeClientError;
        }

        // Wire response — switch on HTTP status code.
        match http {
            429 => Self::CosmosErrorCodeThrottled,
            404 => Self::CosmosErrorCodeNotFound,
            409 => Self::CosmosErrorCodeConflict,
            412 => Self::CosmosErrorCodePreconditionFailed,
            408 => Self::CosmosErrorCodeTimeout,
            410 => Self::CosmosErrorCodeGone,
            401 => Self::CosmosErrorCodeUnauthorized,
            403 => Self::CosmosErrorCodeForbidden,
            400 => Self::CosmosErrorCodeBadRequest,
            503 => Self::CosmosErrorCodeServiceUnavailable,
            _ => Self::CosmosErrorCodeServiceError,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Rich error handle
// ─────────────────────────────────────────────────────────────────────────────

/// The C ABI handle for a rich error (`cosmos_error_t`).
///
/// Reference-counted via `Arc` so the completion's borrow accessor and the
/// take-ownership accessor can share the same allocation cheaply. Lazy-caches
/// the rendered backtrace and the four header-derived convenience strings as
/// `CString`s so the FFI accessors can hand out borrowed pointers with a
/// stable lifetime.
pub struct CosmosErrorHandle {
    pub(crate) err: DriverCosmosError,
    // Cached null-terminated copies of the strings the FFI returns by
    // borrowed pointer. Populated lazily on first access. We use `OnceLock`
    // because the handle may be retrieved from any thread.
    message_cstring: std::sync::OnceLock<CString>,
    backtrace_cstring: std::sync::OnceLock<Option<CString>>,
    activity_id_cstring: std::sync::OnceLock<Option<CString>>,
    session_token_cstring: std::sync::OnceLock<Option<CString>>,
    etag_cstring: std::sync::OnceLock<Option<CString>>,
}

impl CosmosErrorHandle {
    pub fn new(err: DriverCosmosError) -> Self {
        Self {
            err,
            message_cstring: std::sync::OnceLock::new(),
            backtrace_cstring: std::sync::OnceLock::new(),
            activity_id_cstring: std::sync::OnceLock::new(),
            session_token_cstring: std::sync::OnceLock::new(),
            etag_cstring: std::sync::OnceLock::new(),
        }
    }

    fn message(&self) -> &CString {
        self.message_cstring.get_or_init(|| {
            // Driver `Display` impl produces "{message}" with no NUL bytes,
            // but be defensive — strip any NULs to satisfy CString::new.
            let msg = self.err.to_string();
            CString::new(msg.replace('\0', "")).unwrap_or_else(|_| CString::default())
        })
    }

    fn backtrace_str(&self) -> Option<&CString> {
        self.backtrace_cstring
            .get_or_init(|| {
                self.err
                    .backtrace()
                    .and_then(|bt| CString::new(bt.as_ref().replace('\0', "")).ok())
            })
            .as_ref()
    }

    fn activity_id_str(&self) -> Option<&CString> {
        self.activity_id_cstring
            .get_or_init(|| {
                self.err
                    .response()
                    .and_then(|r| r.headers().activity_id.as_ref())
                    .and_then(|aid| CString::new(aid.as_str().to_owned()).ok())
            })
            .as_ref()
    }

    fn session_token_str(&self) -> Option<&CString> {
        self.session_token_cstring
            .get_or_init(|| {
                self.err
                    .response()
                    .and_then(|r| r.headers().session_token.as_ref())
                    .and_then(|tok| CString::new(tok.as_str().to_owned()).ok())
            })
            .as_ref()
    }

    fn etag_str(&self) -> Option<&CString> {
        self.etag_cstring
            .get_or_init(|| {
                self.err
                    .response()
                    .and_then(|r| r.headers().etag.as_ref())
                    .and_then(|e| CString::new(e.to_string()).ok())
            })
            .as_ref()
    }
}

/// Opaque heap-allocated handle around an error payload.
///
/// The FFI hands out `*mut CosmosErrorHandle` as `cosmos_error_t *` from the
/// synchronous `out_error` slots (driver-create blocking, account-reference
/// parsing). The completion path carries error detail inline, not as a handle.
impl CosmosErrorHandle {
    /// Wraps a driver error into a heap-allocated FFI handle. Returns a raw
    /// pointer suitable for handing across the C boundary.
    pub(crate) fn into_raw(err: DriverCosmosError) -> *mut Self {
        Arc::into_raw(Arc::new(CosmosErrorHandle::new(err))) as *mut Self
    }

    /// Borrows the handle from a raw pointer for the duration of an FFI call.
    fn inner_from_ptr<'a>(p: *const CosmosErrorHandle) -> Option<&'a CosmosErrorHandle> {
        if p.is_null() {
            None
        } else {
            // SAFETY: caller guarantees `p` was obtained from a library API.
            Some(unsafe { &*p })
        }
    }

    pub(crate) fn drop_raw(p: *mut CosmosErrorHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: caller guarantees `p` was obtained from a library API and
        // has not already been freed.
        unsafe {
            drop(Arc::from_raw(p as *const CosmosErrorHandle));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI accessors (cosmos_error_*)
//
// Each accessor returns NULL / 0 / -1 / false when the input is NULL or when
// the underlying field is absent. See spec section 3.5.2 for the contract per
// accessor.
// ─────────────────────────────────────────────────────────────────────────────

/// HTTP status code (always populated, including for synthetic errors).
#[no_mangle]
pub extern "C" fn cosmos_error_status_code(e: *const CosmosErrorHandle) -> u16 {
    let Some(inner) = CosmosErrorHandle::inner_from_ptr(e) else {
        return 0;
    };
    u16::from(inner.err.status().status_code())
}

/// Sub-status code. Returns -1 when absent. Driver-side `SubStatusCode` is
/// `u16`; widening to `i32` lets us reserve -1 for "no sub-status" without
/// clipping any real value.
#[no_mangle]
pub extern "C" fn cosmos_error_sub_status(e: *const CosmosErrorHandle) -> i32 {
    let Some(inner) = CosmosErrorHandle::inner_from_ptr(e) else {
        return -1;
    };
    inner
        .err
        .status()
        .sub_status()
        .map_or(-1, |s| i32::from(s.value()))
}

/// True iff the error originated from a service wire response. Mirrors
/// `CosmosError::is_from_wire`.
#[no_mangle]
pub extern "C" fn cosmos_error_is_from_wire(e: *const CosmosErrorHandle) -> bool {
    CosmosErrorHandle::inner_from_ptr(e).is_some_and(|inner| inner.err.is_from_wire())
}

/// Borrowed message string. Returns NULL only when `e` is NULL.
///
/// Lifetime = until [`cosmos_error_free`].
#[no_mangle]
pub extern "C" fn cosmos_error_message(e: *const CosmosErrorHandle) -> *const c_char {
    let Some(inner) = CosmosErrorHandle::inner_from_ptr(e) else {
        return std::ptr::null();
    };
    inner.message().as_ptr()
}

/// Borrowed activity id from the wire response headers (or NULL when there
/// is no wire response or no activity id was present).
#[no_mangle]
pub extern "C" fn cosmos_error_activity_id(e: *const CosmosErrorHandle) -> *const c_char {
    let Some(inner) = CosmosErrorHandle::inner_from_ptr(e) else {
        return std::ptr::null();
    };
    inner
        .activity_id_str()
        .map_or(std::ptr::null(), |s| s.as_ptr())
}

/// Borrowed session token from the wire response headers (or NULL).
#[no_mangle]
pub extern "C" fn cosmos_error_session_token(e: *const CosmosErrorHandle) -> *const c_char {
    let Some(inner) = CosmosErrorHandle::inner_from_ptr(e) else {
        return std::ptr::null();
    };
    inner
        .session_token_str()
        .map_or(std::ptr::null(), |s| s.as_ptr())
}

/// Borrowed ETag from the wire response headers (or NULL).
#[no_mangle]
pub extern "C" fn cosmos_error_etag(e: *const CosmosErrorHandle) -> *const c_char {
    let Some(inner) = CosmosErrorHandle::inner_from_ptr(e) else {
        return std::ptr::null();
    };
    inner.etag_str().map_or(std::ptr::null(), |s| s.as_ptr())
}

/// Retry-after duration in milliseconds, or -1 when absent / no wire response.
#[no_mangle]
pub extern "C" fn cosmos_error_retry_after_ms(e: *const CosmosErrorHandle) -> i64 {
    let Some(inner) = CosmosErrorHandle::inner_from_ptr(e) else {
        return -1;
    };
    inner
        .err
        .response()
        .and_then(|r| r.headers().retry_after_ms)
        .map_or(-1, |ms| i64::try_from(ms).unwrap_or(i64::MAX))
}

/// Borrowed backtrace string (rate-limited per
/// [`cosmos_set_backtrace_options`]).
/// Returns NULL when no backtrace was captured.
#[no_mangle]
pub extern "C" fn cosmos_error_backtrace(e: *const CosmosErrorHandle) -> *const c_char {
    let Some(inner) = CosmosErrorHandle::inner_from_ptr(e) else {
        return std::ptr::null();
    };
    inner
        .backtrace_str()
        .map_or(std::ptr::null(), |s| s.as_ptr())
}

// ─────────────────────────────────────────────────────────────────────────────
// Lifecycle
// ─────────────────────────────────────────────────────────────────────────────

/// Free a `cosmos_error_t *` obtained via `cosmos_completion_take_error` or
/// a synchronous `out_error` slot. NULL is a no-op. Calling on a borrowed
/// pointer (e.g. `cosmos_completion_error` result) is undefined behavior.
#[no_mangle]
pub extern "C" fn cosmos_error_free(e: *mut CosmosErrorHandle) {
    if e.is_null() {
        return;
    }
    tracing::trace!(?e, "freeing cosmos_error_t");
    CosmosErrorHandle::drop_raw(e);
}

// ─────────────────────────────────────────────────────────────────────────────
// Process-global backtrace knobs (spec section 6.4).
//
// Lives here rather than in lib.rs because it directly drives the optional
// backtrace surface exposed by `cosmos_error_backtrace` above.
// ─────────────────────────────────────────────────────────────────────────────

/// Sets process-global backtrace capture / resolution rate limits.
///
/// Last-writer-wins across concurrent calls. Pass `0` to either parameter to
/// disable that knob. Environment-derived defaults (`RUST_LIB_BACKTRACE`,
/// `RUST_BACKTRACE`, `AZURE_COSMOS_BACKTRACE_*`) are overridden for the rest
/// of the process once this is called. See spec section 6.4.
#[no_mangle]
pub extern "C" fn cosmos_set_backtrace_options(
    max_captures_per_second: u32,
    max_resolutions_per_second: u32,
) {
    // `BacktraceOptions` is `#[non_exhaustive]` on the driver side; build via
    // the public `Default` impl + field mutation so we tolerate future fields
    // landing without a rebuild.
    let mut opts = azure_data_cosmos_driver::error::BacktraceOptions::default();
    opts.max_captures_per_second = max_captures_per_second;
    opts.max_resolutions_per_second = max_resolutions_per_second;
    azure_data_cosmos_driver::error::set_backtrace_options(opts);
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use azure_data_cosmos_driver::error::{CosmosStatus, SubStatusCode};

    fn make_synthetic_error(status: CosmosStatus, message: &'static str) -> *mut CosmosErrorHandle {
        let err = DriverCosmosError::builder()
            .with_status(status)
            .with_message(message)
            .build();
        CosmosErrorHandle::into_raw(err)
    }

    /// Helper for tests that produces a fresh client-side timeout error.
    fn synth_client_timeout() -> *mut CosmosErrorHandle {
        let status = CosmosStatus::new(azure_core::http::StatusCode::RequestTimeout)
            .with_sub_status(SubStatusCode::CLIENT_OPERATION_TIMEOUT.value());
        make_synthetic_error(status, "operation timeout")
    }

    #[test]
    fn null_handle_returns_safe_defaults() {
        assert_eq!(cosmos_error_status_code(std::ptr::null()), 0);
        assert_eq!(cosmos_error_sub_status(std::ptr::null()), -1);
        assert!(!cosmos_error_is_from_wire(std::ptr::null()));
        assert!(cosmos_error_message(std::ptr::null()).is_null());
        assert!(cosmos_error_activity_id(std::ptr::null()).is_null());
        assert!(cosmos_error_session_token(std::ptr::null()).is_null());
        assert!(cosmos_error_etag(std::ptr::null()).is_null());
        assert!(cosmos_error_backtrace(std::ptr::null()).is_null());
        assert_eq!(cosmos_error_retry_after_ms(std::ptr::null()), -1);
        // Freeing NULL is a no-op.
        cosmos_error_free(std::ptr::null_mut());
    }

    #[test]
    fn synthetic_client_timeout_fields() {
        let raw = synth_client_timeout();
        assert_eq!(cosmos_error_status_code(raw), 408);
        assert_eq!(cosmos_error_sub_status(raw), 20008);
        assert!(!cosmos_error_is_from_wire(raw));
        // Message is non-null and matches.
        let msg = unsafe { std::ffi::CStr::from_ptr(cosmos_error_message(raw)) }
            .to_string_lossy()
            .to_string();
        assert!(msg.contains("operation timeout"), "got: {msg}");
        cosmos_error_free(raw);
    }

    #[test]
    fn synthetic_client_error_has_no_wire_headers() {
        let status = CosmosStatus::new(azure_core::http::StatusCode::NotFound);
        let raw = make_synthetic_error(status, "synthetic not found");
        // is_from_wire is true only when a CosmosResponse is attached; we
        // build with no response so it must be false.
        assert!(!cosmos_error_is_from_wire(raw));
        // Wire-only convenience accessors all return NULL / -1.
        assert!(cosmos_error_activity_id(raw).is_null());
        assert!(cosmos_error_session_token(raw).is_null());
        assert!(cosmos_error_etag(raw).is_null());
        assert_eq!(cosmos_error_retry_after_ms(raw), -1);
        cosmos_error_free(raw);
    }

    #[test]
    fn from_driver_error_routes_synthetic_codes() {
        // Synthetic transport.
        let transport = DriverCosmosError::builder()
            .with_status(
                CosmosStatus::new(azure_core::http::StatusCode::ServiceUnavailable)
                    .with_sub_status(SubStatusCode::TRANSPORT_GENERATED_503.value()),
            )
            .with_message("synthetic transport failure")
            .build();
        assert_eq!(
            CosmosErrorCode::from_driver_error(&transport),
            CosmosErrorCode::CosmosErrorCodeTransportFailure
        );

        // Synthetic client timeout.
        let timeout = DriverCosmosError::builder()
            .with_status(
                CosmosStatus::new(azure_core::http::StatusCode::RequestTimeout)
                    .with_sub_status(SubStatusCode::CLIENT_OPERATION_TIMEOUT.value()),
            )
            .with_message("op timeout")
            .build();
        assert_eq!(
            CosmosErrorCode::from_driver_error(&timeout),
            CosmosErrorCode::CosmosErrorCodeClientOperationTimeout
        );

        // Synthetic generic client error.
        let generic_client = DriverCosmosError::builder()
            .with_status(CosmosStatus::new(azure_core::http::StatusCode::NotFound))
            .with_message("synthetic")
            .build();
        assert_eq!(
            CosmosErrorCode::from_driver_error(&generic_client),
            CosmosErrorCode::CosmosErrorCodeClientError
        );
    }

    #[test]
    fn error_code_band_assignments() {
        // Sanity-check that every variant lands in the band the spec promises.
        assert_eq!(CosmosErrorCode::CosmosErrorCodeSuccess.as_i32(), 0);
        assert!((1..=999).contains(&CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()));
        assert!((2001..=2999).contains(&CosmosErrorCode::CosmosErrorCodeNotFound.as_i32()));
        assert!((2001..=2999).contains(&CosmosErrorCode::CosmosErrorCodeThrottled.as_i32()));
        assert!((3001..=3999).contains(&CosmosErrorCode::CosmosErrorCodeTransportFailure.as_i32()));
        assert!((3001..=3999)
            .contains(&CosmosErrorCode::CosmosErrorCodeClientOperationTimeout.as_i32()));
        assert!((4001..=4999).contains(&CosmosErrorCode::CosmosErrorCodeQueueShutdown.as_i32()));
        assert!(
            (4001..=4999).contains(&CosmosErrorCode::CosmosErrorCodeOperationCancelled.as_i32())
        );
        assert!((4001..=4999).contains(&CosmosErrorCode::CosmosErrorCodeQueueFull.as_i32()));
        assert!((5001..=5999)
            .contains(&CosmosErrorCode::CosmosErrorCodeOptionsIgnoredOnCacheHit.as_i32()));
    }
}
