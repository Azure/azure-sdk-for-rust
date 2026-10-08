// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    collections::BTreeMap,
    num::NonZeroU32,
    sync::{Arc, Mutex},
    time::Duration,
};

use azure_data_cosmos_driver::{
    models::{ContainerReference, ContinuationToken, MaxItemCountHint},
    CosmosDriver, CosmosError, Result,
};
use futures::future::BoxFuture;
use tokio::{
    sync::Semaphore,
    task::{JoinHandle, JoinSet},
    time::{sleep, timeout},
};

use crate::{
    run_lease, BalanceCycle, BatchState, BootstrapStartPolicy, BootstrapStore, ChangeFeedMode,
    ChangeFeedReadOptions, CheckpointError, CheckpointStore, LeaseBalancer, LeaseControl,
    LeaseOwnershipOptions, LeaseReleaseOutcome, LeaseRunOptions, LeaseRunOutcome, LeaseRunPhase,
    LeaseSession, OwnedLease, ProcessorEngine, RawBatchSource, RawChangeFeedPage,
};

mod topology_controller;
mod worker;
use worker::{run_worker, WorkerEvent};

/// Completion-bearing schema-agnostic application callback.
pub type RawChangeHandler =
    Arc<dyn Fn(RawChangeFeedPage) -> BoxFuture<'static, Result<()>> + Send + Sync>;

/// Configuration for continuous managed processing against prepared resources.
#[derive(Clone)]
pub struct ManagedProcessorOptions {
    instance_name: String,
    host_capacity: NonZeroU32,
    callback_concurrency: NonZeroU32,
    mode: ChangeFeedMode,
    start: BootstrapStartPolicy,
    ownership: LeaseOwnershipOptions,
    startup_timeout: Duration,
    balance_interval: Duration,
    coverage_interval: Duration,
    poll_interval: Duration,
    recovery_backoff: Duration,
    drain_timeout: Duration,
    max_item_count: NonZeroU32,
}

