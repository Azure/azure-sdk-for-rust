use super::load_balancer::LoadBalancer;
use super::partition_client::PartitionClient;
use crate::models::ConsumerClientDetails;
use crate::{ConsumerClient, StartPosition};
use async_trait::async_trait;
use azure_core::Result;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub mod models {
    use std::time::SystemTime;

    use azure_core::{error::ErrorKind as AzureErrorKind, Error, Result};

    /// Represents a checkpoint in an Event Hub.
    ///
    /// This structure is used to track the progress of event processing
    /// by storing the offset and sequence number of the last processed event
    /// for a specific partition. It helps in resuming event processing from
    /// the correct position in case of failures or restarts.
    #[derive(Debug, Default, Clone)]
    pub struct Checkpoint {
        /// The name of the consumer group.
        pub(crate) consumer_group: String,
        /// The name of the Event Hub.
        pub(crate) event_hub_name: String,
        /// The fully qualified namespace of the Event Hub.
        pub(crate) fully_qualified_namespace: String,
        /// The identifier of the partition.
        pub(crate) partition_id: String,
        /// The offset of the last processed event.
        pub(crate) offset: Option<String>,
        /// The sequence number of the last processed event.
        pub(crate) sequence_number: Option<i64>,
    }

    impl Checkpoint {
        /// Returns the prefix for the checkpoint blob name.
        pub fn get_checkpoint_blob_prefix_name(
            fully_qualified_namespace: &str,
            event_hub_name: &str,
            consumer_group: &str,
        ) -> Result<String> {
            if fully_qualified_namespace.is_empty()
                || event_hub_name.is_empty()
                || consumer_group.is_empty()
            {
                return Err(Error::message(
                    AzureErrorKind::Other,
                    "Namespace, Event Hub name, or Consumer Group is empty",
                ));
            }
            Ok(fully_qualified_namespace.to_ascii_lowercase()
                + "/"
                + event_hub_name.to_ascii_lowercase().as_str()
                + "/"
                + consumer_group.to_ascii_lowercase().as_str()
                + "/checkpoint/")
        }

        /// Returns the full name of the checkpoint blob.
        pub fn get_checkpoint_blob_name(
            fully_qualified_namespace: &str,
            event_hub_name: &str,
            consumer_group: &str,
            partition_id: &str,
        ) -> Result<String> {
            if partition_id.is_empty() {
                return Err(Error::message(
                    AzureErrorKind::Other,
                    "Partition ID is empty",
                ));
            }
            Ok(Self::get_checkpoint_blob_prefix_name(
                fully_qualified_namespace,
                event_hub_name,
                consumer_group,
            )? + partition_id)
        }
    }

    /// Represents the ownership information for a partition in an Event Hub.
    ///
    /// This structure is used to manage and track the ownership of partitions
    /// by different consumers in a consumer group. It helps in load balancing
    /// and ensuring that each partition is processed by only one consumer at a time.
    #[derive(Debug, Default, Clone)]
    pub struct Ownership {
        /// The name of the consumer group.
        pub(crate) consumer_group: String,
        /// The name of the Event Hub.
        pub(crate) event_hub_name: String,
        /// The fully qualified namespace of the Event Hub.
        pub(crate) fully_qualified_namespace: String,
        /// The identifier of the partition.
        pub(crate) partition_id: String,

        /// The identifier of the owner (consumer) of the partition.
        pub(crate) owner_id: String,
        /// The ETag associated with the ownership.
        pub(crate) etag: Option<String>,
        /// The last modified time of the ownership.
        pub(crate) last_modified_time: Option<SystemTime>,
    }

    impl Ownership {
        /// Returns the prefix for the ownership blob name.
        pub fn get_ownership_prefix_name(
            fully_qualified_namespace: &str,
            event_hub_name: &str,
            consumer_group: &str,
        ) -> Result<String> {
            if fully_qualified_namespace.is_empty()
                || event_hub_name.is_empty()
                || consumer_group.is_empty()
            {
                return Err(Error::message(
                    AzureErrorKind::Other,
                    "Namespace, Event Hub name, or Consumer Group is empty",
                ));
            }
            Ok(fully_qualified_namespace.to_ascii_lowercase()
                + "/"
                + event_hub_name.to_ascii_lowercase().as_str()
                + "/"
                + consumer_group.to_ascii_lowercase().as_str()
                + "/ownership/")
        }

        /// Returns the full name of the ownership blob.
        pub fn get_ownership_name(
            fully_qualified_namespace: &str,
            event_hub_name: &str,
            consumer_group: &str,
            partition_id: &str,
        ) -> Result<String> {
            if partition_id.is_empty() {
                return Err(Error::message(
                    AzureErrorKind::Other,
                    "Partition ID is empty",
                ));
            }
            Ok(Self::get_ownership_prefix_name(
                fully_qualified_namespace,
                event_hub_name,
                consumer_group,
            )? + partition_id)
        }
    }
}
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
        ownerships: Vec<models::Ownership>,
        options: Option<ClaimOwnershipOptions>,
    ) -> azure_core::Result<Vec<models::Ownership>>;

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
    ) -> azure_core::Result<Vec<models::Checkpoint>>;

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
    ) -> azure_core::Result<Vec<models::Ownership>>;

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
    async fn update_checkpoint(&self, checkpoint: models::Checkpoint) -> azure_core::Result<()>;
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
    pub start_positions: Option<Vec<StartPosition>>,
    pub prefetch: Option<i32>,
}

const DEFAULT_PREFETCH: i32 = 300;
const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(30);
const DEFAULT_PARTITION_EXPIRATION_DURATION: Duration = Duration::from_secs(10);
pub struct EventProcessor {
    //    strategy: ProcessorStrategy,
    checkpoint_store: Arc<dyn CheckpointStore + Send + Sync>,
    load_balancer: Arc<Mutex<LoadBalancer>>,
    consumer_client: Arc<ConsumerClient>,
    running: Arc<Mutex<bool>>,
    processing_thread: Option<thread::JoinHandle<()>>,
    client_details: ConsumerClientDetails,
    prefetch: i32,
    update_interval: Duration,
    start_positions: Vec<StartPosition>,
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
        todo!()
    }

    pub async fn next_partition_client(&self) -> Result<PartitionClient> {
        todo!()
    }

    pub async fn close(&self) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::event_processor;
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
