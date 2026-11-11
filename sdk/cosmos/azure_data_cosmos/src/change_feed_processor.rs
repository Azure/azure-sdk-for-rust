// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    diagnostics::DiagnosticsContext,
    feed::ContinuationToken,
    models::ChangeFeedItem,
    options::{ChangeFeedMode, MaxItemCountHint},
    ContainerClient, CosmosError, CosmosStatus,
};
use azure_core::{fmt::SafeDebug, http::StatusCode};
use futures::future::BoxFuture;
use serde::de::DeserializeOwned;
use std::{
    future::Future,
    marker::PhantomData,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use time::OffsetDateTime;

mod execution;

type ChangeFeedHandler<T, Fut> = fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut;
type ErrorHandler =
    dyn Fn(Option<ChangeFeedProcessorContext>, CosmosError) -> BoxFuture<'static, ()> + Send + Sync;

/// Controls when successful change feed callback progress is persisted.
///
/// Callback page sizes, ordering, and lease renewal are independent of this policy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChangeFeedCheckpointPolicy {
    /// Persists each successful callback before reading its lease's next batch.
    #[default]
    EveryBatch,
    /// Coalesces successful callbacks until the interval since the first
    /// unpersisted success elapses. Idle reads do not trigger persistence.
    ///
    /// Successful callbacks can replay after a crash or ownership loss. Graceful
    /// shutdown and topology recovery flush without waiting for the interval.
    /// The interval schedules a write, not a storage-availability guarantee.
    Coalesced {
        /// Positive maximum interval for deferring persistence of successful progress.
        max_interval: Duration,
    },
}

impl ChangeFeedCheckpointPolicy {
    /// Coalesces successful progress for up to five minutes.
    ///
    /// Use [`Self::Coalesced`] to select a different interval. Graceful cleanup
    /// and explicit checkpoints can persist progress earlier.
    pub fn coalesced() -> Self {
        Self::Coalesced {
            max_interval: Duration::from_secs(300),
        }
    }
}

/// Context identifying the lease associated with a callback batch.
///
/// Passed by value to the callback registered with
/// [`ChangeFeedProcessorBuilder::handle_change_feed()`] in either mode.
/// Applications may retain the context across an await, but it is not proof
/// of current lease ownership or authorization to save a checkpoint.
#[derive(Clone, SafeDebug)]
pub struct ChangeFeedProcessorContext {
    lease_token: String,
    #[safe(false)]
    diagnostics: Option<Arc<DiagnosticsContext>>,
}

impl ChangeFeedProcessorContext {
    /// Returns the opaque identifier of the lease associated with this batch.
    ///
    /// This is not a continuation token or a physical partition identifier.
    /// Treat it as meaningful within the processor's lease namespace.
    pub fn lease_token(&self) -> &str {
        &self.lease_token
    }

    /// Returns operation diagnostics associated with this notification.
    ///
    /// For change callbacks, these describe the read that produced the batch,
    /// not subsequent callback execution or checkpoint writes. Error notifications
    /// may have no diagnostics. Retaining this handle does not retain the batch
    /// payload, and does not render or log the diagnostics.
    pub fn diagnostics(&self) -> Option<Arc<DiagnosticsContext>> {
        self.diagnostics.clone()
    }
}

/// Monitoring information for one lease in a logical change feed processor.
///
/// Returned by [`ChangeFeedProcessor::current_state()`]. The record describes
/// observed state, not a guarantee of current ownership. Monitoring is not yet
/// implemented, so no records are currently produced.
#[derive(Clone, SafeDebug)]
pub struct ChangeFeedProcessorState {
    lease_token: String,
    host_name: Option<String>,
    continuation_token: Option<ContinuationToken>,
    estimated_lag: Option<u64>,
}

impl ChangeFeedProcessorState {
    /// Returns the opaque identifier of the lease this record describes.
    ///
    /// Uses the same identity as [`ChangeFeedProcessorContext::lease_token()`].
    pub fn lease_token(&self) -> &str {
        &self.lease_token
    }

    /// Returns the observed owner's instance name, or `None` for an unowned lease.
    pub fn host_name(&self) -> Option<&str> {
        self.host_name.as_deref()
    }

    /// Returns the saved continuation for this lease, if progress has been saved.
    ///
    /// This is lease-scoped progress, not a container-wide restart token.
    pub fn continuation_token(&self) -> Option<&ContinuationToken> {
        self.continuation_token.as_ref()
    }