impl ManagedProcessorOptions {
    /// Sets the required host name; every start obtains a fresh incarnation.
    pub fn new(instance_name: impl Into<String>) -> Self {
        Self {
            instance_name: instance_name.into(),
            host_capacity: NonZeroU32::new(32).expect("nonzero"),
            callback_concurrency: NonZeroU32::new(1).expect("nonzero"),
            mode: ChangeFeedMode::LatestVersion,
            start: BootstrapStartPolicy::Beginning,
            ownership: LeaseOwnershipOptions::new(
                Duration::from_secs(60),
                Duration::from_secs(10),
                Duration::from_secs(10),
                Duration::from_secs(5),
            )
            .expect("valid default ownership policy"),
            startup_timeout: Duration::from_secs(60),
            balance_interval: Duration::from_secs(1),
            coverage_interval: Duration::from_secs(30),
            poll_interval: Duration::from_millis(100),
            recovery_backoff: Duration::from_secs(1),
            drain_timeout: Duration::from_secs(5),
            max_item_count: NonZeroU32::new(100).expect("nonzero"),
        }
    }
    /// Bounds simultaneously owned leases independently from callback concurrency.
    pub fn with_host_capacity(mut self, value: NonZeroU32) -> Self {
        self.host_capacity = value;
        self
    }
    /// Bounds callbacks across all local leases; validated at start to at most 32.
    pub fn with_callback_concurrency(mut self, value: NonZeroU32) -> Self {
        self.callback_concurrency = value;
        self
    }
    /// Sets the feed mode.
    pub fn with_mode(mut self, value: ChangeFeedMode) -> Self {
        self.mode = value;
        self
    }
    /// Sets the immutable bootstrap starting policy.
    pub fn with_start_policy(mut self, value: BootstrapStartPolicy) -> Self {
        self.start = value;
        self
    }
    /// Sets the shared persisted ownership timing policy.
    pub fn with_ownership_policy(mut self, value: LeaseOwnershipOptions) -> Self {
        self.ownership = value;
        self
    }
    /// Sets the overall preparation deadline, at most five minutes.
    pub fn with_startup_timeout(mut self, value: Duration) -> Self {
        self.startup_timeout = value;
        self
    }
    /// Sets the balancing cadence.
    pub fn with_balance_interval(mut self, value: Duration) -> Self {
        self.balance_interval = value;
        self
    }
    /// Sets the coverage verification and durable split-reconciliation cadence.
    pub fn with_coverage_interval(mut self, value: Duration) -> Self {
        self.coverage_interval = value;
        self
    }
    /// Sets the delay between idle polls.
    pub fn with_poll_interval(mut self, value: Duration) -> Self {
        self.poll_interval = value;
        self
    }
    /// Sets the delay before reopening a failed source or callback.
    pub fn with_recovery_backoff(mut self, value: Duration) -> Self {
        self.recovery_backoff = value;
        self
    }
    /// Sets the callback/checkpoint drain budget after stop.
    pub fn with_drain_timeout(mut self, value: Duration) -> Self {
        self.drain_timeout = value;
        self
    }
    /// Sets the service page-size hint.
    pub fn with_max_item_count(mut self, value: NonZeroU32) -> Self {
        self.max_item_count = value;
        self
    }
    /// Returns the configured host name, not the per-start incarnation.
    pub fn instance_name(&self) -> &str {
        &self.instance_name
    }
    /// Returns the local ownership bound.
    pub fn host_capacity(&self) -> NonZeroU32 {
        self.host_capacity
    }
    /// Returns the independent callback bound.
    pub fn callback_concurrency(&self) -> NonZeroU32 {
        self.callback_concurrency
    }
    /// Returns the feed mode.
    pub fn mode(&self) -> ChangeFeedMode {
        self.mode
    }
    /// Returns the bootstrap policy.
    pub fn start_policy(&self) -> &BootstrapStartPolicy {
        &self.start
    }
    /// Returns the ownership policy.
    pub fn ownership_policy(&self) -> &LeaseOwnershipOptions {
        &self.ownership
    }
    /// Returns the startup budget.
    pub fn startup_timeout(&self) -> Duration {
        self.startup_timeout
    }
    /// Returns the balancing cadence.
    pub fn balance_interval(&self) -> Duration {
        self.balance_interval
    }
    /// Returns the coverage cadence.
    pub fn coverage_interval(&self) -> Duration {
        self.coverage_interval
    }
    /// Returns the idle cadence.
    pub fn poll_interval(&self) -> Duration {
        self.poll_interval
    }
    /// Returns the recovery backoff.
    pub fn recovery_backoff(&self) -> Duration {
        self.recovery_backoff
    }
    /// Returns the drain budget.
    pub fn drain_timeout(&self) -> Duration {
        self.drain_timeout
    }
    /// Returns the page-size hint.
    pub fn max_item_count(&self) -> NonZeroU32 {
        self.max_item_count
    }

    fn validate(&self) -> Result<()> {
        if self.instance_name.trim().is_empty()
            || self.host_capacity.get() > 32
            || self.callback_concurrency.get() > 32
            || self.startup_timeout.is_zero()
            || self.startup_timeout > Duration::from_secs(300)
            || [
                self.balance_interval,
                self.coverage_interval,
                self.poll_interval,
                self.recovery_backoff,
            ]
            .iter()
            .any(|value| value.is_zero() || *value > Duration::from_secs(300))
            || self.drain_timeout > Duration::from_secs(300)
        {
            return Err(invalid(
                "managed processing requires bounded positive intervals and capacity at most 32",
            ));
        }
        Ok(())
    }
}

/// State of a managed processor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManagedProcessorState {
    /// The coordinator is scheduling leases.
    Running,
    /// Stop has been requested; owned tasks are draining.
    Stopping,
    /// Every owned task was joined.
    Stopped,
    /// A coordinator or task failure terminated scheduling.
    Failed,
}

/// Local processing state for one lease.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManagedLeaseState {
    /// Reading or awaiting a callback/checkpoint.
    Running,
    /// Reopening from durable progress after a read failure.
    Recovering,
    /// Callback failure is isolated and delayed before redelivery.
    Quarantined,
    /// Ownership has been revoked.
    OwnershipLost,
    /// No more local work is running.
    Stopped,
}

