// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    fmt,
    future::Future,
    num::{NonZeroU32, NonZeroU64},
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
    time::Duration,
};

use azure_data_cosmos_driver::{
    models::{ContinuationToken, FeedRange},
    CosmosError, CosmosErrorBuilder, DiagnosticsContext,
};
use futures::FutureExt;
use tokio::{
    sync::watch,
    time::{sleep, sleep_until, Instant},
};

use crate::{BatchState, RawBatchSource, RawChangeFeedPage};

mod decisions;
use decisions::{checkpoint_retry, validate_checkpoint_receipt, CheckpointRetry};

/// An already-owned lease with a confirmed recoverable position and store revision.
///
/// This slice does not acquire or renew ownership. Callers must signal loss
/// before its safety deadline and use a store that enforces conditional writes.
#[derive(Clone, Debug)]
pub struct OwnedLease {
    id: String,
    owner: String,
    epoch: NonZeroU64,
    revision: String,
    range: FeedRange,
    checkpoint: ContinuationToken,
}

impl OwnedLease {
    /// Describes confirmed lease state obtained from the store.
    ///
    /// # Errors
    ///
    /// Rejects empty identity, revision, or checkpoint values and non-interval scopes.
    pub fn new(
        id: impl Into<String>,
        owner: impl Into<String>,
        epoch: NonZeroU64,
        revision: impl Into<String>,
        range: FeedRange,
        checkpoint: ContinuationToken,
    ) -> azure_data_cosmos_driver::Result<Self> {
        let lease = Self {
            id: id.into(),
            owner: owner.into(),
            epoch,
            revision: revision.into(),
            range,
            checkpoint,
        };
        if lease.range.is_logical_partition()
            || lease.range.min_inclusive() >= lease.range.max_exclusive()
        {
            return Err(client_error(
                "owned lease requires a non-empty explicit EPK range",
            ));
        }
        if lease.id.trim().is_empty()
            || lease.owner.trim().is_empty()
            || lease.revision.trim().is_empty()
            || lease.checkpoint.as_str().is_empty()
        {
            return Err(client_error(
                "owned lease requires non-empty identity, revision, and checkpoint",
            ));
        }
        Ok(lease)
    }
    /// Returns the lease identity.
    pub fn id(&self) -> &str {
        &self.id
    }
    /// Returns the owner identity.
    pub fn owner(&self) -> &str {
        &self.owner
    }
    /// Returns the ownership epoch.
    pub fn epoch(&self) -> NonZeroU64 {
        self.epoch
    }
    /// Returns the authoritative store revision.
    pub fn revision(&self) -> &str {
        &self.revision
    }
    /// Returns the lease's source range.
    pub fn range(&self) -> &FeedRange {
        &self.range
    }
    /// Returns confirmed durable progress, not fetched progress.
    pub fn checkpoint(&self) -> &ContinuationToken {
        &self.checkpoint
    }
}

/// Classification supplied by the checkpoint store, not inferred from HTTP status.
#[derive(Debug)]
pub enum CheckpointError {
    /// The store guarantees the write did not take effect; bounded retry is safe.
    Retryable(CosmosError),
    /// The result may have taken effect. Stop and reconcile before further processing.
    Ambiguous(CosmosError),
    /// Ownership, revision, or another condition rejected the write.
    Rejected(CosmosError),
}

/// Minimal persistence seam for this foundation's conditional checkpoint writes.
pub trait CheckpointStore: Send {
    /// Persists the candidate only if owner, epoch, and revision still match.
    ///
    /// `Ok` must return confirmed durable state, including the new revision and
    /// exact candidate. A timeout or unknown outcome must return `Ambiguous`,
    /// never `Ok` or `Retryable`. Implementations preserve all other lease fields.
    fn persist<'a>(
        &'a mut self,
        lease: &'a OwnedLease,
        candidate: &'a ContinuationToken,
    ) -> futures::future::BoxFuture<'a, Result<OwnedLease, CheckpointError>>;
}

#[derive(Clone, Copy, Debug, Default)]
struct ControlState {
    stop: bool,
    lost: bool,
}

/// Monotonic stop and simulated ownership-loss signals for one lease run.
#[derive(Clone, Debug)]
pub struct LeaseControl(watch::Sender<ControlState>);

