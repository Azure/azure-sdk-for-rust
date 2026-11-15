// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod balancing;
mod distribution;
mod retry;
#[cfg(all(test, feature = "__internal_in_memory_emulator"))]
mod tests;
mod topology;

use crate::{
    change_feed_processor::{
        bootstrap,
        bootstrap::{
            error, namespace, store::LeaseStore as Store, LeaseDocument,
            WorkloadIdentity as Identity, SCHEMA_VERSION,
        },
        operation_options,
        processing::{
            distribution::{AcquisitionDelivery, ChangeFeedRequestDistributor},
            retry::ReadRetry,
        },
    },
    driver::dataflow::split_feed_envelope,
    models::{
        ChangeFeedFactoryMode, ChangeFeedStartFrom, ContainerReference, ContinuationToken,
        CosmosResponse, MaxItemCountHint, ResponseBody,
    },
    options::PlanOptions,
    CosmosDriver, Result,
};
use async_lock::Mutex;
use azure_core::{
    fmt::SafeDebug,
    http::{Etag, StatusCode},
};
use futures::{
    channel::oneshot,
    future::{BoxFuture, Shared},
    stream::FuturesUnordered,
    FutureExt, StreamExt,
};
use std::{
    collections::{HashMap, HashSet},
    future::Future,
    panic::AssertUnwindSafe,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex as StdMutex,
    },
    time::{Duration, Instant},
};
use time::OffsetDateTime;
use uuid::Uuid;

type Signal = Shared<BoxFuture<'static, ()>>;
type ErrorObserver = dyn Fn(Option<String>, crate::CosmosError) + Send + Sync;

/// Controls persistence of successfully acknowledged change feed batches.
///
/// Defaults to deferred persistence with a five-minute interval.
/// This policy does not change callback page sizes or lease renewal schedules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CheckpointPolicy {
    /// Persists each successful acknowledgment before polling the next batch.
    Eager,
    /// Retains acknowledged progress in memory until the interval since its
    /// first unpersisted acknowledgment elapses.
    ///
    /// The interval must be positive and representable by the runtime clock.
    /// It schedules persistence, but cannot guarantee storage availability.
    /// Idle reads do not trigger persistence. Graceful shutdown and topology
    /// recovery flush without waiting for the interval.
    /// A crash can replay multiple successfully handled batches.
    Deferred {
        /// Maximum time to defer attempting persistence of acknowledged progress.
        max_interval: Duration,
    },
}

impl Default for CheckpointPolicy {
    fn default() -> Self {
        Self::deferred()
    }
}

impl CheckpointPolicy {
    /// Defers persistence of successful progress for up to five minutes.
    ///
    /// Use [`Self::Deferred`] to select a different interval. Explicit
    /// checkpoints and graceful cleanup can persist progress earlier.
    pub fn deferred() -> Self {
        Self::Deferred {
            max_interval: Duration::from_secs(300),
        }
    }

    fn validate(self) -> Result<()> {
        if let Self::Deferred { max_interval } = self {
            if max_interval.is_zero()
                || Instant::now().checked_add(max_interval).is_none()
                || azure_core::time::Duration::try_from(max_interval).is_err()
            {
                return Err(error(
                    StatusCode::BadRequest,
                    "Invalid checkpoint interval.",
                ));
            }
        }
        self.processing_deadline(now_millis())?;
        Ok(())
    }

    fn processing_deadline(self, now: i64) -> Result<i64> {
        let duration = match self {
            Self::Eager => Some(Duration::from_secs(600)),
            Self::Deferred { max_interval } => max_interval.checked_mul(2),
        };
        duration
            .and_then(|duration| i64::try_from(duration.as_nanos().div_ceil(1_000_000)).ok())
            .and_then(|millis| now.checked_add(millis))
            .ok_or_else(|| {
                error(
                    StatusCode::BadRequest,
                    "Processing inactivity interval is not representable.",
                )
            })
    }
}

/// Configuration for driver-owned lease coordination and feed polling.
///
/// Each acquired lease permits one outstanding batch. The consumer must
/// acknowledge it after processing, or drop its receipt to abandon the lease.
/// There is no shared batch-count limit across leases or processors.
///
/// Ordinary HTTP 429 responses defer only the affected acquisition. Retry
/// count and wait budgets inherit the source driver's throttling options.
/// A read turn, including planning, is bounded to five seconds or a shorter
/// configured operation deadline. Lease maintenance continues during cooldown.
///
/// Other workers may also acquire a lease whose processing deadline has expired:
/// ten minutes without an acknowledged nonempty batch with
/// [`CheckpointPolicy::Eager`], or twice the deferral interval.
/// Acquisition starts a fresh grace period. Idle reads, missing demand, and
/// throttling do not refresh processing activity. Activity is published by
/// existing lease writes, independently of checkpoint progress.
#[derive(Clone, SafeDebug)]
pub struct ProcessorOptions {
    mode: ChangeFeedFactoryMode,
    workload_name: String,
    worker_name: String,
    verify_leases_on_restart: bool,
    start: ChangeFeedStartFrom,
    continuation: Option<ContinuationToken>,
    page_size: MaxItemCountHint,
    poll_delay: Duration,
    bootstrap_timeout: Duration,
    min_leases: u32,
    max_leases: u32,
    expiration: Duration,
    checkpoint_policy: CheckpointPolicy,
    #[safe(false)]
    error_observer: Option<Arc<ErrorObserver>>,
}

impl ProcessorOptions {
    /// Creates options with a 60-second lease expiration budget.
    ///
    /// Renewal is scheduled after one third of the budget without a successful
    /// lease write. Acquisition checks run every one sixth of the budget.
    /// These schedules are independent of feed polling and callback duration.
    /// Fresh feeds start from now, request 100 items, and wait five seconds
    /// after idle responses. Lease counts are uncapped.
    ///
    /// Workers sharing the source, lease storage, and `workload_name` cooperate.
    /// `worker_name` identifies the worker. Names are validated during bootstrap.
    pub fn new(
        mode: ChangeFeedFactoryMode,
        workload_name: impl Into<String>,
        worker_name: impl Into<String>,
    ) -> Self {
        Self {
            mode,
            workload_name: workload_name.into(),
            worker_name: worker_name.into(),
            verify_leases_on_restart: false,
            start: ChangeFeedStartFrom::Now,
            continuation: None,
            page_size: MaxItemCountHint::Limit(std::num::NonZeroU32::new(100).unwrap()),
            poll_delay: Duration::from_secs(5),
            bootstrap_timeout: bootstrap::BOOTSTRAP_TIMEOUT,
            min_leases: 0,
            max_leases: 0,
            expiration: Duration::from_secs(60),
            checkpoint_policy: CheckpointPolicy::default(),
            error_observer: None,
        }
    }