    /// Returns an estimate of outstanding changes, or `None` if unavailable.
    ///
    /// `Some(0)` is a zero estimate, not an unavailable estimate. This value is
    /// not an exact count of documents or callback batches remaining.
    pub fn estimated_lag(&self) -> Option<u64> {
        self.estimated_lag
    }
}

enum ProcessorStartFrom {
    Now,
    Beginning,
    PointInTime(OffsetDateTime),
    Continuation(ContinuationToken),
}

/// Builder for configuring a change feed processor.
///
/// Owns the monitored and lease container clients and the processor and instance names.
/// Pass a [`ChangeFeedMode`] to [`new()`](Self::new)
/// to select a mode explicitly. There is no mode-free constructor or
/// mode-changing setter.
///
/// Register an async batch callback with [`handle_change_feed()`](Self::handle_change_feed),
/// then call [`build()`](Self::build) to transfer the configuration into a
/// [`ChangeFeedProcessor`]. The start
/// configuration applies to initialization without saved progress; it does not
/// configure a reset of existing checkpoints.
///
/// `T` identifies the deserialized item type and `H` the callback
/// type. Before registration, `T` and `H` default to `()`. Registering a callback
/// infers both types and makes [`build()`](Self::build) available.
///
/// Each owned lease has at most one outstanding batch. SDK processing defaults
/// to one batch poller; [`with_max_concurrency()`](Self::with_max_concurrency) bounds
/// concurrent receive, callback, and acknowledgment cycles for this instance.
/// A worker is one started processor instance; it can run multiple batch pollers.
/// Page-size hints and poller counts are not byte or heap limits.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos::{options::ChangeFeedMode, ChangeFeedProcessorBuilder, ContainerClient};
///
/// fn configure(
///     monitored_container: ContainerClient,
///     lease_container: ContainerClient,
/// ) -> ChangeFeedProcessorBuilder {
///     ChangeFeedProcessorBuilder::new(
///         ChangeFeedMode::LatestVersion,
///         monitored_container,
///         lease_container,
///         "order-projection",
///         "worker-01",
///     )
///     .with_start_from_beginning()
/// }
/// ```
///
/// Select all versions and deletes by passing its mode value:
///
/// ```
/// use azure_data_cosmos::{
///     options::ChangeFeedMode, ChangeFeedProcessorBuilder, ContainerClient,
/// };
///
/// fn configure(
///     monitored_container: ContainerClient,
///     lease_container: ContainerClient,
/// ) -> ChangeFeedProcessorBuilder {
///     ChangeFeedProcessorBuilder::new(
///         ChangeFeedMode::AllVersionsAndDeletes,
///         monitored_container,
///         lease_container,
///         "order-audit",
///         "worker-01",
///     )
/// }
/// ```
///
/// Beginning and time-based starts are rejected by [`build()`](Self::build)
/// in all-versions-and-deletes mode.
///
/// Construction requires both clients and both names:
///
/// ```compile_fail,E0061
/// use azure_data_cosmos::{options::ChangeFeedMode, ChangeFeedProcessorBuilder};
///
/// let _builder = ChangeFeedProcessorBuilder::new(ChangeFeedMode::LatestVersion);
/// ```
///
/// Construction without selecting a mode is unavailable:
///
/// ```compile_fail,E0061
/// use azure_data_cosmos::{ChangeFeedProcessorBuilder, ContainerClient};
///
/// fn configure(monitored_container: ContainerClient, lease_container: ContainerClient) {
///     let _builder = ChangeFeedProcessorBuilder::new(
///         monitored_container, lease_container, "orders", "worker-01",
///     );
/// }
/// ```
///
#[must_use]
pub struct ChangeFeedProcessorBuilder<T = (), H = ()> {
    monitored_container: ContainerClient,
    lease_container: ContainerClient,
    processor_name: String,
    instance_name: String,
    mode: ChangeFeedMode,
    start_from: ProcessorStartFrom,
    max_item_count: MaxItemCountHint,
    feed_poll_delay: Duration,
    checkpoint_policy: ChangeFeedCheckpointPolicy,
    max_concurrency: NonZeroU32,
    error_handler: Option<Arc<ErrorHandler>>,
    min_scale_count: u32,
    max_scale_count: u32,
    lease_verification_on_restart_enabled: bool,
    handler: Arc<H>,
    item: PhantomData<fn() -> T>,
}

