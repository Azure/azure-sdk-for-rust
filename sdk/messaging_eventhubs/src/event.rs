use std::borrow::Cow;

use fe2o3_amqp_types::messaging::{Data, Message};
use fe2o3_amqp_types::primitives::{OrderedMap, SimpleValue};
use serde_amqp::primitives::Binary;

use crate::amqp::{
    amqp_message_extension::{AmqpMessageExt, AmqpMessageMutExt},
    error::SetMessageIdError,
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
}
