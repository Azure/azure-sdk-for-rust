use std::borrow::Cow;

use time::Duration as TimeSpan;

use fe2o3_amqp_types::{
    messaging::{
        annotations::OwnedKey, ApplicationProperties, Body, Data, Message, MessageAnnotations,
    },
    primitives::Binary,
};
use time::OffsetDateTime;

use crate::amqp::{
    amqp_message_constants::{
        DEAD_LETTER_ERROR_DESCRIPTION_HEADER, DEAD_LETTER_REASON_HEADER, DEAD_LETTER_SOURCE_NAME,
        ENQUEUED_TIME_UTC_NAME, ENQUEUE_SEQUENCE_NUMBER_NAME, LOCKED_UNTIL_NAME,
        MESSAGE_STATE_NAME, SEQUENCE_NUMBER_NAME,
    },
    amqp_message_extensions::{AmqpMessageExt, AmqpMessageMutExt},
    error::Error,
};

use super::service_bus_received_message::ServiceBusReceivedMessage;

/// The [ServiceBusMessage] is used to send data to Service Bus Queues and Topics. When receiving messages, the <see
/// cref="ServiceBusReceivedMessage"/> is used.
///
/// The message structure is discussed in detail in the [product
/// documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads)
pub struct ServiceBusMessage {
    // TODO: change to generics?
    pub(crate) amqp_message: Message<Data>,
}

impl Default for ServiceBusMessage {
    fn default() -> Self {
        Self {
            amqp_message: Message::builder()
                .data(Binary::from(Vec::with_capacity(0)))
                .build(),
        }
    }
}

impl<T> From<T> for ServiceBusMessage
where
    T: Into<Vec<u8>>,
{
    fn from(value: T) -> Self {
        Self {
            amqp_message: Message::builder().data(Binary::from(value)).build(),
        }
    }
}

impl TryFrom<ServiceBusReceivedMessage> for ServiceBusMessage {
    type Error = ServiceBusReceivedMessage;

    fn try_from(received: ServiceBusReceivedMessage) -> Result<Self, Self::Error> {
        use fe2o3_amqp_types::messaging::Header;

        // A raw AMQP message may be sent to a queue or topic.
        //
        // TODO: what about empty body?
        match &received.amqp_message.body {
            Body::Data(batch) => match batch.len() {
                1 => {}
                // There should be only one data section
                _ => return Err(received),
            },
            _ => return Err(received),
        }

        let src = received.amqp_message;

        // copy header except for delivery count which should be set to null
        let header = src.header.map(|h| Header {
            delivery_count: Default::default(), // null will give the default value
            ..h
        });

        // copy delivery annotations
        let delivery_annotations = src.delivery_annotations;

        // copy message annotations except for broker set ones
        let message_annotations = src.message_annotations.map(|MessageAnnotations(map)| {
            let map = map
                .into_iter()
                .filter(|(k, _)| match k {
                    OwnedKey::Symbol(kstr) => match kstr.as_str() {
                        LOCKED_UNTIL_NAME
                        | SEQUENCE_NUMBER_NAME
                        | DEAD_LETTER_SOURCE_NAME
                        | ENQUEUE_SEQUENCE_NUMBER_NAME
                        | ENQUEUED_TIME_UTC_NAME
                        | MESSAGE_STATE_NAME => false,
                        _ => true,
                    },
                    OwnedKey::ULong(_) => true,
                })
                .collect();
            MessageAnnotations(map)
        });

        // copy properties
        let properties = src.properties;

        // copy application properties except for broker set ones
        let application_properties =
            src.application_properties
                .map(|ApplicationProperties(map)| {
                    let map =
                        map.into_iter()
                            .filter(|(k, _)| match k.as_str() {
                                DEAD_LETTER_REASON_HEADER
                                | DEAD_LETTER_ERROR_DESCRIPTION_HEADER => false,
                                _ => true,
                            })
                            .collect();
                    ApplicationProperties(map)
                });

        let body = match src.body {
            Body::Data(batch) => match batch.into_inner().drain(..).next() {
                Some(data) => data,
                None => unreachable!(),
            },
            _ => unreachable!(),
        };

        // copy footer
        let footer = src.footer;

        let amqp_message = Message {
            header,
            delivery_annotations,
            message_annotations,
            properties,
            application_properties,
            body,
            footer,
        };

        Ok(Self { amqp_message })
    }
}

impl ServiceBusMessage {
    // /// Gets the raw AMQP message
    // pub fn raw_amqp_message(&self) -> &Message<Data> {
    //     &self.amqp_message
    // }