impl Default for LeaseControl {
    fn default() -> Self {
        Self(watch::channel(ControlState::default()).0)
    }
}
impl LeaseControl {
    pub(crate) fn is_lost(&self) -> bool {
        self.0.borrow().lost
    }
    pub(crate) async fn interrupted(&self) -> LeaseRunOutcome {
        let mut signals = self.0.subscribe();
        loop {
            let state = *signals.borrow_and_update();
            if state.lost {
                return LeaseRunOutcome::OwnershipLost;
            }
            if state.stop {
                return LeaseRunOutcome::StoppedDrained;
            }
            if signals.changed().await.is_err() {
                return LeaseRunOutcome::OwnershipLost;
            }
        }
    }
    /// Stops new reads/delivery and starts the bounded drain policy.
    pub fn stop(&self) {
        self.0.send_modify(|state| state.stop = true);
    }
    /// Revokes local ownership. Obsolete handler completions cannot checkpoint.
    pub fn lose_ownership(&self) {
        self.0.send_modify(|state| state.lost = true);
    }
}

/// Bounds a single-lease run and its persistence retries.
#[derive(Clone, Debug)]
pub struct LeaseRunOptions {
    max_batches: NonZeroU32,
    max_duration: Duration,
    drain_timeout: Duration,
    checkpoint_attempts: NonZeroU32,
    retry_delay: Duration,
    idle_delay: Duration,
}

impl LeaseRunOptions {
    /// Sets the required maximum number of confirmed pages, including idle pages.
    pub fn new(max_batches: NonZeroU32) -> Self {
        Self {
            max_batches,
            max_duration: Duration::from_secs(60),
            drain_timeout: Duration::from_secs(5),
            checkpoint_attempts: NonZeroU32::new(3).expect("three is non-zero"),
            retry_delay: Duration::from_millis(100),
            idle_delay: Duration::from_millis(100),
        }
    }
    /// Sets the total run deadline, including application handling.
    pub fn with_max_duration(mut self, value: Duration) -> Self {
        self.max_duration = value;
        self
    }
    /// Sets the time allowed to drain after stop; zero means immediate interruption.
    pub fn with_drain_timeout(mut self, value: Duration) -> Self {
        self.drain_timeout = value;
        self
    }
    /// Sets the maximum definitely-not-persisted attempts for one candidate.
    pub fn with_checkpoint_attempts(mut self, value: NonZeroU32) -> Self {
        self.checkpoint_attempts = value;
        self
    }
    /// Sets the delay between safe persistence retries.
    pub fn with_retry_delay(mut self, value: Duration) -> Self {
        self.retry_delay = value;
        self
    }
    /// Sets the delay after an idle page is durably confirmed.
    pub fn with_idle_delay(mut self, value: Duration) -> Self {
        self.idle_delay = value;
        self
    }
    /// Returns the confirmed-page limit.
    pub fn max_batches(&self) -> NonZeroU32 {
        self.max_batches
    }
    /// Returns the total time budget.
    pub fn max_duration(&self) -> Duration {
        self.max_duration
    }
    /// Returns the stop-drain budget.
    pub fn drain_timeout(&self) -> Duration {
        self.drain_timeout
    }
    /// Returns the persistence-attempt limit.
    pub fn checkpoint_attempts(&self) -> NonZeroU32 {
        self.checkpoint_attempts
    }
    /// Returns the persistence-retry delay.
    pub fn retry_delay(&self) -> Duration {
        self.retry_delay
    }
    /// Returns the idle-poll delay.
    pub fn idle_delay(&self) -> Duration {
        self.idle_delay
    }
}

/// The stage interrupted or failed by a run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeaseRunPhase {
    /// Fetching the next page or waiting before an idle poll.
    Reading,
    /// Awaiting application processing completion.
    Handling,
    /// Awaiting conditional persistence or a safe retry.
    Checkpointing,
}

/// Why a bounded lease run ended; not all outcomes indicate successful draining.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeaseRunOutcome {
    /// A source, handler, or checkpoint error ended the run.
    Failed,
    /// The requested number of pages was handled and durably confirmed.
    BatchLimitReached,
    /// Stop completed with no unconfirmed candidate.
    StoppedDrained,
    /// Stop prevented delivery of a fetched batch; the candidate remains unprocessed.
    StoppedUnprocessed,
    /// Stop's drain deadline expired with unfinished or uncertain work.
    DrainTimedOut,
    /// Ownership was revoked; a pending store operation may require reconciliation.
    OwnershipLost,
    /// The total run budget expired.
    RunTimedOut,
}

