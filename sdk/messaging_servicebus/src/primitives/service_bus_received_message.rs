use std::borrow::Cow;

use fe2o3_amqp::link::delivery::DeliveryInfo;
use time::Duration as TimeSpan;

use fe2o3_amqp_types::{
    messaging::{
        annotations::AnnotationKey, ApplicationProperties, Body, Message, MessageAnnotations,
    },
    primitives::{SimpleValue, Timestamp, Uuid, Value},
};
use time::OffsetDateTime;

use crate::{
    amqp::{
        amqp_message_constants::{
            DEAD_LETTER_ERROR_DESCRIPTION_HEADER, DEAD_LETTER_REASON_HEADER,
            DEAD_LETTER_SOURCE_NAME, ENQUEUED_TIME_UTC_NAME, ENQUEUE_SEQUENCE_NUMBER_NAME,
            LOCKED_UNTIL_NAME, MESSAGE_STATE_NAME, SEQUENCE_NUMBER_NAME,
        },
        amqp_message_extensions::AmqpMessageExt,
        error::Error,
    },
    constants::{DEFAULT_OFFSET_DATE_TIME, MAX_OFFSET_DATE_TIME},
};

use super::service_bus_message_state::ServiceBusMessageState;

/// The lock token for a received message
#[derive(Debug, Clone)]
pub(crate) enum ReceivedMessageLockToken {
    /// Message that is received using the request-response link will only
    /// have a UUID lock token
    LockToken(Uuid),

    /// Message that is received using the receive link will have complete
    /// delivery information that will be used for disposition on the AMQP receiver link
    Delivery {
        delivery_info: DeliveryInfo,
        lock_token: Uuid,
    },
}

/// The <see cref="ServiceBusReceivedMessage"/> is used to receive data from Service Bus Queues and
/// Subscriptions. When sending messages, the <see cref="ServiceBusMessage"/> is used.
///
/// The message structure is discussed in detail in the [product
/// documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads)
#[derive(Debug)]
pub struct ServiceBusReceivedMessage {
    /// Indicates whether the user has settled the message as part of their callback.
    /// If they have done so, we will not autocomplete.
    pub(crate) is_settled: bool,

    /// Gets the raw Amqp message data that was transmitted over the wire.
    /// This can be used to enable scenarios that require reading AMQP header, footer, property, or annotation
    /// data that is not exposed as top level properties in the <see cref="ServiceBusReceivedMessage"/>.
    pub(crate) raw_amqp_message: Message<Body<Value>>,

    /// Delivery Info
    pub(crate) lock_token: ReceivedMessageLockToken,
}

impl From<ServiceBusReceivedMessage> for ReceivedMessageLockToken {
    fn from(message: ServiceBusReceivedMessage) -> Self {
        message.lock_token
    }
}

impl From<&ServiceBusReceivedMessage> for ReceivedMessageLockToken {
    fn from(message: &ServiceBusReceivedMessage) -> Self {
        message.lock_token.clone()
    }
}

impl ServiceBusReceivedMessage {
    /// Gets the raw Amqp message data that was transmitted over the wire. This can be used to
    /// enable scenarios that require reading AMQP header, footer, property, or annotation data that
    /// is not exposed as top level properties in the [`ServiceBusReceivedMessage`].
    ///
    /// # Returns
    ///
    /// The raw Amqp message.
    pub fn raw_amqp_message(&self) -> &Message<Body<Value>> {
        &self.raw_amqp_message
    }

    /// Gets the body of the message.
    pub fn body(&self) -> Result<&[u8], Error> {
        match &self.raw_amqp_message.body {
            Body::Data(batch) => match batch.len() {
                1 => Ok(batch[0].0.as_ref()),
                _ => Err(Error::RawAmqpMessage),
            },
            _ => Err(Error::RawAmqpMessage),
        }
    }

    /// Gets the MessageId to identify the message.
    ///
    /// # Remarks
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
    /// function over this value and cannot be chosen directly. For session-aware entities, the
    /// [`session_id()`](#method.session_id) property overrides this value.
    pub fn partition_key(&self) -> Option<&str> {
        self.raw_amqp_message.partition_key()
    }

    /// Gets a partition key for sending a message into an entity via a partitioned transfer queue.
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
    /// Via](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-transactions#transfers-and-send-viaå).
    pub fn transaction_partition_key(&self) -> Option<&str> {
        self.raw_amqp_message.via_partition_key()
    }