    /// Observes background coordination and lease-processing failures.
    ///
    /// The first argument is the affected lease identifier, or `None` for
    /// worker-wide failures. Startup errors are returned directly. Transient
    /// request retries do not each produce a notification.
    ///
    /// This synchronous observer must return promptly without blocking. Language
    /// adapters can forward notifications to their own bounded asynchronous
    /// dispatcher. Observer panics are logged and do not interrupt recovery.
    /// Replaces any previously configured observer.
    pub fn with_error_observer<F>(mut self, observer: F) -> Self
    where
        F: Fn(Option<String>, crate::CosmosError) + Send + Sync + 'static,
    {
        self.error_observer = Some(Arc::new(observer));
        self
    }

    /// Selects the initial position for leases without saved progress.
    ///
    /// Beginning and time-based starts require latest-version mode.
    pub fn with_start(mut self, start: ChangeFeedStartFrom) -> Self {
        self.start = start;
        self
    }

    /// Sets an initial lease-scoped continuation.
    ///
    /// Saved lease progress takes precedence. The existing planner validates
    /// the token's container, range, and mode; raw service tokens are not
    /// interchangeable with composite driver continuations.
    pub fn with_continuation(mut self, continuation: ContinuationToken) -> Self {
        self.continuation = Some(continuation);
        self
    }

    /// Sets the service page-size hint, not a hard payload or memory bound.
    pub fn with_max_item_count(mut self, hint: MaxItemCountHint) -> Self {
        self.page_size = hint;
        self
    }

    /// Sets how [`BatchReceipt::acknowledge()`] persists successful progress.
    ///
    /// Defaults to [`CheckpointPolicy::deferred()`], with a five-minute interval. Explicit
    /// [`BatchReceipt::checkpoint()`] calls always wait for durable persistence.
    /// Invalid intervals are rejected by [`Processor::start()`] before I/O.
    /// With a `Now` start, restarting before the first durable checkpoint can
    /// resolve a later starting position.
    pub fn with_checkpoint_policy(mut self, policy: CheckpointPolicy) -> Self {
        self.checkpoint_policy = policy;
        self
    }

    /// Sets the delay after an idle feed response. Zero is allowed.
    pub fn with_poll_delay(mut self, delay: Duration) -> Self {
        self.poll_delay = delay;
        self
    }

    /// Sets the caller's bootstrap wait budget (default 120 seconds); invalid values fail before I/O.
    /// Attempt and lock limits are unchanged; optional restart verification is outside this budget.
    pub fn with_bootstrap_timeout(mut self, timeout: Duration) -> Self {
        self.bootstrap_timeout = timeout;
        self
    }

    /// Sets minimum and maximum lease targets. A zero maximum is uncapped.
    ///
    /// A minimum greater than a nonzero maximum is rejected when starting.
    pub fn with_lease_counts(mut self, minimum: u32, maximum: u32) -> Self {
        self.min_leases = minimum;
        self.max_leases = maximum;
        self
    }

    /// Checks the persisted initial range plan and recreates missing leases on start.
    ///
    /// Disabled by default. Existing owners and checkpoints are never replaced.
    /// Complete replacement coverage prevents recreation of deleted split parents.
    /// Partial or overlapping replacement coverage fails verification rather
    /// than creating an overlapping lease.
    /// This does not discover topology changes or modify bootstrap completion.
    pub fn with_lease_verification_enabled_on_restart(mut self, enabled: bool) -> Self {
        self.verify_leases_on_restart = enabled;
        self
    }

    fn renew_interval(&self) -> Duration {
        self.expiration / 3
    }

    fn acquire_interval(&self) -> Duration {
        self.expiration / 6
    }
}

/// Local lease document and ETag, renewal timing, and acknowledged progress awaiting persistence.
struct LeaseState {
    value: LeaseDocument,
    etag: Etag,
    renewed: Option<Instant>,
    pending: Option<PendingCheckpoint>,
}

impl From<(LeaseDocument, Etag)> for LeaseState {
    fn from((value, etag): (LeaseDocument, Etag)) -> Self {
        Self {
            value,
            etag,
            renewed: None,
            pending: None,
        }
    }
}

#[derive(Clone)]
struct PendingCheckpoint {
    token: ContinuationToken,
    since: Instant,
    handled_changes: u64,
    failed: bool,
}

impl LeaseDocument {
    fn processed(&mut self, policy: CheckpointPolicy, now: i64) -> Result<()> {
        let now = self
            .last_processed_at
            .map_or(now, |previous| previous.max(now));
        self.processing_deadline = Some(policy.processing_deadline(now)?);
        self.last_processed_at = Some(now);
        Ok(())
    }
}

impl LeaseState {
    fn acknowledge(&mut self, token: ContinuationToken, handled_changes: u64) {
        // Both policies record safe progress locally first; this does not write the lease document.
        if let Some(pending) = &mut self.pending {
            // Advance the token without resetting the first-unsaved-progress deadline.
            pending.token = token;
            pending.handled_changes = pending.handled_changes.saturating_add(handled_changes);
        } else {
            self.pending = Some(PendingCheckpoint {
                token,
                since: Instant::now(),
                handled_changes,
                failed: false,
            });
        }
    }

    fn checkpoint_delay(&self, policy: CheckpointPolicy, now: Instant) -> Option<Duration> {
        // Only Deferred needs a timer; Eager flushes in the acknowledgment path.
        match (policy, &self.pending) {
            (CheckpointPolicy::Deferred { max_interval }, Some(pending)) => {
                Some(max_interval.saturating_sub(now.saturating_duration_since(pending.since)))
            }
            _ => None,
        }
    }
}

struct Context {
    source_driver: Arc<CosmosDriver>,
    source: ContainerReference,
    lease_driver: Arc<CosmosDriver>,
    leases: ContainerReference,
    identity: Identity,
    namespace: String,
    options: ProcessorOptions,
}

