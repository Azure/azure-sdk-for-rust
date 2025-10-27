// Copyright (c) Microsoft Corp. All Rights Reserved.
// Licensed under the MIT License.

//use async_channel::{bounded, Receiver, Sender};
use super::{
    load_balancer::LoadBalancer,
    models::{Checkpoint, StartPositions},
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
use tracing::{debug, error, info};

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
    partition_ids: Vec<String>,
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
    /// If a partition client already exists for the given partition ID,
    /// it will not be added again.
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
        info!("Adding partition client for partition: {}", partition_id);
        let mut consumers = self
            .consumers
            .lock()
            .map_err(|_| EventHubsError::with_message("Could not lock consumers mutex."))?;
        if consumers.contains_key(partition_id) {
            info!(
                "Partition client already exists for partition: {}",
                partition_id
            );
            return Ok(false);
        }
        consumers.insert(partition_id.to_string(), Arc::downgrade(&partition_client));
        info!("Consumers for partition: {:?}", consumers.keys());
        Ok(true)
    }

    pub fn remove_partition_client(&self, partition_id: &str) -> Result<()> {
        info!("Removing partition client for partition: {}", partition_id);
        let mut consumers = self
            .consumers
            .lock()
            .map_err(|_| EventHubsError::with_message("Could not lock consumers mutex."))?;
        consumers.remove(partition_id);
        info!("Consumers for partition now: {:?}", consumers.keys());
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
            partition_ids: options.partition_ids,
        }))
    }

    /// Starts the event processor.
    /// This method initiates the event processing loop and begins
    /// processing events from the Event Hub partitions.
    /// It uses the specified checkpoint store and load balancing strategy
    /// to manage the ownership of partitions and distribute the load
    /// among consumers.
    /// The event processor will run until it is stopped or interrupted.
    /// # Errors
    /// Returns an error if the event processor fails to start.
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
    ///       .with_partition_expiration_duration(Duration::seconds(10))
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
        {
            let mut is_running = self.is_running.lock().map_err(|_| {
                Error::new(AzureErrorKind::Io, "Could not lock is_running on startup")
            })?;
            *is_running = true;
        }

        let consumers = Arc::new(ProcessorConsumersMap::new());
        let partition_ids = &self
            .partition_ids
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>();

        loop {
            let result = self.dispatch(partition_ids, &consumers).await;
            match result {
                Ok(_) => {
                    debug!("Event processor dispatched successfully.");
                }
                Err(e) => {
                    info!("Error dispatching event processor: {:?}", e);
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
    pub async fn shutdown(&self) -> Result<()> {
        // Implement shutdown logic if needed

        let mut is_running = self.is_running.lock().map_err(|_| {
            EventHubsError::with_message("Failed to acquire lock on is_running for shutdown")
        })?;

        *is_running = false;
        Ok(())
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

    async fn dispatch(
        &self,
        partition_ids: &[&str],
        consumers: &Arc<ProcessorConsumersMap>,
    ) -> Result<()> {
        debug!("Dispatch partition clients to consumers.");
        let load_balancer = self.load_balancer.lock().await;

        let ownerships = load_balancer.load_balance(partition_ids).await;
        let ownerships = ownerships.map_err(|e| {
            error!("Error in load balancing: {:?}", e);
            e
        })?;

        let checkpoints = self.get_checkpoint_map().await;
        let checkpoints = checkpoints.map_err(|e| {
            error!("Error in getting checkpoint map: {:?}", e);
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
                error!("Error adding partition client: {:?}", e);
                return Err(e);
            }
        }

        Ok(())
    }

    async fn add_partition_client(
        &self,
        partition_id: String,
        checkpoints: &HashMap<String, Checkpoint>,
        consumers: Weak<ProcessorConsumersMap>,
    ) -> Result<()> {
        info!("Add partition client for partition ID: {:?}", partition_id);

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
                    "Partition client already exists for partition: {}, ignoring.",
                    partition_id
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
            "Start position for partition {}: {:?}",
            partition_id, start_position
        );
        let receiver = self
            .consumer_client
            .open_receiver_on_partition(
                partition_id.clone(),
                Some(OpenReceiverOptions {
                    start_position: Some(start_position),
                    prefetch: Some(self.prefetch),
                    ..Default::default()
                }),
            )
            .await;
        if let Err(e) = receiver {
            error!("Error opening receiver for partition client: {:?}", e);
            return Err(e);
        }
        info!("Receiver opened for partition client: {:?}", &partition_id);
        let receiver = receiver.unwrap();
        partition_client.set_event_receiver(receiver)?;

        info!("Adding partition client to queue.");

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
        info!(
            "add_partition_client: Partition client added for partition: {:?}",
            partition_id
        );

        Ok(())
    }

    /// Retrieves the next partition client for processing events.
    ///
    /// This method returns the next available partition client.
    pub async fn next_partition_client(&self) -> Result<Arc<PartitionClient>> {
        // Implement the function or remove it if not needed
        info!("next_partition_client: Waiting to receive the next partition client.",);

        {
            // Wait for the next partition client to be available
            let mut clients = self.next_partition_clients.lock().await;
            let next_client = clients.next().await.ok_or_else(|| {
                EventHubsError::with_message("No next partition client available: ")
            })?;

            info!(
                "next_partition_client: Returning partition client for partition {:?}.",
                next_client.get_partition_id()
            );
            Ok(next_client)
        }
    }

    /// Closes the event processor.
    pub async fn close(self) -> Result<()> {
        // Close all partition clients.
        info!("Closing all partition clients.");
        let mut clients = self.next_partition_clients.lock().await;
        while let Some(client) = clients.try_next().ok().flatten() {
            info!(
                "Closing partition client for partition: {}",
                client.get_partition_id()
            );
            let client = Arc::try_unwrap(client).map_err(|_| {
                EventHubsError::with_message("Partition client still has multiple references.")
            })?;
            let res = client.close().await;
            if let Err(e) = res {
                error!("Failed to close partition client: {:?}", e);
            } else {
                info!("Partition client closed successfully");
            }
        }

        // Close the event processor and release resources.
        info!("Closing consumer client.");
        let res = self.consumer_client.close().await;
        if let Err(e) = res {
            error!("Failed to close consumer client: {:?}", e);
        } else {
            info!("Consumer client closed successfully");
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
    const DEFAULT_PARTITION_EXPIRATION_DURATION: Duration = Duration::seconds(10);

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
        pub async fn build(
            self,
            consumer_client: ConsumerClient,
            checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
        ) -> Result<Arc<EventProcessor>> {
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
                    partition_expiration_duration: self
                        .partition_expiration_duration
                        .unwrap_or(DEFAULT_PARTITION_EXPIRATION_DURATION),
                    update_interval: self.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
                    start_positions: self.start_positions.unwrap_or_default(),
                    prefetch: self.prefetch.unwrap_or(DEFAULT_PREFETCH),
                    partition_ids: eh_properties.partition_ids,
                },
            )
        }
    }
}
