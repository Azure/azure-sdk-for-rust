// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! C ABI surface for `cosmos_operation_options_t` +
//! `cosmos_operation_options_builder_t` — wraps the driver's
//! [`azure_data_cosmos_driver::options::OperationOptions`] and its
//! auto-generated `OperationOptionsBuilder`.
//!
//! `OperationOptions` carries 16 cross-cutting per-operation settings
//! (consistency, regions, content-response-on-write, throughput control,
//! end-to-end timeout, retry counts, session capture, per-partition
//! circuit-breaker tuning, and custom headers). Every field is
//! `Option<T>`; `None` means "inherit from the higher-priority layer"
//! (operation → account → runtime → environment per the driver's
//! layered-resolution model).
//!
//! The FFI mirrors that semantic with paired `_with_<field>` / `_clear_<field>`
//! setters: `_with_*` sets `Some(value)`, `_clear_*` sets back to `None`.
//! Cross-language SDKs that distinguish "user explicitly cleared" from
//! "never set" therefore have the same expressive power as Rust callers
//! (and Phase 3's `cosmos_driver_options_builder_with_operation_options`
//! can carry a partially-cleared options bag through to the driver
//! without re-introducing inherited defaults).
//!
//! Setters mutate the builder in place — `with_*` on the driver builder
//! consume `self`, so each FFI setter does a `.take()` / call / restore
//! dance against an `Option<OperationOptionsBuilder>` slot.
//!
//! See [`docs/NATIVE_WRAPPER_SPEC.md`] §4.6 (the per-operation `with_*`
//! mutators are exposed on `cosmos_operation_t`; THIS surface covers the
//! shared options bag those mutators *do not* cover).
//!
//! [`docs/NATIVE_WRAPPER_SPEC.md`]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md

use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::sync::Arc;
use std::time::Duration;

use azure_core::http::headers::{HeaderName, HeaderValue};
use azure_data_cosmos_driver::models::ThroughputControlGroupName;
use azure_data_cosmos_driver::options::{
    ContentResponseOnWrite, EndToEndOperationLatencyPolicy, ExcludedRegions, OperationOptions,
    OperationOptionsBuilder, ReadConsistencyStrategy, Region,
};

use crate::error::CosmosErrorCode;

// ─────────────────────────────────────────────────────────────────────────────
// Wire-stable enums (re-exported through cbindgen)
// ─────────────────────────────────────────────────────────────────────────────

/// Per spec §4.6 — mirrors [`ReadConsistencyStrategy`].
///
/// Variant prefixes are baked into the Rust variant names so the
/// `ScreamingSnakeCase` cbindgen rule emits the spec-mandated
/// `COSMOS_READ_CONSISTENCY_*` constants without per-enum overrides
/// (matches Phase 1's `CosmosCqState` pattern).
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosReadConsistency {
    /// Use the default behavior for the chosen consistency level.
    CosmosReadConsistencyDefault = 0,
    /// Eventual consistency.
    CosmosReadConsistencyEventual = 1,
    /// Session consistency (the driver's typical default).
    CosmosReadConsistencySession = 2,
    /// Read the latest version across all regions
    /// (single-master / Strong-only).
    CosmosReadConsistencyGlobalStrong = 3,
}

impl CosmosReadConsistency {
    fn to_driver(self) -> ReadConsistencyStrategy {
        match self {
            Self::CosmosReadConsistencyDefault => ReadConsistencyStrategy::Default,
            Self::CosmosReadConsistencyEventual => ReadConsistencyStrategy::Eventual,
            Self::CosmosReadConsistencySession => ReadConsistencyStrategy::Session,
            Self::CosmosReadConsistencyGlobalStrong => ReadConsistencyStrategy::GlobalStrong,
        }
    }
}

/// Per spec §4.6 — mirrors [`ContentResponseOnWrite`].
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosContentResponseOnWrite {
    /// The driver's default — server returns no body on write responses.
    CosmosContentResponseOnWriteDisabled = 0,
    /// Server returns the written resource in the response body.
    CosmosContentResponseOnWriteEnabled = 1,
}

