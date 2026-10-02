// Copyright (c) Microsoft Corp. All Rights Reserved.
// Licensed under the MIT License.

//use async_channel::{bounded, Receiver, Sender};
use super::{
    load_balancer::LoadBalancer,
    models::{Checkpoint, Ownership, StartPositions},
    partition_client::PartitionClient,
    CheckpointStore, ProcessorStrategy,
};
use crate::{
    error::Result, models::ConsumerClientDetails, ConsumerClient, EventHubsError,
    OpenReceiverOptions, StartLocation, StartPosition,
};
//use async_io::Timer;
use async_lock::Mutex as AsyncMutex;
use azure_core::{error::ErrorKind as AzureErrorKind, time::Duration, Error};
use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    SinkExt, StreamExt,
};
use std::{
    sync::{
        Arc,
        Mutex as SyncMutex, // Mutex for blocking operations
    },
    {collections::HashMap, sync::Weak},
};
use tracing::{debug, error, info, warn};

// AMQP epoch (owner level) used for every partition receiver opened by
// `EventProcessor`. Matches `EventProcessorClient` in the .NET and Java
// SDKs. `0` is itself an exclusive epoch (a new receiver-at-0 displaces a
// prior receiver-at-0), which is how the processor detects steals.
const PROCESSOR_OWNER_LEVEL: i64 = 0;

/// Represents the event processor responsible for processing events
/// from Event Hub partitions.
///
/// This struct manages the load balancing strategy, checkpoint store,
/// and consumer client for processing events.
/// It provides methods for starting the event processor, dispatching
/// events, and managing partition clients.
///
/// The event processor uses a load balancer to distribute the load
/// across partitions and a checkpoint store to manage checkpoints.
///
/// Each per-partition receiver opens with AMQP epoch `0`, matching the .NET
/// and Java `EventProcessorClient`. When another `EventProcessor` attaches
/// to the same partition, the broker disconnects this receiver and
/// [`PartitionClient::stream_events()`] yields
/// [`ErrorKind::ConsumerDisconnected`](crate::error::ErrorKind::ConsumerDisconnected).
/// To use a different epoch, open receivers directly via
/// `ConsumerClient::open_receiver_on_partition`.
///
/// For more information on Event Processors and scenarios in which you would
/// use an Event Processor, see the [Event Processor documentation](https://learn.microsoft.com/azure/event-hubs/event-processor-balance-partition-load).
///
pub struct EventProcessor {
    checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
    load_balancer: Arc<AsyncMutex<LoadBalancer>>,
    consumer_client: ConsumerClient,
    next_partition_clients: AsyncMutex<Receiver<Arc<PartitionClient>>>,
    next_partition_client_sender: Sender<Arc<PartitionClient>>,
    client_details: ConsumerClientDetails,
    prefetch: u32,
    update_interval: Duration,
    start_positions: StartPositions,
    is_running: std::sync::Mutex<bool>,
    run_lock: AsyncMutex<()>,
    lifecycle_lock: AsyncMutex<()>,
    partition_ids: Vec<String>,
    consumers: Arc<ProcessorConsumersMap>,
}

struct EventProcessorOptions {
    strategy: ProcessorStrategy,
    partition_expiration_duration: Duration,
    update_interval: Duration,
    start_positions: StartPositions,
    prefetch: u32,
    partition_ids: Vec<String>,
}

pub(crate) struct ProcessorConsumersMap {
    consumers: SyncMutex<HashMap<String, Weak<PartitionClient>>>,
}

impl ProcessorConsumersMap {
    fn new() -> Self {
        ProcessorConsumersMap {
            consumers: SyncMutex::new(HashMap::new()),
        }
    }

    /// Adds a partition client to the consumers map.
    /// Replaces a closed or dropped client for the same partition ID.
    /// An existing open client is retained.
    /// Returns `true` if the partition client was added successfully,
    /// or `false` if it already exists.
    ///
    /// # Arguments
    /// * `partition_id` - The ID of the partition for which the client is being added.
    /// * `partition_client` - The partition client to be added.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the operation.
    /// If successful, returns `true` if the partition client was added,
    /// or `false` if it already exists.
    ///
    pub async fn add_partition_client(
        &self,
        partition_id: &str,
        partition_client: Arc<PartitionClient>,
    ) -> Result<bool> {
        debug!(partition_id = %partition_id, "Adding partition client for partition.");
        let mut consumers = self
            .consumers
            .lock()
            .map_err(|_| EventHubsError::with_message("Could not lock consumers mutex."))?;
        if consumers
            .get(partition_id)
            .and_then(Weak::upgrade)
            .is_some_and(|client| !client.is_closed())
        {
            debug!(
                partition_id = %partition_id,
                "Partition client already exists for partition."
            );
            return Ok(false);
        }
        consumers.insert(partition_id.to_string(), Arc::downgrade(&partition_client));
        debug!(partitions = ?consumers.keys(), "Consumers for partition.");
        Ok(true)
    }

    pub fn remove_partition_client(&self, partition_client: &PartitionClient) -> Result<()> {
        let partition_id = partition_client.get_partition_id();
        debug!(partition_id = %partition_id, "Removing partition client for partition.");
        let mut consumers = self
            .consumers
            .lock()
            .map_err(|_| EventHubsError::with_message("Could not lock consumers mutex."))?;
        // A retained client from an earlier run must not remove its replacement.
        if consumers.get(partition_id).is_some_and(|client| {
            client
                .upgrade()
                .is_none_or(|client| client.client_id() == partition_client.client_id())
        }) {
            consumers.remove(partition_id);
        }
        debug!(partitions = ?consumers.keys(), "Consumers for partition now.");
        Ok(())
    }

    /// Returns the set of partition IDs that have active partition clients.
    fn get_active_partition_ids(&self) -> Result<Vec<String>> {
        let consumers = self
            .consumers
            .lock()
            .map_err(|_| EventHubsError::with_message("Could not lock consumers mutex."))?;
        Ok(consumers.keys().cloned().collect())
    }

    /// Removes partitions reassigned away from this processor and closes
    /// their receivers so consumer streams terminate. Backstop for the
    /// broker's epoch-based disconnect.
    async fn revoke_partition_clients(&self, partition_ids: &[String]) -> Result<()> {
        // Collect under the sync lock, then release before awaiting:
        // SyncMutex guards cannot be held across `.await`.
        let to_close: Vec<Arc<PartitionClient>> = {
            let mut consumers = self
                .consumers
                .lock()
                .map_err(|_| EventHubsError::with_message("Could not lock consumers mutex."))?;
            partition_ids
                .iter()
                .filter_map(|id| {
                    let client = consumers.get(id).and_then(Weak::upgrade);
                    if client.is_none() {
                        consumers.remove(id);
                    }
                    client
                })
                .collect()
        };
        for client in to_close {
            // Keep the remaining clients discoverable by shutdown while detach awaits.
            client.request_close_receiver().await;
            self.remove_partition_client(&client)?;
        }
        Ok(())
    }

    /// Closes the receiver of every partition client in the map, so an
    /// in-flight `stream_events()` resolves. The entries stay in the map,
    /// because a client that the application still holds keeps its place
    /// until it is replaced on restart or explicitly closed.
    async fn close_all_receivers(&self) -> Result<()> {
        // Collect under the sync lock, then release before awaiting:
        // SyncMutex guards cannot be held across `.await`.
        let to_close: Vec<Arc<PartitionClient>> = {
            let consumers = self
                .consumers
                .lock()
                .map_err(|_| EventHubsError::with_message("Could not lock consumers mutex."))?;
            consumers
                .values()
                .filter_map(|client| client.upgrade())
                .collect()
        };
        for client in to_close {
            client.request_close_receiver().await;
        }
        Ok(())
    }
}

