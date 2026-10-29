// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Read-only diagnostics surface exposed on a completion.
//!
//! A completion whose operation produced driver diagnostics carries a
//! non-NULL [`diagnostics`](crate::completion::CosmosCompletion::diagnostics)
//! pointer to a [`CosmosDiagnostics`] handle. The handle exposes the
//! aggregate metrics (total request charge, elapsed time, attempt count),
//! operation-level status (completed, failure), the set of regions contacted,
//! the per-attempt retry timeline, and a JSON snapshot for logging.
//!
//! Under a retry storm the driver caps the per-attempt records it retains, so
//! the timeline yielded by [`cosmos_diagnostics_iter_attempts`] may be a
//! compacted subset (signalled by [`cosmos_diagnostics_is_compacted`] /
//! [`cosmos_diagnostics_retained_request_count`]); the aggregate charge and
//! [`cosmos_diagnostics_request_count`] stay exact.
//!
//! The handle is owned by the completion's backing and stays valid until the
//! completion is freed by `cosmos_completion_queue_free_completions`; it must
//! not be freed separately. Every accessor is read-only and NULL-safe.

use std::ffi::{c_char, c_void, CString};
use std::sync::Arc;

use azure_data_cosmos_driver::{
    diagnostics::{
        PipelineKind, RequestSentStatus, TransportHttpVersion, TransportKind, TransportSecurity,
    },
    options::DiagnosticsVerbosity,
    DiagnosticsContext, ExecutionContext,
};

use crate::error::{CosmosErrorCode, CosmosStatusCode, COSMOS_STATUS_SUCCESS};

/// Version of the structured diagnostics records exposed by this module.
pub const COSMOS_DIAGNOSTICS_ABI_VERSION_2: u32 = 2;

/// Borrowed counted UTF-8 output.
///
/// `data` is NULL only when an optional value is unavailable. Otherwise it
/// points to `len` UTF-8 bytes, which may be empty and are not NUL-terminated.
/// The view is valid only for the lifetime documented by the accessor that
/// returned it and must never be freed by the caller.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CosmosBorrowedStringView {
    /// Borrowed UTF-8 bytes, or NULL when the optional value is unavailable.
    pub data: *const u8,
    /// Number of readable bytes.
    pub len: usize,
}

impl CosmosBorrowedStringView {
    fn from_str(value: &str) -> Self {
        Self {
            data: value.as_ptr(),
            len: value.len(),
        }
    }

    fn from_optional(value: Option<&str>) -> Self {
        value.map(Self::from_str).unwrap_or_default()
    }
}

/// Versioned operation-level structured diagnostics.
///
/// Initialize no fields: [`cosmos_diagnostics_get_info_v2`] writes the complete
/// record. String views borrow the diagnostics handle and remain valid until
/// the owning completion is freed.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CosmosDiagnosticsInfoV2 {
    /// Size of this record in bytes.
    pub struct_size_bytes: u32,
    /// [`COSMOS_DIAGNOSTICS_ABI_VERSION_2`].
    pub schema_version: u32,
    /// Operation activity ID.
    pub activity_id: CosmosBorrowedStringView,
    /// Canonical `db.operation.name`, or NULL/0 when unknown.
    pub operation_name: CosmosBorrowedStringView,
    /// Final HTTP status, or 0 when no final status is available.
    pub status_code: u16,
    /// Reserved for alignment; always zero.
    pub reserved_status: u16,
    /// Final Cosmos sub-status, or -1 when unavailable.
    pub sub_status: i32,
    /// Reserved for future schema-compatible additions; always zero.
    pub reserved: [u32; 4],
}

/// Versioned structured diagnostics for one retained request attempt.
///
/// The record and every string view it contains are valid only during the
/// [`cosmos_diagnostics_iter_attempts_v2`] callback. Integer classifications
/// use the `COSMOS_DIAGNOSTICS_*` constants emitted in the C header; zero is
/// always the stable unknown value.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CosmosDiagnosticsAttemptV2 {
    /// Size of this record in bytes.
    pub struct_size_bytes: u32,
    /// [`COSMOS_DIAGNOSTICS_ABI_VERSION_2`].
    pub schema_version: u32,
    /// Zero-based position in the retained attempt sequence.
    pub retained_index: u32,
    /// Why this request was dispatched.
    pub execution_context: i32,
    /// Metadata or data-plane pipeline.
    pub pipeline_type: i32,
    /// Transport certificate-validation mode.
    pub transport_security: i32,
    /// Concrete transport implementation.
    pub transport_kind: i32,
    /// HTTP protocol version selected by the transport.
    pub transport_http_version: i32,
    /// Tri-state indication of whether bytes reached the wire.
    pub request_sent: i32,
    /// Number of transport-local shard retries within this request.
    pub local_shard_retry_count: u32,
    /// HTTP status for this request, or 0 before completion.
    pub status_code: u16,
    /// One when the request has reached any terminal state.
    pub is_completed: u8,
    /// One when the request ended in a client-side timeout.
    pub timed_out: u8,
    /// One when `server_duration_ms` was reported by the service.
    pub has_server_duration: u8,
    /// Reserved alignment bytes; always zero.
    pub reserved_flags: [u8; 3],
    /// Cosmos sub-status, or -1 when unavailable.
    pub sub_status: i32,
    /// End-to-end request duration in milliseconds.
    pub duration_ms: u64,
    /// Request charge for this attempt.
    pub request_charge: f64,
    /// Service-reported duration, meaningful only when `has_server_duration` is 1.
    pub server_duration_ms: f64,
    /// Endpoint contacted.
    pub endpoint: CosmosBorrowedStringView,
    /// Region contacted, or NULL/0 when unresolved.
    pub region: CosmosBorrowedStringView,
    /// Per-request operation name, or NULL/0 when it matches the parent.
    pub operation_name: CosmosBorrowedStringView,
    /// Response activity ID, or NULL/0 when unavailable.
    pub activity_id: CosmosBorrowedStringView,
    /// Response session token, or NULL/0 when unavailable.
    pub session_token: CosmosBorrowedStringView,
    /// Transport or pipeline error text, or NULL/0 when unavailable.
    pub error: CosmosBorrowedStringView,
    /// Reserved for future schema-compatible additions; always zero.
    pub reserved: [u32; 4],
}

