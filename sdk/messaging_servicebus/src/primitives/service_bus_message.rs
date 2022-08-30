use std::{collections::BTreeMap, time::Duration};

use fe2o3_amqp_types::{
    messaging::{Body, Data, Message, MessageId, Properties},
    primitives::{Binary, Value},
};
use time::OffsetDateTime;

use super::service_bus_received_message::ServiceBusReceivedMessage;

pub(crate) type AmqpMessage = Message<Value>;

#[derive(Debug, thiserror::Error)]
pub enum ServiceBusMessageError {
    #[error("Not supported {}", .0)]
    NotSupported(String),
}

/// The [ServiceBusMessage] is used to send data to Service Bus Queues and Topics. When receiving messages, the <see
/// cref="ServiceBusReceivedMessage"/> is used.
///
/// The message structure is discussed in detail in the [product
/// documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads)
pub struct ServiceBusMessage {
    pub(crate) amqp_message: AmqpMessage,
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

impl From<ServiceBusReceivedMessage> for ServiceBusMessage {
    fn from(received: ServiceBusReceivedMessage) -> Self {
        todo!()
    }
}

impl ServiceBusMessage {
    /// Gets the raw AMQP message
    pub fn raw_amqp_message(&self) -> &AmqpMessage {
        &self.amqp_message
    }

    /// Gets the body of the message
    pub fn body(&self) -> Result<&[u8], ServiceBusMessageError> {
        match &self.amqp_message.body {
            Body::Data(Data(buf)) => Ok(buf),
            Body::Sequence(_) => Err(not_supported_error(
                "Body::Sequence",
                "body()",
                "raw_amqp_message()",
            )),
            Body::Value(_) => Err(not_supported_error(
                "Body::Value",
                "body()",
                "raw_amqp_message()",
            )),
            Body::Empty => Err(not_supported_error(
                "Body::Empty",
                "body()",
                "raw_amqp_message()",
            )),
        }
    }

    /// Sets the body of the message
    pub fn set_body(&mut self, body: impl Into<Vec<u8>>) {
        self.amqp_message.body = Body::Data(Data(Binary::from(body)))
    }

    /// Gets the MessageId to identify the message.
    ///
    /// The message identifier is an application-defined value that uniquely identifies the message and its payload. The
    /// identifier is a free-form string and can reflect a GUID or an identifier derived from the application context.
    /// If enabled, the [duplicate
    /// detection](https://docs.microsoft.com/azure/service-bus-messaging/duplicate-detection) feature identifies and
    /// removes second and further submissions of messages with the same MessageId.
    pub fn message_id(&self) -> Option<Result<&str, ServiceBusMessageError>> {
        match self.amqp_message.properties.as_ref()?.message_id.as_ref()? {
            MessageId::String(val) => Some(Ok(val)),
            MessageId::ULong(_) => Some(Err(not_supported_error(
                "MessageId::ULong",
                "message_id()",
                "raw_amqp_message()",
            ))),
            MessageId::Uuid(_) => Some(Err(not_supported_error(
                "MessageId::Uuid",
                "message_id()",
                "raw_amqp_message()",
            ))),
            MessageId::Binary(_) => Some(Err(not_supported_error(
                "MessageId::Binary",
                "message_id()",
                "raw_amqp_message()",
            ))),
        }
    }

    /// Sets the MessageId
    pub fn set_message_id(&mut self, message_id: impl Into<String>) {
        self.amqp_message
            .properties
            .get_or_insert(Properties::default())
            .message_id = Some(MessageId::String(message_id.into()))
    }

    /// <summary>Gets or sets a partition key for sending a message to a partitioned entity.</summary>
    /// <value>The partition key. Maximum length is 128 characters.</value>
    /// <remarks>
    /// For <see href="https://docs.microsoft.com/azure/service-bus-messaging/service-bus-partitioning">partitioned entities</see>,
    /// setting this value enables assigning related messages to the same internal partition, so that submission sequence
    /// order is correctly recorded. The partition is chosen by a hash function over this value and cannot be chosen
    /// directly. For session-aware entities, the <see cref="SessionId"/> property overrides this value.
    /// </remarks>
    pub fn partition_key(&self) -> Option<&str> {
        todo!()
    }

