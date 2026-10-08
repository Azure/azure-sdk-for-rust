// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    collections::BTreeMap,
    num::NonZeroU64,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    models::{
        ContainerReference, ContinuationToken, CosmosOperation, ItemReference, PartitionKey,
        Precondition,
    },
    options::{ContentResponseOnWrite, OperationOptions, ReadConsistencyStrategy},
    CosmosDriver, CosmosError, CosmosErrorBuilder, CosmosResponse, Result,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{
    sync::{watch, Mutex},
    time::{timeout, timeout_at, Instant},
};

use crate::{CheckpointError, CheckpointStore, LeaseControl, OwnedLease};

mod bootstrap;
mod decisions;
pub use bootstrap::{
    BalanceCycle, BalanceRunOptions, BalanceRunReport, BootstrapPlan, BootstrapReady,
    BootstrapStartPolicy, BootstrapStore, InitialLease, LeaseBalancer,
};
use decisions::{
    authority_deadline, classify_checkpoint_failure, confirm_lease_write, plan_acquisition,
    plan_checkpoint, plan_release, plan_transfer, validate_record, AuthorityState, WriteTiming,
};

/// Timing policy shared by all workers for one lease.
///
/// Takeover waits for an unchanged ETag for the full duration. Local authority
/// expires earlier by the safety margin, measured from the start of a write.
/// Timers must have bounded drift smaller than that margin; suspension consumes
/// authority time. This is not a distributed clock or a server-side TTL.
#[derive(Clone, Debug)]
pub struct LeaseOwnershipOptions {
    duration: Duration,
    renew_interval: Duration,
    safety_margin: Duration,
    request_timeout: Duration,
}
impl LeaseOwnershipOptions {
    /// Validates all timing inputs before any lease I/O.
    ///
    /// # Errors
    ///
    /// Rejects zero/sub-millisecond or excessive durations, and policies without
    /// room for two request timeouts plus the renewal delay before the safety deadline.
    pub fn new(
        duration: Duration,
        renew_interval: Duration,
        safety_margin: Duration,
        request_timeout: Duration,
    ) -> Result<Self> {
        let options = Self {
            duration,
            renew_interval,
            safety_margin,
            request_timeout,
        };
        let valid = [duration, renew_interval, safety_margin, request_timeout]
            .iter()
            .all(|value| {
                value.as_millis() > 0
                    && value.as_millis() <= 3_600_000
                    && value.subsec_nanos() % 1_000_000 == 0
            });
        if !valid
            || safety_margin >= duration
            || request_timeout
                .checked_mul(2)
                .and_then(|writes| renew_interval.checked_add(writes))
                .is_none_or(|budget| budget >= duration - safety_margin)
        {
            return Err(invalid(
                "lease policy requires positive millisecond durations and renewal headroom",
            ));
        }
        Ok(options)
    }
    /// Returns the occupied-record observation interval before takeover.
    pub fn duration(&self) -> Duration {
        self.duration
    }
    /// Returns the delay between renewal attempts.
    pub fn renew_interval(&self) -> Duration {
        self.renew_interval
    }
    /// Returns the local authority safety margin.
    pub fn safety_margin(&self) -> Duration {
        self.safety_margin
    }
    /// Returns the outer deadline for each lease store request.
    pub fn request_timeout(&self) -> Duration {
        self.request_timeout
    }
    fn duration_millis(&self) -> u64 {
        self.duration.as_millis() as u64
    }
    fn safe_duration(&self) -> Duration {
        self.duration - self.safety_margin
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LeaseIdentity {
    owner: Option<String>,
    generation: u64,
}

#[derive(Clone, Serialize, Deserialize)]
struct LeaseRecord {
    id: String,
    version: u32,
    ownership: LeaseIdentity,
    range: azure_data_cosmos_driver::models::FeedRange,
    checkpoint: String,
    lease_duration_ms: u64,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

/// A point-read observation of one pre-created lease.
///
/// Its elapsed-time origin is local and must not be serialized or transferred
/// to another process. An observation never grants authority.
pub struct LeaseObservation {
    record: LeaseRecord,
    revision: String,
    observed_at: Instant,
    store_identity: Arc<()>,
}
impl LeaseObservation {
    /// Returns the current owner, or none for a released lease.
    pub fn owner(&self) -> Option<&str> {
        self.record.ownership.owner.as_deref()
    }
    /// Returns the persisted ownership generation.
    pub fn generation(&self) -> u64 {
        self.record.ownership.generation
    }
    /// Returns the last confirmed checkpoint.
    pub fn checkpoint(&self) -> &str {
        &self.record.checkpoint
    }
    /// Returns the record ETag.
    pub fn revision(&self) -> &str {
        &self.revision
    }
}

/// Cosmos point-operation store for one pre-created lease.
///
/// Use a single-write-region account for the lease container: multi-writer
/// conflict resolution is not an ownership fence. All workers must use the
/// same persisted lease duration and must not rewrite leases outside this
/// protocol. No provisioning, bootstrap, or topology subdivision is performed.
#[derive(Clone)]
pub struct CosmosLeaseStore {
    driver: Arc<CosmosDriver>,
    item: ItemReference,
    id: String,
    options: LeaseOwnershipOptions,
    identity: Arc<()>,
    container: ContainerReference,
    partition_key: PartitionKey,
    readiness: Option<bootstrap::ReadinessGate>,
}

impl CosmosLeaseStore {
    /// Resolves an existing lease container and addresses a pre-created item.
    ///
    /// The lease schema is documented in the crate README. `driver` can be
    /// independent of the source driver and point to a separate lease account.
    /// Calling this constructor asserts that the lease account has one write
    /// region. The current driver API cannot verify that capability here.
    ///
    /// # Errors
    ///
    /// Returns invalid identity or container-resolution errors.
    pub async fn new_single_writer(
        driver: Arc<CosmosDriver>,
        database: &str,
        container: &str,
        partition_key: PartitionKey,
        id: impl Into<String>,
        options: LeaseOwnershipOptions,
    ) -> Result<Self> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(invalid("lease id cannot be empty"));
        }
        let container: ContainerReference = driver
            .resolve_container(database, container, OperationOptions::default())
            .await?;
        Self::from_resolved_single_writer(driver, container, partition_key, id, options)
    }

    /// Addresses a lease locally using an already-resolved container and its driver.
    ///
    /// Performs no I/O or acquisition. Calling this constructor asserts that
    /// the account has one write region, as with [`new_single_writer()`](Self::new_single_writer).
    /// Subsequent operations reuse this reference; driver metadata refresh is allowed.
    ///
    /// # Errors
    ///
    /// Rejects empty lease IDs or a container with a different account or credential binding.
    pub fn from_resolved_single_writer(
        driver: Arc<CosmosDriver>,
        container: ContainerReference,
        partition_key: PartitionKey,
        id: impl Into<String>,
        options: LeaseOwnershipOptions,
    ) -> Result<Self> {
        crate::prepared_container::validate_binding(&driver, &container)?;
        let id = id.into();
        if id.trim().is_empty() {
            return Err(invalid("lease id cannot be empty"));
        }
        let item = ItemReference::from_name(&container, partition_key.clone(), id.clone());
        Ok(Self {
            driver,
            item,
            id,
            options,
            identity: Arc::new(()),
            container,
            partition_key,
            readiness: None,
        })
    }

    /// Reads and validates the lease without acquiring it.
    ///
    /// # Errors
    ///
    /// Returns missing-item, transport, malformed-record, or policy-mismatch errors.
    pub async fn observe(&self) -> Result<LeaseObservation> {
        let response = self
            .execute(CosmosOperation::read_item(self.item.clone()))
            .await?;
        let diagnostics = response.diagnostics();
        let revision = revision(&response)?;
        let record: LeaseRecord = response.into_body().into_single().map_err(|error| {
            CosmosErrorBuilder::from_error(error)
                .with_diagnostics(diagnostics.clone())
                .build()
        })?;
        self.validate(&record).map_err(|error| {
            CosmosErrorBuilder::from_error(error)
                .with_diagnostics(diagnostics)
                .build()
        })?;
        // Starting after the read is conservative even when the read is stale.
        Ok(LeaseObservation {
            record,
            revision,
            observed_at: Instant::now(),
            store_identity: self.identity.clone(),
        })
    }

    /// Attempts acquisition from an observation using its exact ETag.
    ///
    /// Returns none while an occupied observation has not aged for the lease
    /// interval. Once eligible, a conditional replace increments generation.
    /// A stale observation cannot overwrite a renewal or another acquisition.
    ///
    /// # Errors
    ///
    /// Returns CAS conflict, uncertain write, invalid observation, or store errors.
    /// An error grants no local authority, even if an uncertain write committed.
    pub async fn acquire(
        &self,
        observation: &LeaseObservation,
        owner: impl Into<String>,
    ) -> Result<Option<LeaseSession>> {
        self.validate_observation(observation)?;
        let Some(record) = plan_acquisition(
            &observation.record,
            &owner.into(),
            observation.observed_at.elapsed(),
            self.options.duration,
        )?
        else {
            return Ok(None);
        };
        self.install_owner(record, &observation.revision)
            .await
            .map(Some)
    }

    /// Conditionally transfers an active lease from its exact observed revision.
    ///
    /// The balancing policy must independently decide the donor is overloaded.
    /// This primitive validates observation ownership/generation through ETag,
    /// preserves the checkpoint, and increments generation. It cannot prevent
    /// application side effects already running under the old owner.
    ///
    /// # Errors
    ///
    /// Returns invalid observation/owner, readiness, conflict, or uncertain-write errors.
    pub async fn transfer(
        &self,
        observation: &LeaseObservation,
        owner: impl Into<String>,
    ) -> Result<LeaseSession> {
        self.validate_observation(observation)?;
        let record = plan_transfer(&observation.record, &owner.into())?;
        self.install_owner(record, &observation.revision).await
    }

    fn validate_observation(&self, observation: &LeaseObservation) -> Result<()> {
        self.validate(&observation.record)?;
        if observation
            .record
            .extra
            .contains_key("initializationGeneration")
            && self.readiness.is_none()
        {
            return Err(invalid(
                "bootstrapped work leases require a workload readiness-gated store",
            ));
        }
        if self.readiness.is_some() && !bootstrap::processing_eligible(&observation.record)? {
            return Err(invalid(
                "work lease is not committed and eligible for processing",
            ));
        }
        if !Arc::ptr_eq(&self.identity, &observation.store_identity) {
            return Err(invalid("lease observation belongs to a different store"));
        }
        Ok(())
    }

    async fn install_owner(
        &self,
        record: LeaseRecord,
        expected_revision: &str,
    ) -> Result<LeaseSession> {
        let began = Instant::now();
        let revision = if let Some(gate) = &self.readiness {
            gate.acquire(self, &record, expected_revision).await?
        } else {
            self.replace(&record, expected_revision).await?
        };
        let safe_until = confirm_lease_write(
            expected_revision,
            &revision,
            WriteTiming {
                started: began,
                completed: Instant::now(),
                previous_deadline: None,
            },
            self.options.safe_duration(),
            false,
        )?;
        owned(&record, &revision)?;
        Ok(LeaseSession {
            store: Arc::new(self.clone()),
            state: Arc::new(Mutex::new(SessionState {
                record,
                revision,
                authority: AuthorityState::Active { safe_until },
            })),
            control: LeaseControl::default(),
            deadline: watch::channel(safe_until).0,
            running: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Reads the record and attempts immediate acquisition if it is released.
    ///
    /// Occupied records return none; use [`observe()`](Self::observe) and
    /// [`acquire()`](Self::acquire) with an aged observation for takeover.
    ///
    /// # Errors
    ///
    /// Returns observation or conditional acquisition errors.
    pub async fn try_acquire(&self, owner: impl Into<String>) -> Result<Option<LeaseSession>> {
        self.acquire(&self.observe().await?, owner).await
    }

    fn validate(&self, record: &LeaseRecord) -> Result<()> {
        validate_record(record, &self.id, self.options.duration_millis())
    }

    async fn execute(&self, operation: CosmosOperation) -> Result<CosmosResponse> {
        let mut options = OperationOptions::default();
        options.read_consistency_strategy = Some(ReadConsistencyStrategy::LatestCommitted);
        options.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
        match timeout(
            self.options.request_timeout,
            self.driver.execute_singleton_operation(operation, options),
        )
        .await
        {
            Ok(result) => result,
            Err(_) => Err(timed_out(
                "lease request timed out; a write outcome may be unknown",
            )),
        }
    }

    async fn replace(&self, record: &LeaseRecord, revision: &str) -> Result<String> {
        let body = serde_json::to_vec(record)?;
        let response = self
            .execute(
                CosmosOperation::replace_item(self.item.clone())
                    .with_precondition(Precondition::if_match(revision.to_owned()))
                    .with_body(body),
            )
            .await?;
        revision_from_response(response)
    }
}

fn revision_from_response(response: CosmosResponse) -> Result<String> {
    revision(&response)
}
fn revision(response: &CosmosResponse) -> Result<String> {
    response.headers().etag.as_ref().map(ToString::to_string)
        .filter(|value| !value.is_empty()).ok_or_else(|| {
            CosmosError::builder()
                .with_status(azure_data_cosmos_driver::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("lease response has no ETag")
                .with_diagnostics(response.diagnostics()).build()
        })
}
fn owned(record: &LeaseRecord, revision: &str) -> Result<OwnedLease> {
    OwnedLease::new(
        &record.id,
        record
            .ownership
            .owner
            .clone()
            .ok_or_else(|| invalid("lease is released"))?,
        NonZeroU64::new(record.ownership.generation)
            .ok_or_else(|| invalid("owned generation cannot be zero"))?,
        revision.to_owned(),
        record.range.clone(),
        ContinuationToken::from_string(record.checkpoint.clone()),
    )
}
fn invalid(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
        .with_message(message)
        .build()
}

fn timed_out(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(
            azure_data_cosmos_driver::CosmosStatus::new(StatusCode::RequestTimeout)
                .with_sub_status(
                azure_data_cosmos_driver::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT
                    .value(),
            ),
        )
        .with_message(message)
        .build()
}

struct SessionState {
    record: LeaseRecord,
    revision: String,
    authority: AuthorityState,
}

struct LeaseWriteGuard {
    control: LeaseControl,
    armed: bool,
}
impl LeaseWriteGuard {
    fn new(control: &LeaseControl) -> Self {
        Self {
            control: control.clone(),
            armed: true,
        }
    }
    fn confirm(mut self) {
        self.armed = false;
    }
}
impl Drop for LeaseWriteGuard {
    fn drop(&mut self) {
        if self.armed {
            self.control.lose_ownership();
        }
    }
}

impl SessionState {
    fn snapshot(&self) -> OwnedLease {
        // Only validated owned records are admitted to this private state.
        owned(&self.record, &self.revision)
            .expect("canonical session record remains a validated owned lease")
    }
}

/// A locally fenced ownership session sharing one authoritative store revision.
///
/// Renewal, checkpoint, and release serialize through an async mutex held only
/// for lease I/O, never across application work. Clones share this authority;
/// independently acquired workers do not share memory.
#[derive(Clone)]
pub struct LeaseSession {
    store: Arc<CosmosLeaseStore>,
    state: Arc<Mutex<SessionState>>,
    control: LeaseControl,
    deadline: watch::Sender<Instant>,
    running: Arc<AtomicBool>,
}

pub(crate) struct SessionRunGuard {
    session: LeaseSession,
}
impl Drop for SessionRunGuard {
    fn drop(&mut self) {
        self.session.control.lose_ownership();
        self.session.running.store(false, Ordering::Release);
    }
}

impl LeaseSession {
    /// Reconciles an uncertain checkpoint and conditionally re-establishes the same generation.
    ///
    /// The old session remains revoked. A new session is returned only after a
    /// latest-committed read and exact-ETag write confirm the same owner, generation,
    /// range, and either the prior or candidate progress. If the candidate is not
    /// stored, retry its persistence with the returned session, not the callback.
    /// Returns none when the observed authority belongs to another owner/generation.
    ///
    /// # Errors
    ///
    /// Returns invalid recovery inputs, unexpected progress, store, or uncertain-write errors.
    pub async fn reconcile_checkpoint(
        &self,
        expected: &OwnedLease,
        candidate: &ContinuationToken,
    ) -> Result<Option<LeaseSession>> {
        {
            let state = self.state.lock().await;
            plan_checkpoint(&state.record, expected, candidate)?;
        }
        self.control.lose_ownership();
        let observed = self.store.observe().await?;
        self.store.validate_observation(&observed)?;
        if observed.record.ownership.owner.as_deref() != Some(expected.owner())
            || observed.record.ownership.generation != expected.epoch().get()
        {
            return Ok(None);
        }
        if observed.record.range != *expected.range()
            || (observed.checkpoint() != expected.checkpoint().as_str()
                && observed.checkpoint() != candidate.as_str())
        {
            return Err(invalid(
                "checkpoint recovery observed unexpected range or progress",
            ));
        }
        self.store
            .install_owner(observed.record, &observed.revision)
            .await
            .map(Some)
    }

    pub(crate) fn begin_run(&self) -> Result<SessionRunGuard> {
        self.running
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| invalid("an ownership session already has a processing run"))?;
        Ok(SessionRunGuard {
            session: self.clone(),
        })
    }
    /// Returns the stop/loss signals used by this session's processor.
    pub fn control(&self) -> &LeaseControl {
        &self.control
    }
    /// Returns the latest confirmed lease state.
    pub async fn lease(&self) -> OwnedLease {
        self.state.lock().await.snapshot()
    }

    /// Conditionally renews current authority without changing its checkpoint.
    ///
    /// Cancelling an in-flight write immediately signals ownership loss to active processing.
    ///
    /// # Errors
    ///
    /// Fails closed on expired authority, CAS rejection, or any uncertain write.
    pub async fn renew(&self) -> Result<OwnedLease> {
        let mut state = self.state.lock().await;
        let deadline = self.admit_write(&mut state)?;
        let began = Instant::now();
        state.authority = AuthorityState::WriteInFlight {
            safe_until: deadline,
        };
        let write = LeaseWriteGuard::new(&self.control);
        let result = self
            .write_before_deadline(&state.record, &state.revision, deadline)
            .await;
        let revision = match result {
            Ok(revision) => revision,
            Err(error) => {
                state.authority = AuthorityState::Revoked;
                self.control.lose_ownership();
                return Err(error);
            }
        };
        let safe_until = self.confirm_write(&mut state, &revision, began, deadline)?;
        state.revision = revision;
        state.authority = AuthorityState::Active { safe_until };
        self.deadline.send_replace(safe_until);
        write.confirm();
        Ok(state.snapshot())
    }

    /// Conditionally persists a completed batch using the latest renewal revision.
    ///
    /// `expected` must match the session's owner/generation/range and prior
    /// checkpoint. Its older revision is tolerated only because this session's
    /// own serialized renewals may have changed it. Independent stale sessions
    /// still write their stale ETag and cannot change a replacement owner's lease.
    /// Cancelling an in-flight write immediately signals ownership loss.
    ///
    /// # Errors
    ///
    /// Returns rejected local validation or ambiguous sent-write failures, including
    /// HTTP 412 when a previous driver attempt may have committed. Neither is automatically retried.
    pub async fn checkpoint(
        &self,
        expected: &OwnedLease,
        candidate: &ContinuationToken,
    ) -> std::result::Result<OwnedLease, CheckpointError> {
        let mut state = self.state.lock().await;
        let deadline = self
            .admit_write(&mut state)
            .map_err(CheckpointError::Rejected)?;
        let record = plan_checkpoint(&state.record, expected, candidate)
            .map_err(CheckpointError::Rejected)?;
        let began = Instant::now();
        state.authority = AuthorityState::WriteInFlight {
            safe_until: deadline,
        };
        let write = LeaseWriteGuard::new(&self.control);
        let revision = match self
            .write_before_deadline(&record, &state.revision, deadline)
            .await
        {
            Ok(revision) => revision,
            Err(error) => {
                state.authority = AuthorityState::Revoked;
                self.control.lose_ownership();
                return Err(classify_checkpoint_failure(error));
            }
        };
        let safe_until = self
            .confirm_write(&mut state, &revision, began, deadline)
            .map_err(CheckpointError::Ambiguous)?;
        state.record = record;
        state.revision = revision;
        state.authority = AuthorityState::Active { safe_until };
        self.deadline.send_replace(safe_until);
        write.confirm();
        Ok(state.snapshot())
    }

    /// Releases authority conditionally, preserving range and durable checkpoint.
    ///
    /// Cancellation during persistence immediately signals ownership loss.
    ///
    /// # Errors
    ///
    /// Rejects stale or expired authority. Unknown write outcomes require a reread.
    pub async fn release(&self) -> Result<()> {
        let mut state = self.state.lock().await;
        let deadline = self.admit_write(&mut state)?;
        let record = plan_release(&state.record);
        state.authority = AuthorityState::WriteInFlight {
            safe_until: deadline,
        };
        let write = LeaseWriteGuard::new(&self.control);
        let result = self
            .write_before_deadline(&record, &state.revision, deadline)
            .await;
        state.authority = AuthorityState::Revoked;
        self.control.lose_ownership();
        if result.is_ok() {
            write.confirm();
        }
        result.map(|_| ())
    }

    fn admit_write(&self, state: &mut SessionState) -> Result<Instant> {
        authority_deadline(state.authority, Instant::now(), self.control.is_lost()).inspect_err(
            |_| {
                state.authority = AuthorityState::Revoked;
                self.control.lose_ownership();
            },
        )
    }

    fn confirm_write(
        &self,
        state: &mut SessionState,
        revision: &str,
        began: Instant,
        deadline: Instant,
    ) -> Result<Instant> {
        confirm_lease_write(
            &state.revision,
            revision,
            WriteTiming {
                started: began,
                completed: Instant::now(),
                previous_deadline: Some(deadline),
            },
            self.store.options.safe_duration(),
            self.control.is_lost(),
        )
        .inspect_err(|_| {
            state.authority = AuthorityState::Revoked;
            self.control.lose_ownership();
        })
    }

    async fn write_before_deadline(
        &self,
        record: &LeaseRecord,
        revision: &str,
        deadline: Instant,
    ) -> Result<String> {
        timeout_at(deadline, self.store.replace(record, revision))
            .await
            .map_err(|_| {
                timed_out("lease safety deadline expired during a write; outcome is unknown")
            })?
    }

    pub(crate) async fn maintain(&self) -> Result<()> {
        loop {
            tokio::time::sleep(self.store.options.renew_interval).await;
            self.renew().await?;
        }
    }

    pub(crate) async fn monitor_safety(&self) {
        let mut deadlines = self.deadline.subscribe();
        loop {
            let deadline = *deadlines.borrow_and_update();
            if self.control.is_lost() || Instant::now() >= deadline {
                self.control.lose_ownership();
                return;
            }
            tokio::select! {
                biased;
                changed = deadlines.changed() => {
                    if changed.is_err() { self.control.lose_ownership(); return; }
                },
                _ = tokio::time::sleep_until(deadline) => { self.control.lose_ownership(); return; },
            }
        }
    }
}

impl CheckpointStore for LeaseSession {
    fn persist<'a>(
        &'a mut self,
        lease: &'a OwnedLease,
        candidate: &'a ContinuationToken,
    ) -> futures::future::BoxFuture<'a, std::result::Result<OwnedLease, CheckpointError>> {
        Box::pin(self.checkpoint(lease, candidate))
    }
}

/// The explicit result of attempting to give up the acquired lease.
#[derive(Debug)]
pub enum LeaseReleaseOutcome {
    /// No release was attempted, for example when another run already owns the session.
    NotAttempted,
    /// A conditional release was confirmed by Cosmos.
    Released,
    /// Local authority was already lost; no release write was admitted.
    AuthorityLost,
    /// Release failed or was uncertain; reread before further acquisition.
    Failed(CosmosError),
}

/// Processing, maintenance, and release results for an acquired lease session.
#[derive(Debug)]
pub struct LeaseOwnershipRun {
    processing: std::result::Result<crate::LeaseRunReport, crate::LeaseRunError>,
    maintenance_error: Option<CosmosError>,
    release: LeaseReleaseOutcome,
}
impl LeaseOwnershipRun {
    /// Returns bounded processing results, including candidate/durable progress.
    pub fn processing(&self) -> &std::result::Result<crate::LeaseRunReport, crate::LeaseRunError> {
        &self.processing
    }
    /// Returns a renewal failure, if maintenance caused local revocation.
    pub fn maintenance_error(&self) -> Option<&CosmosError> {
        self.maintenance_error.as_ref()
    }
    /// Returns confirmed release versus lost/uncertain authority.
    pub fn release(&self) -> &LeaseReleaseOutcome {
        &self.release
    }
    pub(crate) fn new(
        processing: std::result::Result<crate::LeaseRunReport, crate::LeaseRunError>,
        maintenance_error: Option<CosmosError>,
        release: LeaseReleaseOutcome,
    ) -> Self {
        Self {
            processing,
            maintenance_error,
            release,
        }
    }
}

#[cfg(test)]
mod tests;
