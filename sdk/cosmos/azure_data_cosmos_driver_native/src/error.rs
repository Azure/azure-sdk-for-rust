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
//!   `(http_status << 16) | sub_status`: the high 16 bits carry the HTTP status,
//!   the low 16 bits carry the driver sub-status (`0` when there is none), and a
//!   fully-zero code means success. Pure-FFI / pre-flight failures (a NULL
//!   argument, invalid UTF-8, a shut-down completion queue, …) use a real HTTP
//!   status paired with a driver `CLIENT_FFI_*` (or `CLIENT_*`) sub-status, so
//!   they fit the same integer as service errors.
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
/// Layout: `(http_status << 16) | sub_status`. A fully-zero code is success.
/// The high 16 bits hold the HTTP status; the low 16 bits hold the driver
/// sub-status, or `0` when the operation had none. Decode on the host with
/// `http = code >> 16` and `sub = code & 0xFFFF` (a `sub` of `0` means there
/// was no sub-status).
///
/// This is a `#[repr(transparent)]` newtype over `i32`, so it stays
/// ABI-identical to a bare `int32_t` in the generated header
/// (`cosmos_status_code_t`) while keeping packed status codes from being
/// silently mixed with unrelated integers on the Rust side.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CosmosStatusCode(pub i32);

/// Success sentinel returned by fallible C functions.
pub const COSMOS_STATUS_SUCCESS: CosmosStatusCode = CosmosStatusCode(0);

impl CosmosStatusCode {
    /// Packs a driver [`CosmosStatus`] into the FFI [`CosmosStatusCode`].
    pub(crate) fn from_status(status: CosmosStatus) -> CosmosStatusCode {
        let http = u32::from(u16::from(status.status_code()));
        let sub = status.sub_status().map_or(0, |s| u32::from(s.value()));
        CosmosStatusCode(((http << 16) | sub) as i32)
    }

