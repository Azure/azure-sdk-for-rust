// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod balancing;
mod distribution;
#[cfg(all(test, feature = "__internal_in_memory_emulator"))]
mod tests;
mod topology;

use crate::{
    change_feed_processor::{
        bootstrap,
        bootstrap::{
            error, namespace,
            store::{
                ChangeFeedProcessorBootstrapStore as Store,
                ChangeFeedProcessorBootstrapStoredRecord as Stored,
            },
            ChangeFeedProcessorBootstrapIdentity as Identity,
            ChangeFeedProcessorBootstrapProcessingLease as ProcessingLease, SCHEMA_VERSION,
        },
        processing::distribution::{BatchDistributor, LeaseSession},
    },
    models::{
        ChangeFeedFactoryMode, ChangeFeedStartFrom, ContainerReference, ContinuationToken,
        CosmosResponse, FeedRange, MaxItemCountHint,
    },
    options::{OperationOptions, PlanOptions},
    CosmosDriver, Result,
};
use async_lock::Mutex;
use azure_core::{fmt::SafeDebug, http::StatusCode};
use futures::{
    channel::oneshot,
    future::{BoxFuture, Shared},
    stream::FuturesUnordered,
    FutureExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex as StdMutex},
    time::{Duration, Instant},
};
use time::OffsetDateTime;
use uuid::Uuid;

type Signal = Shared<BoxFuture<'static, ()>>;

/// Configuration for driver-owned lease coordination and feed polling.
///
/// Each acquired lease permits one outstanding batch. The consumer must
/// acknowledge it after processing, or drop its receipt to abandon the lease.
#[derive(Clone, SafeDebug)]
pub struct ProcessorOptions {
    mode: ChangeFeedFactoryMode,
    processor_name: String,
    instance_name: String,
    verify_leases_on_restart: bool,
    start: ChangeFeedStartFrom,
    continuation: Option<ContinuationToken>,
    page_size: MaxItemCountHint,
    poll_delay: Duration,
    min_leases: u32,
    max_leases: u32,
    acquire_interval: Duration,
    renew_interval: Duration,
    expiration: Duration,
}

