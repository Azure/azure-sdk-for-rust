//! Management operations for the AMQP protocol.

use const_format::concatcp;

use super::amqp_constants::VENDOR;

pub(crate) mod event_hub_properties;
pub(crate) mod partition_properties;

// /// <summary>The location to specify for management operations.</summary>
// public const string Address = "$management";

// /// <summary>The type to specify for an AMQP link used for management operations.</summary>
// public const string LinkType = "svc";

// /// <summary>The key to use for specifying an Event Hubs resource name.</summary>
// public const string ResourceNameKey = "name";
const RESOURCE_NAME_KEY: &str = "name";

// /// <summary>The key to use for specifying a partition. </summary>
// public const string PartitionNameKey = "partition";
const PARTITION_NAME_KEY: &str = "partition";

// /// <summary>The key to use for specifying an operation.</summary>
// public const string OperationKey = "operation";

// /// <summary>The key to use for specifying the type of Event Hubs resource.</summary>
// public const string ResourceTypeKey = "type";
const RESOURCE_TYPE_KEY: &str = "type";

// /// <summary>The key to use for specifying a security token.</summary>
// public const string SecurityTokenKey = "security_token";
const SECURITY_TOKEN_KEY: &str = "security_token";

// /// <summary>The value to specify when requesting a read-based operation.</summary>
// public const string ReadOperationValue = "READ";
const READ_OPERATION_VALUE: &str = "READ";

// /// <summary>The value to specify when identifying an Event Hub resource.</summary>
// public const string EventHubResourceTypeValue = AmqpConstants.Vendor + ":eventhub";
const EVENT_HUB_RESOURCE_TYPE_VALUE: &str = concatcp!(VENDOR, ":eventhub");

// /// <summary>The value to specify when identifying a partition resource.</summary>
// public const string PartitionResourceTypeValue = AmqpConstants.Vendor + ":partition";
const PARTITION_RESOURCE_TYPE_VALUE: &str = concatcp!(VENDOR, ":partition");

// /// <summary>
// ///   The set of property mappings to use for reading management-related
// ///   responses from the Event Hubs service.
// /// </summary>
// ///
// public static class ResponseMap
pub(self) mod response_map {
    // {
    //     /// <summary>
    //     ///   The message property that identifies the name of a resource.
    //     /// </summary>
    //     ///
    //     public static MapKey Name { get; } = new MapKey("name");
    pub(super) const NAME: &str = "name";

    //     /// <summary>
    //     ///   The message property that identifies the date/time that a resource was created.
    //     /// </summary>
    //     ///
    //     public static MapKey CreatedAt { get; } = new MapKey("created_at");
    pub(super) const CREATED_AT: &str = "created_at";

    //     /// <summary>
    //     ///   The message property that identifies the unique identifier associated with a partition.
    //     /// </summary>
    //     ///
    //     public static MapKey PartitionIdentifier { get; } = new MapKey("partition");
    pub(super) const PARTITION_IDENTIFIER: &str = "partition";

    //     /// <summary>
    //     ///   The message property that identifies the set of unique identifiers for each partition of an Event Hub.
    //     /// </summary>
    //     ///
    //     public static MapKey PartitionIdentifiers { get; } = new MapKey("partition_ids");
    pub(super) const PARTITION_IDENTIFIERS: &str = "partition_ids";

    //     /// <summary>
    //     ///   The message property that identifies the beginning sequence number in a partition.
    //     /// </summary>
    //     ///
    //     public static MapKey PartitionBeginSequenceNumber { get; } = new MapKey("begin_sequence_number");
    pub(super) const PARTITION_BEGIN_SEQUENCE_NUMBER: &str = "begin_sequence_number";

    //     /// <summary>
    //     ///   The message property that identifies the last sequence number enqueued for a partition.
    //     /// </summary>
    //     ///
    //     public static MapKey PartitionLastEnqueuedSequenceNumber { get; } = new MapKey("last_enqueued_sequence_number");
    pub(super) const PARTITION_LAST_ENQUEUED_SEQUENCE_NUMBER: &str =
        "last_enqueued_sequence_number";

    //     /// <summary>
    //     ///   The message property that identifies the last offset enqueued for a partition.
    //     /// </summary>
    //     ///
    //     public static MapKey PartitionLastEnqueuedOffset { get; } = new MapKey("last_enqueued_offset");
    pub(super) const PARTITION_LAST_ENQUEUED_OFFSET: &str = "last_enqueued_offset";

    //     /// <summary>
    //     ///   The message property that identifies the last time enqueued for a partition.
    //     /// </summary>
    //     ///
    //     public static MapKey PartitionLastEnqueuedTimeUtc { get; } = new MapKey("last_enqueued_time_utc");
    pub(super) const PARTITION_LAST_ENQUEUED_TIME_UTC: &str = "last_enqueued_time_utc";

    //     /// <summary>
    //     ///   The message property that identifies the date and time, in UTC, that partition information was sent from the Event Hubs service.
    //     /// </summary>
    //     ///
    //     public static MapKey PartitionRuntimeInfoRetrievalTimeUtc { get; } = new MapKey("runtime_info_retrieval_time_utc");
    pub(super) const PARTITION_RUNTIME_INFO_RETRIEVAL_TIME_UTC: &str =
        "runtime_info_retrieval_time_utc";

    /// The message property that identifies whether or not a partition is considered empty.
    pub(super) const PARTITION_RUNTIME_INFO_PARTITION_IS_EMPTY: &str = "is_partition_empty";
}