impl Context {
    fn store(&self) -> Store<'_> {
        Store {
            driver: &self.lease_driver,
            container: &self.leases,
            deadline: Instant::now() + Duration::from_secs(25),
        }
    }

    fn validate(&self, lease: &LeaseDocument) -> Result<()> {
        bootstrap::validate_lease(&lease, &self.identity, &self.namespace)
    }

    #[cfg(all(test, feature = "__internal_in_memory_emulator"))]
    async fn acquire(&self, observed: &LeaseDocument) -> Result<LeaseState> {
        self.acquire_tracked(observed, None).await
    }

    async fn acquire_tracked(
        &self,
        observed: &LeaseDocument,
        state: Option<&SharedState>,
    ) -> Result<LeaseState> {
        let incarnation = Uuid::new_v4().to_string();
        self.modify_lease(observed, Update::Acquire(&incarnation), state)
            .await
    }

    async fn update(&self, lease: &mut LeaseState, update: Update<'_>) -> Result<()> {
        let preserve_pending = matches!(update, Update::Renew);
        let mut updated = self.modify_lease(&lease.value, update, None).await?;
        if preserve_pending {
            updated.pending = lease.pending.take();
        }
        *lease = updated;
        Ok(())
    }

    /// Applies a mutation to fresh state while preserving the observed acquisition.
    /// Acquisition races fail immediately; owned mutations retry only write conflicts.
    async fn modify_lease(
        &self,
        observed: &LeaseDocument,
        update: Update<'_>,
        acquisition_state: Option<&SharedState>,
    ) -> Result<LeaseState> {
        let id = &observed.record.id;
        let retry_conflicts = !matches!(update, Update::Acquire(_));
        let attempts = if retry_conflicts { 5 } else { 1 };
        for _ in 0..attempts {
            let (current, etag) = self
                .store()
                .read::<LeaseDocument>(id)
                .await?
                .ok_or_else(lost)?;
            self.validate(&current)?;
            let next = transition_lease(current, observed, &update, &self.options, now_millis())?;
            let renewed = Instant::now();
            let key = (id.clone(), next.incarnation.clone());
            if let Some(state) = acquisition_state {
                state
                    .unreleased
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .insert(key.clone(), next.clone());
            }
            match self.store().replace(id, &next, etag).await {
                Ok(updated) => {
                    if let Some(state) = acquisition_state {
                        state
                            .unreleased
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .remove(&key);
                    }
                    return Ok(LeaseState {
                        renewed: Some(renewed),
                        ..updated.into()
                    });
                }
                Err(err) if retry_conflicts && err.status().is_precondition_failed() => continue,
                Err(err) => return Err(err),
            }
        }
        Err(error(
            StatusCode::PreconditionFailed,
            "Lease update conflicts exhausted the retry budget.",
        ))
    }

    async fn flush(&self, lease: &mut LeaseState) -> Result<()> {
        // Both Eager and Deferred use the same ownership-fenced write of acknowledged progress.
        if let Some(pending) = lease.pending.clone() {
            if let Err(err) = self.update(lease, Update::Checkpoint(&pending.token)).await {
                // Preserve failed progress for cleanup decisions; never reinterpret this as a feed error.
                if let Some(pending) = &mut lease.pending {
                    pending.failed = true;
                }
                return Err(err);
            }
            if matches!(
                self.options.checkpoint_policy,
                CheckpointPolicy::Deferred { .. }
            ) {
                tracing::debug!(
                    handled_changes = pending.handled_changes,
                    elapsed_ms = pending.since.elapsed().as_millis() as u64,
                    "Persisted deferred checkpoint"
                );
            }
        }
        Ok(())
    }

    async fn wait_with_checkpoint<T>(
        &self,
        lease: &Mutex<LeaseState>,
        cancelled: &Signal,
        force: &Signal,
        work: impl Future<Output = Result<T>>,
    ) -> Result<Option<T>> {
        let work = work.fuse();
        futures::pin_mut!(work);
        loop {
            let delay = lease
                .lock()
                .await
                .checkpoint_delay(self.options.checkpoint_policy, Instant::now());
            let deadline = async {
                match delay {
                    Some(delay) => sleep(delay).await,
                    None => futures::future::pending().await,
                }
            }
            .fuse();
            futures::pin_mut!(deadline);
            futures::select_biased! {
                _ = force.clone().fuse() => return Ok(None),
                _ = cancelled.clone().fuse() => return Err(lost()),
                _ = deadline => {
                    let mut lease = lease.lock().await;
                    if force.clone().now_or_never().is_some() {
                        return Ok(None);
                    }
                    if cancelled.clone().now_or_never().is_some() {
                        return Err(lost());
                    }
                    // Deferred deadline: persist prior safe progress while the current work stays pinned.
                    // Failures leave through the maintenance result, not source cooldown or recovery.
                    self.flush(&mut lease).await?;
                },
                result = work => return result.map(Some),
            }
        }
    }
}

