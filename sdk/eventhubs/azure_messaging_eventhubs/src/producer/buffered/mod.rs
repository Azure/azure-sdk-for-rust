// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell: ignore retryable

//! A producer client that buffers events and publishes them in the background.

pub(crate) mod partition_resolver;
pub(crate) mod send_client;
pub(crate) mod worker;

use crate::{
    error::Result,
    models::EventData,
    producer::{
        buffered::{
            partition_resolver::PartitionResolver,
            send_client::BufferedSendClient,
            worker::{Command, PartitionWorker},
        },
        ProducerClient,
    },
    EventHubsError,
};
use async_lock::Semaphore;
use azure_core::{
    async_runtime::{get_async_runtime, SpawnedTask},
    fmt::SafeDebug,
    time::Duration,
    Uuid,
};
use azure_core_amqp::{AmqpMessage, AmqpSymbol};
use futures::{
    channel::{mpsc, oneshot},
    future::{BoxFuture, Shared},
    FutureExt,
};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
};
use tracing::{debug, trace, warn};

/// The default maximum time that the client waits before it sends a batch that
/// is not full.
///
/// This matches the .NET, JavaScript, and Python clients.
pub(crate) const DEFAULT_MAX_WAIT_TIME_SECONDS: i64 = 1;

/// The default maximum number of events that the client buffers for one partition.
///
/// This matches the .NET, Java, JavaScript, and Python clients.
pub(crate) const DEFAULT_MAX_BUFFERED_EVENT_COUNT_PER_PARTITION: usize = 1500;

/// Options for [`BufferedProducerClient::enqueue_event`] and
/// [`BufferedProducerClient::enqueue_events`].
///
/// Set at most one of `partition_id` and `partition_key`. The client rejects a
/// request that sets both. When neither is set, the client assigns a partition
/// in round-robin order.
///
/// [`BufferedProducerClient::enqueue_event`]: crate::BufferedProducerClient::enqueue_event
/// [`BufferedProducerClient::enqueue_events`]: crate::BufferedProducerClient::enqueue_events
///
/// # Examples
///
/// ```
/// use azure_messaging_eventhubs::EnqueueEventOptions;
///
/// let to_partition = EnqueueEventOptions {
///     partition_id: Some("0".to_string()),
///     ..Default::default()
/// };
///
/// let by_key = EnqueueEventOptions {
///     partition_key: Some("customer-17".to_string()),
///     ..Default::default()
/// };
/// ```
// This type stays constructible with a struct expression, like the other option
// types in this crate (`SendEventOptions`, `EventDataBatchOptions`). Callers use
// `..Default::default()`, so a new field later does not break them.
#[derive(Default, Clone, SafeDebug)]
pub struct EnqueueEventOptions {
    /// The ID of the partition that receives the event.
    ///
    /// The partition ID must be one of the partitions that the client read when
    /// it opened.
    pub partition_id: Option<String>,

    /// The partition key that selects the partition for the event.
    ///
    /// Events with the same partition key go to the same partition. The client
    /// keeps the key on the event, so the service also sees it.
    pub partition_key: Option<String>,
}

/// Reports that the service accepted a batch of events.
///
/// The client passes this to the handler that
/// [`BufferedProducerClientBuilder::with_on_send_succeeded`] registers.
///
/// [`BufferedProducerClientBuilder::with_on_send_succeeded`]: crate::builders::BufferedProducerClientBuilder::with_on_send_succeeded
#[derive(SafeDebug)]
#[non_exhaustive]
pub struct SendBatchSucceededContext {
    /// The ID of the partition that received the events.
    pub partition_id: String,

    /// The events in the batch, in the order that the caller enqueued them.
    pub events: Vec<EventData>,
}

/// Reports that the service did not durably accept a batch of events.
///
/// The client passes this to the handler that
/// [`BufferedProducerClientBuilder::with_on_send_failed`] registers. The client
/// reports a failure only after the retry policy is exhausted, or when the
/// error is not retryable.
///
/// A failure does not always mean that the batch never reached the service. An
/// AMQP `Modified` or `Released` outcome settles the transfer without a durable
/// accept, and neither outcome proves whether the service stored the events.
///
/// The client does not enqueue the events again. Re-enqueueing can change the
/// order of events, and it can store an event two times. The events are in this
/// context, so the application decides what to do with them.
///
/// [`BufferedProducerClientBuilder::with_on_send_failed`]: crate::builders::BufferedProducerClientBuilder::with_on_send_failed
#[derive(SafeDebug)]
#[non_exhaustive]
pub struct SendBatchFailedContext {
    /// The ID of the partition that the client tried to send to.
    pub partition_id: String,

