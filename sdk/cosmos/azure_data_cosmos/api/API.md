# azure_data_cosmos

- **Description**: Rust wrappers around Microsoft Azure REST APIs - Azure Cosmos DB
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `hmac_rust`
  - `reqwest`
  - `rustls`
- `control_plane`
- `distributed_tracing`
- `fault_injection`
- `hmac_openssl`
- `hmac_rust`
- `key_auth`
- `metrics`
- `native_tls`
- `preview_patch`
- `rustls`

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
#[doc(inline)]
pub use azure_data_cosmos::clients::container_client::ContainerClient;
#[doc(inline)]
pub use azure_data_cosmos::clients::cosmos_client::CosmosClient;
#[doc(inline)]
pub use azure_data_cosmos::clients::cosmos_client_builder::CosmosClientBuilder;
pub use azure_data_cosmos::error::CosmosError;
pub use azure_data_cosmos::error::CosmosStatus;
#[doc(inline)]
pub use azure_data_cosmos::clients::database_client::DatabaseClient;
#[cfg(feature = "preview_dtx")]
pub use azure_data_cosmos::clients::distributed_transaction::DistributedReadTransaction;
#[cfg(feature = "preview_dtx")]
pub use azure_data_cosmos::clients::distributed_transaction::DistributedWriteTransaction;
pub use azure_data_cosmos::feed::query::FeedScope;
pub use azure_data_cosmos::feed::query::Query;
pub use azure_data_cosmos::error::Result;
pub use azure_data_cosmos::options::routing_strategy::RoutingStrategy;
pub use azure_data_cosmos::error::SubStatusCode;
pub use azure_data_cosmos::models::transactional_batch::TransactionalBatch;
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AccountEndpoint(/* private fields */);
impl AccountEndpoint {
    pub fn into_url(self) -> Url;
    pub fn url(&self) -> &Url;
}
impl Display for AccountEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
impl From<Url> for AccountEndpoint {
    fn from(url: Url) -> Self;
}
impl FromStr for AccountEndpoint {
    type Err = CosmosError;
    fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AccountReference {
}
impl AccountReference {
    #[cfg(feature = "key_auth")]
    pub fn with_authentication_key<impl Into<Secret>: Into<Secret>>(endpoint: AccountEndpoint, key: impl Into<Secret>) -> Self;
    pub fn with_credential(endpoint: AccountEndpoint, credential: Arc<dyn TokenCredential>) -> Self;
}
#[derive(Clone, Debug)]
pub struct CosmosRuntime(/* private fields */);
impl CosmosRuntime {
    pub fn builder() -> CosmosRuntimeBuilder;
}
#[derive(Clone, Debug, Default)]
pub struct CosmosRuntimeBuilder(/* private fields */);
impl CosmosRuntimeBuilder {
    pub async fn build(self) -> crate::Result<CosmosRuntime>;
    pub fn new() -> Self;
    pub fn with_connection_pool(self, options: ConnectionPoolOptions) -> Self;
    pub fn with_cpu_refresh_interval(self, interval: Duration) -> Self;
    pub fn with_default_operation_options(self, options: OperationOptions) -> Self;
    pub fn with_diagnostics_options(self, options: DiagnosticsOptions) -> Self;
    pub fn with_user_agent_suffix(self, suffix: UserAgentSuffix) -> Self;
}
impl From<CosmosDriverRuntimeBuilder> for CosmosRuntimeBuilder {
    fn from(value: CosmosDriverRuntimeBuilder) -> Self;
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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ResourceId(/* private fields */);
impl ResourceId {
    pub fn as_str(&self) -> &str;
}
impl AsRef<str> for ResourceId {
    fn as_ref(&self) -> &str;
}
impl Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
impl From<&ResourceId> for ResourceIdentity {
    fn from(rid: &ResourceId) -> Self;
}
impl From<&String> for ResourceId {
    fn from(rid: &String) -> Self;
}
impl From<&str> for ResourceId {
    fn from(rid: &str) -> Self;
}
impl From<ResourceId> for ResourceIdentity {
    fn from(rid: ResourceId) -> Self;
}
impl From<String> for ResourceId {
    fn from(rid: String) -> Self;
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CosmosCredential {
    TokenCredential(std::sync::Arc<dyn TokenCredential>),
    #[cfg(feature = "key_auth")]
    MasterKey(azure_core::credentials::Secret),
}
impl From<Arc<dyn TokenCredential>> for CosmosCredential {
    fn from(credential: Arc<dyn TokenCredential>) -> Self;
}
#[cfg(feature = "key_auth")]
impl From<Secret> for CosmosCredential {
    fn from(key: Secret) -> Self;
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ResourceIdentity {
    Name(String),
    Rid(ResourceId),
}
impl ResourceIdentity {
    pub fn is_rid(&self) -> bool;
}
impl From<&ResourceIdentity> for ResourceIdentity {
    fn from(identity: &ResourceIdentity) -> Self;
}
impl From<&String> for ResourceIdentity {
    fn from(name: &String) -> Self;
}
impl From<&str> for ResourceIdentity {
    fn from(name: &str) -> Self;
}
impl From<String> for ResourceIdentity {
    fn from(name: String) -> Self;
}
pub mod clients {
    #[derive(Clone)]
    pub struct ContainerClient {
    }
    impl ContainerClient {
        #[cfg(feature = "control_plane")]
        pub async fn begin_replace_throughput(&self, throughput: ThroughputProperties, options: Option<ThroughputOptions>) -> crate::Result<ThroughputPoller>;
        pub async fn create_item<T: Serialize, impl Into<PartitionKey>: Into<PartitionKey>>(&self, partition_key: impl Into<PartitionKey>, item_id: &str, item: T, options: Option<ItemWriteOptions>) -> crate::Result<ItemResponse>;
        #[cfg(feature = "control_plane")]
        pub async fn delete(&self, options: Option<DeleteContainerOptions>) -> crate::Result<ResourceResponse<()>>;
        pub async fn delete_item<impl Into<PartitionKey>: Into<PartitionKey>>(&self, partition_key: impl Into<PartitionKey>, item_id: &str, options: Option<ItemWriteOptions>) -> crate::Result<ItemResponse>;
        pub async fn execute_transactional_batch(&self, batch: TransactionalBatch, options: Option<BatchOptions>) -> crate::Result<BatchResponse>;
        pub async fn feed_range_from_partition_key<impl Into<PartitionKey>: Into<PartitionKey>>(&self, partition_key: impl Into<PartitionKey>, options: Option<ReadFeedRangesOptions>) -> crate::Result<Vec<FeedRange>>;
        pub fn get_latest_session_token(&self, feed_ranges_to_session_tokens: &[(FeedRange, SessionToken)], target_feed_range: &FeedRange) -> crate::Result<SessionToken>;
        #[cfg(feature = "preview_patch")]
        pub async fn patch_item<impl Into<PartitionKey>: Into<PartitionKey>>(&self, partition_key: impl Into<PartitionKey>, item_id: &str, patch: PatchInstructions, options: Option<PatchItemOptions>) -> crate::Result<ItemResponse>;
        pub async fn query_change_feed<T: DeserializeOwned + Send + 'static>(&self, scope: FeedScope, start_from: ChangeFeedStartFrom, options: Option<ChangeFeedOptions>) -> crate::Result<ChangeFeedPageIterator<ChangeFeedItem<T>>>;
        pub async fn query_items<T: DeserializeOwned + Send + 'static, impl Into<Query>: Into<Query>>(&self, query: impl Into<Query>, scope: FeedScope, options: Option<QueryOptions>) -> crate::Result<QueryItemIterator<T>>;
        pub async fn read(&self, options: Option<ReadContainerOptions>) -> crate::Result<ResourceResponse<ContainerProperties>>;
        pub async fn read_feed_ranges(&self, options: Option<ReadFeedRangesOptions>) -> crate::Result<Vec<FeedRange>>;
        pub async fn read_item<impl Into<PartitionKey>: Into<PartitionKey>>(&self, partition_key: impl Into<PartitionKey>, item_id: &str, options: Option<ItemReadOptions>) -> crate::Result<ItemResponse>;
        #[cfg(feature = "control_plane")]
        pub async fn read_throughput(&self, options: Option<ThroughputOptions>) -> crate::Result<Option<ThroughputProperties>>;
        #[cfg(feature = "control_plane")]
        pub async fn replace(&self, properties: ContainerProperties, options: Option<ReplaceContainerOptions>) -> crate::Result<ResourceResponse<ContainerProperties>>;
        pub async fn replace_item<T: Serialize, impl Into<PartitionKey>: Into<PartitionKey>>(&self, partition_key: impl Into<PartitionKey>, item_id: &str, item: T, options: Option<ItemWriteOptions>) -> crate::Result<ItemResponse>;
        pub async fn upsert_item<T: Serialize, impl Into<PartitionKey>: Into<PartitionKey>>(&self, partition_key: impl Into<PartitionKey>, item_id: &str, item: T, options: Option<ItemWriteOptions>) -> crate::Result<ItemResponse>;
    }
    #[derive(Clone, Debug)]
    pub struct CosmosClient {
    }
    impl CosmosClient {
        pub fn builder() -> CosmosClientBuilder;
        #[cfg(feature = "preview_dtx")]
        pub async fn commit_distributed_write(&self, transaction: crate::clients::DistributedWriteTransaction) -> crate::Result<crate::clients::DistributedTransactionResponse>;
        #[cfg(feature = "control_plane")]
        pub async fn create_database(&self, id: &str, options: Option<CreateDatabaseOptions>) -> crate::Result<ResourceResponse<DatabaseProperties>>;
        pub fn database_client<impl Into<ResourceIdentity>: Into<ResourceIdentity>>(&self, database: impl Into<ResourceIdentity>) -> DatabaseClient;
        pub fn endpoint(&self) -> &Url;
        #[cfg(feature = "preview_dtx")]
        pub async fn execute_distributed_read(&self, transaction: crate::clients::DistributedReadTransaction) -> crate::Result<crate::clients::DistributedTransactionResponse>;
        #[cfg(feature = "control_plane")]
        pub async fn query_databases<impl Into<Query>: Into<Query>>(&self, query: impl Into<Query>, options: Option<QueryDatabasesOptions>) -> crate::Result<QueryItemIterator<DatabaseProperties>>;
    }
    #[derive(Default)]
    pub struct CosmosClientBuilder {
    }
    impl CosmosClientBuilder {
        pub async fn build(self, account: AccountReference, routing_strategy: RoutingStrategy) -> crate::Result<CosmosClient>;
        pub fn new() -> Self;
        pub fn register_throughput_control_group(self, group: ThroughputControlGroupOptions) -> crate::Result<Self>;
        pub fn with_backup_endpoints(self, endpoints: Vec<crate::AccountEndpoint>) -> Self;
        pub fn with_binary_encoding_options(self, options: BinaryEncodingOptions) -> Self;
        pub fn with_default_operation_options(self, options: OperationOptions) -> Self;
        pub fn with_diagnostics_handler(self, handler: Arc<dyn DiagnosticsHandler>) -> Self;
        #[cfg(feature = "fault_injection")]
        pub fn with_fault_injection_rules(self, rules: Vec<Arc<azure_data_cosmos_driver::fault_injection::FaultInjectionRule>>) -> crate::Result<Self>;
        pub fn with_partition_failover_options(self, options: PartitionFailoverOptions) -> Self;
        pub fn with_partition_key_range_cache_enabled(self, enabled: bool) -> Self;
        pub fn with_runtime(self, runtime: CosmosRuntime) -> Self;
        pub fn with_user_agent_suffix(self, suffix: UserAgentSuffix) -> Self;
    }
    pub struct DatabaseClient {
    }
    impl DatabaseClient {
        #[cfg(feature = "control_plane")]
        pub async fn begin_replace_throughput(&self, throughput: ThroughputProperties, options: Option<ThroughputOptions>) -> crate::Result<ThroughputPoller>;
        pub async fn container_client<impl Into<ResourceIdentity>: Into<ResourceIdentity>>(&self, container: impl Into<ResourceIdentity>, options: Option<ContainerClientOptions>) -> crate::Result<ContainerClient>;
        #[cfg(feature = "control_plane")]
        pub async fn create_container(&self, properties: ContainerProperties, options: Option<CreateContainerOptions>) -> crate::Result<ResourceResponse<ContainerProperties>>;
        #[cfg(feature = "control_plane")]
        pub async fn delete(&self, options: Option<DeleteDatabaseOptions>) -> crate::Result<ResourceResponse<()>>;
        pub fn id(&self) -> &ResourceIdentity;
        pub fn name(&self) -> Option<&str>;
        #[cfg(feature = "control_plane")]
        pub async fn query_containers<impl Into<Query>: Into<Query>>(&self, query: impl Into<Query>, options: Option<QueryContainersOptions>) -> crate::Result<QueryItemIterator<ContainerProperties>>;
        #[cfg(feature = "control_plane")]
        pub async fn read(&self, options: Option<ReadDatabaseOptions>) -> crate::Result<ResourceResponse<DatabaseProperties>>;
        #[cfg(feature = "control_plane")]
        pub async fn read_throughput(&self, options: Option<ThroughputOptions>) -> crate::Result<Option<ThroughputProperties>>;
        pub fn rid(&self) -> Option<&ResourceId>;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Debug)]
    pub struct DistributedReadTransaction {
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedReadTransaction {
        pub fn new() -> Self;
        pub fn read_item<impl Into<PartitionKey>: Into<PartitionKey>, impl Into<std::borrow::Cow<'static, str>>: Into<std::borrow::Cow<'static, str>>>(self, container: &ContainerClient, partition_key: impl Into<PartitionKey>, item_id: impl Into<std::borrow::Cow<'static, str>>, options: Option<DistributedTransactionOperationOptions>) -> Self;
    }
    #[cfg(feature = "preview_dtx")]
    impl Default for DistributedReadTransaction {
        fn default() -> Self;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct DistributedTransactionOperationOptions {
        pub session_token: Option<crate::options::SessionToken>,
        pub precondition: Option<crate::options::Precondition>,
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedTransactionOperationOptions {
        pub fn with_precondition(self, precondition: Precondition) -> Self;
        pub fn with_session_token<impl Into<SessionToken>: Into<SessionToken>>(self, session_token: impl Into<SessionToken>) -> Self;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Copy, Debug)]
    pub struct DistributedTransactionOperationResult<'a> {
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedTransactionOperationResult<'_> {
        pub fn etag(&self) -> Option<&azure_core::http::Etag>;
        pub fn index(&self) -> usize;
        pub fn is_completed_status_code(&self) -> bool;
        pub fn is_success_status_code(&self) -> bool;
        pub fn partition_key_range_id(&self) -> Option<&str>;
        pub fn request_charge(&self) -> Option<f64>;
        pub fn resource<T: DeserializeOwned>(&self) -> crate::Result<Option<T>>;
        pub fn session_token(&self) -> Option<&SessionToken>;
        pub fn status_code(&self) -> azure_core::http::StatusCode;
        pub fn sub_status_code(&self) -> Option<crate::SubStatusCode>;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct DistributedTransactionPatchOperationOptions {
        pub session_token: Option<crate::options::SessionToken>,
        pub precondition: Option<crate::options::Precondition>,
        pub filter_predicate: Option<std::borrow::Cow<'static, str>>,
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedTransactionPatchOperationOptions {
        pub fn with_filter_predicate<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, predicate: impl Into<Cow<'static, str>>) -> Self;
        pub fn with_precondition(self, precondition: Precondition) -> Self;
        pub fn with_session_token<impl Into<SessionToken>: Into<SessionToken>>(self, session_token: impl Into<SessionToken>) -> Self;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Debug)]
    pub struct DistributedTransactionResponse {
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedTransactionResponse {
        pub fn activity_id(&self) -> Option<&str>;
        pub fn diagnostic_string(&self) -> Option<&str>;
        pub fn diagnostics(&self) -> Option<Arc<DiagnosticsContext>>;
        pub fn error_message(&self) -> Option<&str>;
        pub fn headers(&self) -> &ResponseHeaders;
        pub fn idempotency_token(&self) -> String;
        pub fn is_completed_status_code(&self) -> bool;
        pub fn is_empty(&self) -> bool;
        pub fn is_retriable(&self) -> bool;
        pub fn is_success_status_code(&self) -> bool;
        pub fn len(&self) -> usize;
        pub fn operation_result(&self, index: usize) -> Option<DistributedTransactionOperationResult<'_>>;
        pub fn request_charge(&self) -> Option<f64>;
        pub fn retry_after_ms(&self) -> Option<u64>;
        pub fn status(&self) -> crate::CosmosStatus;
    }
    #[cfg(feature = "preview_dtx")]
    #[derive(Clone, Debug)]
    pub struct DistributedWriteTransaction {
    }
    #[cfg(feature = "preview_dtx")]
    impl DistributedWriteTransaction {
        pub fn create_item<T: Serialize, impl Into<PartitionKey>: Into<PartitionKey>, impl Into<std::borrow::Cow<'static, str>>: Into<std::borrow::Cow<'static, str>>>(self, container: &ContainerClient, partition_key: impl Into<PartitionKey>, item_id: impl Into<std::borrow::Cow<'static, str>>, item: T, options: Option<DistributedTransactionOperationOptions>) -> crate::Result<Self>;
        pub fn delete_item<impl Into<PartitionKey>: Into<PartitionKey>, impl Into<std::borrow::Cow<'static, str>>: Into<std::borrow::Cow<'static, str>>>(self, container: &ContainerClient, partition_key: impl Into<PartitionKey>, item_id: impl Into<std::borrow::Cow<'static, str>>, options: Option<DistributedTransactionOperationOptions>) -> Self;
        pub fn new() -> Self;
        pub fn patch_item<impl Into<PartitionKey>: Into<PartitionKey>, impl Into<std::borrow::Cow<'static, str>>: Into<std::borrow::Cow<'static, str>>>(self, container: &ContainerClient, partition_key: impl Into<PartitionKey>, item_id: impl Into<std::borrow::Cow<'static, str>>, patch: PatchInstructions, options: Option<DistributedTransactionPatchOperationOptions>) -> crate::Result<Self>;
        pub fn replace_item<T: Serialize, impl Into<PartitionKey>: Into<PartitionKey>, impl Into<std::borrow::Cow<'static, str>>: Into<std::borrow::Cow<'static, str>>>(self, container: &ContainerClient, partition_key: impl Into<PartitionKey>, item_id: impl Into<std::borrow::Cow<'static, str>>, item: T, options: Option<DistributedTransactionOperationOptions>) -> crate::Result<Self>;
        pub fn upsert_item<T: Serialize, impl Into<PartitionKey>: Into<PartitionKey>, impl Into<std::borrow::Cow<'static, str>>: Into<std::borrow::Cow<'static, str>>>(self, container: &ContainerClient, partition_key: impl Into<PartitionKey>, item_id: impl Into<std::borrow::Cow<'static, str>>, item: T, options: Option<DistributedTransactionOperationOptions>) -> crate::Result<Self>;
    }
    #[cfg(feature = "preview_dtx")]
    impl Default for DistributedWriteTransaction {
        fn default() -> Self;
    }
    #[cfg(feature = "control_plane")]
    pub struct ThroughputPoller {
    }
    #[cfg(feature = "control_plane")]
    impl IntoFuture for ThroughputPoller {
        type IntoFuture = Pin<Box<dyn Future<Output = Result<ResourceResponse<ThroughputProperties>, CosmosError>> + Send>>;
        type Output = Result<ResourceResponse<ThroughputProperties>, CosmosError>;
        fn into_future(self) -> <Self as >::IntoFuture;
    }
    #[cfg(feature = "control_plane")]
    impl Stream for ThroughputPoller {
        type Item = Result<ResourceResponse<ThroughputProperties>, CosmosError>;
        fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Option<<Self as >::Item>>;
    }
}
pub mod diagnostics {
    #[cfg(feature = "metrics")]
    pub use azure_data_cosmos::diagnostics::metrics::handler::CosmosMetricsHandler;
    #[cfg(feature = "metrics")]
    pub use azure_data_cosmos::diagnostics::metrics::options::MetricsOptions;
    pub struct ClientLifetimeToken {
    }
    impl ClientLifetimeToken {
        pub fn new<impl FnOnce() + Send + Sync + 'static: FnOnce() + Send + Sync + 'static>(on_drop: impl FnOnce() + Send + Sync + 'static) -> Self;
    }
    impl Debug for ClientLifetimeToken {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Drop for ClientLifetimeToken {
        fn drop(&mut self);
    }
    #[derive(Clone, Debug)]
    pub struct CosmosClientInfo {
    }
    impl CosmosClientInfo {
        pub fn server_address(&self) -> Option<&str>;
        pub fn server_port(&self) -> Option<u16>;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct CosmosOperationContext {
    }
    impl CosmosOperationContext {
        pub fn connection_mode(&self) -> Option<&str>;
        pub fn consistency_level(&self) -> Option<&str>;
        pub fn container_name(&self) -> Option<&str>;
        pub fn database_name(&self) -> Option<&str>;
        pub fn new() -> Self;
        pub fn operation_name(&self) -> Option<&str>;
        pub fn returned_item_count(&self) -> Option<u64>;
        pub fn server_address(&self) -> Option<&str>;
        #[must_use]
        pub fn with_connection_mode<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, mode: impl Into<Cow<'static, str>>) -> Self;
        #[must_use]
        pub fn with_consistency_level<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, level: impl Into<Cow<'static, str>>) -> Self;
        #[must_use]
        pub fn with_container_name<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, name: impl Into<Cow<'static, str>>) -> Self;
        #[must_use]
        pub fn with_database_name<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, name: impl Into<Cow<'static, str>>) -> Self;
        #[must_use]
        pub fn with_operation_name<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, name: impl Into<Cow<'static, str>>) -> Self;
        #[must_use]
        pub fn with_returned_item_count(self, count: u64) -> Self;
        #[must_use]
        pub fn with_server_address<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, address: impl Into<Cow<'static, str>>) -> Self;
    }
    #[cfg(feature = "distributed_tracing")]
    pub struct CosmosTracingHandler {
    }
    #[cfg(feature = "distributed_tracing")]
    impl CosmosTracingHandler {
        pub fn new() -> Self;
        pub fn should_emit(&self, diagnostics: &DiagnosticsContext) -> bool;
        pub fn thresholds(&self) -> &DiagnosticsThresholds;
        pub fn with_thresholds(thresholds: DiagnosticsThresholds) -> Self;
        pub fn with_thresholds_and_rate_limit(thresholds: DiagnosticsThresholds, rate_limit: RateLimiterConfig) -> Self;
    }
    #[cfg(feature = "distributed_tracing")]
    impl Default for CosmosTracingHandler {
        fn default() -> Self;
    }
    #[cfg(feature = "distributed_tracing")]
    impl DiagnosticsHandler for CosmosTracingHandler {
        fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>);
    }
    #[doc(inline)]
    #[non_exhaustive]
    pub struct DiagnosticsContext {
    }
    #[doc(inline)]
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
    #[doc(inline)]
    impl Clone for DiagnosticsContext {
        fn clone(&self) -> Self;
    }
    #[doc(inline)]
    impl Debug for DiagnosticsContext {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[doc(inline)]
    impl Display for DiagnosticsContext {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[doc(inline)]
    impl Eq for DiagnosticsContext {
    }
    #[doc(inline)]
    impl PartialEq for DiagnosticsContext {
        fn eq(&self, other: &Self) -> bool;
    }
    #[derive(Clone)]
    pub struct DiagnosticsHandlerChain {
    }
    impl DiagnosticsHandlerChain {
        pub fn from_handlers(handlers: Vec<Arc<dyn DiagnosticsHandler>>) -> Self;
        pub fn handlers(&self) -> &[Arc<dyn DiagnosticsHandler>];
        pub fn is_empty(&self) -> bool;
        pub fn len(&self) -> usize;
        pub fn new() -> Self;
        #[must_use]
        pub fn with_handler(&self, handler: Arc<dyn DiagnosticsHandler>) -> Self;
    }
    impl Debug for DiagnosticsHandlerChain {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Default for DiagnosticsHandlerChain {
        fn default() -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[non_exhaustive]
    pub struct DiagnosticsThresholds {
    }
    #[doc(inline)]
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
    #[doc(inline)]
    impl Default for DiagnosticsThresholds {
        fn default() -> Self;
    }
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct RateLimiterConfig {
        pub max_per_window: u32,
        pub window: std::time::Duration,
        pub failure_reserve: u32,
    }
    impl Default for RateLimiterConfig {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct RequestedRegion {
        pub region: crate::options::Region,
        pub reason: RequestedRegionReason,
    }
    impl From<RequestedRegion> for RequestedRegion {
        fn from(driver: azure_data_cosmos_driver::diagnostics::RequestedRegion) -> RequestedRegion;
    }
    pub struct SamplingLogHandler {
    }
    impl SamplingLogHandler {
        pub fn new() -> Self;
        pub fn should_log(&self, diagnostics: &DiagnosticsContext) -> bool;
        pub fn thresholds(&self) -> &DiagnosticsThresholds;
        pub fn with_handler(inner: Arc<dyn DiagnosticsHandler>) -> Self;
        pub fn with_thresholds(thresholds: DiagnosticsThresholds) -> Self;
        pub fn with_thresholds_and_handler(thresholds: DiagnosticsThresholds, inner: Arc<dyn DiagnosticsHandler>) -> Self;
        pub fn with_thresholds_and_rate_limit(thresholds: DiagnosticsThresholds, rate_limit: RateLimiterConfig) -> Self;
        pub fn with_thresholds_rate_limit_and_handler(thresholds: DiagnosticsThresholds, rate_limit: RateLimiterConfig, inner: Arc<dyn DiagnosticsHandler>) -> Self;
    }
    impl Default for SamplingLogHandler {
        fn default() -> Self;
    }
    impl DiagnosticsHandler for SamplingLogHandler {
        fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>);
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct TracingLogHandler;
    impl TracingLogHandler {
        pub fn new() -> Self;
    }
    impl DiagnosticsHandler for TracingLogHandler {
        fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>);
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum RequestedRegionReason {
        Initial,
        OperationRetry,
        TransportRetry,
        Hedging,
        RegionFailover,
        CircuitBreakerProbe,
    }
    impl From<ExecutionContext> for RequestedRegionReason {
        fn from(driver: azure_data_cosmos_driver::diagnostics::ExecutionContext) -> RequestedRegionReason;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum ThresholdBreach {
        PointLatency,
        NonPointLatency,
        RequestCharge,
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "snake_case")]
    pub enum TransportKind {
        #[default]
        Gateway,
        GatewayV2,
    }
    #[doc(inline)]
    impl TransportKind {
        pub fn as_str(self) -> &'static str;
        pub fn is_gateway(self) -> bool;
        pub fn is_gateway_v2(self) -> bool;
    }
    #[doc(inline)]
    impl AsRef<str> for TransportKind {
        fn as_ref(&self) -> &str;
    }
    #[doc(inline)]
    impl Display for TransportKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    pub trait DiagnosticsHandler: Send + Sync {
        fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>);
        fn on_client_created(&self, client: &CosmosClientInfo) -> Option<ClientLifetimeToken>;
    }
    #[cfg(feature = "metrics")]
    pub mod metrics {
        pub struct CosmosMetricsHandler {
        }
        impl CosmosMetricsHandler {
            pub fn new() -> Self;
            pub fn with_meter(meter: Meter) -> Self;
            pub fn with_meter_and_options(meter: Meter, options: MetricsOptions) -> Self;
            pub fn with_options(options: MetricsOptions) -> Self;
        }
        impl Debug for CosmosMetricsHandler {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
        }
        impl Default for CosmosMetricsHandler {
            fn default() -> Self;
        }
        impl DiagnosticsHandler for CosmosMetricsHandler {
            fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>);
            fn on_client_created(&self, client: &CosmosClientInfo) -> Option<ClientLifetimeToken>;
        }
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct MetricsOptions {
        }
        impl MetricsOptions {
            pub fn active_instance_metric_enabled(&self) -> bool;
            pub fn extended_attributes_enabled(&self) -> bool;
            pub fn hedged_metric_enabled(&self) -> bool;
            pub fn new() -> Self;
            pub fn request_charge_metric_enabled(&self) -> bool;
            pub fn returned_rows_metric_enabled(&self) -> bool;
            #[must_use]
            pub fn with_active_instance_metric(self, enabled: bool) -> Self;
            #[must_use]
            pub fn with_extended_attributes(self, enabled: bool) -> Self;
            #[must_use]
            pub fn with_hedged_metric(self, enabled: bool) -> Self;
            #[must_use]
            pub fn with_request_charge_metric(self, enabled: bool) -> Self;
            #[must_use]
            pub fn with_returned_rows_metric(self, enabled: bool) -> Self;
        }
        pub mod attributes {
            pub const ATTR_CONNECTION_MODE: &str = attributes::CONNECTION_MODE;
            pub const ATTR_CONSISTENCY_LEVEL: &str = attributes::CONSISTENCY_LEVEL;
            pub const ATTR_CONTACTED_REGIONS: &str = attributes::CONTACTED_REGIONS;
            pub const ATTR_DB_COLLECTION_NAME: &str = attributes::DB_COLLECTION_NAME;
            pub const ATTR_DB_NAMESPACE: &str = attributes::DB_NAMESPACE;
            pub const ATTR_DB_OPERATION_NAME: &str = attributes::DB_OPERATION_NAME;
            pub const ATTR_DB_RESPONSE_STATUS_CODE: &str = attributes::DB_RESPONSE_STATUS_CODE;
            pub const ATTR_DB_SYSTEM_NAME: &str = attributes::DB_SYSTEM_NAME;
            pub const ATTR_ERROR_TYPE: &str = attributes::ERROR_TYPE;
            pub const ATTR_HEDGE_REGION: &str = attributes::HEDGE_REGION;
            pub const ATTR_HEDGE_TERMINAL_STATE: &str = attributes::HEDGE_TERMINAL_STATE;
            pub const ATTR_SERVER_ADDRESS: &str = attributes::SERVER_ADDRESS;
            pub const ATTR_SERVER_PORT: &str = attributes::SERVER_PORT;
            pub const ATTR_SUB_STATUS_CODE: &str = attributes::SUB_STATUS_CODE;
            pub const BUCKETS_OPERATION_DURATION_SECONDS: &[f64] = _;
            pub const BUCKETS_REQUEST_CHARGE_RU: &[f64] = _;
            pub const BUCKETS_RETURNED_ROWS: &[f64] = _;
            pub const DB_SYSTEM_NAME_VALUE: &str = attributes::DB_SYSTEM_NAME_VALUE;
            pub const ERROR_TYPE_OTHER: &str = attributes::ERROR_TYPE_OTHER;
            pub const METRIC_ACTIVE_INSTANCE_COUNT: &str = "azure.cosmosdb.client.active_instance.count";
            pub const METRIC_OPERATION_DURATION: &str = "db.client.operation.duration";
            pub const METRIC_OPERATION_HEDGED: &str = "azure.cosmosdb.client.operation.hedged";
            pub const METRIC_OPERATION_REQUEST_CHARGE: &str = "azure.cosmosdb.client.operation.request_charge";
            pub const METRIC_RESPONSE_RETURNED_ROWS: &str = "db.client.response.returned_rows";
            pub const UNIT_INSTANCE: &str = "{instance}";
            pub const UNIT_OPERATION: &str = "{operation}";
            pub const UNIT_REQUEST_UNIT: &str = "{request_unit}";
            pub const UNIT_ROW: &str = "{row}";
            pub const UNIT_SECONDS: &str = "s";
        }
    }
}
pub mod error {
    #[derive(Clone)]
    #[repr(transparent)]
    pub struct CosmosError(/* private fields */);
    impl CosmosError {
        pub fn diagnostics(&self) -> Option<Arc<DiagnosticsContext>>;
        #[cfg(feature = "preview_patch")]
        pub fn patch_tracking_id(&self) -> Option<PatchTrackingId>;
        pub fn response(&self) -> Option<&CosmosResponse>;
        pub fn status(&self) -> CosmosStatus;
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
    impl From<CosmosError> for CosmosError {
        fn from(inner: DriverCosmosError) -> Self;
    }
    impl From<CosmosError> for azure_core::Error {
        fn from(err: CosmosError) -> Self;
    }
    impl From<Error> for CosmosError {
        fn from(error: serde_json::Error) -> Self;
    }
    impl From<ParseError> for CosmosError {
        fn from(error: url::ParseError) -> Self;
    }
    pub type CosmosStatus = azure_data_cosmos_driver::error::CosmosStatus;
    pub type Result<T> = std::result::Result<T, CosmosError>;
    pub type SubStatusCode = azure_data_cosmos_driver::error::SubStatusCode;
}
#[cfg(feature = "fault_injection")]
pub mod fault_injection {
    #[doc(inline)]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct CustomResponse {
    }
    #[doc(inline)]
    impl CustomResponse {
        pub fn body(&self) -> &[u8];
        pub fn headers(&self) -> &Headers;
        pub fn status_code(&self) -> StatusCode;
    }
    #[doc(inline)]
    #[non_exhaustive]
    pub struct CustomResponseBuilder {
    }
    #[doc(inline)]
    impl CustomResponseBuilder {
        pub fn build(self) -> CustomResponse;
        pub fn new(status_code: StatusCode) -> Self;
        pub fn with_body<impl Into<Vec<u8>>: Into<Vec<u8>>>(self, body: impl Into<Vec<u8>>) -> Self;
        pub fn with_header<impl Into<HeaderName>: Into<HeaderName>, impl Into<HeaderValue>: Into<HeaderValue>>(self, name: impl Into<HeaderName>, value: impl Into<HeaderValue>) -> Self;
        pub fn with_sub_status(self, code: u16) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct FaultInjectionCondition {
    }
    #[doc(inline)]
    impl FaultInjectionCondition {
        pub fn container_id(&self) -> Option<&str>;
        pub fn operation_type(&self) -> Option<FaultOperationType>;
        pub fn region(&self) -> Option<&Region>;
        pub fn transport_kind(&self) -> Option<TransportKind>;
    }
    #[doc(inline)]
    #[derive(Default)]
    #[non_exhaustive]
    pub struct FaultInjectionConditionBuilder {
    }
    #[doc(inline)]
    impl FaultInjectionConditionBuilder {
        pub fn build(self) -> FaultInjectionCondition;
        pub fn new() -> Self;
        pub fn with_container_id<impl Into<String>: Into<String>>(self, container_id: impl Into<String>) -> Self;
        pub fn with_operation_type(self, operation_type: FaultOperationType) -> Self;
        pub fn with_region(self, region: Region) -> Self;
        pub fn with_transport_kind(self, transport_kind: TransportKind) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct FaultInjectionResult {
    }
    #[doc(inline)]
    impl FaultInjectionResult {
        pub fn custom_response(&self) -> Option<&CustomResponse>;
        pub fn delay(&self) -> Option<Duration>;
        pub fn error_type(&self) -> Option<FaultInjectionErrorType>;
        pub fn probability(&self) -> f32;
    }
    #[doc(inline)]
    #[non_exhaustive]
    pub struct FaultInjectionResultBuilder {
    }
    #[doc(inline)]
    impl FaultInjectionResultBuilder {
        pub fn build(self) -> FaultInjectionResult;
        pub fn new() -> Self;
        pub fn with_custom_response(self, response: CustomResponse) -> Self;
        pub fn with_delay(self, delay: Duration) -> Self;
        pub fn with_error(self, error_type: FaultInjectionErrorType) -> Self;
        pub fn with_probability(self, probability: f32) -> Self;
    }
    #[doc(inline)]
    impl Default for FaultInjectionResultBuilder {
        fn default() -> Self;
    }
    #[doc(inline)]
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct FaultInjectionRule {
    }
    #[doc(inline)]
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
    #[doc(inline)]
    #[non_exhaustive]
    pub struct FaultInjectionRuleBuilder {
    }
    #[doc(inline)]
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
    #[doc(inline)]
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
    #[doc(inline)]
    impl Display for FaultInjectionErrorType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[doc(inline)]
    impl FromStr for FaultInjectionErrorType {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[doc(inline)]
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
    #[doc(inline)]
    impl FaultOperationType {
        pub fn as_str(&self) -> &'static str;
        pub fn from_operation_and_resource(operation_type: &OperationType, resource_type: &ResourceType) -> Option<Self>;
    }
    #[doc(inline)]
    impl Display for FaultOperationType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[doc(inline)]
    impl FromStr for FaultOperationType {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
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
}
pub mod feed {
    #[pin_project]
    pub struct ChangeFeedPageIterator<T: Send> {
    }
    impl<T: Send + DeserializeOwned + 'static> ChangeFeedPageIterator<T> {
        pub fn to_continuation_token(&self) -> crate::Result<ContinuationToken>;
    }
    impl<T: Send + DeserializeOwned + 'static> Stream for ChangeFeedPageIterator<T> {
        type Item = Result<FeedPage<T>, CosmosError>;
        fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Option<<Self as >::Item>>;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ContinuationToken(/* private fields */);
    #[doc(inline)]
    impl ContinuationToken {
        pub fn as_str(&self) -> &str;
        pub fn from_string(token: String) -> Self;
    }
    #[doc(inline)]
    impl Serialize for ContinuationToken {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>;
    }
    #[doc(inline)]
    impl<'de> Deserialize<'de> for ContinuationToken {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, <D as >::Error>;
    }
    #[derive(Debug)]
    pub struct FeedPage<T> {
    }
    impl<T> FeedPage<T> {
        pub fn diagnostics(&self) -> Arc<DiagnosticsContext>;
        pub fn headers(&self) -> &ResponseHeaders;
        pub fn into_items(self) -> Vec<T>;
        pub fn items(&self) -> &[T];
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct FeedRange(/* private fields */);
    #[doc(inline)]
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
    #[doc(inline)]
    impl Display for FeedRange {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[doc(inline)]
    impl FromStr for FeedRange {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[doc(inline)]
    impl Serialize for FeedRange {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    #[doc(inline)]
    impl TryFrom<&PartitionKeyRange> for FeedRange {
        type Error = CosmosError;
        fn try_from(pkr: &PartitionKeyRange) -> Result<Self, <Self as >::Error>;
    }
    #[doc(inline)]
    impl<'de> Deserialize<'de> for FeedRange {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    #[derive(Clone, Debug, serde::Serialize)]
    pub struct Query {
    }
    impl Query {
        pub fn append_text(self, text: &str) -> Self;
        pub fn with_parameter<impl Into<String>: Into<String>, impl Serialize: Serialize>(self, name: impl Into<String>, value: impl Serialize) -> crate::Result<Self>;
        pub fn with_text<impl Into<String>: Into<String>>(self, text: impl Into<String>) -> Self;
    }
    impl<T: Into<String>> From<T> for Query {
        fn from(value: T) -> Self;
    }
    #[derive(Debug)]
    pub struct QueryFeedPage<T> {
    }
    impl<T> QueryFeedPage<T> {
        pub fn as_feed_page(&self) -> &FeedPage<T>;
        pub fn diagnostics(&self) -> Arc<DiagnosticsContext>;
        pub fn headers(&self) -> &ResponseHeaders;
        pub fn index_metrics(&self) -> Option<&str>;
        pub fn into_items(self) -> Vec<T>;
        pub fn items(&self) -> &[T];
        pub fn query_metrics(&self) -> Option<&str>;
    }
    #[pin_project]
    pub struct QueryItemIterator<T: Send> {
    }
    impl<T: Send + DeserializeOwned + 'static> QueryItemIterator<T> {
        pub fn into_pages(self) -> QueryPageIterator<T>;
    }
    impl<T: Send + DeserializeOwned + 'static> Stream for QueryItemIterator<T> {
        type Item = Result<T, CosmosError>;
        fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Option<<Self as >::Item>>;
    }
    #[pin_project]
    pub struct QueryPageIterator<T: Send> {
    }
    impl<T: Send + DeserializeOwned + 'static> QueryPageIterator<T> {
        pub fn to_continuation_token(&self) -> crate::Result<ContinuationToken>;
    }
    impl<T: Send + DeserializeOwned + 'static> Stream for QueryPageIterator<T> {
        type Item = Result<QueryFeedPage<T>, CosmosError>;
        fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Option<<Self as >::Item>>;
    }
    #[derive(Clone)]
    #[non_exhaustive]
    pub enum FeedScope {
        Partition(azure_data_cosmos_driver::models::PartitionKey),
        Range(azure_data_cosmos_driver::models::FeedRange),
    }
    impl FeedScope {
        pub fn full_container() -> Self;
        pub fn partition<impl Into<PartitionKey>: Into<PartitionKey>>(pk: impl Into<PartitionKey>) -> Self;
        pub fn range<impl Into<FeedRange>: Into<FeedRange>>(fr: impl Into<FeedRange>) -> Self;
    }
}
pub mod models {
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct BatchResponse {
    }
    impl BatchResponse {
        pub fn diagnostics(&self) -> Arc<DiagnosticsContext>;
        pub fn headers(&self) -> &ResponseHeaders;
        pub fn into_body(self) -> ResponseBody;
        pub fn into_model(self) -> crate::Result<TransactionalBatchResponse>;
        pub fn status(&self) -> CosmosStatus;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct ChangeFeedItem<T> {
    }
    impl<T> ChangeFeedItem<T> {
        pub fn current(&self) -> Option<&T>;
        pub fn metadata(&self) -> Option<&ChangeFeedMetadata>;
        pub fn operation_type(&self) -> Option<ChangeFeedOperationType>;
        pub fn previous(&self) -> Option<&T>;
    }
    impl<'de, T> Deserialize<'de> for ChangeFeedItem<T> where T: DeserializeOwned {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, serde::Deserialize)]
    #[non_exhaustive]
    pub struct ChangeFeedMetadata {
    }
    impl ChangeFeedMetadata {
        pub fn conflict_resolution_timestamp(&self) -> Option<Duration>;
        pub fn id(&self) -> Option<&str>;
        pub fn lsn(&self) -> Option<LogicalSequenceNumber>;
        pub fn operation_type(&self) -> Option<ChangeFeedOperationType>;
        pub fn partition_key(&self) -> Option<&serde_json::Value>;
        pub fn previous_image_lsn(&self) -> Option<LogicalSequenceNumber>;
        pub fn time_to_live_expired(&self) -> Option<bool>;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ChangeFeedPolicy {
    }
    impl ChangeFeedPolicy {
        pub fn retention_duration(&self) -> Option<Duration>;
        pub fn with_retention_duration(self, retention: Duration) -> Self;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(transparent)]
    pub struct CompositeIndex {
        pub properties: Vec<CompositeIndexProperty>,
    }
    impl CompositeIndex {
        pub fn with_property(self, property: CompositeIndexProperty) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct CompositeIndexProperty {
        pub path: String,
        pub order: CompositeIndexOrder,
    }
    impl CompositeIndexProperty {
        pub fn new<impl Into<String>: Into<String>>(path: impl Into<String>, order: CompositeIndexOrder) -> Self;
        pub fn with_order(self, order: CompositeIndexOrder) -> Self;
        pub fn with_path<impl Into<String>: Into<String>>(self, path: impl Into<String>) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct ConflictResolutionPolicy {
        pub mode: ConflictResolutionMode,
        #[serde(rename = "conflictResolutionPath")]
        pub resolution_path: String,
        #[serde(rename = "conflictResolutionProcedure")]
        pub resolution_procedure: String,
    }
    impl ConflictResolutionPolicy {
        pub fn new(mode: ConflictResolutionMode) -> Self;
        pub fn with_resolution_path<impl Into<String>: Into<String>>(self, resolution_path: impl Into<String>) -> Self;
        pub fn with_resolution_procedure<impl Into<String>: Into<String>>(self, resolution_procedure: impl Into<String>) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct ContainerProperties {
        pub id: std::borrow::Cow<'static, str>,
        pub partition_key: crate::models::PartitionKeyDefinition,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub indexing_policy: Option<crate::models::IndexingPolicy>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unique_key_policy: Option<UniqueKeyPolicy>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub conflict_resolution_policy: Option<ConflictResolutionPolicy>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub vector_embedding_policy: Option<VectorEmbeddingPolicy>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub full_text_policy: Option<FullTextPolicy>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub change_feed_policy: Option<ChangeFeedPolicy>,
        #[serde(default)]
        #[serde(skip_serializing_if = "TimeToLive::is_forever")]
        pub default_ttl: TimeToLive,
        #[serde(default)]
        #[serde(skip_serializing_if = "TimeToLive::is_forever")]
        pub analytical_storage_ttl: TimeToLive,
        #[serde(flatten)]
        pub system_properties: crate::models::SystemProperties,
    }
    impl ContainerProperties {
        pub fn new<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(id: impl Into<Cow<'static, str>>, partition_key: PartitionKeyDefinition) -> Self;
        pub fn with_analytical_storage_ttl<impl Into<TimeToLive>: Into<TimeToLive>>(self, analytical_storage_ttl: impl Into<TimeToLive>) -> Self;
        pub fn with_change_feed_policy(self, change_feed_policy: ChangeFeedPolicy) -> Self;
        pub fn with_conflict_resolution_policy(self, conflict_resolution_policy: ConflictResolutionPolicy) -> Self;
        pub fn with_default_ttl<impl Into<TimeToLive>: Into<TimeToLive>>(self, default_ttl: impl Into<TimeToLive>) -> Self;
        pub fn with_full_text_policy(self, full_text_policy: FullTextPolicy) -> Self;
        pub fn with_indexing_policy(self, indexing_policy: IndexingPolicy) -> Self;
        pub fn with_unique_key_policy(self, unique_key_policy: UniqueKeyPolicy) -> Self;
        pub fn with_vector_embedding_policy(self, vector_embedding_policy: VectorEmbeddingPolicy) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct ContainerReference(/* private fields */);
    #[doc(inline)]
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
    #[doc(inline)]
    impl Eq for ContainerReference {
    }
    #[doc(inline)]
    impl Hash for ContainerReference {
        fn hash<H: Hasher>(&self, state: &mut H);
    }
    #[doc(inline)]
    impl PartialEq for ContainerReference {
        fn eq(&self, other: &Self) -> bool;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct CosmosStatus {
    }
    #[doc(inline)]
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
    #[doc(inline)]
    impl Debug for CosmosStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[doc(inline)]
    impl Display for CosmosStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[doc(inline)]
    impl From<CosmosStatus> for azure_core::http::StatusCode {
        fn from(s: CosmosStatus) -> Self;
    }
    #[doc(inline)]
    impl From<CosmosStatus> for u16 {
        fn from(s: CosmosStatus) -> Self;
    }
    #[doc(inline)]
    impl PartialEq<CosmosStatus> for azure_core::http::StatusCode {
        fn eq(&self, other: &CosmosStatus) -> bool;
    }
    #[doc(inline)]
    impl PartialEq<StatusCode> for CosmosStatus {
        fn eq(&self, other: &StatusCode) -> bool;
    }
    #[doc(inline)]
    impl Serialize for CosmosStatus {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    #[cfg(feature = "control_plane")]
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct DatabaseProperties {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(flatten)]
        pub system_properties: crate::models::SystemProperties,
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq)]
    pub struct EffectivePartitionKey(/* private fields */);
    #[doc(inline)]
    impl EffectivePartitionKey {
        const MAX: Self = _;
        const MIN: Self = _;
        pub fn to_hex(&self) -> String;
    }
    #[doc(inline)]
    impl Display for EffectivePartitionKey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[doc(inline)]
    impl From<&str> for EffectivePartitionKey {
        fn from(s: &str) -> Self;
    }
    #[doc(inline)]
    impl From<String> for EffectivePartitionKey {
        fn from(s: String) -> Self;
    }
    #[doc(inline)]
    impl Hash for EffectivePartitionKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H);
    }
    #[doc(inline)]
    impl Ord for EffectivePartitionKey {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering;
    }
    #[doc(inline)]
    impl PartialEq<&str> for EffectivePartitionKey {
        fn eq(&self, other: &&str) -> bool;
    }
    #[doc(inline)]
    impl PartialEq<str> for EffectivePartitionKey {
        fn eq(&self, other: &str) -> bool;
    }
    #[doc(inline)]
    impl PartialEq for EffectivePartitionKey {
        fn eq(&self, other: &Self) -> bool;
    }
    #[doc(inline)]
    impl PartialOrd for EffectivePartitionKey {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>;
    }
    #[doc(inline)]
    impl Serialize for EffectivePartitionKey {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    #[doc(inline)]
    impl<'de> Deserialize<'de> for EffectivePartitionKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct FullTextIndex {
        pub path: String,
    }
    impl FullTextIndex {
        pub fn new<impl Into<String>: Into<String>>(path: impl Into<String>) -> Self;
        pub fn with_path<impl Into<String>: Into<String>>(self, path: impl Into<String>) -> Self;
    }
    impl<T: Into<String>> From<T> for FullTextIndex {
        fn from(value: T) -> Self;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct FullTextPath {
        pub path: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub language: Option<String>,
    }
    impl FullTextPath {
        pub fn new<impl Into<String>: Into<String>>(path: impl Into<String>) -> Self;
        pub fn with_language<impl Into<String>: Into<String>>(self, language: impl Into<String>) -> Self;
        pub fn with_path<impl Into<String>: Into<String>>(self, path: impl Into<String>) -> Self;
    }
    impl<T: Into<String>> From<T> for FullTextPath {
        fn from(value: T) -> Self;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct FullTextPolicy {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_language: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub full_text_paths: Vec<FullTextPath>,
    }
    impl FullTextPolicy {
        pub fn new<impl Into<String>: Into<String>>(default_language: impl Into<String>) -> Self;
        pub fn with_default_language<impl Into<String>: Into<String>>(self, default_language: impl Into<String>) -> Self;
        pub fn with_full_text_path<impl Into<FullTextPath>: Into<FullTextPath>>(self, full_text_path: impl Into<FullTextPath>) -> Self;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct IndexingPolicy {
        #[serde(default)]
        pub automatic: bool,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub indexing_mode: Option<IndexingMode>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub included_paths: Vec<PropertyPath>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub excluded_paths: Vec<PropertyPath>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub spatial_indexes: Vec<SpatialIndex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub composite_indexes: Vec<CompositeIndex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub vector_indexes: Vec<VectorIndex>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub full_text_indexes: Vec<FullTextIndex>,
    }
    impl IndexingPolicy {
        pub fn with_composite_index(self, composite_index: CompositeIndex) -> Self;
        pub fn with_excluded_path<impl Into<PropertyPath>: Into<PropertyPath>>(self, excluded_path: impl Into<PropertyPath>) -> Self;
        pub fn with_full_text_index<impl Into<FullTextIndex>: Into<FullTextIndex>>(self, full_text_index: impl Into<FullTextIndex>) -> Self;
        pub fn with_included_path<impl Into<PropertyPath>: Into<PropertyPath>>(self, included_path: impl Into<PropertyPath>) -> Self;
        pub fn with_indexing_mode(self, indexing_mode: IndexingMode) -> Self;
        pub fn with_spatial_index(self, spatial_index: SpatialIndex) -> Self;
        pub fn with_vector_index(self, vector_index: VectorIndex) -> Self;
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct ItemResponse {
    }
    impl ItemResponse {
        pub fn diagnostics(&self) -> Arc<DiagnosticsContext>;
        pub fn headers(&self) -> &ResponseHeaders;
        pub fn into_body(self) -> ResponseBody;
        pub fn into_model<T: DeserializeOwned>(self) -> crate::Result<T>;
        #[cfg(feature = "preview_patch")]
        pub fn patch_tracking_id(&self) -> Option<PatchTrackingId>;
        pub fn status(&self) -> CosmosStatus;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize)]
    #[serde(transparent)]
    pub struct LogicalSequenceNumber(/* private fields */);
    impl LogicalSequenceNumber {
        pub fn value(&self) -> i64;
    }
    impl From<LogicalSequenceNumber> for i64 {
        fn from(value: LogicalSequenceNumber) -> Self;
    }
    impl From<i64> for LogicalSequenceNumber {
        fn from(value: i64) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct PartitionKey(/* private fields */);
    #[doc(inline)]
    impl PartitionKey {
        const NULL: PartitionKeyValue = PartitionKeyValue::NULL;
        const UNDEFINED: PartitionKeyValue = PartitionKeyValue::UNDEFINED;
        pub fn is_empty(&self) -> bool;
        pub fn len(&self) -> usize;
        pub fn values(&self) -> &[PartitionKeyValue];
    }
    #[doc(inline)]
    impl AsHeaders for PartitionKey {
        type Error = CosmosError;
        type Iter = IntoIter<(HeaderName, HeaderValue)>;
        fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
    }
    #[doc(inline)]
    impl From<Vec<PartitionKeyValue>> for PartitionKey {
        fn from(values: Vec<PartitionKeyValue>) -> Self;
    }
    #[doc(inline)]
    impl<T1, T2, T3> From<(T1, T2, T3)> for PartitionKey where T1: Into<PartitionKeyValue>, T2: Into<PartitionKeyValue>, T3: Into<PartitionKeyValue> {
        fn from((v1, v2, v3): (T1, T2, T3)) -> Self;
    }
    #[doc(inline)]
    impl<T1, T2> From<(T1, T2)> for PartitionKey where T1: Into<PartitionKeyValue>, T2: Into<PartitionKeyValue> {
        fn from((v1, v2): (T1, T2)) -> Self;
    }
    #[doc(inline)]
    impl<T: Into<PartitionKeyValue>> From<T> for PartitionKey {
        fn from(value: T) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct PartitionKeyDefinition {
    }
    #[doc(inline)]
    impl PartitionKeyDefinition {
        pub fn is_complete(&self, pk: &PartitionKey) -> bool;
        pub fn kind(&self) -> PartitionKeyKind;
        pub fn new(paths: Vec<Cow<'static, str>>) -> Self;
        pub fn paths(&self) -> &[Cow<'static, str>];
        pub fn version(&self) -> PartitionKeyVersion;
        pub fn with_kind(self, kind: PartitionKeyKind) -> Self;
        pub fn with_version(self, version: PartitionKeyVersion) -> Self;
    }
    #[doc(inline)]
    impl From<&str> for PartitionKeyDefinition {
        fn from(value: &str) -> Self;
    }
    #[doc(inline)]
    impl From<String> for PartitionKeyDefinition {
        fn from(value: String) -> Self;
    }
    #[doc(inline)]
    impl<'de> Deserialize<'de> for PartitionKeyDefinition {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    #[doc(inline)]
    impl<S1: Into<String>, S2: Into<String>, S3: Into<String>> From<(S1, S2, S3)> for PartitionKeyDefinition {
        fn from(value: (S1, S2, S3)) -> Self;
    }
    #[doc(inline)]
    impl<S1: Into<String>, S2: Into<String>> From<(S1, S2)> for PartitionKeyDefinition {
        fn from(value: (S1, S2)) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct PartitionKeyValue(/* private fields */);
    #[doc(inline)]
    impl PartitionKeyValue {
        const INFINITY: Self = _;
        const NULL: Self = _;
        const UNDEFINED: Self = _;
    }
    #[doc(inline)]
    impl From<&'static str> for PartitionKeyValue {
        fn from(value: &'static str) -> Self;
    }
    #[doc(inline)]
    impl From<&String> for PartitionKeyValue {
        fn from(value: &String) -> Self;
    }
    #[doc(inline)]
    impl From<Cow<'static, str>> for PartitionKeyValue {
        fn from(value: Cow<'static, str>) -> Self;
    }
    #[doc(inline)]
    impl From<String> for PartitionKeyValue {
        fn from(value: String) -> Self;
    }
    #[doc(inline)]
    impl From<bool> for PartitionKeyValue {
        fn from(value: bool) -> Self;
    }
    #[doc(inline)]
    impl From<f32> for PartitionKeyValue {
        fn from(value: f32) -> Self;
    }
    #[doc(inline)]
    impl From<f64> for PartitionKeyValue {
        fn from(value: f64) -> Self;
    }
    #[doc(inline)]
    impl From<i16> for PartitionKeyValue {
        fn from(value: i16) -> Self;
    }
    #[doc(inline)]
    impl From<i32> for PartitionKeyValue {
        fn from(value: i32) -> Self;
    }
    #[doc(inline)]
    impl From<i64> for PartitionKeyValue {
        fn from(value: i64) -> Self;
    }
    #[doc(inline)]
    impl From<i8> for PartitionKeyValue {
        fn from(value: i8) -> Self;
    }
    #[doc(inline)]
    impl From<isize> for PartitionKeyValue {
        fn from(value: isize) -> Self;
    }
    #[doc(inline)]
    impl From<u16> for PartitionKeyValue {
        fn from(value: u16) -> Self;
    }
    #[doc(inline)]
    impl From<u32> for PartitionKeyValue {
        fn from(value: u32) -> Self;
    }
    #[doc(inline)]
    impl From<u64> for PartitionKeyValue {
        fn from(value: u64) -> Self;
    }
    #[doc(inline)]
    impl From<u8> for PartitionKeyValue {
        fn from(value: u8) -> Self;
    }
    #[doc(inline)]
    impl From<usize> for PartitionKeyValue {
        fn from(value: usize) -> Self;
    }
    #[doc(inline)]
    impl<T: Into<PartitionKeyValue>> From<Option<T>> for PartitionKeyValue {
        fn from(value: Option<T>) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct PatchInstructions {
        pub operations: Vec<PatchOperation>,
    }
    #[doc(inline)]
    impl PatchInstructions {
        pub fn is_retry_safe(&self) -> bool;
        pub fn new() -> Self;
        pub fn with_operation(self, operation: PatchOperation) -> Self;
    }
    #[doc(inline)]
    impl From<Vec<PatchOperation>> for PatchInstructions {
        fn from(operations: Vec<PatchOperation>) -> Self;
    }
    #[cfg(feature = "preview_patch")]
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct PatchTrackingId(/* private fields */);
    #[cfg(feature = "preview_patch")]
    impl PatchTrackingId {
        pub fn as_uuid(&self) -> Uuid;
        pub fn new() -> Self;
    }
    #[cfg(feature = "preview_patch")]
    impl Default for PatchTrackingId {
        fn default() -> Self;
    }
    #[cfg(feature = "preview_patch")]
    impl Display for PatchTrackingId {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[cfg(feature = "preview_patch")]
    impl From<Uuid> for PatchTrackingId {
        fn from(value: Uuid) -> Self;
    }
    #[cfg(feature = "preview_patch")]
    impl FromStr for PatchTrackingId {
        type Err = Error;
        fn from_str(value: &str) -> Result<Self, <Self as >::Err>;
    }
    #[cfg(feature = "preview_patch")]
    impl Serialize for PatchTrackingId {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    #[cfg(feature = "preview_patch")]
    impl<'de> Deserialize<'de> for PatchTrackingId {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct PropertyPath {
        pub path: String,
    }
    impl PropertyPath {
        pub fn with_path<impl Into<String>: Into<String>>(self, path: impl Into<String>) -> Self;
    }
    impl<T: Into<String>> From<T> for PropertyPath {
        fn from(value: T) -> Self;
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct ResourceResponse<T> {
    }
    impl<T: DeserializeOwned> ResourceResponse<T> {
        pub fn into_model(self) -> crate::Result<T>;
    }
    impl<T> ResourceResponse<T> {
        pub fn diagnostics(&self) -> Arc<DiagnosticsContext>;
        pub fn headers(&self) -> &ResponseHeaders;
        pub fn into_body(self) -> ResponseBody;
        pub fn status(&self) -> CosmosStatus;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct ResponseBody(/* private fields */);
    impl ResponseBody {
        pub fn into_single<T: DeserializeOwned>(self) -> crate::Result<T>;
        pub fn is_empty(&self) -> bool;
        pub fn items(self) -> crate::Result<Vec<Bytes>>;
        pub fn single(self) -> crate::Result<Bytes>;
    }
    impl From<ResponseBody> for ResponseBody {
        fn from(inner: DriverResponseBody) -> Self;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct ResponseHeaders(/* private fields */);
    impl ResponseHeaders {
        pub fn activity_id(&self) -> Option<&ActivityId>;
        pub fn collection_index_transformation_progress(&self) -> Option<i64>;
        pub fn collection_lazy_indexing_progress(&self) -> Option<i64>;
        pub fn continuation(&self) -> Option<&str>;
        pub fn correlated_activity_id(&self) -> Option<&str>;
        pub fn etag(&self) -> Option<&Etag>;
        pub fn gateway_version(&self) -> Option<&str>;
        pub fn global_committed_lsn(&self) -> Option<i64>;
        pub fn index_metrics(&self) -> Option<&str>;
        pub fn internal_partition_id(&self) -> Option<&str>;
        pub fn item_count(&self) -> Option<u32>;
        pub fn item_local_lsn(&self) -> Option<u64>;
        pub fn item_lsn(&self) -> Option<u64>;
        pub fn local_lsn(&self) -> Option<u64>;
        pub fn lsn(&self) -> Option<u64>;
        pub fn offer_replace_pending(&self) -> Option<bool>;
        pub fn partition_key_range_id(&self) -> Option<&str>;
        pub fn query_metrics(&self) -> Option<&str>;
        pub fn request_charge(&self) -> Option<&RequestCharge>;
        pub fn resource_quota(&self) -> Option<&str>;
        pub fn resource_usage(&self) -> Option<&str>;
        pub fn retry_after_ms(&self) -> Option<u64>;
        pub fn server_duration_ms(&self) -> Option<f64>;
        pub fn session_token(&self) -> Option<&SessionToken>;
        pub fn substatus(&self) -> Option<&SubStatusCode>;
        pub fn transport_request_id(&self) -> Option<u32>;
    }
    impl From<CosmosResponseHeaders> for ResponseHeaders {
        fn from(inner: DriverCosmosResponseHeaders) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct SpatialIndex {
        pub path: String,
        pub types: Vec<SpatialType>,
    }
    impl SpatialIndex {
        pub fn new<impl Into<String>: Into<String>>(path: impl Into<String>) -> Self;
        pub fn with_type(self, spatial_type: SpatialType) -> Self;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct SystemProperties {
        #[serde(default)]
        #[serde(skip_serializing)]
        #[serde(rename = "_etag")]
        pub etag: Option<azure_core::http::Etag>,
        #[serde(default)]
        #[serde(skip_serializing)]
        #[serde(rename = "_self")]
        pub self_link: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "_rid")]
        pub resource_id: Option<String>,
        #[serde(default)]
        #[serde(rename = "_ts")]
        #[serde(skip_serializing)]
        #[serde(deserialize_with = "deserialize_cosmos_timestamp")]
        pub last_modified: Option<azure_core::time::OffsetDateTime>,
    }
    #[cfg(feature = "control_plane")]
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct ThroughputProperties {
    }
    #[cfg(feature = "control_plane")]
    impl ThroughputProperties {
        pub fn autoscale(starting_maximum_throughput: usize, increment_percent: Option<usize>) -> ThroughputProperties;
        pub fn autoscale_increment(&self) -> Option<usize>;
        pub fn autoscale_maximum(&self) -> Option<usize>;
        pub fn manual(throughput: usize) -> ThroughputProperties;
        pub fn throughput(&self) -> Option<usize>;
    }
    #[derive(Clone, Debug)]
    pub struct TransactionalBatch {
    }
    impl TransactionalBatch {
        pub fn create_item<T: Serialize>(self, item: T) -> crate::Result<Self>;
        pub fn delete_item<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, item_id: impl Into<Cow<'static, str>>, options: Option<BatchDeleteOptions>) -> Self;
        pub fn new<impl Into<PartitionKey>: Into<PartitionKey>>(partition_key: impl Into<PartitionKey>) -> Self;
        pub fn partition_key(&self) -> &PartitionKey;
        pub fn read_item<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, item_id: impl Into<Cow<'static, str>>, options: Option<BatchReadOptions>) -> Self;
        pub fn replace_item<T: Serialize, impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(self, item_id: impl Into<Cow<'static, str>>, item: T, options: Option<BatchReplaceOptions>) -> crate::Result<Self>;
        pub fn upsert_item<T: Serialize>(self, item: T, options: Option<BatchUpsertOptions>) -> crate::Result<Self>;
    }
    #[derive(Clone, Debug, serde::Deserialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct TransactionalBatchOperationResult {
    }
    impl TransactionalBatchOperationResult {
        pub fn etag(&self) -> Option<&str>;
        pub fn into_model<T: serde::de::DeserializeOwned>(&self) -> crate::Result<Option<T>>;
        pub fn is_success(&self) -> bool;
        pub fn request_charge(&self) -> Option<f64>;
        pub fn resource_body(&self) -> Option<&serde_json::value::RawValue>;
        pub fn retry_after_milliseconds(&self) -> Option<u64>;
        pub fn status_code(&self) -> u16;
        pub fn substatus_code(&self) -> Option<u32>;
    }
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct TransactionalBatchResponse {
    }
    impl TransactionalBatchResponse {
        pub fn results(&self) -> &[TransactionalBatchOperationResult];
    }
    impl<'de> Deserialize<'de> for TransactionalBatchResponse {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct UniqueKey {
        pub paths: Vec<String>,
    }
    impl UniqueKey {
        pub fn with_path<impl Into<String>: Into<String>>(self, path: impl Into<String>) -> Self;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct UniqueKeyPolicy {
        pub unique_keys: Vec<UniqueKey>,
    }
    impl UniqueKeyPolicy {
        pub fn with_unique_key(self, unique_key: UniqueKey) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct VectorEmbedding {
        pub path: String,
        pub data_type: VectorDataType,
        pub dimensions: u32,
        pub distance_function: VectorDistanceFunction,
    }
    impl VectorEmbedding {
        pub fn new<impl Into<String>: Into<String>>(path: impl Into<String>, data_type: VectorDataType, dimensions: u32, distance_function: VectorDistanceFunction) -> Self;
        pub fn with_data_type(self, data_type: VectorDataType) -> Self;
        pub fn with_dimensions(self, dimensions: u32) -> Self;
        pub fn with_distance_function(self, distance_function: VectorDistanceFunction) -> Self;
        pub fn with_path<impl Into<String>: Into<String>>(self, path: impl Into<String>) -> Self;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct VectorEmbeddingPolicy {
        #[serde(rename = "vectorEmbeddings")]
        pub embeddings: Vec<VectorEmbedding>,
    }
    impl VectorEmbeddingPolicy {
        pub fn with_embedding(self, embedding: VectorEmbedding) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct VectorIndex {
        pub path: String,
        #[serde(rename = "type")] // "type" is a reserved word in Rust.
        pub index_type: VectorIndexType,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quantizer_type: Option<QuantizerType>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub quantization_byte_size: Option<u32>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub indexing_search_list_size: Option<u32>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub vector_index_shard_key: Vec<String>,
    }
    impl VectorIndex {
        pub fn new<impl Into<String>: Into<String>>(path: impl Into<String>, index_type: VectorIndexType) -> Self;
        pub fn with_index_type(self, index_type: VectorIndexType) -> Self;
        pub fn with_indexing_search_list_size(self, indexing_search_list_size: u32) -> Self;
        pub fn with_path<impl Into<String>: Into<String>>(self, path: impl Into<String>) -> Self;
        pub fn with_quantization_byte_size(self, quantization_byte_size: u32) -> Self;
        pub fn with_quantizer_type(self, quantizer_type: QuantizerType) -> Self;
        pub fn with_shard_key_path<impl Into<String>: Into<String>>(self, path: impl Into<String>) -> Self;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub enum ChangeFeedOperationType {
        Create,
        Replace,
        Delete,
        #[serde(other)]
        Unknown,
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub enum CompositeIndexOrder {
        Ascending,
        Descending,
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "PascalCase")]
    pub enum ConflictResolutionMode {
        LastWriterWins,
        Custom,
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum CosmosNumber {
        Int(i64),
        Float(f64),
    }
    #[doc(inline)]
    impl From<f64> for CosmosNumber {
        fn from(v: f64) -> Self;
    }
    #[doc(inline)]
    impl From<i64> for CosmosNumber {
        fn from(v: i64) -> Self;
    }
    #[doc(inline)]
    impl Serialize for CosmosNumber {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>;
    }
    #[doc(inline)]
    impl<'de> Deserialize<'de> for CosmosNumber {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, <D as >::Error>;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub enum IndexingMode {
        Consistent,
        None,
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub enum PartitionKeyKind {
        #[default]
        Hash,
        MultiHash,
        Range,
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(try_from = "u32", into = "u32")]
    pub enum PartitionKeyVersion {
        V1,
        V2,
    }
    #[doc(inline)]
    impl PartitionKeyVersion {
        pub const fn value(self) -> u32;
    }
    #[doc(inline)]
    impl From<PartitionKeyVersion> for u32 {
        fn from(version: PartitionKeyVersion) -> Self;
    }
    #[doc(inline)]
    impl TryFrom<u32> for PartitionKeyVersion {
        type Error = &'static str;
        fn try_from(value: u32) -> Result<Self, <Self as >::Error>;
    }
    #[doc(inline)]
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
    #[doc(inline)]
    impl PatchOperation {
        pub fn add<impl Into<String>: Into<String>>(path: impl Into<String>, value: Value) -> Self;
        pub fn increment<impl Into<String>: Into<String>, impl Into<CosmosNumber>: Into<CosmosNumber>>(path: impl Into<String>, value: impl Into<CosmosNumber>) -> Self;
        pub fn move_value<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(from: impl Into<String>, path: impl Into<String>) -> Self;
        pub fn path(&self) -> &str;
        pub fn remove<impl Into<String>: Into<String>>(path: impl Into<String>) -> Self;
        pub fn replace<impl Into<String>: Into<String>>(path: impl Into<String>, value: Value) -> Self;
        pub fn set<impl Into<String>: Into<String>>(path: impl Into<String>, value: Value) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub enum QuantizerType {
        Product,
        Spherical,
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "PascalCase")]
    pub enum SpatialType {
        Point,
        Polygon,
        LineString,
        MultiPolygon,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum TimeToLive {
        #[default]
        Forever,
        NoDefault,
        Seconds(u32),
    }
    impl TimeToLive {
        pub fn is_forever(&self) -> bool;
    }
    impl From<u32> for TimeToLive {
        fn from(n: u32) -> Self;
    }
    impl Serialize for TimeToLive {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for TimeToLive {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub enum VectorDataType {
        Float16,
        Float32,
        Uint8,
        Int8,
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub enum VectorDistanceFunction {
        Euclidean,
        Cosine,
        #[serde(rename = "dotproduct")]
        DotProduct,
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub enum VectorIndexType {
        Flat,
        QuantizedFlat,
        DiskANN,
    }
    #[cfg(feature = "preview_patch")]
    pub const DEFAULT_PATCH_TRACKING_CAPACITY: std::num::NonZeroU16 = _;
    #[cfg(feature = "preview_patch")]
    pub const PATCH_TRACKING_PROPERTY: &str = "_azsdkPatchTracking";
    #[cfg(feature = "preview_patch")]
    pub const PATCH_TRACKING_RETENTION: std::time::Duration = _;
}
pub mod options {
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct BatchDeleteOptions {
        pub precondition: Option<azure_data_cosmos_driver::models::Precondition>,
    }
    impl BatchDeleteOptions {
        pub fn with_precondition(self, precondition: Precondition) -> Self;
    }
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct BatchOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
        pub session_token: Option<azure_data_cosmos_driver::models::SessionToken>,
    }
    impl BatchOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
        pub fn with_session_token<impl Into<SessionToken>: Into<SessionToken>>(self, session_token: impl Into<SessionToken>) -> Self;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct BatchReadOptions {
        pub precondition: Option<azure_data_cosmos_driver::models::Precondition>,
    }
    impl BatchReadOptions {
        pub fn with_precondition(self, precondition: Precondition) -> Self;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct BatchReplaceOptions {
        pub precondition: Option<azure_data_cosmos_driver::models::Precondition>,
    }
    impl BatchReplaceOptions {
        pub fn with_precondition(self, precondition: Precondition) -> Self;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct BatchUpsertOptions {
        pub precondition: Option<azure_data_cosmos_driver::models::Precondition>,
    }
    impl BatchUpsertOptions {
        pub fn with_precondition(self, precondition: Precondition) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct BinaryEncodingOptions {
        pub enabled: bool,
        pub request_text_response: bool,
    }
    #[doc(inline)]
    impl BinaryEncodingOptions {
        pub fn new() -> Self;
        pub fn with_enabled(self, enabled: bool) -> Self;
        pub fn with_request_text_response(self, request_text_response: bool) -> Self;
    }
    #[doc(inline)]
    impl Default for BinaryEncodingOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct ChangeFeedOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
        pub feed: crate::options::FeedOptions,
        pub session_token: Option<azure_data_cosmos_driver::models::SessionToken>,
        pub mode: ChangeFeedMode,
    }
    impl ChangeFeedOptions {
        pub fn with_continuation_token(self, token: ContinuationToken) -> Self;
        pub fn with_feed_options(self, feed: FeedOptions) -> Self;
        pub fn with_max_item_count(self, max_item_count: MaxItemCountHint) -> Self;
        pub fn with_mode(self, mode: ChangeFeedMode) -> Self;
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
        pub fn with_session_token<impl Into<SessionToken>: Into<SessionToken>>(self, token: impl Into<SessionToken>) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct ConnectionPoolOptions {
    }
    #[doc(inline)]
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
    #[doc(inline)]
    impl Default for ConnectionPoolOptions {
        fn default() -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct ConnectionPoolOptionsBuilder {
    }
    #[doc(inline)]
    #[automatically_derived]
    impl ConnectionPoolOptionsBuilder {
        pub fn from_env() -> Self;
        pub fn from_env_override() -> Self;
    }
    #[doc(inline)]
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
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct ContainerClientOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct CosmosClientOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    impl CosmosClientOptions {
        pub fn with_diagnostics_handler(self, handler: Arc<dyn DiagnosticsHandler>) -> Self;
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
        pub fn with_user_agent_suffix(self, suffix: UserAgentSuffix) -> Self;
    }
    #[cfg(feature = "control_plane")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct CreateContainerOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    #[cfg(feature = "control_plane")]
    impl CreateContainerOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
        pub fn with_throughput(self, throughput: ThroughputProperties) -> Self;
    }
    #[cfg(feature = "control_plane")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct CreateDatabaseOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    #[cfg(feature = "control_plane")]
    impl CreateDatabaseOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
    }
    #[cfg(feature = "control_plane")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct DeleteContainerOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    #[cfg(feature = "control_plane")]
    impl DeleteContainerOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
    }
    #[cfg(feature = "control_plane")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct DeleteDatabaseOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    #[cfg(feature = "control_plane")]
    impl DeleteDatabaseOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct DiagnosticsOptions {
    }
    #[doc(inline)]
    impl DiagnosticsOptions {
        pub fn builder() -> DiagnosticsOptionsBuilder;
        pub fn default_verbosity(&self) -> DiagnosticsVerbosity;
        pub fn max_request_diagnostics(&self) -> usize;
        pub fn max_summary_size_bytes(&self) -> usize;
    }
    #[doc(inline)]
    impl Default for DiagnosticsOptions {
        fn default() -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct DiagnosticsOptionsBuilder {
    }
    #[doc(inline)]
    #[automatically_derived]
    impl DiagnosticsOptionsBuilder {
        pub fn from_env() -> Self;
    }
    #[doc(inline)]
    impl DiagnosticsOptionsBuilder {
        pub fn build(self) -> crate::error::Result<DiagnosticsOptions>;
        pub fn new() -> Self;
        pub fn with_default_verbosity(self, verbosity: DiagnosticsVerbosity) -> Self;
        pub fn with_max_request_diagnostics(self, max: usize) -> Self;
        pub fn with_max_summary_size_bytes(self, size: usize) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct EndToEndOperationLatencyPolicy {
    }
    #[doc(inline)]
    impl EndToEndOperationLatencyPolicy {
        pub fn new(timeout: Duration) -> Self;
        pub fn timeout(&self) -> Duration;
    }
    #[doc(inline)]
    impl From<Duration> for EndToEndOperationLatencyPolicy {
        fn from(timeout: Duration) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct ExcludedRegions(pub Vec<crate::options::Region>);
    #[doc(inline)]
    impl ExcludedRegions {
        pub fn is_empty(&self) -> bool;
        pub fn iter(&self) -> impl Iterator<Item = &Region>;
        pub fn len(&self) -> usize;
        pub fn new() -> Self;
        pub fn with_region<impl Into<Region>: Into<Region>>(self, region: impl Into<Region>) -> Self;
    }
    #[doc(inline)]
    impl<T: Into<crate::options::Region>> FromIterator<T> for ExcludedRegions {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;
    }
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct FeedOptions {
        pub max_item_count: Option<azure_data_cosmos_driver::models::MaxItemCountHint>,
        pub continuation_token: Option<crate::feed::ContinuationToken>,
        pub max_fan_out: Option<u32>,
    }
    impl FeedOptions {
        pub fn with_continuation_token(self, continuation_token: ContinuationToken) -> Self;
        pub fn with_max_fan_out(self, max_fan_out: u32) -> Self;
        pub fn with_max_item_count(self, max_item_count: MaxItemCountHint) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct HedgeThreshold(/* private fields */);
    #[doc(inline)]
    impl HedgeThreshold {
        pub const fn get(self) -> Duration;
        pub const fn new(duration: Duration) -> Option<Self>;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub struct HedgingStrategy {
    }
    #[doc(inline)]
    impl HedgingStrategy {
        pub const fn new(threshold: HedgeThreshold) -> Self;
        pub const fn threshold(&self) -> HedgeThreshold;
    }
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct ItemReadOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
        pub session_token: Option<azure_data_cosmos_driver::models::SessionToken>,
        pub precondition: Option<azure_data_cosmos_driver::models::Precondition>,
    }
    impl ItemReadOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
        pub fn with_precondition(self, precondition: Precondition) -> Self;
        pub fn with_session_token<impl Into<SessionToken>: Into<SessionToken>>(self, session_token: impl Into<SessionToken>) -> Self;
    }
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct ItemWriteOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
        pub session_token: Option<azure_data_cosmos_driver::models::SessionToken>,
        pub precondition: Option<azure_data_cosmos_driver::models::Precondition>,
    }
    impl ItemWriteOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
        pub fn with_precondition(self, precondition: Precondition) -> Self;
        pub fn with_session_token<impl Into<SessionToken>: Into<SessionToken>>(self, session_token: impl Into<SessionToken>) -> Self;
    }
    #[doc(inline)]
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
    #[doc(inline)]
    #[automatically_derived]
    impl OperationOptions {
        pub fn from_env() -> Self;
        pub fn from_env_override() -> Self;
    }
    #[doc(inline)]
    #[automatically_derived]
    pub struct OperationOptionsBuilder {
    }
    #[doc(inline)]
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
    #[doc(inline)]
    #[automatically_derived]
    pub struct OperationOptionsView<'a> {
    }
    #[doc(inline)]
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
    #[doc(inline)]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct PartitionFailoverOptions {
    }
    #[doc(inline)]
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
    #[doc(inline)]
    impl Default for PartitionFailoverOptions {
        fn default() -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct PartitionFailoverOptionsBuilder {
    }
    #[doc(inline)]
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
    #[cfg(feature = "preview_patch")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct PatchItemOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
        pub strategy: Option<azure_data_cosmos_driver::options::PatchStrategy>,
        pub session_token: Option<azure_data_cosmos_driver::models::SessionToken>,
        pub precondition: Option<azure_data_cosmos_driver::models::Precondition>,
        pub max_attempts: Option<std::num::NonZeroU8>,
        pub tracking_id: Option<crate::models::PatchTrackingId>,
        pub tracking_capacity: Option<std::num::NonZeroU16>,
        pub tracking_retention_seconds: Option<std::num::NonZeroU32>,
    }
    #[cfg(feature = "preview_patch")]
    impl PatchItemOptions {
        pub fn with_max_attempts(self, max_attempts: std::num::NonZeroU8) -> Self;
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
        pub fn with_precondition(self, precondition: Precondition) -> Self;
        pub fn with_session_token<impl Into<SessionToken>: Into<SessionToken>>(self, session_token: impl Into<SessionToken>) -> Self;
        pub fn with_strategy(self, strategy: PatchStrategy) -> Self;
        pub fn with_tracking_capacity(self, capacity: std::num::NonZeroU16) -> Self;
        pub fn with_tracking_id(self, tracking_id: PatchTrackingId) -> Self;
        pub fn with_tracking_retention_seconds(self, retention_seconds: std::num::NonZeroU32) -> Self;
    }
    #[cfg(feature = "control_plane")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct QueryContainersOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    #[cfg(feature = "control_plane")]
    impl QueryContainersOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
    }
    #[cfg(feature = "control_plane")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct QueryDatabasesOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    #[cfg(feature = "control_plane")]
    impl QueryDatabasesOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
    }
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct QueryOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
        pub feed: FeedOptions,
        pub session_token: Option<azure_data_cosmos_driver::models::SessionToken>,
        pub populate_index_metrics: Option<bool>,
        pub populate_query_metrics: Option<bool>,
    }
    impl QueryOptions {
        pub fn with_continuation_token(self, continuation_token: ContinuationToken) -> Self;
        pub fn with_feed_options(self, feed: FeedOptions) -> Self;
        pub fn with_max_item_count(self, max_item_count: MaxItemCountHint) -> Self;
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
        pub fn with_populate_index_metrics(self, enable: bool) -> Self;
        pub fn with_populate_query_metrics(self, enable: bool) -> Self;
        pub fn with_session_token<impl Into<SessionToken>: Into<SessionToken>>(self, session_token: impl Into<SessionToken>) -> Self;
    }
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct ReadContainerOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    impl ReadContainerOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
    }
    #[cfg(feature = "control_plane")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct ReadDatabaseOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    #[cfg(feature = "control_plane")]
    impl ReadDatabaseOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct ReadFeedRangesOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    impl ReadFeedRangesOptions {
        pub fn with_force_refresh(self, force_refresh: bool) -> Self;
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize)]
    #[non_exhaustive]
    #[serde(transparent)]
    pub struct Region {
    }
    #[doc(inline)]
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
    #[doc(inline)]
    impl AsRef<str> for Region {
        fn as_ref(&self) -> &str;
    }
    #[doc(inline)]
    impl Display for Region {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[doc(inline)]
    impl From<&'static str> for Region {
        fn from(name: &'static str) -> Self;
    }
    #[doc(inline)]
    impl From<String> for Region {
        fn from(name: String) -> Self;
    }
    #[doc(inline)]
    impl<'de> Deserialize<'de> for Region {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[cfg(feature = "control_plane")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct ReplaceContainerOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    #[cfg(feature = "control_plane")]
    impl ReplaceContainerOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct SessionToken(pub std::borrow::Cow<'static, str>);
    #[doc(inline)]
    impl SessionToken {
        pub fn as_str(&self) -> &str;
        pub fn merge(&self, other: &Self) -> crate::error::Result<Self>;
        pub fn new<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(value: impl Into<Cow<'static, str>>) -> Self;
    }
    #[doc(inline)]
    impl AsRef<str> for SessionToken {
        fn as_ref(&self) -> &str;
    }
    #[doc(inline)]
    impl<T: Into<std::borrow::Cow<'static, str>>> From<T> for SessionToken {
        fn from(value: T) -> Self;
    }
    #[doc(inline)]
    impl Display for SessionToken {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct ThrottlingRetryOptions {
        pub max_retry_count: Option<u32>,
        pub max_retry_wait_time: Option<std::time::Duration>,
    }
    #[doc(inline)]
    #[automatically_derived]
    impl ThrottlingRetryOptions {
        pub fn from_env() -> Self;
    }
    #[doc(inline)]
    #[automatically_derived]
    pub struct ThrottlingRetryOptionsBuilder {
    }
    #[doc(inline)]
    #[automatically_derived]
    impl ThrottlingRetryOptionsBuilder {
        #[must_use]
        pub fn build(self) -> ThrottlingRetryOptions;
        pub fn new() -> Self;
        pub fn with_max_retry_count(self, value: u32) -> Self;
        pub fn with_max_retry_wait_time(self, value: Duration) -> Self;
    }
    #[doc(inline)]
    #[automatically_derived]
    pub struct ThrottlingRetryOptionsView<'a> {
    }
    #[doc(inline)]
    #[automatically_derived]
    impl<'a> ThrottlingRetryOptionsView<'a> {
        pub fn max_retry_count(&self) -> Option<&u32>;
        pub fn max_retry_wait_time(&self) -> Option<&Duration>;
        pub fn new(env: Option<::std::sync::Arc<ThrottlingRetryOptions>>, runtime: Option<::std::sync::Arc<ThrottlingRetryOptions>>, account: Option<::std::sync::Arc<ThrottlingRetryOptions>>, operation: Option<&'a ThrottlingRetryOptions>) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct ThroughputControlGroupName(pub std::borrow::Cow<'static, str>);
    #[doc(inline)]
    impl ThroughputControlGroupName {
        pub fn as_str(&self) -> &str;
        pub fn new<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(name: impl Into<Cow<'static, str>>) -> Self;
    }
    #[doc(inline)]
    impl AsRef<str> for ThroughputControlGroupName {
        fn as_ref(&self) -> &str;
    }
    #[doc(inline)]
    impl Display for ThroughputControlGroupName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[doc(inline)]
    impl From<&'static str> for ThroughputControlGroupName {
        fn from(name: &'static str) -> Self;
    }
    #[doc(inline)]
    impl From<Cow<'static, str>> for ThroughputControlGroupName {
        fn from(name: Cow<'static, str>) -> Self;
    }
    #[doc(inline)]
    impl From<String> for ThroughputControlGroupName {
        fn from(name: String) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub struct ThroughputControlGroupOptions {
    }
    #[doc(inline)]
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
    #[doc(inline)]
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct ThroughputControlOptions {
        pub group_name: Option<crate::models::ThroughputControlGroupName>,
        pub throughput_bucket: Option<u32>,
        pub priority_level: Option<crate::options::PriorityLevel>,
    }
    #[doc(inline)]
    #[automatically_derived]
    pub struct ThroughputControlOptionsBuilder {
    }
    #[doc(inline)]
    #[automatically_derived]
    impl ThroughputControlOptionsBuilder {
        #[must_use]
        pub fn build(self) -> ThroughputControlOptions;
        pub fn new() -> Self;
        pub fn with_group_name(self, value: ThroughputControlGroupName) -> Self;
        pub fn with_priority_level(self, value: PriorityLevel) -> Self;
        pub fn with_throughput_bucket(self, value: u32) -> Self;
    }
    #[doc(inline)]
    #[automatically_derived]
    pub struct ThroughputControlOptionsView<'a> {
    }
    #[doc(inline)]
    #[automatically_derived]
    impl<'a> ThroughputControlOptionsView<'a> {
        pub fn group_name(&self) -> Option<&ThroughputControlGroupName>;
        pub fn new(env: Option<::std::sync::Arc<ThroughputControlOptions>>, runtime: Option<::std::sync::Arc<ThroughputControlOptions>>, account: Option<::std::sync::Arc<ThroughputControlOptions>>, operation: Option<&'a ThroughputControlOptions>) -> Self;
        pub fn priority_level(&self) -> Option<&PriorityLevel>;
        pub fn throughput_bucket(&self) -> Option<&u32>;
    }
    #[cfg(feature = "control_plane")]
    #[derive(Clone, Default)]
    #[non_exhaustive]
    pub struct ThroughputOptions {
        pub operation: azure_data_cosmos_driver::options::OperationOptions,
    }
    #[cfg(feature = "control_plane")]
    impl ThroughputOptions {
        pub fn with_operation_options(self, operation: OperationOptions) -> Self;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct UserAgentSuffix(/* private fields */);
    #[doc(inline)]
    impl UserAgentSuffix {
        const MAX_LENGTH: usize = 25;
        pub fn as_str(&self) -> &str;
        pub fn new<impl Into<String>: Into<String>>(value: impl Into<String>) -> Self;
        pub fn try_new<impl Into<String>: Into<String>>(value: impl Into<String>) -> Option<Self>;
    }
    #[doc(inline)]
    impl AsRef<str> for UserAgentSuffix {
        fn as_ref(&self) -> &str;
    }
    #[doc(inline)]
    impl Display for UserAgentSuffix {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum AvailabilityStrategy {
        Hedging(HedgingStrategy),
        Disabled,
    }
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum ChangeFeedMode {
        #[default]
        LatestVersion,
        AllVersionsAndDeletes,
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(tag = "kind", content = "value", rename_all = "snake_case")]
    pub enum ChangeFeedStartFrom {
        Beginning,
        Now,
        PointInTime(time::OffsetDateTime),
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub enum ConsistencyLevel {
        ConsistentPrefix,
        Eventual,
        Session,
        BoundedStaleness,
        Strong,
    }
    impl Display for ConsistencyLevel {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    pub enum ContentResponseOnWrite {
        Enabled,
        #[default]
        Disabled,
    }
    #[doc(inline)]
    impl Display for ContentResponseOnWrite {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[doc(inline)]
    impl From<ContentResponseOnWrite> for bool {
        fn from(value: ContentResponseOnWrite) -> Self;
    }
    #[doc(inline)]
    impl From<bool> for ContentResponseOnWrite {
        fn from(value: bool) -> Self;
    }
    #[doc(inline)]
    impl FromStr for ContentResponseOnWrite {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum DiagnosticsVerbosity {
        #[default]
        Default,
        Summary,
        Detailed,
    }
    #[doc(inline)]
    impl DiagnosticsVerbosity {
        pub fn as_str(&self) -> &'static str;
    }
    #[doc(inline)]
    impl AsRef<str> for DiagnosticsVerbosity {
        fn as_ref(&self) -> &str;
    }
    #[doc(inline)]
    impl Display for DiagnosticsVerbosity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[doc(inline)]
    impl FromStr for DiagnosticsVerbosity {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum MaxItemCountHint {
        ServerDecides,
        Limit(std::num::NonZeroU32),
    }
    #[cfg(feature = "preview_patch")]
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum PatchStrategy {
        #[default]
        Auto,
        ClientSide,
        ServerSide,
    }
    #[cfg(feature = "preview_patch")]
    #[doc(inline)]
    impl PatchStrategy {
        pub fn as_str(&self) -> &'static str;
    }
    #[cfg(feature = "preview_patch")]
    #[doc(inline)]
    impl Display for PatchStrategy {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[cfg(feature = "preview_patch")]
    #[doc(inline)]
    impl FromStr for PatchStrategy {
        type Err = CosmosError;
        fn from_str(value: &str) -> Result<Self, <Self as >::Err>;
    }
    #[doc(inline)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum Precondition {
        IfMatch(azure_core::http::Etag),
        IfNoneMatch(azure_core::http::Etag),
    }
    #[doc(inline)]
    impl Precondition {
        pub fn as_if_match(&self) -> Option<&Etag>;
        pub fn as_if_none_match(&self) -> Option<&Etag>;
        pub fn if_match<impl Into<Etag>: Into<Etag>>(etag: impl Into<Etag>) -> Self;
        pub fn if_none_match<impl Into<Etag>: Into<Etag>>(etag: impl Into<Etag>) -> Self;
        pub fn is_if_match(&self) -> bool;
        pub fn is_if_none_match(&self) -> bool;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum PriorityLevel {
        #[default]
        High,
        Low,
    }
    #[doc(inline)]
    impl PriorityLevel {
        pub fn as_str(&self) -> &'static str;
    }
    #[doc(inline)]
    impl Display for PriorityLevel {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[doc(inline)]
    impl FromStr for PriorityLevel {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum QueryPlanMode {
        #[default]
        LocalPreferred,
        GatewayOnly,
    }
    #[doc(inline)]
    impl QueryPlanMode {
        pub fn as_str(self) -> &'static str;
    }
    #[doc(inline)]
    impl Display for QueryPlanMode {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[doc(inline)]
    impl FromStr for QueryPlanMode {
        type Err = CosmosError;
        fn from_str(value: &str) -> Result<Self, <Self as >::Err>;
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum ReadConsistencyStrategy {
        Default,
        Eventual,
        Session,
        LatestCommitted,
        GlobalStrong,
    }
    #[doc(inline)]
    impl ReadConsistencyStrategy {
        pub fn as_str(&self) -> &'static str;
    }
    #[doc(inline)]
    impl Display for ReadConsistencyStrategy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    #[doc(inline)]
    impl FromStr for ReadConsistencyStrategy {
        type Err = CosmosError;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub enum RoutingStrategy {
        ProximityTo(super::Region),
        PreferredRegions(Vec<super::Region>),
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum ServerCertificateValidation {
        #[default]
        Required,
        RequiredUnlessEmulator,
    }
    #[doc(inline)]
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum TlsBackend {
        #[default]
        Rustls,
    }
}
```
