// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_runtime_builder_t` — the wrapper-side mirror of
//! [`azure_data_cosmos_driver::driver::CosmosDriverRuntimeBuilder`].
//!
//! Phase 2 ships the smallest useful builder surface: primitive setters that
//! correspond to **actually-existing** simple setters on the merged driver
//! builder, plus `_build` (async-bridged through the wrapper's own Tokio
//! runtime). Complex nested setters
//! (`with_client_options` / `with_connection_pool` /
//! `with_operation_options` / `register_throughput_control_group` /
//! `with_fault_injection_rules`) are intentional Phase 2+ follow-ups — each
//! requires its own FFI builder surface, which would dwarf this commit if
//! folded in here.
//!
//! The setters mutate in-place because builder-pattern chaining is awkward
//! across an FFI boundary; callers create, configure, then `_build` (which
//! consumes the builder).
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] §4.1 + §8 Phase 2.
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
// RuntimeBuilderHandle — opaque storage pun
// ─────────────────────────────────────────────────────────────────────────────

/// Internal storage owned by every `cosmos_runtime_builder_t *`.
///
/// We carry the driver's `CosmosDriverRuntimeBuilder` directly. Setters
/// take `&mut`, but `with_*` on the driver builder consume `self` by value,
/// so each setter does a `mem::take(&mut inner.builder)` → call → store
/// dance. `Default` on the builder is cheap (`Self::default()`), so the
/// take/replace is just two moves.
pub(crate) struct RuntimeBuilderInner {
    pub(crate) builder: CosmosDriverRuntimeBuilder,
}

/// Opaque C ABI handle for a runtime builder.
///
/// Storage pun: see the matching pattern on [`RuntimeContext`] in
/// [`crate::runtime`]. The public `#[repr(C)]` struct only carries the
/// `_opaque` marker; the real state lives in the trailing
/// `RuntimeBuilderStorage` field.
#[repr(C)]
pub struct RuntimeBuilderHandle {
    _opaque: [u8; 0],
}

#[repr(C)]
struct RuntimeBuilderStorage {
    _opaque: [u8; 0],
    inner: RuntimeBuilderInner,
}

impl RuntimeBuilderHandle {
    fn new_raw() -> *mut Self {
        let storage = Box::new(RuntimeBuilderStorage {
            _opaque: [],
            inner: RuntimeBuilderInner {
                builder: CosmosDriverRuntimeBuilder::new(),
            },
        });
        // SAFETY: both structs share the `_opaque: [u8; 0]` first-field
        // marker so the pointer is interchangeable across the FFI boundary.
        Box::into_raw(storage).cast::<RuntimeBuilderHandle>()
    }

