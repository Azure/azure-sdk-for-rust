// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod coalescing;
pub(super) mod store;

pub(crate) use coalescing::BootstrapCoalescer;

use crate::{
    change_feed_processor::{bootstrap::coalescing::AttemptKey, operation_options},
    models::{
        effective_partition_key::EffectivePartitionKey, ChangeFeedFactoryMode, ContainerReference,
        ContinuationToken, CosmosOperation, FeedRange,
    },
    CosmosDriver, CosmosError, CosmosStatus, Result,
};
use azure_core::{
    fmt::SafeDebug,
    http::{Etag, StatusCode},
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use futures::{future::Either, pin_mut, FutureExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    borrow::Cow,
    error::Error as StdError,
    fmt,
    future::Future,
    panic::AssertUnwindSafe,
    time::{Duration, Instant},
};
use store::LeaseStore;
use uuid::Uuid;

pub(super) const SCHEMA_VERSION: u32 = 1;
const LOCK_TTL_SECONDS: i32 = 30;
const ATTEMPT_TIMEOUT: Duration = Duration::from_secs(25);
pub(super) const BOOTSTRAP_TIMEOUT: Duration = Duration::from_secs(120);
const RETRY_DELAY: Duration = Duration::from_secs(1);

/// Initializes a workload's metadata and unowned processing leases.
///
/// The namespace combines the monitored account endpoint's origin, container RID,
/// and `workload_name` within the supplied lease container. `worker_name`
/// identifies the worker, not the workload. Names must not be blank and are not
/// trimmed. Source and lease containers may belong to different accounts.
///
/// The lease container must use `/id` partitioning and a default TTL of `-1`.
/// Only the initialization lock has an expiring TTL. Single-write and multi-write
/// lease accounts use the lease driver's normal routing and consistency settings.
///
/// A durable definition binds the mode before initialization. A create-only range
/// plan lets interrupted attempts finish the same initial coverage. New leases
/// have neither an owner nor a continuation; the reader applies its configured
/// start options when no checkpoint exists. A compatible completion marker ends
/// bootstrap without inspecting the range plan or processing leases. Missing
/// leases are filled only while initialization is incomplete; later topology
/// changes belong to the processing runtime.
///
/// Claims expire after 30 seconds; each claimed attempt is bounded to 25 seconds.
/// TTL election is not strict fencing: delayed attempts can overlap after expiry.
/// Attempts use a persisted plan and create-only lease writes; lease-create
/// conflicts mean "already exists" without read-back validation or progress replacement. The claim does not provide
/// global fencing across write regions; visibility and conflict resolution follow
/// the lease account's consistency and conflict resolution policies.
///
/// Concurrent compatible calls on the same factory-created drivers share
/// in-flight attempts. Each caller has its own two-minute budget, including retry
/// delays. Joining never extends an existing attempt's deadline. A caller with
/// remaining time re-registers after a retryable failure, even if the previous
/// attempt exhausted another caller's budget. Worker names do not split work.
/// Dropping one waiting future does not cancel other waiters; dropping all
/// waiters cancels local work but cannot undo requests already sent. Driver
/// values unwrapped from their shared handles retain inline execution.
///
/// Completed results are not cached. Unwinding panics are logged and retried as
/// failed attempts. After a caught initialization panic, claim release is
/// attempted; interrupted or unsuccessful cleanup leaves the claim for TTL
/// expiration. Retries wait one second without resetting the caller's deadline.
/// Other failures are returned.
///
/// # Errors
///
/// Returns an error for invalid names, timeouts, container configuration, malformed
/// persisted records, incompatible modes or ranges, or failed service operations.
/// A mode mismatch is rejected even if initialization has already completed.
/// Repeated transient failures or unwinding panics exhaust the caller's budget
/// and return a timeout. Process aborts and non-yielding code cannot be recovered.
///
/// # Examples
///
/// ```no_run
/// use azure_data_cosmos_driver::{
///     change_feed_processor::bootstrap,
///     models::{ChangeFeedFactoryMode, ContainerReference},
///     CosmosDriver,
/// };
///
/// async fn initialize(
///     source_driver: &CosmosDriver,
///     source: &ContainerReference,
///     lease_driver: &CosmosDriver,
///     leases: &ContainerReference,
/// ) -> azure_data_cosmos_driver::Result<()> {
///     bootstrap(
///         source_driver,
///         source,
///         lease_driver,
///         leases,
///         ChangeFeedFactoryMode::Incremental,
///         "order-projection",
///         "worker-01",
///     ).await
/// }
/// ```
pub async fn bootstrap(
    source_driver: &CosmosDriver,
    source: &ContainerReference,
    lease_driver: &CosmosDriver,
    leases: &ContainerReference,
    mode: ChangeFeedFactoryMode,
    workload_name: &str,
    worker_name: &str,
) -> Result<()> {
    bootstrap_with_timeout(
        source_driver,
        source,
        lease_driver,
        leases,
        mode,
        workload_name,
        worker_name,
        BOOTSTRAP_TIMEOUT,
    )
    .await
}

// Keep the existing public bootstrap signature while letting processor options set the caller budget.
#[allow(clippy::too_many_arguments)]
pub(super) async fn bootstrap_with_timeout(
    source_driver: &CosmosDriver,
    source: &ContainerReference,
    lease_driver: &CosmosDriver,
    leases: &ContainerReference,
    mode: ChangeFeedFactoryMode,
    workload_name: &str,
    worker_name: &str,
    timeout: Duration,
) -> Result<()> {
    let deadline = Instant::now()
        .checked_add(timeout)
        .filter(|_| !timeout.is_zero() && azure_core::time::Duration::try_from(timeout).is_ok())
        .ok_or_else(|| error(StatusCode::BadRequest, "Invalid bootstrap timeout."))?;
    let mut context = Context::new(
        source_driver,
        source,
        lease_driver,
        leases,
        mode,
        workload_name,
        worker_name,
    )?;
    context.store.deadline = deadline;
    let (Some(source_owner), Some(lease_owner)) = (
        source_driver.retain_for_bootstrap(),
        lease_driver.retain_for_bootstrap(),
    ) else {
        tracing::debug!("Running bootstrap inline for drivers without shared ownership");
        return context.run().await;
    };
    let key = AttemptKey {
        namespace: context.namespace.clone(),
        lease_container: leases.rid().to_owned(),
        lease_driver: std::ptr::from_ref(lease_driver) as usize,
        mode: std::mem::discriminant(&mode),
    };
    let source = source.clone();
    let leases = leases.clone();
    let deadline = context.store.deadline;
    let identity = context.identity;
    let namespace = context.namespace;
    let worker_name = context.worker_name;
    let retry_delay = context.retry_delay;
    retry_bootstrap(
        || {
            let source_owner = source_owner.clone();
            let lease_owner = lease_owner.clone();
            let source = source.clone();
            let leases = leases.clone();
            let identity = identity.clone();
            let namespace = namespace.clone();
            let worker_name = worker_name.clone();
            source_driver
                .bootstrap_coalescer()
                .start_or_join(key.clone(), deadline, move || {
                    async move {
                        let context = Context {
                            source_driver: &source_owner,
                            source: &source,
                            store: LeaseStore {
                                driver: &lease_owner,
                                container: &leases,
                                deadline,
                            },
                            identity,
                            namespace,
                            worker_name,
                            retry_delay,
                        };
                        Box::pin(context.run_attempt()).await
                    }
                    .boxed()
                })
        },
        deadline,
        retry_delay,
    )
    .await
}

/// Persisted workload identity used to reject incompatible source, mode, or schema records.
/// Worker identity is excluded so another worker can initialize or resume the same workload.
#[derive(Clone, SafeDebug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct WorkloadIdentity {
    pub(super) version: u32,
    pub(super) account: String,
    pub(super) container_rid: String,
    #[serde(rename = "processorName")]
    pub(super) workload_name: String,
    #[serde(
        serialize_with = "serialize_mode",
        deserialize_with = "deserialize_mode"
    )]
    pub(super) mode: ChangeFeedFactoryMode,
}

