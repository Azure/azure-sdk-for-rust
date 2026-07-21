// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Unified error / status model for the C ABI boundary.
//!
//! Both surfaces are derived from the driver's canonical
//! [`azure_data_cosmos_driver::error::CosmosStatus`] and
//! [`azure_data_cosmos_driver::error::CosmosError`], so every consuming SDK
//! learns one taxonomy instead of a parallel FFI-specific one:
//!
//! - **Packed status code** ([`CosmosStatusCode`] / `cosmos_status_code_t` in
//!   C). Every fallible C function returns this 32-bit integer. The encoding is
//!   `(http_status << 16) | sub_status`, where a low-16-bit value of
//!   [`COSMOS_STATUS_NO_SUB_STATUS`] (`0xFFFF`) means "no sub-status" and `0`
//!   means success. Pure-FFI / pre-flight failures (a NULL argument, invalid
//!   UTF-8, a shut-down completion queue, …) use a real HTTP status paired with
//!   a driver `CLIENT_FFI_*` (or `CLIENT_*`) sub-status, so they fit the same
//!   integer as service errors.
//! - **Flat rich error** ([`CosmosError`] / `cosmos_error_t` in C). An owned
//!   `#[repr(C)]` struct that carries the packed status plus the message and
//!   wire diagnostics inline, mirroring `cosmos_completion_t`. It is produced
//!   through the synchronous `out_error` slots and freed with
//!   [`cosmos_error_free`].

use std::ffi::{c_char, CString};

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::error::{
    CosmosError as DriverCosmosError, CosmosStatus, SubStatusCode,
};

// ─────────────────────────────────────────────────────────────────────────────
// Packed status code
// ─────────────────────────────────────────────────────────────────────────────

/// 32-bit packed Cosmos status returned by every fallible C function.
///
/// Layout: `(http_status << 16) | sub_status`. `0` is success. A low-16-bit
/// value of [`COSMOS_STATUS_NO_SUB_STATUS`] (`0xFFFF`) means "no sub-status".
/// Decode on the host with `http = code >> 16` and
/// `sub = code & 0xFFFF` (treating `0xFFFF` as absent).
pub type CosmosStatusCode = i32;

/// Success sentinel returned by fallible C functions.
pub const COSMOS_STATUS_SUCCESS: CosmosStatusCode = 0;

/// Low-16-bit sentinel meaning "no sub-status present" in a packed
/// [`CosmosStatusCode`].
pub const COSMOS_STATUS_NO_SUB_STATUS: u16 = 0xFFFF;

/// Packs a driver [`CosmosStatus`] into the FFI [`CosmosStatusCode`].
pub(crate) fn status_code(status: CosmosStatus) -> CosmosStatusCode {
    let http = u32::from(u16::from(status.status_code()));
    let sub = status
        .sub_status()
        .map_or(u32::from(COSMOS_STATUS_NO_SUB_STATUS), |s| {
            u32::from(s.value())
        });
    ((http << 16) | sub) as CosmosStatusCode
}

/// Packs the status of a driver [`CosmosError`] into a [`CosmosStatusCode`].
pub(crate) fn driver_status_code(err: &DriverCosmosError) -> CosmosStatusCode {
    status_code(err.status())
}

// ─────────────────────────────────────────────────────────────────────────────
// Well-known synthetic sub-status codes (single source of truth)
// ─────────────────────────────────────────────────────────────────────────────