impl ChangeFeedProcessorBuilder {
    /// Creates a builder with an explicit mode and the required clients and names.
    ///
    /// Uses the same [`ChangeFeedMode`] as the pull change feed API.
    ///
    /// `monitored_container` identifies the source of changes, and `lease_container`
    /// identifies storage for leases and checkpoints. `processor_name` supplies
    /// the workload's lease prefix. Cooperating instances use the same processor
    /// name, monitored container, and lease container; different processor names
    /// identify independent workloads. `instance_name` identifies this instance
    /// within the group, not the workload.
    ///
    /// Takes ownership of both clients. Clone a client before passing it here if
    /// it is needed elsewhere. This method does not validate the names, perform
    /// network I/O, or start background work. The initial start position is now.
    /// Beginning and time-based starts are validated against the mode by [`build()`](Self::build).
    /// Lease identifiers are constructed when the processor starts.
    pub fn new(
        mode: ChangeFeedMode,
        monitored_container: ContainerClient,
        lease_container: ContainerClient,
        processor_name: impl Into<String>,
        instance_name: impl Into<String>,
    ) -> Self {
        Self {
            monitored_container,
            lease_container,
            processor_name: processor_name.into(),
            instance_name: instance_name.into(),
            mode,
            start_from: ProcessorStartFrom::Now,
            max_item_count: MaxItemCountHint::Limit(NonZeroU32::new(100).expect("100 is nonzero")),
            feed_poll_delay: Duration::from_secs(5),
            checkpoint_policy: ChangeFeedCheckpointPolicy::default(),
            max_concurrency: NonZeroU32::MIN,
            error_handler: None,
            min_scale_count: 0,
            max_scale_count: 0,
            lease_verification_on_restart_enabled: false,
            handler: Arc::new(()),
            item: PhantomData,
        }
    }

    /// Registers an async function that receives owned context and a batch of changes.
    ///
    /// The first argument is [`ChangeFeedProcessorContext`], identifying the batch's
    /// lease. The second is an owned vector of [`ChangeFeedItem<T>`] envelopes in
    /// either mode. The function returns a future resolving to [`crate::Result<()>`].
    /// Each returned future must be [`Send`] and cannot borrow short-lived data.
    /// The callback is stored as a function pointer; capturing closures are not accepted.
    ///
    /// Consumes the builder and makes [`build()`](Self::build) available. A
    /// callback can be registered only once. Callbacks for different leases can
    /// overlap when [`with_max_concurrency()`](Self::with_max_concurrency) exceeds one.
    /// For each lease, successful callback progress is saved before the
    /// next batch by default. [`with_checkpoint_policy()`](Self::with_checkpoint_policy)
    /// can defer persistence across callbacks. Failures can cause replay;
    /// callbacks should make their external effects idempotent.
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_data_cosmos::{
    ///     models::ChangeFeedItem, ChangeFeedProcessorBuilder, ChangeFeedProcessorContext,
    /// };
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct Order {
    ///     id: String,
    /// }
    ///
    /// async fn on_changes(
    ///     context: ChangeFeedProcessorContext,
    ///     changes: Vec<ChangeFeedItem<Order>>,
    /// ) -> azure_data_cosmos::Result<()> {
    ///     println!("Lease: {}", context.lease_token());
    ///     for change in changes {
    ///         if let Some(order) = change.current() {
    ///             println!("Changed order: {}", order.id);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder) -> azure_data_cosmos::Result<()> {
    ///     let _processor = builder.handle_change_feed(on_changes).build()?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// A synchronous callback is not accepted:
    ///
    /// ```compile_fail,E0277
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder};
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder) {
    ///     let _builder = builder.handle_change_feed(|_context, _: Vec<ChangeFeedItem<String>>| {
    ///         Ok::<(), azure_data_cosmos::CosmosError>(())
    ///     });
    /// }
    /// ```
    ///
    /// The callback's future must return a result:
    ///
    /// ```compile_fail,E0271
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder};
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder) {
    ///     let _builder = builder.handle_change_feed(|_context, _: Vec<ChangeFeedItem<String>>| async {});
    /// }
    /// ```
    ///
    /// Item types must support deserialization:
    ///
    /// ```compile_fail,E0277
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder};
    ///
    /// struct NotDeserializable;
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder) {
    ///     let _builder = builder
    ///         .handle_change_feed(|_context, _: Vec<ChangeFeedItem<NotDeserializable>>| async { Ok(()) });
    /// }
    /// ```
    ///
    /// The callback must accept both the context and the batch:
    ///
    /// ```compile_fail,E0593,E0308
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder};
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder) {
    ///     let _builder = builder
    ///         .handle_change_feed(|_: Vec<ChangeFeedItem<String>>| async { Ok(()) });
    /// }
    /// ```
    ///
    /// A callback cannot be registered twice:
    ///
    /// ```compile_fail,E0599
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder};
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder) {
    ///     let _builder = builder
    ///         .handle_change_feed(|_, _: Vec<ChangeFeedItem<String>>| async { Ok(()) })
    ///         .handle_change_feed(|_, _: Vec<ChangeFeedItem<String>>| async { Ok(()) });
    /// }
    /// ```
    ///
    /// A callback cannot capture application state:
    ///
    /// ```compile_fail,E0308
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder};
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder) {
    ///     let label = String::from("orders");
    ///     let _builder = builder.handle_change_feed(
    ///         move |_, _: Vec<ChangeFeedItem<String>>| {
    ///             let label = label.clone();
    ///             async move {
    ///                 println!("{label}");
    ///                 Ok(())
    ///             }
    ///         },
    ///     );
    /// }
    /// ```
    pub fn handle_change_feed<T, Fut>(
        self,
        handler: ChangeFeedHandler<T, Fut>,
    ) -> ChangeFeedProcessorBuilder<T, ChangeFeedHandler<T, Fut>>
    where
        T: DeserializeOwned + Send + 'static,
        Fut: Future<Output = crate::Result<()>> + Send + 'static,
    {
        ChangeFeedProcessorBuilder {
            monitored_container: self.monitored_container,
            lease_container: self.lease_container,
            processor_name: self.processor_name,
            instance_name: self.instance_name,
            mode: self.mode,
            start_from: self.start_from,
            max_item_count: self.max_item_count,
            feed_poll_delay: self.feed_poll_delay,
            checkpoint_policy: self.checkpoint_policy,
            max_concurrency: self.max_concurrency,
            error_handler: self.error_handler,
            min_scale_count: self.min_scale_count,
            max_scale_count: self.max_scale_count,
            lease_verification_on_restart_enabled: self.lease_verification_on_restart_enabled,
            handler: Arc::new(handler),
            item: PhantomData,
        }
    }
}