    /// The events in the batch, in the order that the caller enqueued them.
    pub events: Vec<EventData>,

    /// The error that stopped the batch.
    pub error: EventHubsError,
}

/// A handler that the client calls after the service accepts a batch.
pub(crate) type SucceededHandler =
    Arc<dyn Fn(SendBatchSucceededContext) -> BoxFuture<'static, ()> + Send + Sync>;

/// A handler that the client calls after a batch fails for the last time.
pub(crate) type FailedHandler =
    Arc<dyn Fn(SendBatchFailedContext) -> BoxFuture<'static, ()> + Send + Sync>;

/// The handlers that a partition worker reports outcomes to.
#[derive(Clone)]
pub(crate) struct DeliveryHandlers {
    pub(crate) succeeded: Option<SucceededHandler>,
    pub(crate) failed: FailedHandler,
}

/// The per-partition state that the client owns.
struct PartitionState {
    /// The queue that feeds the worker.
    ///
    /// A close takes the sender out. That ends the queue, which tells the worker
    /// to drain and stop. An enqueue waits for capacity before taking this
    /// lock, then holds it through accounting and the send.
    sender: Mutex<Option<mpsc::UnboundedSender<Command>>>,

    /// The capacity of the partition buffer.
    ///
    /// One permit stands for one event that the client accepted but has not
    /// finished. The permit returns once the event reaches a terminal outcome.
    /// An enqueue that finds no permit waits, so the buffer never grows without
    /// a bound.
    capacity: Arc<Semaphore>,

    /// The number of events for this partition that have no terminal outcome yet.
    buffered: Arc<AtomicUsize>,

    /// The worker task.
    task: Mutex<Option<SpawnedTask>>,

    /// Resolves once the worker stopped.
    ///
    /// The worker completes this channel when it leaves its loop, and a runtime
    /// that drops the task cancels it. A close waits for it, so the client never
    /// reports that it closed while a worker still sends, still calls a delivery
    /// handler, or still holds the connection. `AbortableTask::abort` cannot
    /// carry that promise: on the standard thread runtime it detaches the thread
    /// and lets the await return at once.
    stopped: Mutex<Option<oneshot::Receiver<()>>>,
}

/// A producer client that buffers events and publishes them in the background.
///
/// The caller enqueues single events. The client groups them into batches for
/// each partition, and it publishes each batch from a background worker. This
/// gives a higher throughput than [`ProducerClient`], because the caller does
/// not wait for each send.
///
/// # Enqueue does not mean delivery
///
/// A successful enqueue means only that the client accepted the event into the
/// local buffer. It does not mean that Event Hubs accepted the event. The
/// client reports the real outcome later, through the handlers that the builder
/// registers.
///
/// The application must handle these trade-offs:
///
/// * The process loses buffered events if it stops before a flush or a close.
/// * A send failure arrives after the enqueue call already returned.
/// * Buffering gives a higher throughput, but the latency of one event is less
///   predictable.
/// * Use [`ProducerClient`] when the application needs the result of each send.
///
/// # The client reads the partition list one time
///
/// The client reads the partition IDs when it opens, and it does not read them
/// again. A partition that the service adds later stays unused until the
/// application opens a new client. This limitation affects the throughput. It
/// does not affect the correctness, because the client still publishes every
/// event to a partition that exists.
///
/// # Examples
///
/// ```no_run
/// use azure_messaging_eventhubs::BufferedProducerClient;
/// use azure_identity::DeveloperToolsCredential;
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
///     let namespace = std::env::var("EVENT_HUB_NAMESPACE")?;
///     let eventhub = std::env::var("EVENT_HUB_NAME")?;
///     let credential = DeveloperToolsCredential::new(None)?;
///
///     let producer = BufferedProducerClient::builder()
///         .with_on_send_failed(|context| async move {
///             eprintln!(
///                 "{} events failed on partition {}: {}",
///                 context.events.len(),
///                 context.partition_id,
///                 context.error
///             );
///         })
///         .open(&namespace, &eventhub, credential.clone())
///         .await?;
///
///     producer.enqueue_event("Hello, Event Hubs!", None).await?;
///     producer.flush().await?;
///     producer.close().await?;
///     Ok(())
/// }
/// ```
pub struct BufferedProducerClient {
    /// The underlying client. Tests build the buffered client over a mock
    /// instead, so this is `None` in those tests.
    producer: Mutex<Option<Arc<ProducerClient>>>,