impl CosmosContentResponseOnWrite {
    fn to_driver(self) -> ContentResponseOnWrite {
        match self {
            Self::CosmosContentResponseOnWriteDisabled => ContentResponseOnWrite::Disabled,
            Self::CosmosContentResponseOnWriteEnabled => ContentResponseOnWrite::Enabled,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Built options handle
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) struct OperationOptionsInner {
    pub(crate) inner: OperationOptions,
}

/// Opaque C ABI handle for a built [`OperationOptions`] value.
///
/// Storage pun: same shape as `DriverOptionsHandle`.
#[repr(C)]
pub struct OperationOptionsHandle {
    _opaque: [u8; 0],
}

#[repr(C)]
struct OperationOptionsStorage {
    _opaque: [u8; 0],
    inner: Arc<OperationOptionsInner>,
}

impl OperationOptionsHandle {
    fn into_raw(inner: OperationOptions) -> *mut Self {
        let storage = Box::new(OperationOptionsStorage {
            _opaque: [],
            inner: Arc::new(OperationOptionsInner { inner }),
        });
        Box::into_raw(storage).cast::<OperationOptionsHandle>()
    }

    pub(crate) fn inner_arc(
        p: *const OperationOptionsHandle,
    ) -> Option<Arc<OperationOptionsInner>> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `into_raw` and
        // has not been freed.
        let storage = unsafe { &*(p as *const OperationOptionsStorage) };
        Some(Arc::clone(&storage.inner))
    }

    fn drop_raw(p: *mut OperationOptionsHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: pun back into the `Box<OperationOptionsStorage>` we
        // originally allocated.
        unsafe {
            drop(Box::from_raw(p.cast::<OperationOptionsStorage>()));
        }
    }
}

/// Frees a built `cosmos_operation_options_t *`. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_operation_options_free(options: *mut OperationOptionsHandle) {
    if options.is_null() {
        return;
    }
    tracing::trace!(?options, "freeing cosmos_operation_options_t");
    OperationOptionsHandle::drop_raw(options);
}

// ─────────────────────────────────────────────────────────────────────────────
// Builder handle
//
// The driver's `OperationOptionsBuilder` is auto-generated by the
// `CosmosOptions` derive macro. Each `with_*` setter takes the **inner**
// type by value and consumes `self`. For `_clear_*` we work around the
// missing native API by writing `None` directly into the corresponding
// field on the built `OperationOptions` at `_build` time — see the
// `clears` bitmap in `OperationOptionsBuilderInner`.
//
// We keep the builder slot in an `Option<...>` so the take/restore dance
// is unambiguous when a setter panics or is interrupted.
// ─────────────────────────────────────────────────────────────────────────────

pub(crate) struct OperationOptionsBuilderInner {
    /// `None` only during a setter's take/restore window; always `Some`
    /// at every observable point.
    pub(crate) builder: Option<OperationOptionsBuilder>,
    /// Bitmap of fields the caller explicitly cleared. A `_clear_*` call
    /// flips the corresponding bit; `_build` zeroes the matching field
    /// after applying any builder setters. This preserves the "user
    /// explicitly chose None" signal across the setter dance even though
    /// the auto-generated builder has no native `clear_*` API.
    ///
    /// Bit indices match the order in `OPTION_FIELD_NAMES` below so the
    /// `clear_<name>` setters can flip the right bit without a string
    /// table at runtime.
    pub(crate) clears: u32,
    /// Incrementally-accumulated custom headers. The auto-generated
    /// builder's `with_custom_headers` setter takes the full HashMap by
    /// value and is not `Clone`, so we cannot read-modify-write through
    /// it. Instead we accumulate here and finalize at `_build` time.
    /// `None` means "no `set_custom_header` call has been made" —
    /// distinct from `Some(empty)` which `_clear_custom_headers`
    /// produces (so the cleared semantics survive even after a later
    /// `_set_*` call).
    pub(crate) custom_headers: Option<HashMap<HeaderName, HeaderValue>>,
}

/// Field-index bits for `OperationOptionsBuilderInner::clears`. Each
/// `_clear_<name>` setter ORs in its bit; `_build` consults the bitmap
/// after applying the builder's positive sets.
mod field_bits {
    pub(super) const READ_CONSISTENCY_STRATEGY: u32 = 1 << 0;
    pub(super) const EXCLUDED_REGIONS: u32 = 1 << 1;
    pub(super) const CONTENT_RESPONSE_ON_WRITE: u32 = 1 << 2;
    pub(super) const THROUGHPUT_CONTROL_GROUP: u32 = 1 << 3;
    pub(super) const END_TO_END_LATENCY_POLICY: u32 = 1 << 4;
    pub(super) const MAX_FAILOVER_RETRY_COUNT: u32 = 1 << 5;
    pub(super) const ENDPOINT_UNAVAILABILITY_TTL: u32 = 1 << 6;
    pub(super) const SESSION_CAPTURING_DISABLED: u32 = 1 << 7;
    pub(super) const MAX_SESSION_RETRY_COUNT: u32 = 1 << 8;
    pub(super) const CIRCUIT_BREAKER_FAILURE_COUNT_FOR_READS: u32 = 1 << 9;
    pub(super) const CIRCUIT_BREAKER_FAILURE_COUNT_FOR_WRITES: u32 = 1 << 10;
    pub(super) const CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES: u32 = 1 << 11;
    pub(super) const ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS: u32 = 1 << 12;
    pub(super) const PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS: u32 = 1 << 13;
    pub(super) const PER_PARTITION_CIRCUIT_BREAKER_ENABLED: u32 = 1 << 14;
    pub(super) const CUSTOM_HEADERS: u32 = 1 << 15;
}

/// Opaque C ABI handle for an `OperationOptionsBuilder`.
#[repr(C)]
pub struct OperationOptionsBuilderHandle {
    _opaque: [u8; 0],
}

#[repr(C)]
struct OperationOptionsBuilderStorage {
    _opaque: [u8; 0],
    inner: OperationOptionsBuilderInner,
}

