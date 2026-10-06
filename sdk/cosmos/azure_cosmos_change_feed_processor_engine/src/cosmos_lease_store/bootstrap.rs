// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{collections::BTreeMap, sync::Arc, time::Duration};

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    models::{
        ContainerReference, ContinuationToken, CosmosOperation, FeedRange, ItemReference,
        PartitionKey,
    },
    options::OperationOptions,
    CosmosDriver, CosmosError, CosmosErrorBuilder, ResponseBody, Result,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::time::{sleep, timeout, Instant};

use super::{
    decisions::AuthorityState, invalid, CosmosLeaseStore, LeaseIdentity, LeaseOwnershipOptions,
    LeaseRecord, LeaseSession, LeaseWriteGuard,
};
use crate::ChangeFeedMode;

const MAX_INITIAL_LEASES: usize = 32;
const INITIALIZATION_GENERATION: u64 = 1;

mod balancing;
mod topology;
pub use balancing::{BalanceCycle, BalanceRunOptions, BalanceRunReport, LeaseBalancer};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
enum StoredMode {
    LatestVersion,
    AllVersionsAndDeletes,
}

/// The immutable initialization policy, including an exact timestamp when applicable.
pub use azure_data_cosmos_driver::models::ChangeFeedStartFrom as BootstrapStartPolicy;

/// One logical assignment and its already-established initial checkpoint.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InitialLease {
    range: FeedRange,
    checkpoint: String,
}
impl InitialLease {
    /// Supplies a recoverable token for this exact source, mode, and range.
    ///
    /// For `Now`, capture a concrete position by polling the original reader;
    /// a pre-poll snapshot that reevaluates now on resume is not sufficient.
    ///
    /// # Errors
    ///
    /// Rejects empty tokens or non-interval/logical-partition scopes.
    pub fn new(range: FeedRange, checkpoint: ContinuationToken) -> Result<Self> {
        if !interval(&range) || checkpoint.as_str().is_empty() {
            return Err(invalid(
                "bootstrap seeds require explicit non-empty ranges and recoverable positions",
            ));
        }
        Ok(Self {
            range,
            checkpoint: checkpoint.as_str().to_owned(),
        })
    }
    /// Returns the logical assignment.
    pub fn range(&self) -> &FeedRange {
        &self.range
    }
    /// Returns the original starting checkpoint, not current processing progress.
    pub fn checkpoint(&self) -> &str {
        &self.checkpoint
    }
}