    pub fn set_partition_key(&mut self, key: impl Into<String>) {
        todo!()
    }

    /// <summary>Gets or sets a partition key for sending a message into an entity via a partitioned transfer queue.</summary>
    /// <value>The partition key. Maximum length is 128 characters. </value>
    /// <remarks>
    /// If a message is sent via a transfer queue in the scope of a transaction, this value selects the
    /// transfer queue partition: This is functionally equivalent to <see cref="PartitionKey"/> and ensures that
    /// messages are kept together and in order as they are transferred.
    /// See <see href="https://docs.microsoft.com/azure/service-bus-messaging/service-bus-transactions#transfers-and-send-via">Transfers and Send Via</see>.
    /// </remarks>
    pub fn transaction_partition_key(&self) -> Option<&str> {
        todo!()
    }

    pub fn set_transaction_partition_key(&mut self, key: impl Into<String>) {
        todo!()
    }

    /// <summary>Gets or sets the session identifier for a session-aware entity.</summary>
    /// <value>The session identifier. Maximum length is 128 characters.</value>
    /// <remarks>
    /// For session-aware entities, this application-defined value specifies the session
    /// affiliation of the message. Messages with the same session identifier are subject
    /// to summary locking and enable exact in-order processing and demultiplexing.
    /// For session-unaware entities, this value is ignored.
    /// See <see href="https://docs.microsoft.com/azure/service-bus-messaging/message-sessions">Message Sessions</see>.
    /// </remarks>
    pub fn session_id(&self) -> Option<&str> {
        todo!()
    }

    pub fn set_session_id(&mut self, session_id: impl Into<String>) {
        todo!()
    }

    /// <summary>Gets or sets a session identifier augmenting the <see cref="ReplyTo"/> address.</summary>
    /// <value>Session identifier. Maximum length is 128 characters.</value>
    /// <remarks>
    /// This value augments the ReplyTo information and specifies which SessionId should be set
    /// for the reply when sent to the reply entity.
    /// See <see href="https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation">Message Routing and Correlation</see>
    /// </remarks>
    pub fn reply_to_session_id(&self) -> Option<&str> {
        todo!()
    }

    pub fn set_reply_to_session_id(&mut self, session_id: impl Into<String>) {
        todo!()
    }

    /// <summary>
    /// Gets or sets the message’s "time to live" value.
    /// </summary>
    /// <value>The message’s time to live value.</value>
    /// <remarks>
    /// This value is the relative duration after which the message expires.
    /// When not set explicitly, the assumed value is the DefaultTimeToLive for the respective queue or topic.
    /// A message-level <see cref="TimeToLive"/> value cannot be longer than the entity's DefaultTimeToLive
    /// setting and it is silently adjusted if it does.
    /// See <see href="https://docs.microsoft.com/azure/service-bus-messaging/message-expiration">Expiration</see>.
    /// </remarks>
    pub fn time_to_live(&self) -> Option<Duration> {
        todo!()
    }

    pub fn set_time_to_live(&mut self, ttl: Duration) {
        todo!()
    }

    /// <summary>Gets or sets the a correlation identifier.</summary>
    /// <value>Correlation identifier.</value>
    /// <remarks>
    /// Allows an application to specify a context for the message for the purposes of correlation,
    /// for example reflecting the MessageId of a message that is being replied to.
    /// See <see href="https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation">Message Routing and Correlation</see>.
    /// </remarks>
    pub fn correlation_id(&self) -> Option<&str> {
        todo!()
    }

    pub fn set_correlation_id(&mut self, id: impl Into<String>) {
        todo!()
    }

    /// <summary>Gets or sets an application specific subject.</summary>
    /// <value>The application specific subject.</value>
    /// <remarks>
    /// This property enables the application to indicate the purpose of the message to the receiver in a standardized
    /// fashion, similar to an email subject line. The mapped AMQP property is "subject".
    /// </remarks>
    pub fn subject(&self) -> Option<&str> {
        todo!()
    }

    pub fn set_subject(&mut self, subject: impl Into<String>) {
        todo!()
    }