enum Update<'a> {
    Acquire(&'a str),
    Renew,
    Checkpoint(&'a ContinuationToken),
    Release,
}

fn transition_lease(
    mut current: LeaseDocument,
    observed: &LeaseDocument,
    update: &Update<'_>,
    options: &ProcessorOptions,
    now: i64,
) -> Result<LeaseDocument> {
    if current.owner != observed.owner || current.incarnation != observed.incarnation {
        return Err(lost());
    }
    if matches!(update, Update::Acquire(_))
        && observed
            .processing_deadline
            .is_some_and(|deadline| now >= deadline)
        && current
            .processing_deadline
            .is_some_and(|deadline| now < deadline)
        && current.renewed_at.is_some_and(|renewed| {
            now.saturating_sub(renewed) <= options.expiration.as_millis() as i64
        })
    {
        return Err(error(
            StatusCode::PreconditionFailed,
            "Lease processing resumed before acquisition.",
        ));
    }
    if !matches!(update, Update::Acquire(_)) {
        current.last_processed_at = current.last_processed_at.max(observed.last_processed_at);
        current.processing_deadline = current
            .processing_deadline
            .max(observed.processing_deadline);
    }
    match update {
        Update::Acquire(incarnation) => {
            current.owner = Some(options.worker_name.clone());
            current.incarnation = Some((*incarnation).to_owned());
            current.last_processed_at = None;
            current.processing_deadline = Some(options.checkpoint_policy.processing_deadline(now)?);
        }
        Update::Renew => {}
        Update::Checkpoint(token) => current.continuation = Some((*token).clone()),
        Update::Release => current.owner = None,
    }
    current.renewed_at = Some(now);
    Ok(current)
}

fn now_millis() -> i64 {
    (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as i64
}

fn lost() -> crate::CosmosError {
    error(
        StatusCode::PreconditionFailed,
        "Processing lease ownership was lost.",
    )
}

async fn sleep(duration: Duration) {
    azure_core::sleep(
        azure_core::time::Duration::try_from(duration).expect("processor durations were validated"),
    )
    .await;
}

fn signal() -> (oneshot::Sender<()>, Signal) {
    let (tx, rx) = oneshot::channel();
    (
        tx,
        async move {
            let _ = rx.await;
        }
        .boxed()
        .shared(),
    )
}

struct Acknowledgment {
    result: oneshot::Sender<Result<()>>,
    durable: bool,
}

/// A delivery receipt tied to one batch and one acquisition of its lease.
///
/// Dropping this receipt without acknowledging it abandons the lease without
/// acknowledging this batch. It cannot authorize arbitrary continuations.
pub struct BatchReceipt {
    sender: oneshot::Sender<Acknowledgment>,
    cancelled: Signal,
}

impl BatchReceipt {
    /// Persists this batch's progress after the application callback succeeds.
    ///
    /// Consumes the receipt. The next poll for this lease remains blocked until
    /// persistence succeeds. Delivery is at least once: callback effects cannot
    /// be committed atomically with this write.
    ///
    /// # Errors
    ///
    /// Returns an error for lost ownership, shutdown, or a failed checkpoint.
    /// An error does not prove that a timed-out service write was not committed.
    pub async fn checkpoint(self) -> Result<()> {
        self.complete(true).await
    }

    /// Acknowledges successful processing according to the checkpoint policy.
    ///
    /// Consumes the receipt and permits the next batch after acknowledgment.
    /// With [`CheckpointPolicy::Deferred`], success can mean only in-memory
    /// acceptance, not persistence. Multiple successful batches may replay.
    /// Use [`checkpoint()`](Self::checkpoint) to require durable persistence.
    ///
    /// # Errors
    ///
    /// Returns an error if the lease session ends or a required checkpoint fails.
    pub async fn acknowledge(self) -> Result<()> {
        self.complete(false).await
    }

    async fn complete(self, durable: bool) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(Acknowledgment {
                result: tx,
                durable,
            })
            .map_err(|_| lost())?;
        rx.await.map_err(|_| lost())?
    }

    /// Waits until this lease session ends, including ownership loss or forced shutdown.
    ///
    /// Consumers can race their callback against this signal. Cancellation
    /// cannot undo application side effects already performed.
    pub async fn cancelled(&self) {
        self.cancelled.clone().await;
    }
}

/// An owned, lease-scoped batch of change feed results and its processing receipt.
///
/// The body preserves the driver's wire envelopes and encoding. No application
/// item deserialization happens in the driver.
pub struct ChangeFeedBatch {
    lease_token: String,
    response: CosmosResponse,
    receipt: BatchReceipt,
}

impl ChangeFeedBatch {
    /// Separates the lease identity, response, and acknowledgment receipt.
    pub fn into_parts(self) -> (String, CosmosResponse, BatchReceipt) {
        (self.lease_token, self.response, self.receipt)
    }
}

#[derive(Default)]
struct SharedState {
    distributor: Arc<ChangeFeedRequestDistributor>,
    stopping: AtomicBool,
    forced: AtomicBool,
    failure: StdMutex<Option<crate::CosmosError>>,
    unreleased: StdMutex<HashMap<(String, Option<String>), LeaseDocument>>,
    error_observer: Option<Arc<ErrorObserver>>,
}

impl SharedState {
    fn close(&self, force: bool) {
        self.stopping.store(true, Ordering::Release);
        if force {
            self.forced.store(true, Ordering::Release);
        }
        self.distributor.stop_delivery(force);
    }

    fn notify(&self, lease: Option<&str>, err: crate::CosmosError) {
        if let Some(observer) = &self.error_observer {
            if std::panic::catch_unwind(AssertUnwindSafe(|| {
                observer(lease.map(str::to_owned), err);
            }))
            .is_err()
            {
                tracing::error!("Processor error observer panicked");
            }
        }
    }

    fn failed(&self, lease: Option<&str>, err: crate::CosmosError) {
        tracing::warn!(error = %err, "Processor lease operation failed");
        if self.stopping.load(Ordering::Acquire) {
            self.failure
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .get_or_insert(err.clone());
        }
        self.notify(lease, err);
    }
}

struct Owner {
    stop: StdMutex<Option<oneshot::Sender<()>>>,
    force: StdMutex<Option<oneshot::Sender<()>>>,
    stopped: Shared<BoxFuture<'static, Result<bool>>>,
    state: Arc<SharedState>,
}

impl Drop for Owner {
    fn drop(&mut self) {
        self.state.close(true);
    }
}

/// A running, schema-agnostic change feed processor.
///
/// Clones share a lifecycle. Dropping the last handle requests shutdown.
/// Lease renewal and balancing continue while consumers process batches.
///
/// Split errors create independently balanced child
/// leases from acknowledged progress. Merges retain separate EPK-slice leases
/// and their checkpoints. Split parents are deleted after their children are
/// established; incomplete transitions and failed deletions are retried.
///
/// Splits and merges are recovered automatically while processor workers
/// remain running. Only the affected lease's processing session is replaced;
/// no application stop or restart is required.
#[derive(Clone)]
pub struct Processor {
    owner: Arc<Owner>,
}

impl Processor {
    /// Bootstraps the workload and starts lease coordination.
    ///
    /// Source and lease containers may use different drivers. Consumers control
    /// feed-read concurrency through concurrent [`read_next_change_feed_batch()`](Self::read_next_change_feed_batch) calls. Background
    /// lease/feed failures are logged and isolated to the affected lease.
    ///
    /// # Errors
    ///
    /// Returns configuration, bootstrap, and restart-verification errors before
    /// starting background work. All-versions-and-deletes mode requires a now
    /// start or continuation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::{
    ///     change_feed_processor::{Processor, ProcessorOptions},
    ///     models::{ChangeFeedFactoryMode, ContainerReference, CosmosResponse}, CosmosDriver, Result,
    /// };
    /// use futures::future::BoxFuture;
    /// use std::sync::Arc;
    ///
    /// async fn consume(
    ///     driver: Arc<CosmosDriver>,
    ///     source: ContainerReference,
    ///     leases: ContainerReference,
    ///     process: fn(CosmosResponse) -> BoxFuture<'static, Result<()>>,
    /// ) -> azure_data_cosmos_driver::Result<()> {
    ///     let options = ProcessorOptions::new(
    ///         ChangeFeedFactoryMode::Incremental, "orders", "worker-01",
    ///     );
    ///     let processor = Processor::start(driver.clone(), source, driver, leases, options).await?;
    ///     if let Some(batch) = processor.read_next_change_feed_batch().await {
    ///         let (_lease, response, receipt) = batch.into_parts();
    ///         process(response).await?;
    ///         receipt.checkpoint().await?;
    ///     }
    ///     processor.stop().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn start(
        source_driver: Arc<CosmosDriver>,
        source: ContainerReference,
        lease_driver: Arc<CosmosDriver>,
        leases: ContainerReference,
        options: ProcessorOptions,
    ) -> Result<Self> {
        // 1. Reject invalid lease bounds, feed start positions, and checkpoint settings.
        if (options.max_leases > 0 && options.min_leases > options.max_leases)
            || (options.mode == ChangeFeedFactoryMode::FullFidelity
                && options.continuation.is_none()
                && !matches!(options.start, ChangeFeedStartFrom::Now))
            || options
                .continuation
                .as_ref()
                .is_some_and(|t| t.as_str().trim().is_empty())
        {
            return Err(error(
                StatusCode::BadRequest,
                "Invalid processor start or lease-count configuration.",
            ));
        }
        azure_core::time::Duration::try_from(options.poll_delay)
            .map_err(|_| error(StatusCode::BadRequest, "Poll delay is not representable."))?;
        options.checkpoint_policy.validate()?;
        // 2. Finish bootstrap before starting coordination; failures return without spawning a task.
        bootstrap::bootstrap_with_timeout(
            &source_driver,
            &source,
            &lease_driver,
            &leases,
            options.mode,
            &options.workload_name,
            &options.worker_name,
            options.bootstrap_timeout,
        )
        .await?;
        // 3. Share workload identity, clients, and configuration with runtime operations.
        let identity = Identity {
            version: SCHEMA_VERSION,
            account: source.account().endpoint().origin().ascii_serialization(),
            container_rid: source.rid().to_owned(),
            workload_name: options.workload_name.clone(),
            mode: options.mode,
        };
        let context = Arc::new(Context {
            namespace: namespace(&identity)?,
            identity,
            source_driver,
            source,
            lease_driver,
            leases,
            options,
        });
        // 4. Optionally verify existing leases before launching the worker.
        if context.options.verify_leases_on_restart {
            context.verify_leases().await?;
        }
        let state = Arc::new(SharedState {
            error_observer: context.options.error_observer.clone(),
            ..Default::default()
        });
        // 5. Create graceful-stop, forced-cancellation, and final-result channels.
        let (stop, stopping) = signal();
        let (force, forced) = signal();
        let (done, stopped) = oneshot::channel();
        // 6. Keep shutdown senders in Owner; callers share the same completion future.
        let owner = Arc::new(Owner {
            stop: StdMutex::new(Some(stop)),
            force: StdMutex::new(Some(force)),
            stopped: async move {
                stopped.await.map_err(|_| {
                    error(
                        StatusCode::InternalServerError,
                        "Processor task did not complete.",
                    )
                })?
            }
            .boxed()
            .shared(),
            state: state.clone(),
        });
        // 7. Launch lease coordination and supervision; convert task panics into processor errors.
        // The task holds shared state, not Owner, so the last handle closes stop.
        let task = azure_core::async_runtime::get_async_runtime().spawn(Box::pin(async move {
            let result = AssertUnwindSafe(run(context, state.clone(), stopping, forced))
                .catch_unwind()
                .await
                .unwrap_or_else(|_| {
                    let err = error(StatusCode::InternalServerError, "Processor task panicked.");
                    state.failed(None, err.clone());
                    Err(err)
                });
            state.distributor.close();
            let _ = done.send(result);
        }));
        // 8. Dropping the task handle leaves it running; return control through our own lifecycle handle.
        // Startup does not wait for lease acquisition or batch delivery, and invokes no SDK callbacks.
        drop(task);
        Ok(Self { owner })
    }

    /// Reports whether background coordination is running.
    pub fn is_running(&self) -> bool {
        self.owner.state.distributor.is_running()
    }

    /// Waits for an owned batch, or returns `None` when delivery has drained.
    ///
    /// Each pending call authorizes one acquisition read at a time. Idle reads
    /// rotate through eligible acquisitions until a batch is available.
    /// Without pending calls, no new feed reads start. Lease maintenance continues.
    /// A throttled acquisition yields this receive to other eligible leases.
    /// Retry-After expiry restores eligibility without starting a read.
    ///
    /// Read turns are bounded to five seconds or a shorter configured driver
    /// deadline; this receive can wait longer across idle reads and cooldown waits.
    /// On throttle-budget exhaustion, the acquisition finishes its final
    /// cooldown, reports failure, and attempts conditional release.
    ///
    /// Canceling this wait discards any unclaimed response without acknowledging
    /// its changes. A read already in flight can finish, but its batch is not
    /// retained for a later call. A later acquisition can replay unsaved changes.
    /// Progress from 304 responses is handled internally without callbacks.
    /// Eligible acquisitions take turns supplying batches; an empty set of owned
    /// leases waits for future acquisitions rather than ending the stream.
    ///
    /// During graceful shutdown, continue consuming already-issued batches and
    /// completing their receipts. `None` does not mean all receipts are settled.
    pub async fn read_next_change_feed_batch(&self) -> Option<ChangeFeedBatch> {
        self.owner
            .state
            .distributor
            .read_next_change_feed_batch()
            .await
    }

    /// Drains issued batches, saves acknowledged progress, and releases owned leases.
    ///
    /// Stops new acquisition and polling. Consumers must keep draining deliveries
    /// and completing receipts while waiting. Renewal continues during the drain.
    /// There is no callback timeout. Canceling this wait does not cancel cleanup.
    /// Concurrent callers observe the same outcome.
    ///
    /// # Errors
    ///
    /// Returns an error if draining or release fails, ownership is lost during
    /// draining, or shutdown is forced. Failed batches may be replayed.
    pub async fn stop(&self) -> Result<()> {
        self.owner.state.close(false);
        if let Some(stop) = self
            .owner
            .stop
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take()
        {
            let _ = stop.send(());
        }
        if self.owner.stopped.clone().await? {
            return Err(error(
                StatusCode::Conflict,
                "Processor shutdown was forced; batches may replay.",
            ));
        }
        Ok(())
    }

    /// Cancels outstanding receipts and waits for conditional lease release.
    ///
    /// Does not wait for consumer callbacks. Already-started service operations
    /// can still complete; cancellation cannot undo their writes or side effects.
    ///
    /// # Errors
    ///
    /// Returns cleanup and worker errors. Other workers recover unreleased leases
    /// after expiration. Concurrent graceful stop waiters report forced shutdown.
    pub async fn abort(&self) -> Result<()> {
        self.request_stop();
        self.owner.stopped.clone().await.map(|_| ())
    }

    /// Requests forced shutdown without waiting, including from a destructor.
    ///
    /// Use [`stop()`](Self::stop) to drain or [`abort()`](Self::abort) to await cleanup.
    pub fn request_stop(&self) {
        self.owner.state.close(true);
        if let Some(stop) = self
            .owner
            .force
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take()
        {
            let _ = stop.send(());
        }
    }
}