/// A bounded workload plan whose original checkpoints are persisted before creation.
///
/// All workers agree on source identity, group, mode, start policy, and expected
/// coverage. The first atomic bootstrap create establishes the seed positions;
/// subsequent workers adopt those positions, not their newly calculated tokens.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootstrapPlan {
    source: String,
    group: String,
    mode: StoredMode,
    start: BootstrapStartPolicy,
    expected: FeedRange,
    seeds: Vec<InitialLease>,
}
impl BootstrapPlan {
    /// Validates explicit complete initial coverage without I/O.
    ///
    /// `source` must identify the account and stable container RID, not only its
    /// name. This slice accepts at most 32 assignments and performs no discovery
    /// or checkpoint transformation inside bootstrap.
    ///
    /// # Errors
    ///
    /// Rejects invalid identities, overlapping/gapped coverage, and excessive plans.
    pub fn new(
        source: impl Into<String>,
        group: impl Into<String>,
        mode: ChangeFeedMode,
        start: BootstrapStartPolicy,
        expected: FeedRange,
        mut seeds: Vec<InitialLease>,
    ) -> Result<Self> {
        seeds.sort_by(|a, b| a.range.min_inclusive().cmp(b.range.min_inclusive()));
        let plan = Self {
            source: source.into(),
            group: group.into(),
            mode: match mode {
                ChangeFeedMode::LatestVersion => StoredMode::LatestVersion,
                ChangeFeedMode::AllVersionsAndDeletes => StoredMode::AllVersionsAndDeletes,
            },
            start,
            expected,
            seeds,
        };
        validate_plan(&plan)?;
        Ok(plan)
    }
    /// Returns the stable source identity.
    pub fn source(&self) -> &str {
        &self.source
    }
    /// Returns the processor group.
    pub fn group(&self) -> &str {
        &self.group
    }
    /// Returns the feed mode.
    pub fn mode(&self) -> ChangeFeedMode {
        match self.mode {
            StoredMode::LatestVersion => ChangeFeedMode::LatestVersion,
            StoredMode::AllVersionsAndDeletes => ChangeFeedMode::AllVersionsAndDeletes,
        }
    }
    /// Returns the initialization policy.
    pub fn start_policy(&self) -> &BootstrapStartPolicy {
        &self.start
    }
    /// Returns the full expected logical coverage.
    pub fn expected_range(&self) -> &FeedRange {
        &self.expected
    }
    /// Returns persisted original assignments/positions.
    pub fn initial_leases(&self) -> &[InitialLease] {
        &self.seeds
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
enum BootstrapPhase {
    Initializing,
    Ready,
}

#[derive(Clone, Serialize, Deserialize)]
struct Metadata {
    protocol: u32,
    phase: BootstrapPhase,
    plan: BootstrapPlan,
    committed_generation: Option<u64>,
}

/// A verified readiness snapshot, not permanent proof of coverage.
pub struct BootstrapReady {
    id: String,
    plan: BootstrapPlan,
    lease_ids: Vec<String>,
}
impl BootstrapReady {
    /// Returns the shared workload bootstrap identity and partition-key value.
    pub fn workload_id(&self) -> &str {
        &self.id
    }
    /// Returns the winning persisted plan.
    pub fn plan(&self) -> &BootstrapPlan {
        &self.plan
    }
    /// Returns logical lease IDs with verified coverage at completion.
    pub fn lease_ids(&self) -> &[String] {
        &self.lease_ids
    }
}

/// Workload-scoped bootstrap in one existing single-writer Cosmos partition.
///
/// The container must have the single partition-key path `/workload`. Bootstrap
/// and work items use the same derived ID/partition key across every worker.
/// Transactional batches fence work creation and readiness against initializer
/// authority. No containers or Azure resources are provisioned.
#[derive(Clone)]
pub struct BootstrapStore {
    store: CosmosLeaseStore,
    plan: BootstrapPlan,
}

impl BootstrapStore {
    /// Loads the winning persisted initialization plan without discovering fresh positions.
    ///
    /// Returns none only when the workload record does not exist. Existing
    /// partial initialization is resumed using its original plan.
    ///
    /// # Errors
    ///
    /// Returns configuration, partitioning, metadata, or lease-account errors.
    pub async fn load_existing_single_writer(
        driver: Arc<CosmosDriver>,
        container: ContainerReference,
        source: &str,
        group: &str,
        mode: ChangeFeedMode,
        start: &BootstrapStartPolicy,
        policy: LeaseOwnershipOptions,
    ) -> Result<Option<Self>> {
        if source.trim().is_empty() || group.trim().is_empty() {
            return Err(invalid("bootstrap requires a non-empty source and group"));
        }
        let id = workload_identity(source, group)?;
        let store = CosmosLeaseStore::from_resolved_single_writer(
            driver,
            container,
            PartitionKey::from(id.clone()),
            id,
            policy,
        )?;
        validate_partition(&store.container)?;
        let observed = match store.observe().await {
            Ok(observed) => observed,
            Err(error) if error.status().status_code() == StatusCode::NotFound => return Ok(None),
            Err(error) => return Err(error),
        };
        let metadata: Metadata = serde_json::from_value(
            observed
                .record
                .extra
                .get("bootstrap")
                .ok_or_else(|| invalid("workload record has no bootstrap metadata"))?
                .clone(),
        )?;
        validate_plan(&metadata.plan)?;
        if metadata.plan.source() != source
            || metadata.plan.group() != group
            || metadata.plan.mode() != mode
            || metadata.plan.start_policy() != start
        {
            return Err(invalid(
                "persisted workload configuration does not match this processor",
            ));
        }
        let workload = Self::with_store(store, metadata.plan.clone())?;
        workload.metadata(&observed.record)?;
        Ok(Some(workload))
    }

    /// Addresses a stable source/group workload in an existing lease container.
    ///
    /// # Errors
    ///
    /// Rejects incompatible partitioning or identities, or resolution failures.
    pub async fn new_single_writer(
        driver: Arc<CosmosDriver>,
        database: &str,
        container: &str,
        plan: BootstrapPlan,
        policy: LeaseOwnershipOptions,
    ) -> Result<Self> {
        validate_plan(&plan)?;
        let id = workload_id(&plan)?;
        let store = CosmosLeaseStore::new_single_writer(
            driver,
            database,
            container,
            PartitionKey::from(id.clone()),
            id,
            policy,
        )
        .await?;
        Self::with_store(store, plan)
    }

    /// Prepares bootstrap locally against an already-resolved single-writer lease container.
    ///
    /// Performs no I/O or initialization. The container must be partitioned by
    /// `/workload`; the caller must verify the account has one write region.
    ///
    /// # Errors
    ///
    /// Rejects invalid plans, partitioning, or mismatched account/credential bindings.
    pub fn from_resolved_single_writer(
        driver: Arc<CosmosDriver>,
        container: ContainerReference,
        plan: BootstrapPlan,
        policy: LeaseOwnershipOptions,
    ) -> Result<Self> {
        validate_plan(&plan)?;
        let id = workload_id(&plan)?;
        let store = CosmosLeaseStore::from_resolved_single_writer(
            driver,
            container,
            PartitionKey::from(id.clone()),
            id,
            policy,
        )?;
        Self::with_store(store, plan)
    }

    fn with_store(store: CosmosLeaseStore, plan: BootstrapPlan) -> Result<Self> {
        validate_partition(&store.container)?;
        Ok(Self { store, plan })
    }

    /// Ensures initialization and verifies complete coverage, including after Ready.
    ///
    /// Competing initializers wait with bounded jitter. The winner reuses original
    /// persisted checkpoints, creates missing uncovered assignments idempotently,
    /// verifies coverage, and conditionally publishes Ready. Any creation/read
    /// failure prevents readiness; cleanup never hides the original failure.
    ///
    /// # Errors
    ///
    /// Returns incompatible configuration, conditional-write, discovery, incomplete
    /// coverage, or startup-deadline errors. An interrupted write may be uncertain.
    pub async fn ensure_initialized(
        &self,
        incarnation: impl Into<String>,
        startup_timeout: Duration,
    ) -> Result<BootstrapReady> {
        let incarnation = incarnation.into();
        if incarnation.trim().is_empty()
            || startup_timeout.is_zero()
            || startup_timeout > Duration::from_secs(300)
        {
            return Err(invalid(
                "bootstrap requires an incarnation and startup budget of at most five minutes",
            ));
        }
        timeout(startup_timeout, self.ensure_loop(&incarnation))
            .await
            .map_err(|_| {
                super::timed_out("bootstrap startup deadline expired; readiness was not confirmed")
            })?
    }

    /// Builds a processing store whose acquisition checks workload readiness atomically.
    ///
    /// # Errors
    ///
    /// Rejects an unrelated snapshot or lease not in its verified coverage.
    pub fn work_lease_store(&self, ready: &BootstrapReady, id: &str) -> Result<CosmosLeaseStore> {
        if ready.id != self.store.id
            || !compatible(&self.plan, &ready.plan)
            || !ready.lease_ids.iter().any(|known| known == id)
        {
            return Err(invalid("work lease is not part of this verified workload"));
        }
        let mut store = self.store.clone();
        store.id = id.to_owned();
        store.item = ItemReference::from_name(
            &store.container,
            store.partition_key.clone(),
            store.id.clone(),
        );
        store.identity = Arc::new(());
        store.readiness = Some(ReadinessGate {
            bootstrap: Arc::new(self.store.clone()),
            plan: ready.plan.clone(),
        });
        Ok(store)
    }

    async fn ensure_loop(&self, incarnation: &str) -> Result<BootstrapReady> {
        self.create_if_absent().await?;
        let mut occupied: Option<super::LeaseObservation> = None;
        loop {
            let observation = self.store.observe().await?;
            let metadata = self.metadata(&observation.record)?;
            if metadata.phase == BootstrapPhase::Ready {
                if self.resume_topology_transitions().await? > 0 {
                    continue;
                }
                let records = self.discover().await?;
                if coverage(&metadata.plan, &records, &self.store.id)?.is_empty() {
                    // Fence the completeness snapshot against a concurrent readiness transition.
                    let guards =
                        verification_reads(&self.store.id, &observation.revision, &records)?;
                    let response = self
                        .store
                        .execute(
                            CosmosOperation::batch(
                                self.store.container.clone(),
                                self.store.partition_key.clone(),
                            )
                            .with_body(serde_json::to_vec(&guards)?),
                        )
                        .await?;
                    match batch_revision(response, guards.len(), 0) {
                        Ok(_) => return ready(&self.store.id, metadata.plan, records),
                        Err(error)
                            if error.status().status_code() == StatusCode::PreconditionFailed =>
                        {
                            continue
                        }
                        Err(error) => return Err(error),
                    }
                }
            }
            let eligible = if let Some(previous) = occupied.as_ref() {
                if previous.revision == observation.revision {
                    previous
                } else {
                    &observation
                }
            } else {
                &observation
            };
            match self.store.acquire(eligible, incarnation).await {
                Ok(Some(session)) => return self.initialize_under_authority(session).await,
                Ok(None) => {}
                Err(error) if error.status().status_code() == StatusCode::PreconditionFailed => {
                    occupied = None;
                    continue;
                }
                Err(error) => return Err(error),
            }
            if occupied
                .as_ref()
                .is_none_or(|previous| previous.revision != observation.revision)
            {
                occupied = Some(observation);
            }
            sleep(Duration::from_millis(rand::random_range(25..=75))).await;
        }
    }

    async fn create_if_absent(&self) -> Result<()> {
        let metadata = Metadata {
            protocol: 1,
            phase: BootstrapPhase::Initializing,
            plan: self.plan.clone(),
            committed_generation: None,
        };
        let record = LeaseRecord {
            id: self.store.id.clone(),
            version: 1,
            ownership: LeaseIdentity {
                owner: None,
                generation: 0,
            },
            range: self.plan.expected.clone(),
            checkpoint: self.plan.seeds[0].checkpoint.clone(),
            lease_duration_ms: self.store.options.duration_millis(),
            extra: BTreeMap::from([
                ("workload".into(), json!(self.store.id)),
                ("bootstrap".into(), serde_json::to_value(metadata)?),
            ]),
        };
        match self
            .store
            .execute(
                CosmosOperation::create_item(self.store.item.clone())
                    .with_precondition(
                        azure_data_cosmos_driver::models::Precondition::if_none_match("*"),
                    )
                    .with_body(serde_json::to_vec(&record)?),
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(error)
                if matches!(
                    error.status().status_code(),
                    StatusCode::Conflict | StatusCode::PreconditionFailed
                ) =>
            {
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn metadata(&self, record: &LeaseRecord) -> Result<Metadata> {
        let metadata: Metadata = serde_json::from_value(
            record
                .extra
                .get("bootstrap")
                .ok_or_else(|| invalid("workload record has no bootstrap metadata"))?
                .clone(),
        )?;
        validate_plan(&metadata.plan)?;
        if metadata.protocol != 1
            || !compatible(&metadata.plan, &self.plan)
            || (metadata.phase == BootstrapPhase::Ready
                && metadata.committed_generation != Some(INITIALIZATION_GENERATION))
        {
            return Err(invalid(
                "bootstrap configuration or readiness generation is incompatible",
            ));
        }
        Ok(metadata)
    }

    async fn discover(&self) -> Result<Vec<LeaseRecord>> {
        self.discover_paged(None).await
    }

    async fn discover_paged(
        &self,
        page_size: Option<std::num::NonZeroU32>,
    ) -> Result<Vec<LeaseRecord>> {
        #[derive(Deserialize)]
        struct Envelope {
            #[serde(rename = "Documents")]
            items: Vec<LeaseRecord>,
        }
        let scope = FeedRange::for_partition(
            self.store.partition_key.clone(),
            self.store.container.partition_key_definition(),
        );
        let query = json!({"query":"SELECT * FROM c WHERE c.workload = @workload AND c.id != @bootstrap AND (NOT IS_DEFINED(c.recordKind) OR c.recordKind = 'WorkLease')",
            "parameters":[{"name":"@workload","value":self.store.id},
                {"name":"@bootstrap","value":self.store.id}]});
        let mut operation = CosmosOperation::query_items(self.store.container.clone(), Some(scope))
            .with_body(serde_json::to_vec(&query)?);
        if let Some(page_size) = page_size {
            operation = operation.with_max_item_count(
                azure_data_cosmos_driver::models::MaxItemCountHint::Limit(page_size),
            );
        }
        let mut options = OperationOptions::default();
        options.read_consistency_strategy =
            Some(azure_data_cosmos_driver::options::ReadConsistencyStrategy::LatestCommitted);
        let mut plan = self
            .store
            .driver
            .plan_operation(
                operation,
                &options,
                None,
                &azure_data_cosmos_driver::options::PlanOptions::default(),
            )
            .await?;
        let mut records = Vec::new();
        loop {
            let response = timeout(
                self.store.options.request_timeout,
                self.store.driver.execute_plan(
                    &mut plan,
                    Some(self.store.container.clone()),
                    options.clone(),
                ),
            )
            .await
            .map_err(|_| super::timed_out("bootstrap lease discovery timed out"))??;
            let Some(response) = response else {
                return Ok(records);
            };
            let diagnostics = response.diagnostics();
            let page = match response.into_body() {
                ResponseBody::Bytes(bytes) => ResponseBody::Bytes(bytes)
                    .into_single::<Envelope>()
                    .map(|page| page.items),
                ResponseBody::Items(items) => {
                    ResponseBody::Items(items).into_items::<LeaseRecord>()
                }
                ResponseBody::NoPayload => Err(invalid("bootstrap discovery returned no payload")),
            }
            .map_err(|error| {
                CosmosErrorBuilder::from_error(error)
                    .with_diagnostics(diagnostics)
                    .build()
            })?;
            records.extend(page);
            for record in &records {
                super::decisions::validate_record(
                    record,
                    &record.id,
                    self.store.options.duration_millis(),
                )?;
            }
            if records.len() > MAX_INITIAL_LEASES {
                return Err(invalid("bootstrap discovery exceeded bounded lease count"));
            }
        }
    }

    async fn initialize_under_authority(&self, session: LeaseSession) -> Result<BootstrapReady> {
        let _guard = session.begin_run()?;
        let result = {
            let work = self.initialize(&session);
            tokio::pin!(work);
            tokio::select! {
                biased;
                _ = session.monitor_safety() => Err(invalid("bootstrap authority became unsafe")),
                result = session.maintain() => result.and_then(|_| Err(invalid("bootstrap maintenance ended"))),
                result = &mut work => result,
            }
        };
        let cleanup = if session.control().is_lost() {
            None
        } else {
            session.release().await.err()
        };
        match (result, cleanup) {
            (Err(error), Some(cleanup)) => Err(CosmosErrorBuilder::from_error(error)
                .with_context(format!("bootstrap cleanup also failed: {cleanup}"))
                .build()),
            (Err(error), None) => Err(error),
            (Ok(_), Some(cleanup)) => Err(cleanup),
            (Ok(ready), None) => Ok(ready),
        }
    }

    async fn initialize(&self, session: &LeaseSession) -> Result<BootstrapReady> {
        let mut metadata = {
            let state = session.state.lock().await;
            self.metadata(&state.record)?
        };
        metadata.phase = BootstrapPhase::Initializing;
        metadata.committed_generation = None;
        session.bootstrap_write(&metadata, None, &[]).await?;
        let existing = self.discover().await?;
        let missing = coverage(&metadata.plan, &existing, &self.store.id)?;
        for index in missing {
            let seed = &metadata.plan.seeds[index];
            let record = LeaseRecord {
                id: format!("{}.lease.{index}", self.store.id),
                version: 1,
                ownership: LeaseIdentity {
                    owner: None,
                    generation: 0,
                },
                range: seed.range.clone(),
                checkpoint: seed.checkpoint.clone(),
                lease_duration_ms: self.store.options.duration_millis(),
                extra: BTreeMap::from([
                    ("workload".into(), json!(self.store.id)),
                    (
                        "initializationGeneration".into(),
                        json!(INITIALIZATION_GENERATION),
                    ),
                ]),
            };
            session
                .bootstrap_write(&metadata, Some(record), &[])
                .await?;
        }
        let records = self.discover().await?;
        if !coverage(&metadata.plan, &records, &self.store.id)?.is_empty() {
            return Err(invalid(
                "required lease coverage remains incomplete; readiness forbidden",
            ));
        }
        metadata.phase = BootstrapPhase::Ready;
        metadata.committed_generation = Some(INITIALIZATION_GENERATION);
        session.bootstrap_write(&metadata, None, &records).await?;
        ready(&self.store.id, metadata.plan, records)
    }
}

#[derive(Clone)]
pub(super) struct ReadinessGate {
    bootstrap: Arc<CosmosLeaseStore>,
    plan: BootstrapPlan,
}
impl ReadinessGate {
    pub(super) async fn acquire(
        &self,
        store: &CosmosLeaseStore,
        record: &LeaseRecord,
        expected_revision: &str,
    ) -> Result<String> {
        if record.extra.get("workload") != Some(&json!(self.bootstrap.id))
            || !processing_eligible(record)?
        {
            return Err(invalid(
                "work lease belongs to an uncommitted initialization generation",
            ));
        }
        let observed = self.bootstrap.observe().await?;
        let metadata: Metadata = serde_json::from_value(
            observed
                .record
                .extra
                .get("bootstrap")
                .ok_or_else(|| invalid("bootstrap metadata missing"))?
                .clone(),
        )?;
        validate_plan(&metadata.plan)?;
        if metadata.phase != BootstrapPhase::Ready
            || metadata.committed_generation != Some(INITIALIZATION_GENERATION)
            || metadata.protocol != 1
            || !compatible(&metadata.plan, &self.plan)
        {
            return Err(invalid("workload is not ready for acquisition"));
        }
        let response = store.execute(CosmosOperation::batch(store.container.clone(), store.partition_key.clone())
            .with_body(serde_json::to_vec(&json!([
                {"operationType":"Read","id":self.bootstrap.id,"ifMatch":observed.revision},
                {"operationType":"Replace","id":record.id,"ifMatch":expected_revision,"resourceBody":record}
            ]))?)).await?;
        batch_revision(response, 2, 1)
    }
}

impl LeaseSession {
    async fn bootstrap_write(
        &self,
        metadata: &Metadata,
        create: Option<LeaseRecord>,
        coverage: &[LeaseRecord],
    ) -> Result<()> {
        let mut state = self.state.lock().await;
        let deadline = self.admit_write(&mut state)?;
        let mut record = state.record.clone();
        record
            .extra
            .insert("bootstrap".into(), serde_json::to_value(metadata)?);
        let mut operations = vec![json!({"operationType":"Replace","id":record.id,
            "ifMatch":state.revision,"resourceBody":record})];
        if let Some(create) = create {
            operations.push(json!({"operationType":"Create","resourceBody":create}));
        }
        for lease in coverage {
            let etag = lease
                .extra
                .get("_etag")
                .and_then(Value::as_str)
                .ok_or_else(|| invalid("coverage lease is missing its store ETag"))?;
            operations.push(json!({"operationType":"Read","id":lease.id,"ifMatch":etag}));
        }
        let began = Instant::now();
        state.authority = AuthorityState::WriteInFlight {
            safe_until: deadline,
        };
        let write = LeaseWriteGuard::new(&self.control);
        let result = tokio::time::timeout_at(
            deadline,
            self.store.execute(
                CosmosOperation::batch(
                    self.store.container.clone(),
                    self.store.partition_key.clone(),
                )
                .with_body(serde_json::to_vec(&operations)?),
            ),
        )
        .await
        .map_err(|_| super::timed_out("bootstrap fenced write exceeded authority deadline"))
        .and_then(|result| result)
        .and_then(|response| batch_revision(response, operations.len(), 0));
        let revision = match result {
            Ok(revision) => revision,
            Err(error) => {
                state.authority = AuthorityState::Revoked;
                self.control.lose_ownership();
                return Err(error);
            }
        };
        let safe_until = self.confirm_write(&mut state, &revision, began, deadline)?;
        state.record = record;
        state.revision = revision;
        state.authority = AuthorityState::Active { safe_until };
        self.deadline.send_replace(safe_until);
        write.confirm();
        Ok(())
    }
}

fn batch_revision(
    response: azure_data_cosmos_driver::CosmosResponse,
    count: usize,
    revision_index: usize,
) -> Result<String> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct BatchResult {
        status_code: u16,
        e_tag: Option<String>,
    }
    let diagnostics = response.diagnostics();
    let results: Vec<BatchResult> = response.into_body().into_single().map_err(|error| {
        CosmosErrorBuilder::from_error(error)
            .with_diagnostics(diagnostics.clone())
            .build()
    })?;
    if results.len() != count
        || results
            .iter()
            .any(|result| !(200..300).contains(&result.status_code))
    {
        let code = results
            .iter()
            .find(|result| result.status_code >= 300 && result.status_code != 424)
            .map(|result| result.status_code);
        let status = match code {
            Some(412) => StatusCode::PreconditionFailed,
            Some(409) => StatusCode::Conflict,
            _ => StatusCode::BadRequest,
        };
        return Err(CosmosError::builder()
            .with_status(azure_data_cosmos_driver::CosmosStatus::new(status))
            .with_message(format!(
                "bootstrap transaction did not commit; operation status {code:?}"
            ))
            .with_diagnostics(diagnostics)
            .build());
    }

    results
        .get(revision_index)
        .and_then(|result| result.e_tag.clone())
        .filter(|etag| !etag.is_empty())
        .ok_or_else(|| {
            CosmosError::builder()
                .with_message("bootstrap transaction has no per-item ETag")
                .with_diagnostics(diagnostics)
                .build()
        })
}

pub(super) fn processing_eligible(record: &LeaseRecord) -> Result<bool> {
    if record
        .extra
        .get("recordKind")
        .is_some_and(|kind| kind.as_str() != Some("WorkLease"))
    {
        return Ok(false);
    }
    if record.extra.get("initializationGeneration") != Some(&json!(INITIALIZATION_GENERATION)) {
        return Ok(false);
    }
    match record.extra.get("leaseState") {
        None => Ok(true),
        Some(value) if value.as_str() == Some("Active") => Ok(true),
        Some(value)
            if matches!(
                value.as_str(),
                Some("Pending" | "Transitioning" | "Retired")
            ) =>
        {
            Ok(false)
        }
        Some(_) => Err(invalid("work lease has an unsupported lifecycle state")),
    }
}

fn verification_reads(id: &str, revision: &str, records: &[LeaseRecord]) -> Result<Vec<Value>> {
    let mut operations = vec![json!({"operationType":"Read","id":id,"ifMatch":revision})];
    for record in records {
        let etag = record
            .extra
            .get("_etag")
            .and_then(Value::as_str)
            .ok_or_else(|| invalid("coverage lease is missing its store ETag"))?;
        operations.push(json!({"operationType":"Read","id":record.id,"ifMatch":etag}));
    }
    Ok(operations)
}

fn workload_id(plan: &BootstrapPlan) -> Result<String> {
    workload_identity(&plan.source, &plan.group)
}
fn validate_partition(container: &ContainerReference) -> Result<()> {
    let paths = container.partition_key_definition().paths();
    if paths.len() != 1 || paths[0] != "/workload" {
        return Err(invalid(
            "bootstrap requires an existing container partitioned by /workload",
        ));
    }
    Ok(())
}
fn workload_identity(source: &str, group: &str) -> Result<String> {
    let id = format!(
        "cfp.{}",
        URL_SAFE_NO_PAD.encode(serde_json::to_vec(&(source, group))?)
    );
    if id.len() > 800 {
        return Err(invalid(
            "workload identity exceeds bounded Cosmos item ID length",
        ));
    }
    Ok(id)
}
fn compatible(a: &BootstrapPlan, b: &BootstrapPlan) -> bool {
    a.source == b.source
        && a.group == b.group
        && a.mode == b.mode
        && a.start == b.start
        && a.expected == b.expected
}
fn interval(range: &FeedRange) -> bool {
    !range.is_logical_partition() && range.min_inclusive() < range.max_exclusive()
}
fn validate_plan(plan: &BootstrapPlan) -> Result<()> {
    if plan.source.trim().is_empty()
        || plan.group.trim().is_empty()
        || !interval(&plan.expected)
        || plan.seeds.is_empty()
        || plan.seeds.len() > MAX_INITIAL_LEASES
    {
        return Err(invalid(
            "bootstrap requires explicit source/configuration and bounded complete seeds",
        ));
    }
    let mut cursor = plan.expected.min_inclusive();
    for seed in &plan.seeds {
        if !interval(&seed.range)
            || seed.checkpoint.is_empty()
            || seed.range.min_inclusive() != cursor
            || seed.range.max_exclusive() > plan.expected.max_exclusive()
        {
            return Err(invalid(
                "initial lease plan has a gap, overlap, or invalid checkpoint",
            ));
        }
        cursor = seed.range.max_exclusive();
    }
    if cursor != plan.expected.max_exclusive() {
        return Err(invalid("initial lease plan is incomplete"));
    }
    Ok(())
}

// Missing assignments are created only when no existing logical lease covers their range.
fn coverage(plan: &BootstrapPlan, records: &[LeaseRecord], workload: &str) -> Result<Vec<usize>> {
    let mut ordered = Vec::new();
    for record in records {
        if record.extra.get("workload") != Some(&json!(workload))
            || record.extra.get("initializationGeneration")
                != Some(&json!(INITIALIZATION_GENERATION))
            || record.version != 1
            || record.checkpoint.is_empty()
            || !interval(&record.range)
        {
            return Err(invalid(
                "workload coverage contains an invalid or unrecognized-generation record",
            ));
        }
        if processing_eligible(record)? {
            ordered.push(record);
        }
    }
    ordered.sort_by(|a, b| a.range.min_inclusive().cmp(b.range.min_inclusive()));
    let mut ids = std::collections::HashSet::new();
    let mut previous_max = None;
    for lease in &ordered {
        if !ids.insert(&lease.id)
            || !processing_eligible(lease)?
            || !interval(&lease.range)
            || lease.extra.get("workload") != Some(&json!(workload))
            || lease.extra.get("initializationGeneration")
                != Some(&json!(INITIALIZATION_GENERATION))
            || lease.checkpoint.is_empty()
            || lease.version != 1
            || lease.range.min_inclusive() < plan.expected.min_inclusive()
            || lease.range.max_exclusive() > plan.expected.max_exclusive()
            || previous_max.is_some_and(|max| lease.range.min_inclusive() < max)
        {
            return Err(invalid(
                "existing workload lease coverage is invalid or overlapping",
            ));
        }
        previous_max = Some(lease.range.max_exclusive());
    }
    let mut missing = Vec::new();
    for (index, seed) in plan.seeds.iter().enumerate() {
        let overlaps: Vec<_> = ordered
            .iter()
            .filter(|lease| lease.range.overlaps(&seed.range))
            .collect();
        if overlaps.is_empty() {
            missing.push(index);
            continue;
        }
        let mut cursor = seed.range.min_inclusive();
        for lease in overlaps {
            if lease.range.min_inclusive() > cursor {
                return Err(invalid(
                    "partial seed coverage cannot be repaired without checkpoint transformation",
                ));
            }
            cursor = lease.range.max_exclusive().min(seed.range.max_exclusive());
        }
        if cursor != seed.range.max_exclusive() {
            return Err(invalid(
                "partial seed coverage cannot be repaired without checkpoint transformation",
            ));
        }
    }
    Ok(missing)
}
fn ready(id: &str, plan: BootstrapPlan, records: Vec<LeaseRecord>) -> Result<BootstrapReady> {
    if !coverage(&plan, &records, id)?.is_empty() {
        return Err(invalid("workload has incomplete coverage"));
    }
    Ok(BootstrapReady {
        id: id.to_owned(),
        plan,
        lease_ids: records
            .into_iter()
            .filter(|record| processing_eligible(record).expect("coverage validated lifecycle"))
            .map(|lease| lease.id)
            .collect(),
    })
}

#[cfg(test)]
mod tests;
