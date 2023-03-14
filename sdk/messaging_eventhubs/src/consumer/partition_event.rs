use std::borrow::Cow;

use fe2o3_amqp_types::{messaging::{Message, Body}, primitives::{OrderedMap, SimpleValue}};
use serde_amqp::Value;
use time::OffsetDateTime;

use crate::{amqp::{error::RawAmqpMessageError, amqp_message_extension::AmqpMessageExt}, AmqpSystemProperties, constants::DEFAULT_OFFSET_DATE_TIME};

use super::PartitionContext;

/// Contains information about a partition that has attempted to receive an event from the Azure
/// Event Hub service, as well as the received event, if any.
#[derive(Debug)]
pub struct PartitionEvent {
    /// The Event Hub partition that the <see cref="PartitionEvent.Data" /> is associated with.
    pub(crate) partition_context: PartitionContext,

    /// An event that was read from the associated <see cref="PartitionEvent.Partition" />.
    pub(crate) raw_amqp_message: Message<Body<Value>>,
}

impl PartitionEvent {
    /// Initializes a new instance of the <see cref="PartitionEvent"/> structure.
    pub(crate) fn new(partition_context: PartitionContext, raw_amqp_message: Message<Body<Value>>) -> Self {
        Self {
            partition_context,
            raw_amqp_message,
        }
    }

    /// The Event Hub partition that the <see cref="PartitionEvent.Data" /> is associated with.
    pub fn partition_context(&self) -> &PartitionContext {
        &self.partition_context
    }


    /// The data associated with the event
    pub fn body(&self) -> Result<&[u8], RawAmqpMessageError> {
        match &self.raw_amqp_message.body {
            Body::Data(batch) => match batch.len() {
                1 => Ok(batch[0].0.as_ref()),
                _ => Err(RawAmqpMessageError {}),
            },
            _ => Err(RawAmqpMessageError {}),
        }
    }

    /// The data associated with the event
    ///
    /// This is an alias for `body()`
    pub fn data(&self) -> Result<&[u8], RawAmqpMessageError> {
        match &self.raw_amqp_message.body {
            Body::Data(batch) => match batch.len() {
                1 => Ok(batch[0].0.as_ref()),
                _ => Err(RawAmqpMessageError {}),
            },
            _ => Err(RawAmqpMessageError {}),
        }
    }

    /// The content type associated with the event
    pub fn content_type(&self) -> Option<&str> {
        self.raw_amqp_message.content_type()
    }

    /// An application-defined value that uniquely identifies the event.  The identifier is
    /// a free-form value and can reflect a GUID or an identifier derived from the application
    /// context.
    pub fn message_id(&self) -> Option<Cow<'_, str>> {
        self.raw_amqp_message.message_id()
    }

    /// An application-defined value that represents the context to use for correlation across
    /// one or more operations.  The identifier is a free-form value and may reflect a unique
    /// identity or a shared data element with significance to the application.
    pub fn correlation_id(&self) -> Option<Cow<'_, str>> {
        self.raw_amqp_message.correlation_id()
    }

    /// The set of free-form properties which may be used for associating metadata with the event that
    /// is meaningful within the application context.
    pub fn properties(&self) -> Option<&OrderedMap<String, SimpleValue>> {
        self.raw_amqp_message
            .application_properties
            .as_ref()
            .map(|p| &p.0)
    }

    /// The set of free-form event properties which were provided by the Event Hubs service to pass metadata associated with the
    /// event or associated Event Hubs operation.
    pub fn system_properties(&self) -> AmqpSystemProperties<'_> {
        AmqpSystemProperties::from(&self.raw_amqp_message)
    }

    /// The sequence number assigned to the event when it was enqueued in the associated Event Hub partition.
    ///
    /// This value is read-only and will only be populated for events that have been read from Event Hubs. The default value
    /// when not populated is [`i64::MIN`].
    pub fn sequence_number(&self) -> i64 {
        self.raw_amqp_message.sequence_number().unwrap_or(i64::MIN)
    }

    /// The offset of the event when it was received from the associated Event Hub partition.
    ///
    /// This value is read-only and will only be populated for events that have been read from Event Hubs. The default value
    /// when not populated is [`i64::MIN`].
    pub fn offset(&self) -> i64 {
        self.raw_amqp_message.offset().unwrap_or(i64::MIN)
    }

    /// The date and time, in UTC, of when the event was enqueued in the Event Hub partition.
    ///
    /// This value is read-only and will only be populated for events that have been read from Event
    /// Hubs. The default value when not populated is [`DEFAULT_OFFSET_DATE_TIME`].
    pub fn enqueued_time(&self) -> OffsetDateTime {
        self.raw_amqp_message
            .enqueued_time()
            .unwrap_or(DEFAULT_OFFSET_DATE_TIME)
    }

    /// The partition hashing key applied to the batch that the associated [`Event`], was published with.
    ///
    /// This value is read-only and will only be populated for events that have been read from Event Hubs.
    pub fn partition_key(&self) -> Option<&str> {
        self.raw_amqp_message.partition_key()
    }

    pub(crate) fn last_partition_sequence_number(&self) -> Option<i64> {
        self.raw_amqp_message.last_partition_sequence_number()
    }

    pub(crate) fn last_partition_offset(&self) -> Option<i64> {
        self.raw_amqp_message.last_partition_offset()
    }

    pub(crate) fn last_partition_enqueued_time(&self) -> Option<OffsetDateTime> {
        self.raw_amqp_message.last_partition_enqueued_time()
    }

    pub(crate) fn last_partition_properties_retrieval_time(&self) -> Option<OffsetDateTime> {
        self.raw_amqp_message.last_partition_properties_retrieval_time()
    }
}
