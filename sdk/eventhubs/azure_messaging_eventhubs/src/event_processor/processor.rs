// Copyright (c) Microsoft Corp. All Rights Reserved.
// Licensed under the MIT License.

use super::{
    load_balancer::LoadBalancer,
    models::{Checkpoint, Ownership, StartPositions},
    partition_client::PartitionClient,
};
use crate::{
    models::{ConsumerClientDetails, EventHubProperties},
    ConsumerClient, EventReceiver, OpenReceiverOptions, StartLocation, StartPosition,
};
use async_trait::async_trait;
use azure_core::{error::ErrorKind as AzureErrorKind, Error, Result};
use futures::channel::mpsc::{Receiver, Sender};
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::info;

#[allow(dead_code)]
pub struct ClaimOwnershipOptions {}

#[allow(dead_code)]
pub struct ListCheckpointsOptions {}

#[allow(dead_code)]
pub struct ListOwnershipOptions {}

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
    Balanced,
    Greedy,
}

#[derive(Default)]
pub struct EventProcessorOptions {
    pub load_balancing_strategy: Option<ProcessorStrategy>,
    pub update_interval: Option<Duration>,
    pub partition_expiration_duration: Option<Duration>,
    pub start_positions: Option<StartPositions>,
    pub prefetch: Option<u32>,
    pub max_partition_count: Option<usize>,
}

const DEFAULT_PREFETCH: u32 = 300;
const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(30);
const DEFAULT_PARTITION_EXPIRATION_DURATION: Duration = Duration::from_secs(10);
pub struct EventProcessor {
    //    strategy: ProcessorStrategy,
    checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
    load_balancer: Arc<Mutex<LoadBalancer>>,
    consumer_client: Arc<ConsumerClient>,
    next_partition_clients: Mutex<Option<Receiver<PartitionClient>>>,
    next_partition_client_sender: OnceLock<Sender<PartitionClient>>,
    running: Arc<Mutex<bool>>,
    processing_thread: Option<thread::JoinHandle<()>>,
    client_details: ConsumerClientDetails,
    prefetch: u32,
    update_interval: Duration,
    start_positions: StartPositions,
    max_partition_count: Option<usize>,
}

