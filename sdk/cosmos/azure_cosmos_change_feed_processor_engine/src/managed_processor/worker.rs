// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    invalid, run_lease, sleep, Arc, BatchState, BoxFuture, ChangeFeedReadOptions, CheckpointError,
    CheckpointStore, ContinuationToken, CosmosError, Duration, LeaseControl, LeaseReleaseOutcome,
    LeaseRunOptions, LeaseRunOutcome, LeaseRunPhase, LeaseSession, ManagedLeaseShutdown,
    ManagedLeaseState, ManagedProcessorOptions, MaxItemCountHint, NonZeroU32, OwnedLease,
    ProcessorEngine, RawBatchSource, RawChangeFeedPage, RawChangeHandler, Result, Semaphore,
    Shared,
};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc;

pub(super) struct WorkerEvent {
    pub session: LeaseSession,
}

struct ObservedSource {
    reader: crate::ChangeFeedReader,
    shared: Shared,
    id: String,
    idle: bool,
    poll_interval: Duration,
}
impl RawBatchSource for ObservedSource {
    fn read_batch(&mut self) -> BoxFuture<'_, Result<RawChangeFeedPage>> {
        Box::pin(async {
            if self.idle {
                sleep(self.poll_interval).await;
            }
            let page = self.reader.read_page().await?;
            self.idle = page.state() == BatchState::Idle;
            let mut shared = self.shared.lock().expect("managed state lock poisoned");
            let snapshot = shared.leases.get_mut(&self.id).expect("registered lease");
            snapshot.last_feed = Some(page.continuation().clone());
            snapshot.state = ManagedLeaseState::Running;
            Ok(page)
        })
    }
}

struct ObservedStore {
    session: LeaseSession,
    shared: Shared,
    id: String,
}
impl CheckpointStore for ObservedStore {
    fn persist<'a>(
        &'a mut self,
        lease: &'a OwnedLease,
        candidate: &'a ContinuationToken,
    ) -> BoxFuture<'a, std::result::Result<OwnedLease, CheckpointError>> {
        Box::pin(async move {
            let receipt = self.session.checkpoint(lease, candidate).await?;
            self.shared
                .lock()
                .expect("managed state lock poisoned")
                .leases
                .get_mut(&self.id)
                .expect("registered lease")
                .last_checkpoint = receipt.checkpoint().clone();
            Ok(receipt)
        })
    }
}

struct End {
    lease: OwnedLease,
    outcome: LeaseRunOutcome,
    error: Option<Arc<CosmosError>>,
    candidate: Option<ContinuationToken>,
}

#[allow(clippy::too_many_arguments)] // A worker owns independent source, authority, and stop resources.
pub(super) async fn run_worker(
    engine: ProcessorEngine,
    session: LeaseSession,
    options: ManagedProcessorOptions,
    handler: RawChangeHandler,
    callbacks: Arc<Semaphore>,
    shared: Shared,
    global_stop: LeaseControl,
    stop: LeaseControl,
    events: mpsc::Sender<WorkerEvent>,
) -> ManagedLeaseShutdown {
    let processing = run_worker_inner(
        engine,
        session,
        options,
        handler,
        callbacks,
        shared,
        stop.clone(),
        events,
    );
    tokio::pin!(processing);
    tokio::select! {
        biased;
        _ = global_stop.interrupted() => { stop.stop(); processing.await }
        result = &mut processing => result,
    }
}