impl ProcessorOptions {
    /// Creates options with Java-compatible lease timing defaults.
    ///
    /// Acquisition runs every 13 seconds, renewal every 17 seconds, and leases
    /// expire after 60 seconds. The progress watchdog is 25 renewal intervals.
    /// Fresh feeds start from now, request 100 items, and wait five seconds
    /// after idle responses. Lease counts are uncapped.
    ///
    /// Workers sharing the source, lease storage, and `processor_name` cooperate.
    /// `instance_name` identifies the worker. Names are validated during bootstrap.
    pub fn new(
        mode: ChangeFeedFactoryMode,
        processor_name: impl Into<String>,
        instance_name: impl Into<String>,
    ) -> Self {
        Self {
            mode,
            processor_name: processor_name.into(),
            instance_name: instance_name.into(),
            verify_leases_on_restart: false,
            start: ChangeFeedStartFrom::Now,
            continuation: None,
            page_size: MaxItemCountHint::Limit(std::num::NonZeroU32::new(100).unwrap()),
            poll_delay: Duration::from_secs(5),
            min_leases: 0,
            max_leases: 0,
            acquire_interval: Duration::from_secs(13),
            renew_interval: Duration::from_secs(17),
            expiration: Duration::from_secs(60),
        }
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

    /// Sets the delay after an idle feed response. Zero is allowed.
    pub fn with_poll_delay(mut self, delay: Duration) -> Self {
        self.poll_delay = delay;
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
    /// This does not discover topology changes or modify bootstrap completion.
    pub fn with_lease_verification_enabled_on_restart(mut self, enabled: bool) -> Self {
        self.verify_leases_on_restart = enabled;
        self
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Lease {
    #[serde(flatten)]
    base: ProcessingLease,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    incarnation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    renewed_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    children: Option<Vec<FeedRange>>,
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

    fn validate(&self, lease: &Lease) -> Result<()> {
        bootstrap::validate_lease(&lease.base, &self.identity, &self.namespace)?;
        if let Some(children) = &lease.children {
            if lease.base.owner.is_some()
                || lease.incarnation.is_some()
                || lease.base.continuation.is_none()
            {
                return Err(error(
                    StatusCode::Conflict,
                    "Invalid retired processing lease.",
                ));
            }
            topology::validate_children(&lease.base.range, children)?;
        }
        Ok(())
    }

    async fn acquire(&self, observed: &Lease) -> Result<Stored<Lease>> {
        self.modify_lease(observed, Update::Acquire).await
    }

    async fn update(&self, lease: &mut Stored<Lease>, update: Update<'_>) -> Result<()> {
        *lease = self.modify_lease(&lease.value, update).await?;
        Ok(())
    }

    /// Applies a mutation to fresh state while preserving the observed acquisition.
    /// Acquisition races fail immediately; owned mutations retry only write conflicts.
    async fn modify_lease(&self, observed: &Lease, update: Update<'_>) -> Result<Stored<Lease>> {
        let id = &observed.base.record.id;
        let retry_conflicts = !matches!(update, Update::Acquire);
        let attempts = if retry_conflicts { 5 } else { 1 };
        for _ in 0..attempts {
            let mut current = self.store().read::<Lease>(id).await?.ok_or_else(lost)?;
            self.validate(&current.value)?;
            if current.value.children.is_some()
                || current.value.base.owner != observed.base.owner
                || current.value.incarnation != observed.incarnation
            {
                return Err(lost());
            }
            match update {
                Update::Acquire => {
                    current.value.base.owner = Some(self.options.instance_name.clone());
                    current.value.incarnation = Some(Uuid::new_v4().to_string());
                }
                Update::Renew => {}
                Update::Checkpoint(token) => current.value.base.continuation = Some(token.clone()),
                Update::Release => current.value.base.owner = None,
                Update::Split(children) => {
                    topology::validate_children(&current.value.base.range, children)?;
                    let checkpoint = self.checkpoint(&current.value, &current.value.base.range)?;
                    current.value.base.continuation = Some(checkpoint);
                    current.value.base.owner = None;
                    current.value.incarnation = None;
                    current.value.children = Some(children.to_vec());
                }
            }
            current.value.renewed_at = Some(now_millis());
            match self.store().replace(id, &current.value, current.etag).await {
                Ok(updated) => return Ok(updated),
                Err(err) if retry_conflicts && err.status().is_precondition_failed() => continue,
                Err(err) => return Err(err),
            }
        }
        Err(error(
            StatusCode::PreconditionFailed,
            "Lease update conflicts exhausted the retry budget.",
        ))
    }
}

enum Update<'a> {
    Acquire,
    Renew,
    Checkpoint(&'a ContinuationToken),
    Release,
    Split(&'a [FeedRange]),
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
}

/// A delivery receipt tied to one batch and one acquisition of its lease.
///
/// Dropping this receipt without saving a checkpoint abandons the lease without
/// advancing its durable position. It cannot authorize arbitrary continuations.
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
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(Acknowledgment { result: tx })
            .map_err(|_| lost())?;
        rx.await.map_err(|_| lost())?
    }

    /// Waits until this lease session stops, including ownership loss or timeout.
    ///
    /// Consumers can race their callback against this signal. Cancellation
    /// cannot undo application side effects already performed.
    pub async fn cancelled(&self) {
        self.cancelled.clone().await;
    }
}

/// An owned change feed response and its processing receipt.
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

struct Owner {
    stop: StdMutex<Option<oneshot::Sender<()>>>,
    stopped: Signal,
    distributor: Arc<BatchDistributor>,
}

/// A running, schema-agnostic change feed processor.
///
/// Clones share a lifecycle. Dropping the last handle requests shutdown.
/// Lease renewal and balancing continue while consumers process batches.
///
/// Splits retire the parent lease and create independently balanced child
/// leases from acknowledged progress. Merges retain separate EPK-slice leases
/// and their checkpoints. Retired parent records must not be deleted.
///
/// Splits and merges are recovered automatically while processor instances
/// remain running. Only the affected lease's processing session is replaced;
/// no application stop or restart is required.
#[derive(Clone)]
pub struct Processor {
    owner: Arc<Owner>,
}

impl Processor {
    /// Bootstraps the workload and starts lease coordination.
    ///
    /// Source and lease containers may use different drivers. Only one caller
    /// should await [`next_batch()`](Self::next_batch) at a time. Background
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
    ///     if let Some(batch) = processor.next_batch().await {
    ///         let (_lease, response, receipt) = batch.into_parts();
    ///         process(response).await?;
    ///         receipt.checkpoint().await?;
    ///     }
    ///     processor.stop().await;
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
        bootstrap(
            &source_driver,
            &source,
            &lease_driver,
            &leases,
            options.mode,
            &options.processor_name,
            &options.instance_name,
        )
        .await?;
        let identity = Identity {
            version: SCHEMA_VERSION,
            account: source.account().endpoint().origin().ascii_serialization(),
            container_rid: source.rid().to_owned(),
            processor_name: options.processor_name.clone(),
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
        if context.options.verify_leases_on_restart {
            context.read_leases(true).await?;
        }
        let distributor = Arc::new(BatchDistributor::default());
        let (stop, stopping) = signal();
        let (done, stopped) = signal();
        let owner = Arc::new(Owner {
            stop: StdMutex::new(Some(stop)),
            stopped,
            distributor: distributor.clone(),
        });
        // The task holds the distributor, not Owner, so the last handle closes stop.
        let task = azure_core::async_runtime::get_async_runtime().spawn(Box::pin(async move {
            run(context, distributor.clone(), stopping).await;
            distributor.close();
            let _ = done.send(());
        }));
        drop(task);
        Ok(Self { owner })
    }

    /// Reports whether background coordination is running.
    pub fn is_running(&self) -> bool {
        self.owner.distributor.is_running()
    }

    /// Waits for an owned batch, or returns `None` after shutdown.
    ///
    /// Canceling this wait does not consume a batch. Progress from 304 responses
    /// is saved internally without invoking application callbacks.
    /// Ready lease sessions take turns supplying batches; an empty set of owned
    /// leases waits for future acquisitions rather than ending the stream.
    pub async fn next_batch(&self) -> Option<ChangeFeedBatch> {
        self.owner.distributor.next_batch().await
    }

    /// Requests shutdown and waits for lease tasks and release attempts.
    ///
    /// In-flight callbacks belong to the consumer. Their receipts are canceled
    /// and cannot checkpoint after ownership ends. Release failures are logged;
    /// unreleased leases remain recoverable through expiration.
    pub async fn stop(&self) {
        self.request_stop();
        self.owner.stopped.clone().await;
    }

    /// Requests shutdown without waiting, including from a destructor.
    ///
    /// Use [`stop()`](Self::stop) when the caller must wait for lease cleanup.
    pub fn request_stop(&self) {
        if let Some(stop) = self
            .owner
            .stop
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take()
        {
            let _ = stop.send(());
        }
    }
}

async fn run(context: Arc<Context>, distributor: Arc<BatchDistributor>, stop: Signal) {
    let mut workers = FuturesUnordered::<BoxFuture<'static, String>>::new();
    let mut owned = HashSet::new();
    let mut balancing = balance(context.clone(), owned.clone(), Duration::ZERO).boxed();
    loop {
        futures::select! {
            _ = stop.clone().fuse() => break,
            id = async {
                if workers.is_empty() {
                    futures::future::pending().await
                } else {
                    workers.next().await
                }
            }.fuse() => {
                if let Some(id) = id { owned.remove(&id); }
            },
            leases = balancing.as_mut().fuse() => {
                for lease in leases {
                    let id = lease.value.base.record.id.clone();
                    if owned.insert(id.clone()) {
                        let context = context.clone();
                        let distributor = distributor.clone();
                        let stop = stop.clone();
                        workers.push(async move {
                            supervise(context, distributor, lease, stop).await;
                            id
                        }.boxed());
                    }
                }
                balancing = balance(context.clone(), owned.clone(), context.options.acquire_interval).boxed();
            },
        }
    }
    // Every supervisor observes stop and conditionally releases its own lease.
    while workers.next().await.is_some() {}
}

async fn balance(
    context: Arc<Context>,
    owned: HashSet<String>,
    delay: Duration,
) -> Vec<Stored<Lease>> {
    sleep(delay).await;
    let result = async {
        let all = context.list().await?;
        let selected = balancing::select(&all, &context.options, now_millis());
        let mut acquired = Vec::new();
        for index in selected {
            if owned.contains(&all[index].base.record.id) {
                continue;
            }
            match context.acquire(&all[index]).await {
                Ok(lease) => acquired.push(lease),
                Err(err) if err.status().is_precondition_failed() => {
                    tracing::debug!(error = %err, "Lease acquisition lost a race");
                }
                Err(err) => tracing::warn!(error = %err, "Lease acquisition failed"),
            }
        }
        Ok::<_, crate::CosmosError>(acquired)
    }
    .await;
    match result {
        Ok(acquired) => acquired,
        Err(err) => {
            tracing::warn!(error = %err, "Lease balancing failed; retrying on the next cycle");
            Vec::new()
        }
    }
}

async fn supervise(
    context: Arc<Context>,
    distributor: Arc<BatchDistributor>,
    lease: Stored<Lease>,
    stop: Signal,
) {
    let token = lease.value.base.record.id.clone();
    let session = distributor.register(token.clone(), lease.value.incarnation.clone());
    let lease = Arc::new(Mutex::new(lease));
    let progress = Arc::new(StdMutex::new(Instant::now()));
    let (cancel, cancelled) = signal();
    let result = match session {
        Ok(session) => {
            let processing = poll(
                context.clone(),
                &session,
                lease.clone(),
                progress.clone(),
                cancelled,
            )
            .fuse();
            let renewing = renew(context.clone(), lease.clone()).fuse();
            let watchdog = watchdog(progress, context.options.renew_interval * 25).fuse();
            futures::pin_mut!(processing, renewing, watchdog);
            futures::select! {
                result = processing => result,
                result = renewing => result,
                _ = watchdog => Err(error(StatusCode::RequestTimeout, "Lease progress watchdog expired.")),
                _ = stop.fuse() => Ok(()),
            }
        }
        Err(err) => Err(err),
    };
    drop(cancel);
    if let Err(err) = result {
        tracing::warn!(lease = %token, error = %err, "Lease processing stopped");
    }
    let mut lease = lease.lock().await;
    if lease.value.children.is_some() {
        return;
    }
    if let Err(err) = context.update(&mut lease, Update::Release).await {
        tracing::warn!(lease = %token, error = %err, "Lease release failed; ownership will expire");
    }
}

async fn renew(context: Arc<Context>, lease: Arc<Mutex<Stored<Lease>>>) -> Result<()> {
    loop {
        sleep(context.options.renew_interval).await;
        let mut lease = lease.lock().await;
        match context.update(&mut lease, Update::Renew).await {
            Ok(()) => {}
            Err(err) if err.status().is_precondition_failed() || err.status().is_not_found() => {
                return Err(err)
            }
            Err(err) => {
                tracing::warn!(error = %err, "Lease renewal failed; retrying at the next interval")
            }
        }
    }
}

async fn watchdog(progress: Arc<StdMutex<Instant>>, timeout: Duration) {
    loop {
        let elapsed = progress.lock().unwrap_or_else(|e| e.into_inner()).elapsed();
        if elapsed >= timeout {
            return;
        }
        sleep(timeout - elapsed).await;
    }
}

async fn poll(
    context: Arc<Context>,
    session: &LeaseSession,
    lease: Arc<Mutex<Stored<Lease>>>,
    progress: Arc<StdMutex<Instant>>,
    cancelled: Signal,
) -> Result<()> {
    let result = poll_feed(context.clone(), session, lease.clone(), progress, cancelled).await;
    match result {
        Err(err) if err.status().is_partition_topology_change() => {
            if context.recover(lease).await? {
                Ok(())
            } else {
                // A merge or an unfinished transition re-enters through normal lease acquisition.
                Err(err)
            }
        }
        result => result,
    }
}

async fn poll_feed(
    context: Arc<Context>,
    session: &LeaseSession,
    lease: Arc<Mutex<Stored<Lease>>>,
    progress: Arc<StdMutex<Instant>>,
    cancelled: Signal,
) -> Result<()> {
    let initial = lease.lock().await.value.clone();
    let operation = context
        .operation(&initial.base.range)
        .with_max_item_count(context.options.page_size);
    let checkpoint = context.checkpoint(&initial, &initial.base.range)?;
    let plan_options = PlanOptions {
        change_feed_processor: true,
        ..Default::default()
    };
    let mut plan = context
        .source_driver
        .plan_operation(
            operation,
            &OperationOptions::default(),
            Some(&checkpoint),
            &plan_options,
        )
        .await?;
    loop {
        let response = context
            .source_driver
            .execute_plan(
                &mut plan,
                Some(context.source.clone()),
                OperationOptions::default(),
            )
            .await?
            .ok_or_else(|| {
                error(
                    StatusCode::InternalServerError,
                    "Change feed unexpectedly drained.",
                )
            })?;
        *progress.lock().unwrap_or_else(|e| e.into_inner()) = Instant::now();
        if response.headers().etag.is_none() {
            return Err(error(
                StatusCode::InternalServerError,
                "Change feed response has no checkpoint ETag.",
            ));
        }
        let idle = response.status().status_code() == StatusCode::NotModified;
        let token = plan.to_continuation_token()?;
        if idle {
            let mut lease = lease.lock().await;
            if lease
                .value
                .base
                .continuation
                .as_ref()
                .map(ContinuationToken::as_str)
                != Some(token.as_str())
            {
                context
                    .update(&mut lease, Update::Checkpoint(&token))
                    .await?;
            }
            drop(lease);
            sleep(context.options.poll_delay).await;
            continue;
        }
        let (sender, received) = oneshot::channel();
        session.deliver(
            response,
            BatchReceipt {
                sender,
                cancelled: cancelled.clone(),
            },
        )?;
        let acknowledgment: Acknowledgment = received.await.map_err(|_| {
            error(
                StatusCode::Conflict,
                "Batch was abandoned without successful processing.",
            )
        })?;
        let mut lease = lease.lock().await;
        let result = context.update(&mut lease, Update::Checkpoint(&token)).await;
        let failed = result.is_err();
        let _ = acknowledgment.result.send(result);
        if failed {
            return Err(error(
                StatusCode::ServiceUnavailable,
                "Checkpoint failed; abandoning the lease.",
            ));
        }
    }
}
