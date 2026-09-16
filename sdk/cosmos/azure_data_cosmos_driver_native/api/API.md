# azurecosmosdriver

- **Description**: C ABI wrapper for the Azure Cosmos DB driver crate (azure_data_cosmos_driver). Exposes a schema-agnostic completion-queue-style FFI for cross-language SDK reuse (.NET, Java, Go, Python, native C/C++).
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `test-abi`

```rust
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#[macro_export]
macro_rules! c_str {
    ($s:expr) => { ... };
}
#[no_mangle]
pub extern "C" fn cosmos_version() -> *const std::ffi::c_char;
#[no_mangle]
pub static COSMOS_BUILD_IDENTIFIER: &std::ffi::CStr = _;
pub mod account_ref {
    #[no_mangle]
    pub extern "C" fn cosmos_account_ref_free(account: *mut AccountRefHandle);
    #[no_mangle]
    pub extern "C" fn cosmos_account_ref_with_credential(endpoint: crate::string::CosmosStringView, provider: crate::credential::CosmosTokenProvider, user_data: isize, out_account: *mut *mut AccountRefHandle, out_error: *mut *mut crate::error::CosmosError) -> crate::error::CosmosStatusCode;
    #[no_mangle]
    pub extern "C" fn cosmos_account_ref_with_master_key(endpoint: crate::string::CosmosStringView, key: crate::string::CosmosStringView, out_account: *mut *mut AccountRefHandle, out_error: *mut *mut crate::error::CosmosError) -> crate::error::CosmosStatusCode;
    pub struct AccountRefHandle {
    }
}
pub mod bytes {
    #[no_mangle]
    pub extern "C" fn cosmos_bytes_free(bytes: CosmosBytes);
    #[repr(C)]
    pub struct CosmosBytes {
        pub ptr: *const u8,
        pub len: usize,
    }
}
pub mod completion {
    #[no_mangle]
    pub extern "C" fn cosmos_completion_patch_tracking_id(completion: *const CosmosCompletion) -> *const std::ffi::c_char;
    #[no_mangle]
    pub extern "C" fn cosmos_completion_queue_create(runtime: *const crate::runtime::RuntimeContext, options: *const CosmosCompletionQueueOptions) -> *mut CompletionQueue;
    #[no_mangle]
    pub extern "C" fn cosmos_completion_queue_free(queue: *mut CompletionQueue);
    #[no_mangle]
    pub extern "C" fn cosmos_completion_queue_free_completions(completions: *mut CosmosCompletion, count: usize);
    #[no_mangle]
    pub extern "C" fn cosmos_completion_queue_shutdown(queue: *mut CompletionQueue);
    #[no_mangle]
    pub extern "C" fn cosmos_completion_queue_state(queue: *const CompletionQueue) -> CosmosCompletionQueueState;
    #[no_mangle]
    pub extern "C" fn cosmos_completion_queue_wait(queue: *mut CompletionQueue, out: *mut CosmosCompletion, max: usize, timeout_ms: u32) -> usize;
    #[no_mangle]
    pub extern "C" fn cosmos_completion_queue_wait_writable(queue: *mut CompletionQueue, timeout_ms: u32) -> bool;
    #[no_mangle]
    pub extern "C" fn cosmos_completion_take_container(c: *mut CosmosCompletion) -> *mut crate::container_ref::ContainerRefHandle;
    #[no_mangle]
    pub extern "C" fn cosmos_completion_take_driver(c: *mut CosmosCompletion) -> *mut crate::driver::DriverHandle;
    #[no_mangle]
    pub extern "C" fn cosmos_operation_handle_cancel(op: *mut OperationHandle);
    #[no_mangle]
    pub extern "C" fn cosmos_operation_handle_free(op: *mut OperationHandle);
    #[no_mangle]
    pub extern "C" fn cosmos_operation_handle_state(op: *const OperationHandle) -> CosmosOperationHandleState;
    pub struct CompletionQueue {
    }
    #[repr(C)]
    pub struct CosmosCompletion {
        pub outcome: CosmosCompletionOutcome,
        pub status: crate::error::CosmosStatusCode,
        pub user_data: isize,
        pub was_cancel_requested: u8,
        pub http_status_code: u16,
        pub is_from_wire: u8,
        pub message: *const std::ffi::c_char,
        pub next_continuation: *const std::ffi::c_char,
        pub backtrace: *const std::ffi::c_char,
        pub headers: *const crate::response_header::CosmosResponseHeader,
        pub headers_len: usize,
        pub body: *const u8,
        pub body_len: usize,
        pub diagnostics: *mut std::ffi::c_void,
        pub driver: *mut crate::driver::DriverHandle,
        pub container: *mut crate::container_ref::ContainerRefHandle,
        pub backing: *mut CosmosCompletionBacking,
    }
    pub struct CosmosCompletionBacking {
    }
    #[repr(C)]
    pub struct CosmosCompletionQueueOptions {
        pub capacity_hint: u32,
        pub max_capacity: u32,
        pub include_error_details: bool,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct CqOptions {
        pub capacity_hint: u32,
        pub max_capacity: u32,
        pub include_error_details: bool,
    }
    impl Default for CqOptions {
        fn default() -> Self;
    }
    pub struct OperationHandle {
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(i32)]
    pub enum CosmosCompletionOutcome {
        CosmosCompletionOutcomeOk = 0,
        CosmosCompletionOutcomeError = 1,
        CosmosCompletionOutcomeCancelled = 2,
        CosmosCompletionOutcomeUnknown = 255,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(i32)]
    pub enum CosmosCompletionQueueState {
        CosmosCompletionQueueStateRunning = 0,
        CosmosCompletionQueueStateShutdown = 1,
        CosmosCompletionQueueStateDrained = 2,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(i32)]
    pub enum CosmosOperationHandleState {
        CosmosOperationHandleStateInFlight = 0,
        CosmosOperationHandleStateCompleted = 1,
        CosmosOperationHandleStateFailed = 2,
        CosmosOperationHandleStateCancelled = 3,
    }
}
pub mod container_ref {
    #[no_mangle]
    pub extern "C" fn cosmos_container_ref_free(container: *mut ContainerRefHandle);
    #[no_mangle]
    pub extern "C" fn cosmos_driver_resolve_container_blocking(runtime: *const crate::runtime::RuntimeContext, driver: *const crate::driver::DriverHandle, database_id: crate::string::CosmosStringView, container_id: crate::string::CosmosStringView, out_container: *mut *mut ContainerRefHandle, out_error: *mut *mut crate::error::CosmosError) -> crate::error::CosmosStatusCode;
    pub struct ContainerRefHandle {
    }
}
pub mod credential {
    #[no_mangle]
    pub unsafe extern "C" fn cosmos_token_request_complete(request_id: u64, status: i32, token: *const u8, token_len: usize, expires_on_unix_seconds: i64, error_message: *const u8, error_message_len: usize) -> crate::error::CosmosStatusCode;
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct CosmosTokenProvider {
        pub get_token: Option<unsafe extern "C" fn(isize, *const CosmosTokenRequest) -> i32>,
        pub user_data_free: Option<unsafe extern "C" fn(isize)>,
    }
    #[repr(C)]
    pub struct CosmosTokenRequest {
        pub request_id: u64,
        pub scope: *const u8,
        pub scope_len: usize,
    }
    pub type CosmosTokenProviderCallback = unsafe extern "C" fn(isize, *const CosmosTokenRequest) -> i32;
    pub type CosmosTokenProviderFree = unsafe extern "C" fn(isize);
}
pub mod database_ref {
    #[no_mangle]
    pub extern "C" fn cosmos_database_ref_create(account: *const crate::account_ref::AccountRefHandle, database_id: crate::string::CosmosStringView, out_database: *mut *mut DatabaseRefHandle) -> crate::error::CosmosStatusCode;
    #[no_mangle]
    pub extern "C" fn cosmos_database_ref_free(database: *mut DatabaseRefHandle);
    pub struct DatabaseRefHandle {
    }
}
pub mod driver {
    #[no_mangle]
    pub extern "C" fn cosmos_driver_free(driver: *mut DriverHandle);
    #[no_mangle]
    pub extern "C" fn cosmos_driver_get_or_create_blocking(runtime: *const crate::runtime::RuntimeContext, account: *const crate::account_ref::AccountRefHandle, options: *const crate::driver_options::DriverOptionsHandle, out_driver: *mut *mut DriverHandle, out_error: *mut *mut crate::error::CosmosError) -> crate::error::CosmosStatusCode;
    pub struct DriverHandle {
    }
}
pub mod driver_options {
    #[no_mangle]
    pub extern "C" fn cosmos_driver_options_build(account: *const crate::account_ref::AccountRefHandle, config: *const CosmosDriverOptionsConfig, out_options: *mut *mut DriverOptionsHandle) -> crate::error::CosmosStatusCode;
    #[no_mangle]
    pub extern "C" fn cosmos_driver_options_config_default() -> CosmosDriverOptionsConfig;
    #[no_mangle]
    pub extern "C" fn cosmos_driver_options_free(options: *mut DriverOptionsHandle);
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct CosmosDriverOptionsConfig {
        pub preferred_regions: *const crate::string::CosmosStringView,
        pub preferred_regions_len: usize,
        pub operation_options: *const crate::op_request::CosmosOperationOptions,
    }
    pub struct DriverOptionsHandle {
    }
}
pub mod error {
    #[no_mangle]
    pub extern "C" fn cosmos_error_free(e: *mut CosmosError);
    #[no_mangle]
    pub extern "C" fn cosmos_set_backtrace_options(max_captures_per_second: u32, max_resolutions_per_second: u32);
    #[repr(C)]
    pub struct CosmosError {
        pub status: CosmosStatusCode,
        pub http_status_code: u16,
        pub sub_status: i32,
        pub is_from_wire: u8,
        pub retry_after_ms: i64,
        pub message: *const std::ffi::c_char,
        pub activity_id: *const std::ffi::c_char,
        pub session_token: *const std::ffi::c_char,
        pub etag: *const std::ffi::c_char,
        pub backtrace: *const std::ffi::c_char,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CosmosStatusCode(pub i32);
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(i32)]
    pub enum CosmosSubStatus {
        CosmosSubStatusTransportGenerated503 = 20003,
        CosmosSubStatusClientCpuOverload = 20004,
        CosmosSubStatusClientThreadStarvation = 20005,
        CosmosSubStatusChannelClosed = 20006,
        CosmosSubStatusMalformedContinuationToken = 20007,
        CosmosSubStatusClientOperationTimeout = 20008,
        CosmosSubStatusTransportConnectionFailed = 20010,
        CosmosSubStatusTransportIoFailed = 20011,
        CosmosSubStatusTransportDnsFailed = 20012,
        CosmosSubStatusTransportBodyReadFailed = 20014,
        CosmosSubStatusTransportHttp2Incompatible = 20015,
        CosmosSubStatusSerializationResponseBodyInvalid = 20020,
        CosmosSubStatusClientPartitionKeyEmpty = 20100,
        CosmosSubStatusClientPartitionKeyTooManyComponents = 20101,
        CosmosSubStatusClientPrefixPartitionKeyRequiresMultihash = 20102,
        CosmosSubStatusClientConnectionStringEmpty = 20104,
        CosmosSubStatusClientConnectionStringMalformedPart = 20105,
        CosmosSubStatusClientConnectionStringMissingAccountKey = 20107,
        CosmosSubStatusClientInvalidAccountEndpointUrl = 20108,
        CosmosSubStatusClientInvalidUrl = 20109,
        CosmosSubStatusClientUnknownConsistencyLevel = 20110,
        CosmosSubStatusClientUnknownPriorityLevel = 20111,
        CosmosSubStatusClientFeedRangeRequiresFanoutPipeline = 20112,
        CosmosSubStatusClientUnsupportedQueryFeature = 20113,
        CosmosSubStatusClientQueryPlanInvalidTopOffsetLimit = 20114,
        CosmosSubStatusClientContinuationTokenNonQueryOperation = 20117,
        CosmosSubStatusClientDuplicateFaultInjectionRuleId = 20150,
        CosmosSubStatusClientThroughputControlGroupNotRegistered = 20152,
        CosmosSubStatusClientHttpClientConstructionFailed = 20153,
        CosmosSubStatusClientReqwestFeatureRequired = 20154,
        CosmosSubStatusClientRequestUrlMissingHost = 20155,
        CosmosSubStatusClientRequestUrlMissingKnownPort = 20156,
        CosmosSubStatusClientImdsHttpClientConstructionFailed = 20157,
        CosmosSubStatusClientImdsReqwestFeatureRequired = 20158,
        CosmosSubStatusClientPartitionKeyRangeCacheRequired = 20159,
        CosmosSubStatusClientContinuationTokenFetchInFlight = 20200,
        CosmosSubStatusClientTopologyProviderMissing = 20201,
        CosmosSubStatusClientDriverNotInitialized = 20202,
        CosmosSubStatusClientContinuationTokenShapeMismatch = 20203,
        CosmosSubStatusClientContinuationTokenInvalidEpkRange = 20205,
        CosmosSubStatusClientSplitRetriesExhausted = 20206,
        CosmosSubStatusClientBuildResponseInvokedOnFailure = 20207,
        CosmosSubStatusClientRootNodeCannotRequestSplit = 20208,
        CosmosSubStatusClientSingletonOperationReturnedEmptyPage = 20210,
        CosmosSubStatusClientContinuationTokenSavedRangeUnhonored = 20213,
        CosmosSubStatusClientNoThroughputOfferForResource = 20301,
        CosmosSubStatusClientQueryPlanProducedEmptyRanges = 20302,
        CosmosSubStatusServiceReturnedOfferWithoutId = 20303,
        CosmosSubStatusClientThroughputPollerIncomplete = 20304,
        CosmosSubStatusClientTopologyResolutionFailed = 20305,
        CosmosSubStatusServiceReturnedObjectWithoutRid = 20306,
        CosmosSubStatusClientFfiNullArgument = 20350,
        CosmosSubStatusClientFfiInvalidUtf8 = 20351,
        CosmosSubStatusClientFfiInvalidHeader = 20352,
        CosmosSubStatusClientFfiInvalidOptionValue = 20353,
        CosmosSubStatusClientFfiOperationConsumed = 20354,
        CosmosSubStatusClientFfiPreconditionAlreadySet = 20355,
        CosmosSubStatusClientFfiUnsupportedOperationForMutator = 20356,
        CosmosSubStatusClientFfiFeedExhausted = 20357,
        CosmosSubStatusClientFfiQueueShutdown = 20358,
        CosmosSubStatusClientFfiQueueFull = 20359,
        CosmosSubStatusClientFfiOperationCancelled = 20360,
        CosmosSubStatusClientFfiRuntimeBuildFailed = 20361,
        CosmosSubStatusClientFfiPanic = 20362,
        CosmosSubStatusClientGenerated401 = 20401,
        CosmosSubStatusAuthenticationTokenAcquisitionFailed = 20402,
        CosmosSubStatusTransitTimeout = 20911,
        CosmosSubStatusServerBarrierThrottled = 21011,
    }
    pub const COSMOS_STATUS_SUCCESS: CosmosStatusCode = _;
}
pub mod feed_range {
    #[no_mangle]
    pub extern "C" fn cosmos_feed_range_for_partition_key(container: *const crate::container_ref::ContainerRefHandle, pk: *const crate::partition_key::PartitionKeyHandle, out_fr: *mut *mut FeedRangeHandle) -> crate::error::CosmosStatusCode;
    #[no_mangle]
    pub extern "C" fn cosmos_feed_range_free(fr: *mut FeedRangeHandle);
    #[no_mangle]
    pub extern "C" fn cosmos_feed_range_full(out_fr: *mut *mut FeedRangeHandle) -> crate::error::CosmosStatusCode;
    pub struct FeedRangeHandle {
    }
}
pub mod op_request {
    #[no_mangle]
    pub extern "C" fn cosmos_operation_options_default() -> CosmosOperationOptions;
    #[repr(C)]
    pub struct CosmosHeaderKv {
        pub name: crate::string::CosmosStringView,
        pub value: crate::string::CosmosStringView,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct CosmosOperationOptions {
        pub read_consistency_strategy: i32,
        pub content_response_on_write: i32,
        pub patch_strategy: i32,
        pub session_capturing_disabled: i8,
        pub max_failover_retry_count: i32,
        pub max_session_retry_count: i32,
        pub end_to_end_timeout_ms: i64,
        pub endpoint_unavailability_ttl_ms: i64,
        pub throughput_control_group: crate::string::CosmosStringView,
        pub excluded_regions: *const crate::string::CosmosStringView,
        pub excluded_regions_len: usize,
        pub custom_headers: *const CosmosHeaderKv,
        pub custom_headers_len: usize,
        pub binary_encoding_enabled: i8,
        pub binary_encoding_request_text_response: i8,
        pub query_plan_mode: i32,
    }
    #[repr(C)]
    pub struct CosmosOperationRequest {
        pub kind: i32,
        pub account: *const crate::account_ref::AccountRefHandle,
        pub database: *const crate::database_ref::DatabaseRefHandle,
        pub container: *const crate::container_ref::ContainerRefHandle,
        pub item_id: crate::string::CosmosStringView,
        pub resource_link: crate::string::CosmosStringView,
        pub partition_key: *const crate::partition_key::PartitionKeyHandle,
        pub partition_key_components: *const crate::partition_key::CosmosPartitionKeyComponent,
        pub partition_key_len: usize,
        pub feed_range: *const crate::feed_range::FeedRangeHandle,
        pub body: *const u8,
        pub body_len: usize,
        pub session_token: crate::string::CosmosStringView,
        pub activity_id: crate::string::CosmosStringView,
        pub continuation_token: crate::string::CosmosStringView,
        pub max_item_count: i32,
        pub max_fan_out: u32,
        pub patch_max_attempts: u8,
        pub populate_index_metrics: i8,
        pub populate_query_metrics: i8,
        pub precondition_kind: i32,
        pub precondition_etag: crate::string::CosmosStringView,
        pub options: *const CosmosOperationOptions,
        pub patch_tracking_id: crate::string::CosmosStringView,
        pub patch_tracking_capacity: u16,
        pub patch_tracking_retention_seconds: u32,
    }
    pub struct OptOf;
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(i32)]
    pub enum CosmosContentResponseOnWriteOpt {
        CosmosContentResponseOnWriteOptUnset = 0,
        CosmosContentResponseOnWriteOptDisabled = 1,
        CosmosContentResponseOnWriteOptEnabled = 2,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(i32)]
    pub enum CosmosOperationKind {
        CosmosOperationKindInvalid = 0,
        CosmosOperationKindCreateDatabase = 1,
        CosmosOperationKindReadAllDatabases = 2,
        CosmosOperationKindQueryDatabases = 3,
        CosmosOperationKindQueryOffers = 4,
        CosmosOperationKindReadOffer = 5,
        CosmosOperationKindReplaceOffer = 6,
        CosmosOperationKindReadDatabase = 7,
        CosmosOperationKindDeleteDatabase = 8,
        CosmosOperationKindCreateContainer = 9,
        CosmosOperationKindReadAllContainers = 10,
        CosmosOperationKindQueryContainers = 11,
        CosmosOperationKindReadContainer = 12,
        CosmosOperationKindReplaceContainer = 13,
        CosmosOperationKindDeleteContainer = 14,
        CosmosOperationKindReadAllItems = 15,
        CosmosOperationKindReadAllItemsCrossPartition = 16,
        CosmosOperationKindQueryItems = 17,
        CosmosOperationKindBatch = 18,
        CosmosOperationKindCreateItem = 19,
        CosmosOperationKindReadItem = 20,
        CosmosOperationKindUpsertItem = 21,
        CosmosOperationKindReplaceItem = 22,
        CosmosOperationKindDeleteItem = 23,
        CosmosOperationKindPatchItem = 24,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(i32)]
    pub enum CosmosPatchStrategy {
        CosmosPatchStrategyUnset = 0,
        CosmosPatchStrategyAuto = 1,
        CosmosPatchStrategyClientSide = 2,
        CosmosPatchStrategyServerSide = 3,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(i32)]
    pub enum CosmosPreconditionKind {
        CosmosPreconditionKindNone = 0,
        CosmosPreconditionKindIfMatch = 1,
        CosmosPreconditionKindIfNoneMatch = 2,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(i32)]
    pub enum CosmosQueryPlanMode {
        CosmosQueryPlanModeUnset = 0,
        CosmosQueryPlanModeLocalPreferred = 1,
        CosmosQueryPlanModeGatewayOnly = 2,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(i32)]
    pub enum CosmosReadConsistencyStrategy {
        CosmosReadConsistencyStrategyUnset = 0,
        CosmosReadConsistencyStrategyDefault = 1,
        CosmosReadConsistencyStrategyEventual = 2,
        CosmosReadConsistencyStrategySession = 3,
        CosmosReadConsistencyStrategyGlobalStrong = 4,
        CosmosReadConsistencyStrategyLatestCommitted = 5,
    }
}
pub mod partition_key {
    pub use azurecosmosdriver::string::CosmosStringView;
    #[no_mangle]
    pub extern "C" fn cosmos_partition_key_component_count(pk: *const PartitionKeyHandle) -> usize;
    #[no_mangle]
    pub extern "C" fn cosmos_partition_key_create(components: *const CosmosPartitionKeyComponent, len: usize, out_pk: *mut *mut PartitionKeyHandle) -> crate::error::CosmosStatusCode;
    #[no_mangle]
    pub extern "C" fn cosmos_partition_key_empty() -> *mut PartitionKeyHandle;
    #[no_mangle]
    pub extern "C" fn cosmos_partition_key_free(pk: *mut PartitionKeyHandle);
    #[no_mangle]
    pub extern "C" fn cosmos_partition_key_is_empty(pk: *const PartitionKeyHandle) -> bool;
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct CosmosPartitionKeyComponent {
        pub kind: u8,
        pub value: CosmosPartitionKeyComponentValue,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CosmosPartitionKeyComponentKind(pub u8);
    impl CosmosPartitionKeyComponentKind {
        const BOOL: Self = _;
        const NULL: Self = _;
        const NUMBER: Self = _;
        const STRING: Self = _;
        const UNDEFINED: Self = _;
    }
    pub struct PartitionKeyHandle {
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub union CosmosPartitionKeyComponentValue {
        pub string_value: CosmosStringView,
        pub number_value: f64,
        pub bool_value: u8,
    }
}
pub mod response_header {
    #[no_mangle]
    pub extern "C" fn cosmos_header_name(id: CosmosHeaderId) -> *const std::ffi::c_char;
    #[repr(C)]
    pub struct CosmosResponseHeader {
        pub id: CosmosHeaderId,
        pub value: CosmosValue,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct CosmosValue {
        pub kind: u8,
        pub payload: CosmosValuePayload,
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CosmosValueKind(pub u8);
    impl CosmosValueKind {
        const BOOL: Self = _;
        const F64: Self = _;
        const I64: Self = _;
        const STRING: Self = _;
        const U64: Self = _;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(i32)]
    pub enum CosmosHeaderId {
        CosmosHeaderIdUnknown = 0,
        CosmosHeaderIdActivityId = 1,
        CosmosHeaderIdRequestCharge = 2,
        CosmosHeaderIdSessionToken = 3,
        CosmosHeaderIdEtag = 4,
        CosmosHeaderIdContinuation = 5,
        CosmosHeaderIdItemCount = 6,
        CosmosHeaderIdSubStatus = 7,
        CosmosHeaderIdIndexMetrics = 8,
        CosmosHeaderIdQueryMetrics = 9,
        CosmosHeaderIdServerDurationMs = 10,
        CosmosHeaderIdLsn = 11,
        CosmosHeaderIdItemLsn = 12,
        CosmosHeaderIdOfferReplacePending = 13,
        CosmosHeaderIdRetryAfterMs = 14,
        CosmosHeaderIdCorrelatedActivityId = 15,
        CosmosHeaderIdGlobalCommittedLsn = 16,
        CosmosHeaderIdNumberOfReadRegions = 17,
        CosmosHeaderIdGatewayVersion = 18,
        CosmosHeaderIdServiceVersion = 19,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub union CosmosValuePayload {
        pub string_value: *const std::ffi::c_char,
        pub i64_value: i64,
        pub f64_value: f64,
        pub bool_value: bool,
        pub u64_value: u64,
    }
}
pub mod runtime {
    #[no_mangle]
    pub extern "C" fn cosmos_runtime_free(runtime: *mut RuntimeContext);
    pub struct RuntimeContext {
    }
}
pub mod runtime_builder {
    #[no_mangle]
    pub extern "C" fn cosmos_runtime_build(options: *const CosmosRuntimeOptions, out_runtime: *mut *mut crate::runtime::RuntimeContext, out_error: *mut *mut crate::error::CosmosError) -> crate::error::CosmosStatusCode;
    #[no_mangle]
    pub extern "C" fn cosmos_runtime_options_default() -> CosmosRuntimeOptions;
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct CosmosRuntimeOptions {
        pub workload_id: u8,
        pub correlation_id: crate::string::CosmosStringView,
        pub user_agent_suffix: crate::string::CosmosStringView,
        pub wrapping_sdk_identifier: crate::string::CosmosStringView,
        pub cpu_refresh_interval_ms: u64,
    }
}
#[attr = MacroUse {arguments:UseAll}]
pub mod string {
    #[no_mangle]
    pub extern "C" fn cosmos_string_free(s: *const std::os::raw::c_char);
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct CosmosStringView {
        pub data: *const u8,
        pub len: usize,
    }
    impl Default for CosmosStringView {
        fn default() -> Self;
    }
}
pub mod submit {
    #[no_mangle]
    pub extern "C" fn cosmos_driver_get_or_create_submit(runtime: *const crate::runtime::RuntimeContext, account: *const crate::account_ref::AccountRefHandle, options: *const crate::driver_options::DriverOptionsHandle, queue: *mut crate::completion::CompletionQueue, user_data: isize, out_pre_error: *mut crate::error::CosmosStatusCode) -> *mut crate::completion::OperationHandle;
    #[no_mangle]
    pub extern "C" fn cosmos_driver_resolve_container_submit(driver: *const crate::driver::DriverHandle, database_id: crate::string::CosmosStringView, container_id: crate::string::CosmosStringView, queue: *mut crate::completion::CompletionQueue, user_data: isize, out_pre_error: *mut crate::error::CosmosStatusCode) -> *mut crate::completion::OperationHandle;
    #[no_mangle]
    pub extern "C" fn cosmos_submit_operation(driver: *const crate::driver::DriverHandle, request: *const crate::op_request::CosmosOperationRequest, queue: *mut crate::completion::CompletionQueue, user_data: isize, out_pre_error: *mut crate::error::CosmosStatusCode) -> *mut crate::completion::OperationHandle;
    #[no_mangle]
    pub extern "C" fn cosmos_submit_singleton_operation(driver: *const crate::driver::DriverHandle, request: *const crate::op_request::CosmosOperationRequest, queue: *mut crate::completion::CompletionQueue, user_data: isize, out_pre_error: *mut crate::error::CosmosStatusCode) -> *mut crate::completion::OperationHandle;
}
```