//pub(crate) type ConsumersType = std::sync::Mutex<HashMap<String, Arc<PartitionClient>>>;

unsafe impl Send for EventProcessor {}
unsafe impl Sync for EventProcessor {}

impl EventProcessor {
    /// Creates a new `EventProcessorBuilder` instance.
    /// This builder allows you to configure various options for the event processor,
    /// such as load balancing strategy, update interval, start positions, and more.
    ///
    /// # Returns a new [`builders::EventProcessorBuilder`] instance.
    pub fn builder() -> builders::EventProcessorBuilder {
        builders::EventProcessorBuilder::new()
    }

    fn new(
        consumer_client: ConsumerClient,
        checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
        options: EventProcessorOptions,
    ) -> Result<Arc<Self>> {
        let (sender, receiver) = channel(options.partition_ids.len());

        let client_details = consumer_client.get_details()?;

        Ok(Arc::new(EventProcessor {
            checkpoint_store: checkpoint_store.clone(),
            consumer_client,

            // Default to Balanced strategy if not provided
            load_balancer: Arc::new(AsyncMutex::new(LoadBalancer::new(
                checkpoint_store.clone(),
                client_details.clone(),
                options.strategy,
                options.partition_expiration_duration,
                None,
            ))),
            client_details,
            prefetch: options.prefetch,
            update_interval: options.update_interval,
            start_positions: options.start_positions,
            next_partition_client_sender: sender,
            next_partition_clients: AsyncMutex::new(receiver),
            is_running: std::sync::Mutex::new(false),
            run_lock: AsyncMutex::new(()),
            lifecycle_lock: AsyncMutex::new(()),
            partition_ids: options.partition_ids,
            consumers: Arc::new(ProcessorConsumersMap::new()),
        }))
    }

    /// Starts the event processor.
    /// This method initiates the event processing loop and begins
    /// processing events from the Event Hub partitions.
    /// It uses the specified checkpoint store and load balancing strategy
    /// to manage the ownership of partitions and distribute the load
    /// among consumers.
    /// The event processor will run until it is stopped or interrupted.
    ///
    /// After [`shutdown()`](Self::shutdown), this loop completes its current
    /// dispatch and sleeps for the configured update interval before returning.
    /// Dispatch can wait for network operations or partition-client queue capacity,
    /// so the update interval does not bound the total time until this method returns.
    /// Event delivery stops when `shutdown()` returns.
    ///
    /// After calling `shutdown()` and waiting for this method to return,
    /// call it again to restart processing.
    /// The restarted processor issues new partition clients; clients from the
    /// previous run remain closed.
    ///
    /// # Errors
    /// Returns an error if another call to `run()` is active or if dispatch fails.
    /// # Examples
    /// ```
    /// use azure_messaging_eventhubs::EventProcessor;
    /// use azure_messaging_eventhubs::ConsumerClient;
    /// use std::sync::Arc;
    /// use azure_core::time::Duration;
    /// use azure_messaging_eventhubs::ProcessorStrategy;
    /// use azure_messaging_eventhubs::CheckpointStore;
    ///
    /// async fn run_processor(consumer_client: ConsumerClient, checkpoint_store: impl CheckpointStore+Send+Sync+'static) -> Result<(), Box<dyn std::error::Error>> {
    ///   // Create an instance of the EventProcessor
    ///   let event_processor = EventProcessor::builder()
    ///       .with_load_balancing_strategy(ProcessorStrategy::Balanced)
    ///       .with_update_interval(Duration::seconds(30))
    ///       .with_partition_expiration_duration(Duration::seconds(60))
    ///       .with_prefetch(300)
    ///       .build(
    ///          consumer_client,
    ///          Arc::new(checkpoint_store)).await?;
    ///
    ///   // Start the event processor
    ///   {
    ///     tokio::select!{
    ///          result = event_processor.run() => {
    ///              if let Err(e) = result {
    ///                  println!("Event processor failed: {:?}", e);
    ///              } else {
    ///                  println!("Event processor finished successfully");
    ///              }
    ///          }
    ///          _ = tokio::time::sleep(std::time::Duration::from_secs(60)) => {}
    ///     }
    ///   }
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn run(&self) -> Result<()> {
        let _run = self
            .run_lock
            .try_lock()
            .ok_or_else(|| EventHubsError::with_message("Event processor is already running."))?;
        {
            // A restart cannot publish new work until the previous stop finishes.
            let _lifecycle = self.lifecycle_lock.lock().await;
            let mut is_running = self.is_running.lock().map_err(|_| {
                Error::new(AzureErrorKind::Io, "Could not lock is_running on startup")
            })?;
            *is_running = true;
        }

        let partition_ids = &self
            .partition_ids
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>();

        loop {
            let result = self.dispatch(partition_ids, &self.consumers).await;
            match result {
                Ok(_) => {
                    debug!("Event processor dispatched successfully.");
                }
                Err(e) => {
                    error!(err = ?e, "Error dispatching event processor.");
                    return Err(e);
                }
            }
            debug!("Event processor sleeping for {:?}", self.update_interval);
            azure_core::sleep::sleep(self.update_interval).await;
            debug!("Event processor woke up from sleep.");
            if self.is_shutdown()? {
                info!("Event processor shutting down.");
                break Ok(());
            }
        }
    }

    /// Shuts down the event processor.
    ///
    /// The call stops the event delivery on every partition client that this
    /// processor issued, including a partition client that the application
    /// still holds. Pending reads are canceled, and
    /// [`PartitionClient::stream_events()`] yields
    /// [`ErrorKind::ConsumerDisconnected`](crate::error::ErrorKind::ConsumerDisconnected)
    /// when next polled. The call then releases the
    /// ownership records of this instance after any pending ownership claims
    /// complete, so that another instance can claim
    /// those partitions immediately, without a wait for the expiration.
    ///
    /// A failure to release the ownership records does not fail the call. The
    /// records expire on their own if the release fails.
    ///
    /// [`close()`](Self::close) is a superset of this call: it
    /// consumes the processor, and it also drains the queued partition clients
    /// and closes the consumer client.
    ///
    /// # Errors
    /// Returns an error if the processor cannot read its own state.
    pub async fn shutdown(&self) -> Result<()> {
        self.stop().await
    }

    /// Stops the processing loop, the event delivery, and the ownership of
    /// this instance. Shared by [`shutdown()`](Self::shutdown) and
    /// [`close()`](Self::close).
    ///
    /// Closes receivers before waiting for the claim phase to finish. The load
    /// balancer lock covers only claims, so shutdown never waits on a queue send.
    /// A stopped dispatch cannot begin another claim.
    async fn stop(&self) -> Result<()> {
        let _lifecycle = self.lifecycle_lock.lock().await;
        {
            let mut is_running = self.is_running.lock().map_err(|_| {
                EventHubsError::with_message("Failed to acquire lock on is_running for shutdown")
            })?;
            *is_running = false;
        }

        self.consumers.close_all_receivers().await?;
        let _claims = self.load_balancer.lock().await;
        self.release_ownerships().await;
        Ok(())
    }

