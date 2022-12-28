//! Implements ServiceBusMessage

use std::borrow::Cow;

use std::time::Duration as StdDuration;

use fe2o3_amqp_types::{
    messaging::{
        annotations::OwnedKey, ApplicationProperties, Body, Data, Message, MessageAnnotations,
    },
    primitives::Binary,
};
use time::OffsetDateTime;

use crate::amqp::{
    amqp_message_constants::{
        self, DEAD_LETTER_ERROR_DESCRIPTION_HEADER, DEAD_LETTER_REASON_HEADER,
        DEAD_LETTER_SOURCE_NAME, ENQUEUED_TIME_UTC_NAME, ENQUEUE_SEQUENCE_NUMBER_NAME,
        LOCKED_UNTIL_NAME, MESSAGE_STATE_NAME, SEQUENCE_NUMBER_NAME,
    },
    amqp_message_extensions::{AmqpMessageExt, AmqpMessageMutExt},
    error::{
        MaxAllowedTtlExceededError, MaxLengthExceededError, SetMessageIdError, SetPartitionKeyError,
    },
};

use super::service_bus_received_message::ServiceBusReceivedMessage;

/// The [ServiceBusMessage] is used to send data to Service Bus Queues and Topics. When receiving
/// messages, the [`ServiceBusReceivedMessage`] is used.
///
/// The message structure is discussed in detail in the [product
/// documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads)
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct ServiceBusMessage {
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
        match &received.raw_amqp_message.body {
            Body::Data(batch) => match batch.len() {
                1 => {}
                // There should be only one data section
                _ => return Err(received),
            },
            _ => return Err(received),
        }

        let src = received.raw_amqp_message;

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
                    OwnedKey::Symbol(ksymbol) => !matches!(
                        ksymbol.as_str(),
                        LOCKED_UNTIL_NAME
                            | SEQUENCE_NUMBER_NAME
                            | DEAD_LETTER_SOURCE_NAME
                            | ENQUEUE_SEQUENCE_NUMBER_NAME
                            | ENQUEUED_TIME_UTC_NAME
                            | MESSAGE_STATE_NAME
                    ),
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
                    let map = map
                        .into_iter()
                        .filter(|(k, _)| {
                            !matches!(
                                k.as_str(),
                                DEAD_LETTER_REASON_HEADER | DEAD_LETTER_ERROR_DESCRIPTION_HEADER
                            )
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
    /// Creates a new [`ServiceBusMessage`] with a raw AMQP message.
    pub fn from_raw_amqp_message(amqp_message: Message<Data>) -> Self {
        Self { amqp_message }
    }

    /// Gets the raw AMQP message
    pub fn raw_amqp_message(&self) -> &Message<Data> {
        &self.amqp_message
    }

    /// Creates a new [`ServiceBusMessage`] with the given data as the body.
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self::from(data)
    }

    /// Gets the body of the message
    pub fn body(&self) -> &[u8] {
        &self.amqp_message.body.0
    }

    /// Gets a mutable reference to the body of the message
    pub fn body_mut(&mut self) -> &mut Vec<u8> {
        &mut self.amqp_message.body.0
    }

    /// Sets the body of the message
    pub fn set_body(&mut self, body: impl Into<Vec<u8>>) {
        self.amqp_message.body = Data(Binary::from(body));
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
        self.amqp_message.message_id()
    }

    /// Sets the MessageId
    pub fn set_message_id(
        &mut self,
        message_id: impl Into<String>,
    ) -> Result<(), SetMessageIdError> {
        self.amqp_message.set_message_id(message_id)
    }

    /// Gets the partition key for sending a message to a partitioned entity.
    ///
    /// For [partitioned
    /// entities](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-partitioning),
    /// setting this value enables assigning related messages to the same internal partition, so
    /// that submission sequence order is correctly recorded. The partition is chosen by a hash
    /// function over this value and cannot be chosen directly. For session-aware entities, the
    /// [`ServiceBusMessage::set_session_id`] method overrides this value.
    pub fn partition_key(&self) -> Option<&str> {
        self.amqp_message.partition_key()
    }

    /// Sets a partition key for sending a message to a partitioned entity. Maximum length is 128
    /// characters.
    pub fn set_partition_key(
        &mut self,
        key: impl Into<Option<String>>,
    ) -> Result<(), SetPartitionKeyError> {
        self.amqp_message.set_partition_key(key)
    }

    /// Gets a partition key for sending a message into an entity via a partitioned transfer queue.
    ///
    /// If a message is sent via a transfer queue in the scope of a transaction, this value selects
    /// the transfer queue partition: This is functionally equivalent to [`Self::partition_key`]
    /// and ensures that messages are kept together and in order as they are transferred. See
    /// [Transfers and Send
    /// Via](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-transactions#transfers-and-send-via).
    #[cfg(feature = "transaction")]
    pub fn transaction_partition_key(&self) -> Option<&str> {
        self.amqp_message.via_partition_key()
    }

    /// Sets a partition key for sending a message into an entity via a partitioned transfer.
    /// Maximum length is 128 characters.
    #[cfg(feature = "transaction")]
    pub fn set_transaction_partition_key(
        &mut self,
        key: impl Into<Option<String>>,
    ) -> Result<(), MaxLengthExceededError> {
        self.amqp_message.set_via_partition_key(key)
    }

    /// Gets the session identifier for a session-aware entity.
    ///
    /// For session-aware entities, this application-defined value specifies the session affiliation
    /// of the message. Messages with the same session identifier are subject to summary locking and
    /// enable exact in-order processing and demultiplexing. For session-unaware entities, this
    /// value is ignored. See [Message
    /// Sessions](https://docs.microsoft.com/azure/service-bus-messaging/message-sessions).
    pub fn session_id(&self) -> Option<&str> {
        self.amqp_message.session_id()
    }

    /// Sets the session identifier for a session-aware entity. Maximum length is 128 characters.
    pub fn set_session_id(
        &mut self,
        session_id: impl Into<Option<String>>,
    ) -> Result<(), MaxLengthExceededError> {
        self.amqp_message.set_session_id(session_id)
    }

    /// Gets session identifier augmenting the [`ServiceBusMessage::reply_to`] address.
    ///
    /// This value augments the ReplyTo information and specifies which SessionId should be set for
    /// the reply when sent to the reply entity. See [Message Routing and
    /// Correlation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation)
    pub fn reply_to_session_id(&self) -> Option<&str> {
        self.amqp_message.reply_to_session_id()
    }

    /// Sets a session identifier augmenting the [`ServiceBusMessage::set_reply_to`] address.
    /// Maximum length is 128 characters.
    pub fn set_reply_to_session_id(
        &mut self,
        session_id: impl Into<Option<String>>,
    ) -> Result<(), MaxLengthExceededError> {
        self.amqp_message.set_reply_to_session_id(session_id)
    }

    /// Gets the message’s "time to live" value.
    ///
    /// This value is the relative duration after which the message expires. When not set
    /// explicitly, the assumed value is the DefaultTimeToLive for the respective queue or topic. A
    /// message-level [time_to_live](#method.time_to_live) value cannot be longer than the entity's
    /// DefaultTimeToLive setting and it is silently adjusted if it does. See
    /// [Expiration](https://docs.microsoft.com/azure/service-bus-messaging/message-expiration).
    pub fn time_to_live(&self) -> Option<StdDuration> {
        self.amqp_message.time_to_live()
    }

    /// Sets the message’s "time to live" value.
    pub fn set_time_to_live(
        &mut self,
        ttl: impl Into<Option<StdDuration>>,
    ) -> Result<(), MaxAllowedTtlExceededError> {
        self.amqp_message.set_time_to_live(ttl.into())
    }

    /// Gets the correlation identifier.
    ///
    /// Allows an application to specify a context for the message for the purposes of correlation,
    /// for example reflecting the MessageId of a message that is being replied to. See [Message
    /// Routing and
    /// Correlation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation).
    pub fn correlation_id(&self) -> Option<Cow<'_, str>> {
        self.amqp_message.correlation_id()
    }

    /// Sets the correlation identifier.
    pub fn set_correlation_id(&mut self, id: impl Into<Option<String>>) {
        self.amqp_message.set_correlation_id(id)
    }

    /// Gets application specific subject.
    ///
    /// This property enables the application to indicate the purpose of the message to the receiver
    /// in a standardized fashion, similar to an email subject line. The mapped AMQP property is
    /// "subject".
    pub fn subject(&self) -> Option<&str> {
        self.amqp_message.subject()
    }

    /// Sets an application specific subject.
    pub fn set_subject(&mut self, subject: impl Into<Option<String>>) {
        self.amqp_message.set_subject(subject)
    }

    /// Gets the "to" address.
    ///
    /// This property is reserved for future use in routing scenarios and presently ignored by the
    /// broker itself. Applications can use this value in rule-driven [auto-forward
    /// chaining](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-auto-forwarding)
    /// scenarios to indicate the intended logical destination of the message.
    pub fn to(&self) -> Option<&str> {
        self.amqp_message.to()
    }

    /// Sets the "to" address.
    pub fn set_to(&mut self, to: impl Into<Option<String>>) {
        self.amqp_message.set_to(to)
    }

    /// Gets the content type descriptor.
    ///
    /// Optionally describes the payload of the message, with a descriptor following the format of
    /// RFC2045, Section 5, for example "application/json".
    pub fn content_type(&self) -> Option<&str> {
        self.amqp_message.content_type()
    }

    /// Sets the content type descriptor.
    pub fn set_content_type(&mut self, content_type: impl Into<Option<String>>) {
        self.amqp_message.set_content_type(content_type)
    }

    /// Gets the address of an entity to send replies to.
    ///
    /// This optional and application-defined value is a standard way to express a reply path to the
    /// receiver of the message. When a sender expects a reply, it sets the value to the absolute or
    /// relative path of the queue or topic it expects the reply to be sent to. See [Message Routing
    /// and
    /// Correlation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation).
    pub fn reply_to(&self) -> Option<&str> {
        self.amqp_message.reply_to()
    }

    /// Sets the address of an entity to send replies to.
    pub fn set_reply_to(&mut self, reply_to: impl Into<Option<String>>) {
        self.amqp_message.set_reply_to(reply_to)
    }

    /// Gets the date and time in UTC at which the message will be enqueued. This property returns
    /// the time in UTC; when setting the property, the supplied `OffsetDateTime` value must also be
    /// in UTC. It is utilized to delay messages sending to a specific time in the future.
    ///
    /// Message enqueuing time does not mean that the message will be sent at the same time. It will
    /// get enqueued, but the actual sending time depends on the queue's workload and its state.
    pub fn scheduled_enqueue_time(&self) -> OffsetDateTime {
        self.amqp_message.scheduled_enqueue_time()
    }

    /// Sets the date and time in UTC at which the message will be enqueued
    pub fn set_scheduled_enqueue_time(&mut self, enqueue_time: OffsetDateTime) {
        let message_annotations = self
            .amqp_message
            .message_annotations
            .get_or_insert_with(MessageAnnotations::default);
        let timestamp = fe2o3_amqp::types::primitives::Timestamp::from(enqueue_time);
        message_annotations.insert(
            amqp_message_constants::SCHEDULED_ENQUEUE_TIME_UTC_NAME.into(),
            timestamp.into(),
        );
    }

    /// Gets the application properties bag, which can be used for custom message metadata.
    pub fn application_properties(&self) -> Option<&ApplicationProperties> {
        self.amqp_message.application_properties.as_ref()
    }
}

