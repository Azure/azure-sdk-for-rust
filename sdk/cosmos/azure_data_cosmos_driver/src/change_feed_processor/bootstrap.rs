// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub(super) mod store;

use crate::{
    change_feed_processor::{BootstrapOptions, ChangeFeedProcessorMode},
    models::{
        effective_partition_key::EffectivePartitionKey, ContainerReference, ContinuationToken,
        CosmosOperation, FeedRange,
    },
    CosmosDriver, CosmosError, CosmosStatus, Result,
};
use azure_core::{
    fmt::SafeDebug,
    http::{Etag, StatusCode},
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use futures::{future::Either, pin_mut};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    future::Future,
    time::{Duration, Instant},
};
use store::Store;
use uuid::Uuid;

pub(super) const SCHEMA_VERSION: u32 = 1;
const LOCK_TTL_SECONDS: i32 = 30;
const ATTEMPT_TIMEOUT: Duration = Duration::from_secs(25);

/// Initializes a workload's metadata and unowned processing leases.
///
/// The namespace combines the monitored account endpoint's origin, container RID,
/// and processor name within the supplied lease container. Names are not trimmed.
/// Source and lease containers may belong to different accounts.
///
/// The lease container must use `/id` partitioning and a default TTL of `-1`.
/// Only the initialization lock has an expiring TTL. Multi-write-region lease
/// accounts are rejected because this protocol cannot arbitrate concurrent
/// writes accepted independently by multiple write regions.
///
/// A durable definition binds the mode before initialization. A create-only range
/// plan lets interrupted attempts finish the same initial coverage. New leases
/// have neither an owner nor a continuation; the reader applies its configured
/// start options when no checkpoint exists. Compatible completed workloads take
/// a read-only fast path unless verification is enabled.
///
/// Claims expire after 30 seconds; each claimed attempt is bounded to 25 seconds.
/// TTL election is not strict fencing: delayed attempts can overlap after expiry.
/// All attempts use the same persisted plan, create missing leases only, and
/// validate conflicts without replacing existing progress.
///
/// Dropping the returned future stops local work but cannot undo requests already
/// sent. An abandoned claim is left for TTL expiration. Transient failures are
/// retried within the total budget; other failures are returned.
///
/// # Errors
///
/// Returns an error for invalid names, timeouts, container configuration, malformed
/// persisted records, incompatible modes or ranges, or failed service operations.
/// A mode mismatch is rejected even if initialization has already completed.
///
/// # Examples
///
/// ```no_run
/// use azure_data_cosmos_driver::{
///     change_feed_processor::{bootstrap, BootstrapOptions, ChangeFeedProcessorMode},
///     models::ContainerReference,
///     CosmosDriver,
/// };
///
/// async fn initialize(
///     source_driver: &CosmosDriver,
///     source: &ContainerReference,
///     lease_driver: &CosmosDriver,
///     leases: &ContainerReference,
/// ) -> azure_data_cosmos_driver::Result<()> {
///     let options = BootstrapOptions::new(
///         ChangeFeedProcessorMode::LatestVersion, "order-projection", "worker-01",
///     );
///     bootstrap(source_driver, source, lease_driver, leases, &options).await
/// }
/// ```
pub async fn bootstrap(
    source_driver: &CosmosDriver,
    source: &ContainerReference,
    lease_driver: &CosmosDriver,
    leases: &ContainerReference,
    options: &BootstrapOptions,
) -> Result<()> {
    for (name, value) in [
        ("processor_name", options.processor_name.as_str()),
        ("instance_name", options.instance_name.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(error(
                StatusCode::BadRequest,
                format!("{name} must not be blank."),
            ));
        }
    }
    let deadline = Instant::now()
        .checked_add(options.timeout)
        .filter(|_| !options.timeout.is_zero())
        .ok_or_else(|| {
            error(
                StatusCode::BadRequest,
                "Bootstrap timeout must be positive and representable.",
            )
        })?;
    if source.account() != source_driver.account() || leases.account() != lease_driver.account() {
        return Err(error(
            StatusCode::BadRequest,
            "Container and driver accounts must match.",
        ));
    }
    if lease_driver.has_multiple_write_locations() {
        return Err(error(
            StatusCode::BadRequest,
            "Bootstrap requires a single-write-region lease account.",
        ));
    }
    let identity = Identity {
        version: SCHEMA_VERSION,
        account: source.account().endpoint().origin().ascii_serialization(),
        container_rid: source.rid().to_owned(),
        processor_name: options.processor_name.clone(),
        mode: options.mode,
    };
    let namespace = namespace(&identity)?;
    let context = BootstrapContext {
        source_driver,
        source,
        store: Store {
            driver: lease_driver,
            container: leases,
            deadline,
        },
        identity,
        namespace,
        options,
    };
    bounded(Box::pin(context.run()), deadline).await
}

