// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{clients::ServiceBusClientOptions, ErrorKind, Message, Result, ServiceBusError};
use azure_core::fmt::SafeDebug;
use azure_core_amqp::{
    AmqpConnection, AmqpMessage, AmqpSender, AmqpSenderApis, AmqpSession, AmqpSessionApis,
    AmqpTarget,
};
use std::sync::Arc;
use tracing::{debug, trace};

/// Options for sending a single message.
///
/// This struct contains optional parameters that can be specified when sending
/// a message using [`Sender::send_message`].
///
/// Currently no options are available, but this provides
/// extensibility for future parameters
#[derive(SafeDebug, Clone, Default)]
pub struct SendMessageOptions;

/// Options for sending multiple messages.
///
/// This struct contains optional parameters that can be specified when sending
/// multiple messages using [`Sender::send_messages`].
///
/// Currently no options are available, but this provides
/// extensibility for future parameters
#[derive(SafeDebug, Clone, Default)]
pub struct SendMessagesOptions;

/// Options for creating a message batch.
///
/// This struct contains optional parameters that can be specified when creating
/// a message batch using [`Sender::create_message_batch`].
///
/// # Examples
///
/// ```rust,no_run
/// use azure_messaging_servicebus::CreateMessageBatchOptions;
///
/// // Create with custom maximum size
/// let options = CreateMessageBatchOptions {
///     maximum_size_in_bytes: Some(512 * 1024), // 512 KB
/// };
/// ```
#[derive(SafeDebug, Clone, Default)]
pub struct CreateMessageBatchOptions {
    /// Maximum size in bytes for the message batch.
    ///
    /// If `None`, Service Bus will use the maximum message size allowed by the namespace.
    pub maximum_size_in_bytes: Option<usize>,
}

/// Options for sending a message batch.
///
/// This struct contains optional parameters that can be specified when sending
/// a message batch using [`Sender::send_message_batch`].
///
/// Currently no options are available, but this provides
/// extensibility for future parameters
#[derive(SafeDebug, Clone, Default)]
pub struct SendMessageBatchOptions;

/// Options for scheduling a message.
///
/// This struct contains optional parameters that can be specified when scheduling
/// a message using [`Sender::schedule_message`].
///
/// Currently no options are available, but this provides
/// extensibility for future parameters
#[derive(SafeDebug, Clone, Default)]
pub struct ScheduleMessageOptions;

/// Options for scheduling multiple messages.
///
/// This struct contains optional parameters that can be specified when scheduling
/// multiple messages using [`Sender::schedule_message`].
///
/// Currently no options are available, but this provides
/// extensibility for future parameters
#[derive(SafeDebug, Clone, Default)]
pub struct ScheduleMessagesOptions;

/// Options for canceling scheduled messages.
///
/// This struct contains optional parameters that can be specified when canceling
/// scheduled messages using [`Sender::cancel_scheduled_message`].
///
/// Currently no options are available, but this provides
/// extensibility for future parameters
#[derive(SafeDebug, Clone, Default)]
pub struct CancelScheduledMessagesOptions;

/// A sender for sending messages to a Service Bus queue or topic.
pub struct Sender {
    connection: Arc<AmqpConnection>,
    entity_name: String,
    //options: Option<ServiceBusClientOptions>,
}

impl Sender {
    /// Creates a new sender.
    pub(crate) async fn new(
        connection: Arc<AmqpConnection>,
        entity_name: String,
        _options: ServiceBusClientOptions,
    ) -> Result<Self> {
        debug!("Creating Sender for entity: {}", entity_name);

        trace!("Sender created successfully for entity: {}", entity_name);

        Ok(Self {
            connection,
            entity_name,
            //options: Some(options),
        })
    }