impl<T, H> ChangeFeedProcessorBuilder<T, H> {
    /// Sets the maximum number of concurrent batch pollers in this worker.
    ///
    /// Defaults to one. Each poller receives one batch, runs its callback, and
    /// completes acknowledgment before requesting another batch. A slow callback
    /// occupies one poller; other pollers can process other eligible leases.
    /// This does not limit lease ownership or delay lease maintenance.
    pub fn with_max_concurrency(mut self, max_concurrency: NonZeroU32) -> Self {
        self.max_concurrency = max_concurrency;
        self
    }

    /// Registers an asynchronous observer of background and processing failures.
    ///
    /// The optional context identifies the affected lease. It is absent for
    /// worker-wide failures. Notifications include driver failures, deserialization
    /// and callback errors, caught callback panics, and acknowledgment failures.
    /// Startup errors are returned directly by [`ChangeFeedProcessor::start()`].
    ///
    /// The observer runs serially, independently of batch pollers and lease
    /// maintenance. Its notification queue is bounded; overflow is logged and
    /// excess notifications are dropped. Related failures may produce multiple
    /// notifications. The observer cannot acknowledge a batch or change recovery.
    ///
    /// Graceful shutdown drains accepted notifications; forced shutdown or drop
    /// cancels the observer. No implicit timeout applies. Do not await processor
    /// shutdown from this observer. Observer panics are logged without recursion.
    ///
    /// Replaces any previously registered error observer.
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_data_cosmos::{
    ///     ChangeFeedProcessorBuilder, ChangeFeedProcessorContext, CosmosError,
    /// };
    ///
    /// async fn on_error(context: Option<ChangeFeedProcessorContext>, error: CosmosError) {
    ///     eprintln!("Lease {:?}: {error}", context.map(|c| c.lease_token().to_owned()));
    /// }
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder) -> ChangeFeedProcessorBuilder {
    ///     builder.handle_error(on_error)
    /// }
    /// ```
    pub fn handle_error<Fut>(
        mut self,
        handler: fn(Option<ChangeFeedProcessorContext>, CosmosError) -> Fut,
    ) -> Self
    where
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.error_handler = Some(Arc::new(move |context, error| {
            Box::pin(handler(context, error))
        }));
        self
    }

