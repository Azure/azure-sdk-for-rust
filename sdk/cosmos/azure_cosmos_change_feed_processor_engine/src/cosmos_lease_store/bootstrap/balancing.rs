// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{models::ItemReference, CosmosErrorBuilder, Result};
use std::{
    collections::{HashMap, HashSet},
    future::Future,
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};
use tokio::time::{sleep, timeout, Instant};

use super::super::{invalid, CosmosLeaseStore, LeaseObservation, LeaseSession};
use super::{BootstrapPhase, BootstrapStore, ReadinessGate};
use crate::{
    plan_equal_lease_balance, BalanceAction, BalanceActionKind, BalanceLease, LeaseControl,
    LeaseRunOutcome,
};

/// One balancing cycle's result; only Acquired grants a session for processing.
pub enum BalanceCycle {
    /// No candidate satisfies the policy.
    NoAction,
    /// The exact observed revision lost a race. Re-read before another attempt.
    Conflict(BalanceAction),
    /// A conditional store operation confirmed new authority.
    Acquired {
        /// The policy decision that led to the confirmed acquisition.
        action: BalanceAction,
        /// Confirmed authority; only this may be handed to processing.
        session: LeaseSession,
    },
}

struct SeenRevision {
    revision: String,
    first_seen: Instant,
}