/// Opaque, read-only diagnostics handle borrowed from a completion.
///
/// Obtained through a completion's `diagnostics` field. The handle is owned by
/// the completion and remains valid until the completion is freed; it must not
/// be freed on its own. All accessors are safe to call from any thread while
/// the owning completion is alive.
pub struct CosmosDiagnostics {
    inner: Arc<DiagnosticsContext>,
}

impl CosmosDiagnostics {
    /// Wraps a driver diagnostics context for exposure across the C ABI.
    pub(crate) fn new(inner: Arc<DiagnosticsContext>) -> Self {
        Self { inner }
    }
}

fn execution_context_value(value: ExecutionContext) -> i32 {
    match value {
        ExecutionContext::Initial => 1,
        ExecutionContext::OperationRetry => 2,
        ExecutionContext::TransportRetry => 3,
        ExecutionContext::Hedging => 4,
        ExecutionContext::RegionFailover => 5,
        ExecutionContext::CircuitBreakerProbe => 6,
        _ => 0,
    }
}

fn pipeline_type_value(value: PipelineKind) -> i32 {
    match value {
        PipelineKind::Metadata => 1,
        PipelineKind::DataPlane => 2,
        _ => 0,
    }
}

fn transport_security_value(value: TransportSecurity) -> i32 {
    match value {
        TransportSecurity::Secure => 1,
        TransportSecurity::EmulatorWithInsecureCertificates => 2,
        _ => 0,
    }
}

fn transport_kind_value(value: TransportKind) -> i32 {
    match value {
        TransportKind::Gateway => 1,
        TransportKind::GatewayV2 => 2,
        _ => 0,
    }
}

fn transport_http_version_value(value: TransportHttpVersion) -> i32 {
    match value {
        TransportHttpVersion::Http11 => 1,
        TransportHttpVersion::Http2 => 2,
        _ => 0,
    }
}

fn request_sent_value(value: RequestSentStatus) -> i32 {
    match value {
        RequestSentStatus::Unknown => 0,
        RequestSentStatus::Sent => 1,
        RequestSentStatus::NotSent => 2,
        _ => 0,
    }
}

/// Writes operation-level structured diagnostics into `out_info`.
///
/// Returns a NULL-argument error without writing when either pointer is NULL.
/// On success, string views borrow `d` and remain valid until the owning
/// completion is freed.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_get_info_v2(
    d: *const CosmosDiagnostics,
    out_info: *mut CosmosDiagnosticsInfoV2,
) -> CosmosStatusCode {
    if d.is_null() || out_info.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: the caller guarantees `d` came from a live completion's
    // diagnostics field and the completion has not been freed.
    let handle = unsafe { &*d };
    let status = handle.inner.status();
    let info = CosmosDiagnosticsInfoV2 {
        struct_size_bytes: std::mem::size_of::<CosmosDiagnosticsInfoV2>() as u32,
        schema_version: COSMOS_DIAGNOSTICS_ABI_VERSION_2,
        activity_id: CosmosBorrowedStringView::from_str(handle.inner.activity_id().as_str()),
        operation_name: CosmosBorrowedStringView::from_optional(handle.inner.operation_name()),
        status_code: status.map_or(0, |value| u16::from(value.status_code())),
        reserved_status: 0,
        sub_status: status
            .and_then(|value| value.sub_status())
            .map_or(-1, |value| i32::from(u16::from(value))),
        reserved: [0; 4],
    };
    // SAFETY: `out_info` is non-NULL per the guard above and points to writable
    // storage for the complete output record by contract.
    unsafe { out_info.write(info) };
    COSMOS_STATUS_SUCCESS
}

/// Total request charge (RU) aggregated across every attempt. Returns `0.0`
/// when `d` is NULL.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_total_request_charge(d: *const CosmosDiagnostics) -> f64 {
    if d.is_null() {
        return 0.0;
    }
    // SAFETY: the caller guarantees `d` came from a live completion's
    // `diagnostics` field and the completion has not been freed.
    let handle = unsafe { &*d };
    handle.inner.total_request_charge().value()
}

/// Total wall-clock time the operation took, in microseconds. Returns `0` when
/// `d` is NULL, and saturates to `u64::MAX` for the (practically impossible)
/// overflow.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_total_elapsed_micros(d: *const CosmosDiagnostics) -> u64 {
    if d.is_null() {
        return 0;
    }
    // SAFETY: see `cosmos_diagnostics_total_request_charge`.
    let handle = unsafe { &*d };
    u64::try_from(handle.inner.duration().as_micros()).unwrap_or(u64::MAX)
}