    resolver: PartitionResolver,
    partitions: HashMap<String, PartitionState>,
    total_buffered: Arc<AtomicUsize>,
    closed: AtomicBool,
    abandon: Arc<AtomicBool>,
    close_signal: Mutex<Option<oneshot::Sender<()>>>,
    closing: Shared<oneshot::Receiver<()>>,
    next_flush_id: AtomicUsize,

    #[cfg(test)]
    enqueue_hook: Mutex<Option<Box<dyn FnOnce() + Send>>>,
}

impl BufferedProducerClient {
    /// Returns a builder that creates a [`BufferedProducerClient`].
    pub fn builder() -> builders::BufferedProducerClientBuilder {
        builders::BufferedProducerClientBuilder::new()
    }

    /// Builds the client and starts one worker for each partition.
    async fn start(
        producer: Option<Arc<ProducerClient>>,
        send_client: Arc<dyn BufferedSendClient>,
        max_wait_time: Duration,
        max_buffered_event_count_per_partition: usize,
        handlers: DeliveryHandlers,
    ) -> Result<Self> {
        let partition_ids = send_client.partition_ids().await?;
        if partition_ids.is_empty() {
            return Err(EventHubsError::with_message(
                "The Event Hub reported no partitions.",
            ));
        }

        let total_buffered = Arc::new(AtomicUsize::new(0));
        let abandon = Arc::new(AtomicBool::new(false));
        let (close_signal, closing) = oneshot::channel();

        let mut partitions = HashMap::with_capacity(partition_ids.len());
        for partition_id in &partition_ids {
            let (sender, receiver) = mpsc::unbounded();
            let buffered = Arc::new(AtomicUsize::new(0));
            let capacity = Arc::new(Semaphore::new(max_buffered_event_count_per_partition));

            let (stopped_sender, stopped) = oneshot::channel();
            let worker = PartitionWorker::new(
                partition_id.clone(),
                receiver,
                send_client.clone(),
                max_wait_time,
                max_buffered_event_count_per_partition,
                handlers.clone(),
                buffered.clone(),
                total_buffered.clone(),
                abandon.clone(),
                stopped_sender,
            );

            let task = get_async_runtime().spawn(Box::pin(worker.run()));

            partitions.insert(
                partition_id.clone(),
                PartitionState {
                    sender: Mutex::new(Some(sender)),
                    capacity,
                    buffered,
                    task: Mutex::new(Some(task)),
                    stopped: Mutex::new(Some(stopped)),
                },
            );
        }

        debug!(
            partition_count = partition_ids.len(),
            buffered_event_count = max_buffered_event_count_per_partition,
            "Buffered producer client started."
        );

        Ok(Self {
            producer: Mutex::new(producer),
            resolver: PartitionResolver::new(partition_ids),
            partitions,
            total_buffered,
            closed: AtomicBool::new(false),
            abandon,
            close_signal: Mutex::new(Some(close_signal)),
            closing: closing.shared(),
            next_flush_id: AtomicUsize::new(0),
            #[cfg(test)]
            enqueue_hook: Mutex::new(None),
        })
    }

