// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    feed::ContinuationToken, models::ChangeFeedItem, options::MaxItemCountHint, ContainerClient,
    CosmosError, CosmosStatus,
};
use azure_core::{fmt::SafeDebug, http::StatusCode};
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

/// Latest-version mode for a [`ChangeFeedProcessorBuilder`].
///
/// Pass this value to [`ChangeFeedProcessorBuilder::new()`].
#[derive(Clone, Copy, Debug)]
pub struct LatestVersion;

/// All-versions-and-deletes mode for a [`ChangeFeedProcessorBuilder`].
///
/// Pass this value to [`ChangeFeedProcessorBuilder::new()`].
/// This mode does not expose beginning or time-based start configuration.
#[derive(Clone, Copy, Debug)]
pub struct AllVersionsAndDeletes;

/// A supported change feed processor mode.
///
/// This trait is sealed and cannot be implemented by applications.
/// Select [`LatestVersion`] or [`AllVersionsAndDeletes`] when constructing a
/// [`ChangeFeedProcessorBuilder`].
pub trait ChangeFeedProcessorMode: private::Sealed {}

impl ChangeFeedProcessorMode for LatestVersion {}
impl ChangeFeedProcessorMode for AllVersionsAndDeletes {}

/// Context identifying the lease associated with a callback batch.
///
/// Passed by value to the callback registered with
/// [`ChangeFeedProcessorBuilder::handle_change_feed()`] in either mode.
/// Applications may retain the context across an await, but it is not proof
/// of current lease ownership or authorization to save a checkpoint.
#[derive(Clone, SafeDebug)]
pub struct ChangeFeedProcessorContext {
    lease_token: String,
}