    /// Sets the persistence policy for successfully handled change feed batches.
    ///
    /// Defaults to [`ChangeFeedCheckpointPolicy::EveryBatch`]. Coalescing retains
    /// progress, not callback payloads, in memory and can replay multiple successful
    /// callbacks after failure. It does not change the service page-size hint.
    /// Invalid intervals are rejected by [`build()`](Self::build).
    ///
    /// Processing inactivity makes a lease eligible for takeover after ten
    /// minutes with every-batch checkpoints, or twice the coalescing interval.
    /// Each acquisition gets a fresh grace period. Idle reads, absent demand,
    /// and throttling do not count as successful batch processing. Activity
    /// becomes visible to other workers through existing lease writes.
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_data_cosmos::{
    ///     ChangeFeedCheckpointPolicy, ChangeFeedProcessorBuilder,
    /// };
    /// use std::time::Duration;
    ///
    /// fn configure(
    ///     builder: ChangeFeedProcessorBuilder,
    ///     replay_interval: Duration,
    /// ) -> ChangeFeedProcessorBuilder {
    ///     builder.with_checkpoint_policy(ChangeFeedCheckpointPolicy::Coalesced {
    ///         max_interval: replay_interval,
    ///     })
    /// }
    /// ```
    pub fn with_checkpoint_policy(mut self, policy: ChangeFeedCheckpointPolicy) -> Self {
        self.checkpoint_policy = policy;
        self
    }

    /// Sets the requested maximum number of items per change feed page.
    ///
    /// Defaults to 100. This is a service hint, not a hard limit on callback batch
    /// size. Pass [`MaxItemCountHint::ServerDecides`] to let the service choose.
    /// The hint applies to each acquired lease's feed requests.
    ///
    /// # Examples
    ///
    /// Shared settings are available in either mode, before or after callback registration:
    ///
    /// ```
    /// use azure_data_cosmos::{
    ///     feed::ContinuationToken,
    ///     models::ChangeFeedItem,
    ///     options::MaxItemCountHint,
    ///     ChangeFeedProcessorBuilder, ChangeFeedProcessorContext,
    /// };
    /// use std::{num::NonZeroU32, time::Duration};
    ///
    /// async fn on_changes(
    ///     _context: ChangeFeedProcessorContext,
    ///     _changes: Vec<ChangeFeedItem<String>>,
    /// ) -> azure_data_cosmos::Result<()> {
    ///     Ok(())
    /// }
    ///
    /// fn configure(
    ///     builder: ChangeFeedProcessorBuilder,
    ///     initial_continuation: ContinuationToken,
    /// ) -> azure_data_cosmos::Result<()> {
    ///     let _processor = builder
    ///         .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(200).unwrap()))
    ///         .with_feed_poll_delay(Duration::from_secs(2))
    ///         .with_start_continuation(initial_continuation)
    ///         .with_min_scale_count(1)
    ///         .with_max_scale_count(4)
    ///         .with_lease_verification_enabled_on_restart(true)
    ///         .handle_change_feed(on_changes)
    ///         .with_feed_poll_delay(Duration::from_secs(3))
    ///         .build()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn with_max_item_count(mut self, max_item_count: MaxItemCountHint) -> Self {
        self.max_item_count = max_item_count;
        self
    }

    /// Sets the delay between feed polls after current changes have been drained.
    ///
    /// Defaults to five seconds. A zero duration requests no polling delay.
    /// The delay applies independently to each idle lease.
    pub fn with_feed_poll_delay(mut self, feed_poll_delay: Duration) -> Self {
        self.feed_poll_delay = feed_poll_delay;
        self
    }

    /// Sets the initial change feed continuation when no saved lease progress exists.
    ///
    /// Replaces any previously configured initial start position.
    /// A subsequent start-position setter replaces this token in turn.
    /// This setting does not reset existing lease checkpoints.
    ///
    /// [`build()`](Self::build) rejects an empty or whitespace-only token.
    /// Supply a lease-scoped change feed token for the monitored source and
    /// selected mode. The planner validates compatibility when the lease is
    /// read; this setting does not distribute a container-wide token across leases.
    pub fn with_start_continuation(mut self, continuation: ContinuationToken) -> Self {
        self.start_from = ProcessorStartFrom::Continuation(continuation);
        self
    }

    /// Sets the minimum target number of leases for this host.
    ///
    /// Defaults to zero, which specifies no minimum target. A nonzero target
    /// requests a minimum allocation rather than relying solely on equal balancing;
    /// it cannot guarantee that enough leases are available.
    ///
    /// [`build()`](Self::build) rejects a minimum greater than a nonzero maximum
    /// set with [`with_max_scale_count()`](Self::with_max_scale_count).
    pub fn with_min_scale_count(mut self, min_scale_count: u32) -> Self {
        self.min_scale_count = min_scale_count;
        self
    }