    /// Adds one event to the buffer.
    ///
    /// The call returns once the client accepts the event into the local
    /// buffer. It does not wait for Event Hubs to accept the event. The client
    /// reports the delivery outcome through the registered handlers.
    ///
    /// When the buffer for the target partition is full, the call waits for
    /// space. A close of the client makes a waiting call return an error.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to add to the buffer.
    /// * `options` - The routing options for the event.
    ///
    /// # Returns
    ///
    /// A `Result` that shows whether the client accepted the event.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use azure_messaging_eventhubs::{BufferedProducerClient, EnqueueEventOptions};
    /// # async fn example(producer: BufferedProducerClient) -> Result<(), Box<dyn std::error::Error>> {
    /// producer.enqueue_event("Hello, Event Hubs!", None).await?;
    ///
    /// producer
    ///     .enqueue_event(
    ///         "For one partition",
    ///         Some(EnqueueEventOptions {
    ///             partition_id: Some("0".to_string()),
    ///             ..Default::default()
    ///         }),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn enqueue_event(
        &self,
        event: impl Into<EventData>,
        options: Option<EnqueueEventOptions>,
    ) -> Result<()> {
        let options = options.unwrap_or_default();
        if options.partition_id.is_some() && options.partition_key.is_some() {
            return Err(EventHubsError::with_message(
                "Set a partition ID or a partition key, not both.",
            ));
        }
        self.enqueue_one(event.into(), &options).await
    }

    /// Adds several events to the buffer.
    ///
    /// The client adds the events one at a time, in order. The call stops at the
    /// first event that it cannot accept, and it returns that error. The client
    /// keeps the events that it already accepted.
    ///
    /// # Arguments
    ///
    /// * `events` - The events to add to the buffer.
    /// * `options` - The routing options for every event in the call.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use azure_messaging_eventhubs::BufferedProducerClient;
    /// # async fn example(producer: BufferedProducerClient) -> Result<(), Box<dyn std::error::Error>> {
    /// producer
    ///     .enqueue_events(vec!["first", "second", "third"], None)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn enqueue_events<E>(
        &self,
        events: impl IntoIterator<Item = E>,
        options: Option<EnqueueEventOptions>,
    ) -> Result<()>
    where
        E: Into<EventData>,
    {
        let options = options.unwrap_or_default();
        if options.partition_id.is_some() && options.partition_key.is_some() {
            return Err(EventHubsError::with_message(
                "Set a partition ID or a partition key, not both.",
            ));
        }
        for event in events {
            self.enqueue_one(event.into(), &options).await?;
        }
        Ok(())
    }

    async fn enqueue_one(&self, event: EventData, options: &EnqueueEventOptions) -> Result<()> {
        if self.closed.load(Ordering::Acquire) {
            return Err(Self::closed_error());
        }

        let partition_id = match (&options.partition_id, &options.partition_key) {
            (Some(partition_id), _) => {
                if !self.resolver.contains(partition_id) {
                    return Err(EventHubsError::with_message(format!(
                        "The Event Hub has no partition with the ID {partition_id}."
                    )));
                }
                partition_id.clone()
            }
            (None, Some(partition_key)) => self.resolver.assign_for_key(partition_key).to_string(),
            (None, None) => self.resolver.assign_round_robin().to_string(),
        };

        let mut message = AmqpMessage::from(event.clone());
        if message.properties.is_none() || message.properties.as_ref().unwrap().message_id.is_none()
        {
            message.set_message_id(Uuid::new_v4());
        }
        if let Some(partition_key) = options.partition_key.as_ref() {
            // Keep the key on the message so the service sees it too.
            message.add_message_annotation(
                AmqpSymbol::from("x-opt-partition-key"),
                partition_key.clone(),
            );
        }

        let state = self
            .partitions
            .get(&partition_id)
            .expect("the resolver only returns known partitions");

        // Take one unit of buffer capacity. The permit travels with the event and
        // returns once the event reaches a terminal outcome.
        let permit = match state.capacity.try_acquire_arc() {
            Some(permit) => permit,
            None => {
                debug!(
                    partition_id = %partition_id,
                    buffered_event_count = state.buffered.load(Ordering::Acquire),
                    "The partition buffer is full; the enqueue is waiting for space."
                );
                let mut acquire = Box::pin(state.capacity.acquire_arc()).fuse();
                let mut closing = self.closing.clone().fuse();
                futures::select! {
                    permit = acquire => permit,
                    _ = closing => return Err(Self::closed_error()),
                }
            }
        };

        let command = Command::Event {
            event,
            message: Box::new(message),
            permit,
        };

        // Keep the sender lock through accounting, send, and rollback. Abort
        // takes this lock before it resets counts, so a failed send cannot
        // subtract from counts that abort already cleared. The capacity wait
        // above remains outside this lock.
        let sender = state.sender.lock().unwrap();
        let sender = sender.as_ref().ok_or_else(Self::closed_error)?;
        // Count the event before the worker can see it. The worker decrements
        // the counts as soon as the event reaches a terminal outcome, and a
        // fast terminal path (an oversized event with a handler that returns at
        // once) can run before this call returns. Counting afterwards lets that
        // decrement reach zero first and wrap the counts to `usize::MAX`.
        state.buffered.fetch_add(1, Ordering::AcqRel);
        self.total_buffered.fetch_add(1, Ordering::AcqRel);

        #[cfg(test)]
        if let Some(hook) = self.enqueue_hook.lock().unwrap().take() {
            hook();
        }

        if sender.unbounded_send(command).is_err() {
            // The worker never saw the event, so it never decrements for it.
            state.buffered.fetch_sub(1, Ordering::AcqRel);
            self.total_buffered.fetch_sub(1, Ordering::AcqRel);
            return Err(Self::closed_error());
        }

        trace!(
            partition_id = %partition_id,
            "The client accepted an event into the buffer."
        );
        Ok(())
    }

    /// Sends every event that the client accepted before this call.
    ///
    /// The call sets a barrier. It completes once every event in front of the
    /// barrier reaches a terminal outcome, either a success or a failure. An
    /// event that the caller enqueues after the barrier does not delay the call.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use azure_messaging_eventhubs::BufferedProducerClient;
    /// # async fn example(producer: BufferedProducerClient) -> Result<(), Box<dyn std::error::Error>> {
    /// producer.enqueue_event("Hello, Event Hubs!", None).await?;
    /// producer.flush().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn flush(&self) -> Result<()> {
        let flush_id = self.next_flush_id.fetch_add(1, Ordering::Relaxed);
        debug!(flush_id, "Flush started.");

        let mut waiters = Vec::with_capacity(self.partitions.len());
        for (partition_id, state) in &self.partitions {
            let (completed, waiter) = oneshot::channel();
            let sender = state.sender.lock().unwrap().as_ref().cloned();
            let sent = match sender {
                Some(sender) => sender.unbounded_send(Command::Flush(completed)).is_ok(),
                None => false,
            };
            if sent {
                waiters.push(waiter);
            } else {
                trace!(
                    flush_id,
                    partition_id = %partition_id,
                    "The partition worker already stopped; the flush skips it."
                );
            }
        }

        let mut cancelled = false;
        for waiter in waiters {
            if waiter.await.is_err() {
                cancelled = true;
            }
        }

        if cancelled {
            debug!(flush_id, "Flush stopped because the client is closing.");
            return Err(Self::closed_error());
        }

        debug!(flush_id, "Flush completed.");
        Ok(())
    }

    /// Returns the number of events that have no terminal outcome yet.
    ///
    /// The count covers every partition. It includes the events in the queues,
    /// the events in an active batch, and the events of a send that is in
    /// flight.
    pub fn total_buffered_event_count(&self) -> usize {
        self.total_buffered.load(Ordering::Acquire)
    }

    /// Returns the number of events for one partition that have no terminal
    /// outcome yet.
    ///
    /// The method returns 0 for a partition ID that the Event Hub does not have.
    pub fn buffered_event_count(&self, partition_id: &str) -> usize {
        self.partitions
            .get(partition_id)
            .map(|state| state.buffered.load(Ordering::Acquire))
            .unwrap_or(0)
    }

    /// Sends the buffered events, then closes the client.
    ///
    /// The client stops accepting new events, sends every event that it already
    /// accepted, stops the workers, and releases the connection.
    ///
    /// Use [`abort`](Self::abort) to close without sending the buffered events.
    ///
    /// The method takes `&self`, so an application can hold the client in an
    /// `Arc`, enqueue from many tasks, and close it from one of them. A second
    /// call does nothing and returns `Ok`.
    pub async fn close(&self) -> Result<()> {
        self.shutdown(false).await
    }

    /// Closes the client at once and abandons the buffered events.
    ///
    /// The client drops every event that it did not send yet. It reports the
    /// number of abandoned events in a warning, and it removes them from the
    /// buffered counts.
    ///
    /// The call waits for every worker to stop, so it does not return while a
    /// worker still publishes, still calls a delivery handler, or still holds
    /// the connection. A worker that is inside a send when the call starts
    /// finishes that send first on a runtime that cannot cancel a task, so the
    /// call can take as long as one send. The retry policy bounds that send. No
    /// batch that the client has not started to send goes to the service.
    ///
    /// A second call does nothing and returns `Ok`.
    pub async fn abort(&self) -> Result<()> {
        self.shutdown(true).await
    }

    async fn shutdown(&self, abandon: bool) -> Result<()> {
        if self.closed.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        if abandon {
            self.abandon.store(true, Ordering::Release);
        }

        debug!(
            abandon,
            buffered_event_count = self.total_buffered.load(Ordering::Acquire),
            "Closing the buffered producer client."
        );

        // Fail every enqueue that is waiting for space.
        self.signal_closing();

        // Taking the sender ends each queue. A worker then sends what it still
        // holds and stops, unless the client abandons the events. The abandon
        // flag is already set above, so a worker that takes this path drops its
        // events instead of sending them.
        let mut tasks = Vec::with_capacity(self.partitions.len());
        let mut acknowledgements = Vec::with_capacity(self.partitions.len());
        for state in self.partitions.values() {
            drop(state.sender.lock().unwrap().take());
            if let Some(stopped) = state.stopped.lock().unwrap().take() {
                acknowledgements.push(stopped);
            }
            if let Some(task) = state.task.lock().unwrap().take() {
                if abandon {
                    // On a runtime with cancellation this ends an in-flight send
                    // at once. On the standard thread runtime it only detaches
                    // the thread, so the acknowledgement below, not this call, is
                    // what makes the close wait for the worker.
                    task.abort();
                }
                tasks.push(task);
            }
        }

        // Wait for every worker to stop. A cancelled task drops its end of the
        // channel, which resolves the receiver with an error, so this waits for
        // the worker to finish or to be dropped, and never for both.
        for acknowledgement in acknowledgements {
            let _ = acknowledgement.await;
        }

        for task in tasks {
            if let Err(error) = task.await {
                debug!("A partition worker stopped with an error: {error}");
            }
        }

        if abandon {
            // A worker that the runtime stopped inside a send cannot clear its
            // own counters, so the client clears them here. The counts and the
            // abandoned events then agree.
            let abandoned = self.total_buffered.swap(0, Ordering::AcqRel);
            for state in self.partitions.values() {
                state.buffered.store(0, Ordering::Release);
            }
            if abandoned > 0 {
                warn!(
                    event_count = abandoned,
                    "Abandoned buffered events during an immediate close."
                );
            }
        }

        let producer = self.producer.lock().unwrap().take();
        if let Some(producer) = producer {
            match Arc::try_unwrap(producer) {
                Ok(producer) => producer.close().await?,
                Err(_) => {
                    // A worker thread that the runtime could not stop still
                    // holds a reference. Dropping this one closes the
                    // connection once that thread finishes.
                    warn!(
                        "Could not close the connection now, because another reference exists; \
                         it closes when the last reference drops."
                    );
                }
            }
        }

        debug!("Buffered producer client closed.");
        Ok(())
    }

    fn signal_closing(&self) {
        if let Some(signal) = self.close_signal.lock().unwrap().take() {
            let _ = signal.send(());
        }
    }

    fn closed_error() -> EventHubsError {
        EventHubsError::with_message("The buffered producer client is closed.")
    }

    /// Returns the underlying producer, so a test can force an error on the
    /// connection.
    #[cfg(test)]
    pub(crate) fn inner_producer(&self) -> Option<Arc<ProducerClient>> {
        self.producer.lock().unwrap().clone()
    }

    /// Installs a one-shot hook after enqueue accounting and before the send.
    #[cfg(test)]
    fn set_enqueue_hook(&self, hook: Box<dyn FnOnce() + Send>) {
        *self.enqueue_hook.lock().unwrap() = Some(hook);
    }
}