// Preserve schema-1 workload values independently of the driver variant names.
fn serialize_mode<S>(
    mode: &ChangeFeedFactoryMode,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(match mode {
        ChangeFeedFactoryMode::Incremental => "latestVersion",
        ChangeFeedFactoryMode::FullFidelity => "allVersionsAndDeletes",
    })
}

fn deserialize_mode<'de, D>(deserializer: D) -> std::result::Result<ChangeFeedFactoryMode, D::Error>
where
    D: Deserializer<'de>,
{
    let mode = String::deserialize(deserializer)?;
    match mode.as_str() {
        "latestVersion" => Ok(ChangeFeedFactoryMode::Incremental),
        "allVersionsAndDeletes" => Ok(ChangeFeedFactoryMode::FullFidelity),
        value => Err(serde::de::Error::unknown_variant(
            value,
            &["latestVersion", "allVersionsAndDeletes"],
        )),
    }
}

/// Common identity and TTL fields for bootstrap documents; also the definition and ready-marker shape.
#[derive(Clone, SafeDebug, Serialize, Deserialize)]
pub(super) struct Record {
    pub(super) id: String,
    pub(super) workload: WorkloadIdentity,
    pub(super) ttl: i32,
}

/// Persisted initial partition ranges that replacement initializers reuse to finish lease creation.
#[derive(Clone, SafeDebug, Serialize, Deserialize)]
pub(super) struct RangePlan {
    #[serde(flatten)]
    pub(super) record: Record,
    pub(super) ranges: Vec<FeedRange>,
}