#[derive(Clone, SafeDebug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Identity {
    pub(super) version: u32,
    pub(super) account: String,
    pub(super) container_rid: String,
    pub(super) processor_name: String,
    pub(super) mode: ChangeFeedProcessorMode,
}

#[derive(Clone, SafeDebug, Serialize, Deserialize)]
pub(super) struct Record {
    pub(super) id: String,
    pub(super) workload: Identity,
    pub(super) ttl: i32,
}

#[derive(Clone, SafeDebug, Serialize, Deserialize)]
pub(super) struct RangePlan {
    #[serde(flatten)]
    pub(super) record: Record,
    pub(super) ranges: Vec<FeedRange>,
}

#[derive(Clone, SafeDebug, Serialize, Deserialize)]
pub(super) struct ProcessingLease {
    #[serde(flatten)]
    pub(super) record: Record,
    pub(super) range: FeedRange,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) continuation: Option<ContinuationToken>,
}

#[derive(SafeDebug, Serialize, Deserialize)]
struct LockRecord {
    #[serde(flatten)]
    record: Record,
    attempt: String,
    owner: String,
}

struct Claim {
    etag: Etag,
    deadline: Instant,
}

struct BootstrapContext<'a> {
    source_driver: &'a CosmosDriver,
    source: &'a ContainerReference,
    store: Store<'a>,
    identity: Identity,
    namespace: String,
    options: &'a BootstrapOptions,
}

