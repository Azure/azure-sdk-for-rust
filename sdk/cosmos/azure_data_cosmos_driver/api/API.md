# azure_data_cosmos_driver

- **Description**: Core implementation layer for Azure Cosmos DB - provides transport, routing, and protocol handling for cross-language SDK reuse
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `rustls`
  - `tokio`
- `fault_injection`
- `native_tls`
- `rustls`
- `tokio`

```rust
#![cfg_attr(fuzzing, recursion_limit = "256")]
pub use azure_data_cosmos_driver::models::activity_id::ActivityId;
pub use azure_data_cosmos_driver::driver::cosmos_driver::CosmosDriver;
pub use azure_data_cosmos_driver::driver::runtime::CosmosDriverRuntime;
pub use azure_data_cosmos_driver::driver::runtime::CosmosDriverRuntimeBuilder;
pub use azure_data_cosmos_driver::error::CosmosError;
pub use azure_data_cosmos_driver::error::CosmosErrorBuilder;
pub use azure_data_cosmos_driver::models::cosmos_response::CosmosResponse;
pub use azure_data_cosmos_driver::error::cosmos_status::CosmosStatus;
pub use azure_data_cosmos_driver::diagnostics::diagnostics_context::DiagnosticsContext;
pub use azure_data_cosmos_driver::options::diagnostics_options::DiagnosticsOptions;
pub use azure_data_cosmos_driver::options::diagnostics_thresholds::DiagnosticsThresholds;
pub use azure_data_cosmos_driver::options::diagnostics_options::DiagnosticsVerbosity;
pub use azure_data_cosmos_driver::options::driver_options::DriverOptions;
pub use azure_data_cosmos_driver::diagnostics::diagnostics_context::ExecutionContext;
pub use azure_data_cosmos_driver::driver::dataflow::pipeline::OperationPlan;
pub use azure_data_cosmos_driver::models::request_charge::RequestCharge;
pub use azure_data_cosmos_driver::diagnostics::diagnostics_context::RequestDiagnostics;
pub use azure_data_cosmos_driver::diagnostics::diagnostics_context::RequestHandle;
pub use azure_data_cosmos_driver::models::response_body::ResponseBody;
pub use azure_data_cosmos_driver::error::Result;
pub use azure_data_cosmos_driver::error::cosmos_status::SubStatusCode;
pub mod binary_json {
    pub use azure_data_cosmos_driver::binary_json::error::BinaryError;
    pub use azure_data_cosmos_driver::binary_json::error::Result;
    pub use azure_data_cosmos_driver::binary_json::reader::decode;
    pub use azure_data_cosmos_driver::binary_json::writer::encode;
    pub use azure_data_cosmos_driver::binary_json::de::from_slice;
    pub use azure_data_cosmos_driver::binary_json::ser::to_vec;
    pub fn is_binary(buffer: &[u8]) -> bool;
    pub fn transcode_to_binary(buffer: &[u8]) -> Result<Vec<u8>>;
    pub fn transcode_to_text(buffer: &[u8]) -> Result<Vec<u8>>;
    pub const PREAMBLE: u8 = 0x80;
    pub mod de {
        pub fn from_slice<'de, T>(buffer: &'de [u8]) -> super::Result<T> where T: serde::Deserialize<'de>;
    }
    pub mod error {
        #[derive(Clone, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum BinaryError {
            UnexpectedEof { needed: usize },
            InvalidMarker { marker: u8, offset: usize },
            InvalidLength { detail: &'static str },
            InvalidUtf8 { offset: usize },
            InvalidNumber { detail: &'static str },
            UnresolvedReference { target: usize },
            UnsupportedUserString { id: usize },
            DepthLimitExceeded { limit: usize },
            MissingPreamble { found: u8 },
            TrailingBytes { remaining: usize },
            Custom(String),
        }
        impl Display for BinaryError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl Error for BinaryError {
        }
        pub type Result<T> = std::result::Result<T, BinaryError>;
    }
    pub mod markers {
        pub const ARR0: u8 = 0xE0;
        pub const ARR1: u8 = 0xE1;
        pub const ARR_ARR_NUM_C1C1: u8 = 0xF2;
        pub const ARR_ARR_NUM_C2C2: u8 = 0xF3;
        pub const ARR_L1: u8 = 0xE2;
        pub const ARR_L2: u8 = 0xE3;
        pub const ARR_L4: u8 = 0xE4;
        pub const ARR_LC1: u8 = 0xE5;
        pub const ARR_LC2: u8 = 0xE6;
        pub const ARR_LC4: u8 = 0xE7;
        pub const ARR_NUM_C1: u8 = 0xF0;
        pub const ARR_NUM_C2: u8 = 0xF1;
        pub const BASE64_STRING_LENGTH1: u8 = 0x71;
        pub const BASE64_STRING_LENGTH2: u8 = 0x72;
        pub const BASE64_URL_STRING_LENGTH1: u8 = 0x73;
        pub const BASE64_URL_STRING_LENGTH2: u8 = 0x74;
        pub const BINARY_1BYTE_LENGTH: u8 = 0xDD;
        pub const BINARY_2BYTE_LENGTH: u8 = 0xDE;
        pub const BINARY_4BYTE_LENGTH: u8 = 0xDF;
        pub const COMPRESSED_DATE_TIME_STRING: u8 = 0x7A;
        pub const COMPRESSED_LOWERCASE_HEX_STRING: u8 = 0x78;
        pub const COMPRESSED_UPPERCASE_HEX_STRING: u8 = 0x79;
        pub const DOUBLE_QUOTED_LOWERCASE_GUID_STRING: u8 = 0x77;
        pub const ENCODED_STRING_LENGTH_MASK: u8 = 0x7F;
        pub const ENCODED_STRING_LENGTH_MAX: u8 = 0xC0;
        pub const ENCODED_STRING_LENGTH_MIN: u8 = 0x80;
        pub const FALSE: u8 = 0xD1;
        pub const FLOAT16: u8 = 0xCF;
        pub const FLOAT32: u8 = 0xCD;
        pub const FLOAT64: u8 = 0xCE;
        pub const GUID: u8 = 0xD3;
        pub const INT16: u8 = 0xD9;
        pub const INT32: u8 = 0xDA;
        pub const INT64: u8 = 0xDB;
        pub const INT8: u8 = 0xD8;
        pub const INVALID: u8 = 0xFF;
        pub const LITERAL_INT_MAX: u8 = 0x20;
        pub const LITERAL_INT_MIN: u8 = 0x00;
        pub const LOWERCASE_GUID_STRING: u8 = 0x75;
        pub const NULL: u8 = 0xD0;
        pub const NUMBER_DOUBLE: u8 = 0xCC;
        pub const NUMBER_INT16: u8 = 0xC9;
        pub const NUMBER_INT32: u8 = 0xCA;
        pub const NUMBER_INT64: u8 = 0xCB;
        pub const NUMBER_UINT64: u8 = 0xC7;
        pub const NUMBER_UINT8: u8 = 0xC8;
        pub const OBJ0: u8 = 0xE8;
        pub const OBJ1: u8 = 0xE9;
        pub const OBJ_L1: u8 = 0xEA;
        pub const OBJ_L2: u8 = 0xEB;
        pub const OBJ_L4: u8 = 0xEC;
        pub const OBJ_LC1: u8 = 0xED;
        pub const OBJ_LC2: u8 = 0xEE;
        pub const OBJ_LC4: u8 = 0xEF;
        pub const PACKED_4BIT_STRING: u8 = 0x7B;
        pub const PACKED_5BIT_STRING: u8 = 0x7C;
        pub const PACKED_6BIT_STRING: u8 = 0x7D;
        pub const PACKED_7BIT_STRING_LENGTH1: u8 = 0x7E;
        pub const PACKED_7BIT_STRING_LENGTH2: u8 = 0x7F;
        pub const STR_L1: u8 = 0xC0;
        pub const STR_L2: u8 = 0xC1;
        pub const STR_L4: u8 = 0xC2;
        pub const STR_R1: u8 = 0xC3;
        pub const STR_R2: u8 = 0xC4;
        pub const STR_R3: u8 = 0xC5;
        pub const STR_R4: u8 = 0xC6;
        pub const SYSTEM_STRING_1BYTE_MAX: u8 = 0x40;
        pub const SYSTEM_STRING_1BYTE_MIN: u8 = 0x20;
        pub const TRUE: u8 = 0xD2;
        pub const UINT32: u8 = 0xDC;
        pub const UINT8: u8 = 0xD7;
        pub const UPPERCASE_GUID_STRING: u8 = 0x76;
        pub const USER_STRING_1BYTE_MAX: u8 = 0x60;
        pub const USER_STRING_1BYTE_MIN: u8 = 0x40;
        pub const USER_STRING_2BYTE_MAX: u8 = 0x68;
        pub const USER_STRING_2BYTE_MIN: u8 = 0x60;
    }
    pub mod reader {
        pub fn decode(buffer: &[u8]) -> super::Result<serde_json::Value>;
    }
    pub mod ser {
        pub fn to_vec<T: Serialize + ?Sized>(value: &T) -> super::Result<Vec<u8>>;
    }
    pub mod system_strings {
        pub fn system_string(index: usize) -> Option<&'static str>;
        pub fn system_string_for_marker(marker: u8) -> Option<&'static str>;
        pub const SYSTEM_STRINGS: [&str; 32] = _;
        pub const SYSTEM_STRING_COUNT: usize = 32;
    }
    pub mod writer {
        pub fn encode(value: &serde_json::Value) -> Vec<u8>;
    }
}
pub mod diagnostics {
    #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    pub struct CompactedRun {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        pub endpoint: String,
        #[serde(flatten)]
        pub status: crate::models::CosmosStatus,
        pub execution_context: super::diagnostics_context::ExecutionContext,
        pub count: usize,
        pub total_request_charge: crate::models::RequestCharge,
        pub min_duration_ms: u64,
        pub max_duration_ms: u64,
        pub p50_duration_ms: u64,
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    pub struct CompactionInfo {
        pub original_request_count: usize,
        pub retained_request_count: usize,
        pub collapsed_runs: usize,
        pub total_runs: usize,
        #[serde(default, skip_serializing_if = "bool_is_false")]
        pub retained_truncated: bool,
        #[serde(default, skip_serializing_if = "usize_is_zero")]
        pub omitted_runs: usize,
        #[serde(default, skip_serializing_if = "usize_is_zero")]
        pub omitted_request_count: usize,
        pub runs: Vec<CompactedRun>,
    }
    #[non_exhaustive]
    pub struct DiagnosticsContext {
    }
    impl DiagnosticsContext {
        pub fn activity_id(&self) -> &ActivityId;
        pub fn compaction(&self) -> Option<&CompactionInfo>;
        pub fn duration(&self) -> Duration;
        pub fn effective_status(&self) -> Option<CosmosStatus>;
        pub fn fault_injection_enabled(&self) -> bool;
        pub fn hedge_diagnostics(&self) -> Option<&HedgeDiagnostics>;
        pub fn hedging_started(&self) -> bool;
        pub fn is_completed(&self) -> bool;
        pub fn is_failure(&self) -> bool;
        pub fn is_threshold_violated(&self, thresholds: &DiagnosticsThresholds) -> bool;
        pub fn is_threshold_violated_for(&self, thresholds: &DiagnosticsThresholds, operation_name: Option<&str>) -> bool;
        pub fn machine_id(&self) -> Option<&str>;
        pub fn operation_name(&self) -> Option<&str>;
        pub fn patch_tracking_id(&self) -> Option<PatchTrackingId>;
        pub fn regions_contacted(&self) -> Vec<Region>;
        pub fn request_count(&self) -> usize;
        pub fn requested_regions(&self) -> Vec<RequestedRegion>;
        pub fn requests(&self) -> Arc<Vec<RequestDiagnostics>>;
        pub fn responded_regions(&self) -> Vec<&Region>;
        pub fn retained_request_count(&self) -> usize;
        pub fn status(&self) -> Option<&CosmosStatus>;
        pub fn threshold_breach_for(&self, thresholds: &DiagnosticsThresholds, operation_name: Option<&str>) -> Option<ThresholdBreach>;
        pub fn to_json_string(&self, verbosity: Option<DiagnosticsVerbosity>) -> &str;
        pub fn total_request_charge(&self) -> RequestCharge;
        pub fn total_requested_regions(&self) -> usize;
        pub fn total_responded_regions(&self) -> usize;
    }
    impl Clone for DiagnosticsContext {
        fn clone(&self) -> Self;
    }
    impl Debug for DiagnosticsContext {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl Display for DiagnosticsContext {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl Eq for DiagnosticsContext {
    }
    impl PartialEq for DiagnosticsContext {
        fn eq(&self, other: &Self) -> bool;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    pub struct FailedTransportShardDiagnostics {
    }
    impl FailedTransportShardDiagnostics {
        pub fn error(&self) -> &str;
        pub fn request_sent(&self) -> RequestSentStatus;
        pub fn transport_shard(&self) -> &TransportShardDiagnostics;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct HedgeDiagnostics {
    }
    impl HedgeDiagnostics {
        pub fn alternate_region(&self) -> Option<&Region>;
        pub fn primary_region(&self) -> &Region;
        pub fn response_region(&self) -> Option<&Region>;
        pub fn strategy_config(&self) -> HedgingStrategyConfig;
        pub fn terminal_state(&self) -> HedgeTerminalState;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct HedgingStrategyConfig {
    }
    impl HedgingStrategyConfig {
        pub fn threshold(&self) -> HedgeThreshold;
    }
    #[derive(Clone, Debug)]
    pub struct ProxyConfig {
        pub proxy_allowed: bool,
        pub https_proxy_set: bool,
        pub http_proxy_set: bool,
    }
    impl ProxyConfig {
        pub fn from_env(proxy_allowed: bool) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    pub struct RequestDiagnostics {
    }
    impl RequestDiagnostics {
        pub fn activity_id(&self) -> Option<&ActivityId>;
        pub fn completed_at(&self) -> Option<Instant>;
        pub fn duration_ms(&self) -> u64;
        pub fn endpoint(&self) -> &str;
        pub fn error(&self) -> Option<&str>;
        pub fn events(&self) -> &[RequestEvent];
        pub fn execution_context(&self) -> ExecutionContext;
        pub fn failed_transport_shards(&self) -> &[FailedTransportShardDiagnostics];
        #[cfg(feature = "fault_injection")]
        pub fn fault_injection_evaluations(&self) -> &[crate::fault_injection::FaultInjectionEvaluation];
        pub fn local_shard_retry_count(&self) -> u32;
        pub fn operation_name(&self) -> Option<&str>;
        pub fn pipeline_type(&self) -> PipelineKind;
        pub fn region(&self) -> Option<&Region>;
        pub fn request_charge(&self) -> RequestCharge;
        pub fn request_sent(&self) -> RequestSentStatus;
        pub fn server_duration_ms(&self) -> Option<f64>;
        pub fn session_token(&self) -> Option<&str>;
        pub fn started_at(&self) -> Instant;
        pub fn status(&self) -> &CosmosStatus;
        pub fn timed_out(&self) -> bool;
        pub fn transport_http_version(&self) -> TransportHttpVersion;
        pub fn transport_kind(&self) -> TransportKind;
        pub fn transport_security(&self) -> TransportSecurity;
        pub fn transport_shard(&self) -> Option<&TransportShardDiagnostics>;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    pub struct RequestEvent {
    }
    impl RequestEvent {
        pub fn details(&self) -> Option<&str>;
        pub fn duration_ms(&self) -> Option<u64>;
        pub fn event_type(&self) -> &RequestEventType;
        pub fn new(event_type: RequestEventType) -> Self;
        pub fn timestamp(&self) -> Instant;
        pub fn with_details<impl Into<String>: Into<String>>(self, details: impl Into<String>) -> Self;
        pub fn with_duration(event_type: RequestEventType, duration: Duration) -> Self;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct RequestHandle(/* private fields */);
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct RequestedRegion {
        pub region: crate::options::Region,
        pub reason: ExecutionContext,
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    pub struct TransportShardDiagnostics {
    }
    impl TransportShardDiagnostics {
        pub fn consecutive_failures(&self) -> u32;
        pub fn estimated_inflight(&self) -> u32;
        pub fn marked_for_eviction(&self) -> bool;
        pub fn shard_id(&self) -> u64;
        pub fn total_cancellations(&self) -> u64;
        pub fn total_failures(&self) -> u64;
        pub fn total_requests(&self) -> u64;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "snake_case")]
    pub enum ExecutionContext {
        Initial,
        OperationRetry,
        TransportRetry,
        Hedging,
        RegionFailover,
        CircuitBreakerProbe,
    }
    impl ExecutionContext {
        pub fn as_str(&self) -> &'static str;
    }
    impl AsRef<str> for ExecutionContext {
        fn as_ref(&self) -> &str;
    }
    impl Display for ExecutionContext {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum HedgeTerminalState {
        PrimaryWonPreThreshold,
        DeadlineExceededPreThreshold,
        PrimaryWonAfterHedge,
        AlternateWon,
        #[non_exhaustive]
        BothTransient { deadline_elapsed: bool },
        CancelledAwaitingPartner,
    }
    impl HedgeTerminalState {
        pub fn as_str(&self) -> &'static str;
    }
    impl Display for HedgeTerminalState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "snake_case")]
    pub enum PipelineKind {
        Metadata,
        DataPlane,
    }
    impl PipelineKind {
        pub fn as_str(self) -> &'static str;
        pub fn is_data_plane(self) -> bool;
        pub fn is_metadata(self) -> bool;
    }
    impl AsRef<str> for PipelineKind {
        fn as_ref(&self) -> &str;
    }
    impl Display for PipelineKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "snake_case")]
    pub enum RequestEventType {
        RoutingFallback,
        TransportStart,
        ResponseHeadersReceived,
        TransportComplete,
        TransportFailed,
    }
    impl RequestEventType {
        pub fn as_str(&self) -> &str;
        pub fn indicates_request_sent(&self) -> bool;
    }
    impl AsRef<str> for RequestEventType {
        fn as_ref(&self) -> &str;
    }
    impl Display for RequestEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "snake_case")]
    pub enum RequestSentStatus {
        Sent,
        NotSent,
        #[default]
        Unknown,
    }
    impl RequestSentStatus {
        pub fn as_str(&self) -> &'static str;
        pub fn definitely_not_sent(&self) -> bool;
        pub fn definitely_sent(&self) -> bool;
        pub fn may_have_been_sent(&self) -> bool;
    }
    impl AsRef<str> for RequestSentStatus {
        fn as_ref(&self) -> &str;
    }
    impl Display for RequestSentStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum ThresholdBreach {
        PointLatency,
        NonPointLatency,
        RequestCharge,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "snake_case")]
    pub enum TransportHttpVersion {
        Http11,
        Http2,
    }
    impl TransportHttpVersion {
        pub fn as_str(self) -> &'static str;
        pub fn is_http11(self) -> bool;
        pub fn is_http2(self) -> bool;
    }
    impl AsRef<str> for TransportHttpVersion {
        fn as_ref(&self) -> &str;
    }
    impl Display for TransportHttpVersion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "snake_case")]
    pub enum TransportKind {
        #[default]
        Gateway,
        GatewayV2,
    }
    impl TransportKind {
        pub fn as_str(self) -> &'static str;
        pub fn is_gateway(self) -> bool;
        pub fn is_gateway_v2(self) -> bool;
    }
    impl AsRef<str> for TransportKind {
        fn as_ref(&self) -> &str;
    }
    impl Display for TransportKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "snake_case")]
    pub enum TransportSecurity {
        #[default]
        Secure,
        EmulatorWithInsecureCertificates,
    }
    impl TransportSecurity {
        pub fn as_str(self) -> &'static str;
        pub fn is_emulator(self) -> bool;
        pub fn is_secure(self) -> bool;
    }
    impl AsRef<str> for TransportSecurity {
        fn as_ref(&self) -> &str;
    }
    impl Display for TransportSecurity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
}
pub mod driver {
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct CosmosDriver {
    }
    impl CosmosDriver {
        #[cfg(any(test, feature = "__internal_testing"))]
        pub fn __test_only_force_ppaf_enabled(&self);
        #[cfg(any(test, feature = "__internal_testing"))]
        pub fn __test_only_hub_region_cache_snapshot(&self) -> Vec<(String, String)>;
        pub fn account(&self) -> &AccountReference;
        #[cfg(feature = "preview_dtx")]
        pub async fn execute_distributed_transaction(&self, request: crate::models::DistributedTransactionRequest, options: OperationOptions) -> crate::error::Result<crate::models::DistributedTransactionResponse>;
        pub async fn execute_operation(&self, operation: CosmosOperation, options: OperationOptions) -> crate::error::Result<Option<crate::models::CosmosResponse>>;
        pub async fn execute_plan(&self, plan: &mut OperationPlan, container: Option<ContainerReference>, options: OperationOptions) -> crate::error::Result<Option<crate::models::CosmosResponse>>;
        pub async fn execute_singleton_operation(&self, operation: CosmosOperation, options: OperationOptions) -> crate::error::Result<crate::models::CosmosResponse>;
        pub async fn initialize(&self) -> crate::error::Result<()>;
        pub fn operation_options_view<'a>(&self, operation_options: &'a OperationOptions) -> OperationOptionsView<'a>;
        pub fn options(&self) -> &DriverOptions;
        pub async fn plan_operation(&self, operation: CosmosOperation, options: &OperationOptions, continuation: Option<&ContinuationToken>, plan_options: &PlanOptions) -> crate::error::Result<OperationPlan>;
        pub async fn prime_container(&self, db_name: &str, container_name: &str) -> crate::error::Result<()>;
        pub async fn resolve_all_partition_key_ranges(&self, container: &ContainerReference, force_refresh: bool) -> crate::error::Result<Option<Vec<crate::models::partition_key_range::PartitionKeyRange>>>;
        pub async fn resolve_container(&self, db_name: &str, container_name: &str, operation_options: OperationOptions) -> crate::error::Result<ContainerReference>;
        pub async fn resolve_container_by_name(&self, db_name: &str, container_name: &str, operation_options: OperationOptions) -> crate::error::Result<ContainerReference>;
        pub async fn resolve_container_by_rid(&self, container_rid: &str, operation_options: OperationOptions) -> crate::error::Result<ContainerReference>;
        pub async fn resolve_partition_key_ranges_for_key(&self, container: &ContainerReference, partition_key: &PartitionKey, force_refresh: bool) -> crate::error::Result<Option<Vec<crate::models::partition_key_range::PartitionKeyRange>>>;
        pub fn runtime(&self) -> &CosmosDriverRuntime;
        pub fn user_agent(&self) -> &Arc<UserAgent>;
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct CosmosDriverRuntime {
    }
    impl CosmosDriverRuntime {
        pub fn builder() -> CosmosDriverRuntimeBuilder;
        pub fn client_options(&self) -> &ClientOptions;
        pub fn connection_pool(&self) -> &ConnectionPoolOptions;
        pub fn correlation_id(&self) -> Option<&CorrelationId>;
        pub async fn create_driver(self: &Arc<Self>, driver_options: DriverOptions) -> crate::error::Result<Arc<CosmosDriver>>;
        pub fn default_operation_options(&self) -> Arc<OperationOptions>;
        pub fn diagnostics_options(&self) -> &DiagnosticsOptions;
        pub fn effective_correlation(&self) -> Option<&str>;
        pub fn env_operation_options(&self) -> &Arc<OperationOptions>;
        pub fn env_override_operation_options(&self) -> &Arc<OperationOptions>;
        pub fn proxy_configuration(&self) -> &ProxyConfig;
        pub fn set_default_operation_options(&self, options: OperationOptions);
        pub fn user_agent(&self) -> &Arc<UserAgent>;
        pub fn user_agent_suffix(&self) -> Option<&UserAgentSuffix>;
        pub fn workload_id(&self) -> Option<WorkloadId>;
        pub fn wrapping_sdk_identifier(&self) -> Option<&str>;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct CosmosDriverRuntimeBuilder {
    }
    impl CosmosDriverRuntimeBuilder {
        pub async fn build(self) -> crate::error::Result<Arc<CosmosDriverRuntime>>;
        pub fn new() -> Self;
        pub fn with_client_options(self, options: ClientOptions) -> Self;
        pub fn with_connection_pool(self, options: ConnectionPoolOptions) -> Self;
        pub fn with_correlation_id(self, correlation_id: CorrelationId) -> Self;
        pub fn with_cpu_refresh_interval(self, interval: Duration) -> Self;
        pub fn with_default_operation_options(self, options: OperationOptions) -> Self;
        pub fn with_diagnostics_options(self, options: DiagnosticsOptions) -> Self;
        #[cfg(feature = "__internal_mocking")]
        pub fn with_mock_http_client_factory(self, factory: Arc<dyn HttpClientFactory>) -> Self;
        pub fn with_user_agent_suffix(self, suffix: UserAgentSuffix) -> Self;
        pub fn with_workload_id(self, workload_id: WorkloadId) -> Self;
        pub fn with_wrapping_sdk_identifier<impl Into<String>: Into<String>>(self, identifier: impl Into<String>) -> Self;
    }
    pub struct OperationPlan {
    }
    impl OperationPlan {
        pub fn to_continuation_token(&self) -> crate::error::Result<ContinuationToken>;
    }
}
pub mod error {
    pub fn set_backtrace_options(options: BacktraceOptions);
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct BacktraceOptions {
        pub max_captures_per_second: u32,
        pub max_resolutions_per_second: u32,
    }
    impl Default for BacktraceOptions {
        fn default() -> Self;
    }
    #[derive(Clone)]
    pub struct CosmosError {
    }
    impl CosmosError {
        pub fn backtrace(&self) -> Option<Arc<str>>;
        pub fn diagnostics(&self) -> Option<Arc<DiagnosticsContext>>;
        pub fn is_from_wire(&self) -> bool;
        pub fn patch_tracking_id(&self) -> Option<PatchTrackingId>;
        pub fn response(&self) -> Option<&CosmosResponse>;
        pub fn status(&self) -> CosmosStatus;
    }
    impl CosmosError {
        pub fn builder() -> CosmosErrorBuilder;
    }
    impl Debug for CosmosError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Display for CosmosError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Error for CosmosError {
        fn source(&self) -> Option<&dyn StdError + 'static>;
    }
    #[must_use = "CosmosErrorBuilder is inert until `.build()` is called"]
    pub struct CosmosErrorBuilder {
    }
    impl CosmosErrorBuilder {
        pub fn build(self) -> CosmosError;
        pub fn from_error(err: CosmosError) -> Self;
        pub fn with_arc_source(self, source: Arc<dyn StdError + Send + Sync + 'static>) -> Self;
        pub fn with_context<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, context: impl Into<Cow<'static, str>>) -> Self;
        pub fn with_diagnostics(self, diagnostics: Arc<DiagnosticsContext>) -> Self;
        pub fn with_message<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, message: impl Into<Cow<'static, str>>) -> Self;
        pub fn with_patch_tracking_id(self, id: PatchTrackingId) -> Self;
        pub fn with_response(self, response: CosmosResponse) -> Self;
        pub fn with_source<E>(self, source: E) -> Self where E: StdError + Send + Sync + 'static;
        pub fn with_status(self, status: CosmosStatus) -> Self;
    }
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct CosmosStatus {
    }
    impl CosmosStatus {
        const AUTHENTICATION_TOKEN_ACQUISITION_FAILED: CosmosStatus = _;
        const CLIENT_BAD_REQUEST: CosmosStatus = _;
        const CLIENT_BUILD_RESPONSE_INVOKED_ON_FAILURE: CosmosStatus = _;
        const CLIENT_CHANGE_FEED_PIPELINE_UNEXPECTEDLY_DRAINED: CosmosStatus = _;
        const CLIENT_COMPUTE_RANGE_INVOKED_WITH_EMPTY_PARTITION_KEY: CosmosStatus = _;
        const CLIENT_CONNECTION_STRING_EMPTY: CosmosStatus = _;
        const CLIENT_CONNECTION_STRING_MALFORMED_PART: CosmosStatus = _;
        const CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_ENDPOINT: CosmosStatus = _;
        const CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_KEY: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_AFTER_TRANSCODE_FAILURE: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_FETCH_IN_FLIGHT: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_NON_QUERY_OPERATION: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_UNEXPECTED_NESTED_SHAPE: CosmosStatus = _;
        const CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED: CosmosStatus = _;
        const CLIENT_CROSS_PARTITION_QUERY_REQUIRES_CONTAINER_REF: CosmosStatus = _;
        const CLIENT_DISTINCT_CANNOT_FORWARD_SPLIT: CosmosStatus = _;
        const CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED: CosmosStatus = _;
        const CLIENT_DISTINCT_VALUE_TOO_DEEPLY_NESTED: CosmosStatus = _;
        const CLIENT_DRIVER_NOT_INITIALIZED: CosmosStatus = _;
        const CLIENT_DUPLICATE_FAULT_INJECTION_RULE_ID: CosmosStatus = _;
        const CLIENT_FEED_RANGE_REQUIRES_FANOUT_PIPELINE: CosmosStatus = _;
        const CLIENT_GENERATED_401: CosmosStatus = _;
        const CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED: CosmosStatus = _;
        const CLIENT_IMDS_HTTP_CLIENT_CONSTRUCTION_FAILED: CosmosStatus = _;
        const CLIENT_IMDS_REQWEST_FEATURE_REQUIRED: CosmosStatus = _;
        const CLIENT_INVALID_ACCOUNT_ENDPOINT_URL: CosmosStatus = _;
        const CLIENT_INVALID_RESOURCE_ID: CosmosStatus = _;
        const CLIENT_INVALID_URL: CosmosStatus = _;
        const CLIENT_MIXED_NAME_RID_ADDRESSING: CosmosStatus = _;
        const CLIENT_NON_MULTIHASH_PARTITION_KEY_ARITY_MISMATCH: CosmosStatus = _;
        const CLIENT_NON_STREAMING_ORDER_BY_CONTINUATION_UNSUPPORTED: CosmosStatus = _;
        const CLIENT_NON_STREAMING_ORDER_BY_REQUIRES_FINITE_WINDOW: CosmosStatus = _;
        const CLIENT_NON_STREAMING_ORDER_BY_WINDOW_TOO_LARGE: CosmosStatus = _;
        const CLIENT_NO_OVERLAPPING_FEED_RANGES_FOR_SESSION_TOKEN: CosmosStatus = _;
        const CLIENT_NO_THROUGHPUT_OFFER_FOR_RESOURCE: CosmosStatus = _;
        const CLIENT_OPAQUE_TOKEN_INVALID_FOR_CROSS_PARTITION_QUERY: CosmosStatus = _;
        const CLIENT_ORDER_BY_COMPLEX_VALUE_UNSUPPORTED: CosmosStatus = _;
        const CLIENT_PARTITION_KEY_EMPTY: CosmosStatus = _;
        const CLIENT_PARTITION_KEY_RANGE_CACHE_REQUIRED: CosmosStatus = _;
        const CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS: CosmosStatus = _;
        const CLIENT_PREFIX_PARTITION_KEY_REQUIRES_MULTIHASH: CosmosStatus = _;
        const CLIENT_QUERY_PLAN_COMPLEX_PROJECTION_UNSUPPORTED: CosmosStatus = _;
        const CLIENT_QUERY_PLAN_INVALID_TOP_OFFSET_LIMIT: CosmosStatus = _;
        const CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES: CosmosStatus = _;
        const CLIENT_QUERY_PLAN_RANGE_NOT_COVERED_BY_TOPOLOGY: CosmosStatus = _;
        const CLIENT_QUERY_REWRITE_BODY_INVALID: CosmosStatus = _;
        const CLIENT_REQUEST_URL_MISSING_HOST: CosmosStatus = _;
        const CLIENT_REQUEST_URL_MISSING_KNOWN_PORT: CosmosStatus = _;
        const CLIENT_REQWEST_FEATURE_REQUIRED: CosmosStatus = _;
        const CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT: CosmosStatus = _;
        const CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE: CosmosStatus = _;
        const CLIENT_SPLIT_RETRIES_EXHAUSTED: CosmosStatus = _;
        const CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID: CosmosStatus = _;
        const CLIENT_THROUGHPUT_CONTROL_GROUP_NOT_REGISTERED: CosmosStatus = _;
        const CLIENT_THROUGHPUT_CONTROL_GROUP_REGISTRATION_FAILED: CosmosStatus = _;
        const CLIENT_THROUGHPUT_POLLER_INCOMPLETE: CosmosStatus = _;
        const CLIENT_TOPOLOGY_PROVIDER_MISSING: CosmosStatus = _;
        const CLIENT_TOPOLOGY_RESOLUTION_FAILED: CosmosStatus = _;
        const CLIENT_UNKNOWN_CONSISTENCY_LEVEL: CosmosStatus = _;
        const CLIENT_UNKNOWN_PRIORITY_LEVEL: CosmosStatus = _;
        const CLIENT_UNSUPPORTED_QUERY_FEATURE: CosmosStatus = _;
        const COMPLETING_PARTITION_MIGRATION: CosmosStatus = _;
        const COMPLETING_SPLIT: CosmosStatus = _;
        const CROSS_PARTITION_QUERY_NOT_SERVABLE: CosmosStatus = _;
        const DATABASE_ACCOUNT_NOT_FOUND: CosmosStatus = _;
        const NAME_CACHE_STALE: CosmosStatus = _;
        const PARTITION_KEY_RANGE_GONE: CosmosStatus = _;
        const READ_SESSION_NOT_AVAILABLE: CosmosStatus = _;
        const RU_BUDGET_EXCEEDED: CosmosStatus = _;
        const SERIALIZATION_REQUEST_BODY_INVALID: CosmosStatus = _;
        const SERIALIZATION_RESPONSE_BODY_INVALID: CosmosStatus = _;
        const SERVICE_ORDER_BY_ENVELOPE_INVALID: CosmosStatus = _;
        const SERVICE_QUERY_PLAN_ORDER_BY_MISSING_REWRITTEN_QUERY: CosmosStatus = _;
        const SERVICE_RETURNED_OBJECT_WITHOUT_RID: CosmosStatus = _;
        const SERVICE_RETURNED_OFFER_WITHOUT_ID: CosmosStatus = _;
        const TRANSPORT_BODY_READ_FAILED: CosmosStatus = _;
        const TRANSPORT_CONNECTION_FAILED: CosmosStatus = _;
        const TRANSPORT_DNS_FAILED: CosmosStatus = _;
        const TRANSPORT_GENERATED_503: CosmosStatus = _;
        const TRANSPORT_HTTP2_INCOMPATIBLE: CosmosStatus = _;
        const TRANSPORT_IO_FAILED: CosmosStatus = _;
        const WRITE_FORBIDDEN: CosmosStatus = _;
        pub fn is_bad_request(&self) -> bool;
        pub fn is_conflict(&self) -> bool;
        pub fn is_database_account_not_found(&self) -> bool;
        pub fn is_forbidden(&self) -> bool;
        pub fn is_gone(&self) -> bool;
        pub fn is_not_found(&self) -> bool;
        pub fn is_partition_key_range_gone(&self) -> bool;
        pub fn is_precondition_failed(&self) -> bool;
        pub fn is_read_session_not_available(&self) -> bool;
        pub fn is_retry_with(&self) -> bool;
        pub fn is_service_unavailable(&self) -> bool;
        pub fn is_success(&self) -> bool;
        pub fn is_throttled(&self) -> bool;
        pub fn is_timeout(&self) -> bool;
        pub fn is_transient(&self) -> bool;
        pub fn is_transport_generated_503(&self) -> bool;
        pub fn is_unauthorized(&self) -> bool;
        pub fn is_write_forbidden(&self) -> bool;
        pub fn name(&self) -> Option<&'static str>;
        pub fn new(status_code: StatusCode) -> Self;
        pub fn status_code(&self) -> StatusCode;
        pub fn sub_status(&self) -> Option<SubStatusCode>;
        pub fn with_sub_status(self, sub_status_code: u16) -> Self;
    }
    impl Debug for CosmosStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Display for CosmosStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<CosmosStatus> for azure_core::http::StatusCode {
        fn from(s: CosmosStatus) -> Self;
    }
    impl From<CosmosStatus> for u16 {
        fn from(s: CosmosStatus) -> Self;
    }
    impl PartialEq<CosmosStatus> for azure_core::http::StatusCode {
        fn eq(&self, other: &CosmosStatus) -> bool;
    }
    impl PartialEq<StatusCode> for CosmosStatus {
        fn eq(&self, other: &StatusCode) -> bool;
    }
    impl Serialize for CosmosStatus {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    #[derive(Clone, Copy, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(transparent)]
    pub struct SubStatusCode(/* private fields */);
    impl SubStatusCode {
        const AAD_TOKEN_EXPIRED: SubStatusCode = _;
        const AE_QUEUE_FULL: SubStatusCode = _;
        const ARCHIVAL_PARTITION_NOT_PRESENT: SubStatusCode = _;
        const ARCHIVAL_PARTITION_PENDING_CATCHUP: SubStatusCode = _;
        const ASYNC_READER_WRITER_LOCK: SubStatusCode = _;
        const AUTHENTICATION_TOKEN_ACQUISITION_FAILED: SubStatusCode = _;
        const AUTH_TOKEN_NOT_FOUND_IN_CACHE: SubStatusCode = _;
        const AZURE_BACKUP_VAULT_INCREMENTAL_BACKUP_PAUSED: SubStatusCode = _;
        const AZURE_BACKUP_VAULT_INCREMENTAL_BACKUP_RESTORE_DISABLED: SubStatusCode = _;
        const AZURE_RBAC_ACCESS_DECISION_UNAVAILABLE: SubStatusCode = _;
        const BATCH_RESPONSE_SIZE_EXCEEDED: SubStatusCode = _;
        const BW_TERM_COUNT_LIMIT_EXCEEDED: SubStatusCode = _;
        const BW_TREE_IO_RATE_LIMITER: SubStatusCode = _;
        const BW_TREE_LOG_FULL_BACKPRESSURE: SubStatusCode = _;
        const CANNOT_ACQUIRE_IN_ACCOUNT_RESTORE_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_LOG_STORE_LOAD_BALANCE_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_MASTER_PARTITION_ACCESS_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_OFFER_OWNER_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_PARTITION_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_PKRANGES_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_PKRANGE_LOCK: SubStatusCode = _;
        const CHANNEL_CLOSED: SubStatusCode = _;
        const CHECKPOINT_QUEUE_DEPTH_BACKPRESSURE: SubStatusCode = _;
        const CLIENT_BUILD_RESPONSE_INVOKED_ON_FAILURE: SubStatusCode = _;
        const CLIENT_CHANGE_FEED_PIPELINE_UNEXPECTEDLY_DRAINED: SubStatusCode = _;
        const CLIENT_COMPUTE_RANGE_INVOKED_WITH_EMPTY_PARTITION_KEY: SubStatusCode = _;
        const CLIENT_CONNECTION_STRING_EMPTY: SubStatusCode = _;
        const CLIENT_CONNECTION_STRING_MALFORMED_PART: SubStatusCode = _;
        const CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_ENDPOINT: SubStatusCode = _;
        const CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_KEY: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_AFTER_TRANSCODE_FAILURE: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_FETCH_IN_FLIGHT: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_NON_QUERY_OPERATION: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_UNEXPECTED_NESTED_SHAPE: SubStatusCode = _;
        const CLIENT_CPU_OVERLOAD: SubStatusCode = _;
        const CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED: SubStatusCode = _;
        const CLIENT_CROSS_PARTITION_QUERY_REQUIRES_CONTAINER_REF: SubStatusCode = _;
        const CLIENT_DISTINCT_CANNOT_FORWARD_SPLIT: SubStatusCode = _;
        const CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED: SubStatusCode = _;
        const CLIENT_DISTINCT_VALUE_TOO_DEEPLY_NESTED: SubStatusCode = _;
        const CLIENT_DRIVER_NOT_INITIALIZED: SubStatusCode = _;
        const CLIENT_DUPLICATE_FAULT_INJECTION_RULE_ID: SubStatusCode = _;
        const CLIENT_FEED_RANGE_REQUIRES_FANOUT_PIPELINE: SubStatusCode = _;
        const CLIENT_FFI_FEED_EXHAUSTED: SubStatusCode = _;
        const CLIENT_FFI_INVALID_HEADER: SubStatusCode = _;
        const CLIENT_FFI_INVALID_OPTION_VALUE: SubStatusCode = _;
        const CLIENT_FFI_INVALID_UTF8: SubStatusCode = _;
        const CLIENT_FFI_NULL_ARGUMENT: SubStatusCode = _;
        const CLIENT_FFI_OPERATION_CANCELLED: SubStatusCode = _;
        const CLIENT_FFI_OPERATION_CONSUMED: SubStatusCode = _;
        const CLIENT_FFI_PANIC: SubStatusCode = _;
        const CLIENT_FFI_PRECONDITION_ALREADY_SET: SubStatusCode = _;
        const CLIENT_FFI_QUEUE_FULL: SubStatusCode = _;
        const CLIENT_FFI_QUEUE_SHUTDOWN: SubStatusCode = _;
        const CLIENT_FFI_RUNTIME_BUILD_FAILED: SubStatusCode = _;
        const CLIENT_FFI_UNSUPPORTED_OPERATION_FOR_MUTATOR: SubStatusCode = _;
        const CLIENT_GENERATED_401: SubStatusCode = _;
        const CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED: SubStatusCode = _;
        const CLIENT_IMDS_HTTP_CLIENT_CONSTRUCTION_FAILED: SubStatusCode = _;
        const CLIENT_IMDS_REQWEST_FEATURE_REQUIRED: SubStatusCode = _;
        const CLIENT_INVALID_ACCOUNT_ENDPOINT_URL: SubStatusCode = _;
        const CLIENT_INVALID_RESOURCE_ID: SubStatusCode = _;
        const CLIENT_INVALID_URL: SubStatusCode = _;
        const CLIENT_MIXED_NAME_RID_ADDRESSING: SubStatusCode = _;
        const CLIENT_NON_MULTIHASH_PARTITION_KEY_ARITY_MISMATCH: SubStatusCode = _;
        const CLIENT_NON_STREAMING_ORDER_BY_CONTINUATION_UNSUPPORTED: SubStatusCode = _;
        const CLIENT_NON_STREAMING_ORDER_BY_REQUIRES_FINITE_WINDOW: SubStatusCode = _;
        const CLIENT_NON_STREAMING_ORDER_BY_WINDOW_TOO_LARGE: SubStatusCode = _;
        const CLIENT_NO_OVERLAPPING_FEED_RANGES_FOR_SESSION_TOKEN: SubStatusCode = _;
        const CLIENT_NO_THROUGHPUT_OFFER_FOR_RESOURCE: SubStatusCode = _;
        const CLIENT_OPAQUE_TOKEN_INVALID_FOR_CROSS_PARTITION_QUERY: SubStatusCode = _;
        const CLIENT_OPERATION_TIMEOUT: SubStatusCode = _;
        const CLIENT_ORDER_BY_COMPLEX_VALUE_UNSUPPORTED: SubStatusCode = _;
        const CLIENT_PARTITION_KEY_EMPTY: SubStatusCode = _;
        const CLIENT_PARTITION_KEY_RANGE_CACHE_REQUIRED: SubStatusCode = _;
        const CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS: SubStatusCode = _;
        const CLIENT_PREFIX_PARTITION_KEY_REQUIRES_MULTIHASH: SubStatusCode = _;
        const CLIENT_QUERY_PLAN_COMPLEX_PROJECTION_UNSUPPORTED: SubStatusCode = _;
        const CLIENT_QUERY_PLAN_INVALID_TOP_OFFSET_LIMIT: SubStatusCode = _;
        const CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES: SubStatusCode = _;
        const CLIENT_QUERY_PLAN_RANGE_NOT_COVERED_BY_TOPOLOGY: SubStatusCode = _;
        const CLIENT_QUERY_REWRITE_BODY_INVALID: SubStatusCode = _;
        const CLIENT_REQUEST_URL_MISSING_HOST: SubStatusCode = _;
        const CLIENT_REQUEST_URL_MISSING_KNOWN_PORT: SubStatusCode = _;
        const CLIENT_REQWEST_FEATURE_REQUIRED: SubStatusCode = _;
        const CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT: SubStatusCode = _;
        const CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE: SubStatusCode = _;
        const CLIENT_SPLIT_RETRIES_EXHAUSTED: SubStatusCode = _;
        const CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID: SubStatusCode = _;
        const CLIENT_THREAD_STARVATION: SubStatusCode = _;
        const CLIENT_THROUGHPUT_CONTROL_GROUP_NOT_REGISTERED: SubStatusCode = _;
        const CLIENT_THROUGHPUT_CONTROL_GROUP_REGISTRATION_FAILED: SubStatusCode = _;
        const CLIENT_THROUGHPUT_POLLER_INCOMPLETE: SubStatusCode = _;
        const CLIENT_TOPOLOGY_PROVIDER_MISSING: SubStatusCode = _;
        const CLIENT_TOPOLOGY_RESOLUTION_FAILED: SubStatusCode = _;
        const CLIENT_UNKNOWN_CONSISTENCY_LEVEL: SubStatusCode = _;
        const CLIENT_UNKNOWN_PRIORITY_LEVEL: SubStatusCode = _;
        const CLIENT_UNSUPPORTED_QUERY_FEATURE: SubStatusCode = _;
        const COLLECTIONS_IN_PARTITION_GOT_UPDATED: SubStatusCode = _;
        const COLLECTION_CREATE_IN_PROGRESS: SubStatusCode = _;
        const COLLECTION_QUOTA_EXCEEDED: SubStatusCode = _;
        const COLLECTION_QUOTA_EXCEEDED_AUTOPILOT: SubStatusCode = _;
        const COLLECTION_RID_MISMATCH: SubStatusCode = _;
        const COLLECTION_STATE_CHANGED: SubStatusCode = _;
        const COLLECTION_TRUNCATE_NOT_ALLOWED_DURING_MERGE: SubStatusCode = _;
        const COMPLETING_PARTITION_MIGRATION: SubStatusCode = _;
        const COMPLETING_SPLIT: SubStatusCode = _;
        const COMPUTE_FEDERATION_NOT_FOUND: SubStatusCode = _;
        const COMPUTE_INTERNAL_ERROR: SubStatusCode = _;
        const CONFIGURATION_NAME_NOT_EMPTY: SubStatusCode = _;
        const CONFIGURATION_OPERATION_CANCELLED: SubStatusCode = _;
        const CONFLICT_OPERATION_IN_USER_TRANSACTION: SubStatusCode = _;
        const CONFLICT_WITH_CONTROL_PLANE: SubStatusCode = _;
        const CONNECTION_RATE_LIMITER: SubStatusCode = _;
        const CROSS_COLLECTION_TRANSACTION_NOT_SUPPORTED: SubStatusCode = _;
        const CROSS_PARTITION_QUERY_NOT_SERVABLE: SubStatusCode = _;
        const CUSTOMER_KEY_ROTATED: SubStatusCode = _;
        const DATABASE_ACCOUNT_NOT_FOUND: SubStatusCode = _;
        const DATABASE_NAME_EXISTS: SubStatusCode = _;
        const DATABASE_QUOTA_EXCEEDED: SubStatusCode = _;
        #[cfg(feature = "preview_dtx")]
        const DTC_ACCOUNT_CONFIG_FAILURE: SubStatusCode = _;
        #[cfg(feature = "preview_dtx")]
        const DTC_COORDINATOR_RACE_CONFLICT: SubStatusCode = _;
        #[cfg(feature = "preview_dtx")]
        const DTC_DISPATCH_FAILURE: SubStatusCode = _;
        #[cfg(feature = "preview_dtx")]
        const DTC_LEDGER_FAILURE: SubStatusCode = _;
        #[cfg(feature = "preview_dtx")]
        const DTC_OPERATION_ROLLED_BACK: SubStatusCode = _;
        const DUPLICATE_RETRIABLE_WRITE_REQUEST: SubStatusCode = _;
        const FEDERATION_DOES_NOT_EXIST_OR_IS_LOCKED: SubStatusCode = _;
        const GATEWAY_ENDPOINT_READ_TIMEOUT: SubStatusCode = _;
        const GATEWAY_ENDPOINT_UNAVAILABLE: SubStatusCode = _;
        const GATEWAY_THROTTLED: SubStatusCode = _;
        const GONE_EXCEPTION: SubStatusCode = _;
        const HOT_PARTITION_KEY_THROTTLED: SubStatusCode = _;
        const HTTP_LISTENER_EXCEPTION: SubStatusCode = _;
        const INITIAL_RETRIABLE_WRITE_REQUEST_COMPLETED: SubStatusCode = _;
        const INSUFFICIENT_BINDABLE_PARTITIONS: SubStatusCode = _;
        const INSUFFICIENT_CAPACITY: SubStatusCode = _;
        const INVALID_ACCOUNT_CONFIGURATION: SubStatusCode = _;
        const INVALID_ACCOUNT_STATUS: SubStatusCode = _;
        const INVALID_KEY_VAULT_CERT_URI: SubStatusCode = _;
        const INVALID_KEY_VAULT_KEY_AND_CERT_URI: SubStatusCode = _;
        const INVALID_KEY_VAULT_SECRET_URI: SubStatusCode = _;
        const INVALID_THROUGHPUT_CAP_VALUE: SubStatusCode = _;
        const INVALID_TOPOLOGY_CHANGE_REQUEST: SubStatusCode = _;
        const INVALID_TRANSACTION_ID: SubStatusCode = _;
        const KEY_DISABLED_OR_EXPIRED: SubStatusCode = _;
        const KEY_VAULT_NOT_FOUND: SubStatusCode = _;
        const KEY_VAULT_OUTBOUND_DENIED_BY_NSP: SubStatusCode = _;
        const LEAKED_PARTITION: SubStatusCode = _;
        const LEASE_NOT_FOUND: SubStatusCode = _;
        const LOCAL_AUTH_DISABLED: SubStatusCode = _;
        const LOG_FLUSH_QUEUE_DEPTH_BACKPRESSURE: SubStatusCode = _;
        const LOG_STORE_NO_FREE_SEGMENTS: SubStatusCode = _;
        const MALFORMED_CONTINUATION_TOKEN: SubStatusCode = _;
        const MASTER_SERVICE_UNAVAILABLE: SubStatusCode = _;
        const MERGE_DISABLED: SubStatusCode = _;
        const MISMATCHING_COLLECTION_RIDS_ON_MIGRATE_PARTITION: SubStatusCode = _;
        const MISSING_PARTITION_RESOURCE_ON_ABORT_MIGRATION: SubStatusCode = _;
        const MISSING_PARTITION_RESOURCE_ON_COMPLETE_MIGRATION: SubStatusCode = _;
        const MISSING_REQUEST_PARAMETER: SubStatusCode = _;
        const NAME_CACHE_STALE: SubStatusCode = _;
        const OFFER_NOT_CONFIGURED: SubStatusCode = _;
        const OFFER_REPLACE_DISABLED_AUTO_SCALE_OFFER: SubStatusCode = _;
        const OFFER_REPLACE_IN_PROGRESS: SubStatusCode = _;
        const OFFER_SCALED_UP_BY_USER: SubStatusCode = _;
        const OFFER_VALIDATION_FAILED: SubStatusCode = _;
        const OPERATION_IN_PROGRESS: SubStatusCode = _;
        const OPERATION_LOG_SIZE_TOO_BIG: SubStatusCode = _;
        const OPERATION_PAUSED: SubStatusCode = _;
        const OWNER_RESOURCE_NOT_FOUND: SubStatusCode = _;
        const PARTITIONED_RESOURCE_QUOTA_EXCEEDED: SubStatusCode = _;
        const PARTITION_FAILOVER_ERROR_CODE: SubStatusCode = _;
        const PARTITION_KEY_DEFINITION_MISSING_FOR_AUTOPILOT: SubStatusCode = _;
        const PARTITION_KEY_DEFINITION_NOT_SPECIFIED: SubStatusCode = _;
        const PARTITION_KEY_DELETE_REQUEST_LIMIT_EXCEEDED: SubStatusCode = _;
        const PARTITION_KEY_HASH_COLLISION: SubStatusCode = _;
        const PARTITION_KEY_MISMATCH: SubStatusCode = _;
        const PARTITION_KEY_QUOTA_OVER_LIMIT: SubStatusCode = _;
        const PARTITION_KEY_RANGE_GONE: SubStatusCode = _;
        const PARTITION_MIGRATING_COLLECTION_DELETED: SubStatusCode = _;
        const PARTITION_MIGRATION_DOC_COUNT_MISMATCH_SOURCE_TARGET: SubStatusCode = _;
        const PARTITION_MIGRATION_DOC_COUNT_MISMATCH_TARGET_REPLICAS: SubStatusCode = _;
        const PARTITION_MIGRATION_FAILED_TO_UPDATE_DNS: SubStatusCode = _;
        const PARTITION_MIGRATION_PARTITION_RESOURCE_NOT_FOUND: SubStatusCode = _;
        const PARTITION_MIGRATION_SHARED_THROUGHPUT_DB_PARTITION_NOT_FOUND: SubStatusCode = _;
        const PARTITION_MIGRATION_SOURCE_PARTITION_DELETED_IN_MASTER: SubStatusCode = _;
        const PARTITION_NOT_IN_MIGRATING_STATUS: SubStatusCode = _;
        const PATCH_CONDITION_NOT_MET: SubStatusCode = _;
        const PREPARE_TIME_EXCEEDED: SubStatusCode = _;
        const PROVISION_LIMIT_REACHED: SubStatusCode = _;
        const QUERY_EXECUTION_COMPLETE: SubStatusCode = _;
        const QUERY_EXECUTION_IN_PROGRESS: SubStatusCode = _;
        const QUERY_REQUEST_INITIALIZED: SubStatusCode = _;
        const QUERY_WAIT_FOR_SEQUENTIAL_PROGRESS: SubStatusCode = _;
        const QUORUM_NOT_MET: SubStatusCode = _;
        const QUOTA_EXCEEDED: SubStatusCode = _;
        const RBAC_AAD_GROUP_UNAVAILABLE: SubStatusCode = _;
        const RBAC_DISABLED_DUE_TO_ARM_PATH: SubStatusCode = _;
        const RBAC_REQUEST_NOT_AUTHORIZED: SubStatusCode = _;
        const READ_SESSION_NOT_AVAILABLE: SubStatusCode = _;
        const REDUNDANT_COLLECTION_PUT: SubStatusCode = _;
        const REPLICATION_QUEUE_FULL: SubStatusCode = _;
        const REQUEST_PREEMPTED: SubStatusCode = _;
        const RESOURCE_NOT_FOUND: SubStatusCode = _;
        const RESOURCE_SOFT_DELETED: SubStatusCode = _;
        const RETRIABLE_WRITE_RESPONSE_EXPIRED_IN_PRIMARY_CACHE: SubStatusCode = _;
        const RNTBD_CLIENT_CHANNEL: SubStatusCode = _;
        const RUPM_PARTITION_LIMIT_EXCEEDED: SubStatusCode = _;
        const RUPM_SHARED_BUDGET_EXCEEDED: SubStatusCode = _;
        const RU_BUDGET_EXCEEDED: SubStatusCode = _;
        const RU_BUDGET_EXCEEDED_FOR_MASTER: SubStatusCode = _;
        const SCHEMA_HASH_OR_ID_MISMATCH: SubStatusCode = _;
        const SCHEMA_OWNER_ID_MISMATCH: SubStatusCode = _;
        const SCRIPT_COMPILE_ERROR: SubStatusCode = _;
        const SERIALIZATION_REQUEST_BODY_INVALID: SubStatusCode = _;
        const SERIALIZATION_RESPONSE_BODY_INVALID: SubStatusCode = _;
        const SERVER_BARRIER_THROTTLED: SubStatusCode = _;
        const SERVICE_IS_OFFLINE: SubStatusCode = _;
        const SERVICE_MODULE: SubStatusCode = _;
        const SERVICE_ORDER_BY_ENVELOPE_INVALID: SubStatusCode = _;
        const SERVICE_QUERY_PLAN_ORDER_BY_MISSING_REWRITTEN_QUERY: SubStatusCode = _;
        const SERVICE_RETURNED_OBJECT_WITHOUT_RID: SubStatusCode = _;
        const SERVICE_RETURNED_OFFER_WITHOUT_ID: SubStatusCode = _;
        const SHARED_THROUGHPUT_DATABASE_COLLECTION_COUNT_EXCEEDED: SubStatusCode = _;
        const SHARED_THROUGHPUT_DATABASE_COUNT_EXCEEDED: SubStatusCode = _;
        const SHARED_THROUGHPUT_OFFER_GROW_NOT_NEEDED: SubStatusCode = _;
        const SINK_PARTITION_VALUE_DOES_NOT_MATCH_EXPECTED_BOUND: SubStatusCode = _;
        const SPLIT_DISABLED: SubStatusCode = _;
        const STALENESS_EXCEEDED_BOUND: SubStatusCode = _;
        const STORAGE_SPLIT_CONFLICTING_WITH_NWAY_THROUGHPUT_SPLIT: SubStatusCode = _;
        const STORED_PROCEDURE_CONCURRENCY: SubStatusCode = _;
        const STORE_NOT_READY: SubStatusCode = _;
        const SYSTEM_PARTITION_KEY_NOT_ALLOWED: SubStatusCode = _;
        const SYSTEM_RESOURCE_UNAVAILABLE: SubStatusCode = _;
        const THROTTLED_BY_BLOB_READ: SubStatusCode = _;
        const THROTTLED_OFFER_SCALE_DOWN: SubStatusCode = _;
        const THROTTLE_DUE_TO_ENCRYPTED_REVOKED_STORE_LOG_NOT_EMPTY: SubStatusCode = _;
        const THROTTLE_DUE_TO_REPLICATION_BACKPRESSURE: SubStatusCode = _;
        const THROTTLE_DUE_TO_RESOURCE_EXHAUSTION: SubStatusCode = _;
        const THROTTLE_DUE_TO_SPLIT: SubStatusCode = _;
        const THROTTLE_DUE_TO_STAGING_INDEX_QUEUE_FULL: SubStatusCode = _;
        const THROTTLE_DUE_TO_TRAFFIC_REGULATION: SubStatusCode = _;
        const THROTTLE_DUE_TO_TRANSPORT_BUFFER_USAGE: SubStatusCode = _;
        const THROUGHPUT_BUCKET_LIMIT_EXHAUSTED: SubStatusCode = _;
        const THROUGHPUT_CAP_EXCEEDED: SubStatusCode = _;
        const THROUGHPUT_CONTROL_REQUEST_RATE_TOO_LARGE: SubStatusCode = _;
        const TOMBSTONE_RECORDS_NOT_FOUND: SubStatusCode = _;
        const TOO_MANY_TENTATIVE_WRITES_TO_SATELLITE_REGION: SubStatusCode = _;
        const TOO_MANY_THROUGHPUT_BUCKET_UPDATES: SubStatusCode = _;
        const TRANSACTION_ALREADY_ACTIVE: SubStatusCode = _;
        const TRANSACTION_LIMIT_EXCEEDED: SubStatusCode = _;
        const TRANSIT_TIMEOUT: SubStatusCode = _;
        const TRANSPORT_BODY_READ_FAILED: SubStatusCode = _;
        const TRANSPORT_CONNECTION_FAILED: SubStatusCode = _;
        const TRANSPORT_DNS_FAILED: SubStatusCode = _;
        const TRANSPORT_GENERATED_503: SubStatusCode = _;
        const TRANSPORT_HTTP2_INCOMPATIBLE: SubStatusCode = _;
        const TRANSPORT_IO_FAILED: SubStatusCode = _;
        const UNDEFINED_DEFAULT_IDENTITY: SubStatusCode = _;
        const UNEXPECTED_THROTTLE: SubStatusCode = _;
        const UNIQUE_INDEX_CONFLICT: SubStatusCode = _;
        const UNIQUE_INDEX_RE_INDEX_IN_PROGRESS: SubStatusCode = _;
        const UNKNOWN: SubStatusCode = _;
        const VALUE_DOES_NOT_MATCH_EXPECTED_BOUND: SubStatusCode = _;
        const WRITE_FORBIDDEN: SubStatusCode = _;
        const XP_COMPOSITE_REPLICATOR: SubStatusCode = _;
        pub fn from_header_value(s: &str) -> Option<Self>;
        pub fn name(&self, status_code: Option<StatusCode>) -> Option<&'static str>;
        pub const fn new(code: u16) -> Self;
        pub const fn value(&self) -> u16;
    }
    impl Debug for SubStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Default for SubStatusCode {
        fn default() -> Self;
    }
    impl Display for SubStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<SubStatusCode> for u16 {
        fn from(code: SubStatusCode) -> Self;
    }
    impl From<u16> for SubStatusCode {
        fn from(value: u16) -> Self;
    }
    pub type Result<T> = std::result::Result<T, CosmosError>;
}
#[cfg(feature = "fault_injection")]
pub mod fault_injection {
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct CustomResponse {
    }
    impl CustomResponse {
        pub fn body(&self) -> &[u8];
        pub fn headers(&self) -> &Headers;
        pub fn status_code(&self) -> StatusCode;
    }
    #[non_exhaustive]
    pub struct CustomResponseBuilder {
    }
    impl CustomResponseBuilder {
        pub fn build(self) -> CustomResponse;
        pub fn new(status_code: StatusCode) -> Self;
        pub fn with_body<impl Into<Vec<u8>>: Into<Vec<u8>>>(self, body: impl Into<Vec<u8>>) -> Self;
        pub fn with_header<impl Into<HeaderName>: Into<HeaderName>, impl Into<HeaderValue>: Into<HeaderValue>>(self, name: impl Into<HeaderName>, value: impl Into<HeaderValue>) -> Self;
        pub fn with_sub_status(self, code: u16) -> Self;
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct FaultClient {
    }
    impl TransportClient for FaultClient {
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn send(&self, request: &HttpRequest) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<HttpResponse, TransportError>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct FaultInjectionCondition {
    }
    impl FaultInjectionCondition {
        pub fn container_id(&self) -> Option<&str>;
        pub fn operation_type(&self) -> Option<FaultOperationType>;
        pub fn region(&self) -> Option<&Region>;
        pub fn transport_kind(&self) -> Option<TransportKind>;
    }
    #[derive(Default)]
    #[non_exhaustive]
    pub struct FaultInjectionConditionBuilder {
    }
    impl FaultInjectionConditionBuilder {
        pub fn build(self) -> FaultInjectionCondition;
        pub fn new() -> Self;
        pub fn with_container_id<impl Into<String>: Into<String>>(self, container_id: impl Into<String>) -> Self;
        pub fn with_operation_type(self, operation_type: FaultOperationType) -> Self;
        pub fn with_region(self, region: Region) -> Self;
        pub fn with_transport_kind(self, transport_kind: TransportKind) -> Self;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct FaultInjectionResult {
    }
    impl FaultInjectionResult {
        pub fn custom_response(&self) -> Option<&CustomResponse>;
        pub fn delay(&self) -> Option<Duration>;
        pub fn error_type(&self) -> Option<FaultInjectionErrorType>;
        pub fn probability(&self) -> f32;
    }
    #[non_exhaustive]
    pub struct FaultInjectionResultBuilder {
    }
    impl FaultInjectionResultBuilder {
        pub fn build(self) -> FaultInjectionResult;
        pub fn new() -> Self;
        pub fn with_custom_response(self, response: CustomResponse) -> Self;
        pub fn with_delay(self, delay: Duration) -> Self;
        pub fn with_error(self, error_type: FaultInjectionErrorType) -> Self;
        pub fn with_probability(self, probability: f32) -> Self;
    }
    impl Default for FaultInjectionResultBuilder {
        fn default() -> Self;
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct FaultInjectionRule {
    }
    impl FaultInjectionRule {
        pub fn condition(&self) -> &FaultInjectionCondition;
        pub fn disable(&self);
        pub fn enable(&self);
        pub fn end_time(&self) -> Option<Instant>;
        pub fn hit_count(&self) -> u32;
        pub fn hit_limit(&self) -> Option<u32>;
        pub fn id(&self) -> &str;
        pub fn is_enabled(&self) -> bool;
        pub fn result(&self) -> &FaultInjectionResult;
        pub fn start_time(&self) -> Option<Instant>;
    }
    #[non_exhaustive]
    pub struct FaultInjectionRuleBuilder {
    }
    impl FaultInjectionRuleBuilder {
        pub fn build(self) -> FaultInjectionRule;
        pub fn new<impl Into<String>: Into<String>>(id: impl Into<String>, result: FaultInjectionResult) -> Self;
        pub fn with_condition(self, condition: FaultInjectionCondition) -> Self;
        pub fn with_end_time(self, end_time: Instant) -> Self;
        pub fn with_hit_limit(self, hit_limit: u32) -> Self;
        pub fn with_result(self, result: FaultInjectionResult) -> Self;
        pub fn with_shared_state(self, enabled: Arc<AtomicBool>, hit_count: Arc<AtomicU32>) -> Self;
        pub fn with_start_time(self, start_time: Instant) -> Self;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum FaultInjectionErrorType {
        InternalServerError,
        TooManyRequests,
        RetryWith,
        ReadSessionNotAvailable,
        Timeout,
        ServiceUnavailable,
        PartitionIsGone,
        WriteForbidden,
        DatabaseAccountNotFound,
        ConnectionError,
        ResponseTimeout,
        ResponseTimeoutAfterService,
    }
    impl Display for FaultInjectionErrorType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl FromStr for FaultInjectionErrorType {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub enum FaultInjectionEvaluation {
        Applied { rule_id: String },
        ProbabilityMiss { rule_id: String, probability: f32 },
        Disabled { rule_id: String },
        BeforeStartTime { rule_id: String },
        AfterEndTime { rule_id: String },
        HitLimitExhausted { rule_id: String, hit_count: u32, hit_limit: u32 },
        OperationMismatch { rule_id: String },
        RegionMismatch { rule_id: String },
        ContainerMismatch { rule_id: String },
        TransportKindMismatch { rule_id: String },
        Superseded { rule_id: String },
    }
    impl FaultInjectionEvaluation {
        pub fn rule_id(&self) -> &str;
        pub fn was_applied(&self) -> bool;
    }
    impl Display for FaultInjectionEvaluation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl Eq for FaultInjectionEvaluation {
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum FaultOperationType {
        ReadItem,
        QueryItem,
        CreateItem,
        UpsertItem,
        ReplaceItem,
        DeleteItem,
        PatchItem,
        BatchItem,
        ChangeFeedItem,
        MetadataReadContainer,
        MetadataReadDatabaseAccount,
        MetadataQueryPlan,
        MetadataPartitionKeyRanges,
    }
    impl FaultOperationType {
        pub fn as_str(&self) -> &'static str;
        pub fn from_operation_and_resource(operation_type: &OperationType, resource_type: &ResourceType) -> Option<Self>;
    }
    impl Display for FaultOperationType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl FromStr for FaultOperationType {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
}
#[cfg(feature = "__internal_in_memory_emulator")]
pub mod in_memory_emulator {
    #[derive(Clone, Debug)]
    pub struct ContainerConfig {
    }
    impl ContainerConfig {
        pub fn build(self) -> crate::error::Result<Self>;
        pub fn new() -> Self;
        pub fn partition_count(&self) -> u32;
        pub fn partition_key_range_page_size(&self) -> Option<u32>;
        pub fn provisioned_throughput_ru(&self) -> Option<u32>;
        pub fn with_partition_count(self, count: u32) -> Self;
        pub fn with_partition_key_range_page_size(self, page_size: u32) -> Self;
        pub fn with_throughput(self, ru_per_second: u32) -> Self;
    }
    impl Default for ContainerConfig {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Eq)]
    pub struct EffectivePartitionKey(/* private fields */);
    impl EffectivePartitionKey {
        const MAX: Self = _;
        const MIN: Self = _;
        pub fn to_hex(&self) -> String;
    }
    impl Display for EffectivePartitionKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<&str> for EffectivePartitionKey {
        fn from(s: &str) -> Self;
    }
    impl From<String> for EffectivePartitionKey {
        fn from(s: String) -> Self;
    }
    impl Hash for EffectivePartitionKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H);
    }
    impl Ord for EffectivePartitionKey {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering;
    }
    impl PartialEq<&str> for EffectivePartitionKey {
        fn eq(&self, other: &&str) -> bool;
    }
    impl PartialEq<str> for EffectivePartitionKey {
        fn eq(&self, other: &str) -> bool;
    }
    impl PartialEq for EffectivePartitionKey {
        fn eq(&self, other: &Self) -> bool;
    }
    impl PartialOrd for EffectivePartitionKey {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>;
    }
    impl Serialize for EffectivePartitionKey {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    impl<'de> Deserialize<'de> for EffectivePartitionKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    pub struct EmulatorStore {
    }
    impl EmulatorStore {
        pub fn add_region(&self, region: VirtualRegion, seeding: SeedingPolicy) -> crate::error::Result<()>;
        pub fn announce_failover(&self, region_name: &str) -> crate::error::Result<()>;
        pub fn begin_failover(&self, region_name: &str) -> crate::error::Result<()>;
        pub fn begin_region_removal(&self, region_name: &str) -> crate::error::Result<()>;
        pub fn cancel_region_removal(&self, region_name: &str) -> crate::error::Result<()>;
        pub fn complete_failover(&self);
        pub fn remove_region(&self, region_name: &str) -> crate::error::Result<()>;
        pub fn restore_region_write(&self, region_name: &str) -> crate::error::Result<()>;
        pub fn revoke_region_write(&self, region_name: &str) -> crate::error::Result<()>;
        pub fn set_failover_priorities(&self, order: &[&str]) -> crate::error::Result<()>;
        pub fn set_region_offline(&self, region_name: &str) -> crate::error::Result<()>;
        pub fn set_region_online(&self, region_name: &str) -> crate::error::Result<()>;
        pub fn set_write_mode(&self, mode: WriteMode);
        pub fn set_write_region(&self, region_name: &str) -> crate::error::Result<()>;
    }
    impl Debug for EmulatorStore {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    pub struct InMemoryEmulatorHttpClient {
    }
    impl InMemoryEmulatorHttpClient {
        pub fn new(config: VirtualAccountConfig) -> Self;
        pub fn runtime_builder(self: &Arc<Self>) -> crate::driver::CosmosDriverRuntimeBuilder;
        #[cfg(feature = "fault_injection")]
        pub fn runtime_builder_with_fault_rules(self: &Arc<Self>, rules: Vec<Arc<crate::fault_injection::FaultInjectionRule>>) -> crate::driver::CosmosDriverRuntimeBuilder;
        pub fn store(&self) -> Arc<EmulatorStore>;
        pub fn with_request_observer(self, observer: Arc<dyn RequestObserver>) -> Self;
    }
    impl InMemoryEmulatorHttpClient {
        pub async fn execute_request(&self, request: &Request) -> crate::error::Result<AsyncRawResponse>;
    }
    impl Debug for InMemoryEmulatorHttpClient {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[derive(Clone)]
    pub struct ReplicationConfig {
    }
    impl ReplicationConfig {
        pub fn fixed(delay: Duration) -> Self;
        pub fn immediate() -> Self;
        pub fn is_immediate(&self) -> bool;
        pub fn max_buffered_replications(&self) -> usize;
        pub fn max_delay(&self) -> Duration;
        pub fn min_delay(&self) -> Duration;
        pub fn range(min: Duration, max: Duration) -> crate::error::Result<Self>;
        pub fn sample_delay(&self) -> Duration;
        pub fn with_jitter_seed(self, seed: u64) -> Self;
        pub fn with_max_buffered_replications(self, max: usize) -> Self;
        pub fn with_replication_delay_fn(self, f: std::sync::Arc<dyn Fn() -> std::time::Duration + Send + Sync>) -> Self;
    }
    impl Debug for ReplicationConfig {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl Default for ReplicationConfig {
        fn default() -> Self;
    }
    #[derive(Clone, Debug)]
    pub struct RequestUnitChargingModel {
        pub read_base_ru: f64,
        pub create_base_ru: f64,
        pub write_multiplier: f64,
        pub indexing_ru_per_property: f64,
    }
    impl RequestUnitChargingModel {
        pub fn compute_create_ru(&self, doc_size: usize, num_properties: usize) -> f64;
        pub fn compute_read_ru(&self, doc_size: usize) -> f64;
        pub fn compute_replace_or_delete_ru(&self, doc_size: usize, num_properties: usize) -> f64;
        pub fn count_properties(body: &serde_json::Value) -> usize;
    }
    impl Default for RequestUnitChargingModel {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ResolvedRegion {
        pub name: String,
        pub status: RegionStatus,
    }
    #[derive(Clone, Debug)]
    pub struct TopologySnapshot {
        pub active: Vec<VirtualRegion>,
        pub write_mode: WriteMode,
        pub write_region: String,
        pub offline: std::collections::HashSet<String>,
        pub adding: std::collections::HashSet<String>,
        pub failing_over_from: Option<String>,
        pub next_write_region: Option<String>,
    }
    impl TopologySnapshot {
        pub fn advertised(&self) -> Vec<&VirtualRegion>;
        pub fn writable(&self, allow_multiple_write_locations: bool) -> Vec<&VirtualRegion>;
    }
    #[derive(Clone, Debug)]
    pub struct VirtualAccountConfig {
    }
    impl VirtualAccountConfig {
        pub fn active_region_names(&self) -> Vec<String>;
        pub fn active_regions(&self) -> Vec<VirtualRegion>;
        pub fn consistency(&self) -> ConsistencyLevel;
        pub fn is_write_region(&self, region_name: &str) -> bool;
        pub fn new(regions: Vec<VirtualRegion>) -> crate::error::Result<Self>;
        pub fn per_partition_failover_enabled(&self) -> bool;
        pub fn region_for_url(&self, url: &Url) -> Option<ResolvedRegion>;
        pub fn region_id_for(&self, region_name: &str) -> u64;
        pub fn replication(&self) -> &ReplicationConfig;
        pub fn replication_for(&self, source: &str, target: &str) -> &ReplicationConfig;
        pub fn ru_model(&self) -> &RequestUnitChargingModel;
        pub fn set_per_partition_failover(&self, enabled: bool);
        pub fn throttling_enabled(&self) -> bool;
        pub fn topology_snapshot(&self) -> TopologySnapshot;
        pub fn with_consistency(self, level: ConsistencyLevel) -> Self;
        pub fn with_per_partition_failover(self, enabled: bool) -> Self;
        pub fn with_replication_config(self, config: ReplicationConfig) -> Self;
        pub fn with_replication_override(self, source: &str, target: &str, config: ReplicationConfig) -> crate::error::Result<Self>;
        pub fn with_ru_model(self, model: RequestUnitChargingModel) -> Self;
        pub fn with_throttling_enabled(self, enabled: bool) -> Self;
        pub fn with_write_mode(self, mode: WriteMode) -> Self;
        pub fn write_mode(&self) -> WriteMode;
        pub fn write_region_name(&self) -> String;
    }
    #[derive(Clone, Debug)]
    pub struct VirtualRegion {
    }
    impl VirtualRegion {
        pub fn gateway_url(&self) -> &Url;
        pub fn name(&self) -> &str;
        pub fn new(name: &str, gateway_url: Url) -> Self;
        pub fn region_id(&self) -> u64;
        pub fn with_region_id(self, id: u64) -> Self;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ConsistencyLevel {
        Strong,
        BoundedStaleness,
        Session,
        ConsistentPrefix,
        Eventual,
    }
    impl ConsistencyLevel {
        pub fn as_str(&self) -> &str;
        pub fn is_session(&self) -> bool;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum RegionStatus {
        Active,
        Draining,
        Offline,
        Retired,
    }
    impl RegionStatus {
        pub fn is_unavailable(&self) -> bool;
        pub fn is_unreachable(&self) -> bool;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub enum SeedingPolicy {
        #[default]
        Immediate,
        Delayed(std::time::Duration),
        HiddenUntilReady(std::time::Duration),
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum WriteMode {
        Single,
        Multi,
    }
    pub trait RequestObserver: Debug + Send + Sync {
        fn on_request(&self, request: &Request);
    }
    pub const DEFAULT_MAX_BUFFERED_REPLICATIONS: usize = 10_000;
}
pub mod models {
    pub use azure_data_cosmos_driver::models::effective_partition_key::EffectivePartitionKey;
    pub fn is_database_rid(rid: &str) -> bool;
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct AccountReference(/* private fields */);
    impl AccountReference {
        pub fn auth(&self) -> &Credential;
        pub fn backup_endpoints(&self) -> &[Url];
        pub fn builder(endpoint: Url) -> AccountReferenceBuilder;
        pub fn endpoint(&self) -> &Url;
        pub fn with_backup_endpoints(self, endpoints: Vec<Url>) -> Self;
        pub fn with_credential(endpoint: Url, credential: Arc<dyn TokenCredential>) -> Self;
        pub fn with_master_key<impl Into<Secret>: Into<Secret>>(endpoint: Url, key: impl Into<Secret>) -> Self;
    }
    impl Eq for AccountReference {
    }
    impl From<AccountReference> for CosmosResourceReference {
        fn from(account: AccountReference) -> Self;
    }
    impl Hash for AccountReference {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H);
    }
    impl PartialEq for AccountReference {
        fn eq(&self, other: &Self) -> bool;
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct AccountReferenceBuilder {
    }
    impl AccountReferenceBuilder {
        pub fn auth(self, credential: Credential) -> Self;
        pub fn build(self) -> crate::error::Result<AccountReference>;
        pub fn credential(self, credential: Arc<dyn TokenCredential>) -> Self;
        pub fn endpoint(self, endpoint: Url) -> Self;
        pub fn master_key<impl Into<Secret>: Into<Secret>>(self, key: impl Into<Secret>) -> Self;
        pub fn new(endpoint: Url) -> Self;
        pub fn with_backup_endpoints(self, endpoints: Vec<Url>) -> Self;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(transparent)]
    pub struct ActivityId(/* private fields */);
    impl ActivityId {
        pub fn as_str(&self) -> &str;
        pub const fn from_static(value: &'static str) -> Self;
        pub fn from_string(value: String) -> Self;
        pub fn new_uuid() -> Self;
    }
    impl AsRef<str> for ActivityId {
        fn as_ref(&self) -> &str;
    }
    impl Display for ActivityId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl From<&str> for ActivityId {
        fn from(value: &str) -> Self;
    }
    impl From<String> for ActivityId {
        fn from(value: String) -> Self;
    }
    impl FromStr for ActivityId {
        type Err = Infallible;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[derive(Clone, Debug, Default, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct AutoscaleAutoUpgradePolicy {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub throughput_policy: Option<AutoscaleThroughputPolicy>,
    }
    #[derive(Clone, Debug, Default, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct AutoscaleThroughputPolicy {
        pub increment_percent: usize,
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct ConnectionString {
    }
    impl ConnectionString {
        pub fn account_endpoint(&self) -> &str;
        pub fn account_key(&self) -> &Secret;
    }
    impl FromStr for ConnectionString {
        type Err = CosmosError;
        fn from_str(connection_string: &str) -> Result<Self, <Self as >::Err>;
    }
    impl TryFrom<&Secret> for ConnectionString {
        type Error = CosmosError;
        fn try_from(secret: &Secret) -> Result<Self, <Self as >::Error>;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct ContainerReference(/* private fields */);
    impl ContainerReference {
        pub fn account(&self) -> &AccountReference;
        pub fn base_path(&self) -> &str;
        pub fn database_name(&self) -> Option<&str>;
        pub fn database_rid(&self) -> &str;
        pub fn is_by_rid(&self) -> bool;
        pub fn name(&self) -> &str;
        pub fn name_based_path(&self) -> Option<&str>;
        pub fn partition_key_definition(&self) -> &crate::models::PartitionKeyDefinition;
        pub fn rid(&self) -> &str;
        pub fn rid_based_path(&self) -> &str;
    }
    impl Eq for ContainerReference {
    }
    impl Hash for ContainerReference {
        fn hash<H: Hasher>(&self, state: &mut H);
    }
    impl PartialEq for ContainerReference {
        fn eq(&self, other: &Self) -> bool;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ContinuationToken(/* private fields */);
    impl ContinuationToken {
        pub fn as_str(&self) -> &str;
        pub fn from_string(token: String) -> Self;
    }
    impl Serialize for ContinuationToken {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>;
    }
    impl<'de> Deserialize<'de> for ContinuationToken {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, <D as >::Error>;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct CosmosOperation {
    }
    impl CosmosOperation {
        pub fn allows_ambiguous_outcome_retry(&self) -> bool;
        pub fn batch(container: ContainerReference, partition_key: PartitionKey) -> Self;
        pub fn body(&self) -> Option<&[u8]>;
        pub fn change_feed(container: ContainerReference, target: Option<FeedRange>) -> Self;
        pub fn change_feed_all_versions_and_deletes(container: ContainerReference, target: Option<FeedRange>) -> Self;
        pub fn change_feed_start(&self) -> Option<&ChangeFeedStartFrom>;
        pub fn container(&self) -> Option<&ContainerReference>;
        pub fn create_container(database: DatabaseReference) -> Self;
        pub fn create_database(account: AccountReference) -> Self;
        pub fn create_item(item: ItemReference) -> Self;
        pub fn db_operation_name(&self) -> Option<&'static str>;
        pub fn delete_container(container: ContainerReference) -> Self;
        pub fn delete_database(database: DatabaseReference) -> Self;
        pub fn delete_item(item: ItemReference) -> Self;
        #[cfg(feature = "preview_dtx")]
        pub fn distributed_transaction(account: AccountReference, transaction_type: crate::models::DistributedTransactionType) -> Self;
        pub fn is_change_feed(&self) -> bool;
        pub fn is_idempotent(&self) -> bool;
        pub fn is_patch_sub_operation(&self) -> bool;
        pub fn is_read_only(&self) -> bool;
        pub fn is_trivial(&self) -> bool;
        pub fn operation_type(&self) -> OperationType;
        pub fn partition_key(&self) -> Option<&PartitionKey>;
        pub fn patch_item(item: ItemReference) -> Self;
        pub fn patch_max_attempts(&self) -> Option<std::num::NonZeroU8>;
        pub fn patch_tracking_capacity(&self) -> Option<std::num::NonZeroU16>;
        pub fn patch_tracking_id(&self) -> Option<crate::models::PatchTrackingId>;
        pub fn patch_tracking_retention_seconds(&self) -> Option<std::num::NonZeroU32>;
        pub fn precondition(&self) -> Option<&Precondition>;
        pub fn query_containers(database: DatabaseReference) -> Self;
        pub fn query_databases(account: AccountReference) -> Self;
        pub fn query_items(container: ContainerReference, target: Option<FeedRange>) -> Self;
        pub fn query_offers(account: AccountReference) -> Self;
        pub fn query_plan(container: ContainerReference, supported_query_features: Cow<'static, str>) -> Self;
        pub fn read_all_containers(database: DatabaseReference) -> Self;
        pub fn read_all_databases(account: AccountReference) -> Self;
        pub fn read_all_items(container: ContainerReference, partition_key: PartitionKey) -> Self;
        pub fn read_all_items_cross_partition(container: ContainerReference) -> Self;
        pub fn read_container(container: ContainerReference) -> Self;
        pub fn read_container_by_name<impl Into<std::borrow::Cow<'static, str>>: Into<std::borrow::Cow<'static, str>>>(database: DatabaseReference, container_name: impl Into<std::borrow::Cow<'static, str>>) -> Self;
        pub fn read_container_by_rid<impl Into<std::borrow::Cow<'static, str>>: Into<std::borrow::Cow<'static, str>>, impl Into<std::borrow::Cow<'static, str>>: Into<std::borrow::Cow<'static, str>>>(account: AccountReference, db_rid: impl Into<std::borrow::Cow<'static, str>>, container_rid: impl Into<std::borrow::Cow<'static, str>>) -> Self;
        pub fn read_database(database: DatabaseReference) -> Self;
        pub fn read_item(item: ItemReference) -> Self;
        pub fn read_offer<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(account: AccountReference, offer_id: impl Into<Cow<'static, str>>) -> Self;
        pub fn replace_container(container: ContainerReference) -> Self;
        pub fn replace_item(item: ItemReference) -> Self;
        pub fn replace_offer<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(account: AccountReference, offer_id: impl Into<Cow<'static, str>>) -> Self;
        pub fn request_headers(&self) -> &CosmosRequestHeaders;
        pub fn resource_type(&self) -> ResourceType;
        pub fn target(&self) -> Option<&FeedRange>;
        pub fn upsert_item(item: ItemReference) -> Self;
        pub fn with_activity_id(self, activity_id: crate::models::ActivityId) -> Self;
        pub fn with_body(self, body: Vec<u8>) -> Self;
        pub fn with_change_feed_start(self, start_from: ChangeFeedStartFrom) -> Self;
        pub fn with_if_modified_since(self, value: String) -> Self;
        pub fn with_max_item_count(self, max_item_count: crate::models::MaxItemCountHint) -> Self;
        pub fn with_patch_max_attempts(self, max_attempts: std::num::NonZeroU8) -> Self;
        pub fn with_patch_tracking_capacity(self, capacity: std::num::NonZeroU16) -> Self;
        pub fn with_patch_tracking_id(self, tracking_id: crate::models::PatchTrackingId) -> Self;
        pub fn with_patch_tracking_retention_seconds(self, retention_seconds: std::num::NonZeroU32) -> Self;
        pub fn with_populate_index_metrics(self, enabled: bool) -> Self;
        pub fn with_populate_query_metrics(self, enabled: bool) -> Self;
        pub fn with_precondition(self, precondition: Precondition) -> Self;
        pub fn with_request_headers(self, headers: CosmosRequestHeaders) -> Self;
        pub fn with_session_token<impl Into<crate::models::SessionToken>: Into<crate::models::SessionToken>>(self, session_token: impl Into<crate::models::SessionToken>) -> Self;
        pub fn with_supported_serialization_formats<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, formats: impl Into<Cow<'static, str>>) -> Self;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct CosmosRequestHeaders {
        pub activity_id: Option<crate::models::ActivityId>,
        pub session_token: Option<crate::models::SessionToken>,
        pub precondition: Option<crate::models::Precondition>,
        pub offer_throughput: Option<usize>,
        pub offer_autopilot_settings: Option<OfferAutoscaleSettings>,
        pub max_item_count: Option<MaxItemCountHint>,
        pub incremental_feed: bool,
        pub full_fidelity_feed: bool,
        pub changefeed_wire_format_version: bool,
        pub if_modified_since: Option<String>,
        pub populate_index_metrics: Option<bool>,
        pub populate_query_metrics: Option<bool>,
        pub enable_cross_partition_query: bool,
        pub supported_query_features: Option<std::borrow::Cow<'static, str>>,
        pub supported_serialization_formats: Option<std::borrow::Cow<'static, str>>,
    }
    impl CosmosRequestHeaders {
        pub fn new() -> Self;
    }
    #[derive(Clone, Debug)]
    pub struct CosmosResourceReference {
    }
    impl CosmosResourceReference {
        pub fn account(&self) -> &AccountReference;
        pub fn container(&self) -> Option<&ContainerReference>;
        pub fn into_feed_reference(self) -> Self;
        pub fn link_for_signing(&self) -> String;
        pub fn request_path(&self) -> String;
        pub fn resource_type(&self) -> ResourceType;
        pub fn with_name(self, name: Cow<'static, str>) -> Self;
        pub fn with_resource_type(self, resource_type: ResourceType) -> Self;
        pub fn with_rid(self, rid: Cow<'static, str>) -> Self;
    }
    impl Display for CosmosResourceReference {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl From<ContainerReference> for CosmosResourceReference {
        fn from(container: ContainerReference) -> Self;
    }
    impl From<DatabaseReference> for CosmosResourceReference {
        fn from(database: DatabaseReference) -> Self;
    }
    impl From<ItemReference> for CosmosResourceReference {
        fn from(item: ItemReference) -> Self;
    }
    impl From<StoredProcedureReference> for CosmosResourceReference {
        fn from(sp: StoredProcedureReference) -> Self;
    }
    impl From<TriggerReference> for CosmosResourceReference {
        fn from(trigger: TriggerReference) -> Self;
    }
    impl From<UdfReference> for CosmosResourceReference {
        fn from(udf: UdfReference) -> Self;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct CosmosResponse {
    }
    impl CosmosResponse {
        pub fn body(&self) -> &ResponseBody;
        pub fn diagnostics(&self) -> Arc<DiagnosticsContext>;
        pub fn diagnostics_ref(&self) -> &Arc<DiagnosticsContext>;
        pub fn headers(&self) -> &CosmosResponseHeaders;
        pub fn into_body(self) -> ResponseBody;
        pub fn patch_tracking_id(&self) -> Option<PatchTrackingId>;
        pub fn serving_region(&self) -> Option<Region>;
        pub fn status(&self) -> CosmosStatus;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct CosmosResponseHeaders {
        pub activity_id: Option<crate::models::ActivityId>,
        pub request_charge: Option<crate::models::RequestCharge>,
        pub session_token: Option<crate::models::SessionToken>,
        pub etag: Option<azure_core::http::Etag>,
        pub continuation: Option<String>,
        pub item_count: Option<u32>,
        pub substatus: Option<crate::models::SubStatusCode>,
        pub index_metrics: Option<String>,
        pub query_metrics: Option<String>,
        pub server_duration_ms: Option<f64>,
        pub lsn: Option<u64>,
        pub item_lsn: Option<u64>,
        pub offer_replace_pending: Option<bool>,
        pub retry_after_ms: Option<u64>,
        pub correlated_activity_id: Option<String>,
        pub transport_request_id: Option<u32>,
        pub global_committed_lsn: Option<i64>,
        pub quorum_acked_lsn: Option<i64>,
        pub quorum_acked_local_lsn: Option<i64>,
        pub local_lsn: Option<u64>,
        pub item_local_lsn: Option<u64>,
        pub number_of_read_regions: Option<u32>,
        pub last_state_change_utc: Option<String>,
        pub gateway_version: Option<String>,
        pub service_version: Option<String>,
        pub resource_quota: Option<String>,
        pub resource_usage: Option<String>,
        pub has_tentative_writes: Option<bool>,
        pub partition_key_range_id: Option<String>,
        pub internal_partition_id: Option<String>,
        pub log_results: Option<String>,
        pub collection_index_transformation_progress: Option<i64>,
        pub collection_lazy_indexing_progress: Option<i64>,
        #[cfg(feature = "preview_dtx")]
        pub distributed_transaction_idempotency_token: Option<uuid::Uuid>,
    }
    impl CosmosResponseHeaders {
        pub fn from_headers(headers: &Headers) -> Self;
        pub fn new() -> Self;
        pub fn to_raw_headers(&self) -> Headers;
    }
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct CosmosStatus {
    }
    impl CosmosStatus {
        const AUTHENTICATION_TOKEN_ACQUISITION_FAILED: CosmosStatus = _;
        const CLIENT_BAD_REQUEST: CosmosStatus = _;
        const CLIENT_BUILD_RESPONSE_INVOKED_ON_FAILURE: CosmosStatus = _;
        const CLIENT_CHANGE_FEED_PIPELINE_UNEXPECTEDLY_DRAINED: CosmosStatus = _;
        const CLIENT_COMPUTE_RANGE_INVOKED_WITH_EMPTY_PARTITION_KEY: CosmosStatus = _;
        const CLIENT_CONNECTION_STRING_EMPTY: CosmosStatus = _;
        const CLIENT_CONNECTION_STRING_MALFORMED_PART: CosmosStatus = _;
        const CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_ENDPOINT: CosmosStatus = _;
        const CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_KEY: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_AFTER_TRANSCODE_FAILURE: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_FETCH_IN_FLIGHT: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_NON_QUERY_OPERATION: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH: CosmosStatus = _;
        const CLIENT_CONTINUATION_TOKEN_UNEXPECTED_NESTED_SHAPE: CosmosStatus = _;
        const CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED: CosmosStatus = _;
        const CLIENT_CROSS_PARTITION_QUERY_REQUIRES_CONTAINER_REF: CosmosStatus = _;
        const CLIENT_DISTINCT_CANNOT_FORWARD_SPLIT: CosmosStatus = _;
        const CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED: CosmosStatus = _;
        const CLIENT_DISTINCT_VALUE_TOO_DEEPLY_NESTED: CosmosStatus = _;
        const CLIENT_DRIVER_NOT_INITIALIZED: CosmosStatus = _;
        const CLIENT_DUPLICATE_FAULT_INJECTION_RULE_ID: CosmosStatus = _;
        const CLIENT_FEED_RANGE_REQUIRES_FANOUT_PIPELINE: CosmosStatus = _;
        const CLIENT_GENERATED_401: CosmosStatus = _;
        const CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED: CosmosStatus = _;
        const CLIENT_IMDS_HTTP_CLIENT_CONSTRUCTION_FAILED: CosmosStatus = _;
        const CLIENT_IMDS_REQWEST_FEATURE_REQUIRED: CosmosStatus = _;
        const CLIENT_INVALID_ACCOUNT_ENDPOINT_URL: CosmosStatus = _;
        const CLIENT_INVALID_RESOURCE_ID: CosmosStatus = _;
        const CLIENT_INVALID_URL: CosmosStatus = _;
        const CLIENT_MIXED_NAME_RID_ADDRESSING: CosmosStatus = _;
        const CLIENT_NON_MULTIHASH_PARTITION_KEY_ARITY_MISMATCH: CosmosStatus = _;
        const CLIENT_NON_STREAMING_ORDER_BY_CONTINUATION_UNSUPPORTED: CosmosStatus = _;
        const CLIENT_NON_STREAMING_ORDER_BY_REQUIRES_FINITE_WINDOW: CosmosStatus = _;
        const CLIENT_NON_STREAMING_ORDER_BY_WINDOW_TOO_LARGE: CosmosStatus = _;
        const CLIENT_NO_OVERLAPPING_FEED_RANGES_FOR_SESSION_TOKEN: CosmosStatus = _;
        const CLIENT_NO_THROUGHPUT_OFFER_FOR_RESOURCE: CosmosStatus = _;
        const CLIENT_OPAQUE_TOKEN_INVALID_FOR_CROSS_PARTITION_QUERY: CosmosStatus = _;
        const CLIENT_ORDER_BY_COMPLEX_VALUE_UNSUPPORTED: CosmosStatus = _;
        const CLIENT_PARTITION_KEY_EMPTY: CosmosStatus = _;
        const CLIENT_PARTITION_KEY_RANGE_CACHE_REQUIRED: CosmosStatus = _;
        const CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS: CosmosStatus = _;
        const CLIENT_PREFIX_PARTITION_KEY_REQUIRES_MULTIHASH: CosmosStatus = _;
        const CLIENT_QUERY_PLAN_COMPLEX_PROJECTION_UNSUPPORTED: CosmosStatus = _;
        const CLIENT_QUERY_PLAN_INVALID_TOP_OFFSET_LIMIT: CosmosStatus = _;
        const CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES: CosmosStatus = _;
        const CLIENT_QUERY_PLAN_RANGE_NOT_COVERED_BY_TOPOLOGY: CosmosStatus = _;
        const CLIENT_QUERY_REWRITE_BODY_INVALID: CosmosStatus = _;
        const CLIENT_REQUEST_URL_MISSING_HOST: CosmosStatus = _;
        const CLIENT_REQUEST_URL_MISSING_KNOWN_PORT: CosmosStatus = _;
        const CLIENT_REQWEST_FEATURE_REQUIRED: CosmosStatus = _;
        const CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT: CosmosStatus = _;
        const CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE: CosmosStatus = _;
        const CLIENT_SPLIT_RETRIES_EXHAUSTED: CosmosStatus = _;
        const CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID: CosmosStatus = _;
        const CLIENT_THROUGHPUT_CONTROL_GROUP_NOT_REGISTERED: CosmosStatus = _;
        const CLIENT_THROUGHPUT_CONTROL_GROUP_REGISTRATION_FAILED: CosmosStatus = _;
        const CLIENT_THROUGHPUT_POLLER_INCOMPLETE: CosmosStatus = _;
        const CLIENT_TOPOLOGY_PROVIDER_MISSING: CosmosStatus = _;
        const CLIENT_TOPOLOGY_RESOLUTION_FAILED: CosmosStatus = _;
        const CLIENT_UNKNOWN_CONSISTENCY_LEVEL: CosmosStatus = _;
        const CLIENT_UNKNOWN_PRIORITY_LEVEL: CosmosStatus = _;
        const CLIENT_UNSUPPORTED_QUERY_FEATURE: CosmosStatus = _;
        const COMPLETING_PARTITION_MIGRATION: CosmosStatus = _;
        const COMPLETING_SPLIT: CosmosStatus = _;
        const CROSS_PARTITION_QUERY_NOT_SERVABLE: CosmosStatus = _;
        const DATABASE_ACCOUNT_NOT_FOUND: CosmosStatus = _;
        const NAME_CACHE_STALE: CosmosStatus = _;
        const PARTITION_KEY_RANGE_GONE: CosmosStatus = _;
        const READ_SESSION_NOT_AVAILABLE: CosmosStatus = _;
        const RU_BUDGET_EXCEEDED: CosmosStatus = _;
        const SERIALIZATION_REQUEST_BODY_INVALID: CosmosStatus = _;
        const SERIALIZATION_RESPONSE_BODY_INVALID: CosmosStatus = _;
        const SERVICE_ORDER_BY_ENVELOPE_INVALID: CosmosStatus = _;
        const SERVICE_QUERY_PLAN_ORDER_BY_MISSING_REWRITTEN_QUERY: CosmosStatus = _;
        const SERVICE_RETURNED_OBJECT_WITHOUT_RID: CosmosStatus = _;
        const SERVICE_RETURNED_OFFER_WITHOUT_ID: CosmosStatus = _;
        const TRANSPORT_BODY_READ_FAILED: CosmosStatus = _;
        const TRANSPORT_CONNECTION_FAILED: CosmosStatus = _;
        const TRANSPORT_DNS_FAILED: CosmosStatus = _;
        const TRANSPORT_GENERATED_503: CosmosStatus = _;
        const TRANSPORT_HTTP2_INCOMPATIBLE: CosmosStatus = _;
        const TRANSPORT_IO_FAILED: CosmosStatus = _;
        const WRITE_FORBIDDEN: CosmosStatus = _;
        pub fn is_bad_request(&self) -> bool;
        pub fn is_conflict(&self) -> bool;
        pub fn is_database_account_not_found(&self) -> bool;
        pub fn is_forbidden(&self) -> bool;
        pub fn is_gone(&self) -> bool;
        pub fn is_not_found(&self) -> bool;
        pub fn is_partition_key_range_gone(&self) -> bool;
        pub fn is_precondition_failed(&self) -> bool;
        pub fn is_read_session_not_available(&self) -> bool;
        pub fn is_retry_with(&self) -> bool;
        pub fn is_service_unavailable(&self) -> bool;
        pub fn is_success(&self) -> bool;
        pub fn is_throttled(&self) -> bool;
        pub fn is_timeout(&self) -> bool;
        pub fn is_transient(&self) -> bool;
        pub fn is_transport_generated_503(&self) -> bool;
        pub fn is_unauthorized(&self) -> bool;
        pub fn is_write_forbidden(&self) -> bool;
        pub fn name(&self) -> Option<&'static str>;
        pub fn new(status_code: StatusCode) -> Self;
        pub fn status_code(&self) -> StatusCode;
        pub fn sub_status(&self) -> Option<SubStatusCode>;
        pub fn with_sub_status(self, sub_status_code: u16) -> Self;
    }
    impl Debug for CosmosStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Display for CosmosStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<CosmosStatus> for azure_core::http::StatusCode {
        fn from(s: CosmosStatus) -> Self;
    }
    impl From<CosmosStatus> for u16 {
        fn from(s: CosmosStatus) -> Self;
    }
    impl PartialEq<CosmosStatus> for azure_core::http::StatusCode {
        fn eq(&self, other: &CosmosStatus) -> bool;
    }
    impl PartialEq<StatusCode> for CosmosStatus {
        fn eq(&self, other: &StatusCode) -> bool;
    }
    impl Serialize for CosmosStatus {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct DatabaseReference(/* private fields */);
    impl DatabaseReference {
        pub fn account(&self) -> &AccountReference;
        pub fn from_name<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(account: AccountReference, name: impl Into<Cow<'static, str>>) -> Self;
        pub fn from_rid<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(account: AccountReference, rid: impl Into<Cow<'static, str>>) -> Self;
        pub fn is_by_name(&self) -> bool;
        pub fn is_by_rid(&self) -> bool;
        pub fn name(&self) -> Option<&str>;
        pub fn name_based_path(&self) -> Option<String>;
        pub fn rid(&self) -> Option<&str>;
        pub fn rid_based_path(&self) -> Option<String>;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct DistributedTransactionOperation {
        pub kind: DistributedTransactionOperationKind,
        pub target: DistributedTransactionTarget,
        pub resource_body: Option<azure_core::Bytes>,
        pub session_token: Option<crate::models::SessionToken>,
        pub precondition: Option<crate::models::Precondition>,
        pub patch_filter_predicate: Option<std::borrow::Cow<'static, str>>,
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedTransactionOperation {
        pub fn new(kind: DistributedTransactionOperationKind, target: DistributedTransactionTarget) -> Self;
        pub fn with_patch_filter_predicate<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, predicate: impl Into<Cow<'static, str>>) -> Self;
        pub fn with_precondition(self, precondition: Precondition) -> Self;
        pub fn with_resource_body<impl Into<Bytes>: Into<Bytes>>(self, body: impl Into<Bytes>) -> Self;
        pub fn with_session_token<impl Into<SessionToken>: Into<SessionToken>>(self, session_token: impl Into<SessionToken>) -> Self;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct DistributedTransactionOperationResult {
        pub raw_response: serde_json::Map<String, serde_json::Value>,
        pub index: usize,
        pub status_code: azure_core::http::StatusCode,
        pub sub_status_code: Option<crate::models::SubStatusCode>,
        pub etag: Option<azure_core::http::Etag>,
        pub session_token: Option<crate::models::SessionToken>,
        pub partition_key_range_id: Option<String>,
        pub request_charge: Option<crate::models::RequestCharge>,
        pub resource_body: DistributedTransactionResultBody,
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedTransactionOperationResult {
        pub fn is_completed_status_code(&self) -> bool;
        pub fn is_success_status_code(&self) -> bool;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct DistributedTransactionRequest {
        pub transaction_type: DistributedTransactionType,
        pub operations: Vec<DistributedTransactionOperation>,
        pub idempotency_token: uuid::Uuid,
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedTransactionRequest {
        pub fn new(transaction_type: DistributedTransactionType, operations: Vec<DistributedTransactionOperation>) -> Self;
        pub fn serialize_body(&self) -> crate::error::Result<Vec<u8>>;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct DistributedTransactionResponse {
        pub status_code: azure_core::http::StatusCode,
        pub sub_status_code: Option<crate::models::SubStatusCode>,
        pub operation_results: Vec<DistributedTransactionOperationResult>,
        pub idempotency_token: uuid::Uuid,
        pub headers: crate::models::CosmosResponseHeaders,
        pub activity_id: Option<crate::models::ActivityId>,
        pub request_charge: Option<crate::models::RequestCharge>,
        pub retry_after_ms: Option<u64>,
        pub diagnostics: Option<std::sync::Arc<crate::diagnostics::DiagnosticsContext>>,
        pub is_retriable: bool,
        pub diagnostic_string: Option<String>,
        pub error_message: Option<String>,
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedTransactionResponse {
        pub fn from_body(status_code: azure_core::http::StatusCode, sub_status_code: Option<crate::models::SubStatusCode>, body: &[u8], operation_count: usize, idempotency_token: Uuid) -> Self;
        pub fn is_completed_status_code(&self) -> bool;
        pub fn is_empty(&self) -> bool;
        pub fn is_success_status_code(&self) -> bool;
        pub fn len(&self) -> usize;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct DistributedTransactionTarget {
        pub container: crate::models::ContainerReference,
        pub partition_key: crate::models::PartitionKey,
        pub id: std::borrow::Cow<'static, str>,
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedTransactionTarget {
        pub fn new<impl Into<PartitionKey>: Into<PartitionKey>, impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(container: ContainerReference, partition_key: impl Into<PartitionKey>, id: impl Into<Cow<'static, str>>) -> Self;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct FeedRange(/* private fields */);
    impl FeedRange {
        pub fn for_partition(partition_key: PartitionKey, definition: &PartitionKeyDefinition) -> Self;
        pub fn full() -> Self;
        pub fn is_logical_partition(&self) -> bool;
        pub fn is_subset_of(&self, other: &FeedRange) -> bool;
        pub fn max_exclusive(&self) -> &EffectivePartitionKey;
        pub fn min_inclusive(&self) -> &EffectivePartitionKey;
        pub fn new(min_inclusive: EffectivePartitionKey, max_exclusive: EffectivePartitionKey) -> crate::error::Result<Self>;
        pub fn overlaps(&self, other: &FeedRange) -> bool;
    }
    impl Display for FeedRange {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl FromStr for FeedRange {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    impl Serialize for FeedRange {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    impl TryFrom<&PartitionKeyRange> for FeedRange {
        type Error = CosmosError;
        fn try_from(pkr: &PartitionKeyRange) -> Result<Self, <Self as >::Error>;
    }
    impl<'de> Deserialize<'de> for FeedRange {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct ItemReference(/* private fields */);
    impl ItemReference {
        pub fn account(&self) -> &AccountReference;
        pub fn container(&self) -> &ContainerReference;
        pub fn from_name<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(container: &ContainerReference, partition_key: PartitionKey, item_name: impl Into<Cow<'static, str>>) -> Self;
        pub fn from_rid<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(container: &ContainerReference, partition_key: PartitionKey, item_rid: impl Into<Cow<'static, str>>) -> Self;
        pub fn is_by_name(&self) -> bool;
        pub fn is_by_rid(&self) -> bool;
        pub fn name(&self) -> Option<&str>;
        pub fn partition_key(&self) -> &PartitionKey;
        pub fn resource_link(&self) -> &str;
        pub fn rid(&self) -> Option<&str>;
    }
    #[derive(Clone, Debug, Default, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct OfferAutoscaleSettings {
        pub max_throughput: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub auto_upgrade_policy: Option<AutoscaleAutoUpgradePolicy>,
    }
    impl OfferAutoscaleSettings {
        pub fn new(max_throughput: usize) -> Self;
        pub fn with_increment_percent(self, increment_percent: usize) -> Self;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct PartitionKey(/* private fields */);
    impl PartitionKey {
        const NULL: PartitionKeyValue = PartitionKeyValue::NULL;
        const UNDEFINED: PartitionKeyValue = PartitionKeyValue::UNDEFINED;
        pub fn is_empty(&self) -> bool;
        pub fn len(&self) -> usize;
        pub fn values(&self) -> &[PartitionKeyValue];
    }
    impl AsHeaders for PartitionKey {
        type Error = CosmosError;
        type Iter = IntoIter<(HeaderName, HeaderValue)>;
        fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
    }
    impl From<Vec<PartitionKeyValue>> for PartitionKey {
        fn from(values: Vec<PartitionKeyValue>) -> Self;
    }
    impl<T1, T2, T3> From<(T1, T2, T3)> for PartitionKey where T1: Into<PartitionKeyValue>, T2: Into<PartitionKeyValue>, T3: Into<PartitionKeyValue> {
        fn from((v1, v2, v3): (T1, T2, T3)) -> Self;
    }
    impl<T1, T2> From<(T1, T2)> for PartitionKey where T1: Into<PartitionKeyValue>, T2: Into<PartitionKeyValue> {
        fn from((v1, v2): (T1, T2)) -> Self;
    }
    impl<T: Into<PartitionKeyValue>> From<T> for PartitionKey {
        fn from(value: T) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct PartitionKeyDefinition {
    }
    impl PartitionKeyDefinition {
        pub fn is_complete(&self, pk: &PartitionKey) -> bool;
        pub fn kind(&self) -> PartitionKeyKind;
        pub fn new(paths: Vec<Cow<'static, str>>) -> Self;
        pub fn paths(&self) -> &[Cow<'static, str>];
        pub fn version(&self) -> PartitionKeyVersion;
        pub fn with_kind(self, kind: PartitionKeyKind) -> Self;
        pub fn with_version(self, version: PartitionKeyVersion) -> Self;
    }
    impl From<&str> for PartitionKeyDefinition {
        fn from(value: &str) -> Self;
    }
    impl From<String> for PartitionKeyDefinition {
        fn from(value: String) -> Self;
    }
    impl<'de> Deserialize<'de> for PartitionKeyDefinition {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    impl<S1: Into<String>, S2: Into<String>, S3: Into<String>> From<(S1, S2, S3)> for PartitionKeyDefinition {
        fn from(value: (S1, S2, S3)) -> Self;
    }
    impl<S1: Into<String>, S2: Into<String>> From<(S1, S2)> for PartitionKeyDefinition {
        fn from(value: (S1, S2)) -> Self;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct PartitionKeyRangeReference {
    }
    impl PartitionKeyRangeReference {
        pub fn account(&self) -> &AccountReference;
        pub fn container(&self) -> &ContainerReference;
        pub fn from_range_id<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(container: &ContainerReference, range_id: impl Into<Cow<'static, str>>) -> Self;
        pub fn range_id(&self) -> &str;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct PartitionKeyValue(/* private fields */);
    impl PartitionKeyValue {
        const INFINITY: Self = _;
        const NULL: Self = _;
        const UNDEFINED: Self = _;
    }
    impl From<&'static str> for PartitionKeyValue {
        fn from(value: &'static str) -> Self;
    }
    impl From<&String> for PartitionKeyValue {
        fn from(value: &String) -> Self;
    }
    impl From<Cow<'static, str>> for PartitionKeyValue {
        fn from(value: Cow<'static, str>) -> Self;
    }
    impl From<String> for PartitionKeyValue {
        fn from(value: String) -> Self;
    }
    impl From<bool> for PartitionKeyValue {
        fn from(value: bool) -> Self;
    }
    impl From<f32> for PartitionKeyValue {
        fn from(value: f32) -> Self;
    }
    impl From<f64> for PartitionKeyValue {
        fn from(value: f64) -> Self;
    }
    impl From<i16> for PartitionKeyValue {
        fn from(value: i16) -> Self;
    }
    impl From<i32> for PartitionKeyValue {
        fn from(value: i32) -> Self;
    }
    impl From<i64> for PartitionKeyValue {
        fn from(value: i64) -> Self;
    }
    impl From<i8> for PartitionKeyValue {
        fn from(value: i8) -> Self;
    }
    impl From<isize> for PartitionKeyValue {
        fn from(value: isize) -> Self;
    }
    impl From<u16> for PartitionKeyValue {
        fn from(value: u16) -> Self;
    }
    impl From<u32> for PartitionKeyValue {
        fn from(value: u32) -> Self;
    }
    impl From<u64> for PartitionKeyValue {
        fn from(value: u64) -> Self;
    }
    impl From<u8> for PartitionKeyValue {
        fn from(value: u8) -> Self;
    }
    impl From<usize> for PartitionKeyValue {
        fn from(value: usize) -> Self;
    }
    impl<T: Into<PartitionKeyValue>> From<Option<T>> for PartitionKeyValue {
        fn from(value: Option<T>) -> Self;
    }
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct PatchInstructions {
        pub operations: Vec<PatchOperation>,
    }
    impl PatchInstructions {
        pub fn is_retry_safe(&self) -> bool;
        pub fn new() -> Self;
        pub fn with_operation(self, operation: PatchOperation) -> Self;
    }
    impl From<Vec<PatchOperation>> for PatchInstructions {
        fn from(operations: Vec<PatchOperation>) -> Self;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct PatchTrackingId(/* private fields */);
    impl PatchTrackingId {
        pub fn as_uuid(&self) -> Uuid;
        pub fn new() -> Self;
    }
    impl Default for PatchTrackingId {
        fn default() -> Self;
    }
    impl Display for PatchTrackingId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl From<Uuid> for PatchTrackingId {
        fn from(value: Uuid) -> Self;
    }
    impl FromStr for PatchTrackingId {
        type Err = Error;
        fn from_str(value: &str) -> Result<Self, <Self as >::Err>;
    }
    impl Serialize for PatchTrackingId {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    impl<'de> Deserialize<'de> for PatchTrackingId {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(transparent)]
    pub struct RequestCharge(/* private fields */);
    impl RequestCharge {
        pub fn new(value: f64) -> Self;
        pub const fn value(self) -> f64;
    }
    impl Add for RequestCharge {
        type Output = RequestCharge;
        fn add(self, rhs: Self) -> <Self as >::Output;
    }
    impl Display for RequestCharge {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<RequestCharge> for f64 {
        fn from(charge: RequestCharge) -> Self;
    }
    impl From<f64> for RequestCharge {
        fn from(value: f64) -> Self;
    }
    impl Ord for RequestCharge {
        fn cmp(&self, other: &Self) -> Ordering;
    }
    impl PartialOrd for RequestCharge {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
    }
    impl Sum for RequestCharge {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct SessionToken(pub std::borrow::Cow<'static, str>);
    impl SessionToken {
        pub fn as_str(&self) -> &str;
        pub fn merge(&self, other: &Self) -> crate::error::Result<Self>;
        pub fn new<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(value: impl Into<Cow<'static, str>>) -> Self;
    }
    impl AsRef<str> for SessionToken {
        fn as_ref(&self) -> &str;
    }
    impl<T: Into<std::borrow::Cow<'static, str>>> From<T> for SessionToken {
        fn from(value: T) -> Self;
    }
    impl Display for SessionToken {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct SessionTokenSegment {
    }
    impl SessionTokenSegment {
        pub fn global_lsn(&self) -> u64;
        pub fn is_as_recent_as(&self, other: &Self) -> bool;
        pub fn merge_value(&mut self, other: &Self) -> bool;
        pub fn pk_range_id(&self) -> &str;
        pub fn set_pk_range_id<impl Into<String>: Into<String>>(&mut self, id: impl Into<String>);
    }
    impl Display for SessionTokenSegment {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl FromStr for SessionTokenSegment {
        type Err = CosmosError;
        fn from_str(s: &str) -> crate::error::Result<Self>;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct StoredProcedureReference {
    }
    impl StoredProcedureReference {
        pub fn account(&self) -> &AccountReference;
        pub fn container(&self) -> &ContainerReference;
        pub fn from_name<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(container: &ContainerReference, stored_procedure_name: impl Into<Cow<'static, str>>) -> Self;
        pub fn from_rid<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(container: &ContainerReference, stored_procedure_rid: impl Into<Cow<'static, str>>) -> Self;
        pub fn is_by_name(&self) -> bool;
        pub fn is_by_rid(&self) -> bool;
        pub fn name(&self) -> Option<&str>;
        pub fn resource_link(&self) -> &str;
        pub fn rid(&self) -> Option<&str>;
    }
    #[derive(Clone, Copy, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(transparent)]
    pub struct SubStatusCode(/* private fields */);
    impl SubStatusCode {
        const AAD_TOKEN_EXPIRED: SubStatusCode = _;
        const AE_QUEUE_FULL: SubStatusCode = _;
        const ARCHIVAL_PARTITION_NOT_PRESENT: SubStatusCode = _;
        const ARCHIVAL_PARTITION_PENDING_CATCHUP: SubStatusCode = _;
        const ASYNC_READER_WRITER_LOCK: SubStatusCode = _;
        const AUTHENTICATION_TOKEN_ACQUISITION_FAILED: SubStatusCode = _;
        const AUTH_TOKEN_NOT_FOUND_IN_CACHE: SubStatusCode = _;
        const AZURE_BACKUP_VAULT_INCREMENTAL_BACKUP_PAUSED: SubStatusCode = _;
        const AZURE_BACKUP_VAULT_INCREMENTAL_BACKUP_RESTORE_DISABLED: SubStatusCode = _;
        const AZURE_RBAC_ACCESS_DECISION_UNAVAILABLE: SubStatusCode = _;
        const BATCH_RESPONSE_SIZE_EXCEEDED: SubStatusCode = _;
        const BW_TERM_COUNT_LIMIT_EXCEEDED: SubStatusCode = _;
        const BW_TREE_IO_RATE_LIMITER: SubStatusCode = _;
        const BW_TREE_LOG_FULL_BACKPRESSURE: SubStatusCode = _;
        const CANNOT_ACQUIRE_IN_ACCOUNT_RESTORE_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_LOG_STORE_LOAD_BALANCE_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_MASTER_PARTITION_ACCESS_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_OFFER_OWNER_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_PARTITION_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_PKRANGES_LOCK: SubStatusCode = _;
        const CANNOT_ACQUIRE_PKRANGE_LOCK: SubStatusCode = _;
        const CHANNEL_CLOSED: SubStatusCode = _;
        const CHECKPOINT_QUEUE_DEPTH_BACKPRESSURE: SubStatusCode = _;
        const CLIENT_BUILD_RESPONSE_INVOKED_ON_FAILURE: SubStatusCode = _;
        const CLIENT_CHANGE_FEED_PIPELINE_UNEXPECTEDLY_DRAINED: SubStatusCode = _;
        const CLIENT_COMPUTE_RANGE_INVOKED_WITH_EMPTY_PARTITION_KEY: SubStatusCode = _;
        const CLIENT_CONNECTION_STRING_EMPTY: SubStatusCode = _;
        const CLIENT_CONNECTION_STRING_MALFORMED_PART: SubStatusCode = _;
        const CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_ENDPOINT: SubStatusCode = _;
        const CLIENT_CONNECTION_STRING_MISSING_ACCOUNT_KEY: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_AFTER_TRANSCODE_FAILURE: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_FETCH_IN_FLIGHT: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_INVALID_EPK_RANGE: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_NON_QUERY_OPERATION: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_ORDER_BY_STATE_INVALID: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_SAVED_RANGE_UNHONORED: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH: SubStatusCode = _;
        const CLIENT_CONTINUATION_TOKEN_UNEXPECTED_NESTED_SHAPE: SubStatusCode = _;
        const CLIENT_CPU_OVERLOAD: SubStatusCode = _;
        const CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED: SubStatusCode = _;
        const CLIENT_CROSS_PARTITION_QUERY_REQUIRES_CONTAINER_REF: SubStatusCode = _;
        const CLIENT_DISTINCT_CANNOT_FORWARD_SPLIT: SubStatusCode = _;
        const CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED: SubStatusCode = _;
        const CLIENT_DISTINCT_VALUE_TOO_DEEPLY_NESTED: SubStatusCode = _;
        const CLIENT_DRIVER_NOT_INITIALIZED: SubStatusCode = _;
        const CLIENT_DUPLICATE_FAULT_INJECTION_RULE_ID: SubStatusCode = _;
        const CLIENT_FEED_RANGE_REQUIRES_FANOUT_PIPELINE: SubStatusCode = _;
        const CLIENT_FFI_FEED_EXHAUSTED: SubStatusCode = _;
        const CLIENT_FFI_INVALID_HEADER: SubStatusCode = _;
        const CLIENT_FFI_INVALID_OPTION_VALUE: SubStatusCode = _;
        const CLIENT_FFI_INVALID_UTF8: SubStatusCode = _;
        const CLIENT_FFI_NULL_ARGUMENT: SubStatusCode = _;
        const CLIENT_FFI_OPERATION_CANCELLED: SubStatusCode = _;
        const CLIENT_FFI_OPERATION_CONSUMED: SubStatusCode = _;
        const CLIENT_FFI_PANIC: SubStatusCode = _;
        const CLIENT_FFI_PRECONDITION_ALREADY_SET: SubStatusCode = _;
        const CLIENT_FFI_QUEUE_FULL: SubStatusCode = _;
        const CLIENT_FFI_QUEUE_SHUTDOWN: SubStatusCode = _;
        const CLIENT_FFI_RUNTIME_BUILD_FAILED: SubStatusCode = _;
        const CLIENT_FFI_UNSUPPORTED_OPERATION_FOR_MUTATOR: SubStatusCode = _;
        const CLIENT_GENERATED_401: SubStatusCode = _;
        const CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED: SubStatusCode = _;
        const CLIENT_IMDS_HTTP_CLIENT_CONSTRUCTION_FAILED: SubStatusCode = _;
        const CLIENT_IMDS_REQWEST_FEATURE_REQUIRED: SubStatusCode = _;
        const CLIENT_INVALID_ACCOUNT_ENDPOINT_URL: SubStatusCode = _;
        const CLIENT_INVALID_RESOURCE_ID: SubStatusCode = _;
        const CLIENT_INVALID_URL: SubStatusCode = _;
        const CLIENT_MIXED_NAME_RID_ADDRESSING: SubStatusCode = _;
        const CLIENT_NON_MULTIHASH_PARTITION_KEY_ARITY_MISMATCH: SubStatusCode = _;
        const CLIENT_NON_STREAMING_ORDER_BY_CONTINUATION_UNSUPPORTED: SubStatusCode = _;
        const CLIENT_NON_STREAMING_ORDER_BY_REQUIRES_FINITE_WINDOW: SubStatusCode = _;
        const CLIENT_NON_STREAMING_ORDER_BY_WINDOW_TOO_LARGE: SubStatusCode = _;
        const CLIENT_NO_OVERLAPPING_FEED_RANGES_FOR_SESSION_TOKEN: SubStatusCode = _;
        const CLIENT_NO_THROUGHPUT_OFFER_FOR_RESOURCE: SubStatusCode = _;
        const CLIENT_OPAQUE_TOKEN_INVALID_FOR_CROSS_PARTITION_QUERY: SubStatusCode = _;
        const CLIENT_OPERATION_TIMEOUT: SubStatusCode = _;
        const CLIENT_ORDER_BY_COMPLEX_VALUE_UNSUPPORTED: SubStatusCode = _;
        const CLIENT_PARTITION_KEY_EMPTY: SubStatusCode = _;
        const CLIENT_PARTITION_KEY_RANGE_CACHE_REQUIRED: SubStatusCode = _;
        const CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS: SubStatusCode = _;
        const CLIENT_PREFIX_PARTITION_KEY_REQUIRES_MULTIHASH: SubStatusCode = _;
        const CLIENT_QUERY_PLAN_COMPLEX_PROJECTION_UNSUPPORTED: SubStatusCode = _;
        const CLIENT_QUERY_PLAN_INVALID_TOP_OFFSET_LIMIT: SubStatusCode = _;
        const CLIENT_QUERY_PLAN_PRODUCED_EMPTY_RANGES: SubStatusCode = _;
        const CLIENT_QUERY_PLAN_RANGE_NOT_COVERED_BY_TOPOLOGY: SubStatusCode = _;
        const CLIENT_QUERY_REWRITE_BODY_INVALID: SubStatusCode = _;
        const CLIENT_REQUEST_URL_MISSING_HOST: SubStatusCode = _;
        const CLIENT_REQUEST_URL_MISSING_KNOWN_PORT: SubStatusCode = _;
        const CLIENT_REQWEST_FEATURE_REQUIRED: SubStatusCode = _;
        const CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT: SubStatusCode = _;
        const CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE: SubStatusCode = _;
        const CLIENT_SPLIT_RETRIES_EXHAUSTED: SubStatusCode = _;
        const CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID: SubStatusCode = _;
        const CLIENT_THREAD_STARVATION: SubStatusCode = _;
        const CLIENT_THROUGHPUT_CONTROL_GROUP_NOT_REGISTERED: SubStatusCode = _;
        const CLIENT_THROUGHPUT_CONTROL_GROUP_REGISTRATION_FAILED: SubStatusCode = _;
        const CLIENT_THROUGHPUT_POLLER_INCOMPLETE: SubStatusCode = _;
        const CLIENT_TOPOLOGY_PROVIDER_MISSING: SubStatusCode = _;
        const CLIENT_TOPOLOGY_RESOLUTION_FAILED: SubStatusCode = _;
        const CLIENT_UNKNOWN_CONSISTENCY_LEVEL: SubStatusCode = _;
        const CLIENT_UNKNOWN_PRIORITY_LEVEL: SubStatusCode = _;
        const CLIENT_UNSUPPORTED_QUERY_FEATURE: SubStatusCode = _;
        const COLLECTIONS_IN_PARTITION_GOT_UPDATED: SubStatusCode = _;
        const COLLECTION_CREATE_IN_PROGRESS: SubStatusCode = _;
        const COLLECTION_QUOTA_EXCEEDED: SubStatusCode = _;
        const COLLECTION_QUOTA_EXCEEDED_AUTOPILOT: SubStatusCode = _;
        const COLLECTION_RID_MISMATCH: SubStatusCode = _;
        const COLLECTION_STATE_CHANGED: SubStatusCode = _;
        const COLLECTION_TRUNCATE_NOT_ALLOWED_DURING_MERGE: SubStatusCode = _;
        const COMPLETING_PARTITION_MIGRATION: SubStatusCode = _;
        const COMPLETING_SPLIT: SubStatusCode = _;
        const COMPUTE_FEDERATION_NOT_FOUND: SubStatusCode = _;
        const COMPUTE_INTERNAL_ERROR: SubStatusCode = _;
        const CONFIGURATION_NAME_NOT_EMPTY: SubStatusCode = _;
        const CONFIGURATION_OPERATION_CANCELLED: SubStatusCode = _;
        const CONFLICT_OPERATION_IN_USER_TRANSACTION: SubStatusCode = _;
        const CONFLICT_WITH_CONTROL_PLANE: SubStatusCode = _;
        const CONNECTION_RATE_LIMITER: SubStatusCode = _;
        const CROSS_COLLECTION_TRANSACTION_NOT_SUPPORTED: SubStatusCode = _;
        const CROSS_PARTITION_QUERY_NOT_SERVABLE: SubStatusCode = _;
        const CUSTOMER_KEY_ROTATED: SubStatusCode = _;
        const DATABASE_ACCOUNT_NOT_FOUND: SubStatusCode = _;
        const DATABASE_NAME_EXISTS: SubStatusCode = _;
        const DATABASE_QUOTA_EXCEEDED: SubStatusCode = _;
        #[cfg(feature = "preview_dtx")]
        const DTC_ACCOUNT_CONFIG_FAILURE: SubStatusCode = _;
        #[cfg(feature = "preview_dtx")]
        const DTC_COORDINATOR_RACE_CONFLICT: SubStatusCode = _;
        #[cfg(feature = "preview_dtx")]
        const DTC_DISPATCH_FAILURE: SubStatusCode = _;
        #[cfg(feature = "preview_dtx")]
        const DTC_LEDGER_FAILURE: SubStatusCode = _;
        #[cfg(feature = "preview_dtx")]
        const DTC_OPERATION_ROLLED_BACK: SubStatusCode = _;
        const DUPLICATE_RETRIABLE_WRITE_REQUEST: SubStatusCode = _;
        const FEDERATION_DOES_NOT_EXIST_OR_IS_LOCKED: SubStatusCode = _;
        const GATEWAY_ENDPOINT_READ_TIMEOUT: SubStatusCode = _;
        const GATEWAY_ENDPOINT_UNAVAILABLE: SubStatusCode = _;
        const GATEWAY_THROTTLED: SubStatusCode = _;
        const GONE_EXCEPTION: SubStatusCode = _;
        const HOT_PARTITION_KEY_THROTTLED: SubStatusCode = _;
        const HTTP_LISTENER_EXCEPTION: SubStatusCode = _;
        const INITIAL_RETRIABLE_WRITE_REQUEST_COMPLETED: SubStatusCode = _;
        const INSUFFICIENT_BINDABLE_PARTITIONS: SubStatusCode = _;
        const INSUFFICIENT_CAPACITY: SubStatusCode = _;
        const INVALID_ACCOUNT_CONFIGURATION: SubStatusCode = _;
        const INVALID_ACCOUNT_STATUS: SubStatusCode = _;
        const INVALID_KEY_VAULT_CERT_URI: SubStatusCode = _;
        const INVALID_KEY_VAULT_KEY_AND_CERT_URI: SubStatusCode = _;
        const INVALID_KEY_VAULT_SECRET_URI: SubStatusCode = _;
        const INVALID_THROUGHPUT_CAP_VALUE: SubStatusCode = _;
        const INVALID_TOPOLOGY_CHANGE_REQUEST: SubStatusCode = _;
        const INVALID_TRANSACTION_ID: SubStatusCode = _;
        const KEY_DISABLED_OR_EXPIRED: SubStatusCode = _;
        const KEY_VAULT_NOT_FOUND: SubStatusCode = _;
        const KEY_VAULT_OUTBOUND_DENIED_BY_NSP: SubStatusCode = _;
        const LEAKED_PARTITION: SubStatusCode = _;
        const LEASE_NOT_FOUND: SubStatusCode = _;
        const LOCAL_AUTH_DISABLED: SubStatusCode = _;
        const LOG_FLUSH_QUEUE_DEPTH_BACKPRESSURE: SubStatusCode = _;
        const LOG_STORE_NO_FREE_SEGMENTS: SubStatusCode = _;
        const MALFORMED_CONTINUATION_TOKEN: SubStatusCode = _;
        const MASTER_SERVICE_UNAVAILABLE: SubStatusCode = _;
        const MERGE_DISABLED: SubStatusCode = _;
        const MISMATCHING_COLLECTION_RIDS_ON_MIGRATE_PARTITION: SubStatusCode = _;
        const MISSING_PARTITION_RESOURCE_ON_ABORT_MIGRATION: SubStatusCode = _;
        const MISSING_PARTITION_RESOURCE_ON_COMPLETE_MIGRATION: SubStatusCode = _;
        const MISSING_REQUEST_PARAMETER: SubStatusCode = _;
        const NAME_CACHE_STALE: SubStatusCode = _;
        const OFFER_NOT_CONFIGURED: SubStatusCode = _;
        const OFFER_REPLACE_DISABLED_AUTO_SCALE_OFFER: SubStatusCode = _;
        const OFFER_REPLACE_IN_PROGRESS: SubStatusCode = _;
        const OFFER_SCALED_UP_BY_USER: SubStatusCode = _;
        const OFFER_VALIDATION_FAILED: SubStatusCode = _;
        const OPERATION_IN_PROGRESS: SubStatusCode = _;
        const OPERATION_LOG_SIZE_TOO_BIG: SubStatusCode = _;
        const OPERATION_PAUSED: SubStatusCode = _;
        const OWNER_RESOURCE_NOT_FOUND: SubStatusCode = _;
        const PARTITIONED_RESOURCE_QUOTA_EXCEEDED: SubStatusCode = _;
        const PARTITION_FAILOVER_ERROR_CODE: SubStatusCode = _;
        const PARTITION_KEY_DEFINITION_MISSING_FOR_AUTOPILOT: SubStatusCode = _;
        const PARTITION_KEY_DEFINITION_NOT_SPECIFIED: SubStatusCode = _;
        const PARTITION_KEY_DELETE_REQUEST_LIMIT_EXCEEDED: SubStatusCode = _;
        const PARTITION_KEY_HASH_COLLISION: SubStatusCode = _;
        const PARTITION_KEY_MISMATCH: SubStatusCode = _;
        const PARTITION_KEY_QUOTA_OVER_LIMIT: SubStatusCode = _;
        const PARTITION_KEY_RANGE_GONE: SubStatusCode = _;
        const PARTITION_MIGRATING_COLLECTION_DELETED: SubStatusCode = _;
        const PARTITION_MIGRATION_DOC_COUNT_MISMATCH_SOURCE_TARGET: SubStatusCode = _;
        const PARTITION_MIGRATION_DOC_COUNT_MISMATCH_TARGET_REPLICAS: SubStatusCode = _;
        const PARTITION_MIGRATION_FAILED_TO_UPDATE_DNS: SubStatusCode = _;
        const PARTITION_MIGRATION_PARTITION_RESOURCE_NOT_FOUND: SubStatusCode = _;
        const PARTITION_MIGRATION_SHARED_THROUGHPUT_DB_PARTITION_NOT_FOUND: SubStatusCode = _;
        const PARTITION_MIGRATION_SOURCE_PARTITION_DELETED_IN_MASTER: SubStatusCode = _;
        const PARTITION_NOT_IN_MIGRATING_STATUS: SubStatusCode = _;
        const PATCH_CONDITION_NOT_MET: SubStatusCode = _;
        const PREPARE_TIME_EXCEEDED: SubStatusCode = _;
        const PROVISION_LIMIT_REACHED: SubStatusCode = _;
        const QUERY_EXECUTION_COMPLETE: SubStatusCode = _;
        const QUERY_EXECUTION_IN_PROGRESS: SubStatusCode = _;
        const QUERY_REQUEST_INITIALIZED: SubStatusCode = _;
        const QUERY_WAIT_FOR_SEQUENTIAL_PROGRESS: SubStatusCode = _;
        const QUORUM_NOT_MET: SubStatusCode = _;
        const QUOTA_EXCEEDED: SubStatusCode = _;
        const RBAC_AAD_GROUP_UNAVAILABLE: SubStatusCode = _;
        const RBAC_DISABLED_DUE_TO_ARM_PATH: SubStatusCode = _;
        const RBAC_REQUEST_NOT_AUTHORIZED: SubStatusCode = _;
        const READ_SESSION_NOT_AVAILABLE: SubStatusCode = _;
        const REDUNDANT_COLLECTION_PUT: SubStatusCode = _;
        const REPLICATION_QUEUE_FULL: SubStatusCode = _;
        const REQUEST_PREEMPTED: SubStatusCode = _;
        const RESOURCE_NOT_FOUND: SubStatusCode = _;
        const RESOURCE_SOFT_DELETED: SubStatusCode = _;
        const RETRIABLE_WRITE_RESPONSE_EXPIRED_IN_PRIMARY_CACHE: SubStatusCode = _;
        const RNTBD_CLIENT_CHANNEL: SubStatusCode = _;
        const RUPM_PARTITION_LIMIT_EXCEEDED: SubStatusCode = _;
        const RUPM_SHARED_BUDGET_EXCEEDED: SubStatusCode = _;
        const RU_BUDGET_EXCEEDED: SubStatusCode = _;
        const RU_BUDGET_EXCEEDED_FOR_MASTER: SubStatusCode = _;
        const SCHEMA_HASH_OR_ID_MISMATCH: SubStatusCode = _;
        const SCHEMA_OWNER_ID_MISMATCH: SubStatusCode = _;
        const SCRIPT_COMPILE_ERROR: SubStatusCode = _;
        const SERIALIZATION_REQUEST_BODY_INVALID: SubStatusCode = _;
        const SERIALIZATION_RESPONSE_BODY_INVALID: SubStatusCode = _;
        const SERVER_BARRIER_THROTTLED: SubStatusCode = _;
        const SERVICE_IS_OFFLINE: SubStatusCode = _;
        const SERVICE_MODULE: SubStatusCode = _;
        const SERVICE_ORDER_BY_ENVELOPE_INVALID: SubStatusCode = _;
        const SERVICE_QUERY_PLAN_ORDER_BY_MISSING_REWRITTEN_QUERY: SubStatusCode = _;
        const SERVICE_RETURNED_OBJECT_WITHOUT_RID: SubStatusCode = _;
        const SERVICE_RETURNED_OFFER_WITHOUT_ID: SubStatusCode = _;
        const SHARED_THROUGHPUT_DATABASE_COLLECTION_COUNT_EXCEEDED: SubStatusCode = _;
        const SHARED_THROUGHPUT_DATABASE_COUNT_EXCEEDED: SubStatusCode = _;
        const SHARED_THROUGHPUT_OFFER_GROW_NOT_NEEDED: SubStatusCode = _;
        const SINK_PARTITION_VALUE_DOES_NOT_MATCH_EXPECTED_BOUND: SubStatusCode = _;
        const SPLIT_DISABLED: SubStatusCode = _;
        const STALENESS_EXCEEDED_BOUND: SubStatusCode = _;
        const STORAGE_SPLIT_CONFLICTING_WITH_NWAY_THROUGHPUT_SPLIT: SubStatusCode = _;
        const STORED_PROCEDURE_CONCURRENCY: SubStatusCode = _;
        const STORE_NOT_READY: SubStatusCode = _;
        const SYSTEM_PARTITION_KEY_NOT_ALLOWED: SubStatusCode = _;
        const SYSTEM_RESOURCE_UNAVAILABLE: SubStatusCode = _;
        const THROTTLED_BY_BLOB_READ: SubStatusCode = _;
        const THROTTLED_OFFER_SCALE_DOWN: SubStatusCode = _;
        const THROTTLE_DUE_TO_ENCRYPTED_REVOKED_STORE_LOG_NOT_EMPTY: SubStatusCode = _;
        const THROTTLE_DUE_TO_REPLICATION_BACKPRESSURE: SubStatusCode = _;
        const THROTTLE_DUE_TO_RESOURCE_EXHAUSTION: SubStatusCode = _;
        const THROTTLE_DUE_TO_SPLIT: SubStatusCode = _;
        const THROTTLE_DUE_TO_STAGING_INDEX_QUEUE_FULL: SubStatusCode = _;
        const THROTTLE_DUE_TO_TRAFFIC_REGULATION: SubStatusCode = _;
        const THROTTLE_DUE_TO_TRANSPORT_BUFFER_USAGE: SubStatusCode = _;
        const THROUGHPUT_BUCKET_LIMIT_EXHAUSTED: SubStatusCode = _;
        const THROUGHPUT_CAP_EXCEEDED: SubStatusCode = _;
        const THROUGHPUT_CONTROL_REQUEST_RATE_TOO_LARGE: SubStatusCode = _;
        const TOMBSTONE_RECORDS_NOT_FOUND: SubStatusCode = _;
        const TOO_MANY_TENTATIVE_WRITES_TO_SATELLITE_REGION: SubStatusCode = _;
        const TOO_MANY_THROUGHPUT_BUCKET_UPDATES: SubStatusCode = _;
        const TRANSACTION_ALREADY_ACTIVE: SubStatusCode = _;
        const TRANSACTION_LIMIT_EXCEEDED: SubStatusCode = _;
        const TRANSIT_TIMEOUT: SubStatusCode = _;
        const TRANSPORT_BODY_READ_FAILED: SubStatusCode = _;
        const TRANSPORT_CONNECTION_FAILED: SubStatusCode = _;
        const TRANSPORT_DNS_FAILED: SubStatusCode = _;
        const TRANSPORT_GENERATED_503: SubStatusCode = _;
        const TRANSPORT_HTTP2_INCOMPATIBLE: SubStatusCode = _;
        const TRANSPORT_IO_FAILED: SubStatusCode = _;
        const UNDEFINED_DEFAULT_IDENTITY: SubStatusCode = _;
        const UNEXPECTED_THROTTLE: SubStatusCode = _;
        const UNIQUE_INDEX_CONFLICT: SubStatusCode = _;
        const UNIQUE_INDEX_RE_INDEX_IN_PROGRESS: SubStatusCode = _;
        const UNKNOWN: SubStatusCode = _;
        const VALUE_DOES_NOT_MATCH_EXPECTED_BOUND: SubStatusCode = _;
        const WRITE_FORBIDDEN: SubStatusCode = _;
        const XP_COMPOSITE_REPLICATOR: SubStatusCode = _;
        pub fn from_header_value(s: &str) -> Option<Self>;
        pub fn name(&self, status_code: Option<StatusCode>) -> Option<&'static str>;
        pub const fn new(code: u16) -> Self;
        pub const fn value(&self) -> u16;
    }
    impl Debug for SubStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Default for SubStatusCode {
        fn default() -> Self;
    }
    impl Display for SubStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<SubStatusCode> for u16 {
        fn from(code: SubStatusCode) -> Self;
    }
    impl From<u16> for SubStatusCode {
        fn from(value: u16) -> Self;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct ThroughputControlGroupName(pub std::borrow::Cow<'static, str>);
    impl ThroughputControlGroupName {
        pub fn as_str(&self) -> &str;
        pub fn new<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(name: impl Into<Cow<'static, str>>) -> Self;
    }
    impl AsRef<str> for ThroughputControlGroupName {
        fn as_ref(&self) -> &str;
    }
    impl Display for ThroughputControlGroupName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl From<&'static str> for ThroughputControlGroupName {
        fn from(name: &'static str) -> Self;
    }
    impl From<Cow<'static, str>> for ThroughputControlGroupName {
        fn from(name: Cow<'static, str>) -> Self;
    }
    impl From<String> for ThroughputControlGroupName {
        fn from(name: String) -> Self;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct TriggerReference {
    }
    impl TriggerReference {
        pub fn account(&self) -> &AccountReference;
        pub fn container(&self) -> &ContainerReference;
        pub fn from_name<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(container: &ContainerReference, trigger_name: impl Into<Cow<'static, str>>) -> Self;
        pub fn from_rid<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(container: &ContainerReference, trigger_rid: impl Into<Cow<'static, str>>) -> Self;
        pub fn is_by_name(&self) -> bool;
        pub fn is_by_rid(&self) -> bool;
        pub fn name(&self) -> Option<&str>;
        pub fn resource_link(&self) -> &str;
        pub fn rid(&self) -> Option<&str>;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct UdfReference {
    }
    impl UdfReference {
        pub fn account(&self) -> &AccountReference;
        pub fn container(&self) -> &ContainerReference;
        pub fn from_name<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(container: &ContainerReference, udf_name: impl Into<Cow<'static, str>>) -> Self;
        pub fn from_rid<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(container: &ContainerReference, udf_rid: impl Into<Cow<'static, str>>) -> Self;
        pub fn is_by_name(&self) -> bool;
        pub fn is_by_rid(&self) -> bool;
        pub fn name(&self) -> Option<&str>;
        pub fn resource_link(&self) -> &str;
        pub fn rid(&self) -> Option<&str>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct UserAgent {
    }
    impl UserAgent {
        pub fn as_str(&self) -> &str;
        pub fn suffix(&self) -> Option<&str>;
    }
    impl Default for UserAgent {
        fn default() -> Self;
    }
    impl Display for UserAgent {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(tag = "kind", content = "value", rename_all = "snake_case")]
    pub enum ChangeFeedStartFrom {
        Beginning,
        Now,
        PointInTime(time::OffsetDateTime),
    }
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum CosmosNumber {
        Int(i64),
        Float(f64),
    }
    impl From<f64> for CosmosNumber {
        fn from(v: f64) -> Self;
    }
    impl From<i64> for CosmosNumber {
        fn from(v: i64) -> Self;
    }
    impl Serialize for CosmosNumber {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>;
    }
    impl<'de> Deserialize<'de> for CosmosNumber {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, <D as >::Error>;
    }
    #[derive(Clone)]
    pub enum Credential {
        MasterKey(azure_core::credentials::Secret),
        TokenCredential(std::sync::Arc<dyn TokenCredential>),
    }
    impl Debug for Credential {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl From<Arc<dyn TokenCredential>> for Credential {
        fn from(credential: Arc<dyn TokenCredential>) -> Self;
    }
    impl From<Secret> for Credential {
        fn from(key: Secret) -> Self;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub enum DistributedTransactionOperationKind {
        Create,
        Read,
        Replace,
        Upsert,
        Delete,
        Patch,
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum DistributedTransactionResultBody {
        #[default]
        None,
        Bytes(azure_core::Bytes),
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum DistributedTransactionType {
        Write,
        Read,
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedTransactionType {
        pub fn as_str(self) -> &'static str;
    }
    #[cfg(feature = "preview_dtx")]
    impl AsRef<str> for DistributedTransactionType {
        fn as_ref(&self) -> &str;
    }
    #[cfg(feature = "preview_dtx")]
    impl Display for DistributedTransactionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum MaxItemCountHint {
        ServerDecides,
        Limit(std::num::NonZeroU32),
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum OperationType {
        Create,
        Read,
        ReadFeed,
        Replace,
        Delete,
        Upsert,
        Query,
        SqlQuery,
        QueryPlan,
        Batch,
        Head,
        HeadFeed,
        Execute,
        Patch,
        #[cfg(feature = "preview_dtx")]
        CommitDistributedTransaction,
        #[cfg(feature = "preview_dtx")]
        ReadDistributedTransaction,
    }
    impl OperationType {
        pub fn as_str(self) -> &'static str;
        pub fn http_method(self) -> azure_core::http::Method;
        pub fn is_feed(self) -> bool;
        pub fn is_idempotent(self) -> bool;
        pub fn is_read_only(self) -> bool;
        pub fn routes_to_write_endpoints(self) -> bool;
    }
    impl AsRef<str> for OperationType {
        fn as_ref(&self) -> &str;
    }
    impl Display for OperationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub enum PartitionKeyKind {
        #[default]
        Hash,
        MultiHash,
        Range,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(try_from = "u32", into = "u32")]
    pub enum PartitionKeyVersion {
        V1,
        V2,
    }
    impl PartitionKeyVersion {
        pub const fn value(self) -> u32;
    }
    impl From<PartitionKeyVersion> for u32 {
        fn from(version: PartitionKeyVersion) -> Self;
    }
    impl TryFrom<u32> for PartitionKeyVersion {
        type Error = &'static str;
        fn try_from(value: u32) -> Result<Self, <Self as >::Error>;
    }
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(tag = "op", rename_all = "lowercase")]
    pub enum PatchOperation {
        Add { path: String, value: serde_json::Value },
        Set { path: String, value: serde_json::Value },
        Replace { path: String, value: serde_json::Value },
        Remove { path: String },
        #[serde(rename = "incr", alias = "increment")]
        Increment { path: String, value: CosmosNumber },
        Move { from: String, path: String },
    }
    impl PatchOperation {
        pub fn add<impl Into<String>: Into<String>>(path: impl Into<String>, value: Value) -> Self;
        pub fn increment<impl Into<String>: Into<String>, impl Into<CosmosNumber>: Into<CosmosNumber>>(path: impl Into<String>, value: impl Into<CosmosNumber>) -> Self;
        pub fn move_value<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(from: impl Into<String>, path: impl Into<String>) -> Self;
        pub fn path(&self) -> &str;
        pub fn remove<impl Into<String>: Into<String>>(path: impl Into<String>) -> Self;
        pub fn replace<impl Into<String>: Into<String>>(path: impl Into<String>, value: Value) -> Self;
        pub fn set<impl Into<String>: Into<String>>(path: impl Into<String>, value: Value) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum Precondition {
        IfMatch(azure_core::http::Etag),
        IfNoneMatch(azure_core::http::Etag),
    }
    impl Precondition {
        pub fn as_if_match(&self) -> Option<&Etag>;
        pub fn as_if_none_match(&self) -> Option<&Etag>;
        pub fn if_match<impl Into<Etag>: Into<Etag>>(etag: impl Into<Etag>) -> Self;
        pub fn if_none_match<impl Into<Etag>: Into<Etag>>(etag: impl Into<Etag>) -> Self;
        pub fn is_if_match(&self) -> bool;
        pub fn is_if_none_match(&self) -> bool;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum ResourceType {
        DatabaseAccount,
        Database,
        DocumentCollection,
        Document,
        StoredProcedure,
        Trigger,
        UserDefinedFunction,
        PartitionKeyRange,
        Offer,
        #[cfg(feature = "preview_dtx")]
        DistributedTransactionBatch,
    }
    impl ResourceType {
        pub fn as_str(self) -> &'static str;
        pub fn is_metadata(self) -> bool;
        pub fn is_partitioned(self, operation_type: OperationType) -> bool;
        pub fn path_segment(self) -> &'static str;
        pub fn requires_container(self) -> bool;
        pub fn requires_database(self) -> bool;
    }
    impl AsRef<str> for ResourceType {
        fn as_ref(&self) -> &str;
    }
    impl Display for ResourceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl FromStr for ResourceType {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[derive(Clone, Debug, Default)]
    pub enum ResponseBody {
        #[default]
        NoPayload,
        Bytes(azure_core::Bytes),
        Items(Vec<azure_core::Bytes>),
    }
    impl ResponseBody {
        pub fn empty() -> Self;
        pub fn from_bytes<impl Into<Bytes>: Into<Bytes>>(bytes: impl Into<Bytes>) -> Self;
        pub fn from_items(items: Vec<Bytes>) -> Self;
        pub fn into_items<T: DeserializeOwned>(self) -> crate::error::Result<Vec<T>>;
        pub fn into_single<T: DeserializeOwned>(self) -> crate::error::Result<T>;
        pub fn is_empty(&self) -> bool;
        pub fn items(self) -> crate::error::Result<Vec<Bytes>>;
        pub fn single(self) -> crate::error::Result<Bytes>;
    }
    impl From<Bytes> for ResponseBody {
        fn from(bytes: Bytes) -> Self;
    }
    impl From<Vec<u8>> for ResponseBody {
        fn from(bytes: Vec<u8>) -> Self;
    }
    pub const DEFAULT_PATCH_TRACKING_CAPACITY: std::num::NonZeroU16 = _;
    pub const PATCH_TRACKING_PROPERTY: &str = "_azsdkPatchTracking";
    pub const PATCH_TRACKING_RETENTION: std::time::Duration = _;
    #[allow(dead_code)]
    pub mod effective_partition_key {
        #[derive(Clone, Debug, Eq)]
        pub struct EffectivePartitionKey(/* private fields */);
        impl EffectivePartitionKey {
            const MAX: Self = _;
            const MIN: Self = _;
            pub fn to_hex(&self) -> String;
        }
        impl Display for EffectivePartitionKey {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl From<&str> for EffectivePartitionKey {
            fn from(s: &str) -> Self;
        }
        impl From<String> for EffectivePartitionKey {
            fn from(s: String) -> Self;
        }
        impl Hash for EffectivePartitionKey {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H);
        }
        impl Ord for EffectivePartitionKey {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering;
        }
        impl PartialEq<&str> for EffectivePartitionKey {
            fn eq(&self, other: &&str) -> bool;
        }
        impl PartialEq<str> for EffectivePartitionKey {
            fn eq(&self, other: &str) -> bool;
        }
        impl PartialEq for EffectivePartitionKey {
            fn eq(&self, other: &Self) -> bool;
        }
        impl PartialOrd for EffectivePartitionKey {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>;
        }
        impl Serialize for EffectivePartitionKey {
            fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
        }
        impl<'de> Deserialize<'de> for EffectivePartitionKey {
            fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
        }
    }
    #[allow(dead_code)]
    pub mod partition_key_range {
        #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
        pub struct PartitionKeyRange {
            #[serde(rename = "id")]
            pub id: String,
            #[serde(rename = "minInclusive")]
            pub min_inclusive: crate::models::effective_partition_key::EffectivePartitionKey,
            #[serde(rename = "maxExclusive")]
            pub max_exclusive: crate::models::effective_partition_key::EffectivePartitionKey,
            #[serde(rename = "throughputFraction", default)]
            pub throughput_fraction: f64,
            #[serde(rename = "parents", skip_serializing_if = "Option::is_none")]
            pub parents: Option<Vec<String>>,
        }
        impl PartitionKeyRange {
            pub fn get_parent_ids(&self) -> HashSet<String>;
            pub fn new<impl Into<EffectivePartitionKey>: Into<EffectivePartitionKey>, impl Into<EffectivePartitionKey>: Into<EffectivePartitionKey>>(id: String, min_inclusive: impl Into<EffectivePartitionKey>, max_exclusive: impl Into<EffectivePartitionKey>) -> Self;
        }
        impl Eq for PartitionKeyRange {
        }
        impl Hash for PartitionKeyRange {
            fn hash<H: Hasher>(&self, state: &mut H);
        }
        impl Ord for PartitionKeyRange {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering;
        }
        impl PartialEq for PartitionKeyRange {
            fn eq(&self, other: &Self) -> bool;
        }
        impl PartialOrd for PartitionKeyRange {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>;
        }
        impl TryFrom<&PartitionKeyRange> for FeedRange {
            type Error = CosmosError;
            fn try_from(pkr: &PartitionKeyRange) -> Result<Self, <Self as >::Error>;
        }
    }
}
pub mod options {
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct BinaryEncodingOptions {
        pub enabled: bool,
        pub request_text_response: bool,
    }
    impl BinaryEncodingOptions {
        pub fn new() -> Self;
        pub fn with_enabled(self, enabled: bool) -> Self;
        pub fn with_request_text_response(self, request_text_response: bool) -> Self;
    }
    impl Default for BinaryEncodingOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct ConnectionPoolOptions {
    }
    impl ConnectionPoolOptions {
        pub fn builder() -> ConnectionPoolOptionsBuilder;
        pub fn gateway_v2_disabled(&self) -> bool;
        pub fn http2_consecutive_failure_threshold(&self) -> u32;
        pub fn http2_eviction_grace_period(&self) -> Duration;
        pub fn http2_fan_out_threshold_percent(&self) -> u8;
        pub fn http2_health_check_interval(&self) -> Duration;
        pub fn http2_keep_alive_interval(&self) -> Duration;
        pub fn http2_keep_alive_timeout(&self) -> Duration;
        pub fn idle_connection_timeout(&self) -> Option<Duration>;
        pub fn idle_http2_client_timeout(&self) -> Duration;
        pub fn is_http2_allowed(&self) -> bool;
        pub fn local_address(&self) -> Option<IpAddr>;
        pub fn max_connect_timeout(&self) -> Duration;
        pub fn max_dataplane_request_timeout(&self) -> Duration;
        pub fn max_http2_connections_per_endpoint(&self) -> usize;
        pub fn max_http2_streams_per_client(&self) -> u32;
        pub fn max_idle_connections_per_endpoint(&self) -> usize;
        pub fn max_metadata_request_timeout(&self) -> Duration;
        pub fn min_connect_timeout(&self) -> Duration;
        pub fn min_dataplane_request_timeout(&self) -> Duration;
        pub fn min_http2_connections_per_endpoint(&self) -> usize;
        pub fn min_metadata_request_timeout(&self) -> Duration;
        pub fn proxy_allowed(&self) -> bool;
        pub fn server_certificate_validation(&self) -> ServerCertificateValidation;
        pub fn tcp_keepalive_interval(&self) -> Option<Duration>;
        pub fn tcp_keepalive_retries(&self) -> Option<u32>;
        pub fn tcp_keepalive_time(&self) -> Option<Duration>;
        #[cfg(feature = "rustls")]
        pub fn tls_backend(&self) -> TlsBackend;
    }
    impl Default for ConnectionPoolOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct ConnectionPoolOptionsBuilder {
    }
    #[automatically_derived]
    impl ConnectionPoolOptionsBuilder {
        pub fn from_env() -> Self;
        pub fn from_env_override() -> Self;
    }
    impl ConnectionPoolOptionsBuilder {
        pub fn build(self) -> crate::error::Result<ConnectionPoolOptions>;
        pub fn new() -> Self;
        pub fn with_gateway_v2_disabled(self, value: bool) -> Self;
        pub fn with_http2_consecutive_failure_threshold(self, value: u32) -> Self;
        pub fn with_http2_eviction_grace_period(self, timeout: Duration) -> Self;
        pub fn with_http2_fan_out_threshold_percent(self, value: u8) -> Self;
        pub fn with_http2_health_check_interval(self, timeout: Duration) -> Self;
        pub fn with_http2_keep_alive_interval(self, timeout: Duration) -> Self;
        pub fn with_http2_keep_alive_timeout(self, timeout: Duration) -> Self;
        pub fn with_idle_connection_timeout(self, timeout: Duration) -> Self;
        pub fn with_idle_http2_client_timeout(self, timeout: Duration) -> Self;
        pub fn with_is_http2_allowed(self, value: bool) -> Self;
        pub fn with_local_address(self, addr: IpAddr) -> Self;
        pub fn with_max_connect_timeout(self, timeout: Duration) -> Self;
        pub fn with_max_dataplane_request_timeout(self, timeout: Duration) -> Self;
        pub fn with_max_http2_connections_per_endpoint(self, value: usize) -> Self;
        pub fn with_max_http2_streams_per_client(self, value: u32) -> Self;
        pub fn with_max_idle_connections_per_endpoint(self, count: usize) -> Self;
        pub fn with_max_metadata_request_timeout(self, timeout: Duration) -> Self;
        pub fn with_min_connect_timeout(self, timeout: Duration) -> Self;
        pub fn with_min_dataplane_request_timeout(self, timeout: Duration) -> Self;
        pub fn with_min_http2_connections_per_endpoint(self, value: usize) -> Self;
        pub fn with_min_metadata_request_timeout(self, timeout: Duration) -> Self;
        pub fn with_proxy_allowed(self, value: bool) -> Self;
        pub fn with_server_certificate_validation(self, value: ServerCertificateValidation) -> Self;
        pub fn with_tcp_keepalive_interval(self, timeout: Duration) -> Self;
        pub fn with_tcp_keepalive_retries(self, value: u32) -> Self;
        pub fn with_tcp_keepalive_time(self, timeout: Duration) -> Self;
        #[cfg(feature = "rustls")]
        pub fn with_tls_backend(self, value: TlsBackend) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct CorrelationId(/* private fields */);
    impl CorrelationId {
        const MAX_LENGTH: usize = 50;
        pub fn as_str(&self) -> &str;
        pub fn new<impl Into<String>: Into<String>>(value: impl Into<String>) -> Self;
        pub fn try_new<impl Into<String>: Into<String>>(value: impl Into<String>) -> Option<Self>;
    }
    impl AsRef<str> for CorrelationId {
        fn as_ref(&self) -> &str;
    }
    impl Display for CorrelationId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct DiagnosticsOptions {
    }
    impl DiagnosticsOptions {
        pub fn builder() -> DiagnosticsOptionsBuilder;
        pub fn default_verbosity(&self) -> DiagnosticsVerbosity;
        pub fn max_request_diagnostics(&self) -> usize;
        pub fn max_summary_size_bytes(&self) -> usize;
    }
    impl Default for DiagnosticsOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct DiagnosticsOptionsBuilder {
    }
    #[automatically_derived]
    impl DiagnosticsOptionsBuilder {
        pub fn from_env() -> Self;
    }
    impl DiagnosticsOptionsBuilder {
        pub fn build(self) -> crate::error::Result<DiagnosticsOptions>;
        pub fn new() -> Self;
        pub fn with_default_verbosity(self, verbosity: DiagnosticsVerbosity) -> Self;
        pub fn with_max_request_diagnostics(self, max: usize) -> Self;
        pub fn with_max_summary_size_bytes(self, size: usize) -> Self;
    }
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[non_exhaustive]
    pub struct DiagnosticsThresholds {
    }
    impl DiagnosticsThresholds {
        pub fn new() -> Self;
        pub fn non_point_operation_latency(&self) -> Duration;
        pub fn payload_size(&self) -> u64;
        pub fn point_operation_latency(&self) -> Duration;
        pub fn request_charge(&self) -> f64;
        #[must_use]
        pub fn with_non_point_operation_latency(self, latency: Duration) -> Self;
        #[must_use]
        pub fn with_payload_size(self, payload_size: u64) -> Self;
        #[must_use]
        pub fn with_point_operation_latency(self, latency: Duration) -> Self;
        #[must_use]
        pub fn with_request_charge(self, request_charge: f64) -> Self;
    }
    impl Default for DiagnosticsThresholds {
        fn default() -> Self;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct DriverOptions {
    }
    impl DriverOptions {
        pub fn account(&self) -> &AccountReference;
        pub fn builder(account: AccountReference) -> DriverOptionsBuilder;
        #[cfg(feature = "fault_injection")]
        pub fn fault_injection_rules(&self) -> Option<&[Arc<FaultInjectionRule>]>;
        pub fn hedging_options(&self) -> &HedgingOptions;
        pub fn operation_options(&self) -> &Arc<OperationOptions>;
        pub fn partition_failover_options(&self) -> &PartitionFailoverOptions;
        pub fn partition_key_range_cache_enabled(&self) -> bool;
        pub fn preferred_regions(&self) -> &[Region];
        pub fn user_agent_suffix(&self) -> Option<&UserAgentSuffix>;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct DriverOptionsBuilder {
    }
    impl DriverOptionsBuilder {
        pub fn build(self) -> DriverOptions;
        pub fn new(account: AccountReference) -> Self;
        pub fn register_throughput_control_group(self, group: ThroughputControlGroupOptions) -> crate::error::Result<Self>;
        #[cfg(feature = "fault_injection")]
        pub fn with_fault_injection_rules(self, rules: Vec<Arc<FaultInjectionRule>>) -> crate::error::Result<Self>;
        pub fn with_hedging_options(self, options: HedgingOptions) -> Self;
        pub fn with_operation_options(self, options: OperationOptions) -> Self;
        pub fn with_partition_failover_options(self, options: PartitionFailoverOptions) -> Self;
        pub fn with_partition_key_range_cache_enabled(self, enabled: bool) -> Self;
        pub fn with_preferred_regions(self, regions: Vec<Region>) -> Self;
        pub fn with_user_agent_suffix(self, suffix: UserAgentSuffix) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct EndToEndOperationLatencyPolicy {
    }
    impl EndToEndOperationLatencyPolicy {
        pub fn new(timeout: Duration) -> Self;
        pub fn timeout(&self) -> Duration;
    }
    impl From<Duration> for EndToEndOperationLatencyPolicy {
        fn from(timeout: Duration) -> Self;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct ExcludedRegions(pub Vec<crate::options::Region>);
    impl ExcludedRegions {
        pub fn is_empty(&self) -> bool;
        pub fn iter(&self) -> impl Iterator<Item = &Region>;
        pub fn len(&self) -> usize;
        pub fn new() -> Self;
        pub fn with_region<impl Into<Region>: Into<Region>>(self, region: impl Into<Region>) -> Self;
    }
    impl<T: Into<crate::options::Region>> FromIterator<T> for ExcludedRegions {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct HedgeThreshold(/* private fields */);
    impl HedgeThreshold {
        pub const fn get(self) -> Duration;
        pub const fn new(duration: Duration) -> Option<Self>;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct HedgingOptions {
    }
    impl HedgingOptions {
        pub fn builder() -> HedgingOptionsBuilder;
        pub fn max_concurrent_metadata_attempts(&self) -> usize;
    }
    impl Default for HedgingOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct HedgingOptionsBuilder {
    }
    impl HedgingOptionsBuilder {
        pub fn build(self) -> HedgingOptions;
        pub fn new() -> Self;
        pub fn with_max_concurrent_metadata_attempts(self, value: usize) -> Self;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct HedgingStrategy {
    }
    impl HedgingStrategy {
        pub const fn new(threshold: HedgeThreshold) -> Self;
        pub const fn threshold(&self) -> HedgeThreshold;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct OperationOptions {
        pub query_plan_mode: Option<crate::options::QueryPlanMode>,
        pub patch_strategy: Option<crate::options::PatchStrategy>,
        pub read_consistency_strategy: Option<crate::options::ReadConsistencyStrategy>,
        pub excluded_regions: Option<crate::options::ExcludedRegions>,
        pub content_response_on_write: Option<crate::options::ContentResponseOnWrite>,
        pub throughput_control: Option<ThroughputControlOptions>,
        pub end_to_end_latency_policy: Option<crate::options::EndToEndOperationLatencyPolicy>,
        pub max_failover_retry_count: Option<u32>,
        pub endpoint_unavailability_ttl: Option<std::time::Duration>,
        pub session_capturing_disabled: Option<bool>,
        pub max_session_retry_count: Option<u32>,
        pub throttling_retry_options: Option<ThrottlingRetryOptions>,
        pub hedging_enabled: Option<bool>,
        pub availability_strategy: Option<crate::options::AvailabilityStrategy>,
        pub custom_headers: Option<std::collections::HashMap<azure_core::http::headers::HeaderName, azure_core::http::headers::HeaderValue>>,
        pub binary_encoding: Option<crate::options::BinaryEncodingOptions>,
    }
    #[automatically_derived]
    impl OperationOptions {
        pub fn from_env() -> Self;
        pub fn from_env_override() -> Self;
    }
    #[automatically_derived]
    pub struct OperationOptionsBuilder {
    }
    #[automatically_derived]
    impl OperationOptionsBuilder {
        #[must_use]
        pub fn build(self) -> OperationOptions;
        pub fn new() -> Self;
        pub fn with_availability_strategy(self, value: AvailabilityStrategy) -> Self;
        pub fn with_binary_encoding(self, value: BinaryEncodingOptions) -> Self;
        pub fn with_content_response_on_write(self, value: ContentResponseOnWrite) -> Self;
        pub fn with_custom_headers(self, value: HashMap<HeaderName, HeaderValue>) -> Self;
        pub fn with_end_to_end_latency_policy(self, value: EndToEndOperationLatencyPolicy) -> Self;
        pub fn with_endpoint_unavailability_ttl(self, value: Duration) -> Self;
        pub fn with_excluded_regions(self, value: ExcludedRegions) -> Self;
        pub fn with_hedging_enabled(self, value: bool) -> Self;
        pub fn with_max_failover_retry_count(self, value: u32) -> Self;
        pub fn with_max_session_retry_count(self, value: u32) -> Self;
        pub fn with_patch_strategy(self, value: PatchStrategy) -> Self;
        pub fn with_query_plan_mode(self, value: QueryPlanMode) -> Self;
        pub fn with_read_consistency_strategy(self, value: ReadConsistencyStrategy) -> Self;
        pub fn with_session_capturing_disabled(self, value: bool) -> Self;
        pub fn with_throttling_retry_options(self, value: ThrottlingRetryOptions) -> Self;
        pub fn with_throughput_control(self, value: ThroughputControlOptions) -> Self;
    }
    #[automatically_derived]
    pub struct OperationOptionsView<'a> {
    }
    #[automatically_derived]
    impl<'a> OperationOptionsView<'a> {
        pub fn availability_strategy(&self) -> Option<&AvailabilityStrategy>;
        pub fn binary_encoding(&self) -> Option<&BinaryEncodingOptions>;
        pub fn content_response_on_write(&self) -> Option<&ContentResponseOnWrite>;
        pub fn custom_headers(&self) -> Option<&HashMap<HeaderName, HeaderValue>>;
        pub fn end_to_end_latency_policy(&self) -> Option<&EndToEndOperationLatencyPolicy>;
        pub fn endpoint_unavailability_ttl(&self) -> Option<&Duration>;
        pub fn excluded_regions(&self) -> Option<&ExcludedRegions>;
        pub fn hedging_enabled(&self) -> Option<&bool>;
        pub fn max_failover_retry_count(&self) -> Option<&u32>;
        pub fn max_session_retry_count(&self) -> Option<&u32>;
        pub fn new(env: Option<::std::sync::Arc<OperationOptions>>, runtime: Option<::std::sync::Arc<OperationOptions>>, account: Option<::std::sync::Arc<OperationOptions>>, operation: Option<&'a OperationOptions>) -> Self;
        pub fn new_with_override(env_override: Option<::std::sync::Arc<OperationOptions>>, env: Option<::std::sync::Arc<OperationOptions>>, runtime: Option<::std::sync::Arc<OperationOptions>>, account: Option<::std::sync::Arc<OperationOptions>>, operation: Option<&'a OperationOptions>) -> Self;
        pub fn patch_strategy(&self) -> Option<&PatchStrategy>;
        pub fn query_plan_mode(&self) -> Option<&QueryPlanMode>;
        pub fn read_consistency_strategy(&self) -> Option<&ReadConsistencyStrategy>;
        pub fn session_capturing_disabled(&self) -> Option<&bool>;
        pub fn throttling_retry_options(&self) -> ThrottlingRetryOptionsView<'_>;
        pub fn throughput_control(&self) -> ThroughputControlOptionsView<'_>;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct PartitionFailoverOptions {
    }
    impl PartitionFailoverOptions {
        pub fn builder() -> PartitionFailoverOptionsBuilder;
        pub fn circuit_breaker_enabled(&self) -> bool;
        pub fn consecutive_hedge_win_threshold(&self) -> u32;
        pub fn counter_reset_window(&self) -> Duration;
        pub fn failback_sweep_interval(&self) -> Duration;
        pub fn partition_unavailability_duration(&self) -> Duration;
        pub fn read_failure_threshold(&self) -> u32;
        pub fn write_failure_threshold(&self) -> u32;
    }
    impl Default for PartitionFailoverOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct PartitionFailoverOptionsBuilder {
    }
    impl PartitionFailoverOptionsBuilder {
        pub fn build(self) -> crate::error::Result<PartitionFailoverOptions>;
        pub fn new() -> Self;
        pub fn with_circuit_breaker_enabled(self, value: bool) -> Self;
        pub fn with_consecutive_hedge_win_threshold(self, value: u32) -> Self;
        pub fn with_counter_reset_window(self, value: Duration) -> Self;
        pub fn with_failback_sweep_interval(self, value: Duration) -> Self;
        pub fn with_partition_unavailability_duration(self, value: Duration) -> Self;
        pub fn with_read_failure_threshold(self, value: u32) -> Self;
        pub fn with_write_failure_threshold(self, value: u32) -> Self;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct PlanOptions {
        pub max_fan_out: u32,
    }
    impl PlanOptions {
        pub fn with_max_fan_out(self, max_fan_out: u32) -> Self;
    }
    impl Default for PlanOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize)]
    #[non_exhaustive]
    #[serde(transparent)]
    pub struct Region {
    }
    impl Region {
        const APAC_SOUTHEAST_2: Region = _;
        const AUSTRALIA_CENTRAL: Region = _;
        const AUSTRALIA_CENTRAL_2: Region = _;
        const AUSTRALIA_EAST: Region = _;
        const AUSTRALIA_SOUTHEAST: Region = _;
        const AUSTRIA_EAST: Region = _;
        const BELGIUM_CENTRAL: Region = _;
        const BLEU_FRANCE_CENTRAL: Region = _;
        const BLEU_FRANCE_SOUTH: Region = _;
        const BRAZIL_SOUTH: Region = _;
        const BRAZIL_SOUTHEAST: Region = _;
        const CANADA_CENTRAL: Region = _;
        const CANADA_EAST: Region = _;
        const CENTRAL_INDIA: Region = _;
        const CENTRAL_US: Region = _;
        const CENTRAL_US_EUAP: Region = _;
        const CHILE_CENTRAL: Region = _;
        const CHINA_EAST: Region = _;
        const CHINA_EAST_2: Region = _;
        const CHINA_EAST_3: Region = _;
        const CHINA_NORTH: Region = _;
        const CHINA_NORTH_10: Region = _;
        const CHINA_NORTH_2: Region = _;
        const CHINA_NORTH_3: Region = _;
        const DELOS_CLOUD_GERMANY_CENTRAL: Region = _;
        const DELOS_CLOUD_GERMANY_NORTH: Region = _;
        const DENMARK_EAST: Region = _;
        const EAST_ASIA: Region = _;
        const EAST_EUROPE: Region = _;
        const EAST_US: Region = _;
        const EAST_US_2: Region = _;
        const EAST_US_2_EUAP: Region = _;
        const EAST_US_3: Region = _;
        const EAST_US_SLV: Region = _;
        const EAST_US_STG: Region = _;
        const FRANCE_CENTRAL: Region = _;
        const FRANCE_SOUTH: Region = _;
        const GERMANY_CENTRAL: Region = _;
        const GERMANY_NORTH: Region = _;
        const GERMANY_NORTHEAST: Region = _;
        const GERMANY_WEST_CENTRAL: Region = _;
        const INDIA_SOUTH_CENTRAL: Region = _;
        const INDONESIA_CENTRAL: Region = _;
        const ISRAEL_CENTRAL: Region = _;
        const ISRAEL_NORTHWEST: Region = _;
        const ITALY_NORTH: Region = _;
        const JAPAN_EAST: Region = _;
        const JAPAN_WEST: Region = _;
        const JIO_INDIA_CENTRAL: Region = _;
        const JIO_INDIA_WEST: Region = _;
        const KOREA_CENTRAL: Region = _;
        const KOREA_SOUTH: Region = _;
        const KOREA_SOUTH_2: Region = _;
        const MALAYSIA_SOUTH: Region = _;
        const MALAYSIA_WEST: Region = _;
        const MEXICO_CENTRAL: Region = _;
        const NEW_ZEALAND_NORTH: Region = _;
        const NORTHEAST_US_5: Region = _;
        const NORTH_CENTRAL_US: Region = _;
        const NORTH_EUROPE: Region = _;
        const NORTH_EUROPE_2: Region = _;
        const NORWAY_EAST: Region = _;
        const NORWAY_WEST: Region = _;
        const POLAND_CENTRAL: Region = _;
        const QATAR_CENTRAL: Region = _;
        const SINGAPORE_CENTRAL: Region = _;
        const SINGAPORE_NORTH: Region = _;
        const SOUTHEAST_ASIA: Region = _;
        const SOUTHEAST_US: Region = _;
        const SOUTHEAST_US_3: Region = _;
        const SOUTHEAST_US_5: Region = _;
        const SOUTHWEST_US: Region = _;
        const SOUTH_AFRICA_NORTH: Region = _;
        const SOUTH_AFRICA_WEST: Region = _;
        const SOUTH_CENTRAL_US: Region = _;
        const SOUTH_CENTRAL_US_2: Region = _;
        const SOUTH_CENTRAL_US_STG: Region = _;
        const SOUTH_INDIA: Region = _;
        const SPAIN_CENTRAL: Region = _;
        const SWEDEN_CENTRAL: Region = _;
        const SWEDEN_SOUTH: Region = _;
        const SWITZERLAND_NORTH: Region = _;
        const SWITZERLAND_WEST: Region = _;
        const TAIWAN_NORTH: Region = _;
        const TAIWAN_NORTHWEST: Region = _;
        const UAE_CENTRAL: Region = _;
        const UAE_NORTH: Region = _;
        const UK_NORTH: Region = _;
        const UK_SOUTH: Region = _;
        const UK_SOUTH_2: Region = _;
        const UK_WEST: Region = _;
        const USDOD_CENTRAL: Region = _;
        const USDOD_EAST: Region = _;
        const USDOD_SOUTHWEST: Region = _;
        const USDOD_SOUTH_CENTRAL: Region = _;
        const USDOD_WEST_CENTRAL: Region = _;
        const USGOV_ARIZONA: Region = _;
        const USGOV_IOWA: Region = _;
        const USGOV_TEXAS: Region = _;
        const USGOV_VIRGINIA: Region = _;
        const USGOV_WYOMING: Region = _;
        const USNAT_EAST: Region = _;
        const USNAT_WEST: Region = _;
        const USSEC_EAST: Region = _;
        const USSEC_WEST: Region = _;
        const USSEC_WEST_CENTRAL: Region = _;
        const WEST_CENTRAL_US: Region = _;
        const WEST_EUROPE: Region = _;
        const WEST_INDIA: Region = _;
        const WEST_US: Region = _;
        const WEST_US_2: Region = _;
        const WEST_US_3: Region = _;
        pub fn as_str(&self) -> &str;
        pub fn display_name(&self) -> &str;
        pub fn new<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(name: impl Into<Cow<'static, str>>) -> Self;
    }
    impl AsRef<str> for Region {
        fn as_ref(&self) -> &str;
    }
    impl Display for Region {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<&'static str> for Region {
        fn from(name: &'static str) -> Self;
    }
    impl From<String> for Region {
        fn from(name: String) -> Self;
    }
    impl<'de> Deserialize<'de> for Region {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct ThrottlingRetryOptions {
        pub max_retry_count: Option<u32>,
        pub max_retry_wait_time: Option<std::time::Duration>,
    }
    #[automatically_derived]
    impl ThrottlingRetryOptions {
        pub fn from_env() -> Self;
    }
    #[automatically_derived]
    pub struct ThrottlingRetryOptionsBuilder {
    }
    #[automatically_derived]
    impl ThrottlingRetryOptionsBuilder {
        #[must_use]
        pub fn build(self) -> ThrottlingRetryOptions;
        pub fn new() -> Self;
        pub fn with_max_retry_count(self, value: u32) -> Self;
        pub fn with_max_retry_wait_time(self, value: Duration) -> Self;
    }
    #[automatically_derived]
    pub struct ThrottlingRetryOptionsView<'a> {
    }
    #[automatically_derived]
    impl<'a> ThrottlingRetryOptionsView<'a> {
        pub fn max_retry_count(&self) -> Option<&u32>;
        pub fn max_retry_wait_time(&self) -> Option<&Duration>;
        pub fn new(env: Option<::std::sync::Arc<ThrottlingRetryOptions>>, runtime: Option<::std::sync::Arc<ThrottlingRetryOptions>>, account: Option<::std::sync::Arc<ThrottlingRetryOptions>>, operation: Option<&'a ThrottlingRetryOptions>) -> Self;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct ThroughputControlGroupOptions {
    }
    impl ThroughputControlGroupOptions {
        pub fn container(&self) -> &ContainerReference;
        pub fn is_default(&self) -> bool;
        pub fn name(&self) -> &ThroughputControlGroupName;
        pub fn new<impl Into<ThroughputControlGroupName>: Into<ThroughputControlGroupName>>(name: impl Into<ThroughputControlGroupName>, container: ContainerReference, is_default: bool) -> Self;
        pub fn priority_level(&self) -> Option<PriorityLevel>;
        pub fn set_priority_level(&self, level: PriorityLevel);
        pub fn set_throughput_bucket(&self, bucket: u32);
        pub fn throughput_bucket(&self) -> Option<u32>;
        pub fn with_priority_level(self, level: PriorityLevel) -> Self;
        pub fn with_throughput_bucket(self, bucket: u32) -> Self;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct ThroughputControlOptions {
        pub group_name: Option<crate::models::ThroughputControlGroupName>,
        pub throughput_bucket: Option<u32>,
        pub priority_level: Option<crate::options::PriorityLevel>,
    }
    #[automatically_derived]
    pub struct ThroughputControlOptionsBuilder {
    }
    #[automatically_derived]
    impl ThroughputControlOptionsBuilder {
        #[must_use]
        pub fn build(self) -> ThroughputControlOptions;
        pub fn new() -> Self;
        pub fn with_group_name(self, value: ThroughputControlGroupName) -> Self;
        pub fn with_priority_level(self, value: PriorityLevel) -> Self;
        pub fn with_throughput_bucket(self, value: u32) -> Self;
    }
    #[automatically_derived]
    pub struct ThroughputControlOptionsView<'a> {
    }
    #[automatically_derived]
    impl<'a> ThroughputControlOptionsView<'a> {
        pub fn group_name(&self) -> Option<&ThroughputControlGroupName>;
        pub fn new(env: Option<::std::sync::Arc<ThroughputControlOptions>>, runtime: Option<::std::sync::Arc<ThroughputControlOptions>>, account: Option<::std::sync::Arc<ThroughputControlOptions>>, operation: Option<&'a ThroughputControlOptions>) -> Self;
        pub fn priority_level(&self) -> Option<&PriorityLevel>;
        pub fn throughput_bucket(&self) -> Option<&u32>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct UserAgentSuffix(/* private fields */);
    impl UserAgentSuffix {
        const MAX_LENGTH: usize = 25;
        pub fn as_str(&self) -> &str;
        pub fn new<impl Into<String>: Into<String>>(value: impl Into<String>) -> Self;
        pub fn try_new<impl Into<String>: Into<String>>(value: impl Into<String>) -> Option<Self>;
    }
    impl AsRef<str> for UserAgentSuffix {
        fn as_ref(&self) -> &str;
    }
    impl Display for UserAgentSuffix {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct WorkloadId(/* private fields */);
    impl WorkloadId {
        const MAX: u8 = 50;
        const MIN: u8 = 1;
        pub fn new(value: u8) -> Self;
        pub fn try_new(value: u8) -> Option<Self>;
        pub fn value(&self) -> u8;
    }
    impl Display for WorkloadId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl TryFrom<u8> for WorkloadId {
        type Error = &'static str;
        fn try_from(value: u8) -> Result<Self, <Self as >::Error>;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum AvailabilityStrategy {
        Hedging(HedgingStrategy),
        Disabled,
    }
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    pub enum ContentResponseOnWrite {
        Enabled,
        #[default]
        Disabled,
    }
    impl Display for ContentResponseOnWrite {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl From<ContentResponseOnWrite> for bool {
        fn from(value: ContentResponseOnWrite) -> Self;
    }
    impl From<bool> for ContentResponseOnWrite {
        fn from(value: bool) -> Self;
    }
    impl FromStr for ContentResponseOnWrite {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum DiagnosticsVerbosity {
        #[default]
        Default,
        Summary,
        Detailed,
    }
    impl DiagnosticsVerbosity {
        pub fn as_str(&self) -> &'static str;
    }
    impl AsRef<str> for DiagnosticsVerbosity {
        fn as_ref(&self) -> &str;
    }
    impl Display for DiagnosticsVerbosity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl FromStr for DiagnosticsVerbosity {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum PatchStrategy {
        #[default]
        Auto,
        ClientSide,
        ServerSide,
    }
    impl PatchStrategy {
        pub fn as_str(&self) -> &'static str;
    }
    impl Display for PatchStrategy {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl FromStr for PatchStrategy {
        type Err = CosmosError;
        fn from_str(value: &str) -> Result<Self, <Self as >::Err>;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum PriorityLevel {
        #[default]
        High,
        Low,
    }
    impl PriorityLevel {
        pub fn as_str(&self) -> &'static str;
    }
    impl Display for PriorityLevel {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl FromStr for PriorityLevel {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum QueryPlanMode {
        #[default]
        LocalPreferred,
        GatewayOnly,
    }
    impl QueryPlanMode {
        pub fn as_str(self) -> &'static str;
    }
    impl Display for QueryPlanMode {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl FromStr for QueryPlanMode {
        type Err = CosmosError;
        fn from_str(value: &str) -> Result<Self, <Self as >::Err>;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum ReadConsistencyStrategy {
        Default,
        Eventual,
        Session,
        LatestCommitted,
        GlobalStrong,
    }
    impl ReadConsistencyStrategy {
        pub fn as_str(&self) -> &'static str;
    }
    impl Display for ReadConsistencyStrategy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl FromStr for ReadConsistencyStrategy {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum ServerCertificateValidation {
        #[default]
        Required,
        RequiredUnlessEmulator,
    }
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum TlsBackend {
        #[default]
        Rustls,
    }
    pub const DEFAULT_MAX_CONCURRENT_METADATA_ATTEMPTS: usize = 32;
    pub const DEFAULT_MAX_FAN_OUT: u32 = 100;
}
#[cfg(feature = "__internal_mocking")]
pub mod test {
    pub use azure_data_cosmos_driver::options::connection_pool::ConnectionPoolOptions;
    #[derive(Clone, Copy, Debug)]
    pub struct HttpClientConfig {
    }
    #[derive(Clone, Debug)]
    pub struct HttpRequest {
        pub url: url::Url,
        pub method: azure_core::http::Method,
        pub headers: azure_core::http::headers::Headers,
        pub body: Option<bytes::Bytes>,
        pub timeout: Option<std::time::Duration>,
    }
    #[derive(Clone, Debug)]
    pub struct HttpResponse {
        pub status: u16,
        pub headers: azure_core::http::headers::Headers,
        pub body: Vec<u8>,
    }
    pub struct TransportError {
        pub error: crate::error::CosmosError,
        pub request_sent: crate::diagnostics::RequestSentStatus,
    }
    impl TransportError {
        pub fn new<impl Into<crate::error::CosmosError>: Into<crate::error::CosmosError>>(error: impl Into<crate::error::CosmosError>, request_sent: RequestSentStatus) -> Self;
    }
    impl Debug for TransportError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Display for TransportError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Error for TransportError {
        fn source(&self) -> Option<&dyn std::error::Error + 'static>;
    }
    pub trait HttpClientFactory: fmt::Debug + Send + Sync {
        fn build(&self, connection_pool: &ConnectionPoolOptions, config: HttpClientConfig) -> crate::error::Result<Arc<dyn TransportClient>>;
    }
    #[async_trait]
    pub trait TransportClient: Send + Sync + fmt::Debug {
        #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn send(&self, request: &HttpRequest) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<HttpResponse, TransportError>> + ::core::marker::Send>>;
    }
}
```
