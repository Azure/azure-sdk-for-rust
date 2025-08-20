// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{ErrorKind, Result, ServiceBusError};
use azure_core::fmt::SafeDebug;
use azure_core::time::Duration;
use azure_core_amqp::{
    message::{AmqpApplicationProperties, AmqpMessageBody, AmqpMessageId, AmqpMessageProperties},
    AmqpMessage, AmqpSimpleValue, AmqpSymbol,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

/// A message to be sent to Service Bus.
#[derive(SafeDebug, Clone)]
pub struct Message {
    body: Vec<u8>,
    properties: HashMap<String, String>,
    session_id: Option<String>,
    message_id: Option<String>,
    correlation_id: Option<String>,
    content_type: Option<String>,
    reply_to: Option<String>,
    reply_to_session_id: Option<String>,
    subject: Option<String>,
    time_to_live: Option<Duration>,
    scheduled_enqueue_time: Option<OffsetDateTime>,
}

impl Message {
    /// Creates a new message with the specified body.
    pub fn new<T: Into<Vec<u8>>>(body: T) -> Self {
        Self {
            body: body.into(),
            properties: HashMap::new(),
            session_id: None,
            message_id: None,
            correlation_id: None,
            content_type: None,
            reply_to: None,
            reply_to_session_id: None,
            subject: None,
            time_to_live: None,
            scheduled_enqueue_time: None,
        }
    }

    /// Gets the message body as bytes.
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Gets the message body as a string, if it's valid UTF-8.
    pub fn body_as_string(&self) -> Result<String> {
        String::from_utf8(self.body.clone())
            .map_err(|_| ServiceBusError::new(ErrorKind::InvalidRequest, "Body is not valid UTF-8"))
    }

    /// Sets a custom property on the message.
    pub fn set_property(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.properties.insert(key.into(), value.into());
    }

    /// Gets a custom property from the message.
    pub fn property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }

    /// Gets all custom properties.
    pub fn properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    /// Sets the session ID for the message.
    pub fn set_session_id(&mut self, session_id: impl Into<String>) {
        self.session_id = Some(session_id.into());
    }

    /// Gets the session ID.
    pub fn session_id(&self) -> Option<&String> {
        self.session_id.as_ref()
    }

    /// Sets the message ID.
    pub fn set_message_id(&mut self, message_id: impl Into<String>) {
        self.message_id = Some(message_id.into());
    }

    /// Gets the message ID.
    pub fn message_id(&self) -> Option<&String> {
        self.message_id.as_ref()
    }

    /// Sets the correlation ID.
    pub fn set_correlation_id(&mut self, correlation_id: impl Into<String>) {
        self.correlation_id = Some(correlation_id.into());
    }

    /// Gets the correlation ID.
    pub fn correlation_id(&self) -> Option<&String> {
        self.correlation_id.as_ref()
    }

    /// Sets the content type.
    pub fn set_content_type(&mut self, content_type: impl Into<String>) {
        self.content_type = Some(content_type.into());
    }

    /// Gets the content type.
    pub fn content_type(&self) -> Option<&String> {
        self.content_type.as_ref()
    }

    /// Sets the reply-to address.
    pub fn set_reply_to(&mut self, reply_to: impl Into<String>) {
        self.reply_to = Some(reply_to.into());
    }

    /// Gets the reply-to address.
    pub fn reply_to(&self) -> Option<&String> {
        self.reply_to.as_ref()
    }

    /// Sets the reply-to session ID.
    pub fn set_reply_to_session_id(&mut self, reply_to_session_id: impl Into<String>) {
        self.reply_to_session_id = Some(reply_to_session_id.into());
    }

    /// Gets the reply-to session ID.
    pub fn reply_to_session_id(&self) -> Option<&String> {
        self.reply_to_session_id.as_ref()
    }

    /// Sets the message subject (label).
    pub fn set_subject(&mut self, subject: impl Into<String>) {
        self.subject = Some(subject.into());
    }

    /// Gets the message subject (label).
    pub fn subject(&self) -> Option<&String> {
        self.subject.as_ref()
    }

    /// Sets the time-to-live for the message.
    pub fn set_time_to_live(&mut self, time_to_live: Duration) {
        self.time_to_live = Some(time_to_live);
    }

    /// Gets the time-to-live.
    pub fn time_to_live(&self) -> Option<Duration> {
        self.time_to_live
    }

    /// Sets the scheduled enqueue time.
    pub fn set_scheduled_enqueue_time(&mut self, scheduled_enqueue_time: OffsetDateTime) {
        self.scheduled_enqueue_time = Some(scheduled_enqueue_time);
    }

    /// Gets the scheduled enqueue time.
    pub fn scheduled_enqueue_time(&self) -> Option<OffsetDateTime> {
        self.scheduled_enqueue_time
    }
}