/// Observable markers; feed/callback progress is not necessarily durable.
#[derive(Clone)]
pub struct ManagedLeaseSnapshot {
    id: String,
    state: ManagedLeaseState,
    last_feed: Option<ContinuationToken>,
    last_callback: Option<ContinuationToken>,
    last_checkpoint: ContinuationToken,
    last_error: Option<Arc<CosmosError>>,
}
impl ManagedLeaseSnapshot {
    /// Returns the lease identity.
    pub fn id(&self) -> &str {
        &self.id
    }
    /// Returns local health.
    pub fn state(&self) -> ManagedLeaseState {
        self.state
    }
    /// Returns the latest fetched candidate.
    pub fn last_feed(&self) -> Option<&ContinuationToken> {
        self.last_feed.as_ref()
    }
    /// Returns the latest successfully completed callback candidate.
    pub fn last_callback(&self) -> Option<&ContinuationToken> {
        self.last_callback.as_ref()
    }
    /// Returns confirmed durable progress.
    pub fn last_checkpoint(&self) -> &ContinuationToken {
        &self.last_checkpoint
    }
    /// Returns the retained driver/application failure, including diagnostics.
    pub fn last_error(&self) -> Option<&Arc<CosmosError>> {
        self.last_error.as_ref()
    }
}

/// Processor health and progress captured without performing service I/O.
#[derive(Clone)]
pub struct ManagedProcessorSnapshot {
    state: ManagedProcessorState,
    leases: Vec<ManagedLeaseSnapshot>,
    last_error: Option<Arc<CosmosError>>,
}
impl ManagedProcessorSnapshot {
    /// Returns lifecycle state.
    pub fn state(&self) -> ManagedProcessorState {
        self.state
    }
    /// Returns per-lease markers, including retired local workers.
    pub fn leases(&self) -> &[ManagedLeaseSnapshot] {
        &self.leases
    }
    /// Returns the latest coordinator failure.
    pub fn last_error(&self) -> Option<&Arc<CosmosError>> {
        self.last_error.as_ref()
    }
    /// Returns true while scheduling runs and no lease is recovering or quarantined.
    pub fn is_healthy(&self) -> bool {
        self.state == ManagedProcessorState::Running
            && self.last_error.is_none()
            && self.leases.iter().all(|lease| {
                matches!(
                    lease.state,
                    ManagedLeaseState::Running | ManagedLeaseState::OwnershipLost
                )
            })
    }
}

/// A joined worker's final drain and conditional release outcome.
pub struct ManagedLeaseShutdown {
    id: String,
    outcome: LeaseRunOutcome,
    release: LeaseReleaseOutcome,
    error: Option<Arc<CosmosError>>,
    snapshot: Option<ManagedLeaseSnapshot>,
}
impl ManagedLeaseShutdown {
    /// Returns the lease identity.
    pub fn id(&self) -> &str {
        &self.id
    }
    /// Returns the final processing outcome.
    pub fn outcome(&self) -> LeaseRunOutcome {
        self.outcome
    }
    /// Returns the conditional release outcome.
    pub fn release(&self) -> &LeaseReleaseOutcome {
        &self.release
    }
    /// Returns a terminal processing or maintenance failure.
    pub fn error(&self) -> Option<&Arc<CosmosError>> {
        self.error.as_ref()
    }
    /// Returns final progress markers, when the worker could publish them.
    pub fn snapshot(&self) -> Option<&ManagedLeaseSnapshot> {
        self.snapshot.as_ref()
    }
}