impl BootstrapContext<'_> {
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
            return self.validate(&existing.value, record);
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
        self.validate(&existing.value, record)
    }

    async fn ready(&self) -> Result<bool> {
        let expected = self.record("info");
        match self.store.read::<Record>(&expected.id).await? {
            Some(existing) => {
                self.validate(&existing.value, &expected)?;
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

    async fn try_claim(&self) -> Result<Option<Claim>> {
        let started = Instant::now();
        let mut record = self.record("lock");
        record.ttl = LOCK_TTL_SECONDS;
        let lock = LockRecord {
            record,
            attempt: Uuid::new_v4().to_string(),
            owner: self.options.instance_name.clone(),
        };
        self.store.create(&lock.record.id, &lock).await?;
        let Some(current) = self.store.read::<LockRecord>(&lock.record.id).await? else {
            return Ok(None);
        };
        self.validate(&current.value.record, &lock.record)?;
        if current.value.attempt != lock.attempt {
            return Ok(None);
        }
        Ok(Some(Claim {
            etag: current.etag,
            deadline: (started + ATTEMPT_TIMEOUT).min(self.store.deadline),
        }))
    }

    async fn check_claim(&self, claim: &Claim) -> Result<()> {
        if Instant::now() >= claim.deadline {
            return Err(error(
                StatusCode::RequestTimeout,
                "Bootstrap attempt deadline exceeded.",
            ));
        }
        let current = self
            .store
            .read::<LockRecord>(&self.record("lock").id)
            .await?;
        if current
            .as_ref()
            .is_none_or(|record| record.etag != claim.etag)
        {
            return Err(error(
                StatusCode::PreconditionFailed,
                "Bootstrap initialization claim was lost.",
            ));
        }
        Ok(())
    }

    async fn plan(&self, claim: &Claim) -> Result<RangePlan> {
        let record = self.record("plan");
        if let Some(existing) = self.store.read::<RangePlan>(&record.id).await? {
            self.validate(&existing.value.record, &record)?;
            validate_ranges(&existing.value.ranges)?;
            return Ok(existing.value);
        }
        if self.ready().await? {
            return Err(error(
                StatusCode::Conflict,
                "Completed workload is missing its original bootstrap range plan.",
            ));
        }
        let topology = self
            .source_driver
            .resolve_all_partition_key_ranges(self.source, true)
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
            return Ok(plan);
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
        self.validate(&existing.value.record, &plan.record)?;
        validate_ranges(&existing.value.ranges)?;
        Ok(existing.value)
    }

    async fn create_leases(&self, plan: &RangePlan, claim: &Claim) -> Result<()> {
        for range in &plan.ranges {
            let expected = ProcessingLease {
                record: self.record(&format!(
                    "lease.{}-{}",
                    range.min_inclusive(),
                    range.max_exclusive()
                )),
                range: range.clone(),
                owner: None,
                continuation: None,
            };
            self.check_claim(claim).await?;
            let existing = match self
                .store
                .read::<ProcessingLease>(&expected.record.id)
                .await?
            {
                Some(existing) => existing,
                None => {
                    if self.store.create(&expected.record.id, &expected).await? {
                        continue;
                    }
                    self.store
                        .read::<ProcessingLease>(&expected.record.id)
                        .await?
                        .ok_or_else(|| {
                            error(
                                StatusCode::ServiceUnavailable,
                                "Conflicting processing lease is not yet readable.",
                            )
                        })?
                }
            };
            self.validate(&existing.value.record, &expected.record)?;
            if existing.value.range != expected.range {
                return Err(error(
                    StatusCode::Conflict,
                    "Existing processing lease has an incompatible range.",
                ));
            }
        }
        Ok(())
    }

    async fn initialize(&self, claim: &Claim) -> Result<()> {
        if self.ready().await? && !self.options.verify_leases_on_restart {
            return Ok(());
        }
        let plan = self.plan(claim).await?;
        self.create_leases(&plan, claim).await?;
        self.check_claim(claim).await?;
        self.ensure_record(&self.record("info")).await
    }

    async fn run(&self) -> Result<()> {
        self.validate_container().await?;
        loop {
            let result = async {
                self.ensure_record(&self.record("definition")).await?;
                if self.ready().await? && !self.options.verify_leases_on_restart {
                    return Ok(true);
                }
                let Some(claim) = self.try_claim().await? else {
                    return Ok(false);
                };
                let result = bounded(self.initialize(&claim), claim.deadline).await;
                let release = self.store.release(&self.record("lock").id, claim.etag).await;
                match (result, release) {
                    (Ok(()), Ok(())) => Ok(true),
                    (Ok(()), Err(err)) => Err(err),
                    (Err(err), cleanup) => {
                        if let Err(cleanup_error) = cleanup {
                            tracing::warn!(error = %cleanup_error, "Failed to release bootstrap claim after initialization failed");
                        }
                        Err(err)
                    }
                }
            }.await;
            match result {
                Ok(true) => return Ok(()),
                Ok(false) => {}
                Err(err) if retryable(&err) => {
                    tracing::warn!(error = %err, "Retrying change feed workload bootstrap");
                }
                Err(err) => return Err(err),
            }
            azure_core::sleep(
                azure_core::time::Duration::try_from(self.options.retry_delay).map_err(|_| {
                    error(
                        StatusCode::BadRequest,
                        "Bootstrap retry delay is not representable.",
                    )
                })?,
            )
            .await;
        }
    }
}

pub(super) fn namespace(identity: &Identity) -> Result<String> {
    let key = serde_json::to_vec(&(
        &identity.account,
        &identity.container_rid,
        &identity.processor_name,
    ))?;
    Ok(format!("cfp1.{}", URL_SAFE_NO_PAD.encode(key)))
}

fn validate_ranges(ranges: &[FeedRange]) -> Result<()> {
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
    error.status().is_transient() || error.status().is_precondition_failed()
}

#[cfg(all(test, feature = "__internal_in_memory_emulator"))]
mod tests;

#[cfg(all(test, feature = "__internal_in_memory_emulator", feature = "reqwest"))]
mod process_tests;