    /// Sets the maximum number of leases this host should own.
    ///
    /// Defaults to zero, meaning no configured maximum. [`build()`](Self::build)
    /// rejects a nonzero maximum below the minimum set with
    /// [`with_min_scale_count()`](Self::with_min_scale_count).
    pub fn with_max_scale_count(mut self, max_scale_count: u32) -> Self {
        self.max_scale_count = max_scale_count;
        self
    }

    /// Configures checking for missing initial leases when the processor starts.
    ///
    /// Disabled by default. After bootstrap, the processing runtime recreates
    /// missing leases from the persisted initial range plan without replacing
    /// existing owners or checkpoints. Complete replacement coverage prevents
    /// recreation of deleted split parents; partial or overlapping coverage
    /// fails verification. This does not discover topology changes.
    pub fn with_lease_verification_enabled_on_restart(mut self, enabled: bool) -> Self {
        self.lease_verification_on_restart_enabled = enabled;
        self
    }
}

impl<T, Fut> ChangeFeedProcessorBuilder<T, ChangeFeedHandler<T, Fut>>
where
    T: DeserializeOwned + Send + 'static,
    Fut: Future<Output = crate::Result<()>> + Send + 'static,
{
    /// Validates local configuration and returns a processor with its callback.
    ///
    /// Available only after [`handle_change_feed()`](ChangeFeedProcessorBuilder::handle_change_feed).
    /// Consumes the builder whether validation succeeds or fails. Names and tokens
    /// are checked for blank values but are not trimmed or otherwise modified.
    /// A zero polling delay and an uncapped maximum of zero are valid.
    ///
    /// Does not invoke the callback, perform network I/O, or start background work.
    /// Container eligibility and continuation-token format, scope, and
    /// mode compatibility are validated later during bootstrap and feed planning.
    ///
    /// # Errors
    ///
    /// Returns a local error with [`StatusCode::BadRequest`] if:
    ///
    /// - The processor or instance name is empty or contains only whitespace.
    /// - The minimum lease count exceeds a nonzero maximum lease count.
    /// - The selected initial continuation is empty or contains only whitespace.
    /// - All-versions-and-deletes mode is combined with a beginning or time-based start.
    /// - A coalesced checkpoint interval is zero or unrepresentable.
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_data_cosmos::{
    ///     models::ChangeFeedItem, ChangeFeedProcessorBuilder, ChangeFeedProcessorContext,
    /// };
    ///
    /// async fn on_changes(
    ///     _context: ChangeFeedProcessorContext,
    ///     _changes: Vec<ChangeFeedItem<String>>,
    /// ) -> azure_data_cosmos::Result<()> {
    ///     Ok(())
    /// }
    ///
    /// fn create(builder: ChangeFeedProcessorBuilder) -> azure_data_cosmos::Result<()> {
    ///     let _processor = builder
    ///         .handle_change_feed(on_changes)
    ///         .with_start_from_now()
    ///         .build()?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// A callback is required before building:
    ///
    /// ```compile_fail,E0599
    /// use azure_data_cosmos::ChangeFeedProcessorBuilder;
    ///
    /// fn create(builder: ChangeFeedProcessorBuilder) {
    ///     let _processor = builder
    ///         .with_feed_poll_delay(std::time::Duration::from_secs(2))
    ///         .with_max_scale_count(4)
    ///         .build();
    /// }
    /// ```
    ///
    /// Building consumes the builder, so it cannot be reused:
    ///
    /// ```compile_fail,E0382
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder};
    ///
    /// fn create(builder: ChangeFeedProcessorBuilder) {
    ///     let builder = builder
    ///         .handle_change_feed(|_context, _: Vec<ChangeFeedItem<String>>| async { Ok(()) });
    ///     let _processor = builder.build();
    ///     let _another = builder.build();
    /// }
    /// ```
    pub fn build(self) -> crate::Result<ChangeFeedProcessor<T, ChangeFeedHandler<T, Fut>>> {
        if let ChangeFeedCheckpointPolicy::Coalesced { max_interval } = self.checkpoint_policy {
            let now = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as i64;
            if max_interval.is_zero()
                || std::time::Instant::now()
                    .checked_add(max_interval)
                    .is_none()
                || azure_core::time::Duration::try_from(max_interval).is_err()
                || max_interval
                    .checked_mul(2)
                    .and_then(|duration| {
                        i64::try_from(duration.as_nanos().div_ceil(1_000_000)).ok()
                    })
                    .and_then(|millis| now.checked_add(millis))
                    .is_none()
            {
                return Err(CosmosError::builder()
                    .with_status(CosmosStatus::new(StatusCode::BadRequest))
                    .with_message("Invalid checkpoint interval.")
                    .build());
            }
        }
        for (name, value) in [
            ("processor_name", self.processor_name.as_str()),
            ("instance_name", self.instance_name.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(CosmosError::builder()
                    .with_status(CosmosStatus::new(StatusCode::BadRequest))
                    .with_message(format!("{name} must not be blank."))
                    .build());
            }
        }
        if self.max_scale_count != 0 && self.min_scale_count > self.max_scale_count {
            return Err(CosmosError::builder()
                .with_status(CosmosStatus::new(StatusCode::BadRequest))
                .with_message("min_scale_count must not exceed a nonzero max_scale_count.")
                .build());
        }
        if matches!(&self.start_from, ProcessorStartFrom::Continuation(token) if token.as_str().trim().is_empty())
        {
            return Err(CosmosError::builder()
                .with_status(CosmosStatus::new(StatusCode::BadRequest))
                .with_message("start_continuation must not be blank.")
                .build());
        }
        if self.mode == ChangeFeedMode::AllVersionsAndDeletes
            && matches!(
                &self.start_from,
                ProcessorStartFrom::Beginning | ProcessorStartFrom::PointInTime(_)
            )
        {
            return Err(CosmosError::builder()
                .with_status(CosmosStatus::new(StatusCode::BadRequest))
                .with_message(
                    "AllVersionsAndDeletes mode does not support beginning or time-based starts.",
                )
                .build());
        }
        Ok(ChangeFeedProcessor {
            configuration: self,
            running: async_lock::Mutex::new(None),
            started: Arc::new(AtomicBool::new(false)),
        })
    }
}

