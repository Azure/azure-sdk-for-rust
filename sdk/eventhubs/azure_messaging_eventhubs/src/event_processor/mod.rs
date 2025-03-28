// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

/// Event Hubs Event Processor.
/// - Load Balancing : 30 seconds
/// - Ownership      : 2 minutes
///
/// Load Balancing Cycle
///  - Query Event Hub partitions; update state
///
///  - Renew Local Ownership
///    - Based on local state
///    - Each that the processor thinks it owns, call to SetMetadata.  May call Upload.
///    - If no longer owned, results in HTTP 412, local state updated to remove ownership   (seen in the logs; indicates stolen between load balancing cycles)
///
///  - List All Ownership
///    - Calls ListBlobs with metadata trait set
///    - If this fails, the load balancing cycle cannot continue; local state is preserved  (seen in logs; processors will fight and Event Hubs will enforce single reader)
///
///  - Calculate Ownership
///    - Update state for all ownership and all unowned partitions; expired ownership is unowned
///    - Determine number of active processors by looking at ownership; an active processor must own at least one partition
///
///  - Claim Ownership
///    - Determine if this instance has its fair share based on count of active processors
///    - If fair share is owned, do nothing; assume unowned will be claimed by a processor without its fair share
///    - If unowned partitions, pick one to claim at random
///    - If no unowned partitions, pick one to steal at random
///    - Update storage with any change, call to SetMetadata.  May call Upload.
///    - If claimed by another, results in HTTP 412, local state updated to remove ownership   (seen in the logs; indicates stolen between load balancing cycles)
///
///  - Determine balance stability
///    - If fair share of partitions are owned and no claim was needed, assume stable
///
///  - Ensure owned partitions are being processed
///    - If no current processing task, initialize and start
///    - If owned partition has a completed task, capture exceptions, initialize and restart
///
///  - Calculate next cycle time
///    - If greedy strategy and not stable, run immediate
///    - If elapsed time was more than the load balancing interval, run immediate
///    - Delay for (load balancing interval - current cycle time), then run next
///
/// CALLS PER CYCLE
///    - Event Hubs
///      - Query partitions (1)
///      - Create receiver, start reading (varies by claimed/faulted, max of owned partitions)
///
///    - Storage
///      - List Blobs (1)
///      - SetMetadata (varies by owned/claimed, max of partition count * 2)
///      - Upload (varies by new partitions, max of partition count)
///
pub mod load_balancer;
pub mod models;
pub mod partition_client;
pub mod processor;

use async_trait::async_trait;
use azure_core::Result;
use models::{Checkpoint, Ownership};

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
    ///
    /// # Returns
    /// A vector of claimed `Ownership` objects.
    ///
    /// # Errors
    /// Returns an error if the ownership claim fails.
    ///
    async fn claim_ownership(&self, ownerships: Vec<Ownership>) -> Result<Vec<Ownership>>;

    /// Lists the checkpoints for the specified Event Hub and consumer group.
    /// This method retrieves the checkpoints for a specific Event Hub and consumer group.
    ///
    /// # Arguments
    /// * `namespace` - The fully qualified namespace of the Event Hub.
    /// * `event_hub_name` - The name of the Event Hub.
    /// * `consumer_group` - The name of the consumer group.
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
    ) -> Result<Vec<Checkpoint>>;

    /// Lists the ownerships for the specified Event Hub and consumer group.
    /// This method retrieves the ownerships for a specific Event Hub and consumer group.
    ///
    /// # Arguments
    /// * `namespace` - The fully qualified namespace of the Event Hub.
    /// * `event_hub_name` - The name of the Event Hub.
    /// * `consumer_group` - The name of the consumer group.
    ///
    /// # Returns
    /// A vector of `Ownership` objects representing the ownerships for the specified Event Hub and consumer group.
    ///
    async fn list_ownerships(
        &self,
        namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
    ) -> Result<Vec<Ownership>>;

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
    async fn update_checkpoint(&self, checkpoint: Checkpoint) -> Result<()>;

    /// Updates the ownership for the specified partition.
    #[cfg(feature = "test_checkpoint_store")]
    async fn update_ownership(&self, ownership: Ownership) -> Result<()>;
}

#[derive(Clone, Debug, Copy)]
/// Represents the strategy for load balancing event processing.
///
/// This enum defines two strategies: `Balanced` and `Greedy`.
/// - `Balanced`: Distributes the load evenly across all partitions.
/// - `Greedy`: Assigns partitions to consumers in a way that maximizes throughput.
///
/// The choice of strategy can impact the performance and efficiency
/// of event processing, depending on the specific use case and workload.
/// The `Greedy` strategy is generally recommended for most scenarios, and is the default.
pub enum ProcessorStrategy {
    /// Balanced strategy for load balancing.
    ///
    /// The event processor will use a steady approach to claim ownership of partitions and slowly trend
    /// towards a stable state where all active processors will have an even distribution of Event Hub partitions.
    /// This strategy may take longer to settle into a balanced partition distribution among active processor
    /// instances. This strategy is geared towards minimizing ownership contention and reducing the need to transfer
    /// ownership frequently, especially when multiple instances are initialized together, until a stable state is
    /// reached.
    Balanced,

    /// Greedy strategy for load balancing.
    ///
    /// The event processor will attempt to claim its fair share of partition ownership greedily. This enables event
    /// processing of all partitions to start/resume quickly when there is an imbalance detected by the processor.
    /// This may result in ownership of partitions frequently changing when multiple instances are starting up
    /// but will eventually converge to a stable state.
    Greedy,
}