impl std::fmt::Debug for BufferedProducerClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferedProducerClient")
            .field("partition_count", &self.partitions.len())
            .field(
                "total_buffered_event_count",
                &self.total_buffered.load(Ordering::Acquire),
            )
            .field("closed", &self.closed.load(Ordering::Acquire))
            .finish()
    }
}

impl Drop for BufferedProducerClient {
    fn drop(&mut self) {
        if self.closed.load(Ordering::Acquire) {
            return;
        }

        warn!(
            buffered_event_count = self.total_buffered.load(Ordering::Acquire),
            "The buffered producer client was dropped without a close; \
             buffered events are abandoned. Call close or flush when delivery matters."
        );

        self.closed.store(true, Ordering::Release);
        self.abandon.store(true, Ordering::Release);
        self.signal_closing();

        // Stop the workers. Dropping the partition map also drops every sender,
        // which ends each queue. The abandon flag is set above, so a worker that
        // reaches the end of its queue drops its events instead of sending them,
        // whatever the runtime does with the abort below. A drop cannot wait for
        // the workers, so call `close` or `abort` when that matters.
        for state in self.partitions.values_mut() {
            if let Some(task) = state.task.lock().unwrap().take() {
                task.abort();
            }
        }
    }
}

/// Builders for the buffered producer client.
pub mod builders {
    use super::{
        BufferedProducerClient, DeliveryHandlers, FailedHandler, SendBatchFailedContext,
        SendBatchSucceededContext, SucceededHandler,
        DEFAULT_MAX_BUFFERED_EVENT_COUNT_PER_PARTITION, DEFAULT_MAX_WAIT_TIME_SECONDS,
    };
    use crate::{
        error::Result,
        producer::{buffered::send_client::ProducerSendClient, ProducerClient},
        EventHubsError, RetryOptions,
    };
    use azure_core::time::Duration;
    use futures::FutureExt;
    use std::{future::Future, sync::Arc};