    /// Gets the body of the message
    pub fn body(&self) -> &[u8] {
        &self.amqp_message.body.0
    }

    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.amqp_message.body.0
    }

    /// Sets the body of the message
    pub fn set_body(&mut self, body: impl Into<Vec<u8>>) {
        self.amqp_message.body = Data(Binary::from(body));
    }

    /// Gets the MessageId to identify the message.
    ///
    /// The message identifier is an application-defined value that uniquely identifies the message and its payload. The
    /// identifier is a free-form string and can reflect a GUID or an identifier derived from the application context.
    /// If enabled, the [duplicate
    /// detection](https://docs.microsoft.com/azure/service-bus-messaging/duplicate-detection) feature identifies and
    /// removes second and further submissions of messages with the same MessageId.
    pub fn message_id(&self) -> Option<Cow<'_, str>> {
        self.amqp_message.message_id()
    }

    /// Sets the MessageId
    pub fn set_message_id(&mut self, message_id: impl Into<String>) {
        self.amqp_message.set_message_id(message_id)
    }

    /// Gets or sets a partition key for sending a message to a partitioned entity.
    ///
    /// # Value
    ///
    /// The partition key. Maximum length is 128 characters.
    ///
    /// # Remarks
    ///
    /// For [partitioned
    /// entities](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-partitioning),
    /// setting this value enables assigning related messages to the same internal partition, so
    /// that submission sequence order is correctly recorded. The partition is chosen by a hash
    /// function over this value and cannot be chosen directly. For session-aware entities, the <see
    /// cref="SessionId"/> property overrides this value.
    pub fn partition_key(&self) -> Option<&str> {
        self.amqp_message.partition_key()
    }

    pub fn set_partition_key(&mut self, key: impl Into<String>) -> Result<(), Error> {
        self.amqp_message.set_partition_key(key)
    }

    /// Gets or sets a partition key for sending a message into an entity via a partitioned transfer
    /// queue.
    ///
    /// # Value
    ///
    /// The partition key. Maximum length is 128 characters.
    ///
    /// # Remarks
    ///
    /// If a message is sent via a transfer queue in the scope of a transaction, this value selects
    /// the transfer queue partition: This is functionally equivalent to <see cref="PartitionKey"/>
    /// and ensures that messages are kept together and in order as they are transferred. See
    /// [Transfers and Send
    /// Via](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-transactions#transfers-and-send-via).
    pub fn transaction_partition_key(&self) -> Option<&str> {
        self.amqp_message.via_partition_key()
    }

    pub fn set_transaction_partition_key(&mut self, key: impl Into<String>) {
        self.amqp_message.set_via_partition_key(key)
    }

    /// Gets or sets the session identifier for a session-aware entity.
    ///
    /// # Value
    ///
    /// The session identifier. Maximum length is 128 characters.
    ///
    /// # Remarks
    ///
    /// For session-aware entities, this application-defined value specifies the session affiliation
    /// of the message. Messages with the same session identifier are subject to summary locking and
    /// enable exact in-order processing and demultiplexing. For session-unaware entities, this
    /// value is ignored. See [Message
    /// Sessions](https://docs.microsoft.com/azure/service-bus-messaging/message-sessions).
    pub fn session_id(&self) -> Option<&str> {
        self.amqp_message.session_id()
    }

    pub fn set_session_id(&mut self, session_id: impl Into<String>) {
        self.amqp_message.set_session_id(session_id)
    }

    /// Gets or sets a session identifier augmenting the <see cref="ReplyTo"/> address.
    ///
    /// # Value
    ///
    /// Session identifier. Maximum length is 128 characters.
    ///
    /// # Remarks
    ///
    /// This value augments the ReplyTo information and specifies which SessionId should be set for
    /// the reply when sent to the reply entity. See [Message Routing and
    /// Correlation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation)
    pub fn reply_to_session_id(&self) -> Option<&str> {
        self.amqp_message.reply_to_session_id()
    }

    pub fn set_reply_to_session_id(&mut self, session_id: Option<impl Into<String>>) {
        self.amqp_message.set_reply_to_session_id(session_id)
    }

    /// Gets or sets the message’s "time to live" value.
    ///
    /// # Value
    ///
    /// The message’s time to live value.
    ///
    /// # Remarks
    ///
    /// This value is the relative duration after which the message expires. When not set
    /// explicitly, the assumed value is the DefaultTimeToLive for the respective queue or topic. A
    /// message-level [time_to_live](#method.time_to_live) value cannot be longer than the entity's
    /// DefaultTimeToLive setting and it is silently adjusted if it does. See
    /// [Expiration](https://docs.microsoft.com/azure/service-bus-messaging/message-expiration).
    pub fn time_to_live(&self) -> Option<TimeSpan> {
        self.amqp_message.time_to_live()
    }

    pub fn set_time_to_live(&mut self, ttl: Option<TimeSpan>) {
        self.amqp_message.set_time_to_live(ttl)
    }

    /// Gets or sets the a correlation identifier.
    ///
    /// # Value
    ///
    /// Correlation identifier.
    ///
    /// # Remarks
    ///
    /// Allows an application to specify a context for the message for the purposes of correlation,
    /// for example reflecting the MessageId of a message that is being replied to. See [Message
    /// Routing and
    /// Correlation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation).
    pub fn correlation_id(&self) -> Option<Cow<'_, str>> {
        self.amqp_message.correlation_id()
    }

    pub fn set_correlation_id(&mut self, id: Option<impl Into<String>>) {
        self.amqp_message.set_correlation_id(id)
    }

    /// Gets or sets an application specific subject.
    ///
    /// # Value
    ///
    /// The application specific subject.
    ///
    /// # Remarks
    ///
    /// This property enables the application to indicate the purpose of the message to the receiver
    /// in a standardized fashion, similar to an email subject line. The mapped AMQP property is
    /// "subject".
    pub fn subject(&self) -> Option<&str> {
        self.amqp_message.subject()
    }

    pub fn set_subject(&mut self, subject: Option<impl Into<String>>) {
        self.amqp_message.set_subject(subject)
    }

    /// Gets or sets the "to" address.
    ///
    /// # Value
    ///
    /// The "to" address.
    ///
    /// # Remarks
    ///
    /// This property is reserved for future use in routing scenarios and presently ignored by the
    /// broker itself. Applications can use this value in rule-driven [auto-forward
    /// chaining](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-auto-forwarding)
    /// scenarios to indicate the intended logical destination of the message.
    pub fn to(&self) -> Option<&str> {
        self.amqp_message.to()
    }

    pub fn set_to(&mut self, to: Option<impl Into<String>>) {
        self.amqp_message.set_to(to)
    }

    /// Gets or sets the content type descriptor.
    ///
    /// # Value
    ///
    /// RFC2045 Content-Type descriptor.
    ///
    /// # Remarks
    ///
    /// Optionally describes the payload of the message, with a descriptor following the format of
    /// RFC2045, Section 5, for example "application/json".
    pub fn content_type(&self) -> Option<&str> {
        self.amqp_message.content_type()
    }

    pub fn set_content_type(&mut self, content_type: Option<impl Into<String>>) {
        self.amqp_message.set_content_type(content_type)
    }

    /// Gets or sets the address of an entity to send replies to.
    ///
    /// # Value
    ///
    /// The reply entity address.
    ///
    /// # Remarks
    ///
    /// This optional and application-defined value is a standard way to express a reply path to the
    /// receiver of the message. When a sender expects a reply, it sets the value to the absolute or
    /// relative path of the queue or topic it expects the reply to be sent to. See [Message Routing
    /// and
    /// Correlation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation).
    /// </remarks>
    pub fn reply_to(&self) -> Option<&str> {
        self.amqp_message.reply_to()
    }

    pub fn set_reply_to(&mut self, reply_to: Option<impl Into<String>>) {
        self.amqp_message.set_reply_to(reply_to)
    }

    /// Gets or sets the date and time in UTC at which the message will be enqueued. This property
    /// returns the time in UTC; when setting the property, the supplied DateTime value must also be
    /// in UTC.
    ///
    /// # Value
    ///
    /// The scheduled enqueue time in UTC. This value is for delayed message sending. It is utilized
    /// to delay messages sending to a specific time in the future.
    ///
    /// # Remarks
    ///
    /// Message enqueuing time does not mean that the message will be sent at the same time. It will
    /// get enqueued, but the actual sending time depends on the queue's workload and its state.
    pub fn scheduled_enqueue_time(&self) -> OffsetDateTime {
        self.amqp_message.scheduled_enqueue_time()
    }

    /// Gets the application properties bag, which can be used for custom message metadata.
    pub fn application_properties(&self) -> Option<&ApplicationProperties> {
        self.amqp_message.application_properties.as_ref()
    }
}

impl ToString for ServiceBusMessage {
    fn to_string(&self) -> String {
        match self.message_id() {
            Some(id) => {
                let mut s = String::from(r#"{MessageId:"#);
                s.push_str(&id);
                s.push('}');
                s
            }
            None => String::from(r#"{MessageId:None"#),
        }
    }
}