/// Named mirror of the driver's synthetic (`2xxxx`) sub-status codes.
///
/// The Cosmos service returns real sub-status codes for wire failures, but the
/// driver also *synthesizes* sub-status codes in the `20000`–`21999` band to
/// describe client-, transport-, serialization-, auth- and FFI-side conditions
/// that never traveled over the wire. A C host decoding the low 16 bits of a
/// [`CosmosStatusCode`] would otherwise see these as bare magic numbers, so this
/// enum re-exports every one of them as a `cosmos_sub_status_t` /
/// `COSMOS_SUB_STATUS_*` constant in the generated header.
///
/// This enum is the **single source of truth** for those names on the C side.
/// Each discriminant is a literal copy of the corresponding
/// [`azure_data_cosmos_driver::error::SubStatusCode`] constant (cbindgen needs
/// literals to emit `= N`). The `sub_status_mirror_matches_driver` unit test is
/// CI-verified to fail the build if any value drifts from the driver, so the two
/// tables can never disagree silently. When the driver adds a new synthetic
/// `2xxxx` sub-status, add the matching variant here (and the test will remind
/// you if you forget by value mismatch on an existing one).
///
/// The values are *not* exhaustive of the `2xxxx` range: the driver leaves gaps
/// (e.g. `20009`, `20013`, `20103`) for future use. Do not invent variants for
/// codes the driver does not define.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CosmosSubStatus {
    /// `TRANSPORT_GENERATED_503` (20003).
    CosmosSubStatusTransportGenerated503 = 20003,
    /// `CLIENT_CPU_OVERLOAD` (20004).
    CosmosSubStatusClientCpuOverload = 20004,
    /// `CLIENT_THREAD_STARVATION` (20005).
    CosmosSubStatusClientThreadStarvation = 20005,
    /// `CHANNEL_CLOSED` (20006).
    CosmosSubStatusChannelClosed = 20006,
    /// `MALFORMED_CONTINUATION_TOKEN` (20007).
    CosmosSubStatusMalformedContinuationToken = 20007,
    /// `CLIENT_OPERATION_TIMEOUT` (20008).
    CosmosSubStatusClientOperationTimeout = 20008,
    /// `TRANSPORT_CONNECTION_FAILED` (20010).
    CosmosSubStatusTransportConnectionFailed = 20010,
    /// `TRANSPORT_IO_FAILED` (20011).
    CosmosSubStatusTransportIoFailed = 20011,
    /// `TRANSPORT_DNS_FAILED` (20012).
    CosmosSubStatusTransportDnsFailed = 20012,
    /// `TRANSPORT_BODY_READ_FAILED` (20014).
    CosmosSubStatusTransportBodyReadFailed = 20014,
    /// `TRANSPORT_HTTP2_INCOMPATIBLE` (20015).
    CosmosSubStatusTransportHttp2Incompatible = 20015,
    /// `SERIALIZATION_RESPONSE_BODY_INVALID` (20020).
    CosmosSubStatusSerializationResponseBodyInvalid = 20020,
    /// `CLIENT_PARTITION_KEY_EMPTY` (20100).
    CosmosSubStatusClientPartitionKeyEmpty = 20100,
    /// `CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS` (20101).
    CosmosSubStatusClientPartitionKeyTooManyComponents = 20101,
    /// `CLIENT_PREFIX_PARTITION_KEY_REQUIRES_MULTIHASH` (20102).
    CosmosSubStatusClientPrefixPartitionKeyRequiresMultihash = 20102,
    /// `CLIENT_CONNECTION_STRING_EMPTY` (20104).
    CosmosSubStatusClientConnectionStringEmpty = 20104,
    /// `CLIENT_CONNECTION_STRING_MALFORMED_PART` (20105).
    CosmosSubStatusClientConnectionStringMalformedPart = 20105,
    /// `CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_KEY` (20107).
    CosmosSubStatusClientConnectionStringMissingAccountKey = 20107,
    /// `CLIENT_INVALID_ACCOUNT_ENDPOINT_URL` (20108).
    CosmosSubStatusClientInvalidAccountEndpointUrl = 20108,
    /// `CLIENT_INVALID_URL` (20109).
    CosmosSubStatusClientInvalidUrl = 20109,
    /// `CLIENT_UNKNOWN_CONSISTENCY_LEVEL` (20110).
    CosmosSubStatusClientUnknownConsistencyLevel = 20110,
    /// `CLIENT_UNKNOWN_PRIORITY_LEVEL` (20111).
    CosmosSubStatusClientUnknownPriorityLevel = 20111,
    /// `CLIENT_FEED_RANGE_REQUIRES_FANOUT_PIPELINE` (20112).
    CosmosSubStatusClientFeedRangeRequiresFanoutPipeline = 20112,
    /// `CLIENT_UNSUPPORTED_QUERY_FEATURE` (20113).
    CosmosSubStatusClientUnsupportedQueryFeature = 20113,
    /// `CLIENT_QUERY_PLAN_INVALID_TOP_OFFSET_LIMIT` (20114).
    CosmosSubStatusClientQueryPlanInvalidTopOffsetLimit = 20114,
    /// `CLIENT_CONTINUATION_TOKEN_NON_QUERY_OPERATION` (20117).
    CosmosSubStatusClientContinuationTokenNonQueryOperation = 20117,
    /// `CLIENT_DUPLICATE_FAULT_INJECTION_RULE_ID` (20150).
    CosmosSubStatusClientDuplicateFaultInjectionRuleId = 20150,
    /// `CLIENT_THROUGHPUT_CONTROL_GROUP_NOT_REGISTERED` (20152).
    CosmosSubStatusClientThroughputControlGroupNotRegistered = 20152,
    /// `CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED` (20153).
    CosmosSubStatusClientHttpClientConstructionFailed = 20153,
    /// `CLIENT_REQWEST_FEATURE_REQUIRED` (20154).
    CosmosSubStatusClientReqwestFeatureRequired = 20154,
    /// `CLIENT_REQUEST_URL_MISSING_HOST` (20155).
    CosmosSubStatusClientRequestUrlMissingHost = 20155,
    /// `CLIENT_REQUEST_URL_MISSING_KNOWN_PORT` (20156).
    CosmosSubStatusClientRequestUrlMissingKnownPort = 20156,
    /// `CLIENT_IMDS_HTTP_CLIENT_CONSTRUCTION_FAILED` (20157).
    CosmosSubStatusClientImdsHttpClientConstructionFailed = 20157,
    /// `CLIENT_IMDS_REQWEST_FEATURE_REQUIRED` (20158).
    CosmosSubStatusClientImdsReqwestFeatureRequired = 20158,
    /// `CLIENT_CONTINUATION_TOKEN_FETCH_IN_FLIGHT` (20200).
    CosmosSubStatusClientContinuationTokenFetchInFlight = 20200,
    /// `CLIENT_TOPOLOGY_PROVIDER_MISSING` (20201).
    CosmosSubStatusClientTopologyProviderMissing = 20201,
    /// `CLIENT_DRIVER_NOT_INITIALIZED` (20202).
    CosmosSubStatusClientDriverNotInitialized = 20202,
    /// `CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH` (20203).
    CosmosSubStatusClientContinuationTokenShapeMismatch = 20203,
    /// `CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE` (20205).
    CosmosSubStatusClientContinuationTokenInvalidEpkRange = 20205,
    /// `CLIENT_SPLIT_RETRIES_EXHAUSTED` (20206).
    CosmosSubStatusClientSplitRetriesExhausted = 20206,
    /// `CLIENT_BUILD_RESPONSE_INVOKED_ON_FAILURE` (20207).
    CosmosSubStatusClientBuildResponseInvokedOnFailure = 20207,
    /// `CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT` (20208).
    CosmosSubStatusClientRootNodeCannotRequestSplit = 20208,
    /// `CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE` (20210).
    CosmosSubStatusClientSingletonOperationReturnedEmptyPage = 20210,
    /// `CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED` (20213).
    CosmosSubStatusClientContinuationTokenSavedRangeUnhonored = 20213,
    /// `CLIENT_NO_THROUGHPUT_OFFER_FOR_RESOURCE` (20301).
    CosmosSubStatusClientNoThroughputOfferForResource = 20301,
    /// `CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES` (20302).
    CosmosSubStatusClientQueryPlanProducedEmptyRanges = 20302,
    /// `SERVICE_RETURNED_OFFER_WITHOUT_ID` (20303).
    CosmosSubStatusServiceReturnedOfferWithoutId = 20303,
    /// `CLIENT_THROUGHPUT_POLLER_INCOMPLETE` (20304).
    CosmosSubStatusClientThroughputPollerIncomplete = 20304,
    /// `CLIENT_TOPOLOGY_RESOLUTION_FAILED` (20305).
    CosmosSubStatusClientTopologyResolutionFailed = 20305,
    /// `SERVICE_RETURNED_OBJECT_WITHOUT_RID` (20306).
    CosmosSubStatusServiceReturnedObjectWithoutRid = 20306,
    /// `CLIENT_FFI_NULL_ARGUMENT` (20350).
    CosmosSubStatusClientFfiNullArgument = 20350,
    /// `CLIENT_FFI_INVALID_UTF8` (20351).
    CosmosSubStatusClientFfiInvalidUtf8 = 20351,
    /// `CLIENT_FFI_INVALID_HEADER` (20352).
    CosmosSubStatusClientFfiInvalidHeader = 20352,
    /// `CLIENT_FFI_INVALID_OPTION_VALUE` (20353).
    CosmosSubStatusClientFfiInvalidOptionValue = 20353,
    /// `CLIENT_FFI_OPERATION_CONSUMED` (20354).
    CosmosSubStatusClientFfiOperationConsumed = 20354,
    /// `CLIENT_FFI_PRECONDITION_ALREADY_SET` (20355).
    CosmosSubStatusClientFfiPreconditionAlreadySet = 20355,
    /// `CLIENT_FFI_UNSUPPORTED_OPERATION_FOR_MUTATOR` (20356).
    CosmosSubStatusClientFfiUnsupportedOperationForMutator = 20356,
    /// `CLIENT_FFI_FEED_EXHAUSTED` (20357).
    CosmosSubStatusClientFfiFeedExhausted = 20357,
    /// `CLIENT_FFI_QUEUE_SHUTDOWN` (20358).
    CosmosSubStatusClientFfiQueueShutdown = 20358,
    /// `CLIENT_FFI_QUEUE_FULL` (20359).
    CosmosSubStatusClientFfiQueueFull = 20359,
    /// `CLIENT_FFI_OPERATION_CANCELLED` (20360).
    CosmosSubStatusClientFfiOperationCancelled = 20360,
    /// `CLIENT_FFI_RUNTIME_BUILD_FAILED` (20361).
    CosmosSubStatusClientFfiRuntimeBuildFailed = 20361,
    /// `CLIENT_FFI_PANIC` (20362).
    CosmosSubStatusClientFfiPanic = 20362,
    /// `CLIENT_GENERATED_401` (20401).
    CosmosSubStatusClientGenerated401 = 20401,
    /// `AUTHENTICATION_TOKEN_ACQUISITION_FAILED` (20402).
    CosmosSubStatusAuthenticationTokenAcquisitionFailed = 20402,
    /// `TRANSIT_TIMEOUT` (20911).
    CosmosSubStatusTransitTimeout = 20911,
    /// `SERVER_BARRIER_THROTTLED` (21011).
    CosmosSubStatusServerBarrierThrottled = 21011,
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI-boundary condition set
// ─────────────────────────────────────────────────────────────────────────────