impl ChangeFeedProcessorContext {
    /// Returns the opaque identifier of the lease associated with this batch.
    ///
    /// This is not a continuation token or a physical partition identifier.
    /// Treat it as meaningful within the processor's lease namespace.
    pub fn lease_token(&self) -> &str {
        &self.lease_token
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

mod private {
    use crate::{
        change_feed_processor::{AllVersionsAndDeletes, LatestVersion},
        options::ChangeFeedMode,
    };

    pub trait Sealed {
        fn mode(&self) -> ChangeFeedMode;
    }

    impl Sealed for LatestVersion {
        fn mode(&self) -> ChangeFeedMode {
            ChangeFeedMode::LatestVersion
        }
    }

    impl Sealed for AllVersionsAndDeletes {
        fn mode(&self) -> ChangeFeedMode {
            ChangeFeedMode::AllVersionsAndDeletes
        }
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
/// Pass [`LatestVersion`] or [`AllVersionsAndDeletes`] to [`new()`](Self::new)
/// to select a mode explicitly. There is no mode-free constructor or
/// mode-changing setter.
///
/// Register an async batch callback with [`handle_change_feed()`](Self::handle_change_feed),
/// then call [`build()`](Self::build) to transfer the configuration into a
/// [`ChangeFeedProcessor`]. The start
/// configuration applies to initialization without saved progress; it does not
/// configure a reset of existing checkpoints.
///
/// `M` identifies the mode, `T` the deserialized item type, and `H` the callback
/// type. Before registration, `T` and `H` default to `()`. Registering a callback
/// infers both types and makes [`build()`](Self::build) available.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos::{ChangeFeedProcessorBuilder, ContainerClient, LatestVersion};
///
/// fn configure(
///     monitored_container: ContainerClient,
///     lease_container: ContainerClient,
/// ) -> ChangeFeedProcessorBuilder<LatestVersion> {
///     ChangeFeedProcessorBuilder::new(
///         LatestVersion,
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
///     AllVersionsAndDeletes, ChangeFeedProcessorBuilder, ContainerClient,
/// };
///
/// fn configure(
///     monitored_container: ContainerClient,
///     lease_container: ContainerClient,
/// ) -> ChangeFeedProcessorBuilder<AllVersionsAndDeletes> {
///     ChangeFeedProcessorBuilder::new(
///         AllVersionsAndDeletes,
///         monitored_container,
///         lease_container,
///         "order-audit",
///         "worker-01",
///     )
/// }
/// ```
///
/// Beginning-based starts are unavailable in all-versions-and-deletes mode:
///
/// ```compile_fail,E0599
/// use azure_data_cosmos::{AllVersionsAndDeletes, ChangeFeedProcessorBuilder};
///
/// fn configure(builder: ChangeFeedProcessorBuilder<AllVersionsAndDeletes>) {
///     builder.with_start_from_beginning();
/// }
/// ```
///
/// Time-based starts are also unavailable in that mode:
///
/// ```compile_fail,E0599
/// use azure_data_cosmos::{AllVersionsAndDeletes, ChangeFeedProcessorBuilder};
/// use time::OffsetDateTime;
///
/// fn configure(builder: ChangeFeedProcessorBuilder<AllVersionsAndDeletes>) {
///     builder.with_start_time(OffsetDateTime::UNIX_EPOCH);
/// }
/// ```
///
/// Construction requires both clients and both names:
///
/// ```compile_fail,E0061
/// use azure_data_cosmos::{ChangeFeedProcessorBuilder, LatestVersion};
///
/// let _builder = ChangeFeedProcessorBuilder::new(LatestVersion);
/// ```
///
/// Construction without selecting a mode is unavailable:
///
/// ```compile_fail,E0061
/// use azure_data_cosmos::{ChangeFeedProcessorBuilder, ContainerClient, LatestVersion};
///
/// fn configure(monitored_container: ContainerClient, lease_container: ContainerClient) {
///     let _builder = ChangeFeedProcessorBuilder::<LatestVersion>::new(
///         monitored_container, lease_container, "orders", "worker-01",
///     );
/// }
/// ```
///
/// Applications cannot construct builders for their own mode types:
///
/// ```compile_fail,E0277
/// use azure_data_cosmos::ChangeFeedProcessorMode;
///
/// struct CustomMode;
///
/// impl ChangeFeedProcessorMode for CustomMode {}
/// ```
///
/// The runtime mode enum cannot replace the typed mode argument:
///
/// ```compile_fail,E0277
/// use azure_data_cosmos::{ChangeFeedProcessorBuilder, ContainerClient};
/// use azure_data_cosmos::options::ChangeFeedMode;
///
/// fn configure(monitored_container: ContainerClient, lease_container: ContainerClient) {
///     let _builder = ChangeFeedProcessorBuilder::new(
///         ChangeFeedMode::LatestVersion,
///         monitored_container,
///         lease_container,
///         "orders",
///         "worker-01",
///     );
/// }
/// ```
#[must_use]
pub struct ChangeFeedProcessorBuilder<M: ChangeFeedProcessorMode, T = (), H = ()> {
    monitored_container: ContainerClient,
    lease_container: ContainerClient,
    processor_name: String,
    instance_name: String,
    mode: M,
    start_from: ProcessorStartFrom,
    max_item_count: MaxItemCountHint,
    feed_poll_delay: Duration,
    min_scale_count: u32,
    max_scale_count: u32,
    lease_verification_on_restart_enabled: bool,
    handler: Arc<H>,
    item: PhantomData<fn() -> T>,
}

impl<M: ChangeFeedProcessorMode> ChangeFeedProcessorBuilder<M> {
    /// Creates a builder with an explicit mode and the required clients and names.
    ///
    /// Pass [`LatestVersion`] or [`AllVersionsAndDeletes`] as `mode`.
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
    /// Only latest-version builders expose beginning and time-based start configuration.
    /// Lease identifiers are constructed when the processor starts.
    pub fn new(
        mode: M,
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
            min_scale_count: 0,
            max_scale_count: 0,
            lease_verification_on_restart_enabled: false,
            handler: Arc::new(()),
            item: PhantomData,
        }
    }

    /// Registers an async callback that receives owned context and a batch of changes.
    ///
    /// The first argument is [`ChangeFeedProcessorContext`], identifying the batch's
    /// lease. The second is an owned vector of [`ChangeFeedItem<T>`] envelopes in
    /// either mode. The callback returns a
    /// future resolving to [`crate::Result<()>`]. It must support repeated calls
    /// through a shared reference, be [`Send`] and [`Sync`], and own its captures
    /// or borrow only `'static` data. Each returned future must also be [`Send`]
    /// and own its state or borrow only `'static` data.
    ///
    /// Consumes the builder and makes [`build()`](Self::build) available. A
    /// callback can be registered only once. Callbacks for different leases can
    /// overlap. For each lease, a successful callback is followed by a durable
    /// checkpoint before the next batch is read. Failures can cause replay;
    /// callbacks should make their external effects idempotent.
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_data_cosmos::{
    ///     models::ChangeFeedItem, ChangeFeedProcessorBuilder, ChangeFeedProcessorContext,
    ///     LatestVersion,
    /// };
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct Order {
    ///     id: String,
    /// }
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder<LatestVersion>) -> azure_data_cosmos::Result<()> {
    ///     let _processor = builder
    ///         .handle_change_feed(|context: ChangeFeedProcessorContext, changes: Vec<ChangeFeedItem<Order>>| async move {
    ///             println!("Lease: {}", context.lease_token());
    ///             for change in changes {
    ///                 if let Some(order) = change.current() {
    ///                     println!("Changed order: {}", order.id);
    ///                 }
    ///             }
    ///             Ok(())
    ///         })
    ///         .build()?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// A synchronous callback is not accepted:
    ///
    /// ```compile_fail,E0277
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder, LatestVersion};
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder<LatestVersion>) {
    ///     let _builder = builder.handle_change_feed(|_context, _: Vec<ChangeFeedItem<String>>| {
    ///         Ok::<(), azure_data_cosmos::CosmosError>(())
    ///     });
    /// }
    /// ```
    ///
    /// The callback's future must return a result:
    ///
    /// ```compile_fail,E0271
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder, LatestVersion};
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder<LatestVersion>) {
    ///     let _builder = builder.handle_change_feed(|_context, _: Vec<ChangeFeedItem<String>>| async {});
    /// }
    /// ```
    ///
    /// Item types must support deserialization:
    ///
    /// ```compile_fail,E0277
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder, LatestVersion};
    ///
    /// struct NotDeserializable;
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder<LatestVersion>) {
    ///     let _builder = builder
    ///         .handle_change_feed(|_context, _: Vec<ChangeFeedItem<NotDeserializable>>| async { Ok(()) });
    /// }
    /// ```
    ///
    /// Registration does not enable beginning-based starts in all-versions-and-deletes mode:
    ///
    /// ```compile_fail,E0599
    /// use azure_data_cosmos::{
    ///     models::ChangeFeedItem, AllVersionsAndDeletes, ChangeFeedProcessorBuilder,
    /// };
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder<AllVersionsAndDeletes>) {
    ///     let _builder = builder
    ///         .handle_change_feed(|_context, _: Vec<ChangeFeedItem<String>>| async { Ok(()) })
    ///         .with_start_from_beginning();
    /// }
    /// ```
    ///
    /// Time-based starts also remain unavailable in that mode:
    ///
    /// ```compile_fail,E0599
    /// use azure_data_cosmos::{
    ///     models::ChangeFeedItem, AllVersionsAndDeletes, ChangeFeedProcessorBuilder,
    /// };
    /// use time::OffsetDateTime;
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder<AllVersionsAndDeletes>) {
    ///     let _builder = builder
    ///         .handle_change_feed(|_context, _: Vec<ChangeFeedItem<String>>| async { Ok(()) })
    ///         .with_start_time(OffsetDateTime::UNIX_EPOCH);
    /// }
    /// ```
    ///
    /// The callback must accept both the context and the batch:
    ///
    /// ```compile_fail,E0593
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder, LatestVersion};
    ///
    /// fn configure(builder: ChangeFeedProcessorBuilder<LatestVersion>) {
    ///     let _builder = builder
    ///         .handle_change_feed(|_: Vec<ChangeFeedItem<String>>| async { Ok(()) });
    /// }
    /// ```
    pub fn handle_change_feed<T, H, Fut>(self, handler: H) -> ChangeFeedProcessorBuilder<M, T, H>
    where
        T: DeserializeOwned + Send + 'static,
        H: Fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut + Send + Sync + 'static,
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
            min_scale_count: self.min_scale_count,
            max_scale_count: self.max_scale_count,
            lease_verification_on_restart_enabled: self.lease_verification_on_restart_enabled,
            handler: Arc::new(handler),
            item: PhantomData,
        }
    }
}

