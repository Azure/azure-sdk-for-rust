use std::borrow::Cow;

use fe2o3_amqp_types::messaging::{Data, Message};
use fe2o3_amqp_types::primitives::{OrderedMap, SimpleValue};
use serde_amqp::primitives::Binary;
use time::OffsetDateTime;

use crate::{
    amqp::{
        amqp_message_extension::{AmqpMessageExt, AmqpMessageMutExt},
        amqp_system_properties::AmqpSystemProperties,
        error::SetMessageIdError,
    },
    constants::DEFAULT_OFFSET_DATE_TIME,
};

/// An Event Hubs event, encapsulating a set of data and its associated metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event {
    pub(crate) amqp_message: Message<Data>,
    pub(crate) published_sequence_number: Option<i32>,
    pub(crate) pending_publish_sequence_number: Option<i32>,
    pub(crate) pending_producer_group_id: Option<i64>,
    pub(crate) pending_producer_owner_level: Option<i16>,
}

impl<T> From<T> for Event
where
    T: Into<Vec<u8>>,
{
    fn from(value: T) -> Self {
        Self {
            amqp_message: Message::builder().data(Binary::from(value)).build(),
            published_sequence_number: None,
            pending_publish_sequence_number: None,
            pending_producer_group_id: None,
            pending_producer_owner_level: None,
        }
    }
}

impl Event {
    /// Creates a new event from the given data
    pub fn new(body: impl Into<Vec<u8>>) -> Self {
        Self::from(body)
    }

    /// The data associated with the event
    pub fn body(&self) -> &[u8] {
        &self.amqp_message.body.0
    }

    /// Sets the body associated with the event
    pub fn set_body(&mut self, body: impl Into<Vec<u8>>) {
        self.amqp_message.body = Data(Binary::from(body));
    }

    /// The content type associated with the event
    pub fn content_type(&self) -> Option<&str> {
        self.amqp_message.content_type()
    }

    /// Sets the content type associated with the event
    pub fn set_content_type(&mut self, content_type: impl Into<Option<String>>) {
        self.amqp_message.set_content_type(content_type)
    }

    /// An application-defined value that uniquely identifies the event.  The identifier is
    /// a free-form value and can reflect a GUID or an identifier derived from the application
    /// context.
    pub fn message_id(&self) -> Option<Cow<'_, str>> {
        self.amqp_message.message_id()
    }

    /// Sets the message ID associated with the event
    pub fn set_message_id(
        &mut self,
        message_id: impl Into<String>,
    ) -> Result<(), SetMessageIdError> {
        self.amqp_message.set_message_id(message_id)
    }

    /// An application-defined value that represents the context to use for correlation across
    /// one or more operations.  The identifier is a free-form value and may reflect a unique
    /// identity or a shared data element with significance to the application.
    pub fn correlation_id(&self) -> Option<Cow<'_, str>> {
        self.amqp_message.correlation_id()
    }

    /// Sets the correlation ID associated with the event
    pub fn set_correlation_id(&mut self, correlation_id: impl Into<Option<String>>) {
        self.amqp_message.set_correlation_id(correlation_id)
    }

    /// The set of free-form properties which may be used for associating metadata with the event that
    /// is meaningful within the application context.
    pub fn properties(&self) -> Option<&OrderedMap<String, SimpleValue>> {
        self.amqp_message
            .application_properties
            .as_ref()
            .map(|p| &p.0)
    }

    /// The set of free-form event properties which were provided by the Event Hubs service to pass metadata associated with the
    /// event or associated Event Hubs operation.
    pub fn system_properties(&self) -> AmqpSystemProperties<'_> {
        AmqpSystemProperties::from(&self.amqp_message)
    }

    /// The sequence number assigned to the event when it was enqueued in the associated Event Hub partition.
    ///
    /// This value is read-only and will only be populated for events that have been read from Event Hubs. The default value
    /// when not populated is [`i64::MIN`].
    pub fn sequence_number(&self) -> i64 {
        self.amqp_message.sequence_number().unwrap_or(i64::MIN)
    }

    /// The offset of the event when it was received from the associated Event Hub partition.
    ///
    /// This value is read-only and will only be populated for events that have been read from Event Hubs. The default value
    /// when not populated is [`i64::MIN`].
    pub fn offset(&self) -> i64 {
        self.amqp_message.offset().unwrap_or(i64::MIN)
    }

    /// The date and time, in UTC, of when the event was enqueued in the Event Hub partition.
    ///
    /// This value is read-only and will only be populated for events that have been read from Event
    /// Hubs. The default value when not populated is [`DEFAULT_OFFSET_DATE_TIME`].
    pub fn enqueued_time(&self) -> OffsetDateTime {
        self.amqp_message
            .enqueued_time()
            .unwrap_or(DEFAULT_OFFSET_DATE_TIME)
    }

    /// The partition hashing key applied to the batch that the associated [`Event`], was published with.
    ///
    /// This value is read-only and will only be populated for events that have been read from Event Hubs.
    pub fn partition_key(&self) -> Option<&str> {
        self.amqp_message.partition_key()
    }

    /// The sequence number of the event that was last enqueued into the Event Hub partition from
    /// which this event was received.
    ///
    /// This value is read-only and will only be populated for events that have been read from Event
    /// Hubs by a consumer specifying [`ReadEventOptions::track_last_enqueued_event_properties`]
    /// as enabled.  The default value when not populated is [`None`].
    pub(crate) fn last_partition_sequence_number(&self) -> Option<i64> {
        self.amqp_message.last_partition_sequence_number()
    }

    pub(crate) fn last_partition_offset(&self) -> Option<i64> {
        self.amqp_message.last_partition_offset()
    }

    pub(crate) fn last_partition_enqueued_time(&self) -> Option<OffsetDateTime> {
        self.amqp_message.last_partition_enqueued_time()
    }

    pub(crate) fn last_partition_properties_retrieval_time(&self) -> Option<OffsetDateTime> {
        self.amqp_message.last_partition_properties_retrieval_time()
    }
}