impl<T, H> ChangeFeedProcessorBuilder<T, H> {
    /// Configures reading from the beginning when no saved progress exists.
    ///
    /// Supported only in [`ChangeFeedMode::LatestVersion`].
    /// [`build()`](Self::build) rejects this start in all-versions-and-deletes mode.
    ///
    /// Replaces any previously configured start position.
    pub fn with_start_from_beginning(mut self) -> Self {
        self.start_from = ProcessorStartFrom::Beginning;
        self
    }

    /// Configures reading after `start_time` when no saved progress exists.
    ///
    /// Supported only in [`ChangeFeedMode::LatestVersion`].
    /// [`build()`](Self::build) rejects this start in all-versions-and-deletes mode.
    ///
    /// Replaces any previously configured start position.
    pub fn with_start_time(mut self, start_time: OffsetDateTime) -> Self {
        self.start_from = ProcessorStartFrom::PointInTime(start_time);
        self
    }

    /// Configures reading from now when no saved progress exists.
    ///
    /// Replaces any previously configured start position. This is the default.
    pub fn with_start_from_now(mut self) -> Self {
        self.start_from = ProcessorStartFrom::Now;
        self
    }
}

/// A change feed processor configured for a specific mode.
///
/// Created by [`ChangeFeedProcessorBuilder::build()`]. Starting bootstraps the
/// workload and coordinates leases through the shared driver. Each lease has
/// at most one outstanding callback batch. The checkpoint policy controls when
/// successful callbacks become durable; replay remains possible after failures
/// or ownership changes.
///
/// Dropping the processor requests shutdown. Use [`stop()`](Self::stop) to wait
/// for shutdown. Monitoring through [`current_state()`](Self::current_state)
/// is not yet implemented.
#[must_use]
pub struct ChangeFeedProcessor<T, H> {
    configuration: ChangeFeedProcessorBuilder<T, H>,
    running: async_lock::Mutex<Option<execution::Running>>,
    started: Arc<AtomicBool>,
}

impl<T, H> ChangeFeedProcessor<T, H> {
    /// Returns whether this processor instance is running.
    ///
    /// This reports local coordination, not proof of lease ownership or current
    /// service connectivity. Bootstrap alone does not make the processor running.
    pub fn is_started(&self) -> bool {
        self.started.load(Ordering::Acquire)
    }