async fn run(
    context: Arc<Context>,
    state: Arc<SharedState>,
    stop: Signal,
    force: Signal,
) -> Result<bool> {
    // 1. Track concurrent lease supervisors and their IDs; begin balancing without an initial delay.
    let mut lease_pollers = FuturesUnordered::<BoxFuture<'static, String>>::new();
    let mut owned = HashSet::new();
    let mut balancing = balance(
        context.clone(),
        state.clone(),
        owned.clone(),
        Duration::ZERO,
        stop.clone(),
        force.clone(),
    )
    .boxed();
    loop {
        // 2. Poll balancing and supervisors together; either shutdown signal begins cleanup.
        futures::select! {
            _ = force.clone().fuse() => break,
            _ = stop.clone().fuse() => break,
            id = async {
                if lease_pollers.is_empty() {
                    // Disable this branch until a supervisor exists, rather than spinning on an empty stream.
                    futures::future::pending().await
                } else {
                    lease_pollers.next().await
                }
            }.fuse() => {
                // 3. A finished supervisor no longer counts as locally owned in later balancing passes.
                if let Some(id) = id { owned.remove(&id); }
            },
            leases = balancing.as_mut().fuse() => {
                // 4. Supervise each newly acquired lease once; SDK callbacks run outside this loop.
                for lease in leases {
                    let id = lease.value.record.id.clone();
                    if owned.insert(id.clone()) {
                        let context = context.clone();
                        let state = state.clone();
                        let stop = stop.clone();
                        let force = force.clone();
                        lease_pollers.push(supervise(context, state, lease, stop, force).map(move |()| id).boxed());
                    }
                }
                // Use the current ownership snapshot and acquisition interval for the next pass.
                balancing = balance(context.clone(), state.clone(), owned.clone(), context.options.acquire_interval(), stop.clone(), force.clone()).boxed();
            },
        }
    }
    // 5. Finish in-flight acquisition writes and supervise their results with shutdown already signaled.
    // Dropping balancing here could forget a lease whose ownership write succeeded.
    for lease in balancing.await {
        let context = context.clone();
        let state = state.clone();
        let stop = stop.clone();
        let force = force.clone();
        let id = lease.value.record.id.clone();
        lease_pollers.push(
            supervise(context, state, lease, stop, force)
                .map(move |()| id)
                .boxed(),
        );
    }
    // 6. Join every supervisor so its processing and lease cleanup finish before final release retries.
    while lease_pollers.next().await.is_some() {}
    // 7. Retry recorded release failures, but never release ownership taken over by a replacement.
    let unreleased =
        std::mem::take(&mut *state.unreleased.lock().unwrap_or_else(|e| e.into_inner()));
    for observed in unreleased.into_values() {
        let result = async {
            let Some(current) = context
                .store()
                .read::<LeaseDocument>(&observed.record.id)
                .await?
            else {
                return Ok(());
            };
            context.validate(&current.0)?;
            if current.0.owner == observed.owner && current.0.incarnation == observed.incarnation {
                context.update(&mut current.into(), Update::Release).await?;
            }
            Ok(())
        }
        .await;
        if let Err(err) = result {
            state.failed(Some(&observed.record.id), err);
        }
    }
    // 8. Return a recorded failure, or whether shutdown was forced when cleanup otherwise succeeded.
    match state
        .failure
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .take()
    {
        Some(err) => Err(err),
        None => Ok(state.forced.load(Ordering::Acquire)),
    }
}

