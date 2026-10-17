// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod balancing;
#[cfg(all(test, feature = "__internal_in_memory_emulator"))]
mod tests;

use crate::{
    change_feed_processor::{
        bootstrap,
        bootstrap::{
            error, namespace,
            store::{Store, Stored},
            Identity, ProcessingLease, RangePlan, SCHEMA_VERSION,
        },
        BootstrapOptions, ChangeFeedProcessorMode,
    },
    models::{
        ChangeFeedStartFrom, ContainerReference, ContinuationToken, CosmosOperation,
        CosmosResponse, MaxItemCountHint,
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
    task::AtomicWaker,
    FutureExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashSet, VecDeque},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex as StdMutex,
    },
    task::Poll,
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
    bootstrap: BootstrapOptions,
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
    pub fn new(bootstrap: BootstrapOptions) -> Self {
        Self {
            bootstrap,
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
}

#[derive(Clone, Serialize, Deserialize)]
struct Lease {
    #[serde(flatten)]
    base: ProcessingLease,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    incarnation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    renewed_at: Option<i64>,
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
        if lease.base.record.workload != self.identity || lease.base.record.ttl != -1 {
            return Err(error(
                StatusCode::Conflict,
                "Incompatible processing lease.",
            ));
        }
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Lease>> {
        let plan = self
            .store()
            .read::<RangePlan>(&format!("{}.plan", self.namespace))
            .await?
            .ok_or_else(|| error(StatusCode::NotFound, "Bootstrap range plan is missing."))?;
        if plan.value.record.workload != self.identity {
            return Err(error(
                StatusCode::Conflict,
                "Incompatible bootstrap range plan.",
            ));
        }
        let mut leases = Vec::with_capacity(plan.value.ranges.len());
        for range in plan.value.ranges {
            let id = format!(
                "{}.lease.{}-{}",
                self.namespace,
                range.min_inclusive(),
                range.max_exclusive()
            );
            let stored = self
                .store()
                .read::<Lease>(&id)
                .await?
                .ok_or_else(|| error(StatusCode::NotFound, "Processing lease is missing."))?;
            self.validate(&stored.value)?;
            if stored.value.base.record.id != id || stored.value.base.range != range {
                return Err(error(
                    StatusCode::Conflict,
                    "Processing lease range does not match the plan.",
                ));
            }
            leases.push(stored.value);
        }
        Ok(leases)
    }

    async fn acquire(&self, observed: &Lease) -> Result<Stored<Lease>> {
        let id = &observed.base.record.id;
        let mut current = self.store().read::<Lease>(id).await?.ok_or_else(lost)?;
        self.validate(&current.value)?;
        if current.value.base.owner != observed.base.owner
            || current.value.incarnation != observed.incarnation
        {
            return Err(lost());
        }
        current.value.base.owner = Some(self.options.bootstrap.instance_name.clone());
        current.value.incarnation = Some(Uuid::new_v4().to_string());
        current.value.renewed_at = Some(now_millis());
        self.store().replace(id, &current.value, current.etag).await
    }

    async fn update(&self, lease: &mut Stored<Lease>, update: Update<'_>) -> Result<()> {
        let id = lease.value.base.record.id.clone();
        for _ in 0..5 {
            let mut current = self.store().read::<Lease>(&id).await?.ok_or_else(lost)?;
            self.validate(&current.value)?;
            if current.value.base.owner != lease.value.base.owner
                || current.value.incarnation != lease.value.incarnation
            {
                return Err(lost());
            }
            match update {
                Update::Renew => {}
                Update::Checkpoint(token) => current.value.base.continuation = Some(token.clone()),
                Update::Release => current.value.base.owner = None,
            }
            current.value.renewed_at = Some(now_millis());
            match self
                .store()
                .replace(&id, &current.value, current.etag)
                .await
            {
                Ok(updated) => {
                    *lease = updated;
                    return Ok(());
                }
                Err(err) if err.status().is_precondition_failed() => continue,
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
    Renew,
    Checkpoint(&'a ContinuationToken),
    Release,
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
pub struct Batch {
    lease_token: String,
    response: CosmosResponse,
    receipt: BatchReceipt,
}

impl Batch {
    /// Separates the lease identity, response, and acknowledgment receipt.
    pub fn into_parts(self) -> (String, CosmosResponse, BatchReceipt) {
        (self.lease_token, self.response, self.receipt)
    }
}

struct SharedState {
    ready: StdMutex<VecDeque<Batch>>,
    waker: AtomicWaker,
    running: AtomicBool,
}

impl SharedState {
    fn deliver(&self, batch: Batch) {
        let mut queue = self.ready.lock().unwrap_or_else(|e| e.into_inner());
        queue.retain(|batch| !batch.receipt.sender.is_canceled());
        queue.push_back(batch);
        drop(queue);
        self.waker.wake();
    }
}

struct Owner {
    stop: StdMutex<Option<oneshot::Sender<()>>>,
    stopped: Signal,
    state: Arc<SharedState>,
}

/// A running, schema-agnostic change feed processor.
///
/// Clones share a lifecycle. Dropping the last handle requests shutdown.
/// Lease renewal and balancing continue while consumers process batches.
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
    /// Returns configuration and bootstrap errors before starting background
    /// work. All-versions-and-deletes mode requires a now start or continuation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::{
    ///     change_feed_processor::{BootstrapOptions, ChangeFeedProcessorMode, Processor, ProcessorOptions},
    ///     models::{ContainerReference, CosmosResponse}, CosmosDriver, Result,
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
    ///     let options = ProcessorOptions::new(BootstrapOptions::new(
    ///         ChangeFeedProcessorMode::LatestVersion, "orders", "worker-01",
    ///     ));
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
            || (options.bootstrap.mode == ChangeFeedProcessorMode::AllVersionsAndDeletes
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
            &options.bootstrap,
        )
        .await?;
        let identity = Identity {
            version: SCHEMA_VERSION,
            account: source.account().endpoint().origin().ascii_serialization(),
            container_rid: source.rid().to_owned(),
            processor_name: options.bootstrap.processor_name.clone(),
            mode: options.bootstrap.mode,
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
        let state = Arc::new(SharedState {
            ready: StdMutex::new(VecDeque::new()),
            waker: AtomicWaker::new(),
            running: AtomicBool::new(true),
        });
        let (stop, stopping) = signal();
        let (done, stopped) = signal();
        let owner = Arc::new(Owner {
            stop: StdMutex::new(Some(stop)),
            stopped,
            state: state.clone(),
        });
        // The task holds state, not Owner, so the last public handle closes stop.
        let task = azure_core::async_runtime::get_async_runtime().spawn(Box::pin(async move {
            run(context, state.clone(), stopping).await;
            state.running.store(false, Ordering::Release);
            state
                .ready
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clear();
            state.waker.wake();
            let _ = done.send(());
        }));
        drop(task);
        Ok(Self { owner })
    }

    /// Reports whether background coordination is running.
    pub fn is_running(&self) -> bool {
        self.owner.state.running.load(Ordering::Acquire)
    }

    /// Waits for an owned batch, or returns `None` after shutdown.
    ///
    /// Canceling this wait does not consume a batch. Progress from 304 responses
    /// is saved internally without invoking application callbacks.
    pub async fn next_batch(&self) -> Option<Batch> {
        futures::future::poll_fn(|cx| {
            self.owner.state.waker.register(cx.waker());
            let mut ready = self
                .owner
                .state
                .ready
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            while let Some(batch) = ready.pop_front() {
                if !batch.receipt.sender.is_canceled() {
                    return Poll::Ready(Some(batch));
                }
            }
            if self.is_running() {
                Poll::Pending
            } else {
                Poll::Ready(None)
            }
        })
        .await
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

async fn run(context: Arc<Context>, state: Arc<SharedState>, stop: Signal) {
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
                        let state = state.clone();
                        let stop = stop.clone();
                        workers.push(async move {
                            supervise(context, state, lease, stop).await;
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
    state: Arc<SharedState>,
    lease: Stored<Lease>,
    stop: Signal,
) {
    let token = lease.value.base.record.id.clone();
    let lease = Arc::new(Mutex::new(lease));
    let progress = Arc::new(StdMutex::new(Instant::now()));
    let (cancel, cancelled) = signal();
    let result = {
        let processing = poll(
            context.clone(),
            state,
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
    };
    drop(cancel);
    if let Err(err) = result {
        tracing::warn!(lease = %token, error = %err, "Lease processing stopped");
    }
    let mut lease = lease.lock().await;
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
    state: Arc<SharedState>,
    lease: Arc<Mutex<Stored<Lease>>>,
    progress: Arc<StdMutex<Instant>>,
    cancelled: Signal,
) -> Result<()> {
    let initial = lease.lock().await.value.clone();
    let operation = match context.options.bootstrap.mode {
        ChangeFeedProcessorMode::LatestVersion => {
            CosmosOperation::change_feed(context.source.clone(), Some(initial.base.range.clone()))
        }
        ChangeFeedProcessorMode::AllVersionsAndDeletes => {
            CosmosOperation::change_feed_all_versions_and_deletes(
                context.source.clone(),
                Some(initial.base.range.clone()),
            )
        }
    }
    .with_change_feed_start(context.options.start.clone())
    .with_max_item_count(context.options.page_size);
    let mut plan = context
        .source_driver
        .plan_operation(
            operation,
            &OperationOptions::default(),
            initial
                .base
                .continuation
                .as_ref()
                .or(context.options.continuation.as_ref()),
            &PlanOptions::default(),
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
        state.deliver(Batch {
            lease_token: initial.base.record.id.clone(),
            response,
            receipt: BatchReceipt {
                sender,
                cancelled: cancelled.clone(),
            },
        });
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