#[allow(clippy::too_many_arguments)] // A per-worker stop must survive replacement ownership sessions.
async fn run_worker_inner(
    engine: ProcessorEngine,
    mut session: LeaseSession,
    options: ManagedProcessorOptions,
    handler: RawChangeHandler,
    callbacks: Arc<Semaphore>,
    shared: Shared,
    stop: LeaseControl,
    events: mpsc::Sender<WorkerEvent>,
) -> ManagedLeaseShutdown {
    let id = session.lease().await.id().to_owned();
    let mut final_error = None;
    let mut final_guard = None;
    let outcome = loop {
        let guard = match session.begin_run() {
            Ok(guard) => guard,
            Err(error) => {
                final_error = Some(Arc::new(error));
                break LeaseRunOutcome::Failed;
            }
        };
        let end = {
            let processing = process(
                &engine,
                &session,
                &options,
                handler.clone(),
                callbacks.clone(),
                shared.clone(),
                &stop,
            );
            tokio::pin!(processing);
            let maintenance = async {
                tokio::select! {
                    biased;
                    _ = session.monitor_safety() => None,
                    result = session.maintain() => result.err(),
                }
            };
            tokio::pin!(maintenance);
            tokio::select! {
                biased;
                result = &mut processing => result,
                error = &mut maintenance => {
                    session.control().lose_ownership();
                    let mut end = processing.await;
                    if let Some(error) = error { end.error = Some(Arc::new(error)); }
                    end
                }
            }
        };
        if let Some(error) = &end.error {
            final_error = Some(error.clone());
            shared
                .lock()
                .expect("managed state lock poisoned")
                .leases
                .get_mut(&id)
                .expect("registered lease")
                .last_error = Some(error.clone());
        }
        if let Some(candidate) = end.candidate {
            drop(guard);
            match recover_checkpoint(session.clone(), end.lease, &candidate).await {
                Ok(Some(next)) => {
                    shared
                        .lock()
                        .expect("managed state lock poisoned")
                        .leases
                        .get_mut(&id)
                        .expect("registered lease")
                        .last_checkpoint = candidate;
                    session = next;
                    final_error = None;
                    if stop_requested(&stop) {
                        break LeaseRunOutcome::StoppedDrained;
                    }
                    let registered = tokio::select! {
                        biased;
                        _ = stop.interrupted() => break LeaseRunOutcome::StoppedDrained,
                        result = events.send(WorkerEvent { session: session.clone() }) => result,
                    };
                    if registered.is_err() {
                        final_error = Some(Arc::new(invalid(
                            "managed coordinator disappeared during recovery",
                        )));
                        break LeaseRunOutcome::Failed;
                    }
                    continue;
                }
                Ok(None) => break LeaseRunOutcome::OwnershipLost,
                Err(error) => {
                    final_error = Some(Arc::new(error));
                    break LeaseRunOutcome::Failed;
                }
            }
        }
        final_guard = Some(guard);
        break end.outcome;
    };
    let release = if session.control().is_lost() {
        LeaseReleaseOutcome::AuthorityLost
    } else {
        match session.release().await {
            Ok(()) => LeaseReleaseOutcome::Released,
            Err(error) => LeaseReleaseOutcome::Failed(error),
        }
    };
    drop(final_guard);
    let mut state = shared.lock().expect("managed state lock poisoned");
    let snapshot = state.leases.get_mut(&id).expect("registered lease");
    snapshot.state = if outcome == LeaseRunOutcome::OwnershipLost {
        ManagedLeaseState::OwnershipLost
    } else if final_error.is_some() {
        ManagedLeaseState::Quarantined
    } else {
        ManagedLeaseState::Stopped
    };
    if let Some(error) = &final_error {
        snapshot.last_error = Some(error.clone());
    }

    async fn recover_checkpoint(
        mut session: LeaseSession,
        mut expected: OwnedLease,
        candidate: &ContinuationToken,
    ) -> Result<Option<LeaseSession>> {
        let mut last_error = None;
        for _ in 0..3 {
            let Some(next) = session.reconcile_checkpoint(&expected, candidate).await? else {
                return Ok(None);
            };
            let confirmed = next.lease().await;
            if confirmed.checkpoint() == candidate {
                return Ok(Some(next));
            }
            match next.checkpoint(&confirmed, candidate).await {
                Ok(_) => return Ok(Some(next)),
                Err(
                    CheckpointError::Ambiguous(error)
                    | CheckpointError::Rejected(error)
                    | CheckpointError::Retryable(error),
                ) => {
                    last_error = Some(error);
                    expected = confirmed;
                    session = next;
                }
            }
        }
        Err(last_error.expect("failed checkpoint attempts retain their error"))
    }
    ManagedLeaseShutdown {
        id,
        outcome,
        release,
        error: final_error,
        snapshot: Some(snapshot.clone()),
    }
}

fn stop_requested(stop: &LeaseControl) -> bool {
    // An immediately ready interruption distinguishes a pending stop from an active host.
    use futures::FutureExt;
    stop.interrupted().now_or_never().is_some()
}