impl EventProcessor {
    pub fn new(
        consumer_client: Arc<ConsumerClient>,
        checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
        options: Option<EventProcessorOptions>,
    ) -> Self {
        let options = options.unwrap_or_default();
        let strategy = options
            .load_balancing_strategy
            .unwrap_or(ProcessorStrategy::Balanced);
        EventProcessor {
            checkpoint_store: checkpoint_store.clone(),
            consumer_client: consumer_client.clone(),
            running: Arc::new(Mutex::new(false)),
            //          strategy,

            // Default to Balanced strategy if not provided
            processing_thread: None,
            load_balancer: Arc::new(Mutex::new(LoadBalancer::new(
                checkpoint_store.clone(),
                consumer_client.get_details(),
                strategy,
                options
                    .partition_expiration_duration
                    .unwrap_or(DEFAULT_PARTITION_EXPIRATION_DURATION),
            ))),
            client_details: consumer_client.get_details(),
            prefetch: options.prefetch.unwrap_or(DEFAULT_PREFETCH),
            update_interval: options.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
            start_positions: options.start_positions.unwrap_or_default(),
            max_partition_count: options.max_partition_count,
            next_partition_client_sender: OnceLock::new(),
            next_partition_clients: Mutex::new(None),
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
    /// use azure_messaging_eventhubs::event_processor::EventProcessor;
    /// use azure_messaging_eventhubs::models::CheckpointStore;
    /// use azure_messaging_eventhubs::models::ConsumerClient;
    /// use std::sync::Arc;
    /// use std::time::Duration;
    /// use std::thread;
    /// use std::sync::Mutex;
    /// use azure_core::Result;
    /// use azure_messaging_eventhubs::event_processor::EventProcessorOptions;
    /// use azure_messaging_eventhubs::event_processor::ProcessorStrategy;
    /// use azure_messaging_eventhubs::event_processor::ClaimOwnershipOptions;
    /// use azure_messaging_eventhubs::event_processor::ListCheckpointsOptions;
    /// use azure_messaging_eventhubs::event_processor::ListOwnershipOptions;
    /// use azure_messaging_eventhubs::event_processor::Checkpoint;
    /// use azure_messaging_eventhubs::event_processor::Ownership;
    /// use azure_messaging_eventhubs::event_processor::CheckpointStore;
    ///
    /// // Create an instance of the EventProcessor
    /// let event_processor = EventProcessor::new(
    ///     Arc::new(ConsumerClient::default()),
    ///     Arc::new(MyCheckpointStore::default()),
    ///     Some(EventProcessorOptions {
    ///         load_balancing_strategy: Some(ProcessorStrategy::Balanced),
    ///         update_interval: Some(Duration::from_secs(30)),
    ///         partition_expiration_duration: Some(Duration::from_secs(10)),
    ///         prefetch: Some(300),
    ///         ..Default::default()
    ///     });
    ///
    /// // Start the event processor
    /// let join_handle = tokio::spawn(async move {
    ///     event_processor.run().await?;
    ///     Ok(())
    /// });
    /// // Wait for the event processor to finish
    /// thread::sleep(Duration::from_secs(60));
    ///
    /// join_handle.drop()
    ///
    /// ```
    ///
    pub async fn run(&self) -> Result<()> {
        let mut eh_properties = self.consumer_client.get_eventhub_properties().await?;
        if let Some(max_partition_count) = self.max_partition_count {
            eh_properties
                .partition_ids
                .truncate(max_partition_count as usize);
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

        let mut consumers: HashMap<String, PartitionClient> = HashMap::new();
        loop {
            self.dispatch(&eh_properties, &mut consumers).await?;
        }
    }

    async fn dispatch(
        &self,
        eventhub_properties: &EventHubProperties,
        consumers: &mut HashMap<String, PartitionClient>,
    ) -> Result<()> {
        let load_balancer = self.load_balancer.lock().await;

        let ownerships = load_balancer
            .load_balance(&eventhub_properties.partition_ids)
            .await?;

        let checkpoints = self.get_checkpoint_map().await?;

        for ownership in ownerships {
            self.add_partition_client(ownership, &checkpoints, consumers)
                .await?;
        }

        Ok(())
    }

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

    pub async fn close(&self) -> Result<()> {
        // Implement the function or remove it if not needed
        let mut running = self.running.lock().await;
        *running = false;
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
    pub async fn get_checkpoint_map(&self) -> Result<HashMap<String, Checkpoint>> {
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

    pub async fn add_partition_client(
        &self,
        ownership: Ownership,
        checkpoints: &HashMap<String, Checkpoint>,
        consumers: &mut HashMap<String, PartitionClient>,
    ) -> Result<()> {
        info!("Add partition client for ownership: {:?}", ownership);

        let partition_client = PartitionClient::new(
            &ownership.partition_id,
            self.checkpoint_store.clone(),
            &self.client_details,
            || {
                // Handle partition client destruction
                consumers.remove(&ownership.partition_id);
            },
        );

        // Ignore the ownership if the partition client already exists
        if consumers.contains_key(&ownership.partition_id) {
            info!(
                "Partition client already exists for partition: {:?}",
                ownership.partition_id
            );
            return Ok(());
        }

        let start_position = self.get_start_position(&ownership, checkpoints);
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
        partition_client.set_event_receiver(receiver);
        consumers.insert(ownership.partition_id.clone(), partition_client);

        Ok(())
    }

    fn get_start_position(
        &self,
        ownership: &Ownership,
        checkpoints: &HashMap<String, Checkpoint>,
    ) -> StartPosition {
        let mut start_position = self.start_positions.default.clone();
        if checkpoints.contains_key(&ownership.partition_id) {
            let checkpoint = checkpoints.get(&ownership.partition_id);
            if let Some(checkpoint) = checkpoint {
                if let Some(offset) = &checkpoint.offset {
                    start_position.location = StartLocation::Offset(offset.clone());
                } else if let Some(sequence_number) = checkpoint.sequence_number {
                    start_position.location = StartLocation::SequenceNumber(sequence_number);
                }
            }
        }
        start_position
    }
}

#[cfg(test)]
mod test {
    use crate::event_processor::in_memory_checkpoint_store::InMemoryCheckpointStore;
    use crate::{
        event_processor::processor::{EventProcessor, EventProcessorOptions, ProcessorStrategy},
        ConsumerClient,
    };
    use azure_core_test::*;
    use std::sync::Arc;
    use std::time::Duration;
    use tracing::event;

    #[recorded::test]
    async fn start_processor(ctx: TestContext) -> azure_core::Result<()> {
        let recording = ctx.recording();

        let consumer_client = ConsumerClient::builder()
            .open(
                recording.var("EVENTHUBS_HOST", None).as_str(),
                recording.var("EVENTHUB_NAME", None).as_str(),
                recording.credential().clone(),
            )
            .await?;

        let event_processor = EventProcessor::new(
            Arc::new(consumer_client),
            Arc::new(InMemoryCheckpointStore::new()),
            Some(EventProcessorOptions {
                load_balancing_strategy: Some(ProcessorStrategy::Balanced),
                update_interval: Some(Duration::from_secs(30)),
                partition_expiration_duration: Some(Duration::from_secs(10)),
                prefetch: Some(300),
                ..Default::default()
            }),
        );

        let event_processor = Arc::new(event_processor);
        {
            //            let event_processor_clone = Arc::clone(&event_processor);
            let event_processor_clone = event_processor.clone();
            let jh = tokio::spawn(async move { event_processor_clone.run().await });

            let r = jh.await;

            match r {
                Ok(_) => {
                    event!(tracing::Level::INFO, "Event processor ran successfully");
                }
                Err(e) => {
                    event!(
                        tracing::Level::ERROR,
                        "Failed to run event processor: {}",
                        e
                    );
                }
            }
        }

        event_processor.close().await?;

        // Start the event processor
        // Wait for the event processor to finish
        //        let partition_manager = event_processor.run().await?;

        Ok(())
    }
}