/// Persisted processing lease, including its range, ownership, durable progress, and liveness timestamps.
#[derive(Clone, SafeDebug, Serialize, Deserialize)]
pub(super) struct LeaseDocument {
    #[serde(flatten)]
    pub(super) record: Record,
    pub(super) range: FeedRange,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) continuation: Option<ContinuationToken>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) incarnation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) renewed_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) last_processed_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) processing_deadline: Option<i64>,
}

/// Expiring initialization-lock document identifying the worker and its unique attempt.
#[derive(SafeDebug, Serialize, Deserialize)]
struct InitializationLock {
    #[serde(flatten)]
    record: Record,
    attempt: String,
    owner: String,
}

/// Local lock ETag and attempt deadline used to detect lost ownership and conditionally release the lock.
struct InitializationClaim {
    etag: Etag,
    deadline: Instant,
}

/// Borrowed drivers, storage, identity, and timing for one caller's initialization work; not persisted.
struct Context<'a> {
    source_driver: &'a CosmosDriver,
    source: &'a ContainerReference,
    store: LeaseStore<'a>,
    identity: WorkloadIdentity,
    namespace: String,
    worker_name: String,
    retry_delay: Duration,
}

impl<'a> Context<'a> {
    fn new(
        source_driver: &'a CosmosDriver,
        source: &'a ContainerReference,
        lease_driver: &'a CosmosDriver,
        leases: &'a ContainerReference,
        mode: ChangeFeedFactoryMode,
        workload_name: &str,
        worker_name: &str,
    ) -> Result<Self> {
        for (name, value) in [
            ("workload_name", workload_name),
            ("worker_name", worker_name),
        ] {
            if value.trim().is_empty() {
                return Err(error(
                    StatusCode::BadRequest,
                    format!("{name} must not be blank."),
                ));
            }
        }
        if source.account() != source_driver.account() || leases.account() != lease_driver.account()
        {
            return Err(error(
                StatusCode::BadRequest,
                "Container and driver accounts must match.",
            ));
        }
        let identity = WorkloadIdentity {
            version: SCHEMA_VERSION,
            account: source.account().endpoint().origin().ascii_serialization(),
            container_rid: source.rid().to_owned(),
            workload_name: workload_name.to_owned(),
            mode,
        };
        let namespace = namespace(&identity)?;
        Ok(Self {
            source_driver,
            source,
            store: LeaseStore {
                driver: lease_driver,
                container: leases,
                deadline: Instant::now() + BOOTSTRAP_TIMEOUT,
            },
            identity,
            namespace,
            worker_name: worker_name.to_owned(),
            retry_delay: RETRY_DELAY,
        })
    }