#[derive(Debug)]
enum ProcessingProgress {
    Ready,
    Reading,
    Handling(ContinuationToken),
    Checkpointing(ContinuationToken),
}

impl ProcessingProgress {
    fn candidate(&self) -> Option<&ContinuationToken> {
        match self {
            Self::Handling(candidate) | Self::Checkpointing(candidate) => Some(candidate),
            Self::Ready | Self::Reading => None,
        }
    }
    fn phase(&self) -> Option<LeaseRunPhase> {
        match self {
            Self::Ready => None,
            Self::Reading => Some(LeaseRunPhase::Reading),
            Self::Handling(_) => Some(LeaseRunPhase::Handling),
            Self::Checkpointing(_) => Some(LeaseRunPhase::Checkpointing),
        }
    }
}

/// Confirmed progress and any unconfirmed candidate at the end of a run.
#[derive(Debug)]
pub struct LeaseRunReport {
    lease: OwnedLease,
    progress: ProcessingProgress,
    confirmed_batches: u32,
    outcome: LeaseRunOutcome,
    diagnostics: Option<Arc<DiagnosticsContext>>,
}
impl LeaseRunReport {
    pub(crate) fn update_authoritative_lease(&mut self, lease: OwnedLease) {
        self.lease = lease;
    }
    pub(crate) fn before_read(lease: OwnedLease, outcome: LeaseRunOutcome) -> Self {
        Self {
            lease,
            progress: ProcessingProgress::Reading,
            confirmed_batches: 0,
            outcome,
            diagnostics: None,
        }
    }
    /// Returns the last confirmed durable lease state.
    pub fn lease(&self) -> &OwnedLease {
        &self.lease
    }
    /// Returns fetched progress that has not been confirmed durable.
    pub fn candidate(&self) -> Option<&ContinuationToken> {
        self.progress.candidate()
    }
    /// Returns the stage interrupted or failed, when applicable.
    pub fn phase(&self) -> Option<LeaseRunPhase> {
        self.progress.phase()
    }
    /// Returns the number of confirmed pages.
    pub fn confirmed_batches(&self) -> u32 {
        self.confirmed_batches
    }
    /// Returns the explicit completion/drain outcome.
    pub fn outcome(&self) -> LeaseRunOutcome {
        self.outcome
    }
    /// Returns diagnostics of the last fetched page, when available.
    pub fn diagnostics(&self) -> Option<&Arc<DiagnosticsContext>> {
        self.diagnostics.as_ref()
    }
}

/// A processing failure together with the progress required for safe recovery.
#[derive(Debug)]
pub struct LeaseRunError {
    error: CosmosError,
    report: LeaseRunReport,
}
impl LeaseRunError {
    pub(crate) fn update_authoritative_lease(&mut self, lease: OwnedLease) {
        self.report.update_authoritative_lease(lease);
    }
    pub(crate) fn before_read(lease: OwnedLease, error: CosmosError) -> Self {
        fail(
            LeaseRunReport {
                lease,
                progress: ProcessingProgress::Reading,
                confirmed_batches: 0,
                outcome: LeaseRunOutcome::RunTimedOut,
                diagnostics: None,
            },
            error,
        )
    }
    /// Returns the failure, including any retained diagnostics.
    pub fn error(&self) -> &CosmosError {
        &self.error
    }
    /// Returns durable and candidate positions and the failed phase.
    pub fn report(&self) -> &LeaseRunReport {
        &self.report
    }
}
impl fmt::Display for LeaseRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}
impl std::error::Error for LeaseRunError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

fn client_error(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
        .with_message(message)
        .build()
}

fn fail(mut report: LeaseRunReport, error: CosmosError) -> LeaseRunError {
    report.outcome = LeaseRunOutcome::Failed;
    let error = if error.diagnostics().is_none() {
        match &report.diagnostics {
            Some(diagnostics) => CosmosErrorBuilder::from_error(error)
                .with_diagnostics(diagnostics.clone())
                .build(),
            None => error,
        }
    } else {
        error
    };
    if report.diagnostics.is_none() {
        report.diagnostics = error.diagnostics();
    }
    LeaseRunError { error, report }
}

