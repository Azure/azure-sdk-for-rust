// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for building a `cosmos_runtime_t` — the wrapper-side bridge to
//! [`azure_data_cosmos_driver::driver::CosmosDriverRuntimeBuilder`].
//!
//! Construction is a single flat call: the host fills a
//! [`CosmosRuntimeOptions`] `#[repr(C)]` struct (sentinel-encoded, so a zeroed
//! value / NULL pointer means "driver defaults for everything") and hands it to
//! [`cosmos_runtime_build`], which validates each set field, bridges the
//! driver-side build through the wrapper's own Tokio runtime, and returns a
//! fresh `cosmos_runtime_t *`. Complex nested config (`with_client_options` /
//! `with_connection_pool` / `with_operation_options` /
//! `register_throughput_control_group` / `with_fault_injection_rules`) is
//! deliberately not surfaced yet — each requires its own flat options struct.
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] section 4.1.
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::ffi::{c_char, CStr};
use std::time::Duration;

use azure_data_cosmos_driver::driver::CosmosDriverRuntimeBuilder;
use azure_data_cosmos_driver::options::{CorrelationId, UserAgentSuffix, WorkloadId};

use crate::error::{CosmosErrorCode, CosmosErrorHandle};
use crate::runtime::RuntimeContext;

// ─────────────────────────────────────────────────────────────────────────────
// CPU-refresh-interval sanity range (mirrors the doc range on
// `CosmosDriverRuntimeBuilder::with_cpu_refresh_interval`). The merged
// driver does not enforce this itself — we surface invalid values via
// `INVALID_OPTION_VALUE` so external SDKs get an early, deterministic error
// instead of opaque behavior down the line.
// ─────────────────────────────────────────────────────────────────────────────

const CPU_REFRESH_INTERVAL_MIN_MS: u64 = 1_000;
const CPU_REFRESH_INTERVAL_MAX_MS: u64 = 60_000;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Converts a NUL-terminated UTF-8 string from C into a Rust `&str` view.
///
/// Returns `Err(INVALID_ARGUMENT)` for NULL, `Err(INVALID_UTF8)` for
/// non-UTF-8 input. The borrow lives for the duration of the FFI call.
fn try_cstr_to_str<'a>(p: *const c_char) -> Result<&'a str, CosmosErrorCode> {
    if p.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: `p` is non-NULL and the caller guarantees it points at a
    // NUL-terminated C string (FFI contract — documented on every setter).
    let cstr = unsafe { CStr::from_ptr(p) };
    cstr.to_str()
        .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
}

// ─────────────────────────────────────────────────────────────────────────────
// Runtime build result mapping
// ─────────────────────────────────────────────────────────────────────────────

/// Maps the outcome of [`RuntimeContext::new_with_builder`] onto the FFI
/// `(out_runtime, out_error)` contract used by [`cosmos_runtime_build`].
///
/// On success writes the runtime handle to `*out_runtime` and returns
/// `SUCCESS`. On failure returns `RUNTIME_BUILD_FAILED` and, when
/// `out_error` is non-NULL, writes a rich `cosmos_error_t *` describing the
/// failure. Callers must null-check `out_runtime` before calling.
fn finish_runtime_build(
    result: Result<*mut RuntimeContext, RuntimeBuildError>,
    out_runtime: *mut *mut RuntimeContext,
    out_error: *mut *mut CosmosErrorHandle,
) -> i32 {
    match result {
        Ok(ptr) => {
            // SAFETY: caller guarantees `out_runtime` is non-NULL and writable
            // for one `*mut RuntimeContext`.
            unsafe { *out_runtime = ptr };
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        }
        Err(RuntimeBuildError::TokioInit(io)) => {
            // The wrapper-side Tokio runtime couldn't be constructed — an
            // OS-level resource failure (thread limit, file descriptor
            // limit). Map to the driver's `TRANSPORT_IO_FAILED` status
            // (the closest existing "wrapper-side resource issue"
            // classification) so callers can log it through the same
            // accessor surface as a real driver-side failure.
            if !out_error.is_null() {
                let driver_err = azure_data_cosmos_driver::error::CosmosError::builder()
                    .with_status(azure_data_cosmos_driver::error::CosmosStatus::TRANSPORT_IO_FAILED)
                    .with_message(format!("wrapper Tokio runtime build failed: {io}"))
                    .build();
                // SAFETY: caller guarantees `out_error` is writable for one
                // `*mut CosmosErrorHandle`.
                unsafe { *out_error = CosmosErrorHandle::into_raw(driver_err) };
            }
            CosmosErrorCode::CosmosErrorCodeRuntimeBuildFailed.as_i32()
        }
        Err(RuntimeBuildError::Driver(driver_err)) => {
            if !out_error.is_null() {
                // SAFETY: caller guarantees `out_error` is writable for one
                // `*mut CosmosErrorHandle`.
                unsafe { *out_error = CosmosErrorHandle::into_raw(driver_err) };
            }
            CosmosErrorCode::CosmosErrorCodeRuntimeBuildFailed.as_i32()
        }
    }
}

