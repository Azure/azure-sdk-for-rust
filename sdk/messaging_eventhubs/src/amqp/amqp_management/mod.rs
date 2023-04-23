//! Management operations for the AMQP protocol.

use const_format::concatcp;

use super::amqp_constants::VENDOR;

pub(crate) mod event_hub_properties;
pub(crate) mod partition_properties;

/// The key to use for specifying an Event Hubs resource name
const RESOURCE_NAME_KEY: &str = "name";

/// The key to use for specifying a partition
const PARTITION_NAME_KEY: &str = "partition";

/// The key to use for specifying the type of Event Hubs resource
const RESOURCE_TYPE_KEY: &str = "type";

/// The key to use for specifying a security token
const SECURITY_TOKEN_KEY: &str = "security_token";

/// The value to specify when requesting a read-based operation
const READ_OPERATION_VALUE: &str = "READ";

/// The value to specify when identifying an Event Hub resource
const EVENT_HUB_RESOURCE_TYPE_VALUE: &str = concatcp!(VENDOR, ":eventhub");

/// The value to specify when identifying a partition resource
const PARTITION_RESOURCE_TYPE_VALUE: &str = concatcp!(VENDOR, ":partition");

pub(self) mod response_map {
    //! The set of property mappings to use for reading management-related
    //! responses from the Event Hubs service.

    /// The message property that identifies the name of a resource.
    pub(super) const NAME: &str = "name";

    ///   The message property that identifies the date/time that a resource was created.
    pub(super) const CREATED_AT: &str = "created_at";

    /// The message property that identifies the unique identifier associated with a partition.
    pub(super) const PARTITION_IDENTIFIER: &str = "partition";

    /// The message property that identifies the set of unique identifiers for each partition of an Event Hub.
    pub(super) const PARTITION_IDENTIFIERS: &str = "partition_ids";

    /// The message property that identifies the beginning sequence number in a partition.
    pub(super) const PARTITION_BEGIN_SEQUENCE_NUMBER: &str = "begin_sequence_number";

    /// The message property that identifies the last sequence number enqueued for a partition.
    pub(super) const PARTITION_LAST_ENQUEUED_SEQUENCE_NUMBER: &str =
        "last_enqueued_sequence_number";

    /// The message property that identifies the last offset enqueued for a partition.
    pub(super) const PARTITION_LAST_ENQUEUED_OFFSET: &str = "last_enqueued_offset";

    /// The message property that identifies the last time enqueued for a partition.
    pub(super) const PARTITION_LAST_ENQUEUED_TIME_UTC: &str = "last_enqueued_time_utc";

    /// The message property that identifies whether or not a partition is considered empty.
    pub(super) const PARTITION_RUNTIME_INFO_PARTITION_IS_EMPTY: &str = "is_partition_empty";
}