    /// A builder that creates a [`BufferedProducerClient`].
    ///
    /// The builder needs a handler for failed batches. A buffered send reports
    /// its failure later, so a client with no failure handler would lose events
    /// without a report. Use
    /// [`with_on_send_failed`](Self::with_on_send_failed) before `open`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_messaging_eventhubs::BufferedProducerClient;
    /// use azure_identity::DeveloperToolsCredential;
    /// use azure_core::time::Duration;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let credential = DeveloperToolsCredential::new(None)?;
    ///     let producer = BufferedProducerClient::builder()
    ///         .with_max_wait_time(Duration::seconds(1))
    ///         .with_max_buffered_event_count_per_partition(1500)
    ///         .with_on_send_succeeded(|context| async move {
    ///             println!("sent {} events", context.events.len());
    ///         })
    ///         .with_on_send_failed(|context| async move {
    ///             eprintln!("failed {} events: {}", context.events.len(), context.error);
    ///         })
    ///         .open("my_namespace", "my_eventhub", credential)
    ///         .await?;
    ///     producer.close().await?;
    ///     Ok(())
    /// }
    /// ```
    pub struct BufferedProducerClientBuilder {
        application_id: Option<String>,
        retry_options: Option<RetryOptions>,
        custom_endpoint: Option<String>,
        max_wait_time: Duration,
        max_buffered_event_count_per_partition: usize,
        on_send_succeeded: Option<SucceededHandler>,
        on_send_failed: Option<FailedHandler>,
    }