    async fn run(&self) -> Result<()> {
        retry_bootstrap(
            || Box::pin(self.run_attempt()),
            self.store.deadline,
            self.retry_delay,
        )
        .await
    }

    fn record(&self, suffix: &str) -> Record {
        Record {
            id: format!("{}.{suffix}", self.namespace),
            workload: self.identity.clone(),
            ttl: -1,
        }
    }

    fn validate(&self, actual: &Record, expected: &Record) -> Result<()> {
        if actual.id != expected.id
            || actual.workload != expected.workload
            || actual.ttl != expected.ttl
        {
            return Err(error(
                StatusCode::Conflict,
                "Bootstrap record is incompatible with the workload identity, mode, or schema.",
            ));
        }
        Ok(())
    }

    async fn ensure_record(&self, record: &Record) -> Result<()> {
        if let Some(existing) = self.store.read::<Record>(&record.id).await? {
            return self.validate(&existing.0, record);
        }
        if self.store.create(&record.id, record).await? {
            return Ok(());
        }
        let existing = self
            .store
            .read::<Record>(&record.id)
            .await?
            .ok_or_else(|| {
                error(
                    StatusCode::ServiceUnavailable,
                    "Conflicting bootstrap record is not yet readable.",
                )
            })?;
        self.validate(&existing.0, record)
    }

