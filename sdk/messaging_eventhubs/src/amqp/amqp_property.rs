use const_format::concatcp;
use serde_amqp::primitives::SymbolRef;

use crate::amqp::amqp_constants;

/// The owner level (a.k.a. epoch) to associate with a receiver link.
pub(crate) const CONSUMER_OWNER_LEVEL: SymbolRef =
    SymbolRef(concatcp!(amqp_constants::VENDOR, ":epoch"));

/// The consumer identifier to associate with a receiver link.
pub(crate) const CONSUMER_IDENTIFIER: SymbolRef =
    SymbolRef(concatcp!(amqp_constants::VENDOR, ":receiver-name"));

/// The owner level (a.k.a. epoch) to associate with a sending link.
pub(crate) const PRODUCER_OWNER_LEVEL: SymbolRef =
    SymbolRef(concatcp!(amqp_constants::VENDOR, ":producer-epoch"));

/// The type of Event Hubs entity to associate with a link.
pub(crate) const ENTITY_TYPE: SymbolRef =
    SymbolRef(concatcp!(amqp_constants::VENDOR, ":entity-type"));

/// The capability for tracking the last event enqueued in a partition, to associate with a link.
pub(crate) const TRACK_LAST_ENQUEUED_EVENT_PROPERTIES: SymbolRef = SymbolRef(concatcp!(
    amqp_constants::VENDOR,
    ":enable-receiver-runtime-metric"
));

/// The capability for opting-into idempotent publishing.
pub(crate) const ENABLE_IDEMPOTENT_PUBLISHING: SymbolRef =
    SymbolRef(concatcp!(amqp_constants::VENDOR, ":idempotent-producer"));

/// The identifier of the producer group to associate with a producer.
pub(crate) const PRODUCER_GROUP_ID: SymbolRef =
    SymbolRef(concatcp!(amqp_constants::VENDOR, ":producer-id"));

/// The sequence number assigned by a producer to an event when it was published.
pub(crate) const PRODUCER_SEQUENCE_NUMBER: SymbolRef = SymbolRef(concatcp!(
    amqp_constants::VENDOR,
    ":producer-sequence-number"
));

/// The timeout to associate with a link.
pub(crate) const TIMEOUT: SymbolRef = SymbolRef(concatcp!(amqp_constants::VENDOR, ":timeout"));

/// The date and time, in UTC, that a message was enqueued.
pub(crate) const ENQUEUED_TIME: SymbolRef = SymbolRef("x-opt-enqueued-time");

/// The sequence number assigned to a message.
pub(crate) const SEQUENCE_NUMBER: SymbolRef = SymbolRef("x-opt-sequence-number");

/// The offset of a message within a given partition.
pub(crate) const OFFSET: SymbolRef = SymbolRef("x-opt-offset");

/// The partition hashing key used for grouping a batch of events together with the intent of routing to a single partition.
pub(crate) const PARTITION_KEY: SymbolRef = SymbolRef("x-opt-partition-key");

/// The message property that identifies the last sequence number enqueued for a partition.
pub(crate) const PARTITION_LAST_ENQUEUED_SEQUENCE_NUMBER: SymbolRef =
    SymbolRef("last_enqueued_sequence_number");

/// The message property that identifies the last offset enqueued for a partition.
pub(crate) const PARTITION_LAST_ENQUEUED_OFFSET: SymbolRef = SymbolRef("last_enqueued_offset");

/// The message property that identifies the last time enqueued for a partition.
pub(crate) const PARTITION_LAST_ENQUEUED_TIME_UTC: SymbolRef = SymbolRef("last_enqueued_time_utc");

/// The message property that identifies the time that the last enqueued event information was
/// received from the service.
pub(crate) const LAST_PARTITION_PROPERTIES_RETRIEVAL_TIME_UTC: SymbolRef =
    SymbolRef("runtime_info_retrieval_time_utc");

/// The set of descriptors for well-known <see cref="DescribedType" />
/// property types.
pub(crate) mod descriptor {
    use const_format::concatcp;
    use serde_amqp::primitives::SymbolRef;

    use crate::amqp::amqp_constants;

    /// The type annotation for representing a `TimeSpan` in a message.
    pub(crate) const TIME_SPAN: SymbolRef =
        SymbolRef(concatcp!(amqp_constants::VENDOR, ":timespan"));

    /// The type annotation for representing a `Url` in a message.
    pub(crate) const URI: SymbolRef = SymbolRef(concatcp!(amqp_constants::VENDOR, ":uri"));

    /// The type annotation for representing a `OffsetDateTime` in a message.
    pub(crate) const DATE_TIME_OFFSET: SymbolRef =
        SymbolRef(concatcp!(amqp_constants::VENDOR, ":datetime-offset"));
}

/// Represents the entity mapping for AMQP properties between the client library and
/// the Event Hubs service.
///
/// # WARNING:
///
/// These values are synchronized between the Event Hubs service and the client
/// library.  You must consult with the Event Hubs service team before making
/// changes, including adding a new member.
///
/// When adding a new member, remember to always do so before the Unknown
/// member.
pub(crate) enum Entity {
    Namespace = 4,
    EventHub = 7,
    ConsumerGroup = 8,
    Partition = 9,
    Checkpoint = 10,
    Unknown = 0x7FFFFFFE,
}

/// Represents the type mapping for AMQP properties between the client library and
/// the Event Hubs service.
///
/// # WARNING:
///
/// These values are synchronized between the Event Hubs service and the client
/// library.  You must consult with the Event Hubs service team before making
/// changes, including adding a new member.
///
/// When adding a new member, remember to always do so before the Unknown
/// member.
pub(crate) enum Type {
    Null,
    Byte,
    SByte,
    Char,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Single,
    Double,
    Decimal,
    Boolean,
    Guid,
    String,
    Uri,
    DateTime,
    DateTimeOffset,
    TimeSpan,
    Stream,
    Unknown,
}