enum Step<T> {
    Completed(T),
    Interrupted(LeaseRunOutcome),
}

async fn complete_callback<H, F>(
    handler: &mut H,
    batch: RawChangeFeedPage,
) -> azure_data_cosmos_driver::Result<()>
where
    H: FnMut(RawChangeFeedPage) -> F,
    F: Future<Output = azure_data_cosmos_driver::Result<()>>,
{
    // Only the application boundary is caught; this lease exits on panic without retrying it.
    let future = catch_unwind(AssertUnwindSafe(|| handler(batch))).map_err(|_| {
        client_error("application callback panicked before returning its completion future")
    })?;
    AssertUnwindSafe(future)
        .catch_unwind()
        .await
        .map_err(|_| client_error("application callback completion future panicked"))?
}

struct RunBudget {
    deadline: Instant,
    drain_deadline: Option<Instant>,
    drain_timeout: Duration,
}

impl RunBudget {
    async fn wait<T>(
        &mut self,
        future: impl Future<Output = T>,
        signals: &mut watch::Receiver<ControlState>,
        phase: LeaseRunPhase,
    ) -> Step<T> {
        tokio::pin!(future);
        loop {
            let state = *signals.borrow_and_update();
            if state.lost {
                return Step::Interrupted(LeaseRunOutcome::OwnershipLost);
            }
            if state.stop {
                if phase == LeaseRunPhase::Reading {
                    return Step::Interrupted(LeaseRunOutcome::StoppedDrained);
                }
                self.drain_deadline.get_or_insert_with(|| {
                    // Draining never extends the total run deadline.
                    Instant::now()
                        .checked_add(self.drain_timeout)
                        .unwrap_or(self.deadline)
                });
            }
            let deadline = self
                .drain_deadline
                .map_or(self.deadline, |drain| drain.min(self.deadline));
            if Instant::now() >= deadline {
                return Step::Interrupted(
                    if self
                        .drain_deadline
                        .is_some_and(|drain| drain <= self.deadline)
                    {
                        LeaseRunOutcome::DrainTimedOut
                    } else {
                        LeaseRunOutcome::RunTimedOut
                    },
                );
            }
            tokio::select! {
                biased;
                changed = signals.changed() => {
                    if changed.is_err() { return Step::Interrupted(LeaseRunOutcome::OwnershipLost); }
                },
                _ = sleep_until(deadline) => return Step::Interrupted(
                    if self.drain_deadline.is_some_and(|drain| drain <= self.deadline) {
                        LeaseRunOutcome::DrainTimedOut
                    } else { LeaseRunOutcome::RunTimedOut }),
                value = &mut future => return Step::Completed(value),
            }
        }
    }
}