/// Internal condition set for pre-flight / plumbing failures that do **not**
/// originate from a driver [`CosmosError`].
///
/// This is **not** part of the C ABI (it is `pub(crate)` and never exported to
/// the generated header). Each condition maps to a driver [`CosmosStatus`] — a
/// real HTTP status paired with a `CLIENT_FFI_*` / `CLIENT_*` sub-status — and
/// is returned across the boundary as a packed [`CosmosStatusCode`] via
/// [`CosmosErrorCode::as_i32`]. Keeping the mapping here lets the whole ABI
/// share the driver's single status taxonomy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CosmosErrorCode {
    /// Operation completed successfully.
    CosmosErrorCodeSuccess,
    /// A required pointer argument was `NULL`.
    CosmosErrorCodeInvalidArgument,
    /// A `*const c_char` argument was not valid UTF-8.
    CosmosErrorCodeInvalidUtf8,
    /// A builder setter was passed a value outside its documented range.
    CosmosErrorCodeInvalidOptionValue,
    /// A partition-key builder produced an empty / inconsistent key.
    CosmosErrorCodeInvalidPartitionKey,
    /// An account endpoint URL or credential could not be parsed.
    CosmosErrorCodeInvalidAccountReference,
    /// An operation was cancelled before it completed.
    CosmosErrorCodeOperationCancelled,
    /// A submit targeted a completion queue that was already shut down.
    CosmosErrorCodeQueueShutdown,
    /// A submit targeted a completion queue already at its hard capacity.
    CosmosErrorCodeQueueFull,
    /// The underlying driver runtime could not be constructed.
    CosmosErrorCodeRuntimeBuildFailed,
    /// A driver future spawned by the wrapper panicked; the panic firewall
    /// synthesized a failure so the host continuation is released.
    CosmosErrorCodeInternalError,
}