/// Internal error type for [`RuntimeContext::new_with_builder`].
pub(crate) enum RuntimeBuildError {
    TokioInit(std::io::Error),
    Driver(azure_data_cosmos_driver::error::CosmosError),
}

// ─────────────────────────────────────────────────────────────────────────────
// Flat single-call construction (cosmos_runtime_options_t / cosmos_runtime_build)
//
// Per docs/DATA_MOVEMENT_MODEL.md: a host fills out one flat `#[repr(C)]`
// struct in its own language and hands it across the boundary in a single
// `cosmos_runtime_build` call. This is the only runtime-construction surface —
// the per-field incremental builder was removed in P5 (no back-compat).
// ─────────────────────────────────────────────────────────────────────────────

/// Flat C ABI options for building a `cosmos_runtime_t` in a single call.
///
/// Every field is sentinel-encoded so a zeroed struct (or a NULL pointer
/// passed to [`cosmos_runtime_build`]) means "use the driver defaults for
/// everything":
///
/// - `workload_id`: `0` = unset (valid range otherwise `1`–`50`).
/// - `correlation_id` / `user_agent_suffix` / `wrapping_sdk_identifier`:
///   NULL = unset (otherwise a NUL-terminated UTF-8 string).
/// - `cpu_refresh_interval_ms`: `0` = unset (valid range otherwise
///   `1000`–`60000`).
///
/// Construct with [`cosmos_runtime_options_default`] to obtain an all-unset
/// value, then set the fields you care about.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CosmosRuntimeOptions {
    /// Workload identifier (valid range `1`–`50`). `0` = unset.
    pub workload_id: u8,
    /// Correlation id for client-side metrics (NUL-terminated UTF-8), or NULL
    /// = unset.
    pub correlation_id: *const c_char,
    /// User-agent suffix (NUL-terminated UTF-8), or NULL = unset.
    pub user_agent_suffix: *const c_char,
    /// Wrapping-SDK identifier prepended to the User-Agent header
    /// (NUL-terminated UTF-8), or NULL = unset.
    pub wrapping_sdk_identifier: *const c_char,
    /// CPU/memory monitoring refresh interval in milliseconds (valid range
    /// `1000`–`60000`). `0` = unset.
    pub cpu_refresh_interval_ms: u64,
}

impl CosmosRuntimeOptions {
    /// Applies every *set* field to `builder`, validating each with the same
    /// rules as the individual setters. Unset (sentinel) fields are left at the
    /// driver default.
    ///
    /// # Safety
    ///
    /// Each non-NULL string pointer must reference a valid NUL-terminated UTF-8
    /// string for the duration of the call.
    unsafe fn apply_to(
        &self,
        mut builder: CosmosDriverRuntimeBuilder,
    ) -> Result<CosmosDriverRuntimeBuilder, CosmosErrorCode> {
        if self.workload_id != 0 {
            let Some(value) = WorkloadId::try_new(self.workload_id) else {
                return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
            };
            builder = builder.with_workload_id(value);
        }
        if !self.correlation_id.is_null() {
            let value = try_cstr_to_str(self.correlation_id)?;
            let Some(parsed) = CorrelationId::try_new(value) else {
                return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
            };
            builder = builder.with_correlation_id(parsed);
        }
        if !self.user_agent_suffix.is_null() {
            let value = try_cstr_to_str(self.user_agent_suffix)?;
            let Some(parsed) = UserAgentSuffix::try_new(value) else {
                return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
            };
            builder = builder.with_user_agent_suffix(parsed);
        }
        if !self.wrapping_sdk_identifier.is_null() {
            let value = try_cstr_to_str(self.wrapping_sdk_identifier)?;
            builder = builder.with_wrapping_sdk_identifier(value);
        }
        if self.cpu_refresh_interval_ms != 0 {
            if !(CPU_REFRESH_INTERVAL_MIN_MS..=CPU_REFRESH_INTERVAL_MAX_MS)
                .contains(&self.cpu_refresh_interval_ms)
            {
                return Err(CosmosErrorCode::CosmosErrorCodeInvalidOptionValue);
            }
            builder = builder
                .with_cpu_refresh_interval(Duration::from_millis(self.cpu_refresh_interval_ms));
        }
        Ok(builder)
    }
}

/// Returns an all-unset [`CosmosRuntimeOptions`] by value. The host mutates the
/// fields it cares about and leaves the rest at their default sentinels.
#[no_mangle]
pub extern "C" fn cosmos_runtime_options_default() -> CosmosRuntimeOptions {
    CosmosRuntimeOptions {
        workload_id: 0,
        correlation_id: std::ptr::null(),
        user_agent_suffix: std::ptr::null(),
        wrapping_sdk_identifier: std::ptr::null(),
        cpu_refresh_interval_ms: 0,
    }
}

