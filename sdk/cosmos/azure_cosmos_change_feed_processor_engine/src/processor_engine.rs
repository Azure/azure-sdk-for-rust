// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{future::Future, num::NonZeroU32, sync::Arc};

use azure_data_cosmos_driver::{
    models::{
        AccountReference, ChangeFeedStartFrom, ContainerReference, ContinuationToken, FeedRange,
        MaxItemCountHint,
    },
    options::{DriverOptions, OperationOptions},
    CosmosDriver, CosmosDriverRuntime, Result,
};

use crate::{
    run_lease, CheckpointStore, LeaseControl, LeaseRunError, LeaseRunOptions, LeaseRunReport,
    OwnedLease,
};
use crate::{ChangeFeedReadOptions, ChangeFeedReader, RawChangeFeedPage};

/// Owns the connection used for schema-agnostic change-feed page reads.
///
/// This is a data-plane building block, not yet a lease-owning background processor.
pub struct ProcessorEngine {
    driver: Arc<CosmosDriver>,
    container: ContainerReference,
}

impl ProcessorEngine {
    /// Validates a bootstrap plan's saved positions against this source driver.
    ///
    /// This does not fetch or choose fresh starting positions. For Now, callers
    /// must already have captured concrete post-poll tokens for every seed.
    ///
    /// # Errors
    ///
    /// Returns incomplete-plan or driver source/mode/scope continuation errors.
    pub async fn bootstrap_plan(
        &self,
        group: impl Into<String>,
        mode: crate::ChangeFeedMode,
        start: crate::BootstrapStartPolicy,
        expected: FeedRange,
        seeds: Vec<crate::InitialLease>,
    ) -> Result<crate::BootstrapPlan> {
        let source = self.source_identity();
        let plan = crate::BootstrapPlan::new(source, group, mode, start.clone(), expected, seeds)?;
        for seed in plan.initial_leases() {
            self.open_reader(
                ChangeFeedReadOptions::new(seed.range().clone(), start.clone())
                    .with_mode(mode)
                    .with_continuation(ContinuationToken::from_string(
                        seed.checkpoint().to_owned(),
                    )),
            )
            .await?;
        }
        Ok(plan)
    }