impl<M: ChangeFeedProcessorMode, T, H> ChangeFeedProcessorBuilder<M, T, H> {
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
    ///     ChangeFeedProcessorBuilder, ChangeFeedProcessorMode,
    /// };
    /// use std::{num::NonZeroU32, time::Duration};
    ///
    /// fn configure<M: ChangeFeedProcessorMode>(
    ///     builder: ChangeFeedProcessorBuilder<M>,
    ///     initial_continuation: ContinuationToken,
    /// ) -> azure_data_cosmos::Result<()> {
    ///     let _processor = builder
    ///         .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(200).unwrap()))
    ///         .with_feed_poll_delay(Duration::from_secs(2))
    ///         .with_start_continuation(initial_continuation)
    ///         .with_min_scale_count(1)
    ///         .with_max_scale_count(4)
    ///         .with_lease_verification_enabled_on_restart(true)
    ///         .handle_change_feed(|_context, _: Vec<ChangeFeedItem<String>>| async { Ok(()) })
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
    /// Replaces any previously configured initial start position. In latest-version
    /// mode, a subsequent start-position setter replaces this token in turn.
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

    /// Configures whether to check for missing leases when the processor restarts.
    ///
    /// Disabled by default. This is a safeguard for externally modified lease
    /// records. Existing owners and continuations are preserved.
    pub fn with_lease_verification_enabled_on_restart(mut self, enabled: bool) -> Self {
        self.lease_verification_on_restart_enabled = enabled;
        self
    }
}