/// Builds a `cosmos_runtime_t *` from a flat [`CosmosRuntimeOptions`] in a
/// single call.
///
/// # Parameters
///
/// - `options` — the flat options. A NULL pointer uses the driver defaults for
///   every field (equivalent to [`cosmos_runtime_options_default`]).
/// - `out_runtime` — on success, receives the new runtime handle. Must be
///   non-NULL.
/// - `out_error` — optional. On `RUNTIME_BUILD_FAILED`, receives a rich
///   `cosmos_error_t *` describing the driver-side failure. If NULL the rich
///   error is dropped. Never populated on success.
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_runtime` populated.
/// - `INVALID_ARGUMENT` (1) when `out_runtime` is NULL.
/// - `INVALID_UTF8` (2) when a string field is not valid UTF-8.
/// - `INVALID_OPTION_VALUE` (4014) when a field is outside its documented
///   range.
/// - `RUNTIME_BUILD_FAILED` (4015) when the underlying build failed;
///   `*out_error` is populated when non-NULL.
#[no_mangle]
pub extern "C" fn cosmos_runtime_build(
    options: *const CosmosRuntimeOptions,
    out_runtime: *mut *mut RuntimeContext,
    out_error: *mut *mut CosmosErrorHandle,
) -> i32 {
    if out_runtime.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let mut builder = CosmosDriverRuntimeBuilder::new();
    if !options.is_null() {
        // SAFETY: caller guarantees `options` points at a valid
        // `CosmosRuntimeOptions` whose string fields are valid NUL-terminated
        // UTF-8 for the duration of the call.
        builder = match unsafe { (*options).apply_to(builder) } {
            Ok(b) => b,
            Err(code) => return code.as_i32(),
        };
    }
    finish_runtime_build(
        RuntimeContext::new_with_builder(builder),
        out_runtime,
        out_error,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    fn ok_cstr(s: &str) -> CString {
        CString::new(s).expect("test inputs must be NUL-free")
    }

    // ── Flat single-call construction (cosmos_runtime_build) ──

    #[test]
    fn runtime_options_default_is_all_unset() {
        let o = cosmos_runtime_options_default();
        assert_eq!(o.workload_id, 0);
        assert!(o.correlation_id.is_null());
        assert!(o.user_agent_suffix.is_null());
        assert!(o.wrapping_sdk_identifier.is_null());
        assert_eq!(o.cpu_refresh_interval_ms, 0);
    }

    #[test]
    fn flat_build_rejects_null_out_runtime() {
        let opts = cosmos_runtime_options_default();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();
        let code = cosmos_runtime_build(&opts, ptr::null_mut(), &mut err);
        assert_eq!(
            code,
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert!(err.is_null());
    }

    #[test]
    fn flat_build_validates_fields() {
        // Out-of-range workload id.
        let mut opts = cosmos_runtime_options_default();
        opts.workload_id = 51;
        let mut runtime: *mut RuntimeContext = ptr::null_mut();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();
        assert_eq!(
            cosmos_runtime_build(&opts, &mut runtime, &mut err),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert!(runtime.is_null());

        // Invalid correlation id (contains a space).
        let bad = ok_cstr("has space");
        let mut opts = cosmos_runtime_options_default();
        opts.correlation_id = bad.as_ptr();
        assert_eq!(
            cosmos_runtime_build(&opts, &mut runtime, &mut err),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert!(runtime.is_null());

        // Out-of-range cpu refresh interval.
        let mut opts = cosmos_runtime_options_default();
        opts.cpu_refresh_interval_ms = 999;
        assert_eq!(
            cosmos_runtime_build(&opts, &mut runtime, &mut err),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert!(runtime.is_null());
    }

    #[test]
    fn flat_build_null_options_uses_defaults_and_builds() {
        let mut runtime: *mut RuntimeContext = ptr::null_mut();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();
        // A NULL options pointer means "all driver defaults".
        let code = cosmos_runtime_build(ptr::null(), &mut runtime, &mut err);
        assert_eq!(code, CosmosErrorCode::CosmosErrorCodeSuccess.as_i32());
        assert!(!runtime.is_null());
        assert!(err.is_null());
        crate::runtime::cosmos_runtime_free(runtime);
    }

    #[test]
    fn flat_build_with_options_builds_and_is_usable() {
        // Configure a couple of fields through the flat struct and confirm the
        // resulting runtime is usable by creating a queue against it — the
        // single-call analog of `build_produces_runtime_compatible_with_cq`.
        let ua = ok_cstr("driver-native-flat-tests");
        let mut opts = cosmos_runtime_options_default();
        opts.workload_id = 7;
        opts.user_agent_suffix = ua.as_ptr();
        opts.cpu_refresh_interval_ms = 5_000;

        let mut runtime: *mut RuntimeContext = ptr::null_mut();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();
        let code = cosmos_runtime_build(&opts, &mut runtime, &mut err);
        assert_eq!(
            code,
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32(),
            "flat build must succeed for valid options"
        );
        assert!(!runtime.is_null());
        assert!(err.is_null());

        let cq = crate::completion::cosmos_completion_queue_create(runtime, std::ptr::null());
        assert!(
            !cq.is_null(),
            "completion_queue_create must accept a flat-built runtime"
        );
        crate::completion::cosmos_completion_queue_free(cq);

        crate::runtime::cosmos_runtime_free(runtime);
    }
}
