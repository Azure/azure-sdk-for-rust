// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

pub mod load_balancer;
pub mod models;
pub mod partition_client;
pub mod processor;

use async_trait::async_trait;
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
    async fn claim_ownership(
        &self,
        ownerships: Vec<Ownership>,
    ) -> azure_core::Result<Vec<Ownership>>;

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
    ) -> azure_core::Result<Vec<Checkpoint>>;

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