/// Honest shutdown evidence, including failures; joining is not proof of release.
pub struct ManagedShutdownReport {
    leases: Vec<ManagedLeaseShutdown>,
    errors: Vec<Arc<CosmosError>>,
    joined_workers: u64,
    topology_error_indices: BTreeMap<String, usize>,
}
impl ManagedShutdownReport {
    fn empty() -> Self {
        Self {
            leases: Vec::new(),
            errors: Vec::new(),
            joined_workers: 0,
            topology_error_indices: BTreeMap::new(),
        }
    }
    /// Returns the latest joined outcome for each logical lease.
    pub fn leases(&self) -> &[ManagedLeaseShutdown] {
        &self.leases
    }
    /// Returns coordinator and task failures.
    pub fn errors(&self) -> &[Arc<CosmosError>] {
        &self.errors
    }
    /// Returns the number of joined worker tasks, including failures and recoveries.
    pub fn joined_workers(&self) -> u64 {
        self.joined_workers
    }
    /// Returns true only when every task drained and conditionally released.
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty()
            && self.leases.iter().all(|lease| {
                lease.error.is_none()
                    && lease.outcome == LeaseRunOutcome::StoppedDrained
                    && matches!(lease.release, LeaseReleaseOutcome::Released)
            })
    }
    fn record_lease(&mut self, result: ManagedLeaseShutdown) {
        if let Some(previous) = self.leases.iter_mut().find(|lease| lease.id == result.id) {
            *previous = result;
        } else {
            self.leases.push(result);
        }
    }
    fn record_topology_error(&mut self, id: &str, error: Arc<CosmosError>) {
        if let Some(index) = self.topology_error_indices.get(id) {
            self.errors[*index] = error;
        } else {
            self.topology_error_indices
                .insert(id.to_owned(), self.errors.len());
            self.errors.push(error);
        }
    }
    fn clear_topology_error(&mut self, id: &str) {
        if let Some(index) = self.topology_error_indices.remove(id) {
            self.errors.remove(index);
            for other in self.topology_error_indices.values_mut() {
                if *other > index {
                    *other -= 1;
                }
            }
        }
    }
}

struct SharedState {
    state: ManagedProcessorState,
    leases: BTreeMap<String, ManagedLeaseSnapshot>,
    last_error: Option<Arc<CosmosError>>,
}
type Shared = Arc<Mutex<SharedState>>;
type OwnedTasks = Arc<tokio::sync::Mutex<JoinSet<ManagedLeaseShutdown>>>;
type Shutdown = Arc<Mutex<ManagedShutdownReport>>;

struct LeaseWorker {
    session: LeaseSession,
    stop: LeaseControl,
}

struct CoordinatorStop(LeaseControl);
impl Drop for CoordinatorStop {
    fn drop(&mut self) {
        self.0.stop();
    }
}