    /// <summary>Gets or sets the "to" address.</summary>
    /// <value>The "to" address.</value>
    /// <remarks>
    /// This property is reserved for future use in routing scenarios and presently ignored by the broker itself.
    /// Applications can use this value in rule-driven
    /// <see href="https://docs.microsoft.com/azure/service-bus-messaging/service-bus-auto-forwarding">auto-forward chaining</see> scenarios to indicate the
    /// intended logical destination of the message.
    /// </remarks>
    pub fn to(&self) -> Option<&str> {
        todo!()
    }

    pub fn set_to(&mut self, to: impl Into<String>) {
        todo!()
    }

    /// <summary>Gets or sets the content type descriptor.</summary>
    /// <value>RFC2045 Content-Type descriptor.</value>
    /// <remarks>
    /// Optionally describes the payload of the message, with a descriptor following the format of
    /// RFC2045, Section 5, for example "application/json".
    /// </remarks>
    pub fn content_type(&self) -> Option<&str> {
        todo!()
    }

    pub fn set_content_type(&mut self, content_type: impl Into<String>) {
        todo!()
    }

    /// <summary>Gets or sets the address of an entity to send replies to.</summary>
    /// <value>The reply entity address.</value>
    /// <remarks>
    /// This optional and application-defined value is a standard way to express a reply path
    /// to the receiver of the message. When a sender expects a reply, it sets the value to the
    /// absolute or relative path of the queue or topic it expects the reply to be sent to.
    /// See <see href="https://docs.microsoft.com/azure/service-bus-messaging/service-bus-messages-payloads?#message-routing-and-correlation">Message Routing and Correlation</see>.
    /// </remarks>
    pub fn reply_to(&self) -> Option<&str> {
        todo!()
    }

    pub fn set_reply_to(&mut self, reply_to: impl Into<String>) {
        todo!()
    }

    /// <summary>
    /// Gets or sets the date and time in UTC at which the message will be enqueued. This
    /// property returns the time in UTC; when setting the property, the supplied DateTime value must also be in UTC.
    /// </summary>
    /// <value>
    /// The scheduled enqueue time in UTC. This value is for delayed message sending.
    /// It is utilized to delay messages sending to a specific time in the future.
    /// </value>
    /// <remarks>
    /// Message enqueuing time does not mean that the message will be sent at the same time. It will get enqueued, but the actual sending time
    /// depends on the queue's workload and its state.
    /// </remarks>
    pub fn scheduled_enqueue_time(&self) -> OffsetDateTime {
        todo!()
    }

    /// <summary>
    /// Gets the application properties bag, which can be used for custom message metadata.
    /// </summary>
    /// <remarks>
    ///   <list type="bullet">
    ///     <listheader><description>The following types are supported:</description></listheader>
    ///     <item><description>string</description></item>
    ///     <item><description>bool</description></item>
    ///     <item><description>byte</description></item>
    ///     <item><description>sbyte</description></item>
    ///     <item><description>short</description></item>
    ///     <item><description>ushort</description></item>
    ///     <item><description>int</description></item>
    ///     <item><description>uint</description></item>
    ///     <item><description>long</description></item>
    ///     <item><description>ulong</description></item>
    ///     <item><description>float</description></item>
    ///     <item><description>decimal</description></item>
    ///     <item><description>double</description></item>
    ///     <item><description>char</description></item>
    ///     <item><description>Guid</description></item>
    ///     <item><description>DateTime</description></item>
    ///     <item><description>DateTimeOffset</description></item>
    ///     <item><description>Stream</description></item>
    ///     <item><description>Uri</description></item>
    ///     <item><description>TimeSpan</description></item>
    ///   </list>
    /// </remarks>
    /// <exception cref="System.Runtime.Serialization.SerializationException">
    ///   Occurs when the <see cref="ServiceBusMessage" /> is serialized for transport when an unsupported type is used as a property.
    /// </exception>
    pub fn application_properties(&self) -> Option<&BTreeMap<String, Value>> {
        todo!()
    }
}

impl ToString for ServiceBusMessage {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[inline]
fn not_supported_error(
    field_type: &str,
    method: &str,
    alternative: &str,
) -> ServiceBusMessageError {
    ServiceBusMessageError::NotSupported(
        format!("{field_type} cannot be retrived using {method} method. Use {alternative} to access the underlying Amqp Message")
    )
}