    impl Default for BufferedProducerClientBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl BufferedProducerClientBuilder {
        pub(super) fn new() -> Self {
            Self {
                application_id: None,
                retry_options: None,
                custom_endpoint: None,
                max_wait_time: Duration::seconds(DEFAULT_MAX_WAIT_TIME_SECONDS),
                max_buffered_event_count_per_partition:
                    DEFAULT_MAX_BUFFERED_EVENT_COUNT_PER_PARTITION,
                on_send_succeeded: None,
                on_send_failed: None,
            }
        }

        /// Sets the application ID that identifies the client.
        pub fn with_application_id(mut self, application_id: String) -> Self {
            self.application_id = Some(application_id);
            self
        }

        /// Sets the options that configure retry operations.
        pub fn with_retry_options(mut self, retry_options: RetryOptions) -> Self {
            self.retry_options = Some(retry_options);
            self
        }

        /// Sets a custom endpoint for the Event Hub.
        pub fn with_custom_endpoint(mut self, endpoint: String) -> Self {
            self.custom_endpoint = Some(endpoint);
            self
        }

        /// Sets how long the client waits before it sends a batch that is not full.
        ///
        /// A short time lowers the latency of one event. A long time makes the
        /// batches larger, which raises the throughput. The default is 1 second.
        pub fn with_max_wait_time(mut self, max_wait_time: Duration) -> Self {
            self.max_wait_time = max_wait_time;
            self
        }

        /// Sets how many events the client buffers for one partition.
        ///
        /// An enqueue waits for space once a partition buffer holds this many
        /// events. The default is 1500.
        pub fn with_max_buffered_event_count_per_partition(mut self, count: usize) -> Self {
            self.max_buffered_event_count_per_partition = count;
            self
        }

        /// Registers the handler that runs after the service accepts a batch.
        ///
        /// The handler is optional. The partition worker waits for the handler,
        /// so a slow handler slows only its own partition. The handler must not
        /// call `flush`, `close`, or `abort` on the same client, because that
        /// would wait for the worker that is waiting for the handler.
        pub fn with_on_send_succeeded<F, Fut>(mut self, handler: F) -> Self
        where
            F: Fn(SendBatchSucceededContext) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static,
        {
            self.on_send_succeeded = Some(Arc::new(move |context| handler(context).boxed()));
            self
        }

        /// Registers the handler that runs after a batch fails for the last time.
        ///
        /// The handler is required. The client calls it only once the retry
        /// policy is exhausted, or when the error is not retryable. The context
        /// holds the events, so the application decides what to do with them.
        /// The client does not enqueue them again.
        ///
        /// The partition worker waits for the handler, so a slow handler slows
        /// only its own partition. The handler must not call `flush`, `close`,
        /// or `abort` on the same client, because that would wait for the worker
        /// that is waiting for the handler.
        pub fn with_on_send_failed<F, Fut>(mut self, handler: F) -> Self
        where
            F: Fn(SendBatchFailedContext) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static,
        {
            self.on_send_failed = Some(Arc::new(move |context| handler(context).boxed()));
            self
        }