    /// Packs the status of a driver [`CosmosError`] into a [`CosmosStatusCode`].
    pub(crate) fn from_driver_error(err: &DriverCosmosError) -> CosmosStatusCode {
        Self::from_status(err.status())
    }
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
/// literals to emit `= N`). A compile-time guard in the Rust source that defines
/// this enum (`src/error.rs`, not part of the generated header) verifies every
/// discriminant against the driver constant it mirrors, so a value that drifts
/// from the driver — or a driver constant that is renamed or removed — fails the
/// build instead of silently diverging.
///
/// [`SubStatusCode`] is a set of associated `pub const`s, not an enumerable
/// type, so exposing a *new* synthetic `2xxxx` sub-status on the C surface is
/// still a manual step: add the variant to the Rust enum and pin it in that same
/// guard. Because the guard maps variants with an exhaustive match, a variant
/// left unpinned fails to compile — the enum and the driver cannot silently
/// diverge.
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
    /// `CLIENT_PARTITION_KEY_RANGE_CACHE_REQUIRED` (20159).
    CosmosSubStatusClientPartitionKeyRangeCacheRequired = 20159,
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
    /// `CLIENT_FFI_OPERATION_CONSUMED` (20354). Reserved: mirrors the driver
    /// constant but no current wrapper path produces it (the handle-mutator
    /// surface it described was superseded by the flat submit model).
    CosmosSubStatusClientFfiOperationConsumed = 20354,
    /// `CLIENT_FFI_PRECONDITION_ALREADY_SET` (20355). Reserved: mirrors the
    /// driver constant but no current wrapper path produces it.
    CosmosSubStatusClientFfiPreconditionAlreadySet = 20355,
    /// `CLIENT_FFI_UNSUPPORTED_OPERATION_FOR_MUTATOR` (20356). Returned when a
    /// request uses an operation that is unavailable in this wrapper build.
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

/// Compile-time guard that keeps [`CosmosSubStatus`] pinned to the driver's
/// canonical [`SubStatusCode`] constants.
///
/// cbindgen requires the enum above to carry literal discriminants (it neither
/// expands macros nor parses the driver crate), so the names and values are
/// spelled out by hand. `mirror_driver_sub_status!` re-expresses that same set
/// once and enforces two invariants at compile time:
///
/// - **No value drift.** Each variant's literal discriminant is asserted equal
///   to the value of the driver constant it mirrors, so changing a driver value
///   — or renaming or removing a mirrored constant — fails the build.
/// - **No unpinned variant.** The mapping is generated as an *exhaustive* match
///   over [`CosmosSubStatus`], so adding a variant without pinning it to a
///   driver constant here also fails to compile.
const _: () = {
    macro_rules! mirror_driver_sub_status {
        ($($variant:ident => $driver:ident),+ $(,)?) => {
            // Completeness: an exhaustive match forces every variant to be
            // pinned to a driver constant (a new, unlisted variant won't compile).
            const fn driver_of(v: CosmosSubStatus) -> SubStatusCode {
                match v {
                    $( CosmosSubStatus::$variant => SubStatusCode::$driver, )+
                }
            }
            // Correctness: each literal discriminant equals its driver value.
            $(
                assert!(
                    CosmosSubStatus::$variant as i32
                        == driver_of(CosmosSubStatus::$variant).value() as i32,
                    concat!(
                        "CosmosSubStatus::",
                        stringify!($variant),
                        " no longer matches driver SubStatusCode::",
                        stringify!($driver)
                    )
                );
            )+
        };
    }

    mirror_driver_sub_status! {
        CosmosSubStatusTransportGenerated503 => TRANSPORT_GENERATED_503,
        CosmosSubStatusClientCpuOverload => CLIENT_CPU_OVERLOAD,
        CosmosSubStatusClientThreadStarvation => CLIENT_THREAD_STARVATION,
        CosmosSubStatusChannelClosed => CHANNEL_CLOSED,
        CosmosSubStatusMalformedContinuationToken => MALFORMED_CONTINUATION_TOKEN,
        CosmosSubStatusClientOperationTimeout => CLIENT_OPERATION_TIMEOUT,
        CosmosSubStatusTransportConnectionFailed => TRANSPORT_CONNECTION_FAILED,
        CosmosSubStatusTransportIoFailed => TRANSPORT_IO_FAILED,
        CosmosSubStatusTransportDnsFailed => TRANSPORT_DNS_FAILED,
        CosmosSubStatusTransportBodyReadFailed => TRANSPORT_BODY_READ_FAILED,
        CosmosSubStatusTransportHttp2Incompatible => TRANSPORT_HTTP2_INCOMPATIBLE,
        CosmosSubStatusSerializationResponseBodyInvalid => SERIALIZATION_RESPONSE_BODY_INVALID,
        CosmosSubStatusClientPartitionKeyEmpty => CLIENT_PARTITION_KEY_EMPTY,
        CosmosSubStatusClientPartitionKeyTooManyComponents => CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS,
        CosmosSubStatusClientPrefixPartitionKeyRequiresMultihash => CLIENT_PREFIX_PARTITION_KEY_REQUIRES_MULTIHASH,
        CosmosSubStatusClientConnectionStringEmpty => CLIENT_CONNECTION_STRING_EMPTY,
        CosmosSubStatusClientConnectionStringMalformedPart => CLIENT_CONNECTION_STRING_MALFORMED_PART,
        CosmosSubStatusClientConnectionStringMissingAccountKey => CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_KEY,
        CosmosSubStatusClientInvalidAccountEndpointUrl => CLIENT_INVALID_ACCOUNT_ENDPOINT_URL,
        CosmosSubStatusClientInvalidUrl => CLIENT_INVALID_URL,
        CosmosSubStatusClientUnknownConsistencyLevel => CLIENT_UNKNOWN_CONSISTENCY_LEVEL,
        CosmosSubStatusClientUnknownPriorityLevel => CLIENT_UNKNOWN_PRIORITY_LEVEL,
        CosmosSubStatusClientFeedRangeRequiresFanoutPipeline => CLIENT_FEED_RANGE_REQUIRES_FANOUT_PIPELINE,
        CosmosSubStatusClientUnsupportedQueryFeature => CLIENT_UNSUPPORTED_QUERY_FEATURE,
        CosmosSubStatusClientQueryPlanInvalidTopOffsetLimit => CLIENT_QUERY_PLAN_INVALID_TOP_OFFSET_LIMIT,
        CosmosSubStatusClientContinuationTokenNonQueryOperation => CLIENT_CONTINUATION_TOKEN_NON_QUERY_OPERATION,
        CosmosSubStatusClientDuplicateFaultInjectionRuleId => CLIENT_DUPLICATE_FAULT_INJECTION_RULE_ID,
        CosmosSubStatusClientThroughputControlGroupNotRegistered => CLIENT_THROUGHPUT_CONTROL_GROUP_NOT_REGISTERED,
        CosmosSubStatusClientHttpClientConstructionFailed => CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED,
        CosmosSubStatusClientReqwestFeatureRequired => CLIENT_REQWEST_FEATURE_REQUIRED,
        CosmosSubStatusClientRequestUrlMissingHost => CLIENT_REQUEST_URL_MISSING_HOST,
        CosmosSubStatusClientRequestUrlMissingKnownPort => CLIENT_REQUEST_URL_MISSING_KNOWN_PORT,
        CosmosSubStatusClientImdsHttpClientConstructionFailed => CLIENT_IMDS_HTTP_CLIENT_CONSTRUCTION_FAILED,
        CosmosSubStatusClientImdsReqwestFeatureRequired => CLIENT_IMDS_REQWEST_FEATURE_REQUIRED,
        CosmosSubStatusClientPartitionKeyRangeCacheRequired => CLIENT_PARTITION_KEY_RANGE_CACHE_REQUIRED,
        CosmosSubStatusClientContinuationTokenFetchInFlight => CLIENT_CONTINUATION_TOKEN_FETCH_IN_FLIGHT,
        CosmosSubStatusClientTopologyProviderMissing => CLIENT_TOPOLOGY_PROVIDER_MISSING,
        CosmosSubStatusClientDriverNotInitialized => CLIENT_DRIVER_NOT_INITIALIZED,
        CosmosSubStatusClientContinuationTokenShapeMismatch => CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH,
        CosmosSubStatusClientContinuationTokenInvalidEpkRange => CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE,
        CosmosSubStatusClientSplitRetriesExhausted => CLIENT_SPLIT_RETRIES_EXHAUSTED,
        CosmosSubStatusClientBuildResponseInvokedOnFailure => CLIENT_BUILD_RESPONSE_INVOKED_ON_FAILURE,
        CosmosSubStatusClientRootNodeCannotRequestSplit => CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT,
        CosmosSubStatusClientSingletonOperationReturnedEmptyPage => CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE,
        CosmosSubStatusClientContinuationTokenSavedRangeUnhonored => CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED,
        CosmosSubStatusClientNoThroughputOfferForResource => CLIENT_NO_THROUGHPUT_OFFER_FOR_RESOURCE,
        CosmosSubStatusClientQueryPlanProducedEmptyRanges => CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES,
        CosmosSubStatusServiceReturnedOfferWithoutId => SERVICE_RETURNED_OFFER_WITHOUT_ID,
        CosmosSubStatusClientThroughputPollerIncomplete => CLIENT_THROUGHPUT_POLLER_INCOMPLETE,
        CosmosSubStatusClientTopologyResolutionFailed => CLIENT_TOPOLOGY_RESOLUTION_FAILED,
        CosmosSubStatusServiceReturnedObjectWithoutRid => SERVICE_RETURNED_OBJECT_WITHOUT_RID,
        CosmosSubStatusClientFfiNullArgument => CLIENT_FFI_NULL_ARGUMENT,
        CosmosSubStatusClientFfiInvalidUtf8 => CLIENT_FFI_INVALID_UTF8,
        CosmosSubStatusClientFfiInvalidHeader => CLIENT_FFI_INVALID_HEADER,
        CosmosSubStatusClientFfiInvalidOptionValue => CLIENT_FFI_INVALID_OPTION_VALUE,
        CosmosSubStatusClientFfiOperationConsumed => CLIENT_FFI_OPERATION_CONSUMED,
        CosmosSubStatusClientFfiPreconditionAlreadySet => CLIENT_FFI_PRECONDITION_ALREADY_SET,
        CosmosSubStatusClientFfiUnsupportedOperationForMutator => CLIENT_FFI_UNSUPPORTED_OPERATION_FOR_MUTATOR,
        CosmosSubStatusClientFfiFeedExhausted => CLIENT_FFI_FEED_EXHAUSTED,
        CosmosSubStatusClientFfiQueueShutdown => CLIENT_FFI_QUEUE_SHUTDOWN,
        CosmosSubStatusClientFfiQueueFull => CLIENT_FFI_QUEUE_FULL,
        CosmosSubStatusClientFfiOperationCancelled => CLIENT_FFI_OPERATION_CANCELLED,
        CosmosSubStatusClientFfiRuntimeBuildFailed => CLIENT_FFI_RUNTIME_BUILD_FAILED,
        CosmosSubStatusClientFfiPanic => CLIENT_FFI_PANIC,
        CosmosSubStatusClientGenerated401 => CLIENT_GENERATED_401,
        CosmosSubStatusAuthenticationTokenAcquisitionFailed => AUTHENTICATION_TOKEN_ACQUISITION_FAILED,
        CosmosSubStatusTransitTimeout => TRANSIT_TIMEOUT,
        CosmosSubStatusServerBarrierThrottled => SERVER_BARRIER_THROTTLED,
    }
};

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
/// [`CosmosErrorCode::as_status_code`]. Keeping the mapping here lets the whole ABI
/// share the driver's single status taxonomy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CosmosErrorCode {
    /// Operation completed successfully.
    CosmosErrorCodeSuccess,
    /// A required pointer argument was `NULL`.
    CosmosErrorCodeInvalidArgument,
    /// A `*const c_char` argument was not valid UTF-8.
    CosmosErrorCodeInvalidUtf8,
    /// A request header name or value contained non-ASCII or control
    /// characters.
    CosmosErrorCodeInvalidHeader,
    /// A builder setter was passed a value outside its documented range.
    CosmosErrorCodeInvalidOptionValue,
    /// A partition-key builder produced an empty / inconsistent key.
    CosmosErrorCodeInvalidPartitionKey,
    /// A partition key was supplied with more components than the driver's
    /// hierarchical-key cap allows.
    CosmosErrorCodeTooManyPartitionKeyComponents,
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
    pub(crate) fn to_status(self) -> Option<CosmosStatus> {
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
            Self::CosmosErrorCodeInvalidHeader => (
                StatusCode::BadRequest,
                SubStatusCode::CLIENT_FFI_INVALID_HEADER,
            ),
            Self::CosmosErrorCodeInvalidOptionValue => (
                StatusCode::BadRequest,
                SubStatusCode::CLIENT_FFI_INVALID_OPTION_VALUE,
            ),
            Self::CosmosErrorCodeInvalidPartitionKey => (
                StatusCode::BadRequest,
                SubStatusCode::CLIENT_PARTITION_KEY_EMPTY,
            ),
            Self::CosmosErrorCodeTooManyPartitionKeyComponents => (
                StatusCode::BadRequest,
                SubStatusCode::CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS,
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
    pub(crate) fn as_status_code(self) -> CosmosStatusCode {
        self.to_status()
            .map_or(COSMOS_STATUS_SUCCESS, CosmosStatusCode::from_status)
    }

    /// Infallible [`CosmosStatus`] for the internal panic-firewall condition
    /// (`500` / [`SubStatusCode::CLIENT_FFI_PANIC`]).
    ///
    /// The submit-path panic firewall must never itself panic, so it builds the
    /// synthesized error's status here instead of unwrapping the `Option`
    /// returned by [`to_status`](Self::to_status). It is single-sourced from the
    /// `CosmosErrorCodeInternalError` mapping, with an infallible constant
    /// fallback so a hypothetical future regression of that mapping to `None`
    /// downgrades to the same status rather than panicking.
    pub(crate) fn panic_status() -> CosmosStatus {
        Self::CosmosErrorCodeInternalError
            .to_status()
            .unwrap_or_else(|| {
                CosmosStatus::new(StatusCode::InternalServerError)
                    .with_sub_status(SubStatusCode::CLIENT_FFI_PANIC.value())
            })
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
    /// Owned NUL-terminated message (never NULL for a real error). Exposed as
    /// `*const c_char`: the buffer is library-owned until [`cosmos_error_free`]
    /// and callers must not mutate it.
    pub message: *const c_char,
    /// Owned activity id from the wire response headers, or NULL.
    pub activity_id: *const c_char,
    /// Owned session token from the wire response headers, or NULL.
    pub session_token: *const c_char,
    /// Owned ETag from the wire response headers, or NULL.
    pub etag: *const c_char,
    /// Owned backtrace, or NULL when none was captured.
    pub backtrace: *const c_char,
}

/// Builds a NUL-terminated copy of `s`, stripping any interior NUL bytes so the
/// conversion cannot fail.
fn to_cstring(s: impl Into<String>) -> Option<CString> {
    CString::new(s.into().replace('\0', "")).ok()
}

/// Consumes an optional `CString` into an owned raw pointer, or NULL. The
/// pointer is exposed as `*const c_char`; ownership is reclaimed by
/// [`free_cstring`] inside [`cosmos_error_free`].
fn cstring_into_raw(s: Option<CString>) -> *const c_char {
    s.map_or(std::ptr::null(), |c| CString::into_raw(c).cast_const())
}

/// Reclaims a raw pointer previously produced by [`cstring_into_raw`].
///
/// # Safety
///
/// `p` must be NULL or a pointer obtained from [`CString::into_raw`] (via
/// [`cstring_into_raw`]) that has not already been reclaimed.
unsafe fn free_cstring(p: *const c_char) {
    if !p.is_null() {
        drop(CString::from_raw(p.cast_mut()));
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
            status: CosmosStatusCode::from_status(status),
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

    /// Decodes a packed status back into `(http, sub)`. A `sub` of `0` means the
    /// operation had no sub-status.
    fn unpack(code: CosmosStatusCode) -> (u16, u16) {
        let bits = code.0 as u32;
        let http = (bits >> 16) as u16;
        let sub = (bits & 0xFFFF) as u16;
        (http, sub)
    }

    #[test]
    fn success_packs_to_zero() {
        assert_eq!(
            CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code(),
            COSMOS_STATUS_SUCCESS
        );
    }

    #[test]
    fn status_code_round_trips_with_sub_status() {
        let status = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
        let packed = CosmosStatusCode::from_status(status);
        assert_eq!(unpack(packed), (429, 3200));
    }

    #[test]
    fn status_code_round_trips_without_sub_status() {
        let packed = CosmosStatusCode::from_status(CosmosStatus::new(StatusCode::NotFound));
        assert_eq!(unpack(packed), (404, 0));
    }

    #[test]
    fn max_sub_status_does_not_collide_with_absent() {
        // `0xFFFF` is a real Cosmos sub-status (`SCRIPT_COMPILE_ERROR`). With a
        // plain `(http << 16) | sub` pack it occupies the low 16 bits like any
        // other value, staying distinct from the `sub == 0` "absent" encoding.
        let present = CosmosStatusCode::from_status(
            CosmosStatus::new(StatusCode::BadRequest).with_sub_status(0xFFFF),
        );
        let absent = CosmosStatusCode::from_status(CosmosStatus::new(StatusCode::BadRequest));
        assert_eq!(unpack(present), (400, 0xFFFF));
        assert_eq!(unpack(absent), (400, 0));
        assert_ne!(present, absent);
    }

    #[test]
    fn every_sub_status_value_round_trips() {
        // No `u16` sub-status value may be sacrificed as a sentinel: each must
        // pack and unpack to itself for a fixed HTTP status.
        for sub in [0u16, 1, 1002, 3200, 20350, 0xFFFE, 0xFFFF] {
            let packed = CosmosStatusCode::from_status(
                CosmosStatus::new(StatusCode::Conflict).with_sub_status(sub),
            );
            assert_eq!(unpack(packed), (409, sub), "sub {sub:#06x} must round-trip");
        }
    }

    #[test]
    fn packed_status_matches_documented_host_decode() {
        // Regression guard for the packed-status ABI contract (PR #4820,
        // comment r3692140719). A host using the *documented* decode — the exact
        // `COSMOS_STATUS_HTTP` / `COSMOS_STATUS_SUB` header macros, `code >> 16`
        // and `code & 0xFFFF`, with no masking or presence flag — must recover
        // the driver's HTTP status and sub-status. Checked directly (not through
        // the `unpack` helper) so a future high-bit encoding change that also
        // "fixed" the helper still trips here.
        let bits = CosmosStatusCode::from_status(
            CosmosStatus::new(StatusCode::BadRequest).with_sub_status(20350),
        )
        .0 as u32;
        assert_eq!(bits >> 16, 400, "http must be the plain high 16 bits");
        assert_eq!(bits & 0xFFFF, 20350, "sub must be the plain low 16 bits");
    }

    #[test]
    fn every_error_code_maps_to_expected_packed_status() {
        fn expected_wire(code: CosmosErrorCode) -> (u16, u16) {
            use CosmosErrorCode as Ec;
            match code {
                Ec::CosmosErrorCodeSuccess => (0, 0), // packed COSMOS_STATUS_SUCCESS
                Ec::CosmosErrorCodeInvalidArgument => (400, 20350), // CLIENT_FFI_NULL_ARGUMENT
                Ec::CosmosErrorCodeInvalidUtf8 => (400, 20351), // CLIENT_FFI_INVALID_UTF8
                Ec::CosmosErrorCodeInvalidHeader => (400, 20352), // CLIENT_FFI_INVALID_HEADER
                Ec::CosmosErrorCodeInvalidOptionValue => (400, 20353), // CLIENT_FFI_INVALID_OPTION_VALUE
                Ec::CosmosErrorCodeInvalidPartitionKey => (400, 20100), // CLIENT_PARTITION_KEY_EMPTY
                Ec::CosmosErrorCodeTooManyPartitionKeyComponents => (400, 20101), // CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS
                Ec::CosmosErrorCodeInvalidAccountReference => (400, 20108), // CLIENT_INVALID_ACCOUNT_ENDPOINT_URL
                Ec::CosmosErrorCodeOperationCancelled => (408, 20360), // CLIENT_FFI_OPERATION_CANCELLED
                Ec::CosmosErrorCodeQueueShutdown => (503, 20358),      // CLIENT_FFI_QUEUE_SHUTDOWN
                Ec::CosmosErrorCodeQueueFull => (503, 20359),          // CLIENT_FFI_QUEUE_FULL
                Ec::CosmosErrorCodeRuntimeBuildFailed => (500, 20361), // CLIENT_FFI_RUNTIME_BUILD_FAILED
                Ec::CosmosErrorCodeInternalError => (500, 20362),      // CLIENT_FFI_PANIC
            }
        }

        use CosmosErrorCode as Ec;
        let all = [
            Ec::CosmosErrorCodeSuccess,
            Ec::CosmosErrorCodeInvalidArgument,
            Ec::CosmosErrorCodeInvalidUtf8,
            Ec::CosmosErrorCodeInvalidHeader,
            Ec::CosmosErrorCodeInvalidOptionValue,
            Ec::CosmosErrorCodeInvalidPartitionKey,
            Ec::CosmosErrorCodeTooManyPartitionKeyComponents,
            Ec::CosmosErrorCodeInvalidAccountReference,
            Ec::CosmosErrorCodeOperationCancelled,
            Ec::CosmosErrorCodeQueueShutdown,
            Ec::CosmosErrorCodeQueueFull,
            Ec::CosmosErrorCodeRuntimeBuildFailed,
            Ec::CosmosErrorCodeInternalError,
        ];
        for code in all {
            assert_eq!(
                unpack(code.as_status_code()),
                expected_wire(code),
                "{code:?} mapped to the wrong packed (http, sub)"
            );
        }
    }

    #[test]
    fn panic_status_matches_internal_error_mapping() {
        // The infallible `panic_status()` used by the submit-path panic firewall
        // must stay identical to the `CosmosErrorCodeInternalError` mapping so
        // the two never drift.
        assert_eq!(
            CosmosStatusCode::from_status(CosmosErrorCode::panic_status()),
            CosmosErrorCode::CosmosErrorCodeInternalError.as_status_code()
        );
        assert_eq!(
            unpack(CosmosStatusCode::from_status(
                CosmosErrorCode::panic_status()
            )),
            (500, SubStatusCode::CLIENT_FFI_PANIC.value())
        );
    }

    #[test]
    fn from_driver_error_packs_wire_status() {
        let err = DriverCosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::Conflict))
            .with_message("conflict")
            .build();
        assert_eq!(unpack(CosmosStatusCode::from_driver_error(&err)), (409, 0));
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
        assert_eq!(unpack(e.status), (408, 20008));
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
}