    /// Releases the ownership records that this instance owns.
    ///
    /// The call keeps the ETag that the store returned, because
    /// `claim_ownership` rejects a record whose ETag does not match the one
    /// the store holds. A failure must not stop the shutdown, so this logs the
    /// error and returns.
    async fn release_ownerships(&self) {
        let ownerships = self
            .checkpoint_store
            .list_ownerships(
                &self.client_details.fully_qualified_namespace,
                &self.client_details.eventhub_name,
                &self.client_details.consumer_group,
            )
            .await;
        let ownerships = match ownerships {
            Ok(ownerships) => ownerships,
            Err(e) => {
                warn!(err = ?e, "Failed to list the ownerships to release on shutdown.");
                return;
            }
        };

        let to_release: Vec<Ownership> = ownerships
            .into_iter()
            .filter(|ownership| {
                ownership.owner_id.as_deref() == Some(self.client_details.client_id.as_str())
            })
            .map(|mut ownership| {
                ownership.owner_id = None;
                ownership
            })
            .collect();
        if to_release.is_empty() {
            return;
        }

        info!(
            count = to_release.len(),
            "Releasing the ownerships of this processor."
        );
        if let Err(e) = self.checkpoint_store.claim_ownership(&to_release).await {
            warn!(err = ?e, "Failed to release the ownerships on shutdown.");
        }
    }

    fn is_shutdown(&self) -> Result<bool> {
        // Implement shutdown logic if needed
        let is_running = self
            .is_running
            .lock()
            .map_err(|_| EventHubsError::with_message("Failed to acquire lock on is_running"))?;
        if *is_running {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    #[tracing::instrument(
        level = "debug",
        skip_all,
        fields(
            eventhub = %self.client_details.eventhub_name,
            consumer_group = %self.client_details.consumer_group,
            fully_qualified_namespace = %self.client_details.fully_qualified_namespace,
            owner_id = %self.client_details.client_id,
        ),
        err,
    )]
    async fn dispatch(
        &self,
        partition_ids: &[&str],
        consumers: &Arc<ProcessorConsumersMap>,
    ) -> Result<()> {
        debug!("Dispatch partition clients to consumers.");
        let ownerships = {
            // Stop waits for this phase, never for receiver creation or queue sends.
            let load_balancer = self.load_balancer.lock().await;
            if self.is_shutdown()? {
                return Ok(());
            }
            load_balancer
                .load_balance(partition_ids)
                .await
                .map_err(|e| {
                    error!(err = ?e, "Error in load balancing.");
                    e
                })?
        };
        if self.is_shutdown()? {
            // Stop releases all completed claims while holding the same lock.
            return Ok(());
        }

        // Revoke clients for any partitions no longer in the ownership set.
        let owned_ids: std::collections::HashSet<&str> =
            ownerships.iter().map(|o| o.partition_id.as_str()).collect();
        let active_ids = consumers.get_active_partition_ids()?;
        let stolen: Vec<String> = active_ids
            .into_iter()
            .filter(|id| !owned_ids.contains(id.as_str()))
            .collect();
        if !stolen.is_empty() {
            info!(
                partitions = %stolen.join(", "),
                "Partitions no longer owned, revoking."
            );
            consumers.revoke_partition_clients(&stolen).await?;
        }

        let checkpoints = self.get_checkpoint_map().await;
        let checkpoints = checkpoints.map_err(|e| {
            error!(err = ?e, "Error in getting checkpoint map.");
            e
        })?;

        debug!(
            "Adding partition clients for {} ownerships ",
            ownerships.len()
        );
        for ownership in ownerships {
            let err = self
                .add_partition_client(
                    ownership.partition_id,
                    &checkpoints,
                    Arc::downgrade(consumers),
                )
                .await;
            if let Err(e) = err {
                error!(err = ?e, "Error adding partition client.");
                return Err(e);
            }
        }

        Ok(())
    }