impl std::fmt::Display for ServiceBusMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.message_id() {
            Some(id) => write!(f, "{{MessageId:{}}}", id),
            None => write!(f, "{{MessageId:None}}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::{MAX_MESSAGE_ID_LENGTH, MAX_SESSION_ID_LENGTH},
        ServiceBusMessage,
    };

    #[test]
    fn message_to_string() {
        let cases = ["123", "jøbber-nå"];

        for case in cases {
            let mut message = ServiceBusMessage::default();
            message.set_message_id(case).unwrap();
            assert_eq!(message.to_string(), format!("{{MessageId:{}}}", case));
        }
    }

    #[test]
    fn setting_empty_message_id_returns_error() {
        let mut message = ServiceBusMessage::new("test message");
        assert!(message.set_message_id("").is_err());
    }

    #[test]
    fn setting_long_message_id_returns_error() {
        let mut message = ServiceBusMessage::new("test message");
        let message_id = "a".repeat(MAX_MESSAGE_ID_LENGTH);
        assert!(message.set_message_id(message_id).is_ok());

        let message_id = "a".repeat(MAX_MESSAGE_ID_LENGTH + 1);
        assert!(message.set_message_id(message_id).is_err());
    }

    #[test]
    fn setting_none_session_id_returns_ok() {
        let mut message = ServiceBusMessage::new("test message");
        assert!(message.set_session_id(None).is_ok());
    }