impl CosmosErrorCode {
    /// The driver [`CosmosStatus`] this condition maps to, or `None` for
    /// success.
    fn to_status(self) -> Option<CosmosStatus> {
        let (status_code, sub_status) = match self {
            Self::CosmosErrorCodeSuccess => return None,
            Self::CosmosErrorCodeInvalidArgument => (
                StatusCode::BadRequest,
                SubStatusCode::CLIENT_FFI_NULL_ARGUMENT,
            ),
            Self::CosmosErrorCodeInvalidUtf8 => (
                StatusCode::BadRequest,
                SubStatusCode::CLIENT_FFI_INVALID_UTF8,
            ),
            Self::CosmosErrorCodeInvalidOptionValue => (
                StatusCode::BadRequest,
                SubStatusCode::CLIENT_FFI_INVALID_OPTION_VALUE,
            ),
            Self::CosmosErrorCodeInvalidPartitionKey => (
                StatusCode::BadRequest,
                SubStatusCode::CLIENT_PARTITION_KEY_EMPTY,
            ),
            Self::CosmosErrorCodeInvalidAccountReference => (
                StatusCode::BadRequest,
                SubStatusCode::CLIENT_INVALID_ACCOUNT_ENDPOINT_URL,
            ),
            Self::CosmosErrorCodeOperationCancelled => (
                StatusCode::RequestTimeout,
                SubStatusCode::CLIENT_FFI_OPERATION_CANCELLED,
            ),
            Self::CosmosErrorCodeQueueShutdown => (
                StatusCode::ServiceUnavailable,
                SubStatusCode::CLIENT_FFI_QUEUE_SHUTDOWN,
            ),
            Self::CosmosErrorCodeQueueFull => (
                StatusCode::ServiceUnavailable,
                SubStatusCode::CLIENT_FFI_QUEUE_FULL,
            ),
            Self::CosmosErrorCodeRuntimeBuildFailed => (
                StatusCode::InternalServerError,
                SubStatusCode::CLIENT_FFI_RUNTIME_BUILD_FAILED,
            ),
            Self::CosmosErrorCodeInternalError => (
                StatusCode::InternalServerError,
                SubStatusCode::CLIENT_FFI_PANIC,
            ),
        };
        Some(CosmosStatus::new(status_code).with_sub_status(sub_status.value()))
    }