async fn balance(
    context: Arc<Context>,
    state: Arc<SharedState>,
    owned: HashSet<String>,
    delay: Duration,
    stop: Signal,
    force: Signal,
) -> Vec<LeaseState> {
    futures::select! {
        _ = sleep(delay).fuse() => {},
        _ = stop.fuse() => return Vec::new(),
        _ = force.fuse() => return Vec::new(),
    }
    if state.stopping.load(Ordering::Acquire) {
        return Vec::new();
    }
    let result = async {
        let active = context.list().await?;
        let mut acquired = Vec::new();
        let selected = balancing::select(&active, &context.options, now_millis());
        for index in selected {
            if state.stopping.load(Ordering::Acquire) {
                break;
            }
            if owned.contains(&active[index].record.id) {
                continue;
            }
            match context.acquire_tracked(&active[index], Some(&state)).await {
                Ok(lease) => acquired.push(lease),
                Err(err) if err.status().is_precondition_failed() => {
                    tracing::debug!(error = %err, "Lease acquisition lost a race");
                }
                Err(err) => state.failed(Some(&active[index].record.id), err),
            }
        }
        Ok::<_, crate::CosmosError>(acquired)
    }
    .await;
    match result {
        Ok(acquired) => acquired,
        Err(err) => {
            state.failed(None, err);
            Vec::new()
        }
    }
}

/// Distinguishes source-feed topology failures from lease maintenance and processing failures.
#[derive(Debug)]
enum PollError {
    SourceTopologyChange(crate::CosmosError),
    Other(crate::CosmosError),
}

impl PollError {
    fn from_source(err: crate::CosmosError) -> Self {
        if err.status().is_partition_topology_change() {
            Self::SourceTopologyChange(err)
        } else {
            Self::Other(err)
        }
    }
}

impl From<crate::CosmosError> for PollError {
    fn from(err: crate::CosmosError) -> Self {
        Self::Other(err)
    }
}

impl From<PollError> for crate::CosmosError {
    fn from(err: PollError) -> Self {
        match err {
            PollError::SourceTopologyChange(err) | PollError::Other(err) => err,
        }
    }
}

