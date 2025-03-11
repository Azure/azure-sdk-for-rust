// Copyright (c) Microsoft Corp. All Rights Reserved.
// Licensed under the MIT License.

use super::{
    load_balancer::LoadBalancer,
    models::{Checkpoint, Ownership, StartPositions},
    partition_client::PartitionClient,
};
use crate::{
    models::{ConsumerClientDetails, EventHubProperties},
    ConsumerClient, OpenReceiverOptions, StartLocation, StartPosition,
};
use async_io::Timer;
use async_lock::Mutex as AsyncMutex;
use async_trait::async_trait;
use azure_core::{error::ErrorKind as AzureErrorKind, Error, Result};
use futures::channel::mpsc::{Receiver, Sender};
use futures::StreamExt;
use std::{
    sync::{Arc, OnceLock},
    time::Duration,
    {collections::HashMap, sync::Weak},
};
use tracing::{debug, info};

/// Options for the [CheckpointStore.claim_ownership()] method.
pub struct ClaimOwnershipOptions {}

/// Options for the [CheckpointStore.list_checkpoints()] method.
pub struct ListCheckpointsOptions {}

/// Options for the [CheckpointStore.list_ownerships()] method.
pub struct ListOwnershipOptions {}

/// Trait representing a checkpoint store.
///
/// This trait defines the methods required for managing checkpoints
/// and ownerships in an Event Hub.
/// It allows for claiming ownership of partitions, listing checkpoints,
/// listing ownerships, and updating checkpoints.
#[async_trait]
pub trait CheckpointStore: Send + Sync {
    /// Claims ownership of the specified partitions.
    /// This method is used to claim ownership of partitions in an Event Hub
    ///
    /// # Arguments
    /// * `ownerships` - A vector of `Ownership` objects representing the partitions to claim.
    /// * `options` - Optional parameters for claiming ownership.
    ///
    /// # Returns
    /// A vector of claimed `Ownership` objects.
    ///
    /// # Errors
    /// Returns an error if the ownership claim fails.
    ///
    async fn claim_ownership(
        &self,
        ownerships: Vec<Ownership>,
        options: Option<ClaimOwnershipOptions>,
    ) -> azure_core::Result<Vec<Ownership>>;

    /// Lists the checkpoints for the specified Event Hub and consumer group.
    /// This method retrieves the checkpoints for a specific Event Hub and consumer group.
    ///
    /// # Arguments
    /// * `namespace` - The fully qualified namespace of the Event Hub.
    /// * `event_hub_name` - The name of the Event Hub.
    /// * `consumer_group` - The name of the consumer group.
    /// * `options` - Optional parameters for listing checkpoints.
    ///
    /// # Returns
    /// A vector of `Checkpoint` objects representing the checkpoints for the specified Event Hub and consumer group.
    ///
    /// # Errors
    /// Returns an error if the listing of checkpoints fails.
    ///
    async fn list_checkpoints(
        &self,
        namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
        options: Option<ListCheckpointsOptions>,
    ) -> azure_core::Result<Vec<Checkpoint>>;

    /// Lists the ownerships for the specified Event Hub and consumer group.
    /// This method retrieves the ownerships for a specific Event Hub and consumer group.
    ///
    /// # Arguments
    /// * `namespace` - The fully qualified namespace of the Event Hub.
    /// * `event_hub_name` - The name of the Event Hub.
    /// * `consumer_group` - The name of the consumer group.
    /// * `options` - Optional parameters for listing ownerships.
    ///
    /// # Returns
    /// A vector of `Ownership` objects representing the ownerships for the specified Event Hub and consumer group.
    ///
    async fn list_ownerships(
        &self,
        namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
        options: Option<ListOwnershipOptions>,
    ) -> azure_core::Result<Vec<Ownership>>;