/// Local observations for decentralized equal-count balancing.
///
/// This owns no renewal task. Processing sessions maintain authority independently.
/// Other zero-lease workers are not registered globally. Expiry conservatively
/// requires observing an unchanged ETag for the stored lease interval.
pub struct LeaseBalancer {
    workload: BootstrapStore,
    worker: String,
    seen: HashMap<String, SeenRevision>,
    sessions: HashMap<String, LeaseSession>,
    tie_break: u64,
    page_size: NonZeroU32,
    maximum_owned: NonZeroU32,
    excluded: HashSet<String>,
}
impl LeaseBalancer {
    /// Attaches to an already-initialized workload; never initializes it.
    ///
    /// # Errors
    ///
    /// Rejects an empty worker incarnation.
    pub fn new(workload: BootstrapStore, worker: impl Into<String>) -> Result<Self> {
        let worker = worker.into();
        if worker.trim().is_empty() {
            return Err(invalid("balancing requires a worker incarnation"));
        }
        Ok(Self {
            workload,
            worker,
            seen: HashMap::new(),
            sessions: HashMap::new(),
            tie_break: rand::random(),
            page_size: NonZeroU32::new(8).expect("eight is non-zero"),
            maximum_owned: NonZeroU32::new(32).expect("32 is non-zero"),
            excluded: HashSet::new(),
        })
    }
    /// Supplies deterministic candidate rotation.
    pub fn with_tie_break(mut self, tie_break: u64) -> Self {
        self.tie_break = tie_break;
        self
    }
    /// Sets the inventory page-size hint; every page must still be read.
    pub fn with_page_size(mut self, page_size: NonZeroU32) -> Self {
        self.page_size = page_size;
        self
    }
    /// Sets this host's ownership ceiling independently of callback concurrency.
    ///
    /// Values above the protocol's 32-record bound are rejected by [`cycle()`](Self::cycle).
    pub fn with_max_owned_leases(mut self, limit: NonZeroU32) -> Self {
        self.maximum_owned = limit;
        self
    }
    /// Returns the configured ownership ceiling.
    pub fn max_owned_leases(&self) -> NonZeroU32 {
        self.maximum_owned
    }
    /// Defers a lease-local failed candidate without removing it from global policy counts.
    ///
    /// # Errors
    ///
    /// Rejects an empty lease ID.
    pub fn exclude_lease(&mut self, id: impl Into<String>) -> Result<()> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(invalid("excluded lease ID cannot be empty"));
        }
        self.excluded.insert(id);
        Ok(())
    }
    /// Allows a previously deferred candidate to be acquired again.
    pub fn include_lease(&mut self, id: &str) {
        self.excluded.remove(id);
    }
    /// Tracks freshly confirmed same-worker recovery authority.
    ///
    /// # Errors
    ///
    /// Rejects lost authority or a different worker identity.
    pub async fn register_session(&mut self, session: LeaseSession) -> Result<()> {
        let state = session.state.lock().await;
        let lease = state.snapshot();
        if super::super::authority_deadline(
            state.authority,
            Instant::now(),
            session.control().is_lost(),
        )
        .is_err()
            || lease.owner() != self.worker
        {
            return Err(invalid(
                "registered session is not this worker's confirmed authority",
            ));
        }
        drop(state);
        if let Some(existing) = self.sessions.get(lease.id()) {
            if !existing.control().is_lost() && !Arc::ptr_eq(&existing.state, &session.state) {
                return Err(invalid(
                    "a different active session is already registered for this lease",
                ));
            }
        }
        self.sessions.insert(lease.id().to_owned(), session);
        Ok(())
    }
    /// Returns this worker's incarnation.
    pub fn worker(&self) -> &str {
        &self.worker
    }
    /// Returns the page-size hint.
    pub fn page_size(&self) -> NonZeroU32 {
        self.page_size
    }
    /// Returns the next candidate rotation input.
    pub fn tie_break(&self) -> u64 {
        self.tie_break
    }

    /// Reads complete inventory and attempts at most one exact-revision acquisition.
    ///
    /// Bootstrap/control, uncommitted-generation, pending/transitioning, and retired
    /// records are excluded. Failed pages are errors, never empty inventories.
    /// Local sessions no longer observed under this worker/generation are revoked.
    ///
    /// # Errors
    ///
    /// Returns readiness, invalid/partial inventory, or uncertain-write errors.
    /// The cycle has a 30-second outer budget and grants no session on failure.
    pub async fn cycle(&mut self) -> Result<BalanceCycle> {
        if self.maximum_owned.get() > 32 {
            return Err(invalid(
                "host lease capacity cannot exceed the 32-record workload bound",
            ));
        }
        timeout(Duration::from_secs(30), self.cycle_inner())
            .await
            .map_err(|_| {
                super::super::timed_out("balancing cycle timed out; acquisition may be uncertain")
            })?
    }

    async fn cycle_inner(&mut self) -> Result<BalanceCycle> {
        let root = self.workload.store.observe().await?;
        let metadata = self.workload.metadata(&root.record)?;
        if metadata.phase != BootstrapPhase::Ready {
            return Err(invalid("balancing requires an initialized Ready workload"));
        }
        let records = self.workload.discover_paged(Some(self.page_size)).await?;
        let now = Instant::now();
        let mut entries = Vec::new();
        let mut next_seen = HashMap::new();
        for record in records {
            let eligible = super::processing_eligible(&record)?;
            let revision = record
                .extra
                .get("_etag")
                .and_then(serde_json::Value::as_str)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| invalid("balancing inventory lease has no ETag"))?
                .to_owned();
            let first_seen = self
                .seen
                .get(&record.id)
                .filter(|previous| previous.revision == revision)
                .map_or(now, |previous| previous.first_seen);
            next_seen.insert(
                record.id.clone(),
                SeenRevision {
                    revision: revision.clone(),
                    first_seen,
                },
            );
            let expired = now.duration_since(first_seen) >= self.workload.store.options.duration();
            let mut snapshot = BalanceLease::new(record.id.clone())?
                .with_expired(expired)
                .with_eligible(eligible);
            if let Some(owner) = &record.ownership.owner {
                snapshot = snapshot.with_owner(owner.clone());
            }
            let mut store: CosmosLeaseStore = self.workload.store.clone();
            store.id = record.id.clone();
            store.item = ItemReference::from_name(
                &store.container,
                store.partition_key.clone(),
                store.id.clone(),
            );
            store.identity = Arc::new(());
            store.readiness = Some(ReadinessGate {
                bootstrap: Arc::new(self.workload.store.clone()),
                plan: metadata.plan.clone(),
            });
            let observation = LeaseObservation {
                record,
                revision,
                observed_at: first_seen,
                store_identity: store.identity.clone(),
            };
            entries.push((snapshot, store, observation));
        }
        let snapshots: Vec<_> = entries
            .iter()
            .map(|(snapshot, _, _)| snapshot.clone())
            .collect();
        let action = plan_equal_lease_balance(&snapshots, &self.worker, self.tie_break)?;
        self.tie_break = self.tie_break.wrapping_add(1);
        self.seen = next_seen;
        // Inventory expiry can refer to a pre-renewal revision; the local watchdog owns expiry.
        for (id, session) in &self.sessions {
            let lease = session.lease().await;
            if !entries.iter().any(|(snapshot, _, observed)| {
                snapshot.id() == id
                    && snapshot.eligible()
                    && snapshot.owner() == Some(self.worker.as_str())
                    && observed.record.ownership.generation == lease.epoch().get()
            }) {
                session.control().lose_ownership();
            }
        }
        self.sessions
            .retain(|_, session| !session.control().is_lost());
        let Some(action) = action else {
            return Ok(BalanceCycle::NoAction);
        };
        if self.sessions.len() >= self.maximum_owned.get() as usize
            || self.excluded.contains(action.lease_id())
        {
            return Ok(BalanceCycle::NoAction);
        }
        let (_, store, observation) = entries
            .iter()
            .find(|(snapshot, _, _)| snapshot.id() == action.lease_id())
            .expect("policy selects an inventory identity");
        let result =
            match action.kind() {
                BalanceActionKind::Pickup => store
                    .acquire(observation, &self.worker)
                    .await
                    .and_then(|session| {
                        session.ok_or_else(|| invalid("expired candidate became ineligible"))
                    }),
                BalanceActionKind::Transfer => store.transfer(observation, &self.worker).await,
            };
        match result {
            Ok(session) => {
                self.sessions
                    .insert(action.lease_id().to_owned(), session.clone());
                Ok(BalanceCycle::Acquired { action, session })
            }
            Err(error) if error.status().status_code() == StatusCode::PreconditionFailed => {
                Ok(BalanceCycle::Conflict(action))
            }
            Err(error) => Err(error),
        }
    }

    /// Runs bounded jittered cycles, handing off only confirmed sessions.
    ///
    /// `on_acquired` registers/starts work, not the long-running application
    /// handler. That work uses the engine's independent renewal. A failed
    /// handoff conditionally releases its session and preserves the failure.
    /// Stop/deadline requests drain on local sessions; callers own task handles
    /// and observe their final processing/release results.
    ///
    /// # Errors
    ///
    /// Returns inventory, acquisition, schedule, deadline, or handoff failures.
    pub async fn run_cycles<H, F>(
        &mut self,
        options: &BalanceRunOptions,
        control: &LeaseControl,
        mut on_acquired: H,
    ) -> Result<BalanceRunReport>
    where
        H: FnMut(LeaseSession) -> F,
        F: Future<Output = Result<()>>,
    {
        if options.interval.is_zero()
            || options.timeout.is_zero()
            || options.timeout > Duration::from_secs(300)
            || options.jitter > Duration::from_secs(60)
            || options.interval > Duration::from_secs(60)
        {
            return Err(invalid(
                "balancing scheduling requires bounded positive intervals and timeout",
            ));
        }
        let mut report = BalanceRunReport {
            cycles: 0,
            acquisitions: 0,
            conflicts: 0,
            stopped: false,
        };
        let result = {
            let run = async {
                for index in 0..options.cycles.get() {
                    match self.cycle().await? {
                        BalanceCycle::NoAction => {}
                        BalanceCycle::Conflict(_) => report.conflicts += 1,
                        BalanceCycle::Acquired { session, .. } => {
                            if let Err(error) = on_acquired(session.clone()).await {
                                let cleanup = session.release().await.err();
                                return Err(if let Some(cleanup) = cleanup {
                                    CosmosErrorBuilder::from_error(error)
                                        .with_context(format!(
                                            "balancing handoff cleanup also failed: {cleanup}"
                                        ))
                                        .build()
                                } else {
                                    error
                                });
                            }
                            report.acquisitions += 1;
                        }
                    }
                    report.cycles += 1;
                    if index + 1 < options.cycles.get() {
                        let jitter_ms = rand::random_range(0..=options.jitter.as_millis() as u64);
                        sleep(options.interval + Duration::from_millis(jitter_ms)).await;
                    }
                }
                Ok(())
            };
            tokio::select! {
                biased;
                outcome = control.interrupted() => Ok(Some(outcome)),
                result = timeout(options.timeout, run) => match result {
                    Ok(result) => result.map(|_| None),
                    Err(_) => Err(super::super::timed_out("balancing run reached its deadline")),
                },
            }
        };
        match result {
            Ok(Some(outcome)) => {
                report.stopped = true;
                for session in self.sessions.values() {
                    if outcome == LeaseRunOutcome::OwnershipLost {
                        session.control().lose_ownership();
                    } else {
                        session.control().stop();
                    }
                }
                Ok(report)
            }
            Ok(None) => Ok(report),
            Err(error) => {
                for session in self.sessions.values() {
                    session.control().stop();
                }
                Err(error)
            }
        }
    }
}