    async fn ready(&self) -> Result<bool> {
        let expected = self.record("info");
        match self.store.read::<Record>(&expected.id).await? {
            Some(existing) => {
                self.validate(&existing.0, &expected)?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    async fn validate_container(&self) -> Result<()> {
        let response = self
            .store
            .execute(CosmosOperation::read_container(
                self.store.container.clone(),
            ))
            .await?;
        let properties: serde_json::Value = response.into_body().into_single()?;
        if properties
            .get("defaultTtl")
            .and_then(serde_json::Value::as_i64)
            != Some(-1)
            || properties.pointer("/partitionKey/paths") != Some(&serde_json::json!(["/id"]))
        {
            return Err(error(
                StatusCode::BadRequest,
                "Bootstrap requires a lease container partitioned by /id with defaultTtl -1.",
            ));
        }
        Ok(())
    }

    async fn try_claim(&self) -> Result<Option<InitializationClaim>> {
        let started = Instant::now();
        let mut record = self.record("lock");
        record.ttl = LOCK_TTL_SECONDS;
        let lock = InitializationLock {
            record,
            attempt: Uuid::new_v4().to_string(),
            owner: self.worker_name.clone(),
        };
        self.store.create(&lock.record.id, &lock).await?;
        let Some(current) = self
            .store
            .read::<InitializationLock>(&lock.record.id)
            .await?
        else {
            return Ok(None);
        };
        self.validate(&current.0.record, &lock.record)?;
        if current.0.attempt != lock.attempt {
            return Ok(None);
        }
        Ok(Some(InitializationClaim {
            etag: current.1,
            deadline: (started + ATTEMPT_TIMEOUT).min(self.store.deadline),
        }))
    }

    async fn check_claim(&self, claim: &InitializationClaim) -> Result<()> {
        if Instant::now() >= claim.deadline {
            return Err(error(
                StatusCode::RequestTimeout,
                "Bootstrap attempt deadline exceeded.",
            ));
        }
        let current = self
            .store
            .read::<InitializationLock>(&self.record("lock").id)
            .await?;
        if current.as_ref().is_none_or(|record| record.1 != claim.etag) {
            return Err(error(
                StatusCode::PreconditionFailed,
                "Bootstrap initialization claim was lost.",
            ));
        }
        Ok(())
    }

    async fn plan(&self, claim: &InitializationClaim) -> Result<Option<RangePlan>> {
        let record = self.record("plan");
        if let Some(existing) = self.store.read::<RangePlan>(&record.id).await? {
            self.validate(&existing.0.record, &record)?;
            validate_ranges(&existing.0.ranges)?;
            return Ok(Some(existing.0));
        }
        if self.ready().await? {
            return Ok(None);
        }
        // Refresh through the shared routing cache's /pkranges metadata feed.
        // Cache fallback can still return an older map; strict bootstrap freshness is not guaranteed.
        let topology = self
            .source_driver
            .resolve_all_partition_key_ranges_with_options(self.source, true, operation_options())
            .await?
            .ok_or_else(|| {
                error(
                    StatusCode::ServiceUnavailable,
                    "Source partition topology is unavailable.",
                )
            })?;
        let ranges = topology
            .iter()
            .map(FeedRange::try_from)
            .collect::<Result<Vec<_>>>()?;
        validate_ranges(&ranges)?;
        let plan = RangePlan { record, ranges };
        self.check_claim(claim).await?;
        if self.store.create(&plan.record.id, &plan).await? {
            return Ok(Some(plan));
        }
        let existing = self
            .store
            .read::<RangePlan>(&plan.record.id)
            .await?
            .ok_or_else(|| {
                error(
                    StatusCode::ServiceUnavailable,
                    "Conflicting bootstrap range plan is not yet readable.",
                )
            })?;
        self.validate(&existing.0.record, &plan.record)?;
        validate_ranges(&existing.0.ranges)?;
        Ok(Some(existing.0))
    }

    async fn create_leases(&self, plan: &RangePlan, claim: &InitializationClaim) -> Result<()> {
        // Create each planned lease directly; a 409 preserves the existing document without reading it.
        for range in &plan.ranges {
            let expected = LeaseDocument {
                record: self.record(&format!(
                    "lease.{}-{}",
                    range.min_inclusive(),
                    range.max_exclusive()
                )),
                range: range.clone(),
                owner: None,
                continuation: None,

                incarnation: None,
                renewed_at: None,
                last_processed_at: None,
                processing_deadline: None,
            };
            self.check_claim(claim).await?;
            if !self.store.create(&expected.record.id, &expected).await? {
                tracing::debug!(lease = %expected.record.id, "Processing lease already exists; preserving it");
            }
        }
        Ok(())
    }

    async fn initialize(&self, claim: &InitializationClaim) -> Result<()> {
        if self.ready().await? {
            return Ok(());
        }
        let Some(plan) = self.plan(claim).await? else {
            return Ok(());
        };
        self.create_leases(&plan, claim).await?;
        self.check_claim(claim).await?;
        // Any unresolved lease error prevents publishing readiness for this plan.
        self.ensure_record(&self.record("info")).await
    }

    async fn initialize_and_release(
        &self,
        claim: &InitializationClaim,
        initialization: impl Future<Output = Result<()>>,
    ) -> Result<()> {
        let result = catch_bootstrap_panic(bounded(initialization, claim.deadline)).await;
        let release = catch_bootstrap_panic(Box::pin(
            self.store
                .release(&self.record("lock").id, claim.etag.clone()),
        ))
        .await;
        match (result, release) {
            (Ok(()), cleanup) => cleanup,
            (Err(err), cleanup) => {
                if let Err(cleanup_error) = cleanup {
                    tracing::warn!(error = %cleanup_error, "Failed to release bootstrap claim after initialization failed");
                }
                Err(err)
            }
        }
    }

    async fn run_attempt(&self) -> Result<bool> {
        self.validate_container().await?;
        self.ensure_record(&self.record("definition")).await?;
        if self.ready().await? {
            return Ok(true);
        }
        let Some(claim) = self.try_claim().await? else {
            return Ok(false);
        };
        self.initialize_and_release(&claim, Box::pin(self.initialize(&claim)))
            .await?;
        Ok(true)
    }
}

async fn retry_bootstrap<F, Fut>(
    mut attempt: F,
    deadline: Instant,
    retry_delay: Duration,
) -> Result<()>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<bool>>,
{
    bounded(
        async {
            loop {
                if Instant::now() >= deadline {
                    return Err(error(
                        StatusCode::RequestTimeout,
                        "Bootstrap deadline exceeded.",
                    ));
                }
                let result = catch_bootstrap_panic(async { attempt().await }).await;
                match result {
                    Ok(true) => return Ok(()),
                    Ok(false) => {}
                    Err(err) if retryable(&err) => {
                        tracing::warn!(error = %err, "Retrying change feed workload bootstrap");
                    }
                    Err(err) => return Err(err),
                }
                azure_core::sleep(azure_core::time::Duration::try_from(retry_delay).map_err(
                    |_| {
                        error(
                            StatusCode::BadRequest,
                            "Bootstrap retry delay is not representable.",
                        )
                    },
                )?)
                .await;
            }
        },
        deadline,
    )
    .await
}

async fn catch_bootstrap_panic<T>(future: impl Future<Output = Result<T>>) -> Result<T> {
    match AssertUnwindSafe(future).catch_unwind().await {
        Ok(result) => result,
        Err(_) => {
            tracing::error!("Change feed bootstrap attempt panicked");
            Err(CosmosError::builder()
                .with_status(CosmosStatus::new(StatusCode::InternalServerError))
                .with_message("Change feed bootstrap attempt panicked.")
                .with_source(BootstrapPanic)
                .build())
        }
    }
}

/// Error-source marker that lets retry classification recognize a caught bootstrap panic.
#[derive(Debug)]
struct BootstrapPanic;

impl fmt::Display for BootstrapPanic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bootstrap attempt unwound")
    }
}

impl StdError for BootstrapPanic {}

pub(super) fn namespace(identity: &WorkloadIdentity) -> Result<String> {
    let key = serde_json::to_vec(&(
        &identity.account,
        &identity.container_rid,
        &identity.workload_name,
    ))?;
    Ok(format!("cfp1.{}", URL_SAFE_NO_PAD.encode(key)))
}

pub(super) fn lease_id(namespace: &str, range: &FeedRange) -> String {
    format!(
        "{namespace}.lease.{}-{}",
        range.min_inclusive(),
        range.max_exclusive()
    )
}

pub(super) fn validate_lease(
    lease: &LeaseDocument,
    identity: &WorkloadIdentity,
    namespace: &str,
) -> Result<()> {
    if lease.record.workload != *identity
        || lease.record.ttl != -1
        || lease.record.id != lease_id(namespace, &lease.range)
    {
        return Err(error(
            StatusCode::Conflict,
            "Incompatible processing lease.",
        ));
    }
    Ok(())
}

pub(super) fn validate_ranges(ranges: &[FeedRange]) -> Result<()> {
    let mut next = &EffectivePartitionKey::MIN;
    for range in ranges {
        if range.min_inclusive() != next || range.min_inclusive() >= range.max_exclusive() {
            return Err(error(
                StatusCode::Conflict,
                "Bootstrap ranges must cover the source without gaps or overlaps.",
            ));
        }
        next = range.max_exclusive();
    }
    if next != &EffectivePartitionKey::MAX {
        return Err(error(
            StatusCode::Conflict,
            "Bootstrap ranges must cover the entire source.",
        ));
    }
    Ok(())
}

async fn bounded<T>(future: impl Future<Output = Result<T>>, deadline: Instant) -> Result<T> {
    let remaining = deadline
        .checked_duration_since(Instant::now())
        .filter(|remaining| !remaining.is_zero())
        .ok_or_else(|| error(StatusCode::RequestTimeout, "Bootstrap deadline exceeded."))?;
    let timer = azure_core::sleep(azure_core::time::Duration::try_from(remaining).map_err(
        |_| {
            error(
                StatusCode::BadRequest,
                "Bootstrap timeout is not representable.",
            )
        },
    )?);
    pin_mut!(future);
    pin_mut!(timer);
    match futures::future::select(future, timer).await {
        Either::Left((result, _)) => result,
        Either::Right(_) => Err(error(
            StatusCode::RequestTimeout,
            "Bootstrap deadline exceeded.",
        )),
    }
}

pub(super) fn error(status: StatusCode, message: impl Into<Cow<'static, str>>) -> CosmosError {
    CosmosError::builder()
        .with_status(CosmosStatus::new(status))
        .with_message(message)
        .build()
}

fn retryable(error: &CosmosError) -> bool {
    error.status().is_transient()
        || error.status().is_precondition_failed()
        || error
            .source()
            .is_some_and(|source| source.is::<BootstrapPanic>())
}

#[cfg(all(test, feature = "__internal_in_memory_emulator"))]
mod tests;

#[cfg(all(test, feature = "__internal_in_memory_emulator", feature = "reqwest"))]
mod process_tests;