/// Number of attempts recorded for the operation (the initial try plus any
/// retries, hedges, or failovers). A value of `1` means no retry occurred.
/// Returns `0` when `d` is NULL.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_request_count(d: *const CosmosDiagnostics) -> u32 {
    if d.is_null() {
        return 0;
    }
    // SAFETY: see `cosmos_diagnostics_total_request_charge`.
    let handle = unsafe { &*d };
    u32::try_from(handle.inner.request_count()).unwrap_or(u32::MAX)
}

/// Number of per-attempt records actually retained for iteration by
/// [`cosmos_diagnostics_iter_attempts`]. Under a retry storm the driver caps
/// the retained set, so this can be smaller than
/// [`cosmos_diagnostics_request_count`] (the true total). Returns `0` when `d`
/// is NULL.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_retained_request_count(d: *const CosmosDiagnostics) -> u32 {
    if d.is_null() {
        return 0;
    }
    // SAFETY: see `cosmos_diagnostics_total_request_charge`.
    let handle = unsafe { &*d };
    u32::try_from(handle.inner.retained_request_count()).unwrap_or(u32::MAX)
}

/// Returns `true` when the per-attempt list was compacted because the operation
/// exceeded the driver's retained-attempt cap. When `true`, the attempts
/// yielded by [`cosmos_diagnostics_iter_attempts`] are the retained subset
/// (see [`cosmos_diagnostics_retained_request_count`]), not the full timeline,
/// while [`cosmos_diagnostics_total_request_charge`] and
/// [`cosmos_diagnostics_request_count`] remain exact. Returns `false` when `d`
/// is NULL.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_is_compacted(d: *const CosmosDiagnostics) -> bool {
    if d.is_null() {
        return false;
    }
    // SAFETY: see `cosmos_diagnostics_total_request_charge`.
    let handle = unsafe { &*d };
    handle.inner.compaction().is_some()
}

/// Returns `true` when the operation reached a terminal state (a final status
/// was recorded or at least one attempt completed). A cancelled or still
/// in-flight operation is not completed. Returns `false` when `d` is NULL.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_is_completed(d: *const CosmosDiagnostics) -> bool {
    if d.is_null() {
        return false;
    }
    // SAFETY: see `cosmos_diagnostics_total_request_charge`.
    let handle = unsafe { &*d };
    handle.inner.is_completed()
}

/// Returns `true` when the operation completed with a non-success status.
/// Returns `false` when `d` is NULL or the operation succeeded.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_is_failure(d: *const CosmosDiagnostics) -> bool {
    if d.is_null() {
        return false;
    }
    // SAFETY: see `cosmos_diagnostics_total_request_charge`.
    let handle = unsafe { &*d };
    handle.inner.is_failure()
}

/// Invokes `visitor` once per distinct region contacted, in first-contact
/// order. `region_name` is a NUL-terminated UTF-8 string valid only for the
/// duration of the call. Does nothing when `d` or `visitor` is NULL.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_iter_regions_contacted(
    d: *const CosmosDiagnostics,
    visitor: Option<extern "C" fn(user_data: *mut c_void, region_name: *const c_char)>,
    user_data: *mut c_void,
) {
    let Some(visitor) = visitor else {
        return;
    };
    if d.is_null() {
        return;
    }
    // SAFETY: see `cosmos_diagnostics_total_request_charge`.
    let handle = unsafe { &*d };
    for region in handle.inner.regions_contacted() {
        if let Ok(name) = CString::new(region.as_str()) {
            visitor(user_data, name.as_ptr());
        }
    }
}

/// Invokes `visitor` once per recorded attempt, in execution order.
///
/// `endpoint` and `region` are NUL-terminated UTF-8 strings valid only for the
/// duration of the call; `region` is NULL when the attempt has no associated
/// region. `status_code` is the attempt's HTTP status and `sub_status` is the
/// Cosmos sub-status, or `-1` when the attempt recorded none. `latency_ms` is
/// the attempt's wall-clock duration in milliseconds, `request_charge` is its
/// RU cost, and `server_duration_ms` is the service-reported processing time
/// or a negative value when the service did not report one.
///
/// When [`cosmos_diagnostics_is_compacted`] is `true` these are the retained
/// attempts only, not the full timeline. Does nothing when `d` or `visitor` is
/// NULL.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_iter_attempts(
    d: *const CosmosDiagnostics,
    visitor: Option<
        extern "C" fn(
            user_data: *mut c_void,
            endpoint: *const c_char,
            region: *const c_char,
            status_code: u16,
            sub_status: i32,
            latency_ms: u64,
            request_charge: f64,
            server_duration_ms: f64,
        ),
    >,
    user_data: *mut c_void,
) {
    let Some(visitor) = visitor else {
        return;
    };
    if d.is_null() {
        return;
    }
    // SAFETY: see `cosmos_diagnostics_total_request_charge`.
    let handle = unsafe { &*d };
    for attempt in handle.inner.requests().iter() {
        let endpoint = CString::new(attempt.endpoint()).ok();
        let region = attempt.region().and_then(|r| CString::new(r.as_str()).ok());
        let endpoint_ptr = endpoint.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());
        let region_ptr = region.as_ref().map_or(std::ptr::null(), |c| c.as_ptr());
        let status = attempt.status();
        let status_code = u16::from(status.status_code());
        let sub_status = status
            .sub_status()
            .map_or(-1, |sub| i32::from(u16::from(sub)));
        visitor(
            user_data,
            endpoint_ptr,
            region_ptr,
            status_code,
            sub_status,
            attempt.duration_ms(),
            attempt.request_charge().value(),
            attempt.server_duration_ms().unwrap_or(-1.0),
        );
    }
}