/// Processes one already-owned lease with at most one outstanding raw batch.
///
/// The source must start from `lease.checkpoint()` and cover `lease.range()`.
/// The handler future's success acknowledges that batch. No next read occurs
/// until the store confirms the exact candidate and a new conditional revision.
/// Idle pages bypass the handler but still persist their position.
///
/// Callback success must mean its actual application work has completed, not
/// merely been scheduled elsewhere. Unwinding callback panics are failures;
/// aborting panics cannot be caught. Cancellation/ownership loss never checkpoint
/// unfinished handling. A crash after callback success but before persistence
/// can redeliver that batch; application side effects are not atomic with this store.
///
/// Stop allows bounded drain of handling/persistence; ownership loss revokes it.
/// Interrupted persistence can have an unknown store outcome and requires
/// reconciliation. This function neither acquires, renews, nor releases leases.
///
/// # Errors
///
/// Returns source, handler/unwinding-panic, rejected/ambiguous persistence, exhausted safe-retry,
/// or invalid receipt failures with confirmed/candidate progress and diagnostics.
pub async fn run_lease<S, C, H, F>(
    source: &mut S,
    store: &mut C,
    lease: OwnedLease,
    control: &LeaseControl,
    options: &LeaseRunOptions,
    mut handler: H,
) -> Result<LeaseRunReport, LeaseRunError>
where
    S: RawBatchSource,
    C: CheckpointStore,
    H: FnMut(RawChangeFeedPage) -> F,
    F: Future<Output = azure_data_cosmos_driver::Result<()>>,
{
    let mut report = LeaseRunReport {
        lease,
        progress: ProcessingProgress::Ready,
        confirmed_batches: 0,
        outcome: LeaseRunOutcome::BatchLimitReached,
        diagnostics: None,
    };
    let mut signals = control.0.subscribe();
    let Some(deadline) = Instant::now().checked_add(options.max_duration) else {
        return Err(fail(
            report,
            client_error("run duration exceeds the timer range"),
        ));
    };
    let mut budget = RunBudget {
        deadline,
        drain_deadline: None,
        drain_timeout: options.drain_timeout,
    };
    loop {
        let state = *signals.borrow();
        if state.lost || state.stop {
            report.outcome = if state.lost {
                LeaseRunOutcome::OwnershipLost
            } else {
                LeaseRunOutcome::StoppedDrained
            };
            return Ok(report);
        }
        if report.confirmed_batches >= options.max_batches.get() {
            return Ok(report);
        }
        report.progress = ProcessingProgress::Reading;
        let raw = match budget
            .wait(source.read_batch(), &mut signals, LeaseRunPhase::Reading)
            .await
        {
            Step::Completed(Ok(raw)) => raw,
            Step::Completed(Err(error)) => return Err(fail(report, error)),
            Step::Interrupted(outcome) => {
                report.outcome = outcome;
                return Ok(report);
            }
        };
        let candidate = raw.continuation().clone();
        report.progress = ProcessingProgress::Handling(candidate.clone());
        report.diagnostics = Some(raw.response().diagnostics());
        let idle = raw.state() == BatchState::Idle;
        if !idle {
            // Do not invoke application code after a signal observed between fetch and delivery.
            let state = *signals.borrow();
            if state.lost || state.stop {
                report.outcome = if state.lost {
                    LeaseRunOutcome::OwnershipLost
                } else {
                    LeaseRunOutcome::StoppedUnprocessed
                };
                return Ok(report);
            }
            match budget
                .wait(
                    complete_callback(&mut handler, raw),
                    &mut signals,
                    LeaseRunPhase::Handling,
                )
                .await
            {
                Step::Completed(Ok(())) => {}
                Step::Completed(Err(error)) => return Err(fail(report, error)),
                Step::Interrupted(outcome) => {
                    report.outcome = outcome;
                    return Ok(report);
                }
            }
        }
        report.progress = ProcessingProgress::Checkpointing(candidate.clone());
        let mut attempt = 0;
        loop {
            attempt += 1;
            let result = budget
                .wait(
                    // Polling the store future is guarded against ownership loss by wait().
                    async { store.persist(&report.lease, &candidate).await },
                    &mut signals,
                    LeaseRunPhase::Checkpointing,
                )
                .await;
            match result {
                Step::Completed(Ok(receipt)) => {
                    if let Err(error) =
                        validate_checkpoint_receipt(&report.lease, &candidate, &receipt)
                    {
                        return Err(fail(report, error));
                    }
                    report.lease = receipt;
                    report.progress = ProcessingProgress::Ready;
                    report.confirmed_batches += 1;
                    break;
                }
                Step::Completed(Err(error)) => {
                    let decision = checkpoint_retry(&error, attempt, options.checkpoint_attempts);
                    let (CheckpointError::Retryable(error)
                    | CheckpointError::Ambiguous(error)
                    | CheckpointError::Rejected(error)) = error;
                    match decision {
                        CheckpointRetry::Fail => return Err(fail(report, error)),
                        CheckpointRetry::Retry => {
                            tracing::warn!(attempt, error = %error, "checkpoint definitely not persisted; retrying candidate without rerunning handler");
                            if let Step::Interrupted(outcome) = budget
                                .wait(
                                    sleep(options.retry_delay),
                                    &mut signals,
                                    LeaseRunPhase::Checkpointing,
                                )
                                .await
                            {
                                report.outcome = outcome;
                                return Ok(report);
                            }
                        }
                    }
                }
                Step::Interrupted(outcome) => {
                    report.outcome = outcome;
                    return Ok(report);
                }
            }
        }
        if idle {
            report.progress = ProcessingProgress::Reading;
            if let Step::Interrupted(outcome) = budget
                .wait(
                    sleep(options.idle_delay),
                    &mut signals,
                    LeaseRunPhase::Reading,
                )
                .await
            {
                report.outcome = outcome;
                return Ok(report);
            }
            report.progress = ProcessingProgress::Ready;
        }
    }
}