impl OperationOptionsBuilderHandle {
    fn new_raw() -> *mut Self {
        let storage = Box::new(OperationOptionsBuilderStorage {
            _opaque: [],
            inner: OperationOptionsBuilderInner {
                builder: Some(OperationOptionsBuilder::new()),
                clears: 0,
                custom_headers: None,
            },
        });
        Box::into_raw(storage).cast::<OperationOptionsBuilderHandle>()
    }

    fn inner_mut<'a>(
        p: *mut OperationOptionsBuilderHandle,
    ) -> Option<&'a mut OperationOptionsBuilderInner> {
        if p.is_null() {
            return None;
        }
        // SAFETY: caller guarantees `p` was obtained from `new_raw` and
        // has not been freed.
        let storage = unsafe { &mut *(p.cast::<OperationOptionsBuilderStorage>()) };
        Some(&mut storage.inner)
    }

    fn into_inner(p: *mut OperationOptionsBuilderHandle) -> Option<OperationOptionsBuilderInner> {
        if p.is_null() {
            return None;
        }
        // SAFETY: pun back into the storage and move the inner state out.
        let storage = unsafe { Box::from_raw(p.cast::<OperationOptionsBuilderStorage>()) };
        Some(storage.inner)
    }

    fn drop_raw(p: *mut OperationOptionsBuilderHandle) {
        if p.is_null() {
            return;
        }
        // SAFETY: pun back into the storage we originally allocated.
        unsafe {
            drop(Box::from_raw(p.cast::<OperationOptionsBuilderStorage>()));
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI helpers
// ─────────────────────────────────────────────────────────────────────────────

fn try_cstr_to_str<'a>(p: *const c_char) -> Result<&'a str, CosmosErrorCode> {
    if p.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: caller contract on every setter.
    let cstr = unsafe { CStr::from_ptr(p) };
    cstr.to_str()
        .map_err(|_| CosmosErrorCode::CosmosErrorCodeInvalidUtf8)
}

/// Borrows the builder for an FFI setter call. Returns the coarse
/// `INVALID_ARGUMENT` code on NULL; returns the inner state otherwise.
fn setter_pre_flight<'a>(
    builder: *mut OperationOptionsBuilderHandle,
) -> Result<&'a mut OperationOptionsBuilderInner, CosmosErrorCode> {
    OperationOptionsBuilderHandle::inner_mut(builder)
        .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)
}