    /// Processes an acquired Cosmos lease while maintaining its authority.
    ///
    /// Renewal is polled independently of the handler. Checkpoint/renewal/release
    /// share the session's authoritative revision. A separate monotonic watchdog
    /// revokes authority even when lease I/O is blocked. No task is detached.
    ///
    /// The result separates processing failure, maintenance failure, and release.
    /// Dropping this future abandons local processing; an in-flight lease write
    /// invalidates the session and may need store reconciliation.
    pub async fn run_with_lease_session<H, F>(
        &self,
        session: &crate::LeaseSession,
        read: ChangeFeedReadOptions,
        options: &LeaseRunOptions,
        handler: H,
    ) -> crate::LeaseOwnershipRun
    where
        H: FnMut(RawChangeFeedPage) -> F,
        F: Future<Output = Result<()>>,
    {
        let _run_guard = match session.begin_run() {
            Ok(guard) => guard,
            Err(error) => {
                return crate::LeaseOwnershipRun::new(
                    Err(LeaseRunError::before_read(session.lease().await, error)),
                    None,
                    crate::LeaseReleaseOutcome::NotAttempted,
                )
            }
        };
        let lease = session.lease().await;
        let mut store = session.clone();
        let mut processing = Box::pin(self.run_owned_lease(
            lease,
            read,
            session.control(),
            options,
            &mut store,
            handler,
        ));
        let (mut result, maintenance_error) = {
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
                error = &mut maintenance => {
                    session.control().lose_ownership();
                    (processing.as_mut().await, error)
                },
                result = &mut processing => (result, None),
            }
        };
        drop(processing);
        let authoritative = session.lease().await;
        match &mut result {
            Ok(report) => report.update_authoritative_lease(authoritative),
            Err(error) => error.update_authoritative_lease(authoritative),
        }
        let release = if session.control().is_lost() {
            crate::LeaseReleaseOutcome::AuthorityLost
        } else {
            match session.release().await {
                Ok(()) => crate::LeaseReleaseOutcome::Released,
                Err(error) => crate::LeaseReleaseOutcome::Failed(error),
            }
        };
        crate::LeaseOwnershipRun::new(result, maintenance_error, release)
    }

    /// Refreshes physical topology and preserves existing logical lease assignments.
    ///
    /// Call at startup and at a bounded periodic cadence; physical splits can be
    /// repaired inside the driver without a 410 reaching the coordinator.
    /// This method plans only: it does not create, retire, or delete leases.
    ///
    /// # Errors
    ///
    /// Returns topology-resolution or lease-coverage validation errors.
    pub async fn reconcile_lease_topology(
        &self,
        leases: &[OwnedLease],
    ) -> Result<Vec<crate::LeaseTopologyAssignment>> {
        let partitions = self.driver.resolve_all_partition_key_ranges(&self.container, true)
            .await?.ok_or_else(|| azure_data_cosmos_driver::CosmosError::builder()
                .with_status(azure_data_cosmos_driver::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("cannot reconcile leases without a refreshed routing map").build())?;
        let ranges = partitions
            .iter()
            .map(FeedRange::try_from)
            .collect::<Result<Vec<_>>>()?;
        crate::plan_lease_topology(leases, &ranges)
    }

    /// Connects to an existing account and resolves the monitored container.
    ///
    /// # Errors
    ///
    /// Returns authentication, transport, or metadata-resolution errors.
    pub async fn connect(
        account: AccountReference,
        database: &str,
        container: &str,
    ) -> Result<Self> {
        let runtime = CosmosDriverRuntime::builder()
            .with_wrapping_sdk_identifier(concat!(
                "azsdk-rust-cosmos-change-feed-processor/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .await?;
        let driver = runtime
            .create_driver(DriverOptions::builder(account).build())
            .await?;
        Self::new(driver, database, container).await
    }

    /// Resolves the monitored container using an existing initialized driver.
    ///
    /// # Errors
    ///
    /// Returns authentication, transport, or container-resolution errors.
    pub async fn new(driver: Arc<CosmosDriver>, database: &str, container: &str) -> Result<Self> {
        let container = driver
            .resolve_container(database, container, OperationOptions::default())
            .await?;
        Self::from_resolved(driver, container)
    }

    /// Retains a prepared driver and container without resolving the name or performing I/O.
    ///
    /// Later operations may refresh driver-managed metadata. Saved checkpoints
    /// remain bound to the original source identity, not a replacement container.
    ///
    /// # Errors
    ///
    /// Rejects references bound to a different account, key, or token provider.
    pub fn from_resolved(driver: Arc<CosmosDriver>, container: ContainerReference) -> Result<Self> {
        crate::prepared_container::validate_binding(&driver, &container)?;
        Ok(Self { driver, container })
    }

    /// Returns the workload source identity derived from the account endpoint and container RID.
    pub fn source_identity(&self) -> String {
        format!(
            "{}|{}",
            self.driver.account().endpoint(),
            self.container.rid()
        )
    }

    /// Reads one latest-version change-feed page across the monitored container.
    ///
    /// A missing continuation starts at the beginning. An idle partition returns
    /// an empty page, not end-of-feed. The page size is a service hint.
    /// Each call reconstructs its plan from the supplied continuation; no
    /// progress is retained in this engine.
    ///
    /// # Errors
    ///
    /// Returns invalid-token, fan-out-limit, transport, or service errors.
    pub async fn read_page(
        &self,
        max_item_count: NonZeroU32,
        continuation: Option<&ContinuationToken>,
    ) -> Result<RawChangeFeedPage> {
        let mut options =
            ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning)
                .with_max_item_count(MaxItemCountHint::Limit(max_item_count));
        if let Some(token) = continuation {
            options = options.with_continuation(token.clone());
        }
        self.open_reader(options).await?.read_page().await
    }

    /// Creates a retained, range-scoped driver reader.
    ///
    /// # Errors
    ///
    /// Returns invalid continuation, mode/scope mismatch, admission, or metadata errors.
    pub async fn open_reader(&self, options: ChangeFeedReadOptions) -> Result<ChangeFeedReader> {
        ChangeFeedReader::open(self.driver.clone(), self.container.clone(), options).await
    }

    /// Runs a bounded single-lease processing path from confirmed durable progress.
    ///
    /// The lease range must match the reader range; its checkpoint overrides any
    /// continuation in the read options. No acquisition, renewal, or release occurs.
    ///
    /// # Errors
    ///
    /// Returns planning, processing, or persistence failures with recovery state.
    pub async fn run_owned_lease<C, H, F>(
        &self,
        lease: OwnedLease,
        read: ChangeFeedReadOptions,
        control: &LeaseControl,
        options: &LeaseRunOptions,
        store: &mut C,
        handler: H,
    ) -> std::result::Result<LeaseRunReport, LeaseRunError>
    where
        C: CheckpointStore,
        H: FnMut(RawChangeFeedPage) -> F,
        F: Future<Output = Result<()>>,
    {
        if read.range() != lease.range() {
            let error = azure_data_cosmos_driver::CosmosError::builder()
                .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
                .with_message("reader source range does not match owned lease")
                .build();
            return Err(LeaseRunError::before_read(lease, error));
        }
        let read = read.with_continuation(lease.checkpoint().clone());
        let began = tokio::time::Instant::now();
        let Some(deadline) = began.checked_add(options.max_duration()) else {
            let error = azure_data_cosmos_driver::CosmosError::builder()
                .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
                .with_message("run duration exceeds the timer range")
                .build();
            return Err(LeaseRunError::before_read(lease, error));
        };
        if tokio::time::Instant::now() >= deadline {
            return Ok(LeaseRunReport::before_read(
                lease,
                crate::LeaseRunOutcome::RunTimedOut,
            ));
        }
        let topology_lease = lease.clone();
        let mut reader = tokio::select! {
            biased;
            outcome = control.interrupted() => return Ok(LeaseRunReport::before_read(lease, outcome)),
            _ = tokio::time::sleep_until(deadline) => return Ok(LeaseRunReport::before_read(
                lease, crate::LeaseRunOutcome::RunTimedOut)),
            result = async {
                self.reconcile_lease_topology(std::slice::from_ref(&topology_lease)).await?;
                self.open_reader(read).await
            } => result
                .map_err(|error| LeaseRunError::before_read(lease.clone(), error))?,
        };
        let options = options
            .clone()
            .with_max_duration(options.max_duration().saturating_sub(began.elapsed()));
        run_lease(&mut reader, store, lease, control, &options, handler).await
    }
}
