use std::borrow::Cow;

use fe2o3_amqp_types::messaging::annotations::AnnotationKey;
use fe2o3_amqp_types::messaging::{ApplicationProperties, Body, Data, Message, Properties};
use fe2o3_amqp_types::primitives::{OrderedMap, SimpleValue};
use serde_amqp::primitives::Binary;
use serde_amqp::Value;
use time::OffsetDateTime;

use crate::amqp::amqp_property;
use crate::amqp::error::RawAmqpMessageError;
use crate::amqp::{
    amqp_message_extension::{AmqpMessageExt, AmqpMessageMutExt},
    error::SetMessageIdError,
};
use crate::constants::DEFAULT_OFFSET_DATE_TIME;

// /// TODO: This is not being used because `send_idempotent` is not implemented yet.
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub(crate) enum PublishSequenceNumber {
//     Pending {
//         value: i32,
//         producer_group_id: i64,
//         owner_level: i16,
//     },
//     Published {
//         value: i32,
//     },
// }

/// An Event Hubs event, encapsulating a set of data and its associated metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event {
    pub(crate) amqp_message: Message<Data>,
    // /// TODO: This is not being used because `send_idempotent` is not implemented yet.
    // pub(crate) sequence_number: Option<PublishSequenceNumber>,
}

impl<T> From<T> for Event
where
    T: Into<Vec<u8>>,
{
    fn from(value: T) -> Self {
        Self {
            amqp_message: Message::builder().data(Binary::from(value)).build(),
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
}

#[derive(Debug, Clone)]
pub struct ReceivedEvent {
    raw_amqp_message: Message<Body<Value>>,
}

impl ReceivedEvent {
    pub(crate) fn from_raw_amqp_message(raw_amqp_message: Message<Body<Value>>) -> Self {
        Self { raw_amqp_message }
    }

    pub fn raw_amqp_message(&self) -> &Message<Body<Value>> {
        &self.raw_amqp_message
    }

    pub fn into_raw_amqp_message(self) -> Message<Body<Value>> {
        self.raw_amqp_message
    }

    /// Gets the body of the message.
    pub fn body(&self) -> Result<&[u8], RawAmqpMessageError> {
        match &self.raw_amqp_message.body {
            Body::Data(batch) => match batch.len() {
                1 => Ok(batch[0].0.as_ref()),
                _ => Err(RawAmqpMessageError {}),
            },
            _ => Err(RawAmqpMessageError {}),
        }
    }

    /// Gets the MessageId to identify the message.
    ///
    /// The message identifier is an application-defined value that uniquely identifies the message
    /// and its payload. The identifier is a free-form string and can reflect a GUID or an
    /// identifier derived from the application context. If enabled, the [duplicate
    /// detection](https://docs.microsoft.com/azure/service-bus-messaging/duplicate-detection)
    /// feature identifies and removes second and further submissions of messages with the same
    /// MessageId.
    pub fn message_id(&self) -> Option<Cow<'_, str>> {
        self.raw_amqp_message.message_id()
    }

    /// Gets the correlation identifier.
    ///
    /// Allows an application to specify a context for the message for the purposes of correlation,
    /// for example reflecting the MessageId of a message that is being replied to. See [Message
    /// Routing and
    /// Correlation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation").
    pub fn correlation_id(&self) -> Option<Cow<'_, str>> {
        self.raw_amqp_message.correlation_id()
    }

    pub fn properties(&self) -> Option<&ApplicationProperties> {
        self.raw_amqp_message.application_properties.as_ref()
    }

    pub fn system_properties(&self) -> Option<&Properties> {
        self.raw_amqp_message.properties.as_ref()
    }

    pub fn sequence_number(&self) -> i64 {
        self.raw_amqp_message
            .message_annotations
            .as_ref()
            .and_then(|m| m.get(&amqp_property::SEQUENCE_NUMBER as &dyn AnnotationKey))
            .map(|value| match value {
                Value::Long(val) => *val,
                _ => unreachable!("Expecting a Long"),
            })
            .unwrap_or_default()
    }

    pub fn offset(&self) -> Option<i64> {
        self.raw_amqp_message
            .message_annotations
            .as_ref()
            .and_then(|m| m.get(&amqp_property::OFFSET as &dyn AnnotationKey))
            .and_then(|value| match value {
                Value::Long(val) => Some(*val),
                Value::String(val) => val.parse().ok(),
                _ => unreachable!("Expecting a Long"),
            })
    }

    pub fn enqueued_time(&self) -> OffsetDateTime {
        self.raw_amqp_message
            .message_annotations
            .as_ref()
            .and_then(|m| m.get(&amqp_property::ENQUEUED_TIME as &dyn AnnotationKey))
            .map(|value| match value {
                Value::Timestamp(val) => OffsetDateTime::from(val.clone()),
                _ => unreachable!("Expecting a Timestamp"),
            })
            .unwrap_or(DEFAULT_OFFSET_DATE_TIME)
    }

    pub fn partition_key(&self) -> Option<&str> {
        self.raw_amqp_message.partition_key()
    }
}