    #[test]
    fn setting_long_session_id_returns_error() {
        let mut message = ServiceBusMessage::new("test message");
        let session_id = "a".repeat(MAX_SESSION_ID_LENGTH);
        assert!(message.set_session_id(Some(session_id)).is_ok());

        let session_id = "a".repeat(MAX_SESSION_ID_LENGTH + 1);
        assert!(message.set_session_id(Some(session_id)).is_err());
    }

    #[test]
    fn setting_none_reply_to_session_id_returns_ok() {
        let mut message = ServiceBusMessage::new("test message");
        assert!(message.set_reply_to_session_id(None).is_ok());
    }

    #[test]
    fn setting_long_reply_to_session_id_returns_error() {
        let mut message = ServiceBusMessage::new("test message");
        let reply_to_session_id = "a".repeat(MAX_SESSION_ID_LENGTH);
        assert!(message
            .set_reply_to_session_id(Some(reply_to_session_id))
            .is_ok());

        let reply_to_session_id = "a".repeat(MAX_SESSION_ID_LENGTH + 1);
        assert!(message
            .set_reply_to_session_id(Some(reply_to_session_id))
            .is_err());
    }

    #[test]
    fn setting_none_partition_key_returns_ok() {
        let mut message = ServiceBusMessage::new("test message");
        assert!(message.set_partition_key(None).is_ok());
    }

    #[test]
    fn partition_key_must_match_session_id_if_both_are_set() {
        let mut message = ServiceBusMessage::new("test message");
        assert!(message.set_session_id(Some("session_id".into())).is_ok());
        assert_eq!(message.session_id(), Some("session_id"));
        assert!(message.partition_key().is_none());
        assert!(message
            .set_partition_key(Some("partition_key".into()))
            .is_err());

        let mut message = ServiceBusMessage::new("test message");
        assert!(message
            .set_partition_key(Some("partition_key".into()))
            .is_ok());
        assert_eq!(message.partition_key(), Some("partition_key"));
        assert!(message.session_id().is_none());
        assert!(message.set_session_id(Some("session_id".into())).is_ok());
        assert_eq!(message.partition_key(), message.session_id());
    }

    #[test]
    fn set_message_body_to_string() {
        let message_body = "some message";
        let mut message = ServiceBusMessage::new(message_body);
        assert_eq!(message.body(), message_body.as_bytes());

        let new_message_body = "some new message";
        message.set_body(new_message_body);
        assert_eq!(message.body(), new_message_body.as_bytes());
    }
}
