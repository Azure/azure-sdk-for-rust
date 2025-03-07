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

#[derive(Debug, Default)]
pub struct StartPositions {
    pub(crate) per_partition: HashMap<String, StartPosition>,
    pub(crate) default: StartPosition,
}