impl From<&str> for Message {
    fn from(body: &str) -> Self {
        Self::new(body.as_bytes().to_vec())
    }
}

impl From<String> for Message {
    fn from(body: String) -> Self {
        Self::new(body.into_bytes())
    }
}

/// A message received from Service Bus.
#[derive(SafeDebug, Clone)]
pub struct ReceivedMessage {
    body: Vec<u8>,
    properties: HashMap<String, String>,
    system_properties: SystemProperties,
    lock_token: Option<Uuid>,
}

/// System properties of a received message.
#[derive(SafeDebug, Clone, Default, Serialize, Deserialize)]
pub struct SystemProperties {
    /// The message ID.
    pub message_id: Option<String>,
    /// The correlation ID.
    pub correlation_id: Option<String>,
    /// The session ID.
    pub session_id: Option<String>,
    /// The content type.
    pub content_type: Option<String>,
    /// The reply-to address.
    pub reply_to: Option<String>,
    /// The reply-to session ID.
    pub reply_to_session_id: Option<String>,
    /// The message subject (label).
    pub subject: Option<String>,
    /// The enqueued time in UTC.
    pub enqueued_time_utc: Option<OffsetDateTime>,
    /// The sequence number.
    pub sequence_number: Option<i64>,
    /// The delivery count.
    pub delivery_count: Option<u32>,
    /// The time-to-live.
    pub time_to_live: Option<Duration>,
    /// The dead letter source.
    pub dead_letter_source: Option<String>,
    /// The dead letter reason.
    pub dead_letter_reason: Option<String>,
    /// The dead letter error description.
    pub dead_letter_error_description: Option<String>,
}

impl ReceivedMessage {
    /// Creates a new received message.
    pub(crate) fn new(
        body: Vec<u8>,
        properties: HashMap<String, String>,
        system_properties: SystemProperties,
        lock_token: Option<Uuid>,
    ) -> Self {
        Self {
            body,
            properties,
            system_properties,
            lock_token,
        }
    }

    /// Gets the message body as bytes.
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Gets the message body as a string, if it's valid UTF-8.
    pub fn body_as_string(&self) -> Result<String> {
        String::from_utf8(self.body.clone())
            .map_err(|_| ServiceBusError::new(ErrorKind::InvalidRequest, "Body is not valid UTF-8"))
    }

    /// Gets a custom property from the message.
    pub fn property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }

    /// Gets all custom properties.
    pub fn properties(&self) -> &HashMap<String, String> {
        &self.properties
    }

    /// Gets the system properties.
    pub fn system_properties(&self) -> &SystemProperties {
        &self.system_properties
    }

    /// Gets the lock token for this message.
    pub fn lock_token(&self) -> Option<Uuid> {
        self.lock_token
    }

    /// Gets the message ID.
    pub fn message_id(&self) -> Option<&String> {
        self.system_properties.message_id.as_ref()
    }

    /// Gets the correlation ID.
    pub fn correlation_id(&self) -> Option<&String> {
        self.system_properties.correlation_id.as_ref()
    }

    /// Gets the session ID.
    pub fn session_id(&self) -> Option<&String> {
        self.system_properties.session_id.as_ref()
    }

    /// Gets the sequence number.
    pub fn sequence_number(&self) -> Option<i64> {
        self.system_properties.sequence_number
    }

    /// Gets the enqueued time in UTC.
    pub fn enqueued_time_utc(&self) -> Option<OffsetDateTime> {
        self.system_properties.enqueued_time_utc
    }

    /// Gets the delivery count.
    pub fn delivery_count(&self) -> Option<u32> {
        self.system_properties.delivery_count
    }
}