        fn build_handlers(&self) -> Result<DeliveryHandlers> {
            let failed = self.on_send_failed.clone().ok_or_else(|| {
                EventHubsError::with_message(
                    "A buffered producer client needs a handler for failed batches. \
                     Call with_on_send_failed before open.",
                )
            })?;
            Ok(DeliveryHandlers {
                succeeded: self.on_send_succeeded.clone(),
                failed,
            })
        }

        fn validate(&self) -> Result<()> {
            if self.max_buffered_event_count_per_partition == 0 {
                return Err(EventHubsError::with_message(
                    "The maximum buffered event count for one partition must be at least 1.",
                ));
            }
            if self.max_wait_time <= Duration::ZERO {
                return Err(EventHubsError::with_message(
                    "The maximum wait time must be longer than zero.",
                ));
            }
            Ok(())
        }

        /// Opens a connection to the Event Hub and starts the background workers.
        ///
        /// # Arguments
        ///
        /// * `fully_qualified_namespace` - The fully qualified namespace of the Event Hubs instance.
        /// * `eventhub` - The name of the Event Hub.
        /// * `credential` - The token credential used for authorization.
        pub async fn open(
            self,
            fully_qualified_namespace: &str,
            eventhub: &str,
            credential: Arc<dyn azure_core::credentials::TokenCredential>,
        ) -> Result<BufferedProducerClient> {
            self.validate()?;
            let handlers = self.build_handlers()?;

            let mut builder = ProducerClient::builder();
            if let Some(application_id) = self.application_id {
                builder = builder.with_application_id(application_id);
            }
            if let Some(retry_options) = self.retry_options {
                builder = builder.with_retry_options(retry_options);
            }
            if let Some(custom_endpoint) = self.custom_endpoint {
                builder = builder.with_custom_endpoint(custom_endpoint);
            }

            let producer = Arc::new(
                builder
                    .open(fully_qualified_namespace, eventhub, credential)
                    .await?,
            );
            let send_client = Arc::new(ProducerSendClient::new(producer.clone()));

            BufferedProducerClient::start(
                Some(producer),
                send_client,
                self.max_wait_time,
                self.max_buffered_event_count_per_partition,
                handlers,
            )
            .await
        }

        /// Opens a connection to the Event Hub with a connection string, and
        /// starts the background workers.
        ///
        /// Prefer [`open`](Self::open) with a `TokenCredential` for production.
        ///
        /// # Arguments
        ///
        /// * `connection_string` - An Event Hubs connection string.
        /// * `eventhub` - The Event Hub name. This is required unless the
        ///   connection string includes an `EntityPath`.
        pub async fn open_with_connection_string(
            self,
            connection_string: &str,
            eventhub: Option<&str>,
        ) -> Result<BufferedProducerClient> {
            self.validate()?;
            let handlers = self.build_handlers()?;

            let mut builder = ProducerClient::builder();
            if let Some(application_id) = self.application_id {
                builder = builder.with_application_id(application_id);
            }
            if let Some(retry_options) = self.retry_options {
                builder = builder.with_retry_options(retry_options);
            }
            if let Some(custom_endpoint) = self.custom_endpoint {
                builder = builder.with_custom_endpoint(custom_endpoint);
            }

            let producer = Arc::new(
                builder
                    .open_with_connection_string(connection_string, eventhub)
                    .await?,
            );
            let send_client = Arc::new(ProducerSendClient::new(producer.clone()));

            BufferedProducerClient::start(
                Some(producer),
                send_client,
                self.max_wait_time,
                self.max_buffered_event_count_per_partition,
                handlers,
            )
            .await
        }

        /// Starts a client over a supplied send client, with no network.
        #[cfg(test)]
        pub(crate) async fn open_with_send_client(
            self,
            send_client: Arc<dyn super::BufferedSendClient>,
        ) -> Result<BufferedProducerClient> {
            self.validate()?;
            let handlers = self.build_handlers()?;
            BufferedProducerClient::start(
                None,
                send_client,
                self.max_wait_time,
                self.max_buffered_event_count_per_partition,
                handlers,
            )
            .await
        }
    }
}

#[cfg(test)]
mod tests;