async fn supervise(
    context: Arc<Context>,
    state: Arc<SharedState>,
    lease: LeaseState,
    stop: Signal,
    force: Signal,
) {
    // 1. Create delivery state for this ownership turn and share lease mutations under one mutex.
    let token = lease.value.record.id.clone();
    let session = state
        .distributor
        .acquire(token.clone(), lease.value.incarnation.clone());
    let lease = Arc::new(Mutex::new(lease));
    // Dropping this sender signals local cancellation to both processing and renewal.
    let (cancel, cancelled) = signal();
    let mut cancel = Some(cancel);
    let result = match session {
        Ok(session) => {
            // 2. Poll processing and renewal concurrently in this supervisor, without spawning tasks.
            let processing = poll(
                context.clone(),
                state.clone(),
                &session,
                lease.clone(),
                cancelled.clone(),
                stop,
                force,
            )
            .boxed()
            .fuse();
            let renewing = renew(context.clone(), lease.clone(), cancelled).fuse();
            futures::pin_mut!(processing, renewing);
            futures::select! {
                result = processing => {
                    // 3. Processing ended: cancel renewal and join it before touching ownership cleanup.
                    drop(cancel.take());
                    let renewed = renewing.await;
                    result.and(renewed.map_err(PollError::Other))
                },
                result = renewing => {
                    // Renewal ended: cancel processing and join its issued work before releasing the lease.
                    drop(cancel.take());
                    let processed = processing.await;
                    result.map_err(PollError::Other).and(processed)
                },
            }
        }
        Err(err) => Err(PollError::Other(err)),
    };
    drop(cancel);
    // 4. Only an explicit source-feed topology failure can trigger recovery, never a lease-store error.
    let result = match result {
        Err(PollError::SourceTopologyChange(err)) if !state.stopping.load(Ordering::Acquire) => {
            match context.recover(lease.clone()).await {
                Ok(true) => return,
                Ok(false) => Err(err),
                Err(err) => Err(err),
            }
        }
        result => result.map_err(crate::CosmosError::from),
    };
    // 5. Report the combined processing/renewal or recovery failure before attempting fenced release.
    if let Err(err) = result {
        tracing::warn!(lease = %token, error = %err, "Lease processing stopped");
        state.failed(Some(&token), err);
    }
    let mut lease = lease.lock().await;
    if let Err(err) = context.update(&mut lease, Update::Release).await {
        // 6. Preserve failed release state for the coordinator's final ownership-checked cleanup.
        tracing::warn!(lease = %token, error = %err, "Lease release failed; ownership will expire");
        state.failed(Some(&token), err);
        state
            .unreleased
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(
                (token, lease.value.incarnation.clone()),
                lease.value.clone(),
            );
    }
}

async fn renew(
    context: Arc<Context>,
    lease: Arc<Mutex<LeaseState>>,
    stopped: Signal,
) -> Result<()> {
    let interval = context.options.renew_interval();
    loop {
        let delay = {
            let lease = lease.lock().await;
            interval.saturating_sub(
                lease
                    .renewed
                    .map(|updated| updated.elapsed())
                    .unwrap_or_default(),
            )
        };
        futures::select_biased! {
            _ = stopped.clone().fuse() => return Ok(()),
            _ = sleep(delay).fuse() => {},
        }
        let mut lease = lease.lock().await;
        if stopped.clone().now_or_never().is_some() {
            return Ok(());
        }
        let elapsed = lease
            .renewed
            .map(|updated| updated.elapsed())
            .unwrap_or(context.options.expiration);
        if elapsed < interval {
            continue;
        }
        if elapsed >= context.options.expiration {
            return Err(error(
                StatusCode::RequestTimeout,
                "Lease renewal budget expired.",
            ));
        }
        match context.update(&mut lease, Update::Renew).await {
            Ok(()) => {}
            Err(err) if err.status().is_precondition_failed() || err.status().is_not_found() => {
                return Err(err)
            }
            Err(err) => {
                tracing::warn!(error = %err, "Lease renewal failed; retrying within the expiration budget");
                drop(lease);
                futures::select_biased! {
                    _ = stopped.clone().fuse() => return Ok(()),
                    _ = sleep(interval).fuse() => {},
                }
            }
        }
    }
}

async fn poll(
    context: Arc<Context>,
    state: Arc<SharedState>,
    session: &AcquisitionDelivery,
    lease: Arc<Mutex<LeaseState>>,
    cancelled: Signal,
    stop: Signal,
    force: Signal,
) -> std::result::Result<(), PollError> {
    // Run this acquisition's read loop, then settle only its previously acknowledged progress.
    let result = poll_feed(
        context.clone(),
        state.clone(),
        session,
        lease.clone(),
        cancelled.clone(),
        stop,
        force,
    )
    .await;
    if !state.forced.load(Ordering::Acquire) {
        let mut lease = lease.lock().await;
        // Do not flush after forced cancellation, a failed checkpoint, or lost ownership.
        if !state.forced.load(Ordering::Acquire)
            && !lease.pending.as_ref().is_some_and(|pending| pending.failed)
        {
            if cancelled.clone().now_or_never().is_some() {
                return Err(lost().into());
            }
            // Final safe-progress barrier for either policy; write failures remain PollError::Other.
            context.flush(&mut lease).await?;
        }
    }
    result
}