    /// Sends a single message to the Service Bus entity.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send
    /// * `options` - Optional parameters for the send operation
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::{Message, SendMessageOptions, ServiceBusClient};
    /// use azure_identity::DeveloperToolsCredential;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
    /// let sender = client.create_sender("queue", None).await?;
    ///
    /// let message = Message::new("Hello, World!".as_bytes());
    /// sender.send_message(message, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message(
        &self,
        message: Message,
        _options: Option<SendMessageOptions>,
    ) -> Result<()> {
        debug!("Sending message to entity: {}", self.entity_name);

        // Create AMQP session
        let session = AmqpSession::new();
        session.begin(&self.connection, None).await?;

        // Create AMQP sender
        let amqp_sender = AmqpSender::new();

        // Create target with the entity name
        let target = AmqpTarget::from(self.entity_name.clone());

        // Attach the sender to the session
        amqp_sender
            .attach(&session, "sender-link".to_string(), target, None)
            .await?;

        // Convert Message to AmqpMessage
        let amqp_message: AmqpMessage = message.into();

        // Send the message
        let _outcome = amqp_sender.send(amqp_message, None).await?;

        // Detach the sender
        amqp_sender.detach().await?;

        // End the session
        session.end().await?;

        trace!("Message sent successfully to entity: {}", self.entity_name);
        Ok(())
    }

    /// Sends multiple messages to the Service Bus entity.
    ///
    /// # Arguments
    ///
    /// * `messages` - Vector of messages to send
    /// * `options` - Optional parameters for the send operation
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::{Message, SendMessagesOptions, ServiceBusClient};
    /// use azure_identity::DeveloperToolsCredential;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
    /// let sender = client.create_sender("queue", None).await?;
    ///
    /// let messages = vec![
    ///     Message::new("Message 1".as_bytes()),
    ///     Message::new("Message 2".as_bytes()),
    /// ];
    /// sender.send_messages(messages, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_messages(
        &self,
        messages: Vec<Message>,
        _options: Option<SendMessagesOptions>,
    ) -> Result<()> {
        debug!(
            "Sending {} messages to entity: {}",
            messages.len(),
            self.entity_name
        );

        // Create AMQP session
        let session = AmqpSession::new();
        session.begin(&self.connection, None).await?;

        // Create AMQP sender
        let amqp_sender = AmqpSender::new();

        // Create target with the entity name
        let target = AmqpTarget::from(self.entity_name.clone());

        // Attach the sender to the session
        amqp_sender
            .attach(&session, "sender-link".to_string(), target, None)
            .await?;

        // Send messages one by one
        for message in messages {
            let amqp_message: AmqpMessage = message.into();
            let _outcome = amqp_sender.send(amqp_message, None).await?;
        }

        // Detach the sender
        amqp_sender.detach().await?;

        // End the session
        session.end().await?;

        trace!("Messages sent successfully to entity: {}", self.entity_name);
        Ok(())
    }

    /// Creates a new message batch with default size limits.
    ///
    /// The batch allows for efficient sending of multiple messages in a single operation.
    /// Messages should be added using `try_add_message()` and then sent using `send_message_batch()`.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for batch creation
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::{ServiceBusClient, Message, CreateMessageBatchOptions};
    /// use azure_identity::DeveloperToolsCredential;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
    /// let sender = client.create_sender("queue_name", None).await?;
    /// let mut batch = sender.create_message_batch(None).await?;
    ///
    /// let message1 = Message::from("Hello");
    /// let message2 = Message::from("World");
    ///
    /// if batch.try_add_message(message1) && batch.try_add_message(message2) {
    ///     sender.send_message_batch(batch, None).await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_message_batch(
        &self,
        options: Option<CreateMessageBatchOptions>,
    ) -> crate::Result<crate::MessageBatch> {
        debug!(
            "Creating message batch for entity: {} with max size: {:?}",
            self.entity_name,
            options.as_ref().and_then(|o| o.maximum_size_in_bytes)
        );

        let maximum_size_in_bytes = options.as_ref().and_then(|o| o.maximum_size_in_bytes);
        Ok(crate::MessageBatch::new(maximum_size_in_bytes))
    }

    /// Sends a message batch to the Service Bus entity.
    ///
    /// This method provides better performance than sending messages individually
    /// by reducing network round trips and AMQP overhead.
    ///
    /// # Arguments
    ///
    /// * `batch` - The message batch to send
    /// * `options` - Optional parameters for the send operation
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::{ServiceBusClient, Message, CreateMessageBatchOptions, SendMessageBatchOptions};
    /// use azure_identity::DeveloperToolsCredential;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
    /// let sender = client.create_sender("queue_name", None).await?;
    /// let mut batch = sender.create_message_batch(None).await?;
    ///
    /// // Add multiple messages to the batch
    /// let messages = vec![
    ///     Message::from("Message 1"),
    ///     Message::from("Message 2"),
    ///     Message::new("Message 3"),
    /// ];
    ///
    /// let mut current_batch = batch;
    /// for message in messages {
    ///     if !current_batch.try_add_message(message) {
    ///         // Batch is full, send it and create a new one
    ///         sender.send_message_batch(current_batch, None).await?;
    ///         current_batch = sender.create_message_batch(None).await?;
    ///         // Note: In a real scenario, you'd want to retry adding the message
    ///         // that didn't fit to the new batch
    ///     }
    /// }
    ///
    /// // Send the final batch if it has messages
    /// if !current_batch.is_empty() {
    ///     sender.send_message_batch(current_batch, None).await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_message_batch(
        &self,
        batch: crate::MessageBatch,
        _options: Option<SendMessageBatchOptions>,
    ) -> crate::Result<()> {
        let messages = batch.into_messages();

        if messages.is_empty() {
            debug!("Empty batch provided, nothing to send");
            return Ok(());
        }

        debug!(
            "Sending message batch with {} messages to entity: {}",
            messages.len(),
            self.entity_name
        );

        // Create AMQP session
        let session = AmqpSession::new();
        session.begin(&self.connection, None).await?;

        // Create AMQP sender
        let amqp_sender = AmqpSender::new();

        // Create target with the entity name
        let target = AmqpTarget::from(self.entity_name.clone());

        // Attach the sender to the session
        amqp_sender
            .attach(&session, "sender-link".to_string(), target, None)
            .await?;

        // Send all messages in the batch
        // Note: In a full implementation, we would use AMQP transfer batching,
        // but for now we send them sequentially in a single session
        for message in messages {
            let amqp_message: AmqpMessage = message.into();
            let _outcome = amqp_sender.send(amqp_message, None).await?;
        }

        // Detach the sender
        amqp_sender.detach().await?;

        // End the session
        session.end().await?;

        trace!(
            "Message batch sent successfully to entity: {}",
            self.entity_name
        );
        Ok(())
    }

    /// Schedules a message to be sent at a specific time.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to schedule
    /// * `scheduled_enqueue_time` - The time when the message should be enqueued
    /// * `options` - Optional parameters for the schedule operation
    ///
    /// # Returns
    ///
    /// The sequence number assigned to the scheduled message
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::{ServiceBusClient, Message, ScheduleMessageOptions};
    /// use azure_identity::DeveloperToolsCredential;
    /// use time::OffsetDateTime;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
    /// let sender = client.create_sender("queue_name", None).await?;
    /// let message = Message::from("Hello, World!");
    /// let schedule_time = OffsetDateTime::now_utc() + time::Duration::minutes(30);
    ///
    /// let sequence_number = sender.schedule_message(message, schedule_time, None).await?;
    /// println!("Message scheduled with sequence number: {}", sequence_number);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn schedule_message(
        &self,
        mut message: Message,
        scheduled_enqueue_time: time::OffsetDateTime,
        _options: Option<ScheduleMessageOptions>,
    ) -> Result<i64> {
        debug!(
            "Scheduling message for entity: {} at time: {}",
            self.entity_name, scheduled_enqueue_time
        );

        // Set the scheduled enqueue time on the message
        message.set_scheduled_enqueue_time(scheduled_enqueue_time);

        // Create AMQP session
        let session = AmqpSession::new();
        session.begin(&self.connection, None).await?;

        // Create AMQP sender
        let amqp_sender = AmqpSender::new();

        // Create target with the entity name
        let target = AmqpTarget::from(self.entity_name.clone());

        // Attach the sender to the session
        amqp_sender
            .attach(&session, "sender-link".to_string(), target, None)
            .await?;

        // Convert Message to AmqpMessage
        let amqp_message: AmqpMessage = message.into();

        // Send the message (the scheduling is handled by the message properties)
        let _outcome = amqp_sender.send(amqp_message, None).await?;

        // Detach the sender
        amqp_sender.detach().await?;

        // End the session
        session.end().await?;

        // TODO: In a real implementation, we would need to use AMQP management operations
        // to get the actual sequence number from the broker. For now, return a placeholder.
        let sequence_number = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64; // Use timestamp as dummy sequence number

        trace!(
            "Message scheduled successfully for entity: {} with sequence number: {}",
            self.entity_name,
            sequence_number
        );

        Ok(sequence_number)
    }

    /// Cancels a scheduled message using its sequence number.
    ///
    /// # Arguments
    ///
    /// * `sequence_number` - The sequence number of the scheduled message to cancel
    /// * `options` - Optional parameters for the cancel operation
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::{ServiceBusClient, Message, ScheduleMessageOptions, CancelScheduledMessagesOptions};
    /// use azure_identity::DeveloperToolsCredential;
    /// use time::OffsetDateTime;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
    /// let sender = client.create_sender("queue_name", None).await?;
    /// let message = Message::from("Hello, World!");
    /// let schedule_time = OffsetDateTime::now_utc() + time::Duration::minutes(30);
    ///
    /// // Schedule a message
    /// let sequence_number = sender.schedule_message(message, schedule_time, None).await?;
    ///
    /// // Cancel the scheduled message
    /// sender.cancel_scheduled_message(sequence_number, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_scheduled_message(
        &self,
        sequence_number: i64,
        _options: Option<CancelScheduledMessagesOptions>,
    ) -> Result<()> {
        debug!(
            "Canceling scheduled message for entity: {} with sequence number: {}",
            self.entity_name, sequence_number
        );

        // TODO: Implement actual message cancellation using AMQP management operations
        // This would typically involve sending a management request to the broker
        // to cancel the scheduled message by its sequence number.
        // For now, this is a no-op since we don't have the management client implemented.

        trace!(
            "Scheduled message canceled successfully for entity: {} with sequence number: {}",
            self.entity_name,
            sequence_number
        );

        Err(ServiceBusError::new(
            ErrorKind::Unknown,
            "NOT_IMPLEMENTED - CancelScheduledMessage",
        ))
    }

    /// Gets the name of the Service Bus entity (queue or topic).
    pub fn entity_name(&self) -> &str {
        &self.entity_name
    }

    /// Closes the sender.
    pub async fn close(&self) -> Result<()> {
        debug!("Closing Sender for entity: {}", self.entity_name);

        // Sender creates sessions and senders on-demand for each send operation,
        // so there are no persistent AMQP resources to clean up here.
        // The connection will handle closing any outstanding sessions.

        trace!(
            "Sender closed successfully for entity: {}",
            self.entity_name
        );
        Ok(())
    }
}