/// Bounds balancing cycles and their jittered schedule.
#[derive(Clone, Debug)]
pub struct BalanceRunOptions {
    cycles: NonZeroU32,
    interval: Duration,
    jitter: Duration,
    timeout: Duration,
}
impl BalanceRunOptions {
    /// Sets the required maximum cycles; defaults to one-second intervals.
    pub fn new(cycles: NonZeroU32) -> Self {
        Self {
            cycles,
            interval: Duration::from_secs(1),
            jitter: Duration::from_millis(250),
            timeout: Duration::from_secs(60),
        }
    }
    /// Sets a positive interval, validated before I/O.
    pub fn with_interval(mut self, value: Duration) -> Self {
        self.interval = value;
        self
    }
    /// Sets maximum jitter, validated before I/O.
    pub fn with_jitter(mut self, value: Duration) -> Self {
        self.jitter = value;
        self
    }
    /// Sets the total run budget, validated before I/O.
    pub fn with_timeout(mut self, value: Duration) -> Self {
        self.timeout = value;
        self
    }
    /// Returns the cycle limit.
    pub fn cycles(&self) -> NonZeroU32 {
        self.cycles
    }
    /// Returns the base interval.
    pub fn interval(&self) -> Duration {
        self.interval
    }
    /// Returns maximum jitter.
    pub fn jitter(&self) -> Duration {
        self.jitter
    }
    /// Returns the total run deadline.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}
/// Movement counts; stopping schedules drain, not completed release.
#[derive(Debug)]
pub struct BalanceRunReport {
    cycles: u32,
    acquisitions: u32,
    conflicts: u32,
    stopped: bool,
}
impl BalanceRunReport {
    /// Returns completed cycles.
    pub fn cycles(&self) -> u32 {
        self.cycles
    }
    /// Returns confirmed acquisitions handed to the callback.
    pub fn acquisitions(&self) -> u32 {
        self.acquisitions
    }
    /// Returns conditional acquisition conflicts.
    pub fn conflicts(&self) -> u32 {
        self.conflicts
    }
    /// Returns whether external stop/loss requested termination.
    pub fn stopped(&self) -> bool {
        self.stopped
    }
}

#[cfg(test)]
mod tests;