async fn poll_feed(
    context: Arc<Context>,
    state: Arc<SharedState>,
    session: &AcquisitionDelivery,
    lease: Arc<Mutex<LeaseState>>,
    cancelled: Signal,
    stop: Signal,
    force: Signal,
) -> std::result::Result<(), PollError> {
    if state.stopping.load(Ordering::Acquire) {
        return Ok(());
    }
    // 1. Prepare the lease-scoped operation and resume anchor; saved progress takes precedence.
    let initial = lease.lock().await.value.clone();
    let operation = context
        .operation(&initial.range)
        .with_max_item_count(context.options.page_size);
    let checkpoint = context.checkpoint(&initial, &initial.range)?;
    let plan_options = PlanOptions {
        change_feed_processor: true,
        ..Default::default()
    };
    let (mut retry, read_options, read_timeout) = ReadRetry::for_driver(&context.source_driver);
    let mut planning_options = operation_options();
    planning_options.end_to_end_latency_policy = Some(read_timeout.into());
    // Keep the evolving read plan locally; creating it is deferred until a receiver requests data.
    let mut plan = None;
    loop {
        if cancelled.clone().now_or_never().is_some() {
            return Err(lost().into());
        }
        if state.stopping.load(Ordering::Acquire) {
            return Ok(());
        }
        // 2. Wait for receiver demand while servicing deferred-checkpoint deadlines and cancellation.
        let Some(Some(demand)) = context
            .wait_with_checkpoint(&lease, &cancelled, &force, session.wait_for_demand())
            .await?
        else {
            return Ok(());
        };
        if state.stopping.load(Ordering::Acquire) {
            return Ok(());
        }
        if demand.is_canceled() {
            continue;
        }
        // 3. Share one read-turn deadline across initial planning and execution.
        let deadline = Instant::now() + read_timeout;
        if plan.is_none() {
            plan = context
                .wait_with_checkpoint(&lease, &cancelled, &force, async {
                    Ok(ReadRetry::bounded(
                        deadline,
                        context.source_driver.plan_operation(
                            operation.clone(),
                            &planning_options,
                            Some(&checkpoint),
                            &plan_options,
                        ),
                    )
                    .await)
                })
                .await?
                .transpose()
                .map_err(PollError::from_source)?;
        }
        let Some(plan) = plan.as_mut() else {
            return Ok(());
        };
        if state.stopping.load(Ordering::Acquire) {
            return Ok(());
        }
        if demand.is_canceled() {
            continue;
        }
        // Execute the existing plan once; renewal continues independently in the supervisor.
        let response = context
            .wait_with_checkpoint(
                &lease,
                &cancelled,
                &force,
                // Keep source errors inside the result so maintenance errors retain their own origin.
                async {
                    Ok(ReadRetry::bounded(
                        deadline,
                        context.source_driver.execute_plan(
                            plan,
                            Some(context.source.clone()),
                            read_options.clone(),
                        ),
                    )
                    .await)
                },
            )
            .await?;
        let Some(response) = response else {
            return Ok(());
        };
        let response = match response {
            Err(err) if ReadRetry::is_throttled(&err) => {
                // 4. Release this receiver to other eligible leases before waiting for cooldown.
                let cooldown = retry.cooldown(
                    err.response()
                        .and_then(|response| response.headers().retry_after_ms),
                    Instant::now(),
                )?;
                let failure = (!cooldown.retry).then_some(err);
                drop(demand);
                tracing::debug!(lease = %initial.record.id, retry = cooldown.retry,
                    delay_ms = cooldown.until.saturating_duration_since(Instant::now()).as_millis(),
                    "Deferring throttled change feed acquisition");
                let waiting = async {
                    futures::select_biased! {
                        _ = stop.clone().fuse() => Ok(false),
                        _ = sleep(cooldown.until.saturating_duration_since(Instant::now())).fuse() => Ok(true),
                    }
                };
                if context
                    .wait_with_checkpoint(&lease, &cancelled, &force, waiting)
                    .await?
                    != Some(true)
                {
                    return Ok(());
                }
                if let Some(err) = failure {
                    // Exhausted retries end this acquisition after its cooldown, not the whole worker.
                    return Err(err.into());
                }
                continue;
            }
            result => result.map_err(PollError::from_source)?,
        };
        let response = response.ok_or_else(|| {
            error(
                StatusCode::InternalServerError,
                "Change feed unexpectedly drained.",
            )
        })?;
        retry.reset();
        if state.stopping.load(Ordering::Acquire) {
            return Ok(());
        }
        if response.headers().etag.is_none() {
            return Err(error(
                StatusCode::InternalServerError,
                "Change feed response has no checkpoint ETag.",
            )
            .into());
        }
        let idle = response.status().status_code() == StatusCode::NotModified;
        // The plan's read position is not durable application progress until this turn is safe.
        let token = plan.to_continuation_token()?;
        if idle {
            // 5. Idle progress needs no callback; Eager persists it and Deferred retains only its token.
            drop(response);
            let mut stored = lease.lock().await;
            if stored
                .value
                .continuation
                .as_ref()
                .map(ContinuationToken::as_str)
                != Some(token.as_str())
            {
                stored.acknowledge(token, 0);
                if context.options.checkpoint_policy == CheckpointPolicy::Eager {
                    context.flush(&mut stored).await?;
                }
            }
            drop(stored);
            // Yield demand before this lease's idle delay so other leases can serve the receiver.
            drop(demand);
            let waiting = async {
                futures::select! {
                _ = sleep(context.options.poll_delay).fuse() => {},
                _ = stop.clone().fuse() => {},
                }
                Ok(())
            };
            if context
                .wait_with_checkpoint(&lease, &cancelled, &force, waiting)
                .await?
                .is_none()
            {
                return Ok(());
            }
            continue;
        }
        let handled_changes = match response.body() {
            ResponseBody::NoPayload => 0,
            ResponseBody::Items(items) => items.len() as u64,
            ResponseBody::Bytes(body) => split_feed_envelope(body)?.len() as u64,
        };
        let (sender, received) = oneshot::channel();
        // 6. Move the response to its original receiver with a one-use acknowledgment receipt.
        if !session.deliver(
            demand,
            response,
            BatchReceipt {
                sender,
                cancelled: cancelled.clone(),
            },
        )? {
            return Ok(());
        }
        // 7. Do not read another batch until receipt completion; dropped receipts abandon the lease.
        let acknowledgment = context
            .wait_with_checkpoint(&lease, &cancelled, &force, async {
                received.await.map_err(|_| {
                    error(
                        StatusCode::Conflict,
                        "Batch was abandoned without successful processing.",
                    )
                })
            })
            .await?;
        let Some(acknowledgment) = acknowledgment else {
            return Ok(());
        };
        let mut lease = lease.lock().await;
        if cancelled.clone().now_or_never().is_some() || state.forced.load(Ordering::Acquire) {
            return Err(lost().into());
        }
        // 8. Only successful processing advances pending progress; nonempty batches update activity.
        lease.acknowledge(token, handled_changes);
        if handled_changes > 0 {
            lease
                .value
                .processed(context.options.checkpoint_policy, now_millis())?;
        }
        // 9. Eager and explicit checkpoints wait for persistence; Deferred may acknowledge in memory.
        let result = if acknowledgment.durable
            || context.options.checkpoint_policy == CheckpointPolicy::Eager
        {
            // Eager (or an explicit durable receipt) must finish the lease write before acknowledging.
            context.flush(&mut lease).await
        } else {
            // Deferred accepts progress in memory; a deadline or final barrier performs the later write.
            Ok(())
        };
        // 10. Reopen delivery only after the required checkpoint succeeds, then complete the receipt.
        let result = result.and_then(|()| session.acknowledged());
        let failed = result.is_err();
        let _ = acknowledgment.result.send(result);
        if failed {
            return Err(error(
                StatusCode::ServiceUnavailable,
                "Checkpoint failed; abandoning the lease.",
            )
            .into());
        }
    }
}