    /// Updates the checkpoint for the specified partition.
    /// This method updates the checkpoint information for a specific partition in an Event Hub.
    ///
    /// # Arguments
    /// * `checkpoint` - The `Checkpoint` object representing the checkpoint to update.
    ///
    /// # Returns
    /// Returns `Ok(())` if the update is successful.
    /// Returns an error if the update fails.
    ///
    async fn update_checkpoint(&self, checkpoint: Checkpoint) -> azure_core::Result<()>;
}

#[derive(Clone, Debug)]
/// Represents the strategy for load balancing event processing.
/// This enum defines two strategies: `Balanced` and `Greedy`.
/// - `Balanced`: Distributes the load evenly across all partitions.
/// - `Greedy`: Assigns partitions to consumers in a way that maximizes throughput.
///
/// The choice of strategy can impact the performance and efficiency
/// of event processing, depending on the specific use case and workload.
/// The `Balanced` strategy is generally recommended for most scenarios,
/// while the `Greedy` strategy may be more suitable for high-throughput
/// scenarios where maximizing throughput is a priority.
pub enum ProcessorStrategy {
    /// Balanced strategy for load balancing.
    ///
    /// This strategy distributes the load evenly across all partitions,
    /// ensuring that each consumer processes a similar amount of data.
    /// This is the default strategy and is generally recommended for most scenarios.
    /// It helps to avoid overloading a single consumer and ensures that
    /// all consumers are utilized effectively.
    Balanced,

    /// Greedy strategy for load balancing.
    ///
    /// This strategy may lead to uneven distribution of load across partitions.
    /// It is generally not recommended for most scenarios.
    /// Use this strategy only if you have a specific reason to do so.
    /// For example, if you want to maximize throughput for a specific partition
    /// or if you have a specific partitioning scheme that benefits from this approach.
    /// Use this strategy with caution, as it may lead to uneven load distribution
    /// and potential performance issues.
    Greedy,
}

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
/// use an Event Processor, see the [Event Processor documentation]
/// (https://learn.microsoft.com/azure/event-hubs/event-processor-balance-partition-load).
///
pub struct EventProcessor {
    //    strategy: ProcessorStrategy,
    checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
    load_balancer: Arc<AsyncMutex<LoadBalancer>>,
    consumer_client: Arc<ConsumerClient>,
    next_partition_clients: AsyncMutex<Option<Receiver<PartitionClient>>>,
    next_partition_client_sender: OnceLock<Sender<PartitionClient>>,
    client_details: ConsumerClientDetails,
    prefetch: u32,
    update_interval: Duration,
    start_positions: StartPositions,
    max_partition_count: Option<usize>,
}

struct EventProcessorOptions {
    strategy: ProcessorStrategy,
    partition_expiration_duration: Duration,
    update_interval: Duration,
    start_positions: StartPositions,
    max_partition_count: Option<usize>,
    prefetch: u32,
}