    #[tracing::instrument(
        level = "debug",
        skip_all,
        fields(partition_id = %partition_id),
        err,
    )]
    async fn add_partition_client(
        &self,
        partition_id: String,
        checkpoints: &HashMap<String, Checkpoint>,
        consumers: Weak<ProcessorConsumersMap>,
    ) -> Result<()> {
        debug!(partition_id = %partition_id, "Add partition client for partition.");

        let partition_client = Arc::new(PartitionClient::new(
            partition_id.clone(),
            self.checkpoint_store.clone(),
            self.client_details.clone(),
            consumers.clone(),
        ));

        if let Some(strong_consumers) = consumers.upgrade() {
            if !strong_consumers
                .add_partition_client(&partition_id, partition_client.clone())
                .await?
            {
                debug!(
                    partition_id = %partition_id,
                    "Partition client already exists for partition, ignoring."
                );
                return Ok(());
            }
        } else {
            error!("Consumers map is no longer valid.");
            return Err(EventHubsError::with_message(
                "Consumers map is no longer valid.",
            ));
        }

        // Since we can only have a single EventReceiver on a partition, we don't actually attempt to create the receiver until
        let start_position = self.get_start_position(&partition_id, checkpoints);
        debug!(
            partition_id = %partition_id,
            start_position = ?start_position,
            "Start position for partition."
        );
        let receiver = self
            .consumer_client
            .open_receiver_on_partition(
                partition_id.clone(),
                Some(OpenReceiverOptions {
                    start_position: Some(start_position),
                    prefetch: Some(self.prefetch),
                    owner_level: Some(PROCESSOR_OWNER_LEVEL),
                    ..Default::default()
                }),
            )
            .await;
        // Roll back the consumers-map entry on failure; otherwise the
        // partition is stuck (the map's `contains_key` check would
        // short-circuit every retry) until steal-revocation or restart.
        let receiver = match receiver {
            Ok(r) => r,
            Err(e) => {
                error!(
                    partition_id = %partition_id,
                    err = ?e,
                    "Error opening receiver for partition client."
                );
                if let Some(strong_consumers) = consumers.upgrade() {
                    let _ = strong_consumers.remove_partition_client(&partition_client);
                }
                return Err(e);
            }
        };
        info!(partition_id = %partition_id, "Receiver opened for partition client.");
        if let Err(e) = partition_client.set_event_receiver(receiver) {
            error!(
                partition_id = %partition_id,
                err = ?e,
                "Error setting event receiver for partition."
            );
            if let Some(strong_consumers) = consumers.upgrade() {
                let _ = strong_consumers.remove_partition_client(&partition_client);
            }
            return Err(e);
        }

        // `stop` snapshots the map, so a receiver set after that snapshot
        // would outlive the shutdown. The `is_running` mutex orders the two:
        // this read sees the stop, or the snapshot sees the receiver. The map
        // entry rolls back the same way as the two failure paths above.
        if self.is_shutdown()? {
            info!(
                partition_id = %partition_id,
                "Event processor stopped while the receiver opened, dropping the client."
            );
            partition_client.request_close_receiver().await;
            if let Some(strong_consumers) = consumers.upgrade() {
                let _ = strong_consumers.remove_partition_client(&partition_client);
            }
            return Ok(());
        }

        debug!(partition_id = %partition_id, "Adding partition client to queue.");

        // Send the partition client to the next partition client receiver
        {
            let mut sender = self.next_partition_client_sender.clone();
            sender.send(partition_client).await.map_err(|e| {
                EventHubsError::from(azure_core::Error::with_message(
                    AzureErrorKind::Other,
                    format!("Failed to send partition client: {:?}", e),
                ))
            })?;
        }
        debug!(
            partition_id = %partition_id,
            "add_partition_client: Partition client added for partition."
        );

        Ok(())
    }

    /// Retrieves the next partition client for processing events.
    ///
    /// Waits for an open partition client, skipping clients closed by an earlier
    /// shutdown or partition revocation.
    pub async fn next_partition_client(&self) -> Result<Arc<PartitionClient>> {
        // Implement the function or remove it if not needed
        debug!("next_partition_client: Waiting to receive the next partition client.");

        let mut clients = self.next_partition_clients.lock().await;
        loop {
            let next_client = clients.next().await.ok_or_else(|| {
                EventHubsError::with_message("No next partition client available.")
            })?;
            // Shutdown keeps queued clients alive. A restart only issues open clients.
            if next_client.is_closed() {
                continue;
            }
            debug!(partition_id = %next_client.get_partition_id(), "Returning partition client.");
            return Ok(next_client);
        }
    }

    /// Closes the event processor.
    ///
    /// The call runs the same stop path as
    /// [`shutdown()`](Self::shutdown), and it also drains the queued
    /// partition clients and closes the consumer client.
    pub async fn close(self) -> Result<()> {
        // Stop the delivery and release the ownership first, then continue
        // with the close: a failure here must not leave the connection open.
        if let Err(e) = self.stop().await {
            error!(err = ?e, "Failed to stop the event processor on close.");
        }

        // Close all partition clients.
        info!("Closing all partition clients.");
        let mut clients = self.next_partition_clients.lock().await;
        while let Ok(client) = clients.try_recv() {
            info!(
                partition_id = %client.get_partition_id(),
                "Closing partition client for partition."
            );
            // A partition client that the application still holds cannot be
            // taken out of its `Arc`. Report it and continue, so that one such
            // client does not stop the processor from closing the clients that
            // follow.
            //
            // The connection solved the same problem by taking `&self`, and a
            // partition client could do the same through
            // `EventReceiver::request_close`, which detaches the receiver the
            // way `EventReceiver::close` does. That needs a new signature for
            // the public `PartitionClient::close`, which is a breaking change.
            // This fix therefore leaves such a client to the application that
            // holds it.
            let Ok(client) = Arc::try_unwrap(client) else {
                warn!(
                    "Could not close a partition client, because the application still holds it."
                );
                continue;
            };
            let res = client.close().await;
            if let Err(e) = res {
                error!(err = ?e, "Failed to close partition client.");
            } else {
                info!("Partition client closed successfully.");
            }
        }

        // Close the event processor and release resources.
        info!("Closing consumer client.");
        let res = self.consumer_client.close().await;
        if let Err(e) = res {
            error!(err = ?e, "Failed to close consumer client.");
        } else {
            info!("Consumer client closed successfully.");
        }
        Ok(())
    }

    /// Retrieves the checkpoint map for the Event Hub.
    ///
    /// This method fetches the checkpoints for all partitions in the Event Hub
    /// and returns them as a `HashMap` where the keys are partition IDs
    ///
    /// # Returns
    /// A `Result` containing a `HashMap` of partition IDs and their corresponding `Checkpoint` objects.
    ///
    ///
    async fn get_checkpoint_map(&self) -> Result<HashMap<String, Checkpoint>> {
        let checkpoints = self.checkpoint_store.list_checkpoints(
            &self.client_details.fully_qualified_namespace,
            &self.client_details.eventhub_name,
            &self.client_details.consumer_group,
        );
        let mut checkpoint_map = HashMap::new();
        for checkpoint in checkpoints.await? {
            checkpoint_map.insert(checkpoint.partition_id.clone(), checkpoint);
        }
        Ok(checkpoint_map)
    }

    /// Retrieve the start position for the specified ownership.
    ///
    /// This method determines the starting position for event processing
    /// based on the ownership information and the provided checkpoints.
    /// It checks if the ownership has a corresponding checkpoint and
    /// returns the appropriate start position.
    ///
    /// If no checkpoint is found for the partition in the ownership, a start
    /// position is chosen from the configured default start positions.
    ///
    /// # Arguments
    /// * partition_id - The partition for which to determine the start position.
    /// * `checkpoints` - A map of checkpoints for all partitions.
    ///
    fn get_start_position(
        &self,
        partition_id: &str,
        checkpoints: &HashMap<String, Checkpoint>,
    ) -> StartPosition {
        let mut start_position = self.start_positions.default.clone();
        if checkpoints.contains_key(partition_id) {
            let checkpoint = checkpoints.get(partition_id).unwrap();
            if let Some(offset) = &checkpoint.offset {
                start_position.location = StartLocation::Offset(offset.clone());
            } else if let Some(sequence_number) = checkpoint.sequence_number {
                start_position.location = StartLocation::SequenceNumber(sequence_number);
            }
        } else if self
            .start_positions
            .per_partition
            .contains_key(partition_id)
        {
            start_position = self
                .start_positions
                .per_partition
                .get(partition_id)
                .unwrap()
                .clone();
        } else {
            start_position = self.start_positions.default.clone();
        }
        start_position
    }
}

pub mod builders {
    use super::{CheckpointStore, EventProcessor};
    use crate::{error::Result, event_processor::models::StartPositions, ConsumerClient};
    use azure_core::time::Duration;
    use std::sync::Arc;

    const DEFAULT_PREFETCH: u32 = 300;
    const DEFAULT_UPDATE_INTERVAL: Duration = Duration::seconds(30);
    const DEFAULT_PARTITION_EXPIRATION_DURATION: Duration = Duration::seconds(60);

    /// Builder for creating an `EventProcessor`.
    /// This builder allows you to configure various options for the event processor,
    /// such as load balancing strategy, update interval, start positions, and more.
    /// It provides a fluent interface for setting these options and building the event processor.
    /// # Examples
    /// ``` no_run
    /// use azure_messaging_eventhubs::{EventProcessor,CheckpointStore ,ConsumerClient};
    /// use std::sync::Arc;
    ///
    /// async fn create_processor(checkpoint_store: Arc<dyn CheckpointStore>) -> Result<(), Box<dyn std::error::Error>> {
    /// use azure_core::Result;
    /// use azure_identity::DeveloperToolsCredential;
    ///
    /// let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    /// let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    /// let consumer = ConsumerClient::builder()
    ///         .open(
    ///             &eventhub_namespace,
    ///             eventhub_name,
    ///             DeveloperToolsCredential::new(None)?.clone(),
    ///         )
    ///         .await?;
    /// println!("Opened consumer client");
    /// let processor = EventProcessor::builder()
    ///     .build(consumer, checkpoint_store.clone())
    ///     .await?;
    /// Ok(())
    /// }
    /// ```
    #[derive(Default)]
    pub struct EventProcessorBuilder {
        update_interval: Option<Duration>,
        start_positions: Option<StartPositions>,
        max_partition_count: Option<usize>,
        prefetch: Option<u32>,
        load_balancing_strategy: Option<super::ProcessorStrategy>,
        partition_expiration_duration: Option<Duration>,
    }
    /// Returns an error if `partition_expiration_duration` is not strictly
    /// greater than `update_interval`. When expiration is shorter than the
    /// load-balancing cycle, every consumer's ownership record expires before
    /// the next cycle observes it, so the load balancer perpetually re-claims
    /// every partition. This is extracted from `build()` to make the rule
    /// directly unit-testable without needing a live `ConsumerClient`.
    pub(crate) fn validate_expiration_vs_update_interval(
        partition_expiration_duration: Duration,
        update_interval: Duration,
    ) -> Result<()> {
        if partition_expiration_duration <= update_interval {
            return Err(crate::EventHubsError::with_message(format!(
                "partition_expiration_duration ({partition_expiration_duration:?}) must be \
                 greater than update_interval ({update_interval:?}); otherwise ownership \
                 records expire between load-balancing cycles and the processor will \
                 perpetually re-claim partitions, causing duplicate processing. A ratio of \
                 at least 2x is recommended."
            )));
        }
        Ok(())
    }