    /// Gets the session identifier for a session-aware entity.
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
        self.raw_amqp_message.session_id()
    }

    /// Gets a session identifier augmenting the [`reply_to()`](#method.reply_to) address.
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
        self.raw_amqp_message.reply_to_session_id()
    }

    /// Gets the message’s "time to live" value.
    ///
    /// # Value
    ///
    /// The message’s time to live value.
    ///
    /// # Remarks
    ///
    /// This value is the relative duration after which the message expires, starting from the
    /// instant the message has been accepted and stored by the broker, as captured in <see
    /// cref="EnqueuedTime"/>. When not set explicitly, the assumed value is the DefaultTimeToLive
    /// for the respective queue or topic. A message-level <see cref="TimeToLive"/> value cannot be
    /// longer than the entity's DefaultTimeToLive setting and it is silently adjusted if it does.
    /// See [Expiration](https://docs.microsoft.com/azure/service-bus-messaging/message-expiration)
    pub fn time_to_live(&self) -> TimeSpan {
        match self.raw_amqp_message.time_to_live() {
            Some(ttl) => ttl,
            None => TimeSpan::MAX, // TODO: is this the same as in dotnet sdk?
        }
    }

    /// Gets the correlation identifier.
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
    /// Correlation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation").
    pub fn correlation_id(&self) -> Option<Cow<'_, str>> {
        self.raw_amqp_message.correlation_id()
    }

    /// Gets an application specific label.
    ///
    /// # Value
    ///
    /// The application specific label
    ///
    /// # Remarks
    /// This property enables the application to indicate the purpose of the message to the receiver
    /// in a standardized fashion, similar to an email subject line. The mapped AMQP property is
    /// "subject".
    pub fn subject(&self) -> Option<&str> {
        self.raw_amqp_message.subject()
    }

    /// Gets the "to" address.
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
        self.raw_amqp_message.to()
    }

    /// Gets the content type descriptor.
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
        self.raw_amqp_message.content_type()
    }

    /// Gets the address of an entity to send replies to.
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
    /// Correlation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation)
    pub fn reply_to(&self) -> Option<&str> {
        self.raw_amqp_message.reply_to()
    }

    /// Gets the date and time in UTC at which the message will be enqueued. This property returns
    /// the time in UTC; when setting the property, the supplied DateTime value must also be in UTC.
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
        self.raw_amqp_message.scheduled_enqueue_time()
    }

    /// Gets the application properties bag, which can be used for custom message metadata.
    ///
    /// # Remarks
    /// Only the following value types are supported: byte, sbyte, char, short, ushort, int, uint,
    /// long, ulong, float, double, decimal, bool, Guid, string, Uri, DateTime, DateTimeOffset,
    /// TimeSpan
    pub fn application_properties(&self) -> Option<&ApplicationProperties> {
        self.raw_amqp_message.application_properties.as_ref()
    }

    /// Gets the lock token for the current message.
    ///
    /// # Remarks
    ///
    /// The lock token is a reference to the lock that is being held by the broker in
    /// ReceiveMode.PeekLock mode. Locks are used to explicitly settle messages as explained in the
    /// documentation in more
    /// detail](https://docs.microsoft.com/azure/service-bus-messaging/message-transfers-locks-settlement).
    /// The token can also be used to pin the lock permanently through the [Deferral
    /// API](https://docs.microsoft.com/azure/service-bus-messaging/message-deferral) and, with
    /// that, take the message out of the regular delivery state flow. This property is read-only.
    ///
    /// # Value
    ///
    /// A `Some(lock_token)` is returned if the lock token is found. Otherwise, `None` is returned.
    pub fn lock_token(&self) -> &Uuid {
        match &self.lock_token {
            ReceivedMessageLockToken::LockToken(lock_token) => lock_token,
            ReceivedMessageLockToken::Delivery { lock_token, .. } => lock_token,
        }
    }

    /// Get the current delivery count.
    ///
    /// # Value
    ///
    /// This value starts at 1. This number is off by one from the raw amqp message delivery count
    ///
    /// # Remarks
    /// Number of deliveries that have been attempted for this message. The count is incremented
    /// when a message lock expires, or the message is explicitly abandoned by the receiver. This
    /// property is read-only.
    pub fn delivery_count(&self) -> Option<u32> {
        self.raw_amqp_message
            .header
            .as_ref()
            .map(|h| h.delivery_count + 1)
    }

    // pub(crate) fn set_delivery_count(&mut self, count: u32) {
    //     self.delivery
    //         .message()
    //         .header
    //         .get_or_insert(Header::default())
    //         .delivery_count = count - 1;
    // }

    /// Gets the date and time in UTC until which the message will be locked in the
    /// queue/subscription.
    ///
    /// # Value
    ///
    /// The date and time until which the message will be locked in the queue/subscription.
    ///
    /// # Remarks
    ///
    /// For messages retrieved under a lock (peek-lock receive mode, not pre-settled) this property
    /// reflects the UTC instant until which the message is held locked in the queue/subscription.
    /// When the lock expires, the [`delivery_count`](#method.delivery_count) is incremented and the
    /// message is again available for retrieval. This property is read-only.
    pub fn locked_until(&self) -> Option<OffsetDateTime> {
        self.raw_amqp_message
            .message_annotations
            .as_ref()?
            .get(&LOCKED_UNTIL_NAME as &dyn AnnotationKey)
            .map(|value| match value {
                Value::Timestamp(timestamp) => {
                    let millis = timestamp.milliseconds();
                    let duration = TimeSpan::milliseconds(millis);
                    OffsetDateTime::UNIX_EPOCH + duration
                }
                _ => unreachable!("Expecting a Timestamp"),
            })
    }

    /// # Panic
    ///
    /// Panics if timestamp exceeds the valid range of i64
    pub(crate) fn set_locked_until(&mut self, value: impl Into<Timestamp>) {
        let value: Timestamp = value.into();
        self.raw_amqp_message
            .message_annotations
            .get_or_insert(MessageAnnotations::default())
            .insert(LOCKED_UNTIL_NAME.into(), value.into());
    }

    /// Gets the unique number assigned to a message by Service Bus.
    ///
    /// # Remarks
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
            .unwrap_or(Default::default())
    }

    // pub(crate) fn set_sequence_number(&mut self, value: i64) {
    //     self.delivery
    //         .message()
    //         .message_annotations
    //         .get_or_insert(MessageAnnotations::default())
    //         .insert(SEQUENCE_NUMBER_NAME.into(), value.into());
    // }

    /// Gets the name of the queue or subscription that this message was enqueued on, before it was
    /// dead-lettered.
    ///
    /// # Remarks
    ///
    /// Only set in messages that have been dead-lettered and subsequently auto-forwarded from the
    /// dead-letter queue to another entity. Indicates the entity in which the message was
    /// dead-lettered. This property is read-only.
    pub fn dead_letter_source(&self) -> Option<&str> {
        self.raw_amqp_message
            .message_annotations
            .as_ref()?
            .get(&DEAD_LETTER_SOURCE_NAME as &dyn AnnotationKey)
            .and_then(|value| match value {
                Value::String(s) => Some(s.as_str()),
                _ => unreachable!("Expecting a String"),
            })
    }

    // pub(crate) fn set_dead_letter_source(&mut self, value: impl Into<String>) {
    //     let value = value.into();
    //     self.delivery
    //         .message()
    //         .message_annotations
    //         .get_or_insert(MessageAnnotations::default())
    //         .insert(DEAD_LETTER_SOURCE_NAME.into(), value.into());
    // }

    /// Gets the original sequence number of the message.
    ///
    /// # Value
    ///
    /// The enqueued sequence number of the message.
    ///
    /// # Remarks
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
            .unwrap_or(Default::default())
    }

    // pub(crate) fn set_enqueued_sequence_number(&mut self, value: i64) {
    //     self.delivery
    //         .message()
    //         .message_annotations
    //         .get_or_insert(MessageAnnotations::default())
    //         .insert(ENQUEUE_SEQUENCE_NUMBER_NAME.into(), value.into());
    // }

    /// Gets the date and time of the sent time in UTC.
    ///
    /// # Value
    ///
    /// The enqueue time in UTC.
    ///
    /// # Remarks
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

    // pub(crate) fn set_enqueued_time(&mut self, value: OffsetDateTime) {
    //     let timespan = value - OffsetDateTime::UNIX_EPOCH;
    //     let millis = timespan.whole_milliseconds();
    //     assert!(millis <= i64::MAX as i128 && millis >= i64::MIN as i128);
    //     let millis = Timestamp::from_milliseconds(millis as i64);

    //     self.delivery
    //         .message()
    //         .message_annotations
    //         .get_or_insert(MessageAnnotations::default())
    //         .insert(ENQUEUED_TIME_UTC_NAME.into(), millis.into());
    // }

    /// Gets the date and time in UTC at which the message is set to expire.
    ///
    /// # Value
    ///
    /// The message expiration time in UTC. This property is read-only.
    ///
    /// # Remarks
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
                let ttl = self.time_to_live();
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
    /// # Value
    ///
    /// The state of the message. </value>
    ///
    /// # Remarks
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

    // pub(crate) fn set_state(&mut self, state: ServiceBusMessageState) {
    //     let value = state as i64;
    //     self.delivery
    //         .message()
    //         .message_annotations
    //         .get_or_insert(MessageAnnotations::default())
    //         .insert(MESSAGE_STATE_NAME.into(), value.into());
    // }
}

/// Returns a string that represents the current message.
impl ToString for ServiceBusReceivedMessage {
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
