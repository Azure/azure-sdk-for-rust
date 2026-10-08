# azure_cosmos_change_feed_processor_engine

- **Description**: Execution engine for the Azure Cosmos DB change feed processor
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`

```rust
#![crate_name = "azure_cosmos_change_feed_processor_engine"]
#![crate_type = "lib"]
#![warn(missing_docs)]
pub use azure_cosmos_change_feed_processor_engine::processor_engine::ProcessorEngine as ChangeFeedProcessorEngine;
pub fn plan_equal_lease_balance(snapshot: &[BalanceLease], worker: &str, tie_break: u64) -> azure_data_cosmos_driver::Result<Option<BalanceAction>>;
pub fn plan_lease_topology(leases: &[crate::OwnedLease], physical_ranges: &[azure_data_cosmos_driver::models::FeedRange]) -> azure_data_cosmos_driver::Result<Vec<LeaseTopologyAssignment>>;
pub async fn run_lease<S, C, H, F>(source: &mut S, store: &mut C, lease: OwnedLease, control: &LeaseControl, options: &LeaseRunOptions, handler: H) -> Result<LeaseRunReport, LeaseRunError> where S: RawBatchSource, C: CheckpointStore, H: FnMut(crate::RawChangeFeedPage) -> F, F: Future<Output = azure_data_cosmos_driver::Result<()>>;
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
pub struct ChangeFeedReader {
}
impl ChangeFeedReader {
    pub async fn read_page(&mut self) -> Result<RawChangeFeedPage>;
    pub fn to_continuation_token(&self) -> Result<ContinuationToken>;
}
impl RawBatchSource for ChangeFeedReader {
    fn read_batch(&mut self) -> BoxFuture<'_, Result<RawChangeFeedPage>>;
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
#[derive(Clone, Debug)]
pub struct LeaseTopologyAssignment {
}
impl LeaseTopologyAssignment {
    pub fn lease(&self) -> &OwnedLease;
    pub fn physical_ranges(&self) -> &[FeedRange];
    pub fn spans_multiple_partitions(&self) -> bool;
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
pub struct ManagedProcessor {
}
impl ManagedProcessor {
    pub fn is_complete(&self) -> bool;
    pub fn snapshot(&self) -> ManagedProcessorSnapshot;
    pub fn state(&self) -> ManagedProcessorState;
    pub async fn stop(&mut self) -> Arc<ManagedShutdownReport>;
}
impl Drop for ManagedProcessor {
    fn drop(&mut self);
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
pub struct ProcessorEngine {
}
impl ProcessorEngine {
    pub async fn bootstrap_plan<impl Into<String>: Into<String>>(&self, group: impl Into<String>, mode: crate::ChangeFeedMode, start: crate::BootstrapStartPolicy, expected: FeedRange, seeds: Vec<crate::InitialLease>) -> Result<crate::BootstrapPlan>;
    pub async fn connect(account: AccountReference, database: &str, container: &str) -> Result<Self>;
    pub fn container(&self) -> &ContainerReference;
    pub async fn discover_bootstrap_plan<impl Into<String>: Into<String>>(&self, group: impl Into<String>, mode: crate::ChangeFeedMode, start: crate::BootstrapStartPolicy) -> Result<crate::BootstrapPlan>;
    pub fn driver(&self) -> &Arc<CosmosDriver>;
    pub fn from_resolved(driver: Arc<CosmosDriver>, container: ContainerReference) -> Result<Self>;
    pub async fn new(driver: Arc<CosmosDriver>, database: &str, container: &str) -> Result<Self>;
    pub async fn open_reader(&self, options: ChangeFeedReadOptions) -> Result<ChangeFeedReader>;
    pub async fn read_page(&self, max_item_count: NonZeroU32, continuation: Option<&ContinuationToken>) -> Result<RawChangeFeedPage>;
    pub async fn reconcile_lease_topology(&self, leases: &[OwnedLease]) -> Result<Vec<crate::LeaseTopologyAssignment>>;
    pub async fn run_owned_lease<C, H, F>(&self, lease: OwnedLease, read: ChangeFeedReadOptions, control: &LeaseControl, options: &LeaseRunOptions, store: &mut C, handler: H) -> std::result::Result<LeaseRunReport, LeaseRunError> where C: CheckpointStore, H: FnMut(RawChangeFeedPage) -> F, F: Future<Output = Result<()>>;
    pub async fn run_with_lease_session<H, F>(&self, session: &crate::LeaseSession, read: ChangeFeedReadOptions, options: &LeaseRunOptions, handler: H) -> crate::LeaseOwnershipRun where H: FnMut(RawChangeFeedPage) -> F, F: Future<Output = Result<()>>;
    pub fn source_identity(&self) -> String;
}
impl crate::ProcessorEngine {
    pub async fn start_managed<impl Into<String>: Into<String>>(&self, lease_driver: Arc<CosmosDriver>, lease_container: ContainerReference, group: impl Into<String>, options: ManagedProcessorOptions, handler: RawChangeHandler) -> Result<ManagedProcessor>;
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
pub trait CheckpointStore: Send {
    fn persist<'a>(&mut self, lease: &'a OwnedLease, candidate: &'a ContinuationToken) -> futures::future::BoxFuture<'a, Result<OwnedLease, CheckpointError>>;
}
pub trait RawBatchSource: Send {
    fn read_batch(&mut self) -> BoxFuture<'_, Result<RawChangeFeedPage>>;
}
pub type RawChangeHandler = std::sync::Arc<dyn Fn(crate::RawChangeFeedPage) -> futures::future::BoxFuture<'static, azure_data_cosmos_driver::Result<()>> + Send + Sync>;
```
