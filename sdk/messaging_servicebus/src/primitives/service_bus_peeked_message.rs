//! Defines the [`ServiceBusPeekedMessage`] struct.

use std::borrow::Cow;
use std::time::Duration as StdDuration;

use fe2o3_amqp_types::{
    messaging::{annotations::AnnotationKey, ApplicationProperties, Body, Message},
    primitives::SimpleValue,
};
use serde_amqp::Value;
use time::{Duration as TimeSpan, OffsetDateTime};

use crate::{
    amqp::{
        amqp_message_constants::{
            DEAD_LETTER_ERROR_DESCRIPTION_HEADER, DEAD_LETTER_REASON_HEADER,
            DEAD_LETTER_SOURCE_NAME, ENQUEUED_TIME_UTC_NAME, ENQUEUE_SEQUENCE_NUMBER_NAME,
            MESSAGE_STATE_NAME, SEQUENCE_NUMBER_NAME,
        },
        amqp_message_extensions::AmqpMessageExt,
        error::RawAmqpMessageError,
    },
    constants::{DEFAULT_OFFSET_DATE_TIME, MAX_OFFSET_DATE_TIME},
};

use super::service_bus_message_state::ServiceBusMessageState;

// Conditional import for docs.rs
#[cfg(docsrs)]
use crate::ServiceBusReceivedMessage;

/// A peeked message from a Service Bus queue or topic.
#[derive(Debug)]
pub struct ServiceBusPeekedMessage {
    pub(crate) raw_amqp_message: Message<Body<Value>>,
}