    /// The packed [`CosmosStatusCode`] this condition is returned as across the
    /// FFI boundary.
    #[inline]
    pub(crate) fn as_i32(self) -> CosmosStatusCode {
        self.to_status().map_or(COSMOS_STATUS_SUCCESS, status_code)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Flat rich error (`cosmos_error_t`)
// ─────────────────────────────────────────────────────────────────────────────

/// Owned, flat rich error handed back through the synchronous `out_error`
/// slots (`cosmos_error_t`).
///
/// Mirrors the inline error fields of `cosmos_completion_t`. Every pointer
/// field is **owned**; free the whole struct — and its strings — with
/// [`cosmos_error_free`]. A `NULL` pointer field means that field was absent.
#[repr(C)]
pub struct CosmosError {
    /// Packed 32-bit status (`(http << 16) | sub_status`). See
    /// [`CosmosStatusCode`].
    pub status: CosmosStatusCode,
    /// Wire HTTP status code (always populated, including for synthetic
    /// errors).
    pub http_status_code: u16,
    /// Cosmos sub-status code, or `-1` when absent.
    pub sub_status: i32,
    /// `1` iff the error originated from a service wire response.
    pub is_from_wire: u8,
    /// Retry-after hint in milliseconds, or `-1` when absent.
    pub retry_after_ms: i64,
    /// Owned NUL-terminated message (never NULL for a real error).
    pub message: *mut c_char,
    /// Owned activity id from the wire response headers, or NULL.
    pub activity_id: *mut c_char,
    /// Owned session token from the wire response headers, or NULL.
    pub session_token: *mut c_char,
    /// Owned ETag from the wire response headers, or NULL.
    pub etag: *mut c_char,
    /// Owned backtrace, or NULL when none was captured.
    pub backtrace: *mut c_char,
}

/// Builds a NUL-terminated copy of `s`, stripping any interior NUL bytes so the
/// conversion cannot fail.
fn to_cstring(s: impl Into<String>) -> Option<CString> {
    CString::new(s.into().replace('\0', "")).ok()
}

/// Consumes an optional `CString` into an owned raw pointer, or NULL.
fn cstring_into_raw(s: Option<CString>) -> *mut c_char {
    s.map_or(std::ptr::null_mut(), CString::into_raw)
}

/// Reclaims a raw pointer previously produced by [`cstring_into_raw`].
///
/// # Safety
///
/// `p` must be NULL or a pointer obtained from [`CString::into_raw`] that has
/// not already been reclaimed.
unsafe fn free_cstring(p: *mut c_char) {
    if !p.is_null() {
        drop(CString::from_raw(p));
    }
}

impl CosmosError {
    /// Builds an owned flat `cosmos_error_t` from a driver error and returns a
    /// raw pointer suitable for handing across the C boundary. Free it with
    /// [`cosmos_error_free`].
    pub(crate) fn into_raw(err: DriverCosmosError) -> *mut CosmosError {
        let status = err.status();
        let http_status_code = u16::from(status.status_code());
        let sub_status = status.sub_status().map_or(-1, |s| i32::from(s.value()));
        let is_from_wire = u8::from(err.is_from_wire());
        let message = to_cstring(err.to_string());
        let backtrace = err
            .backtrace()
            .and_then(|bt| to_cstring(bt.as_ref().to_string()));

        let (activity_id, session_token, etag, retry_after_ms) = match err.response() {
            Some(resp) => {
                let headers = resp.headers();
                (
                    headers
                        .activity_id
                        .as_ref()
                        .and_then(|a| to_cstring(a.as_str())),
                    headers
                        .session_token
                        .as_ref()
                        .and_then(|t| to_cstring(t.as_str())),
                    headers
                        .etag
                        .as_ref()
                        .and_then(|e| to_cstring(e.to_string())),
                    headers
                        .retry_after_ms
                        .map_or(-1, |ms| i64::try_from(ms).unwrap_or(i64::MAX)),
                )
            }
            None => (None, None, None, -1),
        };

        Box::into_raw(Box::new(CosmosError {
            status: status_code(status),
            http_status_code,
            sub_status,
            is_from_wire,
            retry_after_ms,
            message: cstring_into_raw(message),
            activity_id: cstring_into_raw(activity_id),
            session_token: cstring_into_raw(session_token),
            etag: cstring_into_raw(etag),
            backtrace: cstring_into_raw(backtrace),
        }))
    }
}

/// Frees a `cosmos_error_t *` obtained from a synchronous `out_error` slot,
/// including all of its owned strings. NULL is a no-op.
#[no_mangle]
pub extern "C" fn cosmos_error_free(e: *mut CosmosError) {
    if e.is_null() {
        return;
    }
    tracing::trace!(?e, "freeing cosmos_error_t");
    // SAFETY: caller guarantees `e` was obtained from a library `out_error`
    // slot and has not already been freed.
    let boxed = unsafe { Box::from_raw(e) };
    // SAFETY: each pointer field was produced by `cstring_into_raw` and is
    // reclaimed exactly once here.
    unsafe {
        free_cstring(boxed.message);
        free_cstring(boxed.activity_id);
        free_cstring(boxed.session_token);
        free_cstring(boxed.etag);
        free_cstring(boxed.backtrace);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Process-global backtrace knobs (spec section 6.4).
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

    /// Decodes a packed status back into `(http, Option<sub>)`.
    fn unpack(code: CosmosStatusCode) -> (u16, Option<u16>) {
        let http = ((code as u32) >> 16) as u16;
        let sub = (code as u32 & 0xFFFF) as u16;
        let sub = (sub != COSMOS_STATUS_NO_SUB_STATUS).then_some(sub);
        (http, sub)
    }

    #[test]
    fn success_packs_to_zero() {
        assert_eq!(CosmosErrorCode::CosmosErrorCodeSuccess.as_i32(), 0);
    }

    #[test]
    fn status_code_round_trips_with_sub_status() {
        let status = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
        let packed = status_code(status);
        assert_eq!(unpack(packed), (429, Some(3200)));
    }

    #[test]
    fn status_code_round_trips_without_sub_status() {
        let packed = status_code(CosmosStatus::new(StatusCode::NotFound));
        assert_eq!(unpack(packed), (404, None));
    }

    #[test]
    fn ffi_conditions_carry_client_ffi_sub_status() {
        // A NULL-argument pre-flight failure is a real 400 + CLIENT_FFI_*.
        let packed = CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_i32();
        assert_eq!(
            unpack(packed),
            (400, Some(SubStatusCode::CLIENT_FFI_NULL_ARGUMENT.value()))
        );

        let cancelled = CosmosErrorCode::CosmosErrorCodeOperationCancelled.as_i32();
        assert_eq!(
            unpack(cancelled),
            (
                408,
                Some(SubStatusCode::CLIENT_FFI_OPERATION_CANCELLED.value())
            )
        );
    }

    #[test]
    fn driver_status_code_packs_wire_status() {
        let err = DriverCosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::Conflict))
            .with_message("conflict")
            .build();
        assert_eq!(unpack(driver_status_code(&err)), (409, None));
    }

    #[test]
    fn into_raw_populates_flat_fields_and_frees() {
        let status = CosmosStatus::new(StatusCode::RequestTimeout)
            .with_sub_status(SubStatusCode::CLIENT_OPERATION_TIMEOUT.value());
        let err = DriverCosmosError::builder()
            .with_status(status)
            .with_message("operation timeout")
            .build();
        let raw = CosmosError::into_raw(err);
        assert!(!raw.is_null());

        // SAFETY: `raw` is a freshly-produced owned pointer.
        let e = unsafe { &*raw };
        assert_eq!(e.http_status_code, 408);
        assert_eq!(
            e.sub_status,
            i32::from(SubStatusCode::CLIENT_OPERATION_TIMEOUT.value())
        );
        assert_eq!(e.is_from_wire, 0);
        assert_eq!(unpack(e.status), (408, Some(20008)));
        assert!(!e.message.is_null());
        let msg = unsafe { std::ffi::CStr::from_ptr(e.message) }
            .to_string_lossy()
            .into_owned();
        assert!(msg.contains("operation timeout"), "got: {msg}");
        // Synthetic error has no wire response — the header-derived fields are
        // NULL / -1.
        assert!(e.activity_id.is_null());
        assert!(e.session_token.is_null());
        assert!(e.etag.is_null());
        assert_eq!(e.retry_after_ms, -1);

        cosmos_error_free(raw);
    }

    #[test]
    fn free_null_is_a_no_op() {
        cosmos_error_free(std::ptr::null_mut());
    }

    /// CI guard: every [`CosmosSubStatus`] variant must carry the exact value of
    /// its canonical driver [`SubStatusCode`]. If the driver renumbers or the
    /// mirror drifts, this fails the build so the header can never silently
    /// disagree with the driver.
    #[test]
    fn sub_status_mirror_matches_driver() {
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusTransportGenerated503 as u16,
            SubStatusCode::TRANSPORT_GENERATED_503.value(),
            "TRANSPORT_GENERATED_503"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientCpuOverload as u16,
            SubStatusCode::CLIENT_CPU_OVERLOAD.value(),
            "CLIENT_CPU_OVERLOAD"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientThreadStarvation as u16,
            SubStatusCode::CLIENT_THREAD_STARVATION.value(),
            "CLIENT_THREAD_STARVATION"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusChannelClosed as u16,
            SubStatusCode::CHANNEL_CLOSED.value(),
            "CHANNEL_CLOSED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusMalformedContinuationToken as u16,
            SubStatusCode::MALFORMED_CONTINUATION_TOKEN.value(),
            "MALFORMED_CONTINUATION_TOKEN"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientOperationTimeout as u16,
            SubStatusCode::CLIENT_OPERATION_TIMEOUT.value(),
            "CLIENT_OPERATION_TIMEOUT"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusTransportConnectionFailed as u16,
            SubStatusCode::TRANSPORT_CONNECTION_FAILED.value(),
            "TRANSPORT_CONNECTION_FAILED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusTransportIoFailed as u16,
            SubStatusCode::TRANSPORT_IO_FAILED.value(),
            "TRANSPORT_IO_FAILED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusTransportDnsFailed as u16,
            SubStatusCode::TRANSPORT_DNS_FAILED.value(),
            "TRANSPORT_DNS_FAILED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusTransportBodyReadFailed as u16,
            SubStatusCode::TRANSPORT_BODY_READ_FAILED.value(),
            "TRANSPORT_BODY_READ_FAILED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusTransportHttp2Incompatible as u16,
            SubStatusCode::TRANSPORT_HTTP2_INCOMPATIBLE.value(),
            "TRANSPORT_HTTP2_INCOMPATIBLE"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusSerializationResponseBodyInvalid as u16,
            SubStatusCode::SERIALIZATION_RESPONSE_BODY_INVALID.value(),
            "SERIALIZATION_RESPONSE_BODY_INVALID"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientPartitionKeyEmpty as u16,
            SubStatusCode::CLIENT_PARTITION_KEY_EMPTY.value(),
            "CLIENT_PARTITION_KEY_EMPTY"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientPartitionKeyTooManyComponents as u16,
            SubStatusCode::CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS.value(),
            "CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientPrefixPartitionKeyRequiresMultihash as u16,
            SubStatusCode::CLIENT_PREFIX_PARTITION_KEY_REQUIRES_MULTIHASH.value(),
            "CLIENT_PREFIX_PARTITION_KEY_REQUIRES_MULTIHASH"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientConnectionStringEmpty as u16,
            SubStatusCode::CLIENT_CONNECTION_STRING_EMPTY.value(),
            "CLIENT_CONNECTION_STRING_EMPTY"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientConnectionStringMalformedPart as u16,
            SubStatusCode::CLIENT_CONNECTION_STRING_MALFORMED_PART.value(),
            "CLIENT_CONNECTION_STRING_MALFORMED_PART"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientConnectionStringMissingAccountKey as u16,
            SubStatusCode::CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_KEY.value(),
            "CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_KEY"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientInvalidAccountEndpointUrl as u16,
            SubStatusCode::CLIENT_INVALID_ACCOUNT_ENDPOINT_URL.value(),
            "CLIENT_INVALID_ACCOUNT_ENDPOINT_URL"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientInvalidUrl as u16,
            SubStatusCode::CLIENT_INVALID_URL.value(),
            "CLIENT_INVALID_URL"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientUnknownConsistencyLevel as u16,
            SubStatusCode::CLIENT_UNKNOWN_CONSISTENCY_LEVEL.value(),
            "CLIENT_UNKNOWN_CONSISTENCY_LEVEL"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientUnknownPriorityLevel as u16,
            SubStatusCode::CLIENT_UNKNOWN_PRIORITY_LEVEL.value(),
            "CLIENT_UNKNOWN_PRIORITY_LEVEL"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFeedRangeRequiresFanoutPipeline as u16,
            SubStatusCode::CLIENT_FEED_RANGE_REQUIRES_FANOUT_PIPELINE.value(),
            "CLIENT_FEED_RANGE_REQUIRES_FANOUT_PIPELINE"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientUnsupportedQueryFeature as u16,
            SubStatusCode::CLIENT_UNSUPPORTED_QUERY_FEATURE.value(),
            "CLIENT_UNSUPPORTED_QUERY_FEATURE"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientQueryPlanInvalidTopOffsetLimit as u16,
            SubStatusCode::CLIENT_QUERY_PLAN_INVALID_TOP_OFFSET_LIMIT.value(),
            "CLIENT_QUERY_PLAN_INVALID_TOP_OFFSET_LIMIT"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientContinuationTokenNonQueryOperation as u16,
            SubStatusCode::CLIENT_CONTINUATION_TOKEN_NON_QUERY_OPERATION.value(),
            "CLIENT_CONTINUATION_TOKEN_NON_QUERY_OPERATION"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientDuplicateFaultInjectionRuleId as u16,
            SubStatusCode::CLIENT_DUPLICATE_FAULT_INJECTION_RULE_ID.value(),
            "CLIENT_DUPLICATE_FAULT_INJECTION_RULE_ID"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientThroughputControlGroupNotRegistered as u16,
            SubStatusCode::CLIENT_THROUGHPUT_CONTROL_GROUP_NOT_REGISTERED.value(),
            "CLIENT_THROUGHPUT_CONTROL_GROUP_NOT_REGISTERED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientHttpClientConstructionFailed as u16,
            SubStatusCode::CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED.value(),
            "CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientReqwestFeatureRequired as u16,
            SubStatusCode::CLIENT_REQWEST_FEATURE_REQUIRED.value(),
            "CLIENT_REQWEST_FEATURE_REQUIRED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientRequestUrlMissingHost as u16,
            SubStatusCode::CLIENT_REQUEST_URL_MISSING_HOST.value(),
            "CLIENT_REQUEST_URL_MISSING_HOST"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientRequestUrlMissingKnownPort as u16,
            SubStatusCode::CLIENT_REQUEST_URL_MISSING_KNOWN_PORT.value(),
            "CLIENT_REQUEST_URL_MISSING_KNOWN_PORT"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientImdsHttpClientConstructionFailed as u16,
            SubStatusCode::CLIENT_IMDS_HTTP_CLIENT_CONSTRUCTION_FAILED.value(),
            "CLIENT_IMDS_HTTP_CLIENT_CONSTRUCTION_FAILED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientImdsReqwestFeatureRequired as u16,
            SubStatusCode::CLIENT_IMDS_REQWEST_FEATURE_REQUIRED.value(),
            "CLIENT_IMDS_REQWEST_FEATURE_REQUIRED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientContinuationTokenFetchInFlight as u16,
            SubStatusCode::CLIENT_CONTINUATION_TOKEN_FETCH_IN_FLIGHT.value(),
            "CLIENT_CONTINUATION_TOKEN_FETCH_IN_FLIGHT"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientTopologyProviderMissing as u16,
            SubStatusCode::CLIENT_TOPOLOGY_PROVIDER_MISSING.value(),
            "CLIENT_TOPOLOGY_PROVIDER_MISSING"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientDriverNotInitialized as u16,
            SubStatusCode::CLIENT_DRIVER_NOT_INITIALIZED.value(),
            "CLIENT_DRIVER_NOT_INITIALIZED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientContinuationTokenShapeMismatch as u16,
            SubStatusCode::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH.value(),
            "CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientContinuationTokenInvalidEpkRange as u16,
            SubStatusCode::CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE.value(),
            "CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientSplitRetriesExhausted as u16,
            SubStatusCode::CLIENT_SPLIT_RETRIES_EXHAUSTED.value(),
            "CLIENT_SPLIT_RETRIES_EXHAUSTED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientBuildResponseInvokedOnFailure as u16,
            SubStatusCode::CLIENT_BUILD_RESPONSE_INVOKED_ON_FAILURE.value(),
            "CLIENT_BUILD_RESPONSE_INVOKED_ON_FAILURE"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientRootNodeCannotRequestSplit as u16,
            SubStatusCode::CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT.value(),
            "CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientSingletonOperationReturnedEmptyPage as u16,
            SubStatusCode::CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE.value(),
            "CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientContinuationTokenSavedRangeUnhonored as u16,
            SubStatusCode::CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED.value(),
            "CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientNoThroughputOfferForResource as u16,
            SubStatusCode::CLIENT_NO_THROUGHPUT_OFFER_FOR_RESOURCE.value(),
            "CLIENT_NO_THROUGHPUT_OFFER_FOR_RESOURCE"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientQueryPlanProducedEmptyRanges as u16,
            SubStatusCode::CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES.value(),
            "CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusServiceReturnedOfferWithoutId as u16,
            SubStatusCode::SERVICE_RETURNED_OFFER_WITHOUT_ID.value(),
            "SERVICE_RETURNED_OFFER_WITHOUT_ID"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientThroughputPollerIncomplete as u16,
            SubStatusCode::CLIENT_THROUGHPUT_POLLER_INCOMPLETE.value(),
            "CLIENT_THROUGHPUT_POLLER_INCOMPLETE"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientTopologyResolutionFailed as u16,
            SubStatusCode::CLIENT_TOPOLOGY_RESOLUTION_FAILED.value(),
            "CLIENT_TOPOLOGY_RESOLUTION_FAILED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusServiceReturnedObjectWithoutRid as u16,
            SubStatusCode::SERVICE_RETURNED_OBJECT_WITHOUT_RID.value(),
            "SERVICE_RETURNED_OBJECT_WITHOUT_RID"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiNullArgument as u16,
            SubStatusCode::CLIENT_FFI_NULL_ARGUMENT.value(),
            "CLIENT_FFI_NULL_ARGUMENT"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiInvalidUtf8 as u16,
            SubStatusCode::CLIENT_FFI_INVALID_UTF8.value(),
            "CLIENT_FFI_INVALID_UTF8"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiInvalidHeader as u16,
            SubStatusCode::CLIENT_FFI_INVALID_HEADER.value(),
            "CLIENT_FFI_INVALID_HEADER"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiInvalidOptionValue as u16,
            SubStatusCode::CLIENT_FFI_INVALID_OPTION_VALUE.value(),
            "CLIENT_FFI_INVALID_OPTION_VALUE"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiOperationConsumed as u16,
            SubStatusCode::CLIENT_FFI_OPERATION_CONSUMED.value(),
            "CLIENT_FFI_OPERATION_CONSUMED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiPreconditionAlreadySet as u16,
            SubStatusCode::CLIENT_FFI_PRECONDITION_ALREADY_SET.value(),
            "CLIENT_FFI_PRECONDITION_ALREADY_SET"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiUnsupportedOperationForMutator as u16,
            SubStatusCode::CLIENT_FFI_UNSUPPORTED_OPERATION_FOR_MUTATOR.value(),
            "CLIENT_FFI_UNSUPPORTED_OPERATION_FOR_MUTATOR"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiFeedExhausted as u16,
            SubStatusCode::CLIENT_FFI_FEED_EXHAUSTED.value(),
            "CLIENT_FFI_FEED_EXHAUSTED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiQueueShutdown as u16,
            SubStatusCode::CLIENT_FFI_QUEUE_SHUTDOWN.value(),
            "CLIENT_FFI_QUEUE_SHUTDOWN"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiQueueFull as u16,
            SubStatusCode::CLIENT_FFI_QUEUE_FULL.value(),
            "CLIENT_FFI_QUEUE_FULL"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiOperationCancelled as u16,
            SubStatusCode::CLIENT_FFI_OPERATION_CANCELLED.value(),
            "CLIENT_FFI_OPERATION_CANCELLED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiRuntimeBuildFailed as u16,
            SubStatusCode::CLIENT_FFI_RUNTIME_BUILD_FAILED.value(),
            "CLIENT_FFI_RUNTIME_BUILD_FAILED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientFfiPanic as u16,
            SubStatusCode::CLIENT_FFI_PANIC.value(),
            "CLIENT_FFI_PANIC"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusClientGenerated401 as u16,
            SubStatusCode::CLIENT_GENERATED_401.value(),
            "CLIENT_GENERATED_401"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusAuthenticationTokenAcquisitionFailed as u16,
            SubStatusCode::AUTHENTICATION_TOKEN_ACQUISITION_FAILED.value(),
            "AUTHENTICATION_TOKEN_ACQUISITION_FAILED"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusTransitTimeout as u16,
            SubStatusCode::TRANSIT_TIMEOUT.value(),
            "TRANSIT_TIMEOUT"
        );
        assert_eq!(
            CosmosSubStatus::CosmosSubStatusServerBarrierThrottled as u16,
            SubStatusCode::SERVER_BARRIER_THROTTLED.value(),
            "SERVER_BARRIER_THROTTLED"
        );
    }
}