impl From<Message> for AmqpMessage {
    fn from(message: Message) -> Self {
        let mut amqp_message_builder = AmqpMessage::builder();

        // Set the body as binary data
        amqp_message_builder =
            amqp_message_builder.with_body(AmqpMessageBody::Binary(vec![message.body]));

        // Set message properties
        let mut properties = AmqpMessageProperties::default();

        if let Some(message_id) = message.message_id {
            properties.message_id = Some(AmqpMessageId::String(message_id));
        }

        if let Some(correlation_id) = message.correlation_id {
            properties.correlation_id = Some(AmqpMessageId::String(correlation_id));
        }

        if let Some(content_type) = message.content_type {
            properties.content_type = Some(AmqpSymbol::from(content_type));
        }

        if let Some(reply_to) = message.reply_to {
            properties.reply_to = Some(reply_to);
        }

        if let Some(subject) = message.subject {
            properties.subject = Some(subject);
        }

        amqp_message_builder = amqp_message_builder.with_properties(properties);

        // Add application properties
        if !message.properties.is_empty() {
            let mut app_props = AmqpApplicationProperties::new();
            for (key, value) in message.properties {
                app_props.insert(key, AmqpSimpleValue::String(value));
            }
            amqp_message_builder = amqp_message_builder.with_application_properties(app_props);
        }

        amqp_message_builder.build()
    }
}

/// Options for creating a message batch.
#[derive(SafeDebug, Clone, Default)]
pub struct CreateMessageBatchOptions {
    /// The maximum size of the batch in bytes.
    /// If not specified, the default Service Bus limit will be used.
    #[allow(dead_code)]
    pub maximum_size_in_bytes: Option<usize>,
}

impl CreateMessageBatchOptions {
    /// Creates new batch options with default values.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum size in bytes for the batch.
    #[allow(dead_code)]
    pub fn with_maximum_size_in_bytes(mut self, size: usize) -> Self {
        self.maximum_size_in_bytes = Some(size);
        self
    }
}

/// A batch of Service Bus messages that can be sent efficiently in a single operation.
///
/// This provides better performance than sending messages individually by reducing
/// the number of network round trips and AMQP operations required.
///
/// Use `Sender::create_message_batch()` to create an instance.
#[derive(SafeDebug)]
pub struct MessageBatch {
    messages: Vec<Message>,
    current_size_in_bytes: usize,
    maximum_size_in_bytes: usize,
}

impl MessageBatch {
    /// The default maximum size in bytes for a Service Bus message batch.
    /// This is based on the Service Bus limit of 1MB minus overhead for headers.
    pub const DEFAULT_MAX_SIZE_BYTES: usize = 1024 * 1024 - 64 * 1024; // 960KB

    /// Creates a new message batch with the specified maximum size.
    pub(crate) fn new(maximum_size_in_bytes: Option<usize>) -> Self {
        Self {
            messages: Vec::new(),
            current_size_in_bytes: 0,
            maximum_size_in_bytes: maximum_size_in_bytes.unwrap_or(Self::DEFAULT_MAX_SIZE_BYTES),
        }
    }

    /// Attempts to add a message to the batch.
    ///
    /// Returns `true` if the message was successfully added, `false` if adding
    /// the message would exceed the batch size limit.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to add to the batch
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use azure_messaging_servicebus::{ServiceBusClient, Message, CreateSenderOptions, CreateMessageBatchOptions};
    /// # use azure_identity::DeveloperToolsCredential;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
    /// let sender = client.create_sender("myqueue", None).await?;
    /// let mut batch = sender.create_message_batch(None).await?;
    /// let message = Message::from("Hello, world!");
    ///
    /// if batch.try_add_message(message) {
    ///     println!("Message added to batch");
    /// } else {
    ///     println!("Batch is full, cannot add message");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn try_add_message(&mut self, message: Message) -> bool {
        let estimated_message_size = self.estimate_message_size(&message);

        if self.current_size_in_bytes + estimated_message_size > self.maximum_size_in_bytes {
            return false;
        }

