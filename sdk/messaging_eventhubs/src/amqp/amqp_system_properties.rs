use std::borrow::Cow;

use fe2o3_amqp_types::messaging::{Properties, MessageAnnotations, Message};

/// The system properties of an AMQP message
#[derive(Debug)]
pub struct AmqpSystemProperties<'a> {
    pub(crate) amqp_properties: Option<Cow<'a, Properties>>,
    pub(crate) amqp_message_annotations: Option<Cow<'a, MessageAnnotations>>,
}

impl<'a, B> From<&'a Message<B>> for AmqpSystemProperties<'a> {
    fn from(message: &'a Message<B>) -> Self {
        Self {
            amqp_properties: message.properties.as_ref().map(Cow::Borrowed),
            amqp_message_annotations: message.message_annotations.as_ref().map(Cow::Borrowed),
        }
    }
}
