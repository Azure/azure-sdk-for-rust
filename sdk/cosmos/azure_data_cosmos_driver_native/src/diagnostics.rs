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

use azure_data_cosmos_driver::{options::DiagnosticsVerbosity, DiagnosticsContext};

use crate::error::{CosmosErrorCode, CosmosStatusCode, COSMOS_STATUS_SUCCESS};

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
    }

    #[test]
    fn null_visitor_is_a_no_op() {
        // Passing a NULL callback must not dereference the (also NULL) handle.
        cosmos_diagnostics_iter_regions_contacted(std::ptr::null(), None, std::ptr::null_mut());
        cosmos_diagnostics_iter_attempts(std::ptr::null(), None, std::ptr::null_mut());
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
        );
        let second = RequestDiagnostics::for_testing(
            "https://acct-westus2.documents.azure.com",
            Some(Region::WEST_US_2),
            ok,
            RequestCharge::new(4.0),
            started,
            started + Duration::from_millis(20),
        );
        let context = DiagnosticsContext::for_testing_with_requests(
            ActivityId::new_uuid(),
            Duration::from_millis(30),
            Some(ok),
            None,
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
