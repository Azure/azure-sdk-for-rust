# azure_cosmos_change_feed_processor

- **Description**: Change feed processor for Azure Cosmos DB
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`

```rust
#![crate_name = "azure_cosmos_change_feed_processor"]
#![crate_type = "lib"]
#![warn(missing_docs)]
pub use azure_cosmos_change_feed_processor::models::ChangeFeedItem;
pub use azure_cosmos_change_feed_processor::models::ChangeFeedMetadata;
pub use azure_cosmos_change_feed_processor::models::ChangeFeedOperationType;
pub use azure_cosmos_change_feed_processor::models::LogicalSequenceNumber;
pub fn plan_equal_lease_balance(snapshot: &[BalanceLease], worker: &str, tie_break: u64) -> azure_data_cosmos_driver::Result<Option<BalanceAction>>;
#[derive(Clone, Debug)]
pub struct BalanceAction {
}
impl BalanceAction {
    pub fn kind(&self) -> BalanceActionKind;
    pub fn lease_id(&self) -> &str;
}
#[derive(Clone, Debug)]
pub struct BalanceLease {
}
impl BalanceLease {
    pub fn eligible(&self) -> bool;
    pub fn expired(&self) -> bool;
    pub fn id(&self) -> &str;
    pub fn new<impl Into<String>: Into<String>>(id: impl Into<String>) -> Result<Self>;
    pub fn owner(&self) -> Option<&str>;
    pub fn with_eligible(self, eligible: bool) -> Self;
    pub fn with_expired(self, expired: bool) -> Self;
    pub fn with_owner<impl Into<String>: Into<String>>(self, owner: impl Into<String>) -> Self;
}
#[derive(Clone, Debug)]
pub struct BalanceRunOptions {
}
impl BalanceRunOptions {
    pub fn cycles(&self) -> NonZeroU32;
    pub fn interval(&self) -> Duration;
    pub fn jitter(&self) -> Duration;
    pub fn new(cycles: NonZeroU32) -> Self;
    pub fn timeout(&self) -> Duration;
    pub fn with_interval(self, value: Duration) -> Self;
    pub fn with_jitter(self, value: Duration) -> Self;
    pub fn with_timeout(self, value: Duration) -> Self;
}
#[derive(Debug)]
pub struct BalanceRunReport {
}
impl BalanceRunReport {
    pub fn acquisitions(&self) -> u32;
    pub fn conflicts(&self) -> u32;
    pub fn cycles(&self) -> u32;
    pub fn stopped(&self) -> bool;
}
#[derive(Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BootstrapPlan {
}
impl BootstrapPlan {
    pub fn expected_range(&self) -> &FeedRange;
    pub fn group(&self) -> &str;
    pub fn initial_leases(&self) -> &[InitialLease];
    pub fn mode(&self) -> ChangeFeedMode;
    pub fn new<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(source: impl Into<String>, group: impl Into<String>, mode: ChangeFeedMode, start: BootstrapStartPolicy, expected: FeedRange, seeds: Vec<InitialLease>) -> Result<Self>;
    pub fn source(&self) -> &str;
    pub fn start_policy(&self) -> &BootstrapStartPolicy;
}
pub struct BootstrapReady {
}
impl BootstrapReady {
    pub fn lease_ids(&self) -> &[String];
    pub fn plan(&self) -> &BootstrapPlan;
    pub fn workload_id(&self) -> &str;
}
#[derive(Clone)]
pub struct BootstrapStore {
}
impl BootstrapStore {
    pub async fn ensure_initialized<impl Into<String>: Into<String>>(&self, incarnation: impl Into<String>, startup_timeout: Duration) -> Result<BootstrapReady>;
    pub fn from_resolved_single_writer(driver: Arc<CosmosDriver>, container: ContainerReference, plan: BootstrapPlan, policy: LeaseOwnershipOptions) -> Result<Self>;
    pub async fn load_existing_single_writer(driver: Arc<CosmosDriver>, container: ContainerReference, source: &str, group: &str, mode: ChangeFeedMode, start: &BootstrapStartPolicy, policy: LeaseOwnershipOptions) -> Result<Option<Self>>;
    pub async fn new_single_writer(driver: Arc<CosmosDriver>, database: &str, container: &str, plan: BootstrapPlan, policy: LeaseOwnershipOptions) -> Result<Self>;
    pub fn work_lease_store(&self, ready: &BootstrapReady, id: &str) -> Result<CosmosLeaseStore>;
}
#[derive(Debug)]
pub struct ChangeFeedPage<T> {
}
impl<T> ChangeFeedPage<T> {
    pub fn activity_id(&self) -> Option<&str>;
    pub fn continuation(&self) -> &str;
    pub fn diagnostics(&self) -> Arc<azure_data_cosmos_driver::DiagnosticsContext>;
    pub fn headers(&self) -> &azure_data_cosmos_driver::models::CosmosResponseHeaders;
    pub fn items(&self) -> &[ChangeFeedItem<T>];
    pub fn raw(&self) -> &RawChangeFeedPage;
    pub fn request_charge(&self) -> Option<f64>;
    pub fn state(&self) -> BatchState;
    pub fn status(&self) -> StatusCode;
}
impl<T: DeserializeOwned> TryFrom<RawChangeFeedPage> for ChangeFeedPage<T> {
    type Error = CosmosError;
    fn try_from(raw: RawChangeFeedPage) -> std::result::Result<Self, <Self as >::Error>;
}
pub struct ChangeFeedProcessor {
}
impl ChangeFeedProcessor {
    pub async fn bootstrap_plan(&self, mode: ChangeFeedMode, start: BootstrapStartPolicy, expected: azure_data_cosmos_driver::models::FeedRange, seeds: Vec<InitialLease>) -> Result<BootstrapPlan>;
    pub async fn bootstrap_store(&self, plan: BootstrapPlan, policy: LeaseOwnershipOptions) -> Result<BootstrapStore>;
    pub fn builder() -> ChangeFeedProcessorBuilder;
    pub async fn connect(endpoint: Url, credential: Arc<dyn TokenCredential>, database: &str, container: &str) -> Result<Self>;
    pub async fn connect_with_key(endpoint: Url, key: Secret, database: &str, container: &str) -> Result<Self>;
    pub fn feed_binding(&self) -> Option<&ContainerBinding>;
    pub fn group(&self) -> Option<&str>;
    pub fn lease_binding(&self) -> Option<&ContainerBinding>;
    pub async fn lease_store<impl Into<String>: Into<String>>(&self, partition_key: azure_data_cosmos_driver::models::PartitionKey, id: impl Into<String>, options: LeaseOwnershipOptions) -> Result<CosmosLeaseStore>;
    pub async fn read_page<T: DeserializeOwned>(&self, options: &ReadChangesOptions) -> Result<ChangeFeedPage<T>>;
    pub async fn read_page_with_options<T: DeserializeOwned>(&self, options: ChangeFeedReadOptions) -> Result<ChangeFeedPage<T>>;
    pub async fn run_owned_lease<T, C, H, F>(&self, lease: OwnedLease, read: ChangeFeedReadOptions, control: &LeaseControl, options: &LeaseRunOptions, store: &mut C, handler: H) -> std::result::Result<LeaseRunReport, LeaseRunError> where T: DeserializeOwned, C: CheckpointStore, H: FnMut(ChangeFeedPage<T>) -> F, F: Future<Output = azure_data_cosmos_driver::Result<()>>;
    pub async fn run_with_lease_session<T, H, F>(&self, session: &LeaseSession, read: ChangeFeedReadOptions, options: &LeaseRunOptions, handler: H) -> LeaseOwnershipRun where T: DeserializeOwned, H: FnMut(ChangeFeedPage<T>) -> F, F: Future<Output = azure_data_cosmos_driver::Result<()>>;
}
impl crate::ChangeFeedProcessor {
    pub async fn start<T, H, F>(&self, options: ManagedProcessorOptions, handler: H) -> azure_core::Result<()> where T: DeserializeOwned + Send + 'static, H: Fn(ChangeFeedPage<T>) -> F + Send + Sync + 'static, F: Future<Output = azure_data_cosmos_driver::Result<()>> + Send + 'static;
    pub async fn state(&self) -> ProcessorLifecycleState;
    pub async fn stop(&self) -> Option<Arc<ManagedShutdownReport>>;
}
pub struct ChangeFeedProcessorBuilder {
}
impl ChangeFeedProcessorBuilder {
    pub async fn build<impl Into<String>: Into<String>>(self, group: impl Into<String>, feed: ContainerBinding, leases: ContainerBinding) -> azure_core::Result<ChangeFeedProcessor>;
    pub fn preparation_timeout(&self) -> Duration;
    pub fn with_preparation_timeout(self, timeout: Duration) -> Self;
}
impl Default for ChangeFeedProcessorBuilder {
    fn default() -> Self;
}
#[derive(Clone, Debug)]
pub struct ChangeFeedReadOptions {
}
impl ChangeFeedReadOptions {
    pub fn continuation(&self) -> Option<&ContinuationToken>;
    pub fn max_item_count(&self) -> Option<MaxItemCountHint>;
    pub fn mode(&self) -> ChangeFeedMode;
    pub fn new(range: FeedRange, start: ChangeFeedStartFrom) -> Self;
    pub fn operation_options(&self) -> &OperationOptions;
    pub fn plan_options(&self) -> &PlanOptions;
    pub fn range(&self) -> &FeedRange;
    pub fn session_token(&self) -> Option<&SessionToken>;
    pub fn start(&self) -> &ChangeFeedStartFrom;
    pub fn with_continuation(self, continuation: ContinuationToken) -> Self;
    pub fn with_max_item_count(self, hint: MaxItemCountHint) -> Self;
    pub fn with_mode(self, mode: ChangeFeedMode) -> Self;
    pub fn with_operation_options(self, options: OperationOptions) -> Self;
    pub fn with_plan_options(self, options: PlanOptions) -> Self;
    pub fn with_session_token(self, token: SessionToken) -> Self;
}
#[derive(Clone, Debug)]
pub struct ContainerBinding {
}
impl ContainerBinding {
    pub fn account(&self) -> &AccountReference;
    pub fn container(&self) -> &str;
    pub fn database(&self) -> &str;
    pub fn endpoint(&self) -> &Url;
    pub fn new<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(account: AccountReference, database: impl Into<String>, container: impl Into<String>) -> Result<Self>;
    pub fn operation_options(&self) -> &OperationOptions;
    pub fn preferred_regions(&self) -> &[Region];
    pub fn runtime_builder(&self) -> &CosmosDriverRuntimeBuilder;
    pub fn with_key<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(endpoint: Url, database: impl Into<String>, container: impl Into<String>, key: Secret) -> Result<Self>;
    pub fn with_operation_options(self, options: OperationOptions) -> Self;
    pub fn with_preferred_regions(self, regions: Vec<Region>) -> Self;
    pub fn with_runtime_builder(self, builder: CosmosDriverRuntimeBuilder) -> Self;
    pub fn with_token_credential<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(endpoint: Url, database: impl Into<String>, container: impl Into<String>, credential: Arc<dyn TokenCredential>) -> Result<Self>;
}
#[derive(Clone)]
pub struct CosmosLeaseStore {
}
impl CosmosLeaseStore {
    pub async fn acquire<impl Into<String>: Into<String>>(&self, observation: &LeaseObservation, owner: impl Into<String>) -> Result<Option<LeaseSession>>;
    pub fn from_resolved_single_writer<impl Into<String>: Into<String>>(driver: Arc<CosmosDriver>, container: ContainerReference, partition_key: PartitionKey, id: impl Into<String>, options: LeaseOwnershipOptions) -> Result<Self>;
    pub async fn new_single_writer<impl Into<String>: Into<String>>(driver: Arc<CosmosDriver>, database: &str, container: &str, partition_key: PartitionKey, id: impl Into<String>, options: LeaseOwnershipOptions) -> Result<Self>;
    pub async fn observe(&self) -> Result<LeaseObservation>;
    pub async fn transfer<impl Into<String>: Into<String>>(&self, observation: &LeaseObservation, owner: impl Into<String>) -> Result<LeaseSession>;
    pub async fn try_acquire<impl Into<String>: Into<String>>(&self, owner: impl Into<String>) -> Result<Option<LeaseSession>>;
}
#[derive(Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InitialLease {
}
impl InitialLease {
    pub fn checkpoint(&self) -> &str;
    pub fn new(range: FeedRange, checkpoint: ContinuationToken) -> Result<Self>;
    pub fn range(&self) -> &FeedRange;
}
pub struct LeaseBalancer {
}
impl LeaseBalancer {
    pub async fn cycle(&mut self) -> Result<BalanceCycle>;
    pub fn exclude_lease<impl Into<String>: Into<String>>(&mut self, id: impl Into<String>) -> Result<()>;
    pub fn include_lease(&mut self, id: &str);
    pub fn max_owned_leases(&self) -> NonZeroU32;
    pub fn new<impl Into<String>: Into<String>>(workload: BootstrapStore, worker: impl Into<String>) -> Result<Self>;
    pub fn page_size(&self) -> NonZeroU32;
    pub async fn register_session(&mut self, session: LeaseSession) -> Result<()>;
    pub async fn run_cycles<H, F>(&mut self, options: &BalanceRunOptions, control: &LeaseControl, on_acquired: H) -> Result<BalanceRunReport> where H: FnMut(LeaseSession) -> F, F: Future<Output = Result<()>>;
    pub fn tie_break(&self) -> u64;
    pub fn with_max_owned_leases(self, limit: NonZeroU32) -> Self;
    pub fn with_page_size(self, page_size: NonZeroU32) -> Self;
    pub fn with_tie_break(self, tie_break: u64) -> Self;
    pub fn worker(&self) -> &str;
}
#[derive(Clone, Debug)]
pub struct LeaseControl(/* private fields */);
impl LeaseControl {
    pub fn lose_ownership(&self);
    pub fn stop(&self);
}
impl Default for LeaseControl {
    fn default() -> Self;
}
pub struct LeaseObservation {
}
impl LeaseObservation {
    pub fn checkpoint(&self) -> &str;
    pub fn generation(&self) -> u64;
    pub fn owner(&self) -> Option<&str>;
    pub fn revision(&self) -> &str;
}
#[derive(Clone, Debug)]
pub struct LeaseOwnershipOptions {
}
impl LeaseOwnershipOptions {
    pub fn duration(&self) -> Duration;
    pub fn new(duration: Duration, renew_interval: Duration, safety_margin: Duration, request_timeout: Duration) -> Result<Self>;
    pub fn renew_interval(&self) -> Duration;
    pub fn request_timeout(&self) -> Duration;
    pub fn safety_margin(&self) -> Duration;
}
#[derive(Debug)]
pub struct LeaseOwnershipRun {
}
impl LeaseOwnershipRun {
    pub fn maintenance_error(&self) -> Option<&CosmosError>;
    pub fn processing(&self) -> &std::result::Result<crate::LeaseRunReport, crate::LeaseRunError>;
    pub fn release(&self) -> &LeaseReleaseOutcome;
}
#[derive(Debug)]
pub struct LeaseRunError {
}
impl LeaseRunError {
    pub fn error(&self) -> &CosmosError;
    pub fn report(&self) -> &LeaseRunReport;
}
impl Display for LeaseRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl Error for LeaseRunError {
    fn source(&self) -> Option<&dyn std::error::Error + 'static>;
}
#[derive(Clone, Debug)]
pub struct LeaseRunOptions {
}
impl LeaseRunOptions {
    pub fn checkpoint_attempts(&self) -> NonZeroU32;
    pub fn drain_timeout(&self) -> Duration;
    pub fn idle_delay(&self) -> Duration;
    pub fn max_batches(&self) -> NonZeroU32;
    pub fn max_duration(&self) -> Duration;
    pub fn new(max_batches: NonZeroU32) -> Self;
    pub fn retry_delay(&self) -> Duration;
    pub fn with_checkpoint_attempts(self, value: NonZeroU32) -> Self;
    pub fn with_drain_timeout(self, value: Duration) -> Self;
    pub fn with_idle_delay(self, value: Duration) -> Self;
    pub fn with_max_duration(self, value: Duration) -> Self;
    pub fn with_retry_delay(self, value: Duration) -> Self;
}
#[derive(Debug)]
pub struct LeaseRunReport {
}
impl LeaseRunReport {
    pub fn candidate(&self) -> Option<&ContinuationToken>;
    pub fn confirmed_batches(&self) -> u32;
    pub fn diagnostics(&self) -> Option<&Arc<DiagnosticsContext>>;
    pub fn lease(&self) -> &OwnedLease;
    pub fn outcome(&self) -> LeaseRunOutcome;
    pub fn phase(&self) -> Option<LeaseRunPhase>;
}
#[derive(Clone)]
pub struct LeaseSession {
}
impl LeaseSession {
    pub async fn checkpoint(&self, expected: &OwnedLease, candidate: &ContinuationToken) -> std::result::Result<OwnedLease, CheckpointError>;
    pub fn control(&self) -> &LeaseControl;
    pub async fn lease(&self) -> OwnedLease;
    pub async fn reconcile_checkpoint(&self, expected: &OwnedLease, candidate: &ContinuationToken) -> Result<Option<LeaseSession>>;
    pub async fn release(&self) -> Result<()>;
    pub async fn renew(&self) -> Result<OwnedLease>;
}
impl CheckpointStore for LeaseSession {
    fn persist<'a>(&mut self, lease: &'a OwnedLease, candidate: &'a ContinuationToken) -> futures::future::BoxFuture<'a, std::result::Result<OwnedLease, CheckpointError>>;
}
pub struct ManagedLeaseShutdown {
}
impl ManagedLeaseShutdown {
    pub fn error(&self) -> Option<&Arc<CosmosError>>;
    pub fn id(&self) -> &str;
    pub fn outcome(&self) -> LeaseRunOutcome;
    pub fn release(&self) -> &LeaseReleaseOutcome;
    pub fn snapshot(&self) -> Option<&ManagedLeaseSnapshot>;
}
#[derive(Clone)]
pub struct ManagedLeaseSnapshot {
}
impl ManagedLeaseSnapshot {
    pub fn id(&self) -> &str;
    pub fn last_callback(&self) -> Option<&ContinuationToken>;
    pub fn last_checkpoint(&self) -> &ContinuationToken;
    pub fn last_error(&self) -> Option<&Arc<CosmosError>>;
    pub fn last_feed(&self) -> Option<&ContinuationToken>;
    pub fn state(&self) -> ManagedLeaseState;
}
#[derive(Clone)]
pub struct ManagedProcessorOptions {
}
impl ManagedProcessorOptions {
    pub fn balance_interval(&self) -> Duration;
    pub fn callback_concurrency(&self) -> NonZeroU32;
    pub fn coverage_interval(&self) -> Duration;
    pub fn drain_timeout(&self) -> Duration;
    pub fn host_capacity(&self) -> NonZeroU32;
    pub fn instance_name(&self) -> &str;
    pub fn max_item_count(&self) -> NonZeroU32;
    pub fn mode(&self) -> ChangeFeedMode;
    pub fn new<impl Into<String>: Into<String>>(instance_name: impl Into<String>) -> Self;
    pub fn ownership_policy(&self) -> &LeaseOwnershipOptions;
    pub fn poll_interval(&self) -> Duration;
    pub fn recovery_backoff(&self) -> Duration;
    pub fn start_policy(&self) -> &BootstrapStartPolicy;
    pub fn startup_timeout(&self) -> Duration;
    pub fn with_balance_interval(self, value: Duration) -> Self;
    pub fn with_callback_concurrency(self, value: NonZeroU32) -> Self;
    pub fn with_coverage_interval(self, value: Duration) -> Self;
    pub fn with_drain_timeout(self, value: Duration) -> Self;
    pub fn with_host_capacity(self, value: NonZeroU32) -> Self;
    pub fn with_max_item_count(self, value: NonZeroU32) -> Self;
    pub fn with_mode(self, value: ChangeFeedMode) -> Self;
    pub fn with_ownership_policy(self, value: LeaseOwnershipOptions) -> Self;
    pub fn with_poll_interval(self, value: Duration) -> Self;
    pub fn with_recovery_backoff(self, value: Duration) -> Self;
    pub fn with_start_policy(self, value: BootstrapStartPolicy) -> Self;
    pub fn with_startup_timeout(self, value: Duration) -> Self;
}
#[derive(Clone)]
pub struct ManagedProcessorSnapshot {
}
impl ManagedProcessorSnapshot {
    pub fn is_healthy(&self) -> bool;
    pub fn last_error(&self) -> Option<&Arc<CosmosError>>;
    pub fn leases(&self) -> &[ManagedLeaseSnapshot];
    pub fn state(&self) -> ManagedProcessorState;
}
pub struct ManagedShutdownReport {
}
impl ManagedShutdownReport {
    pub fn errors(&self) -> &[Arc<CosmosError>];
    pub fn is_clean(&self) -> bool;
    pub fn joined_workers(&self) -> u64;
    pub fn leases(&self) -> &[ManagedLeaseShutdown];
}
#[derive(Clone, Debug)]
pub struct OwnedLease {
}
impl OwnedLease {
    pub fn checkpoint(&self) -> &ContinuationToken;
    pub fn epoch(&self) -> NonZeroU64;
    pub fn id(&self) -> &str;
    pub fn new<impl Into<String>: Into<String>, impl Into<String>: Into<String>, impl Into<String>: Into<String>>(id: impl Into<String>, owner: impl Into<String>, epoch: NonZeroU64, revision: impl Into<String>, range: FeedRange, checkpoint: ContinuationToken) -> azure_data_cosmos_driver::Result<Self>;
    pub fn owner(&self) -> &str;
    pub fn range(&self) -> &FeedRange;
    pub fn revision(&self) -> &str;
}
#[derive(Clone)]
pub struct RawChangeFeedPage {
}
impl RawChangeFeedPage {
    pub fn continuation(&self) -> &ContinuationToken;
    pub fn response(&self) -> &CosmosResponse;
    pub fn state(&self) -> BatchState;
}
impl From<(CosmosResponse, ContinuationToken)> for RawChangeFeedPage {
    fn from((response, continuation): (CosmosResponse, ContinuationToken)) -> Self;
}
impl From<RawChangeFeedPage> for (azure_data_cosmos_driver::CosmosResponse, azure_data_cosmos_driver::models::ContinuationToken) {
    fn from(page: RawChangeFeedPage) -> Self;
}
#[derive(Clone, Debug)]
pub struct ReadChangesOptions {
}
impl ReadChangesOptions {
    pub fn continuation(&self) -> Option<&str>;
    pub fn max_item_count(&self) -> NonZeroU32;
    pub fn with_continuation<impl Into<String>: Into<String>>(self, continuation: impl Into<String>) -> Self;
    pub fn with_max_item_count(self, max_item_count: NonZeroU32) -> Self;
}
impl Default for ReadChangesOptions {
    fn default() -> Self;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BalanceActionKind {
    Pickup,
    Transfer,
}
pub enum BalanceCycle {
    NoAction,
    Conflict(crate::BalanceAction),
    Acquired { action: crate::BalanceAction, session: super::super::LeaseSession },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchState {
    Idle,
    Page,
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
#[derive(Debug)]
pub enum CheckpointError {
    Retryable(azure_data_cosmos_driver::CosmosError),
    Ambiguous(azure_data_cosmos_driver::CosmosError),
    Rejected(azure_data_cosmos_driver::CosmosError),
}
#[derive(Debug)]
pub enum LeaseReleaseOutcome {
    NotAttempted,
    Released,
    AuthorityLost,
    Failed(azure_data_cosmos_driver::CosmosError),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeaseRunOutcome {
    Failed,
    BatchLimitReached,
    StoppedDrained,
    StoppedUnprocessed,
    DrainTimedOut,
    OwnershipLost,
    RunTimedOut,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeaseRunPhase {
    Reading,
    Handling,
    Checkpointing,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManagedLeaseState {
    Running,
    Recovering,
    Quarantined,
    OwnershipLost,
    Stopped,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManagedProcessorState {
    Running,
    Stopping,
    Stopped,
    Failed,
}
#[non_exhaustive]
pub enum ProcessorLifecycleState {
    Prepared,
    Running(azure_cosmos_change_feed_processor_engine::ManagedProcessorSnapshot),
    Stopping(azure_cosmos_change_feed_processor_engine::ManagedProcessorSnapshot),
    Stopped(std::sync::Arc<azure_cosmos_change_feed_processor_engine::ManagedShutdownReport>),
}
pub trait CheckpointStore: Send {
    fn persist<'a>(&mut self, lease: &'a OwnedLease, candidate: &'a ContinuationToken) -> futures::future::BoxFuture<'a, Result<OwnedLease, CheckpointError>>;
}
pub mod models {
    #[derive(Clone, Debug)]
    pub struct ChangeFeedItem<T> {
    }
    impl<T> ChangeFeedItem<T> {
        pub fn current(&self) -> Option<&T>;
        pub fn metadata(&self) -> Option<&ChangeFeedMetadata>;
        pub fn operation_type(&self) -> Option<ChangeFeedOperationType>;
        pub fn previous(&self) -> Option<&T>;
        pub fn with_current(self, value: T) -> Self;
        pub fn with_metadata(self, value: ChangeFeedMetadata) -> Self;
        pub fn with_previous(self, value: T) -> Self;
    }
    impl<'de, T: DeserializeOwned> Deserialize<'de> for ChangeFeedItem<T> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, <D as >::Error>;
    }
    impl<T> Default for ChangeFeedItem<T> {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize)]
    pub struct ChangeFeedMetadata {
    }
    impl ChangeFeedMetadata {
        pub fn conflict_resolution_timestamp(&self) -> Option<Duration>;
        pub fn id(&self) -> Option<&str>;
        pub fn lsn(&self) -> Option<LogicalSequenceNumber>;
        pub fn operation_type(&self) -> Option<ChangeFeedOperationType>;
        pub fn partition_key(&self) -> Option<&Value>;
        pub fn previous_image_lsn(&self) -> Option<LogicalSequenceNumber>;
        pub fn time_to_live_expired(&self) -> Option<bool>;
        pub fn with_conflict_resolution_timestamp(self, value: Duration) -> Self;
        pub fn with_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
        pub fn with_lsn(self, value: LogicalSequenceNumber) -> Self;
        pub fn with_operation_type(self, value: ChangeFeedOperationType) -> Self;
        pub fn with_partition_key(self, value: Value) -> Self;
        pub fn with_previous_image_lsn(self, value: LogicalSequenceNumber) -> Self;
        pub fn with_time_to_live_expired(self, value: bool) -> Self;
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
}
```