        self.current_size_in_bytes += estimated_message_size;
        self.messages.push(message);
        true
    }

    /// Gets the number of messages in the batch.
    pub fn count(&self) -> usize {
        self.messages.len()
    }

    /// Returns `true` if the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Gets the current size of the batch in bytes.
    pub fn size_in_bytes(&self) -> usize {
        self.current_size_in_bytes
    }

    /// Gets the maximum size limit for the batch in bytes.
    pub fn maximum_size_in_bytes(&self) -> usize {
        self.maximum_size_in_bytes
    }

    /// Consumes the batch and returns the messages contained within it.
    ///
    /// This is used internally by the sender to extract the messages for sending.
    pub(crate) fn into_messages(self) -> Vec<Message> {
        self.messages
    }

    /// Gets a reference to the messages in the batch.
    ///
    /// This is used internally by the sender.
    #[allow(dead_code)]
    pub(crate) fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Estimates the size of a message in bytes for batching purposes.
    ///
    /// This is an approximation that includes the message body, properties,
    /// and AMQP overhead. The actual wire size may be slightly different.
    fn estimate_message_size(&self, message: &Message) -> usize {
        let mut size = 0;

        // Message body size
        size += message.body.len();

        // Message properties overhead (estimated)
        if let Some(id) = &message.message_id {
            size += id.len() + 16; // Property overhead
        }
        if let Some(correlation_id) = &message.correlation_id {
            size += correlation_id.len() + 16;
        }
        if let Some(content_type) = &message.content_type {
            size += content_type.len() + 16;
        }
        if let Some(reply_to) = &message.reply_to {
            size += reply_to.len() + 16;
        }
        if let Some(subject) = &message.subject {
            size += subject.len() + 16;
        }

        // Custom properties
        for (key, value) in &message.properties {
            size += key.len() + value.len() + 32; // Key-value pair overhead
        }

        // AMQP frame overhead (estimated)
        size += 128;

        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_static_str() {
        let message = Message::from("Hello, world!");
        assert_eq!(message.body_as_string().unwrap(), "Hello, world!");
    }

    #[test]
    fn from_owned_string() {
        let text = String::from("Hello, world!");
        let message = Message::from(text);
        assert_eq!(message.body_as_string().unwrap(), "Hello, world!");
    }

    #[test]
    fn from_string_literal() {
        let message: Message = "Test message".into();
        assert_eq!(message.body_as_string().unwrap(), "Test message");
    }

    #[test]
    fn from_string_variable() {
        let text = format!("Dynamic message {}", 42);
        let message: Message = text.into();
        assert_eq!(message.body_as_string().unwrap(), "Dynamic message 42");
    }
}

#[cfg(test)]
mod batch_tests {
    use super::*;

    #[test]
    fn new_batch_is_empty() {
        let batch = MessageBatch::new(None);
        assert!(batch.is_empty());
        assert_eq!(batch.count(), 0);
        assert_eq!(batch.size_in_bytes(), 0);
    }

    #[test]
    fn try_add_message_success() {
        let mut batch = MessageBatch::new(Some(1024));
        let message = Message::new("Small message");

        let result = batch.try_add_message(message);
        assert!(result);
        assert_eq!(batch.count(), 1);
        assert!(!batch.is_empty());
        assert!(batch.size_in_bytes() > 0);
    }

    #[test]
    fn try_add_message_exceeds_size_limit() {
        let mut batch = MessageBatch::new(Some(100)); // Very small limit
        let message = Message::new(
            "This message is definitely too large for the tiny batch size limit that was set",
        );

        let result = batch.try_add_message(message);
        assert!(!result);
        assert_eq!(batch.count(), 0);
        assert!(batch.is_empty());
        assert_eq!(batch.size_in_bytes(), 0);
    }

    #[test]
    fn batch_respects_maximum_size() {
        let max_size = 500;
        let batch = MessageBatch::new(Some(max_size));
        assert_eq!(batch.maximum_size_in_bytes(), max_size);
    }

    #[test]
    fn default_batch_options() {
        let options = CreateMessageBatchOptions::default();
        assert!(options.maximum_size_in_bytes.is_none());
    }

    #[test]
    fn batch_options_with_size() {
        let size = 2048;
        let options = CreateMessageBatchOptions::new().with_maximum_size_in_bytes(size);
        assert_eq!(options.maximum_size_in_bytes, Some(size));
    }

    #[test]
    fn estimate_message_size() {
        let batch = MessageBatch::new(None);
        let mut message = Message::new("Hello, world!");
        message.set_message_id("test-id");
        message.set_correlation_id("correlation-123");
        message.set_property("custom-key", "custom-value");

        let estimated_size = batch.estimate_message_size(&message);

        // Should include body, properties, and overhead
        assert!(estimated_size > "Hello, world!".len());
        assert!(estimated_size > 100); // At minimum should have overhead
    }

    #[test]
    fn into_messages_consumes_batch() {
        let mut batch = MessageBatch::new(None);
        let message1 = Message::new("Message 1");
        let message2 = Message::new("Message 2");

        batch.try_add_message(message1);
        batch.try_add_message(message2);

        let messages = batch.into_messages();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].body_as_string().unwrap(), "Message 1");
        assert_eq!(messages[1].body_as_string().unwrap(), "Message 2");
    }
}
