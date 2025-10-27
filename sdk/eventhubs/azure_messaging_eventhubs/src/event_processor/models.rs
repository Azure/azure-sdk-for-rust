// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// Note that this module returns azure_core errors, *not* eventhub errors. That is because these structures are used by checkpoint stores which always return azure_core errors.
use crate::StartPosition;
use azure_core::{
    error::ErrorKind as AzureErrorKind, http::Etag, time::OffsetDateTime, Error, Result,
};
use std::collections::HashMap;

/// Represents a checkpoint in an Event Hub.
///
/// This structure is used to track the progress of event processing
/// by storing the offset and sequence number of the last processed event
/// for a specific partition. It helps in resuming event processing from
/// the correct position in case of failures or restarts.
#[derive(Debug, Default, Clone)]
pub struct Checkpoint {
    /// The fully qualified namespace of the Event Hub.
    pub fully_qualified_namespace: String,
    /// The name of the Event Hub.
    pub event_hub_name: String,
    /// The name of the consumer group.
    pub consumer_group: String,
    /// The identifier of the partition.
    pub partition_id: String,
    /// The offset of the last processed event.
    pub offset: Option<String>,
    /// The sequence number of the last processed event.
    pub sequence_number: Option<i64>,
}

macro_rules! check_non_empty_parameter(
    ($field:expr) => {
        if $field.is_empty() {
            return Err(Error::with_message(AzureErrorKind::Other,
                String::from("Required field ") + stringify!($field) + " is empty",
            ));
        }
    }
);

impl Checkpoint {
    /// Returns the prefix for the checkpoint blob name.
    pub fn get_checkpoint_blob_prefix_name(
        fully_qualified_namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
    ) -> Result<String> {
        check_non_empty_parameter!(fully_qualified_namespace);
        check_non_empty_parameter!(event_hub_name);
        check_non_empty_parameter!(consumer_group);
        Ok(fully_qualified_namespace.to_string()
            + "/"
            + event_hub_name
            + "/"
            + consumer_group
            + "/checkpoint/")
    }

    /// Returns the full name of the checkpoint blob.
    pub fn get_checkpoint_blob_name(
        fully_qualified_namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
        partition_id: &str,
    ) -> Result<String> {
        check_non_empty_parameter!(partition_id);
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
    /// The fully qualified namespace of the Event Hub.
    pub fully_qualified_namespace: String,
    /// The name of the Event Hub.
    pub event_hub_name: String,
    /// The name of the consumer group.
    pub consumer_group: String,
    /// The identifier of the partition.
    pub partition_id: String,

    /// The identifier of the owner (consumer) of the partition.
    pub owner_id: Option<String>,
    /// The ETag associated with the ownership.
    pub etag: Option<Etag>,
    /// The last modified time of the ownership.
    pub last_modified_time: Option<OffsetDateTime>,
}

impl Ownership {
    /// Returns the prefix for the ownership blob name.
    pub fn get_ownership_prefix_name(
        fully_qualified_namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
    ) -> Result<String> {
        check_non_empty_parameter!(fully_qualified_namespace);
        check_non_empty_parameter!(event_hub_name);
        check_non_empty_parameter!(consumer_group);
        Ok(fully_qualified_namespace.to_string()
            + "/"
            + event_hub_name
            + "/"
            + consumer_group
            + "/ownership/")
    }

    /// Returns the full name of the ownership blob.
    pub fn get_ownership_name(
        fully_qualified_namespace: &str,
        event_hub_name: &str,
        consumer_group: &str,
        partition_id: &str,
    ) -> Result<String> {
        check_non_empty_parameter!(fully_qualified_namespace);
        check_non_empty_parameter!(event_hub_name);
        check_non_empty_parameter!(consumer_group);
        check_non_empty_parameter!(partition_id);
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