/// Owns the coordinator and every local lease task until they are joined.
///
/// Call [`stop()`](Self::stop) to drain, join, and obtain release evidence. Drop
/// requests stop and aborts owned tasks without awaiting them or confirming release.
/// Dropping a Tokio join handle alone would detach its task; this handle explicitly
/// requests abortion instead.
///
/// Managed execution requires a live Tokio runtime with its time driver enabled.
/// Both current-thread and multi-thread runtimes are supported. Keep the runtime
/// alive until [`is_complete()`](Self::is_complete) is true.
pub struct ManagedProcessor {
    control: LeaseControl,
    shared: Shared,
    coordinator: Option<JoinHandle<()>>,
    tasks: OwnedTasks,
    shutdown: Shutdown,
    completed: Option<Arc<ManagedShutdownReport>>,
}
impl ManagedProcessor {
    /// Returns lifecycle state without service I/O.
    pub fn state(&self) -> ManagedProcessorState {
        self.effective_state(
            self.shared
                .lock()
                .expect("managed state lock poisoned")
                .state,
        )
    }
    /// Returns true only after stop has joined all work and cached its final report.
    ///
    /// Callers may discard the handle and start a new incarnation at this point.
    /// Completion does not imply clean drain or confirmed remote release.
    pub fn is_complete(&self) -> bool {
        self.completed.is_some()
    }
    /// Returns an in-memory state and progress snapshot.
    pub fn snapshot(&self) -> ManagedProcessorSnapshot {
        let state = self.shared.lock().expect("managed state lock poisoned");
        ManagedProcessorSnapshot {
            state: self.effective_state(state.state),
            leases: state.leases.values().cloned().collect(),
            last_error: state.last_error.clone(),
        }
    }
    fn effective_state(&self, state: ManagedProcessorState) -> ManagedProcessorState {
        if state == ManagedProcessorState::Running
            && self
                .coordinator
                .as_ref()
                .is_some_and(JoinHandle::is_finished)
        {
            ManagedProcessorState::Failed
        } else {
            state
        }
    }
    /// Stops scheduling, drains callbacks/checkpoints, joins tasks, and releases authority.
    ///
    /// Failures are retained in the report rather than disguised as a clean stop.
    /// Cancelling this future retains handles and partial evidence in this object;
    /// a later stop resumes joining. Repeated completed stops return the same report.
    /// Poll an incomplete stop on the Tokio runtime that drives this processor.
    pub async fn stop(&mut self) -> Arc<ManagedShutdownReport> {
        if let Some(report) = &self.completed {
            return report.clone();
        }
        self.control.stop();
        self.shared
            .lock()
            .expect("managed state lock poisoned")
            .state = ManagedProcessorState::Stopping;
        if let Some(coordinator) = self.coordinator.as_mut() {
            let result = coordinator.await;
            self.coordinator.take();
            if let Err(error) = result {
                self.shutdown
                    .lock()
                    .expect("shutdown report lock poisoned")
                    .errors
                    .push(Arc::new(task_error(error)));
            }
        }
        {
            let mut tasks = self.tasks.lock().await;
            while let Some(result) = tasks.join_next().await {
                record_join(&self.shutdown, result);
            }
        }
        let report = Arc::new(std::mem::replace(
            &mut *self.shutdown.lock().expect("shutdown report lock poisoned"),
            ManagedShutdownReport::empty(),
        ));
        publish_shutdown_state(&self.shared, &report);
        self.completed = Some(report.clone());
        report
    }
}
impl Drop for ManagedProcessor {
    fn drop(&mut self) {
        self.control.stop();
        if let Some(coordinator) = &self.coordinator {
            coordinator.abort();
        }
        if let Ok(mut tasks) = self.tasks.try_lock() {
            tasks.abort_all();
        }
    }
}

impl ProcessorEngine {
    /// Starts continuous raw processing with independent source and lease contexts.
    ///
    /// Existing persisted bootstrap positions win before source discovery. The
    /// lease container must use `/workload` and a single-write-region account.
    /// No clients, named resources, or Azure infrastructure are created.
    ///
    /// Poll this method inside a Tokio runtime with its time driver enabled.
    /// Azure Core's configurable async runtime is not used for this lifecycle.
    ///
    /// # Errors
    ///
    /// Returns a missing-Tokio-runtime error, or configuration, preparation,
    /// discovery, or bootstrap failures before
    /// publishing a processor or invoking application callbacks.
    ///
    /// # Panics
    ///
    /// Tokio timers panic if the current runtime's time driver is disabled.
    pub async fn start_managed(
        &self,
        lease_driver: Arc<CosmosDriver>,
        lease_container: ContainerReference,
        group: impl Into<String>,
        options: ManagedProcessorOptions,
        handler: RawChangeHandler,
    ) -> Result<ManagedProcessor> {
        options.validate()?;
        tokio::runtime::Handle::try_current().map_err(|_| {
            invalid(
                "managed processing requires an active Tokio runtime with its time driver enabled",
            )
        })?;
        let group = group.into();
        let incarnation = format!("{}-{:016x}", options.instance_name, rand::random::<u64>());
        let workload = timeout(options.startup_timeout, async {
            let existing = BootstrapStore::load_existing_single_writer(
                lease_driver.clone(),
                lease_container.clone(),
                &self.source_identity(),
                &group,
                options.mode,
                &options.start,
                options.ownership.clone(),
            )
            .await?;
            let workload = match existing {
                Some(workload) => workload,
                None => {
                    let plan = self
                        .discover_bootstrap_plan(group, options.mode, options.start.clone())
                        .await?;
                    BootstrapStore::from_resolved_single_writer(
                        lease_driver,
                        lease_container,
                        plan,
                        options.ownership.clone(),
                    )?
                }
            };
            workload
                .ensure_initialized(&incarnation, options.startup_timeout)
                .await?;
            Ok::<_, CosmosError>(workload)
        })
        .await
        .map_err(|_| {
            CosmosError::builder()
                .with_status(azure_data_cosmos_driver::CosmosStatus::new(azure_core::http::StatusCode::RequestTimeout)
                    .with_sub_status(azure_data_cosmos_driver::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT.value()))
                .with_message("managed startup deadline expired; readiness is unconfirmed").build()
        })??;
        let balancer = LeaseBalancer::new(workload.clone(), &incarnation)?
            .with_max_owned_leases(options.host_capacity);
        let shared = Arc::new(Mutex::new(SharedState {
            state: ManagedProcessorState::Running,
            leases: BTreeMap::new(),
            last_error: None,
        }));
        let control = LeaseControl::default();
        let tasks = Arc::new(tokio::sync::Mutex::new(JoinSet::new()));
        let shutdown = Arc::new(Mutex::new(ManagedShutdownReport::empty()));
        let coordinator = tokio::spawn(coordinate(
            self.clone(),
            workload,
            balancer,
            incarnation,
            options,
            handler,
            control.clone(),
            shared.clone(),
            tasks.clone(),
            shutdown.clone(),
        ));
        Ok(ManagedProcessor {
            control,
            shared,
            coordinator: Some(coordinator),
            tasks,
            shutdown,
            completed: None,
        })
    }
}