    impl EventProcessorBuilder {
        pub(super) fn new() -> Self {
            EventProcessorBuilder {
                ..Default::default()
            }
        }

        /// Sets the load balancing strategy for the event processor.
        /// The default strategy is `Greedy`.
        pub fn with_load_balancing_strategy(
            mut self,
            load_balancing_strategy: super::ProcessorStrategy,
        ) -> Self {
            self.load_balancing_strategy = Some(load_balancing_strategy);
            self
        }

        /// Sets the processor update interval for the event processor.
        ///
        /// The processor will sleep for the update interval between each iteration.
        /// The default update interval is 30 seconds.
        pub fn with_update_interval(mut self, update_interval: Duration) -> Self {
            self.update_interval = Some(update_interval);
            self
        }

        /// Sets the start positions for each partition and the default start position.
        pub fn with_start_positions(mut self, start_positions: StartPositions) -> Self {
            self.start_positions = Some(start_positions);
            self
        }

        /// Sets the maximum number of partitions to process.
        pub fn with_max_partition_count(mut self, max_partition_count: usize) -> Self {
            self.max_partition_count = Some(max_partition_count);
            self
        }

        /// Sets the prefetch count for the event processor.
        pub fn with_prefetch(mut self, prefetch: u32) -> Self {
            self.prefetch = Some(prefetch);
            self
        }

        /// Sets the partition expiration duration for the event processor.
        pub fn with_partition_expiration_duration(
            mut self,
            partition_expiration_duration: Duration,
        ) -> Self {
            self.partition_expiration_duration = Some(partition_expiration_duration);
            self
        }

        /// Builds the event processor with the specified consumer client and checkpoint store.
        /// Returns a `Result` containing the constructed `EventProcessor`.
        ///
        /// # Connection options
        ///
        /// The event processor does not open its own connection. It processes
        /// partitions using the [`ConsumerClient`] passed here, and every
        /// per-partition receiver reuses that client's connection. Connection-level
        /// options, such as the transport, a custom endpoint, retry options, and the
        /// application id, are therefore configured on the [`ConsumerClient`] before
        /// it is passed to `build`.
        ///
        /// For example, select TCP on the consumer client before building the processor:
        ///
        /// ```no_run
        /// use azure_messaging_eventhubs::{EventProcessor, CheckpointStore, ConsumerClient};
        /// use azure_messaging_eventhubs::models::AmqpTransport;
        /// use std::sync::Arc;
        ///
        /// async fn create_processor(checkpoint_store: Arc<dyn CheckpointStore>) -> Result<(), Box<dyn std::error::Error>> {
        /// use azure_identity::DeveloperToolsCredential;
        ///
        /// let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
        /// let eventhub_name = std::env::var("EVENTHUB_NAME")?;
        /// let consumer = ConsumerClient::builder()
        ///     .with_transport(AmqpTransport::Tcp)
        ///     .open(
        ///         &eventhub_namespace,
        ///         eventhub_name,
        ///         DeveloperToolsCredential::new(None)?.clone(),
        ///     )
        ///     .await?;
        /// let processor = EventProcessor::builder()
        ///     .build(consumer, checkpoint_store.clone())
        ///     .await?;
        /// Ok(())
        /// }
        /// ```
        pub async fn build(
            self,
            consumer_client: ConsumerClient,
            checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
        ) -> Result<Arc<EventProcessor>> {
            let update_interval = self.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL);
            let partition_expiration_duration = self
                .partition_expiration_duration
                .unwrap_or(DEFAULT_PARTITION_EXPIRATION_DURATION);

            validate_expiration_vs_update_interval(partition_expiration_duration, update_interval)?;

            // Retrieve the set of partitions from the consumer client
            // and limit the number of partitions to the specified max_partition_count.
            let mut eh_properties = consumer_client.get_eventhub_properties().await?;
            if let Some(max_partition_count) = self.max_partition_count {
                eh_properties.partition_ids.truncate(max_partition_count);
            }