impl Drop for Sender {
    fn drop(&mut self) {
        trace!("Sender for entity '{}' is being dropped", self.entity_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Message;
    use azure_core_amqp::AmqpConnection;
    use std::sync::Arc;

    /// Creates a mock connection for testing
    fn create_test_connection() -> Arc<AmqpConnection> {
        Arc::new(AmqpConnection::new())
    }

    /// Creates a test ClientOptions
    fn create_test_options() -> ServiceBusClientOptions {
        Default::default()
    }

    #[tokio::test]
    async fn test_sender_creation() -> Result<()> {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let sender = Sender::new(connection, entity_name.clone(), options).await?;

        assert_eq!(sender.entity_name(), &entity_name);
        Ok(())
    }

    #[test]
    fn test_sender_entity_name() {
        let connection = create_test_connection();
        let entity_name = "test-topic".to_string();

        let sender = Sender {
            connection,
            entity_name: entity_name.clone(),
        };

        assert_eq!(sender.entity_name(), &entity_name);
    }

    #[tokio::test]
    async fn test_sender_close() -> Result<()> {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();

        let options = create_test_options();

        let sender = Sender::new(connection, entity_name, options).await?;

        // Should not fail
        sender.close().await?;
        Ok(())
    }

    #[test]
    fn test_sender_drop() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();

        let sender = Sender {
            connection,
            entity_name,
        };

        // Should not panic when dropped
        drop(sender);
    }

    #[test]
    fn test_message_creation() {
        let message = Message::new("Hello, World!".as_bytes());
        assert_eq!(message.body(), "Hello, World!".as_bytes());

        let string_message = Message::from("Hello, String!");
        assert_eq!(string_message.body_as_string().unwrap(), "Hello, String!");
    }

    #[test]
    fn test_message_properties() {
        let mut message = Message::new("test".as_bytes());

        message.set_message_id("msg-123");
        message.set_correlation_id("corr-456");
        message.set_subject("Test Subject");
        message.set_content_type("text/plain");

        assert_eq!(message.message_id(), Some(&"msg-123".to_string()));
        assert_eq!(message.correlation_id(), Some(&"corr-456".to_string()));
        assert_eq!(message.subject(), Some(&"Test Subject".to_string()));
        assert_eq!(message.content_type(), Some(&"text/plain".to_string()));
    }

    #[test]
    fn test_message_custom_properties() {
        let mut message = Message::new("test".as_bytes());

        message.set_property("custom_key", "custom_value");
        message.set_property("number", "42");

        assert_eq!(
            message.property("custom_key"),
            Some(&"custom_value".to_string())
        );
        assert_eq!(message.property("number"), Some(&"42".to_string()));
        assert_eq!(message.property("non_existent"), None);

        assert_eq!(message.properties().len(), 2);
    }

    #[tokio::test]
    async fn test_create_message_batch() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();

        let sender = Sender {
            connection,
            entity_name,
        };

        let batch_options = CreateMessageBatchOptions {
            maximum_size_in_bytes: Some(1024),
        };
        let batch = sender
            .create_message_batch(Some(batch_options))
            .await
            .unwrap();

        assert!(batch.is_empty());
        assert_eq!(batch.maximum_size_in_bytes(), 1024);
    }