impl ServiceBusPeekedMessage {
    /// Gets the raw Amqp message data that was transmitted over the wire. This can be used to
    /// enable scenarios that require reading AMQP header, footer, property, or annotation data that
    /// is not exposed as top level properties in the [`ServiceBusReceivedMessage`].
    pub fn raw_amqp_message(&self) -> &Message<Body<Value>> {
        &self.raw_amqp_message
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

    /// Gets a partition key for sending a message to a partitioned entity.
    ///
    /// For [partitioned
    /// entities](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-partitioning),
    /// setting this value enables assigning related messages to the same internal partition, so
    /// that submission sequence order is correctly recorded. The partition is chosen by a hash
    /// function over this value and cannot be chosen directly. For session-aware entities, the
    /// [`session_id`](#method.session_id) property overrides this value.
    pub fn partition_key(&self) -> Option<&str> {
        self.raw_amqp_message.partition_key()
    }

    /// Gets a partition key for sending a message into an entity via a partitioned transfer queue.
    ///
    /// If a message is sent via a transfer queue in the scope of a transaction, this value selects
    /// the transfer queue partition: This is functionally equivalent to [`Self::partition_key`]
    /// and ensures that messages are kept together and in order as they are transferred. See
    /// [Transfers and Send
    /// Via](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-transactions#transfers-and-send-viaå).
    pub fn transaction_partition_key(&self) -> Option<&str> {
        self.raw_amqp_message.via_partition_key()
    }

    /// Gets the session identifier for a session-aware entity.
    ///
    /// For session-aware entities, this application-defined value specifies the session affiliation
    /// of the message. Messages with the same session identifier are subject to summary locking and
    /// enable exact in-order processing and demultiplexing. For session-unaware entities, this
    /// value is ignored. See [Message
    /// Sessions](https://docs.microsoft.com/azure/service-bus-messaging/message-sessions).
    pub fn session_id(&self) -> Option<&str> {
        self.raw_amqp_message.session_id()
    }

    /// Gets a session identifier augmenting the [`Self::reply_to`] address.
    ///
    /// This value augments the ReplyTo information and specifies which SessionId should be set for
    /// the reply when sent to the reply entity. See [Message Routing and
    /// Correlation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation)
    pub fn reply_to_session_id(&self) -> Option<&str> {
        self.raw_amqp_message.reply_to_session_id()
    }

    /// Gets the message’s "time to live" value.
    ///
    /// This value is the relative duration after which the message expires, starting from the
    /// instant the message has been accepted and stored by the broker. When not set explicitly, the assumed value is the DefaultTimeToLive
    /// for the respective queue or topic. A message-level time to live value cannot be
    /// longer than the entity's DefaultTimeToLive setting and it is silently adjusted if it does.
    /// See [Expiration](https://docs.microsoft.com/azure/service-bus-messaging/message-expiration)
    pub fn time_to_live(&self) -> Option<StdDuration> {
        self.raw_amqp_message.time_to_live()
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

    /// Gets an application specific label.
    ///
    /// This property enables the application to indicate the purpose of the message to the receiver
    /// in a standardized fashion, similar to an email subject line. The mapped AMQP property is
    /// "subject".
    pub fn subject(&self) -> Option<&str> {
        self.raw_amqp_message.subject()
    }

    /// Gets the "to" address.
    ///
    /// This property is reserved for future use in routing scenarios and presently ignored by the
    /// broker itself. Applications can use this value in rule-driven [auto-forward
    /// chaining](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-auto-forwarding)
    /// scenarios to indicate the intended logical destination of the message.
    pub fn to(&self) -> Option<&str> {
        self.raw_amqp_message.to()
    }

    /// Gets the content type descriptor.
    ///
    /// Optionally describes the payload of the message, with a descriptor following the format of
    /// RFC2045, Section 5, for example "application/json".
    pub fn content_type(&self) -> Option<&str> {
        self.raw_amqp_message.content_type()
    }

    /// Gets the address of an entity to send replies to.
    ///
    /// This optional and application-defined value is a standard way to express a reply path to the
    /// receiver of the message. When a sender expects a reply, it sets the value to the absolute or
    /// relative path of the queue or topic it expects the reply to be sent to. See [Message Routing
    /// and
    /// Correlation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation)
    pub fn reply_to(&self) -> Option<&str> {
        self.raw_amqp_message.reply_to()
    }

    /// Gets the date and time in UTC at which the message will be enqueued. This property returns
    /// the time in UTC; when setting the property, the supplied DateTime value must also be in UTC.
    ///
    /// Message enqueuing time does not mean that the message will be sent at the same time. It will
    /// get enqueued, but the actual sending time depends on the queue's workload and its state.
    pub fn scheduled_enqueue_time(&self) -> OffsetDateTime {
        self.raw_amqp_message.scheduled_enqueue_time()
    }

    /// Gets the application properties bag, which can be used for custom message metadata.
    ///
    /// Only the following value types are supported: byte, sbyte, char, short, ushort, int, uint,
    /// long, ulong, float, double, decimal, bool, Guid, string, Uri, DateTime, DateTimeOffset,
    /// TimeSpan
    pub fn application_properties(&self) -> Option<&ApplicationProperties> {
        self.raw_amqp_message.application_properties.as_ref()
    }

    /// Get the current delivery count. This value starts at 1. This number is off by one from the
    /// raw amqp message delivery count
    ///
    /// Number of deliveries that have been attempted for this message. The count is incremented
    /// when a message lock expires, or the message is explicitly abandoned by the receiver. This
    /// property is read-only.
    pub fn delivery_count(&self) -> Option<u32> {
        self.raw_amqp_message
            .header
            .as_ref()
            .map(|h| h.delivery_count + 1)
    }

    /// Gets the unique number assigned to a message by Service Bus.
    ///
    /// The sequence number is a unique 64-bit integer assigned to a message as it is accepted and
    /// stored by the broker and functions as its true identifier. For partitioned entities, the
    /// topmost 16 bits reflect the partition identifier. Sequence numbers monotonically increase.
    /// They roll over to 0 when the 48-64 bit range is exhausted. This property is read-only.
    pub fn sequence_number(&self) -> i64 {
        self.raw_amqp_message
            .message_annotations
            .as_ref()
            .and_then(|m| m.get(&SEQUENCE_NUMBER_NAME as &dyn AnnotationKey))
            .map(|value| match value {
                Value::Long(val) => *val,
                _ => unreachable!("Expecting a Long"),
            })
            .unwrap_or_default()
    }

    /// Gets the name of the queue or subscription that this message was enqueued on, before it was
    /// dead-lettered.
    ///
    /// Only set in messages that have been dead-lettered and subsequently auto-forwarded from the
    /// dead-letter queue to another entity. Indicates the entity in which the message was
    /// dead-lettered. This property is read-only.
    pub fn dead_letter_source(&self) -> Option<&str> {
        self.raw_amqp_message
            .message_annotations
            .as_ref()?
            .get(&DEAD_LETTER_SOURCE_NAME as &dyn AnnotationKey)
            .map(|value| match value {
                Value::String(s) => s.as_str(),
                _ => unreachable!("Expecting a String"),
            })
    }

    /// Gets the original sequence number of the message.
    ///
    /// For messages that have been auto-forwarded, this property reflects the sequence number that
    /// had first been assigned to the message at its original point of submission. This property is
    /// read-only.
    pub fn enqueued_sequence_number(&self) -> i64 {
        self.raw_amqp_message
            .message_annotations
            .as_ref()
            .and_then(|m| m.get(&ENQUEUE_SEQUENCE_NUMBER_NAME as &dyn AnnotationKey))
            .map(|value| match value {
                Value::Long(val) => *val,
                _ => unreachable!("Expecting a Long"),
            })
            .unwrap_or_default()
    }

    /// Gets the date and time of the sent time in UTC.
    ///
    /// The UTC instant at which the message has been accepted and stored in the entity. This value
    /// can be used as an authoritative and neutral arrival time indicator when the receiver does
    /// not want to trust the sender's clock. This property is read-only.
    pub fn enqueued_time(&self) -> OffsetDateTime {
        self.raw_amqp_message
            .message_annotations
            .as_ref()
            .and_then(|m| m.get(&ENQUEUED_TIME_UTC_NAME as &dyn AnnotationKey))
            .map(|value| match value {
                Value::Timestamp(timestamp) => {
                    let millis = timestamp.milliseconds();
                    let duration = TimeSpan::milliseconds(millis);
                    OffsetDateTime::UNIX_EPOCH + duration
                }
                _ => unreachable!("Expecting a Timestamp"),
            })
            .unwrap_or(DEFAULT_OFFSET_DATE_TIME)
    }

    /// Gets the date and time in UTC at which the message is set to expire.
    ///
    /// The UTC instant at which the message is marked for removal and no longer available for
    /// retrieval from the entity due to expiration. Expiry is controlled by the
    /// [time_to_live](#method.time_to_live) property and this property is computed from
    /// [enqueued_time](#method.enqueued_time)
    pub fn expires_at(&self) -> OffsetDateTime {
        match self
            .raw_amqp_message
            .properties
            .as_ref()
            .and_then(|p| p.absolute_expiry_time.as_ref())
        {
            Some(timestamp) => {
                let millis = timestamp.milliseconds();
                let duration = TimeSpan::milliseconds(millis);
                OffsetDateTime::UNIX_EPOCH + duration
            }
            None => {
                let ttl = self
                    .time_to_live()
                    .map(|ttl| TimeSpan::try_from(ttl).unwrap_or(TimeSpan::MAX))
                    .unwrap_or(TimeSpan::MAX);
                let enqueue_time = self.enqueued_time();
                if ttl >= MAX_OFFSET_DATE_TIME - enqueue_time {
                    MAX_OFFSET_DATE_TIME
                } else {
                    enqueue_time + ttl
                }
            }
        }
    }

    /// Gets the dead letter reason for the message.
    pub fn dead_letter_reason(&self) -> Option<&str> {
        self.raw_amqp_message
            .application_properties
            .as_ref()?
            .get(DEAD_LETTER_REASON_HEADER)
            .map(|value| match value {
                SimpleValue::String(s) => s.as_str(),
                _ => unreachable!("Expecting a String"),
            })
    }

    /// Gets the dead letter error description for the message.
    pub fn dead_letter_error_description(&self) -> Option<&str> {
        self.raw_amqp_message
            .application_properties
            .as_ref()?
            .get(DEAD_LETTER_ERROR_DESCRIPTION_HEADER)
            .map(|value| match value {
                SimpleValue::String(s) => s.as_str(),
                _ => unreachable!("Expecting a String"),
            })
    }

    /// Gets the state of the message.
    ///
    /// The state of the message can be Active, Deferred, or Scheduled. Deferred messages have
    /// Deferred state, scheduled messages have Scheduled state, all other messages have Active
    /// state.
    pub fn state(&self) -> ServiceBusMessageState {
        self.raw_amqp_message
            .message_annotations
            .as_ref()
            .and_then(|m| m.get(&MESSAGE_STATE_NAME as &dyn AnnotationKey))
            .map(|value| match value {
                Value::Long(val) => ServiceBusMessageState::from(*val),
                _ => unreachable!("Expecting a Long"),
            })
            .unwrap_or_default()
    }
}

impl std::fmt::Display for ServiceBusPeekedMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.message_id() {
            Some(id) => write!(f, "{{MessageId:{}}}", id),
            None => write!(f, "{{MessageId:None"),
        }
    }
}