            EventProcessor::new(
                consumer_client,
                checkpoint_store,
                super::EventProcessorOptions {
                    strategy: self
                        .load_balancing_strategy
                        .unwrap_or(super::ProcessorStrategy::Greedy),
                    partition_expiration_duration,
                    update_interval,
                    start_positions: self.start_positions.unwrap_or_default(),
                    prefetch: self.prefetch.unwrap_or(DEFAULT_PREFETCH),
                    partition_ids: eh_properties.partition_ids,
                },
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::builders::validate_expiration_vs_update_interval;
    use super::{
        Checkpoint, CheckpointStore, ConsumerClientDetails, EventProcessor, EventProcessorOptions,
        HashMap, PartitionClient, ProcessorConsumersMap, ProcessorStrategy, StartPositions,
    };
    use crate::{
        consumer::event_receiver::{
            receiver_with_delayed_close, receiver_with_failing_attach,
            receiver_with_pending_receive,
        },
        error::ErrorKind,
        models::Ownership,
        ConsumerClient, InMemoryCheckpointStore,
    };
    use azure_core::{error::ErrorKind as AzureErrorKind, time::Duration};
    use azure_core_amqp::AmqpError;
    use azure_core_test::credentials::MockCredential;
    use futures::{channel::oneshot, poll, SinkExt, StreamExt};
    use std::sync::{Arc, Mutex};

    /// Builds a processor that holds `partition_ids` queued partition clients,
    /// with no connection to the service. The returned map is the one that a
    /// `PartitionClient::close` removes itself from, so a test reads it to
    /// find out which clients closed.
    async fn processor_with_queued_clients_and_store(
        partition_ids: &[&str],
        checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
    ) -> (Arc<EventProcessor>, Arc<ProcessorConsumersMap>) {
        let consumer_client = ConsumerClient::new_unconnected(
            "example.servicebus.windows.net",
            "test-eventhub",
            Arc::new(MockCredential),
        )
        .expect("the client must build");
        let client_details = consumer_client.get_details().expect("details must parse");

        let processor = EventProcessor::new(
            consumer_client,
            checkpoint_store.clone(),
            EventProcessorOptions {
                strategy: ProcessorStrategy::Greedy,
                partition_expiration_duration: Duration::seconds(60),
                update_interval: Duration::seconds(30),
                start_positions: StartPositions::default(),
                prefetch: 300,
                partition_ids: partition_ids.iter().map(|id| id.to_string()).collect(),
            },
        )
        .expect("the processor must build");

        let consumers = processor.consumers.clone();
        let mut sender = processor.next_partition_client_sender.clone();
        for partition_id in partition_ids {
            let client = Arc::new(PartitionClient::new(
                partition_id.to_string(),
                checkpoint_store.clone(),
                client_details.clone(),
                Arc::downgrade(&consumers),
            ));
            consumers
                .add_partition_client(partition_id, client.clone())
                .await
                .expect("the map must accept the client");
            sender.send(client).await.expect("the queue must accept it");
        }

        (processor, consumers)
    }

    async fn processor_with_queued_clients(
        partition_ids: &[&str],
    ) -> (Arc<EventProcessor>, Arc<ProcessorConsumersMap>) {
        processor_with_queued_clients_and_store(
            partition_ids,
            Arc::new(InMemoryCheckpointStore::new()),
        )
        .await
    }

    /// Stands in for an application that holds a partition client it took.
    fn strong_client(
        consumers: &Arc<ProcessorConsumersMap>,
        partition_id: &str,
    ) -> Arc<PartitionClient> {
        consumers
            .consumers
            .lock()
            .expect("the map must lock")
            .get(partition_id)
            .unwrap_or_else(|| panic!("partition {partition_id} must be in the map"))
            .upgrade()
            .unwrap_or_else(|| panic!("partition {partition_id} must still be alive"))
    }

    /// Gives the client a receiver that answers offline. Without this the
    /// client has an empty `event_receiver`, and `stream_events` returns a
    /// canned "Event receiver is not set" stream that proves nothing.
    fn install_offline_receiver(client: &Arc<PartitionClient>, partition_id: &str) {
        client
            .set_event_receiver(receiver_with_failing_attach(
                partition_id,
                AmqpError::with_message("attach failed"),
            ))
            .expect("the receiver must install");
    }

    async fn assert_stream_stops(client: &PartitionClient, partition_id: &str) {
        let mut stream = std::pin::pin!(client.stream_events());
        let error = stream
            .next()
            .await
            .expect("the stream must yield an item")
            .expect_err("the stream must stop with an error");
        assert!(
            matches!(error.kind, ErrorKind::ConsumerDisconnected(None)),
            "partition {partition_id} must stop with ConsumerDisconnected, got {:?}",
            error.kind
        );
    }

    async fn claim_for(processor: &EventProcessor, partition_id: &str, owner: &str) -> Ownership {
        let details = &processor.client_details;
        let ownership = Ownership {
            fully_qualified_namespace: details.fully_qualified_namespace.clone(),
            event_hub_name: details.eventhub_name.clone(),
            consumer_group: details.consumer_group.clone(),
            partition_id: partition_id.to_string(),
            owner_id: Some(owner.to_string()),
            etag: None,
            ..Default::default()
        };
        processor
            .checkpoint_store
            .claim_ownership(&[ownership])
            .await
            .expect("the store must accept the claim")
            .pop()
            .expect("the store must return the claimed record")
    }

    /// Takes the store and the details, because `close` consumes the processor.
    async fn ownership_for(
        checkpoint_store: &Arc<dyn CheckpointStore + Send + Sync>,
        client_details: &ConsumerClientDetails,
        partition_id: &str,
    ) -> Ownership {
        checkpoint_store
            .list_ownerships(
                &client_details.fully_qualified_namespace,
                &client_details.eventhub_name,
                &client_details.consumer_group,
            )
            .await
            .expect("the store must list ownerships")
            .into_iter()
            .find(|o| o.partition_id == partition_id)
            .unwrap_or_else(|| panic!("partition {partition_id} must have an ownership record"))
    }

    struct FailingCheckpointStore;

    #[async_trait::async_trait]
    impl CheckpointStore for FailingCheckpointStore {
        async fn claim_ownership(
            &self,
            _ownerships: &[Ownership],
        ) -> azure_core::Result<Vec<Ownership>> {
            Err(azure_core::Error::with_message(
                AzureErrorKind::Other,
                "claim_ownership fails in this test".to_string(),
            ))
        }

        async fn list_checkpoints(
            &self,
            _namespace: &str,
            _event_hub_name: &str,
            _consumer_group: &str,
        ) -> azure_core::Result<Vec<Checkpoint>> {
            Ok(Vec::new())
        }

        async fn list_ownerships(
            &self,
            _namespace: &str,
            _event_hub_name: &str,
            _consumer_group: &str,
        ) -> azure_core::Result<Vec<Ownership>> {
            Err(azure_core::Error::with_message(
                AzureErrorKind::Other,
                "list_ownerships fails in this test".to_string(),
            ))
        }

        async fn update_checkpoint(&self, _checkpoint: Checkpoint) -> azure_core::Result<()> {
            Ok(())
        }
    }

    struct DelayedClaimStore {
        inner: InMemoryCheckpointStore,
        started: Mutex<Option<oneshot::Sender<()>>>,
        finish: async_lock::Mutex<Option<oneshot::Receiver<()>>>,
    }

    #[async_trait::async_trait]
    impl CheckpointStore for DelayedClaimStore {
        async fn claim_ownership(
            &self,
            ownerships: &[Ownership],
        ) -> azure_core::Result<Vec<Ownership>> {
            let started = self.started.lock().unwrap().take();
            if let Some(started) = started {
                let _ = started.send(());
                let finish = self.finish.lock().await.take().unwrap();
                finish.await.unwrap();
            }
            self.inner.claim_ownership(ownerships).await
        }
        async fn list_checkpoints(
            &self,
            namespace: &str,
            hub: &str,
            group: &str,
        ) -> azure_core::Result<Vec<Checkpoint>> {
            self.inner.list_checkpoints(namespace, hub, group).await
        }
        async fn list_ownerships(
            &self,
            namespace: &str,
            hub: &str,
            group: &str,
        ) -> azure_core::Result<Vec<Ownership>> {
            self.inner.list_ownerships(namespace, hub, group).await
        }
        async fn update_checkpoint(&self, checkpoint: Checkpoint) -> azure_core::Result<()> {
            self.inner.update_checkpoint(checkpoint).await
        }
    }

    #[tokio::test]
    async fn shutdown_waits_for_pending_ownership_claim() {
        let (started, claiming) = oneshot::channel();
        let (finish, delayed) = oneshot::channel();
        let store = Arc::new(DelayedClaimStore {
            inner: InMemoryCheckpointStore::new(),
            started: Mutex::new(Some(started)),
            finish: async_lock::Mutex::new(Some(delayed)),
        });
        let (mut processor, _consumers) =
            processor_with_queued_clients_and_store(&["0"], store).await;
        Arc::get_mut(&mut processor).unwrap().update_interval = Duration::milliseconds(1);
        drain_partition_client_queue(&processor).await;
        let running_processor = processor.clone();
        let running = tokio::spawn(async move { running_processor.run().await });
        tokio::time::timeout(std::time::Duration::from_secs(1), claiming)
            .await
            .unwrap()
            .unwrap();
        let mut shutdown = std::pin::pin!(processor.shutdown());
        let waits_for_claim = poll!(shutdown.as_mut()).is_pending();
        finish.send(()).unwrap();
        if waits_for_claim {
            tokio::time::timeout(std::time::Duration::from_secs(1), shutdown)
                .await
                .unwrap()
                .unwrap();
        }
        tokio::time::timeout(std::time::Duration::from_secs(1), running)
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        let ownership =
            ownership_for(&processor.checkpoint_store, &processor.client_details, "0").await;
        assert!(
            waits_for_claim,
            "shutdown must wait until outstanding ownership claims settle"
        );
        assert!(
            ownership.owner_id.is_none(),
            "shutdown must release the completed claim"
        );
    }

    #[tokio::test]
    async fn shutdown_cancels_clients_being_revoked() {
        let (processor, consumers) = processor_with_queued_clients(&["0", "1"]).await;
        let zero = strong_client(&consumers, "0");
        let one = strong_client(&consumers, "1");
        let (receiver, closing, finish) = receiver_with_delayed_close("0");
        zero.set_event_receiver(receiver).unwrap();
        install_offline_receiver(&one, "1");
        let ids = ["0".to_string(), "1".to_string()];
        let mut revoking = std::pin::pin!(consumers.revoke_partition_clients(&ids));
        assert!(poll!(revoking.as_mut()).is_pending());
        closing.await.unwrap();
        processor.shutdown().await.unwrap();
        let mut stream = std::pin::pin!(one.stream_events());
        let error = stream.next().await.unwrap().unwrap_err();
        finish.send(()).unwrap();
        revoking.await.unwrap();
        assert!(
            matches!(error.kind, ErrorKind::ConsumerDisconnected(None)),
            "shutdown must close the second client while the first detach is pending"
        );
    }

    /// `close` must not stop at a partition client that the application still
    /// holds. It used to take each client out of its `Arc` and return an error
    /// when that failed, which left the clients behind it open and skipped the
    /// close of the consumer connection.
    #[tokio::test]
    async fn close_continues_past_a_retained_partition_client() {
        let (processor, consumers) = processor_with_queued_clients(&["0", "1"]).await;

        // Stand in for an application that holds the first client it took.
        let retained = consumers
            .consumers
            .lock()
            .expect("the map must lock")
            .get("0")
            .expect("partition 0 must be in the map")
            .upgrade()
            .expect("partition 0 must still be alive");

        let connection = {
            let Ok(processor) = Arc::try_unwrap(processor) else {
                panic!("the test must be the only holder of the processor");
            };
            let connection = processor.consumer_client.recoverable_connection();
            processor.close().await.expect("close must succeed");
            connection
        };

        let active = consumers
            .get_active_partition_ids()
            .expect("the map must lock");
        assert!(
            active.contains(&"0".to_string()),
            "the retained client must stay in the map, because it did not close"
        );
        assert!(
            !active.contains(&"1".to_string()),
            "the client behind the retained one must close, got: {active:?}"
        );
        assert!(
            connection.is_closed(),
            "the consumer connection must close after the partition clients"
        );

        drop(retained);
    }

    /// `shutdown` must stop delivery on every partition client it issued,
    /// including a client the application still holds. Before the fix it only
    /// flipped `is_running`, so a held client kept receiving events.
    #[tokio::test]
    async fn shutdown_closes_receivers_of_issued_partition_clients() {
        let (processor, consumers) = processor_with_queued_clients(&["0", "1"]).await;
        let zero = strong_client(&consumers, "0");
        let one = strong_client(&consumers, "1");
        install_offline_receiver(&zero, "0");
        install_offline_receiver(&one, "1");

        processor.shutdown().await.expect("shutdown must succeed");

        assert_stream_stops(&zero, "0").await;
        assert_stream_stops(&one, "1").await;

        // Shutdown closes a receiver, it does not drop the map entry. This
        // guards `close_continues_past_a_retained_partition_client`.
        let active = consumers
            .get_active_partition_ids()
            .expect("the map must lock");
        assert!(
            active.contains(&"0".to_string()) && active.contains(&"1".to_string()),
            "shutdown must keep the map entries, got: {active:?}"
        );
    }

    /// `shutdown` must release the ownership records this instance holds, so
    /// another processor can claim the partitions without waiting for the
    /// expiration. It must not touch another instance's records.
    #[tokio::test]
    async fn shutdown_releases_only_this_instances_ownerships() {
        let (processor, _consumers) = processor_with_queued_clients(&["0", "1"]).await;
        let own_id = processor.client_details.client_id.clone();
        claim_for(&processor, "0", &own_id).await;
        claim_for(&processor, "1", "other-processor").await;

        processor.shutdown().await.expect("shutdown must succeed");

        let store = processor.checkpoint_store.clone();
        let details = processor.client_details.clone();
        let mine = ownership_for(&store, &details, "0").await;
        let theirs = ownership_for(&store, &details, "1").await;
        assert_eq!(
            mine.owner_id, None,
            "shutdown must release the ownership of this instance"
        );
        assert_eq!(
            theirs.owner_id,
            Some("other-processor".to_string()),
            "shutdown must leave the ownership of another instance alone"
        );
        assert!(
            mine.etag.is_some() && theirs.etag.is_some(),
            "both records must keep an etag, so a later claim can match it"
        );
    }

    /// A checkpoint store that rejects the ownership release must not stop
    /// shutdown, and must not stop the receivers from closing.
    #[tokio::test]
    async fn shutdown_continues_when_the_ownership_release_fails() {
        let (processor, consumers) =
            processor_with_queued_clients_and_store(&["0", "1"], Arc::new(FailingCheckpointStore))
                .await;
        let zero = strong_client(&consumers, "0");
        let one = strong_client(&consumers, "1");
        install_offline_receiver(&zero, "0");
        install_offline_receiver(&one, "1");

        processor
            .shutdown()
            .await
            .expect("a failed ownership release must not fail shutdown");

        assert_stream_stops(&zero, "0").await;
        assert_stream_stops(&one, "1").await;
    }

    /// `shutdown` is idempotent, and the second call keeps the release.
    #[tokio::test]
    async fn shutdown_twice_succeeds_and_keeps_ownership_released() {
        let (processor, consumers) = processor_with_queued_clients(&["0"]).await;
        let zero = strong_client(&consumers, "0");
        install_offline_receiver(&zero, "0");
        let own_id = processor.client_details.client_id.clone();
        claim_for(&processor, "0", &own_id).await;

        processor
            .shutdown()
            .await
            .expect("the first shutdown must succeed");
        processor
            .shutdown()
            .await
            .expect("the second shutdown must succeed");

        let store = processor.checkpoint_store.clone();
        let details = processor.client_details.clone();
        let record = ownership_for(&store, &details, "0").await;
        assert_eq!(
            record.owner_id, None,
            "the ownership must stay released after a second shutdown"
        );
        assert_stream_stops(&zero, "0").await;
    }

    /// `close` must run the same stop path as `shutdown`: release this
    /// instance's ownership and stop delivery on a client the application
    /// still holds, which the drain loop cannot take out of its `Arc`.
    #[tokio::test]
    async fn close_runs_the_shutdown_stop_path() {
        let (processor, consumers) = processor_with_queued_clients(&["0", "1"]).await;
        let retained = strong_client(&consumers, "0");
        install_offline_receiver(&retained, "0");
        let own_id = processor.client_details.client_id.clone();
        claim_for(&processor, "0", &own_id).await;

        let store = processor.checkpoint_store.clone();
        let details = processor.client_details.clone();

        let connection = {
            let Ok(processor) = Arc::try_unwrap(processor) else {
                panic!("the test must be the only holder of the processor");
            };
            let connection = processor.consumer_client.recoverable_connection();
            processor.close().await.expect("close must succeed");
            connection
        };

        let record = ownership_for(&store, &details, "0").await;
        assert_eq!(
            record.owner_id, None,
            "close must release the ownership of this instance"
        );
        assert_stream_stops(&retained, "0").await;
        assert!(
            connection.is_closed(),
            "the consumer connection must close after the partition clients"
        );
    }

    /// The shutdown future must stay `Send`. This passes because the stop
    /// path drops the `is_running` guard before it awaits. It exists to
    /// become a compile error if the stop path holds the
    /// `std::sync::MutexGuard<bool>` across an await, because that guard is
    /// not `Send`.
    #[tokio::test]
    async fn shutdown_future_is_send() {
        fn assert_send<T: Send>(_: &T) {}

        let (processor, _consumers) = processor_with_queued_clients(&["0"]).await;
        let fut = processor.shutdown();
        assert_send(&fut);
        fut.await.expect("shutdown must succeed");
    }

    /// A dispatch started after shutdown must not reclaim released partitions.
    #[tokio::test]
    async fn dispatch_after_shutdown_does_not_reclaim_ownership() {
        let (processor, _consumers) = processor_with_queued_clients(&["0", "1"]).await;
        drain_partition_client_queue(&processor).await;
        processor.shutdown().await.expect("shutdown must succeed");

        processor
            .dispatch(&["0", "1"], &processor.consumers)
            .await
            .expect("a dispatch after shutdown must not fail");

        let own_id = processor.client_details.client_id.clone();
        let still_ours: Vec<String> = processor
            .checkpoint_store
            .list_ownerships(
                &processor.client_details.fully_qualified_namespace,
                &processor.client_details.eventhub_name,
                &processor.client_details.consumer_group,
            )
            .await
            .expect("the store must list ownerships")
            .into_iter()
            .filter(|o| o.owner_id.as_deref() == Some(own_id.as_str()))
            .map(|o| o.partition_id)
            .collect();
        assert!(
            still_ours.is_empty(),
            "a dispatch after shutdown must leave no ownership claimed, got: {still_ours:?}"
        );
    }

    /// The other half of the same race. `close_all_receivers` either missed
    /// this client or found it with an empty `event_receiver`, so the client
    /// must not reach the application and must leave the map as it found it.
    #[tokio::test]
    async fn a_receiver_that_opens_after_shutdown_never_reaches_the_application() {
        let (processor, consumers) = processor_with_queued_clients(&["1"]).await;
        drain_partition_client_queue(&processor).await;
        processor.shutdown().await.expect("shutdown must succeed");

        processor
            .add_partition_client("0".to_string(), &HashMap::new(), Arc::downgrade(&consumers))
            .await
            .expect("adding a partition client after shutdown must not fail");

        let active = consumers
            .get_active_partition_ids()
            .expect("the map must lock");
        assert!(
            !active.contains(&"0".to_string()),
            "a client opened after shutdown must roll its map entry back, got: {active:?}"
        );

        let mut queue = processor.next_partition_clients.lock().await;
        assert!(
            queue.try_recv().is_err(),
            "a client opened after shutdown must not reach the application"
        );
    }

    /// The queue holds one client for each partition, and its buffer is the
    /// partition count. A test that adds one more client would park on a full
    /// queue instead of reaching its assertion, so it drains the queue first.
    async fn drain_partition_client_queue(processor: &EventProcessor) {
        let mut queue = processor.next_partition_clients.lock().await;
        while queue.try_recv().is_ok() {}
    }

    #[tokio::test]
    async fn shutdown_cancels_pending_partition_read() {
        let (processor, _consumers) = processor_with_queued_clients(&["0"]).await;
        let client = processor.next_partition_client().await.unwrap();
        let (receiver, delivery) = receiver_with_pending_receive("0");
        client.set_event_receiver(receiver).unwrap();
        let mut stream = std::pin::pin!(client.stream_events());
        assert!(poll!(stream.next()).is_pending());
        processor.shutdown().await.unwrap();
        let result = tokio::time::timeout(std::time::Duration::from_secs(1), stream.next())
            .await
            .expect("shutdown must cancel a pending partition read")
            .unwrap()
            .unwrap_err();
        assert!(matches!(result.kind, ErrorKind::ConsumerDisconnected(None)));
        assert!(delivery.is_canceled());
    }

    async fn restart_replaces_client(retain_old_client: bool) {
        let (mut processor, consumers) = processor_with_queued_clients(&["0"]).await;
        Arc::get_mut(&mut processor).unwrap().update_interval = Duration::milliseconds(1);
        let old_client = processor.next_partition_client().await.unwrap();
        install_offline_receiver(&old_client, "0");
        processor.shutdown().await.unwrap();
        assert_stream_stops(&old_client, "0").await;
        let retained = if retain_old_client {
            Some(old_client)
        } else {
            drop(old_client);
            None
        };

        let running_processor = processor.clone();
        let running = tokio::spawn(async move { running_processor.run().await });
        let replacement = tokio::time::timeout(
            std::time::Duration::from_secs(1),
            processor.next_partition_client(),
        )
        .await;
        processor.shutdown().await.unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(1), running)
            .await
            .expect("run must finish after shutdown")
            .unwrap()
            .unwrap();
        let replacement = replacement
            .expect("restart must issue a replacement client")
            .unwrap();
        if let Some(retained) = retained {
            let retained = Arc::try_unwrap(retained)
                .unwrap_or_else(|_| panic!("only the application holds the old client"));
            retained.close().await.unwrap();
            assert!(
                Arc::ptr_eq(&strong_client(&consumers, "0"), &replacement),
                "closing the old client must preserve its replacement"
            );
        }
    }