async fn process(
    engine: &ProcessorEngine,
    session: &LeaseSession,
    options: &ManagedProcessorOptions,
    handler: RawChangeHandler,
    callbacks: Arc<Semaphore>,
    shared: Shared,
    stop: &LeaseControl,
) -> End {
    let mut lease = session.lease().await;
    let id = lease.id().to_owned();
    let run_options = LeaseRunOptions::new(NonZeroU32::new(1).expect("one is nonzero"))
        .with_max_duration(Duration::from_secs(31_536_000))
        .with_drain_timeout(options.drain_timeout);
    let mut source = None;
    loop {
        if stop_requested(stop) {
            session.control().stop();
        }
        if source.is_none() {
            let read = ChangeFeedReadOptions::new(lease.range().clone(), options.start.clone())
                .with_mode(options.mode)
                .with_max_item_count(MaxItemCountHint::Limit(options.max_item_count))
                .with_continuation(lease.checkpoint().clone());
            let opened = tokio::select! {
                biased;
                _ = stop.interrupted() => {
                    return End { lease, outcome: LeaseRunOutcome::StoppedDrained, error: None, candidate: None };
                }
                outcome = session.control().interrupted() => {
                    return End { lease, outcome, error: None, candidate: None };
                }
                result = engine.open_reader(read) => result,
            };
            match opened {
                Ok(reader) => {
                    source = Some(ObservedSource {
                        reader,
                        shared: shared.clone(),
                        id: id.clone(),
                        idle: false,
                        poll_interval: options.poll_interval,
                    })
                }
                Err(error) => {
                    record_failure(&shared, &id, ManagedLeaseState::Recovering, error);
                    if let Some(outcome) = backoff(session, stop, options.recovery_backoff).await {
                        return End {
                            lease,
                            outcome,
                            error: None,
                            candidate: None,
                        };
                    }
                    continue;
                }
            }
        }
        let mut store = ObservedStore {
            session: session.clone(),
            shared: shared.clone(),
            id: id.clone(),
        };
        let delivery_cancelled = Arc::new(AtomicBool::new(false));
        let handle = {
            let shared = shared.clone();
            let id = id.clone();
            let handler = handler.clone();
            let callbacks = callbacks.clone();
            let stop = stop.clone();
            let delivery_cancelled = delivery_cancelled.clone();
            move |page: RawChangeFeedPage| {
                let shared = shared.clone();
                let id = id.clone();
                let handler = handler.clone();
                let callbacks = callbacks.clone();
                let stop = stop.clone();
                let delivery_cancelled = delivery_cancelled.clone();
                async move {
                    let candidate = page.continuation().clone();
                    let permit = tokio::select! {
                        biased;
                        _ = stop.interrupted() => {
                            delivery_cancelled.store(true, Ordering::Release);
                            return Err(invalid("callback delivery stopped before application work began"));
                        }
                        result = callbacks.acquire() => result.map_err(|_| invalid("callback gate closed"))?,
                    };
                    if stop_requested(&stop) {
                        delivery_cancelled.store(true, Ordering::Release);
                        return Err(invalid(
                            "callback delivery stopped before application work began",
                        ));
                    }
                    let _permit = permit;
                    handler(page).await?;
                    shared
                        .lock()
                        .expect("managed state lock poisoned")
                        .leases
                        .get_mut(&id)
                        .expect("registered lease")
                        .last_callback = Some(candidate);
                    Ok(())
                }
            }
        };
        let result = {
            let run = run_lease(
                source.as_mut().expect("opened reader"),
                &mut store,
                lease.clone(),
                session.control(),
                &run_options,
                handle,
            );
            tokio::pin!(run);
            tokio::select! {
                biased;
                _ = stop.interrupted() => { session.control().stop(); run.await }
                result = &mut run => result,
            }
        };
        match result {
            Ok(report) => {
                lease = report.lease().clone();
                if report.outcome() == LeaseRunOutcome::BatchLimitReached {
                    continue;
                }
                return End {
                    lease,
                    outcome: report.outcome(),
                    error: None,
                    candidate: if report.phase() == Some(LeaseRunPhase::Checkpointing) {
                        report.candidate().cloned()
                    } else {
                        None
                    },
                };
            }
            Err(error) => {
                lease = error.report().lease().clone();
                if delivery_cancelled.load(Ordering::Acquire) {
                    return End {
                        lease,
                        outcome: LeaseRunOutcome::StoppedUnprocessed,
                        error: None,
                        candidate: None,
                    };
                }
                if error.report().phase() == Some(LeaseRunPhase::Checkpointing) {
                    return End {
                        lease,
                        outcome: LeaseRunOutcome::Failed,
                        candidate: error.report().candidate().cloned(),
                        error: Some(Arc::new(error.error().clone())),
                    };
                }
                let health = if error.report().phase() == Some(LeaseRunPhase::Handling) {
                    ManagedLeaseState::Quarantined
                } else {
                    ManagedLeaseState::Recovering
                };
                record_failure(&shared, &id, health, error.error().clone());
                source = None;
                if let Some(outcome) = backoff(session, stop, options.recovery_backoff).await {
                    return End {
                        lease,
                        outcome,
                        error: None,
                        candidate: None,
                    };
                }
            }
        }
    }
}

fn record_failure(shared: &Shared, id: &str, health: ManagedLeaseState, error: CosmosError) {
    tracing::warn!(lease = id, error = %error, "managed lease failed; reopening from durable progress after backoff");
    let mut state = shared.lock().expect("managed state lock poisoned");
    let snapshot = state.leases.get_mut(id).expect("registered lease");
    snapshot.state = health;
    snapshot.last_error = Some(Arc::new(error));
}
async fn backoff(
    session: &LeaseSession,
    stop: &LeaseControl,
    delay: Duration,
) -> Option<LeaseRunOutcome> {
    tokio::select! {
        biased;
        _ = stop.interrupted() => Some(LeaseRunOutcome::StoppedDrained),
        outcome = session.control().interrupted() => Some(outcome),
        _ = sleep(delay) => None,
    }
}

#[cfg(test)]
mod tests;