type ConsumersType = std::sync::Mutex<HashMap<String, PartitionClient>>;

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
        consumer_client: Arc<ConsumerClient>,
        checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
        options: EventProcessorOptions,
    ) -> Self {
        EventProcessor {
            checkpoint_store: checkpoint_store.clone(),
            consumer_client: consumer_client.clone(),

            // Default to Balanced strategy if not provided
            load_balancer: Arc::new(AsyncMutex::new(LoadBalancer::new(
                checkpoint_store.clone(),
                consumer_client.get_details(),
                options.strategy,
                options.partition_expiration_duration,
            ))),
            client_details: consumer_client.get_details(),
            prefetch: options.prefetch,
            update_interval: options.update_interval,
            start_positions: options.start_positions,
            max_partition_count: options.max_partition_count,
            next_partition_client_sender: OnceLock::new(),
            next_partition_clients: AsyncMutex::new(None),
        }
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
    /// use std::time::Duration;
    /// use std::thread;
    /// use std::sync::Mutex;
    /// use azure_core::Result;
    /// use azure_messaging_eventhubs::ProcessorStrategy;
    /// use azure_messaging_eventhubs::models::Checkpoint;
    /// use azure_messaging_eventhubs::models::Ownership;
    /// use azure_messaging_eventhubs::CheckpointStore;
    ///
    /// // Create an instance of the EventProcessor
    /// let event_processor = EventProcessor::builder()
    ///        .with_load_balancing_strategy(ProcessorStrategy::Balanced)
    ///       .with_update_interval(Duration::from_secs(30))
    ///       .with_partition_expiration_duration(Duration::from_secs(10))
    ///       .with_prefetch(300)
    ///       .build(
    ///           Arc::new(ConsumerClient::default()),
    ///          Arc::new(MyCheckpointStore::default()));
    ///
    /// // Start the event processor
    /// let join_handle = tokio::spawn(async move {
    ///     event_processor.run().await?;
    ///     Ok(())
    /// });
    /// // Wait for the event processor to finish
    /// thread::sleep(Duration::from_secs(60));
    ///
    /// join_handle.abort();
    ///
    /// ```
    ///
    pub async fn run(&self) -> Result<()> {
        let mut eh_properties = self.consumer_client.get_eventhub_properties().await?;
        if let Some(max_partition_count) = self.max_partition_count {
            eh_properties.partition_ids.truncate(max_partition_count);
        }
        let (sender, mut receiver) =
            futures::channel::mpsc::channel(eh_properties.partition_ids.len());
        self.next_partition_client_sender.set(sender).map_err(|_| {
            Error::message(
                AzureErrorKind::Other,
                "Failed to set next partition client sender",
            )
        })?;
        let mut partition_client = self.next_partition_clients.lock().await;
        partition_client.as_mut().replace(&mut receiver);
        let consumers: Arc<ConsumersType> = Arc::new(std::sync::Mutex::new(HashMap::new()));
        loop {
            let result = self.dispatch(&eh_properties, &consumers).await;
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
            Timer::after(self.update_interval).await;
            debug!("Event processor waking up.");
        }
    }

    async fn dispatch(
        &self,
        eventhub_properties: &EventHubProperties,
        consumers: &Arc<ConsumersType>,
    ) -> Result<()> {
        debug!("Dispatch partition clients to consumers.");
        let load_balancer = self.load_balancer.lock().await;

        let ownerships = load_balancer
            .load_balance(&eventhub_properties.partition_ids)
            .await?;

        let checkpoints = self.get_checkpoint_map().await?;

        for ownership in ownerships {
            self.add_partition_client(ownership, &checkpoints, Arc::downgrade(consumers))
                .await?;
        }

        Ok(())
    }

    /// Retrieves the next partition client for processing events.
    ///
    /// This method returns the next available partition client.
    pub async fn next_partition_client(&self) -> Result<PartitionClient> {
        let mut locked_partition_clients = self.next_partition_clients.lock().await;
        // Implement the function or remove it if not needed
        let receiver = locked_partition_clients.as_mut().ok_or_else(|| {
            azure_core::Error::message(
                AzureErrorKind::Other,
                "Unable to retrieve next partition client - is the processor running?",
            )
        })?;
        let next_client = receiver.next().await.ok_or_else(|| {
            azure_core::Error::message(AzureErrorKind::Other, "No next partition client available")
        })?;

        Ok(next_client)
    }

    /// Closes the event processor.
    pub async fn close(self) -> Result<()> {
        // Close the event processor and release resources.
        let consumer = Arc::into_inner(self.consumer_client);
        if let Some(consumer) = consumer {
            info!("Closing consumer client.");
            consumer.close().await?;
        } else {
            info!("Consumer client externally referenced.");
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
            None,
        );
        let mut checkpoint_map = HashMap::new();
        for checkpoint in checkpoints.await? {
            checkpoint_map.insert(checkpoint.partition_id.clone(), checkpoint);
        }
        Ok(checkpoint_map)
    }

    async fn add_partition_client(
        &self,
        ownership: Ownership,
        checkpoints: &HashMap<String, Checkpoint>,
        consumers: Weak<ConsumersType>,
    ) -> Result<()> {
        info!("Add partition client for ownership: {:?}", ownership);

        let cloned_consumers = consumers.clone();
        let partition_id = ownership.partition_id.clone();
        let partition_client = PartitionClient::new(
            &ownership.partition_id,
            self.checkpoint_store.clone(),
            &self.client_details,
            move || {
                if let Some(strong_consumers) = cloned_consumers.upgrade() {
                    if let Ok(mut strong_consumers) = strong_consumers.lock() {
                        // Handle partition client destruction
                        strong_consumers.remove(&partition_id);
                    }
                }
            },
        );

        let start_position = self.get_start_position(&ownership.partition_id, checkpoints);
        let receiver = self
            .consumer_client
            .open_receiver_on_partition(
                &ownership.partition_id,
                Some(OpenReceiverOptions {
                    start_position: Some(start_position),
                    receive_timeout: Some(self.update_interval),
                    prefetch: Some(self.prefetch),
                    ..Default::default()
                }),
            )
            .await?;
        partition_client.set_event_receiver(receiver)?;

        if let Some(strong_consumers) = consumers.upgrade() {
            if let Ok(mut strong_consumers) = strong_consumers.lock() {
                // Check if the partition client already exists
                if strong_consumers.contains_key(&ownership.partition_id) {
                    info!(
                        "Partition client already exists for partition: {:?}",
                        ownership.partition_id
                    );
                    return Ok(());
                }

                strong_consumers.insert(ownership.partition_id.clone(), partition_client);
            }
        }

        Ok(())
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
    /// * `ownership` - The ownership information for the partition.
    /// * `checkpoints` - A map of checkpoints for all partitions.
    ///
    fn get_start_position(
        &self,
        partition_id: &String,
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
    use crate::event_processor::models::StartPositions;
    use crate::ConsumerClient;
    use azure_core::Result;
    use std::sync::Arc;
    use std::time::Duration;

    const DEFAULT_PREFETCH: u32 = 300;
    const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(30);
    const DEFAULT_PARTITION_EXPIRATION_DURATION: Duration = Duration::from_secs(10);

    /// Builder for creating an `EventProcessor`.
    /// This builder allows you to configure various options for the event processor,
    /// such as load balancing strategy, update interval, start positions, and more.
    /// It provides a fluent interface for setting these options and building the event processor.
    /// # Examples
    /// ```
    /// use azure_messaging_eventhubs::EventProcessor;
    /// use azure_messaging_eventhubs::CheckpointStore;
    /// use azure_messaging_eventhubs::ConsumerClient;
    /// use std::sync::Arc;
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
        /// The default strategy is `Balanced`.
        pub fn with_load_balancing_strategy(
            mut self,
            load_balancing_strategy: super::ProcessorStrategy,
        ) -> Self {
            self.load_balancing_strategy = Some(load_balancing_strategy);
            self
        }

        /// Sets the partition expiration duration for the event processor.
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
        pub fn build(
            self,
            consumer_client: Arc<ConsumerClient>,
            checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
        ) -> Result<Arc<EventProcessor>> {
            Ok(Arc::new(EventProcessor::new(
                consumer_client,
                checkpoint_store,
                super::EventProcessorOptions {
                    strategy: self
                        .load_balancing_strategy
                        .unwrap_or(super::ProcessorStrategy::Balanced),
                    partition_expiration_duration: self
                        .partition_expiration_duration
                        .unwrap_or(DEFAULT_PARTITION_EXPIRATION_DURATION),
                    update_interval: self.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
                    start_positions: self.start_positions.unwrap_or_default(),
                    max_partition_count: self.max_partition_count,
                    prefetch: self.prefetch.unwrap_or(DEFAULT_PREFETCH),
                },
            )))
        }
    }
}