    #[tokio::test]
    async fn run_rejects_another_active_run() {
        let (mut processor, _consumers) = processor_with_queued_clients(&["0"]).await;
        Arc::get_mut(&mut processor).unwrap().update_interval = Duration::milliseconds(1);
        let mut running = std::pin::pin!(processor.run());
        assert!(poll!(running.as_mut()).is_pending());
        let second_run =
            tokio::time::timeout(std::time::Duration::from_secs(1), processor.run()).await;
        processor.shutdown().await.unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(1), running)
            .await
            .unwrap()
            .unwrap();
        let error = second_run.expect("a second run must not wait").unwrap_err();
        assert!(error.to_string().contains("already running"));
    }

    #[tokio::test]
    async fn restart_skips_queued_closed_clients() {
        let (processor, consumers) = processor_with_queued_clients(&["0"]).await;
        let old_client = strong_client(&consumers, "0");
        install_offline_receiver(&old_client, "0");
        processor.shutdown().await.unwrap();
        let running_processor = processor.clone();
        let running = tokio::spawn(async move { running_processor.run().await });
        let replacement = tokio::time::timeout(
            std::time::Duration::from_secs(1),
            processor.next_partition_client(),
        )
        .await;
        running.abort();
        let _ = running.await;
        processor.shutdown().await.unwrap();
        let replacement = replacement.expect("restart must issue a client").unwrap();
        assert!(
            !Arc::ptr_eq(&old_client, &replacement),
            "restart must skip the old queued client"
        );
    }