#[allow(clippy::too_many_arguments)] // These are owned lifecycle resources, not caller options.
async fn coordinate(
    engine: ProcessorEngine,
    workload: BootstrapStore,
    mut balancer: LeaseBalancer,
    incarnation: String,
    options: ManagedProcessorOptions,
    handler: RawChangeHandler,
    control: LeaseControl,
    shared: Shared,
    tasks: OwnedTasks,
    shutdown: Shutdown,
) {
    let _stop_guard = CoordinatorStop(control.clone());
    let callbacks = Arc::new(Semaphore::new(options.callback_concurrency.get() as usize));
    let mut tasks = tasks.lock().await;
    let mut sessions = BTreeMap::<String, LeaseWorker>::new();
    let (events, mut recoveries) = tokio::sync::mpsc::channel::<WorkerEvent>(32);
    let mut balance = tokio::time::interval(options.balance_interval);
    let mut coverage = tokio::time::interval(options.coverage_interval);
    loop {
        tokio::select! {
            biased;
            _ = control.interrupted() => break,
            Some(event) = recoveries.recv() => {
                let id = event.session.lease().await.id().to_owned();
                if let Some(worker) = sessions.get_mut(&id) {
                    if let Err(error) = balancer.register_session(event.session.clone()).await {
                        shutdown.lock().expect("shutdown report lock poisoned").errors.push(Arc::new(error));
                        worker.stop.stop();
                        break;
                    }
                    worker.session = event.session;
                } else {
                    event.session.control().lose_ownership();
                    tracing::warn!(lease_id=%id, "ignoring recovery notification from a completed lease task");
                }
            }
            joined = tasks.join_next(), if !tasks.is_empty() => {
                let joined = joined.expect("nonempty task set");
                let failed = joined.is_err();
                if let Ok(result) = &joined {
                        sessions.remove(result.id());
                        if result.outcome != LeaseRunOutcome::OwnershipLost
                            && result.outcome != LeaseRunOutcome::StoppedDrained
                        {
                            if let Err(error) = balancer.exclude_lease(result.id()) {
                                shutdown.lock().expect("shutdown report lock poisoned").errors.push(Arc::new(error));
                            }
                        }
                }
                record_join(&shutdown, joined);
                if failed { break; }
            }
            _ = coverage.tick() => {
                let result = async {
                    workload.ensure_initialized(&incarnation, options.startup_timeout).await?;
                    topology_controller::reconcile(&engine, &workload, &mut balancer, &mut sessions,
                        &mut tasks, &options, &control, &shared, &shutdown).await?;
                    Ok::<_, CosmosError>(())
                };
                tokio::select! {
                    _ = control.interrupted() => break,
                    result = result => if let Err(error) = result {
                        shutdown.lock().expect("shutdown report lock poisoned").errors.push(Arc::new(error)); break;
                    }
                }
            }
            _ = balance.tick(), if sessions.len() < options.host_capacity.get() as usize => {
                // Finish a sent acquisition even after stop so its authority is owned and released.
                let cycle = balancer.cycle().await;
                match cycle {
                    Ok(BalanceCycle::Acquired { session, .. }) => {
                        let lease = session.lease().await;
                        let id = lease.id().to_owned();
                        shared.lock().expect("managed state lock poisoned").leases.insert(id.clone(), ManagedLeaseSnapshot {
                            id: id.clone(), state: ManagedLeaseState::Running, last_feed: None,
                            last_callback: None, last_checkpoint: lease.checkpoint().clone(), last_error: None,
                        });
                        let stop = LeaseControl::default();
                        sessions.insert(id, LeaseWorker { session: session.clone(), stop: stop.clone() });
                        tasks.spawn(run_worker(engine.clone(), session, options.clone(), handler.clone(), callbacks.clone(), shared.clone(), control.clone(), stop, events.clone()));
                    }
                    Ok(BalanceCycle::NoAction | BalanceCycle::Conflict(_)) => {}
                    Err(error) => { shutdown.lock().expect("shutdown report lock poisoned").errors.push(Arc::new(error)); break; }
                }
            }
        }
    }
    control.stop();
    for worker in sessions.values() {
        worker.stop.stop();
    }
    while let Some(result) = tasks.join_next().await {
        record_join(&shutdown, result);
    }
    for (id, worker) in sessions {
        let session = worker.session;
        if shutdown
            .lock()
            .expect("shutdown report lock poisoned")
            .leases
            .iter()
            .any(|lease| lease.id == id)
        {
            continue;
        }
        let release = if session.control().is_lost() {
            LeaseReleaseOutcome::AuthorityLost
        } else {
            match session.release().await {
                Ok(()) => LeaseReleaseOutcome::Released,
                Err(error) => LeaseReleaseOutcome::Failed(error),
            }
        };
        let snapshot = shared
            .lock()
            .expect("managed state lock poisoned")
            .leases
            .get(&id)
            .cloned();
        shutdown
            .lock()
            .expect("shutdown report lock poisoned")
            .record_lease(ManagedLeaseShutdown {
                id,
                outcome: LeaseRunOutcome::Failed,
                release,
                error: Some(Arc::new(invalid(
                    "lease task did not return a shutdown outcome",
                ))),
                snapshot,
            });
    }
    publish_shutdown_state(
        &shared,
        &shutdown.lock().expect("shutdown report lock poisoned"),
    );
}

fn record_join(
    shutdown: &Shutdown,
    result: std::result::Result<ManagedLeaseShutdown, tokio::task::JoinError>,
) {
    let mut report = shutdown.lock().expect("shutdown report lock poisoned");
    report.joined_workers += 1;
    match result {
        Ok(result) => report.record_lease(result),
        Err(error) => report.errors.push(Arc::new(task_error(error))),
    }
}
fn publish_shutdown_state(shared: &Shared, report: &ManagedShutdownReport) {
    let mut state = shared.lock().expect("managed state lock poisoned");
    state.state = if report.errors.is_empty() {
        ManagedProcessorState::Stopped
    } else {
        ManagedProcessorState::Failed
    };
    state.last_error = report.errors.last().cloned();
}

fn invalid(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
        .with_message(message)
        .build()
}
fn task_error(error: tokio::task::JoinError) -> CosmosError {
    CosmosError::builder()
        .with_status(azure_data_cosmos_driver::CosmosStatus::new(
            azure_core::http::StatusCode::InternalServerError,
        ))
        .with_message(format!("managed task failed: {error}"))
        .build()
}