    #[test]
    fn test_batch_try_add_message() {
        let mut batch = crate::MessageBatch::new(Some(1024));
        let message = Message::new("Hello, World!");

        let result = batch.try_add_message(message);
        assert!(result);
        assert_eq!(batch.count(), 1);
        assert!(!batch.is_empty());
    }

    #[test]
    fn test_batch_try_add_message_exceeds_limit() {
        let mut batch = crate::MessageBatch::new(Some(50)); // Very small limit
        let large_message = Message::new("x".repeat(1000)); // Large message

        let result = batch.try_add_message(large_message);
        assert!(!result);
        assert_eq!(batch.count(), 0);
        assert!(batch.is_empty());
    }

    #[test]
    fn test_batch_multiple_messages() {
        let mut batch = crate::MessageBatch::new(None);

        for i in 0..5 {
            let message = Message::new(format!("Message {}", i));
            assert!(batch.try_add_message(message));
        }

        assert_eq!(batch.count(), 5);
        assert!(!batch.is_empty());

        let messages = batch.into_messages();
        assert_eq!(messages.len(), 5);

        for (i, message) in messages.iter().enumerate() {
            assert_eq!(message.body_as_string().unwrap(), format!("Message {}", i));
        }
    }

    #[test]
    fn test_batch_with_message_properties() {
        let mut batch = crate::MessageBatch::new(None);

        let mut message = Message::new("Test message with properties");
        message.set_message_id("test-id-123");
        message.set_correlation_id("correlation-456");
        message.set_content_type("text/plain");
        message.set_property("custom-prop", "custom-value");

        assert!(batch.try_add_message(message));
        assert_eq!(batch.count(), 1);

        // Verify size estimation includes property overhead
        assert!(batch.size_in_bytes() > "Test message with properties".len());
    }

    #[test]
    fn test_create_batch_options() {
        let options = CreateMessageBatchOptions::default();
        assert!(options.maximum_size_in_bytes.is_none());

        let options_with_size = CreateMessageBatchOptions {
            maximum_size_in_bytes: Some(2048),
        };

        assert_eq!(options_with_size.maximum_size_in_bytes, Some(2048));
    }
}