    #[tokio::test]
    async fn restart_replaces_retained_closed_client() {
        restart_replaces_client(true).await;
    }

    #[tokio::test]
    async fn restart_replaces_dropped_closed_client() {
        restart_replaces_client(false).await;
    }

    /// The validation must reject the historical default (expiration=10s,
    /// update_interval=30s). This combination is the root cause of issue
    /// #3851: the ownership record expires 20s before the next load-balancing
    /// cycle observes it, so every consumer reports `current=0` and
    /// re-claims every partition every cycle.
    #[test]
    fn historical_default_combination_is_rejected() {
        let result =
            validate_expiration_vs_update_interval(Duration::seconds(10), Duration::seconds(30));
        assert!(
            result.is_err(),
            "10s expiration with 30s interval must be rejected"
        );
    }

    /// The new default (expiration=60s, update_interval=30s) must be accepted.
    #[test]
    fn new_default_combination_is_accepted() {
        validate_expiration_vs_update_interval(Duration::seconds(60), Duration::seconds(30))
            .expect("60s expiration with 30s interval must be valid");
    }

    /// Equal values are rejected: if the record expires exactly at the
    /// instant the next cycle reads it, the read is racing the expiry.
    #[test]
    fn equal_values_are_rejected() {
        let result =
            validate_expiration_vs_update_interval(Duration::seconds(30), Duration::seconds(30));
        assert!(
            result.is_err(),
            "equal expiration and interval must be rejected"
        );
    }

    /// Custom configurations with adequate headroom are accepted.
    #[test]
    fn larger_expiration_is_accepted() {
        validate_expiration_vs_update_interval(Duration::seconds(120), Duration::seconds(60))
            .expect("2x ratio should be accepted");
    }
}