/// Performs a take-call-restore on the inner `Option<...>` builder slot.
/// `f` is invoked with the moved-out builder and returns the chained
/// builder; the result is stored back. If the slot was already `None`
/// (programmer error from outside the module — e.g. a setter call after
/// `_build` consumed the builder), returns `INVALID_ARGUMENT`.
fn apply_setter<F>(inner: &mut OperationOptionsBuilderInner, f: F) -> i32
where
    F: FnOnce(OperationOptionsBuilder) -> OperationOptionsBuilder,
{
    let Some(taken) = inner.builder.take() else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    inner.builder = Some(f(taken));
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: lifecycle
// ─────────────────────────────────────────────────────────────────────────────

/// Allocates a new operation-options builder. Always succeeds.
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_new() -> *mut OperationOptionsBuilderHandle {
    OperationOptionsBuilderHandle::new_raw()
}

/// Frees a builder that was never consumed by [`cosmos_operation_options_builder_build`].
/// NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_free(
    builder: *mut OperationOptionsBuilderHandle,
) {
    if builder.is_null() {
        return;
    }
    tracing::trace!(?builder, "freeing cosmos_operation_options_builder_t");
    OperationOptionsBuilderHandle::drop_raw(builder);
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: setters — typed enums
// ─────────────────────────────────────────────────────────────────────────────

/// Sets the read-consistency strategy (driver's
/// `read_consistency_strategy`). See [`CosmosReadConsistency`].
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_with_read_consistency_strategy(
    builder: *mut OperationOptionsBuilderHandle,
    value: CosmosReadConsistency,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    let driver_value = value.to_driver();
    inner.clears &= !field_bits::READ_CONSISTENCY_STRATEGY;
    apply_setter(inner, |b| b.with_read_consistency_strategy(driver_value))
}

/// Clears any explicitly-set read-consistency strategy so the field
/// inherits from the higher-priority options layer.
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_clear_read_consistency_strategy(
    builder: *mut OperationOptionsBuilderHandle,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.clears |= field_bits::READ_CONSISTENCY_STRATEGY;
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Sets the content-response-on-write mode (driver's
/// `content_response_on_write`).
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_with_content_response_on_write(
    builder: *mut OperationOptionsBuilderHandle,
    value: CosmosContentResponseOnWrite,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    let driver_value = value.to_driver();
    inner.clears &= !field_bits::CONTENT_RESPONSE_ON_WRITE;
    apply_setter(inner, |b| b.with_content_response_on_write(driver_value))
}

/// Clears any explicitly-set content-response-on-write mode.
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_clear_content_response_on_write(
    builder: *mut OperationOptionsBuilderHandle,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.clears |= field_bits::CONTENT_RESPONSE_ON_WRITE;
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: setters — primitive integers / bools
// ─────────────────────────────────────────────────────────────────────────────

macro_rules! u32_setter {
    ($with_fn:ident, $clear_fn:ident, $driver_setter:ident, $bit:expr) => {
        /// Sets the corresponding `OperationOptions::*` field.
        #[no_mangle]
        pub extern "C" fn $with_fn(builder: *mut OperationOptionsBuilderHandle, value: u32) -> i32 {
            let inner = match setter_pre_flight(builder) {
                Ok(i) => i,
                Err(code) => return code.as_i32(),
            };
            inner.clears &= !$bit;
            apply_setter(inner, |b| b.$driver_setter(value))
        }

        /// Clears the corresponding `OperationOptions::*` field so it
        /// inherits from the higher-priority options layer.
        #[no_mangle]
        pub extern "C" fn $clear_fn(builder: *mut OperationOptionsBuilderHandle) -> i32 {
            let inner = match setter_pre_flight(builder) {
                Ok(i) => i,
                Err(code) => return code.as_i32(),
            };
            inner.clears |= $bit;
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        }
    };
}

u32_setter!(
    cosmos_operation_options_builder_with_max_failover_retry_count,
    cosmos_operation_options_builder_clear_max_failover_retry_count,
    with_max_failover_retry_count,
    field_bits::MAX_FAILOVER_RETRY_COUNT
);
u32_setter!(
    cosmos_operation_options_builder_with_max_session_retry_count,
    cosmos_operation_options_builder_clear_max_session_retry_count,
    with_max_session_retry_count,
    field_bits::MAX_SESSION_RETRY_COUNT
);
u32_setter!(
    cosmos_operation_options_builder_with_circuit_breaker_failure_count_for_reads,
    cosmos_operation_options_builder_clear_circuit_breaker_failure_count_for_reads,
    with_circuit_breaker_failure_count_for_reads,
    field_bits::CIRCUIT_BREAKER_FAILURE_COUNT_FOR_READS
);
u32_setter!(
    cosmos_operation_options_builder_with_circuit_breaker_failure_count_for_writes,
    cosmos_operation_options_builder_clear_circuit_breaker_failure_count_for_writes,
    with_circuit_breaker_failure_count_for_writes,
    field_bits::CIRCUIT_BREAKER_FAILURE_COUNT_FOR_WRITES
);
u32_setter!(
    cosmos_operation_options_builder_with_circuit_breaker_timeout_counter_reset_window_in_minutes,
    cosmos_operation_options_builder_clear_circuit_breaker_timeout_counter_reset_window_in_minutes,
    with_circuit_breaker_timeout_counter_reset_window_in_minutes,
    field_bits::CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES
);
u32_setter!(
    cosmos_operation_options_builder_with_allowed_partition_unavailability_duration_in_seconds,
    cosmos_operation_options_builder_clear_allowed_partition_unavailability_duration_in_seconds,
    with_allowed_partition_unavailability_duration_in_seconds,
    field_bits::ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS
);
u32_setter!(
    cosmos_operation_options_builder_with_ppcb_stale_partition_unavailability_refresh_interval_in_seconds,
    cosmos_operation_options_builder_clear_ppcb_stale_partition_unavailability_refresh_interval_in_seconds,
    with_ppcb_stale_partition_unavailability_refresh_interval_in_seconds,
    field_bits::PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS
);

/// Sets `session_capturing_disabled`. `false` re-enables session
/// capture; `true` disables it.
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_with_session_capturing_disabled(
    builder: *mut OperationOptionsBuilderHandle,
    value: bool,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.clears &= !field_bits::SESSION_CAPTURING_DISABLED;
    apply_setter(inner, |b| b.with_session_capturing_disabled(value))
}

#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_clear_session_capturing_disabled(
    builder: *mut OperationOptionsBuilderHandle,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.clears |= field_bits::SESSION_CAPTURING_DISABLED;
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Sets `per_partition_circuit_breaker_enabled`.
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_with_per_partition_circuit_breaker_enabled(
    builder: *mut OperationOptionsBuilderHandle,
    value: bool,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.clears &= !field_bits::PER_PARTITION_CIRCUIT_BREAKER_ENABLED;
    apply_setter(inner, |b| {
        b.with_per_partition_circuit_breaker_enabled(value)
    })
}

#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_clear_per_partition_circuit_breaker_enabled(
    builder: *mut OperationOptionsBuilderHandle,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.clears |= field_bits::PER_PARTITION_CIRCUIT_BREAKER_ENABLED;
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: setters — Duration-shaped fields (ms input)
// ─────────────────────────────────────────────────────────────────────────────

/// Sets the end-to-end operation latency policy (the timeout below
/// which a request is abandoned in favor of retry). The driver clamps
/// values below 1 s to 1 s.
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_with_end_to_end_timeout_ms(
    builder: *mut OperationOptionsBuilderHandle,
    timeout_ms: u64,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    let policy = EndToEndOperationLatencyPolicy::new(Duration::from_millis(timeout_ms));
    inner.clears &= !field_bits::END_TO_END_LATENCY_POLICY;
    apply_setter(inner, |b| b.with_end_to_end_latency_policy(policy))
}

#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_clear_end_to_end_timeout(
    builder: *mut OperationOptionsBuilderHandle,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.clears |= field_bits::END_TO_END_LATENCY_POLICY;
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Sets `endpoint_unavailability_ttl` (how long an endpoint stays
/// marked unavailable after a failure).
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_with_endpoint_unavailability_ttl_ms(
    builder: *mut OperationOptionsBuilderHandle,
    ttl_ms: u64,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    let ttl = Duration::from_millis(ttl_ms);
    inner.clears &= !field_bits::ENDPOINT_UNAVAILABILITY_TTL;
    apply_setter(inner, |b| b.with_endpoint_unavailability_ttl(ttl))
}

#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_clear_endpoint_unavailability_ttl(
    builder: *mut OperationOptionsBuilderHandle,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.clears |= field_bits::ENDPOINT_UNAVAILABILITY_TTL;
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: setters — string-shaped fields
// ─────────────────────────────────────────────────────────────────────────────

/// Sets `throughput_control_group` (references a group registered on
/// the runtime).
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_with_throughput_control_group(
    builder: *mut OperationOptionsBuilderHandle,
    group_name: *const c_char,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    let name = match try_cstr_to_str(group_name) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };
    let driver_value = ThroughputControlGroupName::from(name.to_owned());
    inner.clears &= !field_bits::THROUGHPUT_CONTROL_GROUP;
    apply_setter(inner, |b| b.with_throughput_control_group(driver_value))
}

#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_clear_throughput_control_group(
    builder: *mut OperationOptionsBuilderHandle,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.clears |= field_bits::THROUGHPUT_CONTROL_GROUP;
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: setters — list-shaped fields
// ─────────────────────────────────────────────────────────────────────────────

/// Sets `excluded_regions` from a C array of NUL-terminated UTF-8
/// region names.
///
/// Replaces any previously-set list. NULL `regions` with `regions_len == 0`
/// is accepted and equivalent to setting an empty list (the field is
/// set to `Some(ExcludedRegions(vec![]))` — distinct from "inherit"
/// which `_clear_excluded_regions` produces).
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_with_excluded_regions(
    builder: *mut OperationOptionsBuilderHandle,
    regions: *const *const c_char,
    regions_len: usize,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    if regions.is_null() && regions_len > 0 {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }

    let mut owned: Vec<Region> = Vec::with_capacity(regions_len);
    for i in 0..regions_len {
        // SAFETY: `regions` is non-NULL when `regions_len > 0` (checked
        // above) and the caller guarantees the array has at least
        // `regions_len` entries.
        let entry_ptr = unsafe { *regions.add(i) };
        if entry_ptr.is_null() {
            return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
        }
        // SAFETY: each entry is a NUL-terminated C string per the
        // caller's contract.
        let cstr = unsafe { CStr::from_ptr(entry_ptr) };
        let s = match cstr.to_str() {
            Ok(s) => s,
            Err(_) => return CosmosErrorCode::CosmosErrorCodeInvalidUtf8.as_i32(),
        };
        owned.push(Region::new(s.to_owned()));
    }
    let driver_value = ExcludedRegions(owned);
    inner.clears &= !field_bits::EXCLUDED_REGIONS;
    apply_setter(inner, |b| b.with_excluded_regions(driver_value))
}

#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_clear_excluded_regions(
    builder: *mut OperationOptionsBuilderHandle,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.clears |= field_bits::EXCLUDED_REGIONS;
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: setters — custom headers (incremental key/value)
// ─────────────────────────────────────────────────────────────────────────────

/// Validates that a header name / value is ASCII printable (i.e.
/// excludes control characters). The driver's `HeaderName` /
/// `HeaderValue` constructors are infallible on `String`, but Cosmos
/// servers reject illegal characters with opaque errors much later — we
/// pre-validate so the caller sees a deterministic `INVALID_HEADER`
/// up-front.
fn is_http_header_ascii(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| (0x20..=0x7e).contains(&b))
}

/// Sets a single custom-header entry. The first call to this on a
/// builder allocates the underlying map; subsequent calls append.
///
/// `INVALID_HEADER` (4010) is returned when either string contains
/// non-ASCII or control characters (Cosmos rejects these wire-side; we
/// fail fast).
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_set_custom_header(
    builder: *mut OperationOptionsBuilderHandle,
    name: *const c_char,
    value: *const c_char,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    let name_str = match try_cstr_to_str(name) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };
    let value_str = match try_cstr_to_str(value) {
        Ok(s) => s,
        Err(code) => return code.as_i32(),
    };
    if !is_http_header_ascii(name_str) || !is_http_header_ascii(value_str) {
        return CosmosErrorCode::CosmosErrorCodeInvalidHeader.as_i32();
    }

    let map = inner.custom_headers.get_or_insert_with(HashMap::new);
    map.insert(
        HeaderName::from(name_str.to_owned()),
        HeaderValue::from(value_str.to_owned()),
    );
    inner.clears &= !field_bits::CUSTOM_HEADERS;
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Drops every previously-set custom header. The `Option<HashMap>`
/// field reverts to `None` (i.e. inherits from a higher layer).
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_clear_custom_headers(
    builder: *mut OperationOptionsBuilderHandle,
) -> i32 {
    let inner = match setter_pre_flight(builder) {
        Ok(i) => i,
        Err(code) => return code.as_i32(),
    };
    inner.custom_headers = None;
    inner.clears |= field_bits::CUSTOM_HEADERS;
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI: build
// ─────────────────────────────────────────────────────────────────────────────

/// Consumes the builder and returns an immutable
/// `cosmos_operation_options_t *`.
///
/// # Lifetime
///
/// `_build` consumes the builder regardless of success or failure.
/// Callers must NOT call [`cosmos_operation_options_builder_free`] on
/// the same pointer afterwards.
///
/// # Returns
///
/// - `SUCCESS` (0) with `*out_options` populated.
/// - `INVALID_ARGUMENT` (1) when `builder` or `out_options` is NULL. In
///   the NULL-`out_options` case the builder is still consumed to avoid
///   leaking the inner allocation.
#[no_mangle]
pub extern "C" fn cosmos_operation_options_builder_build(
    builder: *mut OperationOptionsBuilderHandle,
    out_options: *mut *mut OperationOptionsHandle,
) -> i32 {
    if builder.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(inner) = OperationOptionsBuilderHandle::into_inner(builder) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    if out_options.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    }
    let Some(driver_builder) = inner.builder else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
    };
    let mut opts = driver_builder.build();
    // Apply incrementally-accumulated custom headers (side-channel
    // because the auto-generated builder is not `Clone`-able).
    if let Some(map) = inner.custom_headers {
        opts.custom_headers = Some(map);
    }
    apply_clears(&mut opts, inner.clears);

    let handle = OperationOptionsHandle::into_raw(opts);
    // SAFETY: caller guarantees `out_options` is writable for one
    // `*mut OperationOptionsHandle`.
    unsafe {
        *out_options = handle;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
}

/// Zeroes every field whose bit is set in `clears`. Called by `_build`
/// after the auto-generated builder has populated the options.
fn apply_clears(opts: &mut OperationOptions, clears: u32) {
    use field_bits::*;
    if clears & READ_CONSISTENCY_STRATEGY != 0 {
        opts.read_consistency_strategy = None;
    }
    if clears & EXCLUDED_REGIONS != 0 {
        opts.excluded_regions = None;
    }
    if clears & CONTENT_RESPONSE_ON_WRITE != 0 {
        opts.content_response_on_write = None;
    }
    if clears & THROUGHPUT_CONTROL_GROUP != 0 {
        opts.throughput_control_group = None;
    }
    if clears & END_TO_END_LATENCY_POLICY != 0 {
        opts.end_to_end_latency_policy = None;
    }
    if clears & MAX_FAILOVER_RETRY_COUNT != 0 {
        opts.max_failover_retry_count = None;
    }
    if clears & ENDPOINT_UNAVAILABILITY_TTL != 0 {
        opts.endpoint_unavailability_ttl = None;
    }
    if clears & SESSION_CAPTURING_DISABLED != 0 {
        opts.session_capturing_disabled = None;
    }
    if clears & MAX_SESSION_RETRY_COUNT != 0 {
        opts.max_session_retry_count = None;
    }
    if clears & CIRCUIT_BREAKER_FAILURE_COUNT_FOR_READS != 0 {
        opts.circuit_breaker_failure_count_for_reads = None;
    }
    if clears & CIRCUIT_BREAKER_FAILURE_COUNT_FOR_WRITES != 0 {
        opts.circuit_breaker_failure_count_for_writes = None;
    }
    if clears & CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES != 0 {
        opts.circuit_breaker_timeout_counter_reset_window_in_minutes = None;
    }
    if clears & ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS != 0 {
        opts.allowed_partition_unavailability_duration_in_seconds = None;
    }
    if clears & PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS != 0 {
        opts.ppcb_stale_partition_unavailability_refresh_interval_in_seconds = None;
    }
    if clears & PER_PARTITION_CIRCUIT_BREAKER_ENABLED != 0 {
        opts.per_partition_circuit_breaker_enabled = None;
    }
    if clears & CUSTOM_HEADERS != 0 {
        opts.custom_headers = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    fn ok_cstr(s: &str) -> CString {
        CString::new(s).expect("test inputs must be NUL-free")
    }

    fn new_builder() -> *mut OperationOptionsBuilderHandle {
        cosmos_operation_options_builder_new()
    }

    fn build(b: *mut OperationOptionsBuilderHandle) -> *mut OperationOptionsHandle {
        let mut out: *mut OperationOptionsHandle = ptr::null_mut();
        let rc = cosmos_operation_options_builder_build(b, &mut out);
        assert_eq!(rc, CosmosErrorCode::CosmosErrorCodeSuccess.as_i32());
        assert!(!out.is_null());
        out
    }

    #[test]
    fn lifecycle_null_safe() {
        cosmos_operation_options_builder_free(ptr::null_mut());
        cosmos_operation_options_free(ptr::null_mut());
    }

    #[test]
    fn empty_build_produces_all_none() {
        let b = new_builder();
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        let o = &inner.inner;
        assert!(o.read_consistency_strategy.is_none());
        assert!(o.excluded_regions.is_none());
        assert!(o.content_response_on_write.is_none());
        assert!(o.throughput_control_group.is_none());
        assert!(o.end_to_end_latency_policy.is_none());
        assert!(o.max_failover_retry_count.is_none());
        assert!(o.endpoint_unavailability_ttl.is_none());
        assert!(o.session_capturing_disabled.is_none());
        assert!(o.max_session_retry_count.is_none());
        assert!(o.circuit_breaker_failure_count_for_reads.is_none());
        assert!(o.circuit_breaker_failure_count_for_writes.is_none());
        assert!(o.per_partition_circuit_breaker_enabled.is_none());
        assert!(o.custom_headers.is_none());
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn read_consistency_strategy_roundtrips() {
        let b = new_builder();
        let rc = cosmos_operation_options_builder_with_read_consistency_strategy(
            b,
            CosmosReadConsistency::CosmosReadConsistencyEventual,
        );
        assert_eq!(rc, CosmosErrorCode::CosmosErrorCodeSuccess.as_i32());
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        assert_eq!(
            inner.inner.read_consistency_strategy,
            Some(ReadConsistencyStrategy::Eventual)
        );
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn content_response_roundtrips() {
        let b = new_builder();
        let rc = cosmos_operation_options_builder_with_content_response_on_write(
            b,
            CosmosContentResponseOnWrite::CosmosContentResponseOnWriteEnabled,
        );
        assert_eq!(rc, CosmosErrorCode::CosmosErrorCodeSuccess.as_i32());
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        assert_eq!(
            inner.inner.content_response_on_write,
            Some(ContentResponseOnWrite::Enabled)
        );
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn u32_field_setters_roundtrip() {
        let b = new_builder();
        for rc in [
            cosmos_operation_options_builder_with_max_failover_retry_count(b, 7),
            cosmos_operation_options_builder_with_max_session_retry_count(b, 11),
            cosmos_operation_options_builder_with_circuit_breaker_failure_count_for_reads(b, 13),
            cosmos_operation_options_builder_with_circuit_breaker_failure_count_for_writes(b, 17),
        ] {
            assert_eq!(rc, CosmosErrorCode::CosmosErrorCodeSuccess.as_i32());
        }
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        let o = &inner.inner;
        assert_eq!(o.max_failover_retry_count, Some(7));
        assert_eq!(o.max_session_retry_count, Some(11));
        assert_eq!(o.circuit_breaker_failure_count_for_reads, Some(13));
        assert_eq!(o.circuit_breaker_failure_count_for_writes, Some(17));
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn bool_setters_roundtrip() {
        let b = new_builder();
        cosmos_operation_options_builder_with_session_capturing_disabled(b, true);
        cosmos_operation_options_builder_with_per_partition_circuit_breaker_enabled(b, false);
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        assert_eq!(inner.inner.session_capturing_disabled, Some(true));
        assert_eq!(
            inner.inner.per_partition_circuit_breaker_enabled,
            Some(false)
        );
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn end_to_end_timeout_ms_clamps_below_1s() {
        let b = new_builder();
        cosmos_operation_options_builder_with_end_to_end_timeout_ms(b, 250);
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        let policy = inner.inner.end_to_end_latency_policy.as_ref().unwrap();
        // Driver enforces a 1-second minimum.
        assert_eq!(policy.timeout(), Duration::from_secs(1));
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn end_to_end_timeout_ms_preserves_normal_values() {
        let b = new_builder();
        cosmos_operation_options_builder_with_end_to_end_timeout_ms(b, 5_000);
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        let policy = inner.inner.end_to_end_latency_policy.as_ref().unwrap();
        assert_eq!(policy.timeout(), Duration::from_secs(5));
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn endpoint_unavailability_ttl_ms_roundtrips() {
        let b = new_builder();
        cosmos_operation_options_builder_with_endpoint_unavailability_ttl_ms(b, 30_000);
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        assert_eq!(
            inner.inner.endpoint_unavailability_ttl,
            Some(Duration::from_secs(30))
        );
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn throughput_control_group_roundtrips() {
        let b = new_builder();
        let name = ok_cstr("default-group");
        let rc = cosmos_operation_options_builder_with_throughput_control_group(b, name.as_ptr());
        assert_eq!(rc, CosmosErrorCode::CosmosErrorCodeSuccess.as_i32());
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        assert_eq!(
            inner
                .inner
                .throughput_control_group
                .as_ref()
                .map(|g| g.0.as_ref()),
            Some("default-group")
        );
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn excluded_regions_roundtrip_and_zero_len() {
        let b = new_builder();
        let r1 = ok_cstr("East US");
        let r2 = ok_cstr("West US 3");
        let arr: [*const c_char; 2] = [r1.as_ptr(), r2.as_ptr()];
        let rc = cosmos_operation_options_builder_with_excluded_regions(b, arr.as_ptr(), arr.len());
        assert_eq!(rc, CosmosErrorCode::CosmosErrorCodeSuccess.as_i32());

        // Also assert len=0 with NULL ptr is accepted.
        assert_eq!(
            cosmos_operation_options_builder_with_excluded_regions(b, ptr::null(), 0),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );

        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        let regions = inner.inner.excluded_regions.as_ref().unwrap();
        assert!(regions.is_empty(), "zero-len call cleared the list");
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn excluded_regions_rejects_nulls() {
        let b = new_builder();
        // NULL ptr with non-zero len.
        assert_eq!(
            cosmos_operation_options_builder_with_excluded_regions(b, ptr::null(), 1),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        // NULL entry within the array.
        let arr: [*const c_char; 1] = [ptr::null()];
        assert_eq!(
            cosmos_operation_options_builder_with_excluded_regions(b, arr.as_ptr(), 1),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        cosmos_operation_options_builder_free(b);
    }

    #[test]
    fn set_custom_header_validation() {
        let b = new_builder();
        let ok_name = ok_cstr("x-app-tag");
        let ok_value = ok_cstr("foo");
        assert_eq!(
            cosmos_operation_options_builder_set_custom_header(
                b,
                ok_name.as_ptr(),
                ok_value.as_ptr()
            ),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );
        // Append a second.
        let ok_name2 = ok_cstr("x-other");
        let ok_value2 = ok_cstr("bar");
        assert_eq!(
            cosmos_operation_options_builder_set_custom_header(
                b,
                ok_name2.as_ptr(),
                ok_value2.as_ptr()
            ),
            CosmosErrorCode::CosmosErrorCodeSuccess.as_i32()
        );

        // Reject control character.
        let bad = ok_cstr("x-bad");
        let bad_value = CString::new("hello\x01world").unwrap();
        assert_eq!(
            cosmos_operation_options_builder_set_custom_header(b, bad.as_ptr(), bad_value.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidHeader.as_i32()
        );

        // Reject empty.
        let empty = ok_cstr("");
        let v = ok_cstr("v");
        assert_eq!(
            cosmos_operation_options_builder_set_custom_header(b, empty.as_ptr(), v.as_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidHeader.as_i32()
        );

        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        let map = inner.inner.custom_headers.as_ref().unwrap();
        assert_eq!(map.len(), 2);
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn clear_setters_take_effect() {
        let b = new_builder();
        cosmos_operation_options_builder_with_max_failover_retry_count(b, 7);
        cosmos_operation_options_builder_clear_max_failover_retry_count(b);
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        assert_eq!(
            inner.inner.max_failover_retry_count, None,
            "clear after set must revert to None"
        );
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn clear_after_set_then_set_again_wins() {
        let b = new_builder();
        cosmos_operation_options_builder_with_max_failover_retry_count(b, 7);
        cosmos_operation_options_builder_clear_max_failover_retry_count(b);
        // The second set must beat the clear (the clear bit is cleared
        // by the subsequent set per the documented semantics).
        cosmos_operation_options_builder_with_max_failover_retry_count(b, 9);
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        assert_eq!(inner.inner.max_failover_retry_count, Some(9));
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn clear_custom_headers_drops_map() {
        let b = new_builder();
        let n = ok_cstr("x-app-tag");
        let v = ok_cstr("foo");
        cosmos_operation_options_builder_set_custom_header(b, n.as_ptr(), v.as_ptr());
        cosmos_operation_options_builder_clear_custom_headers(b);
        let opts = build(b);
        let inner = OperationOptionsHandle::inner_arc(opts).unwrap();
        assert!(inner.inner.custom_headers.is_none());
        drop(inner);
        cosmos_operation_options_free(opts);
    }

    #[test]
    fn setters_reject_null_builder() {
        let s = ok_cstr("x");
        assert_eq!(
            cosmos_operation_options_builder_with_read_consistency_strategy(
                ptr::null_mut(),
                CosmosReadConsistency::CosmosReadConsistencyEventual
            ),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_operation_options_builder_with_max_failover_retry_count(ptr::null_mut(), 1),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_operation_options_builder_with_throughput_control_group(
                ptr::null_mut(),
                s.as_ptr()
            ),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_operation_options_builder_set_custom_header(
                ptr::null_mut(),
                s.as_ptr(),
                s.as_ptr()
            ),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        assert_eq!(
            cosmos_operation_options_builder_clear_max_failover_retry_count(ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
    }

    #[test]
    fn build_rejects_null_arguments() {
        let mut out: *mut OperationOptionsHandle = ptr::null_mut();
        assert_eq!(
            cosmos_operation_options_builder_build(ptr::null_mut(), &mut out),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
        let b = new_builder();
        assert_eq!(
            cosmos_operation_options_builder_build(b, ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32()
        );
    }
}