/// Invokes `visitor` once per retained request attempt, in execution order.
///
/// The callback receives a versioned record containing the legacy attempt
/// fields plus execution, pipeline, transport, activity, session, send-state,
/// timeout, and error details. The record and its borrowed string views are
/// valid only for that callback invocation. When
/// [`cosmos_diagnostics_is_compacted`] is `true`, iteration covers only the
/// retained head/tail subset. Does nothing when `d` or `visitor` is NULL.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_iter_attempts_v2(
    d: *const CosmosDiagnostics,
    visitor: Option<
        extern "C" fn(user_data: *mut c_void, attempt: *const CosmosDiagnosticsAttemptV2),
    >,
    user_data: *mut c_void,
) {
    let Some(visitor) = visitor else {
        return;
    };
    if d.is_null() {
        return;
    }
    // SAFETY: see `cosmos_diagnostics_total_request_charge`.
    let handle = unsafe { &*d };
    let requests = handle.inner.requests();
    for (index, request) in requests.iter().enumerate() {
        let status = request.status();
        let server_duration_ms = request.server_duration_ms();
        let attempt = CosmosDiagnosticsAttemptV2 {
            struct_size_bytes: std::mem::size_of::<CosmosDiagnosticsAttemptV2>() as u32,
            schema_version: COSMOS_DIAGNOSTICS_ABI_VERSION_2,
            retained_index: u32::try_from(index).unwrap_or(u32::MAX),
            execution_context: execution_context_value(request.execution_context()),
            pipeline_type: pipeline_type_value(request.pipeline_type()),
            transport_security: transport_security_value(request.transport_security()),
            transport_kind: transport_kind_value(request.transport_kind()),
            transport_http_version: transport_http_version_value(request.transport_http_version()),
            request_sent: request_sent_value(request.request_sent()),
            local_shard_retry_count: request.local_shard_retry_count(),
            status_code: u16::from(status.status_code()),
            is_completed: u8::from(request.completed_at().is_some()),
            timed_out: u8::from(request.timed_out()),
            has_server_duration: u8::from(server_duration_ms.is_some()),
            reserved_flags: [0; 3],
            sub_status: status
                .sub_status()
                .map_or(-1, |value| i32::from(u16::from(value))),
            duration_ms: request.duration_ms(),
            request_charge: request.request_charge().value(),
            server_duration_ms: server_duration_ms.unwrap_or(0.0),
            endpoint: CosmosBorrowedStringView::from_str(request.endpoint()),
            region: CosmosBorrowedStringView::from_optional(
                request.region().map(|value| value.as_str()),
            ),
            operation_name: CosmosBorrowedStringView::from_optional(request.operation_name()),
            activity_id: CosmosBorrowedStringView::from_optional(
                request.activity_id().map(|value| value.as_str()),
            ),
            session_token: CosmosBorrowedStringView::from_optional(request.session_token()),
            error: CosmosBorrowedStringView::from_optional(request.error()),
            reserved: [0; 4],
        };
        visitor(user_data, &attempt);
    }
}

/// Verbosity selector for [`cosmos_diagnostics_to_json`].
///
/// A newtype over the wire integer (rather than a Rust `enum`) so an
/// unrecognized value is well-defined — it renders at
/// [`CosmosDiagnosticsVerbosity::DEFAULT`] instead of being undefined
/// behavior — matching the other integer-valued FFI selectors in this crate.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CosmosDiagnosticsVerbosity(pub i32);

impl CosmosDiagnosticsVerbosity {
    /// Render using the runtime's configured default verbosity.
    pub const DEFAULT: Self = Self(0);
    /// Render a compact, size-bounded summary.
    pub const SUMMARY: Self = Self(1);
    /// Render the full per-attempt detail.
    pub const DETAILED: Self = Self(2);

    fn to_driver(self) -> DiagnosticsVerbosity {
        // Values match the associated constants above; anything else falls
        // back to the runtime default rather than being undefined behavior.
        match self.0 {
            1 => DiagnosticsVerbosity::Summary,
            2 => DiagnosticsVerbosity::Detailed,
            _ => DiagnosticsVerbosity::Default,
        }
    }
}

/// Writes a borrowed, read-only JSON rendering of the diagnostics at the
/// requested `verbosity`.
///
/// On success writes `*out_data` / `*out_len` describing UTF-8 bytes (not
/// NUL-terminated) owned by the diagnostics handle and valid until the
/// completion is freed; the caller must not free them. An unrecognized
/// `verbosity` value renders at the runtime default. Returns a NULL-argument
/// error status without writing anything when `d`, `out_data`, or `out_len`
/// is NULL.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_to_json(
    d: *const CosmosDiagnostics,
    verbosity: CosmosDiagnosticsVerbosity,
    out_data: *mut *const u8,
    out_len: *mut usize,
) -> CosmosStatusCode {
    if d.is_null() || out_data.is_null() || out_len.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: see `cosmos_diagnostics_total_request_charge`.
    let handle = unsafe { &*d };
    // The rendered string is cached inside the diagnostics context, so the
    // borrowed bytes remain valid for as long as this handle (and thus the
    // owning completion) is alive — the same borrowed-view contract as the
    // completion body.
    let json = handle.inner.to_json_string(Some(verbosity.to_driver()));
    // SAFETY: `out_data` and `out_len` are non-NULL per the guard above.
    unsafe {
        *out_data = json.as_ptr();
        *out_len = json.len();
    }
    COSMOS_STATUS_SUCCESS
}