    fn inner_mut<'a>(p: *mut RuntimeBuilderHandle) -> Option<&'a mut RuntimeBuilderInner> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `new_raw` and not
        // yet freed. Lifetime is scoped to the FFI call.
        let storage = unsafe { &mut *(p.cast::<RuntimeBuilderStorage>()) };
        Some(&mut storage.inner)
    }

    fn drop_raw(p: *mut RuntimeBuilderHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: pun back into the `Box<RuntimeBuilderStorage>` we
        // originally allocated.
        unsafe {
            drop(Box::from_raw(p.cast::<RuntimeBuilderStorage>()));
        }
    }

    /// Consumes a `*mut RuntimeBuilderHandle`, returning the owned builder.
    ///
    /// Used by [`cosmos_runtime_builder_build`] to take the builder out of
    /// its FFI storage and pass it by value to
    /// `CosmosDriverRuntimeBuilder::build`.
    fn into_owned_builder(p: *mut RuntimeBuilderHandle) -> Option<CosmosDriverRuntimeBuilder> {
        if p.is_null() {
            return None;
        }
        // SAFETY: pun back into the `Box<RuntimeBuilderStorage>` we
        // originally allocated and move the inner builder out.
        let storage = unsafe { Box::from_raw(p.cast::<RuntimeBuilderStorage>()) };
        Some(storage.inner.builder)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: lifecycle
// ─────────────────────────────────────────────────────────────────────────────

/// Lifecycle: allocate a new `cosmos_runtime_builder_t`.
///
/// The returned handle must be freed via [`cosmos_runtime_builder_free`] if
/// `cosmos_runtime_builder_build` is never called on it. Successful
/// `_build` consumes the handle.
#[no_mangle]
pub extern "C" fn cosmos_runtime_builder_new() -> *mut RuntimeBuilderHandle {
    RuntimeBuilderHandle::new_raw()
}

/// Lifecycle: free a `cosmos_runtime_builder_t *` that was never consumed
/// by [`cosmos_runtime_builder_build`].
///
/// Calling `_free` on a builder after a successful `_build` is undefined
/// behavior. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_runtime_builder_free(builder: *mut RuntimeBuilderHandle) {
    if builder.is_null() {
        return;
    }
    tracing::trace!(?builder, "freeing cosmos_runtime_builder_t");
    RuntimeBuilderHandle::drop_raw(builder);
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: setters
//
// Common contract:
//
// - `INVALID_ARGUMENT` (1) — `builder` is NULL.
// - `INVALID_UTF8`     (2) — for `*const c_char` setters, the supplied
//   string contained invalid UTF-8.
// - `INVALID_OPTION_VALUE` (4014) — the supplied value was outside the
//   driver-side type's documented validation range (`WorkloadId` 1-50,
//   `CorrelationId` ≤50 chars + HTTP-header-safe, `UserAgentSuffix`
//   ≤25 chars + HTTP-header-safe, cpu refresh interval
//   1000–60000 ms).
//
// The setters mutate the builder in place. They do not chain because
// chaining `Self` across the FFI boundary requires the caller to thread
// the new pointer back through — error-prone and offers no benefit over
// in-place mutation.
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

/// Sets the workload identifier.
///
/// Valid range: 1–50. Out-of-range values return `INVALID_OPTION_VALUE`
/// with no mutation to the builder.
#[no_mangle]
pub extern "C" fn cosmos_runtime_builder_with_workload_id(
    builder: *mut RuntimeBuilderHandle,
    workload_id: u8,
) -> i32 {
    let Some(inner) = RuntimeBuilderHandle::inner_mut(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let Some(value) = WorkloadId::try_new(workload_id) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32();
    };
    let taken = std::mem::take(&mut inner.builder);
    inner.builder = taken.with_workload_id(value);
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Sets the correlation ID for client-side metrics.
///
/// Constraints: ≤50 characters, HTTP-header-safe (alphanumeric, hyphen,
/// underscore, dot, tilde). Strings outside this contract return
/// `INVALID_OPTION_VALUE` with no mutation.
#[no_mangle]
pub extern "C" fn cosmos_runtime_builder_with_correlation_id(
    builder: *mut RuntimeBuilderHandle,
    correlation_id: *const c_char,
) -> i32 {
    let Some(inner) = RuntimeBuilderHandle::inner_mut(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let value = match try_cstr_to_str(correlation_id) {
        Ok(v) => v,
        Err(code) => return code.as_i32(),
    };
    let Some(parsed) = CorrelationId::try_new(value) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32();
    };
    let taken = std::mem::take(&mut inner.builder);
    inner.builder = taken.with_correlation_id(parsed);
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Sets the user-agent suffix.
///
/// Constraints: ≤25 characters, HTTP-header-safe (alphanumeric, hyphen,
/// underscore, dot, tilde). Strings outside this contract return
/// `INVALID_OPTION_VALUE` with no mutation.
#[no_mangle]
pub extern "C" fn cosmos_runtime_builder_with_user_agent_suffix(
    builder: *mut RuntimeBuilderHandle,
    suffix: *const c_char,
) -> i32 {
    let Some(inner) = RuntimeBuilderHandle::inner_mut(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let value = match try_cstr_to_str(suffix) {
        Ok(v) => v,
        Err(code) => return code.as_i32(),
    };
    let Some(parsed) = UserAgentSuffix::try_new(value) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32();
    };
    let taken = std::mem::take(&mut inner.builder);
    inner.builder = taken.with_user_agent_suffix(parsed);
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Sets a wrapping-SDK identifier prepended to the User-Agent header.
///
/// Per the driver contract, the value is sanitized server-side (non-ASCII
/// stripped, whitespace trimmed); empty / whitespace-only is treated as
/// "unset". The FFI does not pre-validate the contents — any UTF-8 string
/// is accepted and forwarded to the driver's normalizer.
#[no_mangle]
pub extern "C" fn cosmos_runtime_builder_with_wrapping_sdk_identifier(
    builder: *mut RuntimeBuilderHandle,
    identifier: *const c_char,
) -> i32 {
    let Some(inner) = RuntimeBuilderHandle::inner_mut(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let value = match try_cstr_to_str(identifier) {
        Ok(v) => v,
        Err(code) => return code.as_i32(),
    };
    let taken = std::mem::take(&mut inner.builder);
    inner.builder = taken.with_wrapping_sdk_identifier(value);
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Sets the CPU/memory monitoring refresh interval (milliseconds).
///
/// Valid range: 1000–60000 ms (1–60 seconds). Out-of-range values return
/// `INVALID_OPTION_VALUE` with no mutation.
///
/// The FFI rejects values outside the documented range up-front — even
/// though the merged driver does not itself validate, surfacing the
/// constraint here gives external language SDKs an early deterministic
/// error rather than opaque behavior at the first sampling tick.
#[no_mangle]
pub extern "C" fn cosmos_runtime_builder_with_cpu_refresh_interval_ms(
    builder: *mut RuntimeBuilderHandle,
    interval_ms: u64,
) -> i32 {
    let Some(inner) = RuntimeBuilderHandle::inner_mut(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    if !(CPU_REFRESH_INTERVAL_MIN_MS..=CPU_REFRESH_INTERVAL_MAX_MS).contains(&interval_ms) {
        return CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32();
    }
    let taken = std::mem::take(&mut inner.builder);
    inner.builder = taken.with_cpu_refresh_interval(Duration::from_millis(interval_ms));
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: build
// ─────────────────────────────────────────────────────────────────────────────

/// Consumes the builder, constructs the underlying
/// [`azure_data_cosmos_driver::driver::CosmosDriverRuntime`], and returns it
/// as a fresh `cosmos_runtime_t *`.
///
/// # Lifetime
///
/// `cosmos_runtime_builder_build` **consumes** the builder regardless of
/// success or failure. Callers must NOT call
/// [`cosmos_runtime_builder_free`] on the same pointer afterwards.
///
/// # Parameters
///
/// - `builder` — the builder to consume. Must be non-NULL.
/// - `out_runtime` — on success, receives the new runtime handle. Must be
///   non-NULL.
/// - `out_error` — optional. On `RUNTIME_BUILD_FAILED`, receives a rich
///   `cosmos_error_t *` describing the driver-side failure. If NULL the
///   rich error is dropped. The slot is never populated on success.
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_runtime` populated.
/// - `INVALID_ARGUMENT` (1) when `builder` or `out_runtime` is NULL. In
///   the `builder == NULL` case nothing is freed; in the `out_runtime ==
///   NULL` case the builder is still consumed (the driver-side builder
///   has been moved out and a fresh runtime would otherwise leak).
/// - `RUNTIME_BUILD_FAILED` (4015) when the underlying
///   `CosmosDriverRuntimeBuilder::build()` returned an error. `*out_error`
///   is populated when non-NULL.
#[no_mangle]
pub extern "C" fn cosmos_runtime_builder_build(
    builder: *mut RuntimeBuilderHandle,
    out_runtime: *mut *mut RuntimeContext,
    out_error: *mut *mut CosmosErrorHandle,
) -> i32 {
    if builder.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    // Always consume the builder. If `out_runtime` is NULL we still take
    // ownership so the inner allocation doesn't leak — the driver build is
    // then a no-op because we early-return below.
    let Some(driver_builder) = RuntimeBuilderHandle::into_owned_builder(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    if out_runtime.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }

    match RuntimeContext::new_with_builder(driver_builder) {
        Ok(ptr) => {
            // SAFETY: caller guarantees `out_runtime` is writable for one
            // `*mut RuntimeContext`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    fn ok_cstr(s: &str) -> CString {
        CString::new(s).expect("test inputs must be NUL-free")
    }

    #[test]
    fn new_and_free_default_builder() {
        let b = cosmos_runtime_builder_new();
        assert!(!b.is_null());
        cosmos_runtime_builder_free(b);
    }

    #[test]
    fn free_handles_null() {
        cosmos_runtime_builder_free(ptr::null_mut());
    }

    #[test]
    fn setters_reject_null_builder() {
        let s = ok_cstr("foo");
        assert_eq!(
            cosmos_runtime_builder_with_workload_id(ptr::null_mut(), 1),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_correlation_id(ptr::null_mut(), s.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_user_agent_suffix(ptr::null_mut(), s.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_wrapping_sdk_identifier(ptr::null_mut(), s.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_cpu_refresh_interval_ms(ptr::null_mut(), 5000),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
    }

    #[test]
    fn workload_id_validates_range() {
        let b = cosmos_runtime_builder_new();
        assert_eq!(
            cosmos_runtime_builder_with_workload_id(b, 1),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_workload_id(b, 50),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_workload_id(b, 0),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_workload_id(b, 51),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_workload_id(b, 255),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        cosmos_runtime_builder_free(b);
    }

    #[test]
    fn correlation_id_validates_input() {
        let b = cosmos_runtime_builder_new();
        let ok = ok_cstr("aks-prod-eastus-001");
        let too_long = ok_cstr(&"x".repeat(51));
        let invalid_char = ok_cstr("has space");
        let valid_edge = ok_cstr(&"x".repeat(50));

        assert_eq!(
            cosmos_runtime_builder_with_correlation_id(b, ok.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_correlation_id(b, valid_edge.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_correlation_id(b, too_long.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_correlation_id(b, invalid_char.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_correlation_id(b, ptr::null()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        cosmos_runtime_builder_free(b);
    }

    #[test]
    fn user_agent_suffix_validates_input() {
        let b = cosmos_runtime_builder_new();
        let ok = ok_cstr("myapp-westus2");
        let too_long = ok_cstr(&"x".repeat(26));
        let valid_edge = ok_cstr(&"x".repeat(25));
        let invalid_char = ok_cstr("bad@suffix");

        assert_eq!(
            cosmos_runtime_builder_with_user_agent_suffix(b, ok.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_user_agent_suffix(b, valid_edge.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_user_agent_suffix(b, too_long.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        assert_eq!(
            cosmos_runtime_builder_with_user_agent_suffix(b, invalid_char.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32()
        );
        cosmos_runtime_builder_free(b);
    }

    #[test]
    fn wrapping_sdk_identifier_accepts_any_utf8() {
        let b = cosmos_runtime_builder_new();
        let token = ok_cstr("azsdk-rust-cosmos/0.34.0");
        let empty = ok_cstr("");

        assert_eq!(
            cosmos_runtime_builder_with_wrapping_sdk_identifier(b, token.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        // Empty / whitespace-only is treated as "unset" by the driver
        // normalizer, but still SUCCESS at the FFI surface.
        assert_eq!(
            cosmos_runtime_builder_with_wrapping_sdk_identifier(b, empty.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        cosmos_runtime_builder_free(b);
    }

    #[test]
    fn cpu_refresh_interval_validates_range() {
        let b = cosmos_runtime_builder_new();
        // Valid edges and a typical middle value.
        for v in [
            CPU_REFRESH_INTERVAL_MIN_MS,
            5_000,
            CPU_REFRESH_INTERVAL_MAX_MS,
        ] {
            assert_eq!(
                cosmos_runtime_builder_with_cpu_refresh_interval_ms(b, v),
                CosmosErrorCode::CosmosErrorCodeSuccess.as_i32(),
                "value {v} should be SUCCESS"
            );
        }
        for v in [
            0,
            CPU_REFRESH_INTERVAL_MIN_MS - 1,
            CPU_REFRESH_INTERVAL_MAX_MS + 1,
            u64::MAX,
        ] {
            assert_eq!(
                cosmos_runtime_builder_with_cpu_refresh_interval_ms(b, v),
                CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_i32(),
                "value {v} should be INVALID_OPTION_VALUE"
            );
        }
        cosmos_runtime_builder_free(b);
    }

    #[test]
    fn build_rejects_null_builder() {
        let mut out: *mut RuntimeContext = ptr::null_mut();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();
        let code = cosmos_runtime_builder_build(ptr::null_mut(), &mut out, &mut err);
        assert_eq!(
            code,
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert!(out.is_null());
        assert!(err.is_null());
    }

    #[test]
    fn build_rejects_null_out_runtime() {
        let b = cosmos_runtime_builder_new();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();
        let code = cosmos_runtime_builder_build(b, ptr::null_mut(), &mut err);
        // INVALID_ARGUMENT — the builder is consumed regardless to avoid
        // leaking the inner allocation, so a subsequent `_free` of `b` is
        // undefined behavior (matches the documented contract).
        assert_eq!(
            code,
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert!(err.is_null());
    }

    #[test]
    fn build_produces_runtime_compatible_with_cq() {
        // End-to-end: build a runtime via the builder, then create a
        // completion queue against it. Mirrors the production sequence
        // and proves the wrapper-side Tokio runtime + driver runtime are
        // wired together correctly.
        let b = cosmos_runtime_builder_new();
        let ua = ok_cstr("driver-native-tests");
        assert_eq!(
            cosmos_runtime_builder_with_user_agent_suffix(b, ua.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );

        let mut runtime: *mut RuntimeContext = ptr::null_mut();
        let mut err: *mut CosmosErrorHandle = ptr::null_mut();
        let code = cosmos_runtime_builder_build(b, &mut runtime, &mut err);
        assert_eq!(
            code,
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32(),
            "build must succeed for a minimally configured builder"
        );
        assert!(!runtime.is_null());
        assert!(err.is_null());

        // Verify the runtime is usable by creating a queue against it.
        let cq = crate::completion::cosmos_cq_create(runtime, std::ptr::null());
        assert!(
            !cq.is_null(),
            "cq_create must accept a builder-built runtime"
        );
        crate::completion::cosmos_cq_free(cq);

        crate::runtime::cosmos_runtime_free(runtime);
    }
}