    /// Requests monitoring information for the logical processor's leases.
    ///
    /// The result is intended to include leases owned by other instances and
    /// unowned leases, not just those assigned to this instance. Monitoring is
    /// not yet implemented: this method performs no I/O and returns an error
    /// rather than an empty or fabricated snapshot.
    ///
    /// # Errors
    ///
    /// Always returns a local error with [`StatusCode::NotImplemented`].
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_data_cosmos::ChangeFeedProcessor;
    ///
    /// async fn inspect<T, H>(
    ///     processor: &ChangeFeedProcessor<T, H>,
    /// ) -> azure_data_cosmos::Result<()> {
    ///     println!("Running: {}", processor.is_started());
    ///     // Currently returns a not-implemented error.
    ///     for lease in processor.current_state().await? {
    ///         println!(
    ///             "Lease: {}, owner: {:?}, estimated lag: {:?}",
    ///             lease.lease_token(), lease.host_name(), lease.estimated_lag(),
    ///         );
    ///         let _saved_progress = lease.continuation_token();
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn current_state(&self) -> crate::Result<Vec<ChangeFeedProcessorState>> {
        Err(CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::NotImplemented))
            .with_message("Change feed processor monitoring is not implemented.")
            .build())
    }

    /// Bootstraps the workload before starting change feed processing.
    ///
    /// Performs network I/O to validate the workload's mode, coordinate
    /// initialization, and create missing initial leases without continuations.
    /// Existing checkpoints are preserved. The lease container must use `/id`
    /// partitioning and a default TTL of `-1`. Single-write and multi-write
    /// lease accounts are supported. Bootstrap leaves completed workloads untouched;
    /// optional restart verification runs afterward in the processing runtime.
    ///
    /// Compatible concurrent calls sharing the source and lease drivers await
    /// the same bootstrap attempts. Each caller retains its own two-minute
    /// budget; cancelling one waiter does not cancel the others.
    ///
    /// Starts background balancing, renewal, polling, and callback dispatch after
    /// bootstrap succeeds. Initial read settings apply only without saved progress.
    /// Lease maintenance is independent of feed polling and callback duration.
    /// Expiration uses a 60-second budget; renewal and acquisition schedules are
    /// derived from that budget. Callbacks have no implicit timeout.
    /// Throttled reads yield to other eligible leases, even with one batch poller.
    /// Each read turn is bounded to five seconds or a shorter configured driver
    /// deadline; this does not impose a callback timeout.
    ///
    /// # Errors
    ///
    /// Propagates bootstrap failures, including incompatible modes, invalid
    /// lease-container configuration, and service errors. Opt-in restart
    /// verification can also fail for invalid range plans or lease records.
    /// After startup, later per-lease failures are logged, reported to the optional
    /// [`error observer`](ChangeFeedProcessorBuilder::handle_error), and recovered
    /// independently. Starting an already-running instance returns
    /// [`StatusCode::Conflict`].
    pub async fn start<Fut>(&self) -> crate::Result<()>
    where
        T: DeserializeOwned + Send + 'static,
        H: Fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = crate::Result<()>> + Send + 'static,
    {
        execution::start(self).await
    }

    /// Drains issued batches, persists successful callbacks, and releases owned leases.
    ///
    /// Stops new acquisition and polling, but continues lease renewal while issued
    /// callbacks finish. There is no implicit timeout; a callback that never
    /// finishes prevents graceful shutdown. Use [`abort()`](Self::abort) to force
    /// cancellation. Canceling this wait leaves shutdown running.
    ///
    /// Concurrent calls await the same shutdown. Stopping an idle instance succeeds.
    /// Released leases may already have new owners when this method returns.
    ///
    /// # Errors
    ///
    /// Returns an error for incomplete draining, ownership loss, failed checkpoint
    /// or release operations, worker failure, or forced cancellation. Unsaved
    /// progress can replay; shutdown cannot undo external callback side effects.
    pub async fn stop(&self) -> crate::Result<()> {
        execution::stop(self, false).await
    }

    /// Cancels pending callback futures and waits for local lease cleanup.
    ///
    /// Already-started service writes can complete. External callback effects and
    /// separately spawned application tasks cannot be undone or canceled here.
    /// Unacknowledged batches remain replayable. A concurrent graceful stop reports
    /// that shutdown was forced.
    ///
    /// # Errors
    ///
    /// Returns worker or cleanup errors. Unreleased leases remain recoverable
    /// through expiration. Dropping this wait does not stop background cleanup.
    pub async fn abort(&self) -> crate::Result<()> {
        execution::stop(self, true).await
    }
}

#[cfg(all(test, feature = "__internal_in_memory_emulator"))]
mod tests;