impl<M, T, H, Fut> ChangeFeedProcessorBuilder<M, T, H>
where
    M: ChangeFeedProcessorMode,
    T: DeserializeOwned + Send + 'static,
    H: Fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut + Send + Sync + 'static,
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
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder, LatestVersion};
    ///
    /// fn create(builder: ChangeFeedProcessorBuilder<LatestVersion>) -> azure_data_cosmos::Result<()> {
    ///     let _processor = builder
    ///         .handle_change_feed(|_context, _: Vec<ChangeFeedItem<String>>| async { Ok(()) })
    ///         .with_start_from_now()
    ///         .build()?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// A callback is required before building:
    ///
    /// ```compile_fail,E0599
    /// use azure_data_cosmos::{ChangeFeedProcessorBuilder, LatestVersion};
    ///
    /// fn create(builder: ChangeFeedProcessorBuilder<LatestVersion>) {
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
    /// use azure_data_cosmos::{models::ChangeFeedItem, ChangeFeedProcessorBuilder, LatestVersion};
    ///
    /// fn create(builder: ChangeFeedProcessorBuilder<LatestVersion>) {
    ///     let builder = builder
    ///         .handle_change_feed(|_context, _: Vec<ChangeFeedItem<String>>| async { Ok(()) });
    ///     let _processor = builder.build();
    ///     let _another = builder.build();
    /// }
    /// ```
    pub fn build(self) -> crate::Result<ChangeFeedProcessor<M, T, H>> {
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
        Ok(ChangeFeedProcessor {
            configuration: self,
            running: async_lock::Mutex::new(None),
            started: Arc::new(AtomicBool::new(false)),
        })
    }
}

impl<T, H> ChangeFeedProcessorBuilder<LatestVersion, T, H> {
    /// Configures reading from the beginning when no saved progress exists.
    ///
    /// Replaces any previously configured start position.
    pub fn with_start_from_beginning(mut self) -> Self {
        self.start_from = ProcessorStartFrom::Beginning;
        self
    }

    /// Configures reading after `start_time` when no saved progress exists.
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
/// at most one outstanding callback batch. Checkpoints follow successful callbacks;
/// replay remains possible after failures or ownership changes.
///
/// Dropping the processor requests shutdown. Use [`stop()`](Self::stop) to wait
/// for shutdown. Monitoring through [`current_state()`](Self::current_state)
/// is not yet implemented.
#[must_use]
pub struct ChangeFeedProcessor<M: ChangeFeedProcessorMode, T, H> {
    configuration: ChangeFeedProcessorBuilder<M, T, H>,
    running: async_lock::Mutex<Option<execution::Running>>,
    started: Arc<AtomicBool>,
}

impl<M: ChangeFeedProcessorMode, T, H> ChangeFeedProcessor<M, T, H> {
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
    /// use azure_data_cosmos::{ChangeFeedProcessor, ChangeFeedProcessorMode};
    ///
    /// async fn inspect<M: ChangeFeedProcessorMode, T, H>(
    ///     processor: &ChangeFeedProcessor<M, T, H>,
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
    /// partitioning and a default TTL of `-1` in a single-write-region account.
    ///
    /// Starts background balancing, renewal, polling, and callback dispatch after
    /// bootstrap succeeds. Initial read settings apply only without saved progress.
    /// The progress watchdog abandons a lease after 425 seconds without a successful
    /// feed response, including time spent waiting for its callback.
    ///
    /// # Errors
    ///
    /// Propagates bootstrap failures, including incompatible modes, invalid
    /// lease-container configuration, and service errors. After successful
    /// bootstrap, starts coordination; later per-lease failures are logged and
    /// recovered independently. Starting an already-running instance returns
    /// [`StatusCode::Conflict`].
    pub async fn start<Fut>(&self) -> crate::Result<()>
    where
        T: DeserializeOwned + Send + 'static,
        H: Fn(ChangeFeedProcessorContext, Vec<ChangeFeedItem<T>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = crate::Result<()>> + Send + 'static,
    {
        execution::start(self).await
    }

    /// Requests that change feed processing stop.
    ///
    /// Cancels pending callbacks, waits for local tasks, and attempts conditional
    /// lease release. Callback cancellation cannot undo external side effects.
    /// An unacknowledged batch can be replayed. Stopping an idle instance succeeds.
    ///
    /// # Errors
    ///
    /// Returns an error if the callback-dispatch task fails. Lease release failures
    /// are logged; other workers can recover those leases after expiration.
    pub async fn stop(&self) -> crate::Result<()> {
        execution::stop(self).await
    }
}

#[cfg(all(test, feature = "__internal_in_memory_emulator"))]
mod tests;