#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;
    use std::time::{Duration, Instant};

    use azure_core::http::StatusCode;
    use azure_data_cosmos_driver::options::Region;
    use azure_data_cosmos_driver::{ActivityId, CosmosStatus, RequestCharge, RequestDiagnostics};

    use super::*;

    // A NULL handle must be inert on every accessor — hosts routinely poll
    // `diagnostics` on completions that carry none (errors without attached
    // diagnostics, degenerate control-plane completions, end-of-feed shells).
    #[test]
    fn null_handle_accessors_are_inert() {
        let d: *const CosmosDiagnostics = std::ptr::null();
        assert_eq!(cosmos_diagnostics_total_request_charge(d), 0.0);
        assert_eq!(cosmos_diagnostics_total_elapsed_micros(d), 0);
        assert_eq!(cosmos_diagnostics_request_count(d), 0);
        assert_eq!(cosmos_diagnostics_retained_request_count(d), 0);
        assert!(!cosmos_diagnostics_is_compacted(d));
        assert!(!cosmos_diagnostics_is_completed(d));
        assert!(!cosmos_diagnostics_is_failure(d));

        let mut info = MaybeUninit::<CosmosDiagnosticsInfoV2>::uninit();
        assert_eq!(
            cosmos_diagnostics_get_info_v2(d, info.as_mut_ptr()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code()
        );
        assert_eq!(
            cosmos_diagnostics_get_info_v2(std::ptr::null(), std::ptr::null_mut()),
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code()
        );
    }

    extern "C" fn count_region(user_data: *mut c_void, _region_name: *const c_char) {
        // SAFETY: `user_data` points at the `u32` counter declared in the test.
        let counter = unsafe { &mut *(user_data as *mut u32) };
        *counter += 1;
    }

    extern "C" fn count_attempt(
        user_data: *mut c_void,
        _endpoint: *const c_char,
        _region: *const c_char,
        _status_code: u16,
        _sub_status: i32,
        _latency_ms: u64,
        _request_charge: f64,
        _server_duration_ms: f64,
    ) {
        // SAFETY: `user_data` points at the `u32` counter declared in the test.
        let counter = unsafe { &mut *(user_data as *mut u32) };
        *counter += 1;
    }

    #[test]
    fn null_handle_visitors_do_not_invoke_callback() {
        let d: *const CosmosDiagnostics = std::ptr::null();
        let mut regions = 0u32;
        cosmos_diagnostics_iter_regions_contacted(
            d,
            Some(count_region),
            &mut regions as *mut u32 as *mut c_void,
        );
        assert_eq!(regions, 0);

        let mut attempts = 0u32;
        cosmos_diagnostics_iter_attempts(
            d,
            Some(count_attempt),
            &mut attempts as *mut u32 as *mut c_void,
        );
        assert_eq!(attempts, 0);

        extern "C" fn count_attempt_v2(
            user_data: *mut c_void,
            _attempt: *const CosmosDiagnosticsAttemptV2,
        ) {
            // SAFETY: `user_data` points at the `u32` counter declared in the test.
            let counter = unsafe { &mut *(user_data as *mut u32) };
            *counter += 1;
        }
        cosmos_diagnostics_iter_attempts_v2(
            d,
            Some(count_attempt_v2),
            &mut attempts as *mut u32 as *mut c_void,
        );
        assert_eq!(attempts, 0);
    }

    #[test]
    fn null_visitor_is_a_no_op() {
        // Passing a NULL callback must not dereference the (also NULL) handle.
        cosmos_diagnostics_iter_regions_contacted(std::ptr::null(), None, std::ptr::null_mut());
        cosmos_diagnostics_iter_attempts(std::ptr::null(), None, std::ptr::null_mut());
        cosmos_diagnostics_iter_attempts_v2(std::ptr::null(), None, std::ptr::null_mut());
    }

    #[test]
    fn to_json_rejects_null_arguments() {
        let mut ptr: *const u8 = std::ptr::null();
        let mut len: usize = 0;
        let invalid = CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();

        // NULL handle.
        assert_eq!(
            cosmos_diagnostics_to_json(
                std::ptr::null(),
                CosmosDiagnosticsVerbosity::DEFAULT,
                &mut ptr,
                &mut len
            ),
            invalid
        );
        // NULL out pointers (handle NULL too, but the arg guard fires first).
        assert_eq!(
            cosmos_diagnostics_to_json(
                std::ptr::null(),
                CosmosDiagnosticsVerbosity::DEFAULT,
                std::ptr::null_mut(),
                &mut len
            ),
            invalid
        );
        assert_eq!(
            cosmos_diagnostics_to_json(
                std::ptr::null(),
                CosmosDiagnosticsVerbosity::DEFAULT,
                &mut ptr,
                std::ptr::null_mut()
            ),
            invalid
        );
    }

    // Builds a two-attempt diagnostics context (a throttled 429 retry, then a
    // 200 in a second region) so the accessor tests see aggregation across
    // attempts, regions, and per-attempt status/sub-status.
    fn populated() -> CosmosDiagnostics {
        let started = Instant::now();
        let throttled = CosmosStatus::new(StatusCode::from(429u16)).with_sub_status(3200);
        let ok = CosmosStatus::new(StatusCode::from(200u16));
        let first = RequestDiagnostics::for_testing(
            "https://acct.documents.azure.com",
            Some(Region::WEST_US),
            throttled,
            RequestCharge::new(2.5),
            started,
            started + Duration::from_millis(10),
        )
        .for_testing_with_operation_name("patch_read_item");
        let second = RequestDiagnostics::for_testing(
            "https://acct-westus2.documents.azure.com",
            Some(Region::WEST_US_2),
            ok,
            RequestCharge::new(4.0),
            started,
            started + Duration::from_millis(20),
        )
        .with_execution_context_for_testing(ExecutionContext::Hedging);
        let context = DiagnosticsContext::for_testing_with_requests(
            ActivityId::from_static("operation-activity"),
            Duration::from_millis(30),
            Some(ok),
            Some("patch_item"),
            vec![first, second],
        );
        CosmosDiagnostics::new(Arc::new(context))
    }

    #[test]
    fn scalar_accessors_report_aggregates() {
        let d = populated();
        let handle = &d as *const CosmosDiagnostics;
        // 2.5 + 4.0 request units across the two attempts.
        assert_eq!(cosmos_diagnostics_total_request_charge(handle), 6.5);
        // Operation-level duration was 30ms == 30_000us.
        assert_eq!(cosmos_diagnostics_total_elapsed_micros(handle), 30_000);
        // Two recorded attempts, none dropped by compaction.
        assert_eq!(cosmos_diagnostics_request_count(handle), 2);
        assert_eq!(cosmos_diagnostics_retained_request_count(handle), 2);
        assert!(!cosmos_diagnostics_is_compacted(handle));
        // Terminal, successful operation (final status 200 despite the 429 retry).
        assert!(cosmos_diagnostics_is_completed(handle));
        assert!(!cosmos_diagnostics_is_failure(handle));
    }

    extern "C" fn push_region(user_data: *mut c_void, region_name: *const c_char) {
        // SAFETY: `user_data` is the `Vec<String>` declared in the test;
        // `region_name` is a valid NUL-terminated string for this call.
        let names = unsafe { &mut *(user_data as *mut Vec<String>) };
        let name = unsafe { std::ffi::CStr::from_ptr(region_name) };
        names.push(name.to_string_lossy().into_owned());
    }

    #[test]
    fn regions_visitor_yields_each_region_once() {
        let d = populated();
        let mut names: Vec<String> = Vec::new();
        cosmos_diagnostics_iter_regions_contacted(
            &d,
            Some(push_region),
            &mut names as *mut Vec<String> as *mut c_void,
        );
        assert_eq!(names, vec!["westus".to_string(), "westus2".to_string()]);
    }

    struct Attempt {
        endpoint: String,
        region: Option<String>,
        status_code: u16,
        sub_status: i32,
        latency_ms: u64,
        request_charge: f64,
    }

    extern "C" fn push_attempt(
        user_data: *mut c_void,
        endpoint: *const c_char,
        region: *const c_char,
        status_code: u16,
        sub_status: i32,
        latency_ms: u64,
        request_charge: f64,
        _server_duration_ms: f64,
    ) {
        // SAFETY: `user_data` is the `Vec<Attempt>` declared in the test;
        // `endpoint` is non-NULL and `region` may be NULL per the contract.
        let attempts = unsafe { &mut *(user_data as *mut Vec<Attempt>) };
        let endpoint = unsafe { std::ffi::CStr::from_ptr(endpoint) }
            .to_string_lossy()
            .into_owned();
        let region = if region.is_null() {
            None
        } else {
            Some(
                unsafe { std::ffi::CStr::from_ptr(region) }
                    .to_string_lossy()
                    .into_owned(),
            )
        };
        attempts.push(Attempt {
            endpoint,
            region,
            status_code,
            sub_status,
            latency_ms,
            request_charge,
        });
    }

    #[test]
    fn attempts_visitor_yields_each_attempt_in_order() {
        let d = populated();
        let mut attempts: Vec<Attempt> = Vec::new();
        cosmos_diagnostics_iter_attempts(
            &d,
            Some(push_attempt),
            &mut attempts as *mut Vec<Attempt> as *mut c_void,
        );
        assert_eq!(attempts.len(), 2);

        assert_eq!(attempts[0].endpoint, "https://acct.documents.azure.com");
        assert_eq!(attempts[0].region.as_deref(), Some("westus"));
        assert_eq!(attempts[0].status_code, 429);
        assert_eq!(attempts[0].sub_status, 3200);
        assert_eq!(attempts[0].latency_ms, 10);
        assert_eq!(attempts[0].request_charge, 2.5);

        assert_eq!(
            attempts[1].endpoint,
            "https://acct-westus2.documents.azure.com"
        );
        assert_eq!(attempts[1].region.as_deref(), Some("westus2"));
        assert_eq!(attempts[1].status_code, 200);
        // No sub-status on the successful attempt maps to the -1 sentinel.
        assert_eq!(attempts[1].sub_status, -1);
        assert_eq!(attempts[1].latency_ms, 20);
        assert_eq!(attempts[1].request_charge, 4.0);
    }

    unsafe fn borrowed_string(view: CosmosBorrowedStringView) -> Option<String> {
        if view.data.is_null() {
            assert_eq!(view.len, 0);
            return None;
        }
        // SAFETY: the test copies the borrowed view during the accessor or
        // callback lifetime guaranteed by the function under test.
        let bytes = unsafe { std::slice::from_raw_parts(view.data, view.len) };
        Some(
            std::str::from_utf8(bytes)
                .expect("diagnostics strings are UTF-8")
                .to_owned(),
        )
    }

    #[test]
    fn structured_info_matches_driver_getters() {
        let d = populated();
        let mut info = MaybeUninit::<CosmosDiagnosticsInfoV2>::uninit();
        assert_eq!(
            cosmos_diagnostics_get_info_v2(&d, info.as_mut_ptr()),
            COSMOS_STATUS_SUCCESS
        );
        // SAFETY: success initializes the complete output record.
        let info = unsafe { info.assume_init() };
        assert_eq!(
            info.struct_size_bytes as usize,
            std::mem::size_of::<CosmosDiagnosticsInfoV2>()
        );
        assert_eq!(info.schema_version, COSMOS_DIAGNOSTICS_ABI_VERSION_2);
        // SAFETY: both views borrow `d`, which remains alive for the test.
        assert_eq!(
            unsafe { borrowed_string(info.activity_id) }.as_deref(),
            Some(d.inner.activity_id().as_str())
        );
        // SAFETY: both views borrow `d`, which remains alive for the test.
        assert_eq!(
            unsafe { borrowed_string(info.operation_name) }.as_deref(),
            d.inner.operation_name()
        );
        let status = d.inner.status().expect("populated status");
        assert_eq!(info.status_code, u16::from(status.status_code()));
        assert_eq!(info.sub_status, -1);
        assert_eq!(info.reserved_status, 0);
        assert_eq!(info.reserved, [0; 4]);
    }

    #[derive(Debug, PartialEq)]
    struct AttemptV2 {
        retained_index: u32,
        execution_context: i32,
        pipeline_type: i32,
        transport_security: i32,
        transport_kind: i32,
        transport_http_version: i32,
        request_sent: i32,
        local_shard_retry_count: u32,
        status_code: u16,
        is_completed: u8,
        timed_out: u8,
        has_server_duration: u8,
        sub_status: i32,
        duration_ms: u64,
        request_charge: f64,
        server_duration_ms: f64,
        endpoint: Option<String>,
        region: Option<String>,
        operation_name: Option<String>,
        activity_id: Option<String>,
        session_token: Option<String>,
        error: Option<String>,
    }

    extern "C" fn push_attempt_v2(
        user_data: *mut c_void,
        attempt: *const CosmosDiagnosticsAttemptV2,
    ) {
        assert!(!attempt.is_null());
        // SAFETY: the iterator supplies a readable record for this callback.
        let attempt = unsafe { &*attempt };
        assert_eq!(
            attempt.struct_size_bytes as usize,
            std::mem::size_of::<CosmosDiagnosticsAttemptV2>()
        );
        assert_eq!(attempt.schema_version, COSMOS_DIAGNOSTICS_ABI_VERSION_2);
        assert_eq!(attempt.reserved_flags, [0; 3]);
        assert_eq!(attempt.reserved, [0; 4]);
        // SAFETY: `user_data` points at the `Vec<AttemptV2>` declared in the test.
        let attempts = unsafe { &mut *(user_data as *mut Vec<AttemptV2>) };
        // SAFETY: every view is copied during the callback lifetime.
        attempts.push(AttemptV2 {
            retained_index: attempt.retained_index,
            execution_context: attempt.execution_context,
            pipeline_type: attempt.pipeline_type,
            transport_security: attempt.transport_security,
            transport_kind: attempt.transport_kind,
            transport_http_version: attempt.transport_http_version,
            request_sent: attempt.request_sent,
            local_shard_retry_count: attempt.local_shard_retry_count,
            status_code: attempt.status_code,
            is_completed: attempt.is_completed,
            timed_out: attempt.timed_out,
            has_server_duration: attempt.has_server_duration,
            sub_status: attempt.sub_status,
            duration_ms: attempt.duration_ms,
            request_charge: attempt.request_charge,
            server_duration_ms: attempt.server_duration_ms,
            endpoint: unsafe { borrowed_string(attempt.endpoint) },
            region: unsafe { borrowed_string(attempt.region) },
            operation_name: unsafe { borrowed_string(attempt.operation_name) },
            activity_id: unsafe { borrowed_string(attempt.activity_id) },
            session_token: unsafe { borrowed_string(attempt.session_token) },
            error: unsafe { borrowed_string(attempt.error) },
        });
    }

    #[test]
    fn structured_attempts_match_driver_getters_and_order() {
        let d = populated();
        let requests = d.inner.requests();
        let mut actual: Vec<AttemptV2> = Vec::new();
        cosmos_diagnostics_iter_attempts_v2(
            &d,
            Some(push_attempt_v2),
            &mut actual as *mut Vec<AttemptV2> as *mut c_void,
        );
        assert_eq!(actual.len(), requests.len());

        for (index, (actual, expected)) in actual.iter().zip(requests.iter()).enumerate() {
            assert_eq!(actual.retained_index, index as u32);
            assert_eq!(
                actual.execution_context,
                execution_context_value(expected.execution_context())
            );
            assert_eq!(
                actual.pipeline_type,
                pipeline_type_value(expected.pipeline_type())
            );
            assert_eq!(
                actual.transport_security,
                transport_security_value(expected.transport_security())
            );
            assert_eq!(
                actual.transport_kind,
                transport_kind_value(expected.transport_kind())
            );
            assert_eq!(
                actual.transport_http_version,
                transport_http_version_value(expected.transport_http_version())
            );
            assert_eq!(
                actual.request_sent,
                request_sent_value(expected.request_sent())
            );
            assert_eq!(
                actual.local_shard_retry_count,
                expected.local_shard_retry_count()
            );
            assert_eq!(
                actual.status_code,
                u16::from(expected.status().status_code())
            );
            assert_eq!(
                actual.is_completed,
                u8::from(expected.completed_at().is_some())
            );
            assert_eq!(actual.timed_out, u8::from(expected.timed_out()));
            assert_eq!(
                actual.has_server_duration,
                u8::from(expected.server_duration_ms().is_some())
            );
            assert_eq!(
                actual.sub_status,
                expected
                    .status()
                    .sub_status()
                    .map_or(-1, |value| i32::from(u16::from(value)))
            );
            assert_eq!(actual.duration_ms, expected.duration_ms());
            assert_eq!(actual.request_charge, expected.request_charge().value());
            assert_eq!(
                actual.server_duration_ms,
                expected.server_duration_ms().unwrap_or(0.0)
            );
            assert_eq!(actual.endpoint.as_deref(), Some(expected.endpoint()));
            assert_eq!(
                actual.region.as_deref(),
                expected.region().map(|value| value.as_str())
            );
            assert_eq!(actual.operation_name.as_deref(), expected.operation_name());
            assert_eq!(
                actual.activity_id.as_deref(),
                expected.activity_id().map(|value| value.as_str())
            );
            assert_eq!(actual.session_token.as_deref(), expected.session_token());
            assert_eq!(actual.error.as_deref(), expected.error());
        }

        assert_eq!(actual[0].execution_context, 1);
        assert_eq!(actual[0].operation_name.as_deref(), Some("patch_read_item"));
        assert_eq!(actual[1].execution_context, 4);
        assert_eq!(actual[1].operation_name, None);
    }

    #[test]
    fn structured_record_layout_is_pinned() {
        assert_eq!(
            std::mem::offset_of!(CosmosDiagnosticsInfoV2, struct_size_bytes),
            0
        );
        assert_eq!(
            std::mem::offset_of!(CosmosDiagnosticsInfoV2, schema_version),
            4
        );
        assert_eq!(
            std::mem::offset_of!(CosmosDiagnosticsAttemptV2, struct_size_bytes),
            0
        );
        assert_eq!(
            std::mem::offset_of!(CosmosDiagnosticsAttemptV2, schema_version),
            4
        );
        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(std::mem::size_of::<CosmosBorrowedStringView>(), 16);
            assert_eq!(std::mem::size_of::<CosmosDiagnosticsInfoV2>(), 64);
            assert_eq!(std::mem::size_of::<CosmosDiagnosticsAttemptV2>(), 192);
            assert_eq!(
                std::mem::offset_of!(CosmosDiagnosticsAttemptV2, endpoint),
                80
            );
        }
    }

    #[test]
    fn to_json_returns_borrowed_bytes() {
        let d = populated();
        let mut ptr: *const u8 = std::ptr::null();
        let mut len: usize = 0;
        assert_eq!(
            cosmos_diagnostics_to_json(&d, CosmosDiagnosticsVerbosity::DEFAULT, &mut ptr, &mut len),
            COSMOS_STATUS_SUCCESS
        );
        assert!(!ptr.is_null());
        assert!(len > 0);
        // SAFETY: the accessor returned success, so `ptr`/`len` describe UTF-8
        // bytes borrowed from `d`, which outlives this borrow.
        let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
        let json = std::str::from_utf8(bytes).expect("diagnostics JSON is UTF-8");
        assert!(json.contains('{'));
    }

    #[test]
    fn to_json_honors_verbosity_and_defaults_unknown() {
        let d = populated();
        for verbosity in [
            CosmosDiagnosticsVerbosity::DEFAULT,
            CosmosDiagnosticsVerbosity::SUMMARY,
            CosmosDiagnosticsVerbosity::DETAILED,
            // An out-of-range value must be well-defined (renders at the
            // runtime default), never undefined behavior.
            CosmosDiagnosticsVerbosity(99),
        ] {
            let mut ptr: *const u8 = std::ptr::null();
            let mut len: usize = 0;
            assert_eq!(
                cosmos_diagnostics_to_json(&d, verbosity, &mut ptr, &mut len),
                COSMOS_STATUS_SUCCESS
            );
            assert!(!ptr.is_null());
            assert!(len > 0);
            // SAFETY: success means `ptr`/`len` borrow UTF-8 bytes from `d`.
            let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
            let json = std::str::from_utf8(bytes).expect("diagnostics JSON is UTF-8");
            assert!(json.contains('{'));
        }
    }
}
