// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use std::{collections::HashMap, time::SystemTime};

use azure_core::{error::ErrorKind as AzureErrorKind, Error, Result};

use crate::StartPosition;

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
    /// Creates a new Checkpoint instance with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `consumer_group` - The name of the consumer group.
    /// * `event_hub_name` - The name of the Event Hub.
    /// * `fully_qualified_namespace` - The fully qualified namespace of the Event Hub.
    /// * `partition_id` - The identifier of the partition.
    /// * `offset` - The offset of the last processed event (optional).
    /// * `sequence_number` - The sequence number of the last processed event (optional).
    ///
    pub fn new(
        fully_qualified_namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
        partition_id: &str,
        offset: Option<String>,
        sequence_number: Option<i64>,
    ) -> Self {
        Checkpoint {
            fully_qualified_namespace: fully_qualified_namespace.to_string(),
            event_hub_name: event_hub_name.to_string(),
            consumer_group: consumer_group.to_string(),
            partition_id: partition_id.to_string(),
            offset,
            sequence_number,
        }
    }

    /// Returns the consumer group associated with this checkpoint.
    pub fn consumer_group(&self) -> &str {
        &self.consumer_group
    }
    /// Returns the Event Hub name associated with this checkpoint.
    pub fn event_hub_name(&self) -> &str {
        &self.event_hub_name
    }
    /// Returns the fully qualified namespace associated with this checkpoint.
    pub fn fully_qualified_namespace(&self) -> &str {
        &self.fully_qualified_namespace
    }
    /// Returns the partition ID associated with this checkpoint.
    pub fn partition_id(&self) -> &str {
        &self.partition_id
    }
    /// Returns the offset of the last processed event.
    pub fn offset(&self) -> Option<&str> {
        self.offset.as_deref()
    }
    /// Returns the sequence number of the last processed event.
    pub fn sequence_number(&self) -> Option<i64> {
        self.sequence_number
    }

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
    pub(crate) etag: Option<azure_core::Etag>,
    /// The last modified time of the ownership.
    pub(crate) last_modified_time: Option<SystemTime>,
}

impl Ownership {
    /// Creates a new Ownership instance with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `consumer_group` - The name of the consumer group.
    /// * `event_hub_name` - The name of the Event Hub.
    /// * `fully_qualified_namespace` - The fully qualified namespace of the Event Hub.
    /// * `partition_id` - The identifier of the partition.
    /// * `owner_id` - The identifier of the owner (consumer) of the partition.
    /// * `etag` - The ETag associated with the ownership (optional).
    /// * `last_modified_time` - The last modified time of the ownership (optional).
    ///
    pub fn new(
        consumer_group: &str,
        event_hub_name: &str,
        fully_qualified_namespace: &str,
        partition_id: &str,
        owner_id: &str,
        etag: Option<azure_core::Etag>,
        last_modified_time: Option<SystemTime>,
    ) -> Self {
        Ownership {
            consumer_group: consumer_group.to_string(),
            event_hub_name: event_hub_name.to_string(),
            fully_qualified_namespace: fully_qualified_namespace.to_string(),
            partition_id: partition_id.to_string(),
            owner_id: owner_id.to_string(),
            etag,
            last_modified_time,
        }
    }
    /// Returns the consumer group associated with this ownership.
    pub fn consumer_group(&self) -> &str {
        &self.consumer_group
    }

    /// Returns the Event Hub name associated with this ownership.
    pub fn event_hub_name(&self) -> &str {
        &self.event_hub_name
    }

    /// Returns the fully qualified namespace associated with this ownership.
    pub fn fully_qualified_namespace(&self) -> &str {
        &self.fully_qualified_namespace
    }

    /// Returns the partition ID associated with this ownership.
    pub fn partition_id(&self) -> &str {
        &self.partition_id
    }

    /// Returns the owner ID associated with this ownership.
    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }
    /// Returns the ETag associated with this ownership.
    pub fn etag(&self) -> Option<&azure_core::Etag> {
        self.etag.as_ref()
    }
    /// Returns the last modified time associated with this ownership.
    pub fn last_modified_time(&self) -> Option<SystemTime> {
        self.last_modified_time
    }

    /// Sets the last modified time for this ownership.
    pub fn set_last_modified_time(&mut self, last_modified_time: SystemTime) {
        self.last_modified_time = Some(last_modified_time);
    }
    /// Sets the ETag for this ownership.
    pub fn set_etag(&mut self, etag: azure_core::Etag) {
        self.etag = Some(etag);
    }

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

/// Represents the starting position for event processing in an Event Hub.
///
/// This structure is used to specify the starting position for each partition
/// when processing events. It allows for flexibility in choosing the starting
/// position based on various criteria, such as the latest event, a specific
/// offset, or a specific sequence number.
#[derive(Debug, Default)]
pub struct StartPositions {
    /// The starting position for each partition in the Event Hub.
    /// The key is the partition ID, and the value is the starting position.
    /// The starting position can be specified as a specific offset, sequence number,
    /// or the latest event.
    pub per_partition: HashMap<String, StartPosition>,

    /// The default starting position for all partitions in the Event Hub.
    /// This position is used if no specific starting position is provided for a partition.
    /// The default starting position can be specified as a specific offset, sequence number,
    /// or the latest event.
    pub default: StartPosition,
}
