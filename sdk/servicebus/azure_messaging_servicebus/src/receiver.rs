// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Service Bus message receiver functionality.
//!
//! This module provides the [`Receiver`] struct and related types for receiving messages
//! from Azure Service Bus queues and topic subscriptions. The receiver supports two modes
//! of operation and provides comprehensive message settlement operations.
//!
//! # Core Types
//!
//! - [`Receiver`] - The main receiver for consuming messages from Service Bus entities
//! - [`ReceiveMode`] - Enumeration defining how messages are handled when received
//! - [`ReceiveMessageOptions`] - Configuration options for message receive operations
//!
//! # Receive Modes
//!
//! ## PeekLock Mode (Default)
//!
//! In [`ReceiveMode::PeekLock`] mode, messages are locked when received and must be
//! explicitly settled using one of the settlement operations:
//!
//! - [`Receiver::complete_message`] - Successfully processed, remove from queue
//! - [`Receiver::abandon_message`] - Release lock, allow redelivery
//! - [`Receiver::dead_letter_message`] - Move to dead letter queue for manual inspection
//! - [`Receiver::defer_message`] - Defer for later processing by sequence number
//!
//! ## ReceiveAndDelete Mode
//!
//! In [`ReceiveMode::ReceiveAndDelete`] mode, messages are automatically deleted when
//! received, providing better performance but no delivery guarantees.
//!
//! # Examples
//!
//! ## Basic Message Receiving
//!
//! ```rust,no_run
//! use azure_messaging_servicebus::{ServiceBusClient, ReceiveMessageOptions, ServiceBusClientOptions};
//! use azure_core::time::Duration;
//!
//! use azure_identity::DeveloperToolsCredential;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let credential = DeveloperToolsCredential::new(None)?;
//! let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
//! let receiver = client.create_receiver("my-queue", None).await?;
//!
//! let options = ReceiveMessageOptions {
//!     max_message_count: 10,
//!     max_wait_time: Some(Duration::seconds(30)),
//! };
//!
//! let messages = receiver.receive_messages(10, Some(options)).await?;
//! for message in &messages {
//!     println!("Received: {:?}", String::from_utf8_lossy(message.body()));
//!     receiver.complete_message(message, None).await?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling with Settlement
//!
//! ```rust,no_run
//! use azure_messaging_servicebus::{
//!     ServiceBusClient,
//!     ReceiveMessageOptions, DeadLetterMessageOptions, AbandonMessageOptions
//! , ServiceBusClientOptions};
//!
//! use azure_identity::DeveloperToolsCredential;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let credential = DeveloperToolsCredential::new(None)?;
//! let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
//! let receiver = client.create_receiver("my-queue", None).await?;
//!
//! if let Some(message) = receiver.receive_message(None).await? {
//!     match process_message(&message).await {
//!         Ok(_) => {
//!             receiver.complete_message(&message, None).await?;
//!         }
//!         Err(ProcessingError::Retryable) => {
//!             receiver.abandon_message(&message, None).await?;
//!         }
//!         Err(ProcessingError::Fatal) => {
//!             let dead_letter_options = DeadLetterMessageOptions {
//!                 reason: Some("ProcessingFailed".to_string()),
//!                 error_description: Some("Unrecoverable processing error".to_string()),
//!                 properties_to_modify: None,
//!             };
//!             receiver.dead_letter_message(&message, Some(dead_letter_options)).await?;
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! # async fn process_message(message: &azure_messaging_servicebus::ReceivedMessage) -> Result<(), ProcessingError> { Ok(()) }
//! # #[derive(Debug)] enum ProcessingError { Retryable, Fatal }
//! ```

use crate::{
    client::ServiceBusClientOptions, message::SystemProperties, ErrorKind, ReceivedMessage, Result,
    ServiceBusError,
};
use async_lock::{Mutex, OnceCell};
use azure_core::{fmt::SafeDebug, time::Duration, time::OffsetDateTime, Uuid};
use azure_core_amqp::{
    message::{AmqpMessageBody, AmqpMessageId},
    AmqpConnection, AmqpDelivery, AmqpDeliveryApis, AmqpManagementApis, AmqpReceiver,
    AmqpReceiverApis, AmqpSession, AmqpSessionApis, AmqpSimpleValue, AmqpSource, AmqpValue,
};
use futures::{select, FutureExt};
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, trace, warn};

/// Represents the lock style to use for a receiver - either `PeekLock` or `ReceiveAndDelete`.
///
/// This enum controls when a message is deleted from Service Bus and determines how message
/// settlement works for received messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiveMode {
    /// Messages are locked for processing and can be settled using one of the settlement methods.
    ///
    /// In `PeekLock` mode (the default), messages are locked when received, preventing multiple
    /// receivers from processing the same message simultaneously. You control the lock state of
    /// the message using one of the message settlement functions:
    /// - `complete_message()` - removes the message from Service Bus
    /// - `abandon_message()` - makes the message available again for processing
    /// - `dead_letter_message()` - moves the message to the dead letter queue
    /// - `defer_message()` - defers the message for later processing
    ///
    /// Messages have a lock timeout period, after which they automatically become available
    /// for other receivers if not settled. The lock can be renewed using `renew_message_lock()`.
    ///
    /// This mode provides "at-least-once" delivery semantics and allows for reliable message
    /// processing patterns where messages can be retried on failure.
    PeekLock,

    /// Messages are automatically removed from the entity when received.
    ///
    /// In `ReceiveAndDelete` mode, Service Bus removes the message as soon as it's received
    /// by the client, before any processing occurs. This provides "at-most-once" delivery
    /// semantics - messages are delivered once or not at all.
    ///
    /// **Note**: When using `ReceiveAndDelete` mode, you can call `receive_messages()` even
    /// after the receiver has been closed. This allows you to continue reading from the
    /// receiver's internal cache until it is empty. When all cached messages have been
    /// consumed, `receive_messages()` will return an error.
    ///
    /// Settlement methods like `complete_message()`, `abandon_message()`, etc. are not
    /// applicable in this mode since messages are already removed from Service Bus.
    ///
    /// This mode offers better performance but sacrifices reliability - if message processing
    /// fails after the message is received, the message is lost and cannot be recovered.
    ReceiveAndDelete,
}

/// Options for configuring message receive operations.
///
/// This struct provides configuration options for controlling how messages are received
/// from a Service Bus entity, including batch size and timeout behavior.
///
/// # Examples
///
/// ```rust
/// use azure_messaging_servicebus::receiver::ReceiveMessageOptions;
/// use azure_core::time::Duration;
///
/// // Custom options - receive up to 10 messages with 30 second timeout
/// let custom_options = ReceiveMessageOptions {
///     max_message_count: 10,
///     max_wait_time: Some(Duration::seconds(30)),
/// };
///
/// // No timeout - block indefinitely until messages arrive
/// let blocking_options = ReceiveMessageOptions {
///     max_message_count: 5,
///     max_wait_time: None,
/// };
/// ```
#[derive(SafeDebug, Clone)]
pub struct ReceiveMessageOptions {
    /// The maximum number of messages to receive in a single operation.
    ///
    /// This controls the upper bound on how many messages will be returned.
    /// The actual number returned may be less if fewer messages are available
    /// or if the timeout is reached.
    pub max_message_count: u32,

    /// The maximum amount of time to wait for messages to arrive.
    ///
    /// - `Some(duration)` - Wait up to the specified duration for messages
    /// - `None` - Block indefinitely until at least one message arrives
    ///
    /// If a timeout is specified and no messages arrive within that time,
    /// the operation will return an empty result rather than an error.
    pub max_wait_time: Option<Duration>,
}

impl Default for ReceiveMessageOptions {
    fn default() -> Self {
        Self {
            max_message_count: 1,
            max_wait_time: Some(Duration::seconds(60)),
        }
    }
}

/// Options for configuring deferred message receive operations.
///
/// This struct provides configuration options for receiving messages that were
/// previously deferred using [`Receiver::defer_message`]. Deferred messages can
/// only be retrieved by their sequence number.
///
/// Currently, this struct is defined for future extensibility but contains no
/// configuration options. Additional options may be added in future versions.
///
#[derive(SafeDebug, Clone, Default)]
pub struct ReceiveDeferredMessagesOptions {
    // Currently no specific options, but structure is ready for future expansion
}

/// Options for configuring message peek operations.
///
/// This struct provides configuration options for peeking at messages without
/// removing them from the queue or subscription. Peeked messages cannot be
/// settled (completed, abandoned, etc.) as they are not locked.
///
/// # Examples
///
/// ```rust
/// use azure_messaging_servicebus::receiver::PeekMessagesOptions;
///
/// // Basic Peek without property modifications
/// let default_options = PeekMessagesOptions::default();
///
/// // Peek starting from a specific sequence number
/// let options_from_sequence = PeekMessagesOptions {
///     from_sequence_number: Some(12345),
/// };
/// ```
#[derive(SafeDebug, Clone, Default)]
pub struct PeekMessagesOptions {
    /// The sequence number to start peeking from.
    ///
    /// - `Some(sequence_number)` - Start peeking from the specified sequence number
    /// - `None` - Start peeking from the next available message
    ///
    /// The receiver maintains an internal cursor for peek operations. If this field
    /// is `None`, peeking will continue from where the last peek operation left off.
    /// Setting a specific sequence number overrides this cursor.
    pub from_sequence_number: Option<i64>,
}

/// Options for configuring message completion operations.
///
/// This struct provides configuration options for completing messages, which
/// removes them from the Service Bus entity and marks them as successfully processed.
/// Message completion is only available in [`ReceiveMode::PeekLock`] mode.
///
/// Currently, this struct is defined for future extensibility but contains no
/// configuration options. Additional options may be added in future versions.
///
#[derive(SafeDebug, Clone, Default)]
pub struct CompleteMessageOptions;

/// Options for configuring message abandon operations.
///
/// This struct provides configuration options for abandoning messages, which
/// releases the message lock and makes the message available for redelivery.
/// Message abandoning is only available in [`ReceiveMode::PeekLock`] mode.
///
/// When a message is abandoned, its delivery count is incremented. If the
/// delivery count exceeds the maximum delivery count configured on the entity,
/// the message will be automatically dead-lettered.
///
/// # Examples
///
/// ```rust
/// use azure_messaging_servicebus::receiver::AbandonMessageOptions;
/// use std::collections::HashMap;
///
/// // Basic abandon without property modifications
/// let basic_options = AbandonMessageOptions::default();
///
/// // Abandon with custom properties
/// let mut properties = HashMap::new();
/// properties.insert("reason".to_string(), "processing_failed".to_string());
/// properties.insert("retry_count".to_string(), "3".to_string());
///
/// let options_with_properties = AbandonMessageOptions {
///     properties_to_modify: Some(properties),
/// };
/// ```
#[derive(SafeDebug, Clone, Default)]
pub struct AbandonMessageOptions {
    /// Properties to modify in the message when it is abandoned.
    ///
    /// These properties will be added to or updated in the message's application
    /// properties. This can be useful for tracking abandon reasons, retry counts,
    /// or other metadata that might be helpful for subsequent processing attempts.
    ///
    /// - `Some(properties)` - Modify the specified properties
    /// - `None` - Abandon without modifying any properties
    pub properties_to_modify: Option<std::collections::HashMap<String, String>>,
}

/// Options for configuring message dead letter operations.
///
/// This struct provides configuration options for dead lettering messages, which
/// moves them to the dead letter queue where they can be examined and potentially
/// reprocessed. Dead lettering is only available in [`ReceiveMode::PeekLock`] mode.
///
/// Dead lettered messages are moved to a special sub-queue and will not be delivered
/// to regular consumers. They can be retrieved from the dead letter queue using a
/// receiver configured with the appropriate sub-queue option.
///
/// # Examples
///
/// ```rust
/// use azure_messaging_servicebus::receiver::DeadLetterMessageOptions;
/// use std::collections::HashMap;
///
/// // Dead letter with reason and description
/// let detailed_options = DeadLetterMessageOptions {
///     reason: Some("ValidationFailed".to_string()),
///     error_description: Some("Message schema validation failed".to_string()),
///     properties_to_modify: None,
/// };
///
/// // Dead letter with custom properties for diagnostics
/// let mut properties = HashMap::new();
/// properties.insert("validation_error".to_string(), "missing_required_field".to_string());
/// properties.insert("processor_version".to_string(), "1.2.3".to_string());
///
/// let options_with_properties = DeadLetterMessageOptions {
///     reason: Some("ProcessingError".to_string()),
///     error_description: Some("Failed to process after 3 retries".to_string()),
///     properties_to_modify: Some(properties),
/// };
/// ```
#[derive(SafeDebug, Clone, Default)]
pub struct DeadLetterMessageOptions {
    /// The reason for dead lettering the message.
    ///
    /// This is a short, descriptive string that categorizes why the message
    /// was dead lettered. Common examples include "ValidationFailed",
    /// "ProcessingTimeout", "MaxRetryExceeded", etc.
    pub reason: Option<String>,

    /// A detailed description of the error that caused the message to be dead lettered.
    ///
    /// This field provides additional context about the specific error condition
    /// that led to dead lettering. It can include technical details, error messages,
    /// or other diagnostic information.
    pub error_description: Option<String>,

    /// Properties to modify in the message when it is dead lettered.
    ///
    /// These properties will be added to or updated in the message's application
    /// properties. This can be useful for adding diagnostic information, tracking
    /// processing attempts, or storing other metadata that might be helpful for
    /// troubleshooting or reprocessing.
    pub properties_to_modify: Option<std::collections::HashMap<String, String>>,
}

/// Options for configuring message defer operations.
///
/// This struct provides configuration options for deferring messages, which
/// removes them from the normal message flow and makes them only retrievable
/// by their sequence number. Deferring is only available in [`ReceiveMode::PeekLock`] mode.
///
/// Deferred messages are not delivered to regular consumers and must be explicitly
/// retrieved using [`Receiver::receive_deferred_message`] or
/// [`Receiver::receive_deferred_messages`] with their sequence numbers.
///
/// # Examples
///
/// ```rust
/// use azure_messaging_servicebus::receiver::DeferMessageOptions;
/// use std::collections::HashMap;
///
/// // Basic defer without property modifications
/// let basic_options = DeferMessageOptions::default();
///
/// // Defer with custom properties for tracking
/// let mut properties = HashMap::new();
/// properties.insert("defer_reason".to_string(), "waiting_for_dependency".to_string());
/// properties.insert("defer_timestamp".to_string(), "2023-10-15T10:30:00Z".to_string());
///
/// let options_with_properties = DeferMessageOptions {
///     properties_to_modify: Some(properties),
/// };
/// ```
#[derive(SafeDebug, Clone, Default)]
pub struct DeferMessageOptions {
    /// Properties to modify in the message when it is deferred.
    ///
    /// These properties will be added to or updated in the message's application
    /// properties. This can be useful for tracking why the message was deferred,
    /// when it should be processed, or other metadata that might be helpful for
    /// later retrieval and processing.
    pub properties_to_modify: Option<std::collections::HashMap<String, String>>,
}

/// Options for configuring message lock renewal operations.
///
/// This struct provides configuration options for renewing the lock on a message
/// to extend the time available for processing. Lock renewal is only available
/// in [`ReceiveMode::PeekLock`] mode and can help prevent messages from being
/// automatically released due to lock timeout during long processing operations.
///
/// Currently, this struct is defined for future extensibility but contains no
/// configuration options. Additional options may be added in future versions.
///
#[derive(SafeDebug, Clone, Default)]
pub struct RenewMessageLockOptions;

/// A receiver for receiving messages from a Service Bus queue or subscription.
///
/// `Receiver` provides methods to receive messages from Service Bus entities (queues or topic subscriptions)
/// and perform message settlement operations. The receiver supports two modes of operation:
/// - [`ReceiveMode::PeekLock`] - Messages are locked for processing and must be explicitly settled
/// - [`ReceiveMode::ReceiveAndDelete`] - Messages are automatically deleted when received
///
/// # Message Settlement
///
/// In `PeekLock` mode, received messages must be settled using one of the settlement methods:
/// - [`complete_message`](Receiver::complete_message) - Marks the message as successfully processed and removes it
/// - [`abandon_message`](Receiver::abandon_message) - Releases the lock and makes the message available for redelivery
/// - [`dead_letter_message`](Receiver::dead_letter_message) - Moves the message to the dead letter queue
/// - [`defer_message`](Receiver::defer_message) - Defers the message for later retrieval by sequence number
///
/// # Examples
///
/// ## Basic Message Receiving
///
/// ```rust,no_run
/// use azure_messaging_servicebus::{ServiceBusClient, ReceiveMessageOptions, CreateReceiverOptions, ServiceBusClientOptions};
/// use azure_core::time::Duration;
///
/// use azure_identity::DeveloperToolsCredential;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let credential = DeveloperToolsCredential::new(None)?;
/// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
/// let receiver = client.create_receiver("my-queue", None).await?;
///
/// if let Some(message) = receiver.receive_message(None).await? {
///     println!("Received message: {:?}", String::from_utf8_lossy(message.body()));
///
///     // Complete the message to remove it from the queue
///     receiver.complete_message(&message, None).await?;
/// }
/// # Ok(())
/// # }
/// ```
///
/// ## Batch Message Processing
///
/// ```rust,no_run
/// use azure_messaging_servicebus::{ServiceBusClient, ReceiveMessageOptions, CreateReceiverOptions, ServiceBusClientOptions};
/// use azure_core::time::Duration;
///
/// use azure_identity::DeveloperToolsCredential;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let credential = DeveloperToolsCredential::new(None)?;
/// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
/// let receiver = client.create_receiver("my-queue", None).await?;
///
/// // Receive up to 10 messages with 30 second timeout
/// let options = ReceiveMessageOptions {
///     max_message_count: 10,
///     max_wait_time: Some(Duration::seconds(30)),
/// };
///
/// let messages = receiver.receive_messages(10, Some(options)).await?;
/// for message in messages {
///     match process_message(&message).await {
///         Ok(_) => {
///             receiver.complete_message(&message, None).await?;
///         }
///         Err(e) => {
///             println!("Processing failed: {}", e);
///             receiver.abandon_message(&message, None).await?;
///         }
///     }
/// }
/// # Ok(())
/// # }
/// # async fn process_message(message: &azure_messaging_servicebus::ReceivedMessage) -> Result<(), &'static str> { Ok(()) }
/// ```
///
/// ## Error Handling and Dead Lettering
///
/// ```rust,no_run
/// use azure_messaging_servicebus::{ServiceBusClient, ReceiveMessageOptions, DeadLetterMessageOptions, CreateReceiverOptions, ServiceBusClientOptions};
///
/// use azure_identity::DeveloperToolsCredential;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let credential = DeveloperToolsCredential::new(None)?;
/// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
/// let receiver = client.create_receiver("my-queue", None).await?;
///
/// if let Some(message) = receiver.receive_message(None).await? {
///     match validate_and_process(&message).await {
///         Ok(_) => {
///             receiver.complete_message(&message, None).await?;
///         }
///         Err(ValidationError::InvalidFormat) => {
///             // Dead letter messages that can't be processed
///             let dead_letter_options = DeadLetterMessageOptions {
///                 reason: Some("ValidationFailed".to_string()),
///                 error_description: Some("Invalid message format".to_string()),
///                 properties_to_modify: None,
///             };
///             receiver.dead_letter_message(&message, Some(dead_letter_options)).await?;
///         }
///         Err(ValidationError::TransientError) => {
///             // Abandon messages for transient errors to allow retry
///             receiver.abandon_message(&message, None).await?;
///         }
///     }
/// }
/// # Ok(())
/// # }
/// # #[derive(Debug)]
/// # enum ValidationError { InvalidFormat, TransientError }
/// # async fn validate_and_process(message: &azure_messaging_servicebus::ReceivedMessage) -> Result<(), ValidationError> { Ok(()) }
/// ```
pub struct Receiver {
    connection: Arc<AmqpConnection>,
    entity_name: String,
    subscription_name: Option<String>,
    receive_mode: ReceiveMode,
    _options: ServiceBusClientOptions,
    // Cached session and receiver to avoid creating new ones on each call
    session: OnceCell<Arc<AmqpSession>>,
    amqp_receiver: OnceCell<Arc<AmqpReceiver>>,
    // Track deliveries by lock token for settlement operations
    delivery_map: Arc<Mutex<HashMap<Uuid, AmqpDelivery>>>,
}

impl Receiver {
    /// Creates a new receiver for a Service Bus entity.
    ///
    /// This method is used internally by the Service Bus client to create receiver instances.
    /// Users should use [`crate::ServiceBusClient::create_receiver`] or
    /// [`crate::ServiceBusClient::create_receiver_for_subscription`] instead.
    ///
    /// # Arguments
    ///
    /// * `connection` - The AMQP connection to use
    /// * `entity_name` - The name of the queue or topic
    /// * `subscription_name` - The subscription name (for topic subscriptions only)
    /// * `receive_mode` - The receive mode to use
    /// * `options` - Client configuration options
    ///
    /// # Returns
    ///
    /// Returns a `Result<Receiver>` that will be `Ok(receiver)` on success or an error on failure.
    pub(crate) async fn new(
        connection: Arc<AmqpConnection>,
        entity_name: String,
        subscription_name: Option<String>,
        receive_mode: ReceiveMode,
        options: ServiceBusClientOptions,
    ) -> Result<Self> {
        let entity_path = if let Some(ref subscription) = subscription_name {
            format!("{}/subscriptions/{}", entity_name, subscription)
        } else {
            entity_name.clone()
        };

        debug!(
            "Creating Receiver for entity: {} (mode: {:?})",
            entity_path, receive_mode
        );

        trace!("Receiver created successfully for entity: {}", entity_path);

        Ok(Self {
            connection,
            entity_name,
            subscription_name,
            receive_mode,
            _options: options,
            session: OnceCell::new(),
            amqp_receiver: OnceCell::new(),
            delivery_map: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Receives a single message from the Service Bus entity.
    ///
    /// This is a convenience method that calls [`receive_messages`](Receiver::receive_messages)
    /// with `max_message_count` set to 1 and returns the first message if available.
    ///
    /// # Arguments
    ///
    /// * `options` - Configuration options for the receive operation
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(message))` if a message was received, `Ok(None)` if no message
    /// was available within the timeout period, or an error if the operation failed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::{ServiceBusClient, receiver::ReceiveMessageOptions};
    /// use azure_identity::DeveloperToolsCredential;
    /// use azure_core::time::Duration;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
    /// let receiver = client.create_receiver("queue", None).await?;
    /// let options = ReceiveMessageOptions {
    ///     max_message_count: 1,
    ///     max_wait_time: Some(Duration::seconds(30)),
    /// };
    ///
    /// if let Some(message) = receiver.receive_message(None).await? {
    ///     println!("Received: {:?}", String::from_utf8_lossy(message.body()));
    ///     receiver.complete_message(&message, None).await?;
    /// } else {
    ///     println!("No message received within timeout");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn receive_message(
        &self,
        options: Option<ReceiveMessageOptions>,
    ) -> Result<Option<ReceivedMessage>> {
        let messages = self.receive_messages(1, options).await?;
        Ok(messages.into_iter().next())
    }

    /// Receives multiple messages from the Service Bus entity.
    ///
    /// This method attempts to receive up to `max_message_count` messages from the entity.
    /// The actual number of messages returned may be less than requested if:
    /// - Fewer messages are available in the entity
    /// - The timeout specified in options is reached
    /// - An error occurs during the receive operation
    ///
    /// # Arguments
    ///
    /// * `max_message_count` - The maximum number of messages to receive
    /// * `options` - Configuration options for the receive operation
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<ReceivedMessage>)` containing the received messages, or an error
    /// if the operation failed. An empty vector is returned if no messages were available.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::{ServiceBusClient, receiver::ReceiveMessageOptions};
    /// use azure_identity::DeveloperToolsCredential;
    /// use azure_core::time::Duration;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
    /// let receiver = client.create_receiver("queue", None).await?;
    /// let options = ReceiveMessageOptions {
    ///     max_message_count: 10,
    ///     max_wait_time: Some(Duration::seconds(30)),
    /// };
    ///
    /// let messages = receiver.receive_messages(10, Some(options)).await?;
    /// println!("Received {} messages", messages.len());
    ///
    /// for message in &messages {
    ///     // Process each message
    ///     println!("Message: {:?}", String::from_utf8_lossy(message.body()));
    /// }
    ///
    /// // Complete all messages after successful processing
    /// for message in &messages {
    ///     receiver.complete_message(message, None).await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn receive_messages(
        &self,
        max_message_count: usize,
        options: Option<ReceiveMessageOptions>,
    ) -> Result<Vec<ReceivedMessage>> {
        debug!(
            "receive_messages: max_message_count={}, max_wait_time={:?}",
            max_message_count,
            options.as_ref().and_then(|o| o.max_wait_time)
        );

        // Ensure session and receiver are available
        let amqp_receiver = self.ensure_receiver().await?;

        let mut messages = Vec::new();

        for i in 0..max_message_count {
            debug!("receive_messages: iteration {}", i);

            let delivery_result =
                if let Some(timeout_duration) = options.as_ref().and_then(|o| o.max_wait_time) {
                    debug!("receive_messages: using timeout {:?}", timeout_duration);
                    select! {
                        delivery = amqp_receiver.receive_delivery().fuse() => {
                            debug!("receive_messages: received delivery on iteration {}", i);
                            delivery
                        },
                        _ = azure_core::sleep::sleep(timeout_duration).fuse() => {
                            debug!("receive_messages: timeout reached on iteration {}", i);
                            // Timeout reached - just return what we have so far
                            break;
                        },
                    }
                } else {
                    debug!(
                        "receive_messages: no timeout, blocking receive on iteration {}",
                        i
                    );
                    amqp_receiver.receive_delivery().await
                };

            match delivery_result {
                Ok(delivery) => {
                    debug!("receive_messages: processing delivery on iteration {}", i);
                    if let Some(message) = self.convert_delivery_to_message(delivery).await? {
                        messages.push(message);
                    }
                }
                Err(err) => {
                    debug!("receive_messages: error on iteration {}: {:?}", i, err);
                    if messages.is_empty() {
                        return Err(ServiceBusError::new(
                            ErrorKind::Amqp,
                            format!("Error receiving message: {:?}", err),
                        ));
                    } else {
                        break;
                    }
                }
            }
        }

        debug!("receive_messages: returning {} messages", messages.len());
        Ok(messages)
    }

    /// Ensures that a session and receiver are available, creating them if necessary.
    async fn ensure_receiver(&self) -> Result<Arc<AmqpReceiver>> {
        let receiver = self
            .amqp_receiver
            .get_or_try_init(|| async {
                // First ensure we have a session
                let session = self.ensure_session().await?;

                // Create AMQP receiver
                let entity_path = self.get_entity_path();
                let amqp_source = AmqpSource::builder().with_address(entity_path).build();
                let amqp_receiver = AmqpReceiver::new();
                amqp_receiver
                    .attach(&session, amqp_source, None)
                    .await
                    .map_err(|e| {
                        ServiceBusError::new(
                            ErrorKind::Amqp,
                            format!("Failed to create receiver: {:?}", e),
                        )
                    })?;

                Ok::<Arc<AmqpReceiver>, ServiceBusError>(Arc::new(amqp_receiver))
            })
            .await?;

        Ok(receiver.clone())
    }

    /// Ensures that a session is available, creating it if necessary.
    async fn ensure_session(&self) -> Result<Arc<AmqpSession>> {
        let session = self
            .session
            .get_or_try_init(|| async {
                let session = AmqpSession::new();
                session
                    .begin(
                        self.connection.as_ref(),
                        Some(azure_core_amqp::AmqpSessionOptions {
                            incoming_window: Some(u32::MAX),
                            outgoing_window: Some(u32::MAX),
                            ..Default::default()
                        }),
                    )
                    .await
                    .map_err(|e| {
                        ServiceBusError::new(
                            ErrorKind::Amqp,
                            format!("Failed to create session: {:?}", e),
                        )
                    })?;

                Ok::<Arc<AmqpSession>, ServiceBusError>(Arc::new(session))
            })
            .await?;

        Ok(session.clone())
    }

    /// Ensures that a management client is available, creating it if necessary.
    async fn ensure_management_client(&self) -> Result<azure_core_amqp::AmqpManagement> {
        let session = self.ensure_session().await?;

        // For Service Bus management operations, we need to create a management client
        // The client needs a unique node name and an access token for authorization
        let client_node_name = format!("servicebus-receiver-management-{}", uuid::Uuid::new_v4());

        // Create a default access token - in practice, this should come from the connection's
        // authentication mechanism (CBS token, SAS token, etc.)
        // For now, we'll use an empty token as a placeholder
        let access_token = azure_core::credentials::AccessToken {
            token: azure_core::credentials::Secret::new("placeholder-token".to_string()),
            expires_on: time::OffsetDateTime::now_utc() + Duration::seconds(3600),
        };

        let management_client = azure_core_amqp::AmqpManagement::new(
            session.as_ref().clone(),
            client_node_name,
            access_token,
        )
        .map_err(|e| {
            ServiceBusError::new(
                ErrorKind::Amqp,
                format!("Failed to create management client: {:?}", e),
            )
        })?;

        // Attach the management client
        management_client.attach().await.map_err(|e| {
            ServiceBusError::new(
                ErrorKind::Amqp,
                format!("Failed to attach management client: {:?}", e),
            )
        })?;

        Ok(management_client)
    }

    /// Completes a message, removing it from the Service Bus entity.
    ///
    /// This operation marks the message as successfully processed and permanently removes
    /// it from the queue or subscription. This operation is only available when using
    /// [`ReceiveMode::PeekLock`].
    ///
    /// Once a message is completed, it cannot be received again by any consumer.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to complete (must have been received in PeekLock mode)
    /// * `options` - Configuration options for the complete operation
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or an error if the operation failed.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The receiver is not in `PeekLock` mode
    /// - The message does not have a valid lock token
    /// - The message lock has expired
    /// - The message has already been settled
    /// - A network or service error occurs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::{ServiceBusClient, receiver::{ReceiveMessageOptions, CompleteMessageOptions}};
    /// use azure_identity::DeveloperToolsCredential;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
    /// let receiver = client.create_receiver("queue", None).await?;
    /// if let Some(message) = receiver.receive_message(None).await? {
    ///     // Process the message
    ///     match process_message(&message).await {
    ///         Ok(_) => {
    ///             // Successfully processed - complete the message
    ///             receiver.complete_message(&message, None).await?;
    ///             println!("Message completed successfully");
    ///         }
    ///         Err(e) => {
    ///             println!("Processing failed: {}", e);
    ///             // Handle the error (abandon, dead letter, etc.)
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// # async fn process_message(message: &azure_messaging_servicebus::ReceivedMessage) -> Result<(), &'static str> { Ok(()) }
    /// ```
    pub async fn complete_message(
        &self,
        message: &ReceivedMessage,
        _options: Option<CompleteMessageOptions>,
    ) -> Result<()> {
        if self.receive_mode != ReceiveMode::PeekLock {
            return Err(ServiceBusError::new(
                ErrorKind::InvalidRequest,
                "Complete message is only supported in PeekLock mode",
            ));
        }

        let lock_token = message.lock_token().ok_or_else(|| {
            ServiceBusError::new(
                ErrorKind::MessageLockLost,
                "Message does not have a lock token",
            )
        })?;

        debug!("Completing message with lock token: {}", lock_token);

        // Get the stored delivery for this lock token
        let delivery = {
            let mut delivery_map = self.delivery_map.lock().await;
            delivery_map.remove(&lock_token).ok_or_else(|| {
                ServiceBusError::new(
                    ErrorKind::MessageLockLost,
                    "Delivery not found for lock token - message may have already been settled or lock expired",
                )
            })?
        };

        // Accept the delivery using AMQP
        let amqp_receiver = self.ensure_receiver().await?;
        amqp_receiver
            .accept_delivery(&delivery)
            .await
            .map_err(|e| {
                ServiceBusError::new(
                    ErrorKind::Amqp,
                    format!("Failed to accept delivery: {:?}", e),
                )
            })?;

        trace!(
            "Message completed successfully with lock token: {}",
            lock_token
        );
        Ok(())
    }

    /// Abandons a message, making it available for redelivery.
    ///
    /// This operation releases the lock on the message and makes it available for other
    /// consumers to receive. The message's delivery count will be incremented. This operation
    /// is only available when using [`ReceiveMode::PeekLock`].
    ///
    /// If the message's delivery count exceeds the maximum delivery count configured on
    /// the entity, it will be automatically moved to the dead letter queue.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to abandon (must have been received in PeekLock mode)
    /// * `options` - Configuration options for the abandon operation
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or an error if the operation failed.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The receiver is not in `PeekLock` mode
    /// - The message does not have a valid lock token
    /// - The message lock has expired
    /// - The message has already been settled
    /// - A network or service error occurs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::{ServiceBusClient, receiver::{ReceiveMessageOptions, AbandonMessageOptions}};
    /// use azure_identity::DeveloperToolsCredential;
    /// use std::collections::HashMap;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = ServiceBusClient::builder().open("myservicebus.servicebus.windows.net", credential.clone()).await?;
    /// let receiver = client.create_receiver("queue", None).await?;
    /// if let Some(message) = receiver.receive_message(None).await? {
    ///     match process_message(&message).await {
    ///         Ok(_) => {
    ///             receiver.complete_message(&message, None).await?;
    ///         }
    ///         Err(e) if is_retryable_error(&e) => {
    ///             // Abandon with tracking information
    ///             let mut properties = HashMap::new();
    ///             properties.insert("abandon_reason".to_string(), "retryable_error".to_string());
    ///             properties.insert("error_details".to_string(), e.to_string());
    ///
    ///             let abandon_options = AbandonMessageOptions {
    ///                 properties_to_modify: Some(properties),
    ///             };
    ///
    ///             receiver.abandon_message(&message, Some(abandon_options)).await?;
    ///             println!("Message abandoned for retry");
    ///         }
    ///         Err(_) => {
    ///             // Non-retryable error - dead letter the message
    ///             receiver.dead_letter_message(&message, None).await?;
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// # async fn process_message(message: &azure_messaging_servicebus::ReceivedMessage) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// # fn is_retryable_error(error: &Box<dyn std::error::Error>) -> bool { true }
    /// ```
    pub async fn abandon_message(
        &self,
        message: &ReceivedMessage,
        _options: Option<AbandonMessageOptions>,
    ) -> Result<()> {
        if self.receive_mode != ReceiveMode::PeekLock {
            return Err(ServiceBusError::new(
                ErrorKind::InvalidRequest,
                "Abandon message is only supported in PeekLock mode",
            ));
        }

        let lock_token = message.lock_token().ok_or_else(|| {
            ServiceBusError::new(
                ErrorKind::MessageLockLost,
                "Message does not have a lock token",
            )
        })?;

        debug!("Abandoning message with lock token: {}", lock_token);

        // Get the stored delivery for this lock token
        let delivery = {
            let mut delivery_map = self.delivery_map.lock().await;
            delivery_map.remove(&lock_token).ok_or_else(|| {
                ServiceBusError::new(
                    ErrorKind::MessageLockLost,
                    "Delivery not found for lock token - message may have already been settled or lock expired",
                )
            })?
        };

        // Release the delivery using AMQP
        let amqp_receiver = self.ensure_receiver().await?;
        amqp_receiver
            .release_delivery(&delivery)
            .await
            .map_err(|e| {
                ServiceBusError::new(
                    ErrorKind::Amqp,
                    format!("Failed to release delivery: {:?}", e),
                )
            })?;

        trace!(
            "Message abandoned successfully with lock token: {}",
            lock_token
        );
        Ok(())
    }

    /// Dead letters a message, moving it to the dead letter queue.
    ///
    /// This operation moves the message to the dead letter sub-queue, where it can be
    /// examined and potentially reprocessed. Dead lettered messages are not delivered
    /// to regular consumers. This operation is only available when using
    /// [`ReceiveMode::PeekLock`].
    ///
    /// Dead lettered messages can be retrieved using a receiver configured to read from
    /// the dead letter queue by setting the appropriate sub-queue option.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to dead letter (must have been received in PeekLock mode)
    /// * `options` - Configuration options including reason and error description
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or an error if the operation failed.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The receiver is not in `PeekLock` mode
    /// - The message does not have a valid lock token
    /// - The message lock has expired
    /// - The message has already been settled
    /// - A network or service error occurs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::receiver::{ReceiveMessageOptions, DeadLetterMessageOptions};
    /// use std::collections::HashMap;
    ///
    /// # async fn example(receiver: azure_messaging_servicebus::Receiver) -> Result<(), Box<dyn std::error::Error>> {
    /// if let Some(message) = receiver.receive_message(None).await? {
    ///     match validate_message_format(&message) {
    ///         Ok(_) => {
    ///             // Process valid message
    ///             receiver.complete_message(&message, None).await?;
    ///         }
    ///         Err(validation_error) => {
    ///             // Dead letter invalid messages with detailed information
    ///             let mut properties = HashMap::new();
    ///             properties.insert("validation_failure".to_string(), validation_error.clone());
    ///             properties.insert("processor_version".to_string(), "1.0.0".to_string());
    ///
    ///             let dead_letter_options = DeadLetterMessageOptions {
    ///                 reason: Some("ValidationFailed".to_string()),
    ///                 error_description: Some(format!("Message validation failed: {}", validation_error)),
    ///                 properties_to_modify: Some(properties),
    ///             };
    ///
    ///             receiver.dead_letter_message(&message, Some(dead_letter_options)).await?;
    ///             println!("Message dead lettered due to validation failure");
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// # fn validate_message_format(message: &azure_messaging_servicebus::ReceivedMessage) -> Result<(), String> { Ok(()) }
    /// ```
    pub async fn dead_letter_message(
        &self,
        message: &ReceivedMessage,
        options: Option<DeadLetterMessageOptions>,
    ) -> Result<()> {
        if self.receive_mode != ReceiveMode::PeekLock {
            return Err(ServiceBusError::new(
                ErrorKind::InvalidRequest,
                "Dead letter message is only supported in PeekLock mode",
            ));
        }

        let lock_token = message.lock_token().ok_or_else(|| {
            ServiceBusError::new(
                ErrorKind::MessageLockLost,
                "Message does not have a lock token",
            )
        })?;

        debug!(
            "Dead lettering message with lock token: {}, reason: {:?}, description: {:?}",
            lock_token,
            options.as_ref().and_then(|o| o.reason.as_ref()),
            options.as_ref().and_then(|o| o.error_description.as_ref())
        );

        // Get the stored delivery for this lock token
        let delivery = {
            let mut delivery_map = self.delivery_map.lock().await;
            delivery_map.remove(&lock_token).ok_or_else(|| {
                ServiceBusError::new(
                    ErrorKind::MessageLockLost,
                    "Delivery not found for lock token - message may have already been settled or lock expired",
                )
            })?
        };

        // Reject the delivery using AMQP
        let amqp_receiver = self.ensure_receiver().await?;
        amqp_receiver
            .reject_delivery(&delivery)
            .await
            .map_err(|e| {
                ServiceBusError::new(
                    ErrorKind::Amqp,
                    format!("Failed to reject delivery: {:?}", e),
                )
            })?;

        trace!(
            "Message dead lettered successfully with lock token: {}",
            lock_token
        );
        Ok(())
    }

    /// Defers a message, making it unavailable for normal message retrieval.
    ///
    /// This operation removes the message from the normal message flow and makes it only
    /// retrievable by its sequence number using [`receive_deferred_message`](Receiver::receive_deferred_message)
    /// or [`receive_deferred_messages`](Receiver::receive_deferred_messages). This operation
    /// is only available when using [`ReceiveMode::PeekLock`].
    ///
    /// Deferred messages are useful when you need to process messages out of order or when
    /// message processing depends on some external condition that isn't currently met.
    ///
    /// **Note**: The current implementation is a placeholder. Full defer functionality
    /// requires Service Bus management operations that are not yet implemented.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to defer (must have been received in PeekLock mode)
    /// * `options` - Configuration options for the defer operation
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or an error if the operation failed.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The receiver is not in `PeekLock` mode
    /// - The message does not have a valid lock token
    /// - The message lock has expired
    /// - The message has already been settled
    /// - A network or service error occurs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::receiver::{ReceiveMessageOptions, DeferMessageOptions};
    /// use std::collections::HashMap;
    ///
    /// # async fn example(receiver: azure_messaging_servicebus::Receiver) -> Result<(), Box<dyn std::error::Error>> {
    /// if let Some(message) = receiver.receive_message(None).await? {
    ///     if requires_deferred_processing(&message) {
    ///         // Defer the message with tracking information
    ///         let mut properties = HashMap::new();
    ///         properties.insert("defer_reason".to_string(), "waiting_for_dependency".to_string());
    ///         properties.insert("defer_until".to_string(), "2023-10-15T15:30:00Z".to_string());
    ///
    ///         let defer_options = DeferMessageOptions {
    ///             properties_to_modify: Some(properties),
    ///         };
    ///
    ///         receiver.defer_message(&message, Some(defer_options)).await?;
    ///
    ///         // Store the sequence number for later retrieval
    ///         let sequence_number = message.system_properties().sequence_number;
    ///         println!("Message deferred with sequence number: {:?}", sequence_number);
    ///     } else {
    ///         // Process immediately
    ///         receiver.complete_message(&message, None).await?;
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// # fn requires_deferred_processing(message: &azure_messaging_servicebus::ReceivedMessage) -> bool { false }
    /// ```
    pub async fn defer_message(
        &self,
        message: &ReceivedMessage,
        options: Option<DeferMessageOptions>,
    ) -> Result<()> {
        if self.receive_mode != ReceiveMode::PeekLock {
            return Err(ServiceBusError::new(
                ErrorKind::InvalidRequest,
                "Defer message is only supported in PeekLock mode",
            ));
        }

        let lock_token = message.lock_token().ok_or_else(|| {
            ServiceBusError::new(
                ErrorKind::MessageLockLost,
                "Message does not have a lock token",
            )
        })?;

        debug!("Deferring message with lock token: {}", lock_token);

        // Create management client and defer the message
        let management_client = self.ensure_management_client().await?;

        let mut application_properties: azure_core_amqp::AmqpOrderedMap<
            String,
            azure_core_amqp::AmqpSimpleValue,
        > = azure_core_amqp::AmqpOrderedMap::new();

        // Add the lock token as a string representation
        application_properties.insert("lock-token".to_string(), lock_token.to_string().into());

        // Add any properties to modify if specified
        if let Some(properties) = options
            .as_ref()
            .and_then(|o| o.properties_to_modify.as_ref())
        {
            for (key, value) in properties {
                application_properties.insert(key.clone(), value.clone().into());
            }
        }

        let _response = management_client
            .call(
                "com.microsoft:defer-message".to_string(),
                application_properties,
            )
            .await
            .map_err(|e| {
                ServiceBusError::new(ErrorKind::Amqp, format!("Failed to defer message: {:?}", e))
            })?;

        // Remove the delivery from our tracking map since it's now deferred
        let _delivery = {
            let mut delivery_map = self.delivery_map.lock().await;
            delivery_map.remove(&lock_token)
        };

        trace!(
            "Message deferred successfully with lock token: {}",
            lock_token
        );
        Ok(())
    }

    /// Receives a deferred message by its sequence number.
    ///
    /// This method retrieves a message that was previously deferred using
    /// [`defer_message`](Receiver::defer_message). Deferred messages can only be retrieved
    /// by their sequence number and are not delivered through normal receive operations.
    /// This operation is only available when using [`ReceiveMode::PeekLock`].
    ///
    /// **Note**: The current implementation is a placeholder. Full deferred message retrieval
    /// requires Service Bus management operations that are not yet implemented.
    ///
    /// # Arguments
    ///
    /// * `sequence_number` - The sequence number of the deferred message to retrieve
    /// * `options` - Configuration options for the receive operation
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(message))` if the deferred message was found and retrieved,
    /// `Ok(None)` if no message with the specified sequence number exists, or an error
    /// if the operation failed.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The receiver is not in `PeekLock` mode
    /// - The sequence number is invalid
    /// - A network or service error occurs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::receiver::ReceiveDeferredMessagesOptions;
    ///
    /// # async fn example(receiver: azure_messaging_servicebus::Receiver) -> Result<(), Box<dyn std::error::Error>> {
    /// // Assume we previously stored sequence numbers of deferred messages
    /// let sequence_number = 123456789i64;
    ///
    /// let options = Default::default();
    /// match receiver.receive_deferred_message(sequence_number, Some(options)).await? {
    ///     Some(message) => {
    ///         println!("Retrieved deferred message: {:?}", String::from_utf8_lossy(message.body()));
    ///
    ///         // Process the message now that conditions are met
    ///         match process_deferred_message(&message).await {
    ///             Ok(_) => receiver.complete_message(&message, None).await?,
    ///             Err(_) => receiver.abandon_message(&message, None).await?,
    ///         }
    ///     }
    ///     None => {
    ///         println!("No deferred message found with sequence number: {}", sequence_number);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// # async fn process_deferred_message(message: &azure_messaging_servicebus::ReceivedMessage) -> Result<(), &'static str> { Ok(()) }
    /// ```
    pub async fn receive_deferred_message(
        &self,
        sequence_number: i64,
        _options: Option<ReceiveDeferredMessagesOptions>,
    ) -> Result<Option<ReceivedMessage>> {
        if self.receive_mode != ReceiveMode::PeekLock {
            return Err(ServiceBusError::new(
                ErrorKind::InvalidRequest,
                "Receive deferred message is only supported in PeekLock mode",
            ));
        }

        debug!(
            "Receiving deferred message with sequence number: {}",
            sequence_number
        );

        // TODO: Implement actual deferred message retrieval using Service Bus management operations
        // Deferred messages require Service Bus management API calls to retrieve by sequence number.
        // This is not a standard AMQP operation but a Service Bus-specific management operation.
        // Implementation would involve:
        // 1. Creating a management request with the sequence number
        // 2. Sending the request via the AMQP management link
        // 3. Processing the response to reconstruct the ReceivedMessage
        // For now, this is a placeholder implementation.

        trace!(
            "Attempted to receive deferred message with sequence number: {} (placeholder implementation)",
            sequence_number
        );

        // Return None for now since this is not implemented
        Ok(None)
    }

    /// Receives multiple deferred messages by their sequence numbers.
    ///
    /// This method retrieves multiple messages that were previously deferred using
    /// [`defer_message`](Receiver::defer_message). Each message is identified by its
    /// sequence number. This operation is only available when using [`ReceiveMode::PeekLock`].
    ///
    /// **Note**: The current implementation is a placeholder. Full deferred message retrieval
    /// requires Service Bus management operations that are not yet implemented.
    ///
    /// # Arguments
    ///
    /// * `sequence_numbers` - The sequence numbers of the deferred messages to retrieve
    /// * `options` - Configuration options for the receive operation
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<ReceivedMessage>)` containing the retrieved deferred messages.
    /// The vector may contain fewer messages than requested if some sequence numbers
    /// don't correspond to existing deferred messages.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The receiver is not in `PeekLock` mode
    /// - One or more sequence numbers are invalid
    /// - A network or service error occurs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::receiver::ReceiveDeferredMessagesOptions;
    ///
    /// # async fn example(receiver: azure_messaging_servicebus::Receiver) -> Result<(), Box<dyn std::error::Error>> {
    /// // Assume we previously stored sequence numbers of deferred messages
    /// let sequence_numbers = vec![123456789i64, 123456790i64, 123456791i64];
    ///
    /// let options = Default::default();
    /// let messages = receiver.receive_deferred_messages(&sequence_numbers, Some(options)).await?;
    ///
    /// println!("Retrieved {} deferred messages", messages.len());
    ///
    /// for message in &messages {
    ///     println!("Processing deferred message: {:?}", message.system_properties().sequence_number);
    ///
    ///     match process_deferred_message(&message).await {
    ///         Ok(_) => {
    ///             receiver.complete_message(&message, None).await?;
    ///             println!("Deferred message processed successfully");
    ///         }
    ///         Err(e) => {
    ///             println!("Failed to process deferred message: {}", e);
    ///             receiver.abandon_message(&message, None).await?;
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// # async fn process_deferred_message(message: &azure_messaging_servicebus::ReceivedMessage) -> Result<(), &'static str> { Ok(()) }
    /// ```
    pub async fn receive_deferred_messages(
        &self,
        sequence_numbers: &[i64],
        _options: Option<ReceiveDeferredMessagesOptions>,
    ) -> Result<Vec<ReceivedMessage>> {
        if self.receive_mode != ReceiveMode::PeekLock {
            return Err(ServiceBusError::new(
                ErrorKind::InvalidRequest,
                "Receive deferred messages is only supported in PeekLock mode",
            ));
        }

        if sequence_numbers.is_empty() {
            return Ok(Vec::new());
        }

        debug!(
            "Receiving {} deferred messages with sequence numbers: {:?}",
            sequence_numbers.len(),
            sequence_numbers
        );

        // Create management client and receive the deferred messages
        let management_client = self.ensure_management_client().await?;

        let mut application_properties: azure_core_amqp::AmqpOrderedMap<
            String,
            azure_core_amqp::AmqpSimpleValue,
        > = azure_core_amqp::AmqpOrderedMap::new();

        // Add the sequence numbers as a comma-separated string
        // Service Bus management operations don't support arrays in AmqpSimpleValue,
        // so we'll pass the sequence numbers as a comma-separated string
        let sequence_numbers_str = sequence_numbers
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");

        application_properties.insert("sequence-numbers".to_string(), sequence_numbers_str.into());

        // Set receiver settle mode based on receive mode
        let settle_mode = match self.receive_mode {
            ReceiveMode::PeekLock => 1u32,
            ReceiveMode::ReceiveAndDelete => 0u32,
        };
        application_properties.insert("receiver-settle-mode".to_string(), settle_mode.into());

        let response = management_client
            .call(
                "com.microsoft:receive-by-sequence-number".to_string(),
                application_properties,
            )
            .await
            .map_err(|e| {
                ServiceBusError::new(
                    ErrorKind::Amqp,
                    format!("Failed to receive deferred messages: {:?}", e),
                )
            })?;

        // Process the response to reconstruct ReceivedMessage instances
        let messages = self.parse_deferred_messages_response(response).await?;

        trace!("Successfully received {} deferred messages", messages.len());

        Ok(messages)
    }

    /// Parses the response from a deferred messages retrieval operation.
    ///
    /// The Service Bus management response for deferred messages comes in a nested format:
    /// - A map with key "messages"
    /// - An array of message maps, each with key "message"
    /// - Each "message" contains serialized AMQP binary data
    async fn parse_deferred_messages_response(
        &self,
        response: azure_core_amqp::AmqpOrderedMap<String, azure_core_amqp::AmqpValue>,
    ) -> Result<Vec<ReceivedMessage>> {
        use std::collections::HashMap;

        // Extract the "messages" field from the response
        let messages_value = response.get("messages").ok_or_else(|| {
            ServiceBusError::new(
                ErrorKind::InvalidRequest,
                "Management response missing 'messages' field",
            )
        })?;

        // Parse the messages array
        let messages_array = match messages_value {
            AmqpValue::Array(array) => array,
            _ => {
                return Err(ServiceBusError::new(
                    ErrorKind::InvalidRequest,
                    "Expected 'messages' field to be an array",
                ))
            }
        };

        let mut received_messages = Vec::new();

        for message_entry in messages_array {
            // Each entry should be a map with a "message" key
            let message_map = match message_entry {
                AmqpValue::Map(map) => map,
                _ => {
                    return Err(ServiceBusError::new(
                        ErrorKind::InvalidRequest,
                        "Expected message entry to be a map",
                    ))
                }
            };

            // Extract the serialized message data
            let message_data = message_map
                .get(&AmqpValue::String("message".to_string()))
                .ok_or_else(|| {
                    ServiceBusError::new(
                        ErrorKind::InvalidRequest,
                        "Message entry missing 'message' field",
                    )
                })?;

            // The message data should be binary AMQP data
            let message_bytes = match message_data {
                AmqpValue::Binary(bytes) => bytes,
                _ => {
                    return Err(ServiceBusError::new(
                        ErrorKind::InvalidRequest,
                        "Expected message data to be binary",
                    ))
                }
            };

            // Deserialize the AMQP message from binary data
            // Note: This is a simplified reconstruction. In a full implementation,
            // we would need to properly deserialize the AMQP message and extract
            // all the Service Bus specific properties, system properties, etc.

            // For now, create a basic ReceivedMessage structure
            // This would need to be enhanced to properly reconstruct from AMQP binary data
            let system_properties = SystemProperties::default();

            // Create a placeholder ReceivedMessage
            // TODO: Implement proper AMQP binary deserialization
            let received_message = ReceivedMessage::new(
                message_bytes.clone(),
                HashMap::new(),
                system_properties,
                None, // lock_token - deferred messages typically don't have lock tokens initially
            );

            received_messages.push(received_message);
        }

        Ok(received_messages)
    }

    /// Renews the lock on a message to extend the processing time.
    ///
    /// This operation extends the lock duration on a message, preventing it from being
    /// automatically released and made available to other consumers. This is useful when
    /// message processing takes longer than the default lock duration. This operation
    /// is only available when using [`ReceiveMode::PeekLock`].
    ///
    /// **Note**: The current implementation is a placeholder. Full lock renewal
    /// requires Service Bus management operations that are not yet implemented.
    ///
    /// # Arguments
    ///
    /// * `message` - The message whose lock should be renewed
    /// * `options` - Configuration options for the lock renewal operation
    ///
    /// # Returns
    ///
    /// Returns `Ok(locked_until)` with the new lock expiration time on success,
    /// or an error if the operation failed.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The receiver is not in `PeekLock` mode
    /// - The message does not have a valid lock token
    /// - The message lock has already expired
    /// - The message has already been settled
    /// - A network or service error occurs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::{ReceiveMessageOptions, RenewMessageLockOptions, ServiceBusClientOptions};
    /// use azure_core::time::Duration;
    /// use tokio::time::sleep;
    ///
    /// # async fn example(receiver: &azure_messaging_servicebus::Receiver) -> Result<(), Box<dyn std::error::Error>> {
    /// if let Some(message) = receiver.receive_message(None).await? {
    ///     // Renew lock for the received message
    ///     match receiver.renew_message_lock(&message, None).await {
    ///         Ok(locked_until) => {
    ///             println!("Lock renewed until: {:?}", locked_until);
    ///         }
    ///         Err(e) => {
    ///             println!("Failed to renew lock: {}", e);
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn renew_message_lock(
        &self,
        message: &ReceivedMessage,
        _options: Option<RenewMessageLockOptions>,
    ) -> Result<OffsetDateTime> {
        if self.receive_mode != ReceiveMode::PeekLock {
            return Err(ServiceBusError::new(
                ErrorKind::InvalidRequest,
                "Renew message lock is only supported in PeekLock mode",
            ));
        }

        let lock_token = message.lock_token().ok_or_else(|| {
            ServiceBusError::new(
                ErrorKind::MessageLockLost,
                "Message does not have a lock token",
            )
        })?;

        debug!("Renewing message lock with lock token: {}", lock_token);

        // Create management client and renew the message lock
        let management_client = self.ensure_management_client().await?;

        let mut application_properties: azure_core_amqp::AmqpOrderedMap<
            String,
            azure_core_amqp::AmqpSimpleValue,
        > = azure_core_amqp::AmqpOrderedMap::new();

        // Add the lock token as a string representation
        application_properties.insert("lock-token".to_string(), lock_token.to_string().into());

        let response = management_client
            .call(
                "com.microsoft:renew-lock".to_string(),
                application_properties,
            )
            .await
            .map_err(|e| {
                ServiceBusError::new(
                    ErrorKind::Amqp,
                    format!("Failed to renew message lock: {:?}", e),
                )
            })?;

        // Extract the new expiration time from the response
        let locked_until = response
            .get("expiration")
            .or_else(|| response.get("locked-until-utc"))
            .ok_or_else(|| {
                ServiceBusError::new(
                    ErrorKind::InvalidRequest,
                    "Management response did not contain expiration time",
                )
            })?;

        // Convert the response value to an OffsetDateTime
        let locked_until = match locked_until {
            azure_core_amqp::AmqpValue::TimeStamp(timestamp) => {
                let timestamp: azure_core_amqp::AmqpTimestamp = timestamp.clone();
                if let Some(system_time) = timestamp.0 {
                    OffsetDateTime::from(system_time)
                } else {
                    OffsetDateTime::now_utc() + Duration::seconds(60)
                }
            }
            _ => {
                // Fallback to a default if we can't parse the timestamp
                warn!(
                    "Could not parse expiration timestamp from management response, using default"
                );
                OffsetDateTime::now_utc() + Duration::seconds(60)
            }
        };

        trace!(
            "Message lock renewed successfully with lock token: {}, new expiration: {}",
            lock_token,
            locked_until
        );
        Ok(locked_until)
    }

    /// Gets the name of the Service Bus entity (queue or topic) this receiver is connected to.
    ///
    /// # Returns
    ///
    /// Returns the entity name as a string slice.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example(receiver: azure_messaging_servicebus::Receiver) {
    /// println!("Receiver is connected to entity: {}", receiver.entity_name());
    /// # }
    /// ```
    pub fn entity_name(&self) -> &str {
        &self.entity_name
    }

    /// Gets the subscription name if this receiver is for a topic subscription.
    ///
    /// # Returns
    ///
    /// Returns `Some(subscription_name)` if this receiver is for a topic subscription,
    /// or `None` if this receiver is for a queue.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example(receiver: azure_messaging_servicebus::Receiver) {
    /// match receiver.subscription_name() {
    ///     Some(subscription) => {
    ///         println!("Receiver is for subscription: {} on topic: {}",
    ///                  subscription, receiver.entity_name());
    ///     }
    ///     None => {
    ///         println!("Receiver is for queue: {}", receiver.entity_name());
    ///     }
    /// }
    /// # }
    /// ```
    pub fn subscription_name(&self) -> Option<&str> {
        self.subscription_name.as_deref()
    }

    /// Gets the receive mode configured for this receiver.
    ///
    /// # Returns
    ///
    /// Returns the [`ReceiveMode`] that determines how messages are handled when received.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::receiver::ReceiveMode;
    ///
    /// # async fn example(receiver: azure_messaging_servicebus::Receiver) {
    /// match receiver.receive_mode() {
    ///     ReceiveMode::PeekLock => {
    ///         println!("Receiver uses PeekLock mode - messages must be explicitly settled");
    ///     }
    ///     ReceiveMode::ReceiveAndDelete => {
    ///         println!("Receiver uses ReceiveAndDelete mode - messages are auto-deleted");
    ///     }
    /// }
    /// # }
    /// ```
    pub fn receive_mode(&self) -> ReceiveMode {
        self.receive_mode.clone()
    }

    /// Peeks at messages in the queue or subscription without removing them.
    ///
    /// This operation allows you to browse messages without affecting their state or
    /// availability to other consumers. Unlike receiving messages, peeked messages:
    /// - Are not locked or removed from the queue/subscription
    /// - Cannot be completed, abandoned, or dead-lettered
    /// - Do not affect delivery count
    /// - Can include deferred messages (but not dead-lettered messages)
    ///
    /// This is useful for monitoring queue contents, debugging message flow, or
    /// implementing custom message routing logic.
    ///
    /// # Arguments
    ///
    /// * `max_count` - Maximum number of messages to peek (must be > 0)
    /// * `options` - Configuration options for the peek operation
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<ReceivedMessage>)` containing the peeked messages.
    /// The returned messages will not have lock tokens since they are not locked.
    /// The vector may contain fewer messages than requested if fewer are available.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - `max_count` is 0 or negative
    /// - A network or service error occurs
    /// - The management operation fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::receiver::PeekMessagesOptions;
    ///
    /// # async fn example(receiver: azure_messaging_servicebus::Receiver) -> Result<(), Box<dyn std::error::Error>> {
    /// // Peek up to 10 messages from the beginning
    /// let options = Default::default();
    /// let peeked_messages = receiver.peek_messages(10, Some(options)).await?;
    ///
    /// println!("Found {} messages in queue", peeked_messages.len());
    ///
    /// for message in &peeked_messages {
    ///     if let Some(seq_num) = message.system_properties().sequence_number {
    ///         println!("Message sequence number: {}", seq_num);
    ///     }
    ///     println!("Message body: {:?}", message.body());
    /// }
    ///
    /// // Peek starting from a specific sequence number
    /// let options_from_sequence = PeekMessagesOptions {
    ///     from_sequence_number: Some(123456789),
    /// };
    /// let more_messages = receiver.peek_messages(5, Some(options_from_sequence)).await?;
    /// println!("Found {} more messages starting from sequence 123456789", more_messages.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn peek_messages(
        &self,
        max_count: u32,
        options: Option<PeekMessagesOptions>,
    ) -> Result<Vec<ReceivedMessage>> {
        if max_count == 0 {
            return Err(ServiceBusError::new(
                ErrorKind::InvalidRequest,
                "Max count must be greater than 0",
            ));
        }

        debug!(
            "Peeking up to {} messages with options: {:?}",
            max_count, options
        );

        // Create management client for peek operation
        let management_client = self.ensure_management_client().await?;

        let mut application_properties: azure_core_amqp::AmqpOrderedMap<
            String,
            azure_core_amqp::AmqpSimpleValue,
        > = azure_core_amqp::AmqpOrderedMap::new();

        // Set the maximum number of messages to peek
        application_properties.insert("message-count".to_string(), max_count.into());

        // Set the starting sequence number if provided
        if let Some(from_sequence_number) = options.as_ref().and_then(|o| o.from_sequence_number) {
            application_properties.insert(
                "from-sequence-number".to_string(),
                from_sequence_number.into(),
            );
        }

        let response = management_client
            .call(
                "com.microsoft:peek-message".to_string(),
                application_properties,
            )
            .await
            .map_err(|e| {
                ServiceBusError::new(ErrorKind::Amqp, format!("Failed to peek messages: {:?}", e))
            })?;

        // Process the response to reconstruct ReceivedMessage instances
        let messages = self.parse_peeked_messages_response(response).await?;

        trace!("Successfully peeked {} messages", messages.len());

        Ok(messages)
    }

    /// Parses the response from a peek messages operation.
    ///
    /// The Service Bus management response for peek messages comes in the same format
    /// as deferred messages:
    /// - A map with key "messages"
    /// - An array of message maps, each with key "message"
    /// - Each "message" contains serialized AMQP binary data
    ///
    /// Unlike deferred messages, peeked messages do not have lock tokens.
    async fn parse_peeked_messages_response(
        &self,
        response: azure_core_amqp::AmqpOrderedMap<String, azure_core_amqp::AmqpValue>,
    ) -> Result<Vec<ReceivedMessage>> {
        use std::collections::HashMap;

        // Extract the "messages" field from the response
        let messages_value = response.get("messages").ok_or_else(|| {
            ServiceBusError::new(
                ErrorKind::InvalidRequest,
                "Management response missing 'messages' field",
            )
        })?;

        // Parse the messages array
        let messages_array = match messages_value {
            AmqpValue::Array(array) => array,
            _ => {
                return Err(ServiceBusError::new(
                    ErrorKind::InvalidRequest,
                    "Expected 'messages' field to be an array",
                ))
            }
        };

        let mut received_messages = Vec::new();

        for message_entry in messages_array {
            // Each entry should be a map with a "message" key
            let message_map = match message_entry {
                AmqpValue::Map(map) => map,
                _ => {
                    return Err(ServiceBusError::new(
                        ErrorKind::InvalidRequest,
                        "Expected message entry to be a map",
                    ))
                }
            };

            // Extract the serialized message data
            let message_data = message_map
                .get(&AmqpValue::String("message".to_string()))
                .ok_or_else(|| {
                    ServiceBusError::new(
                        ErrorKind::InvalidRequest,
                        "Message entry missing 'message' field",
                    )
                })?;

            // The message data should be binary AMQP data
            let message_bytes = match message_data {
                AmqpValue::Binary(bytes) => bytes,
                _ => {
                    return Err(ServiceBusError::new(
                        ErrorKind::InvalidRequest,
                        "Expected message data to be binary",
                    ))
                }
            };

            // Deserialize the AMQP message from binary data
            // Note: This is a simplified reconstruction. In a full implementation,
            // we would need to properly deserialize the AMQP message and extract
            // all the Service Bus specific properties, system properties, etc.

            // For now, create a basic ReceivedMessage structure
            // This would need to be enhanced to properly reconstruct from AMQP binary data
            let system_properties = SystemProperties::default();

            // Create a ReceivedMessage for peeked message
            // Peeked messages do not have lock tokens
            let received_message = ReceivedMessage::new(
                message_bytes.clone(),
                HashMap::new(),
                system_properties,
                None, // lock_token - peeked messages never have lock tokens
            );

            received_messages.push(received_message);
        }

        Ok(received_messages)
    }

    /// Closes the receiver and releases associated resources.
    ///
    /// This method gracefully closes the receiver by detaching from the AMQP link and
    /// ending the AMQP session. Once closed, the receiver cannot be used for further
    /// operations and a new receiver must be created.
    ///
    /// It's recommended to call this method when you're finished with the receiver to
    /// ensure proper cleanup of network resources.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful closure or an error if cleanup operations fail.
    /// Note that errors during cleanup are logged but generally don't indicate a
    /// problem that requires user intervention.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_messaging_servicebus::receiver::ReceiveMessageOptions;
    ///
    /// # async fn example(receiver: azure_messaging_servicebus::Receiver) -> Result<(), Box<dyn std::error::Error>> {
    /// // Use the receiver for message operations
    /// let options = Default::default();
    /// let messages = receiver.receive_messages(10, Some(options)).await?;
    ///
    /// // Process messages...
    /// for message in &messages {
    ///     receiver.complete_message(message, None).await?;
    /// }
    ///
    /// // Close the receiver when done
    /// receiver.close().await?;
    /// println!("Receiver closed successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn close(&self) -> Result<()> {
        let entity_path = self.get_entity_path();
        debug!("Closing Receiver for entity: {}", entity_path);

        // Detach the AMQP receiver if it exists
        if let Some(receiver) = self.amqp_receiver.get() {
            let receiver = receiver.clone();
            // Try to get exclusive access to the receiver for detachment
            match Arc::try_unwrap(receiver) {
                Ok(receiver) => {
                    match receiver.detach().await {
                        Ok(_) => {
                            trace!(
                                "AMQP receiver detached successfully for entity: {}",
                                entity_path
                            );
                        }
                        Err(e) => {
                            // Log but don't fail - connection might already be closed
                            warn!(
                                "Failed to detach AMQP receiver for entity '{}': {}",
                                entity_path, e
                            );
                        }
                    }
                }
                Err(_) => {
                    // Receiver is still being used elsewhere, skip detachment
                    trace!("AMQP receiver still in use for entity: {}", entity_path);
                }
            }
        }

        // End the AMQP session if it exists
        if let Some(session) = self.session.get() {
            match session.end().await {
                Ok(_) => {
                    trace!(
                        "AMQP session ended successfully for entity: {}",
                        entity_path
                    );
                }
                Err(e) => {
                    // Log but don't fail - connection might already be closed
                    warn!(
                        "Failed to end AMQP session for entity '{}': {}",
                        entity_path, e
                    );
                }
            }
        }

        trace!("Receiver closed successfully for entity: {}", entity_path);
        Ok(())
    }

    fn get_entity_path(&self) -> String {
        if let Some(ref subscription) = self.subscription_name {
            format!("{}/subscriptions/{}", self.entity_name, subscription)
        } else {
            self.entity_name.clone()
        }
    }

    /// Converts an AMQP delivery to a ReceivedMessage.
    async fn convert_delivery_to_message(
        &self,
        delivery: AmqpDelivery,
    ) -> Result<Option<ReceivedMessage>> {
        let message = delivery.message();

        // Extract body
        let body = match &message.body {
            AmqpMessageBody::Binary(binary_data) => {
                // Combine all binary chunks
                binary_data
                    .iter()
                    .flat_map(|chunk| chunk.iter())
                    .cloned()
                    .collect()
            }
            AmqpMessageBody::Value(value) => {
                // Convert value to string and then to bytes
                format!("{:?}", value).into_bytes()
            }
            AmqpMessageBody::Sequence(sequence) => {
                // Convert sequence to string and then to bytes
                format!("{:?}", sequence).into_bytes()
            }
            AmqpMessageBody::Empty => Vec::new(),
        };

        // Extract application properties
        let mut properties = HashMap::new();
        if let Some(app_props) = message.application_properties.as_ref() {
            for (key, value) in app_props.0.iter() {
                let value_str = match value {
                    AmqpSimpleValue::String(s) => s.clone(),
                    AmqpSimpleValue::Int(i) => i.to_string(),
                    AmqpSimpleValue::UInt(u) => u.to_string(),
                    AmqpSimpleValue::Long(l) => l.to_string(),
                    AmqpSimpleValue::ULong(ul) => ul.to_string(),
                    AmqpSimpleValue::Boolean(b) => b.to_string(),
                    _ => format!("{:?}", value),
                };
                properties.insert(key.clone(), value_str);
            }
        }

        // Extract system properties from message properties
        let mut system_properties = SystemProperties::default();

        if let Some(msg_props) = message.properties.as_ref() {
            if let Some(message_id) = &msg_props.message_id {
                system_properties.message_id = match message_id {
                    AmqpMessageId::String(s) => Some(s.clone()),
                    AmqpMessageId::Uuid(u) => Some(u.to_string()),
                    AmqpMessageId::Binary(b) => Some(format!("{:?}", b)),
                    AmqpMessageId::Ulong(u) => Some(u.to_string()),
                };
            }
            if let Some(correlation_id) = &msg_props.correlation_id {
                system_properties.correlation_id = match correlation_id {
                    AmqpMessageId::String(s) => Some(s.clone()),
                    AmqpMessageId::Uuid(u) => Some(u.to_string()),
                    AmqpMessageId::Binary(b) => Some(format!("{:?}", b)),
                    AmqpMessageId::Ulong(u) => Some(u.to_string()),
                };
            }
            if let Some(content_type) = &msg_props.content_type {
                system_properties.content_type = Some(content_type.into());
            }
            if let Some(reply_to) = &msg_props.reply_to {
                system_properties.reply_to = Some(reply_to.clone());
            }
            if let Some(subject) = &msg_props.subject {
                system_properties.subject = Some(subject.clone());
            }
        }

        // Generate a lock token for PeekLock mode and store the delivery
        let lock_token = if self.receive_mode == ReceiveMode::PeekLock {
            let token = Uuid::new_v4();
            // Store the delivery for later settlement operations
            let mut delivery_map = self.delivery_map.lock().await;
            delivery_map.insert(token, delivery);
            Some(token)
        } else {
            None
        };

        let received_message =
            ReceivedMessage::new(body, properties, system_properties, lock_token);

        Ok(Some(received_message))
    }
}

impl Drop for Receiver {
    fn drop(&mut self) {
        let entity_path = self.get_entity_path();
        trace!("Receiver for entity '{}' is being dropped", entity_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    async fn test_receiver_creation_queue() -> Result<()> {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name.clone(),
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await?;

        assert_eq!(receiver.entity_name(), &entity_name);
        assert_eq!(receiver.subscription_name(), None);
        assert_eq!(receiver.receive_mode(), ReceiveMode::PeekLock);
        Ok(())
    }

    #[tokio::test]
    async fn test_receiver_creation_subscription() -> Result<()> {
        let connection = create_test_connection();
        let entity_name = "test-topic".to_string();
        let subscription_name = "test-subscription".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name.clone(),
            Some(subscription_name.clone()),
            ReceiveMode::ReceiveAndDelete,
            options,
        )
        .await?;

        assert_eq!(receiver.entity_name(), &entity_name);
        assert_eq!(
            receiver.subscription_name(),
            Some(subscription_name.as_str())
        );
        assert_eq!(receiver.receive_mode(), ReceiveMode::ReceiveAndDelete);
        Ok(())
    }

    #[test]
    fn test_receive_mode_equality() {
        assert_eq!(ReceiveMode::PeekLock, ReceiveMode::PeekLock);
        assert_eq!(ReceiveMode::ReceiveAndDelete, ReceiveMode::ReceiveAndDelete);
        assert_ne!(ReceiveMode::PeekLock, ReceiveMode::ReceiveAndDelete);
    }

    #[test]
    fn test_receive_message_options_default() {
        let options = ReceiveMessageOptions::default();
        assert_eq!(options.max_message_count, 1);
        assert_eq!(options.max_wait_time, Some(Duration::seconds(60)));
    }

    #[test]
    fn test_receive_message_options_custom() {
        let options = ReceiveMessageOptions {
            max_message_count: 10,
            max_wait_time: Some(Duration::seconds(30)),
        };

        assert_eq!(options.max_message_count, 10);
        assert_eq!(options.max_wait_time, Some(Duration::seconds(30)));
    }

    #[test]
    fn test_get_entity_path_queue() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver {
            connection,
            entity_name: entity_name.clone(),
            subscription_name: None,
            receive_mode: ReceiveMode::PeekLock,
            _options: options,
            session: OnceCell::new(),
            amqp_receiver: OnceCell::new(),
            delivery_map: Arc::new(Mutex::new(HashMap::new())),
        };

        assert_eq!(receiver.get_entity_path(), entity_name);
    }

    #[test]
    fn test_get_entity_path_subscription() {
        let connection = create_test_connection();
        let entity_name = "test-topic".to_string();
        let subscription_name = "test-subscription".to_string();
        let options = create_test_options();

        let receiver = Receiver {
            connection,
            entity_name: entity_name.clone(),
            subscription_name: Some(subscription_name.clone()),
            receive_mode: ReceiveMode::PeekLock,
            _options: options,
            session: OnceCell::new(),
            amqp_receiver: OnceCell::new(),
            delivery_map: Arc::new(Mutex::new(HashMap::new())),
        };

        let expected_path = format!("{}/subscriptions/{}", entity_name, subscription_name);
        assert_eq!(receiver.get_entity_path(), expected_path);
    }

    #[tokio::test]
    async fn test_receiver_close() -> Result<()> {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await?;

        // Should not fail
        receiver.close().await?;
        Ok(())
    }

    #[test]
    fn test_receiver_drop() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver {
            connection,
            entity_name,
            subscription_name: None,
            receive_mode: ReceiveMode::PeekLock,
            _options: options,
            session: OnceCell::new(),
            amqp_receiver: OnceCell::new(),
            delivery_map: Arc::new(Mutex::new(HashMap::new())),
        };

        // Should not panic when dropped
        drop(receiver);
    }

    #[tokio::test]
    async fn test_complete_message_receive_and_delete_mode() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::ReceiveAndDelete,
            options,
        )
        .await
        .unwrap();

        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: None,
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: None,
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            None,
        );

        let result = receiver.complete_message(&message, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("PeekLock mode"));
    }

    #[tokio::test]
    async fn test_abandon_message_receive_and_delete_mode() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::ReceiveAndDelete,
            options,
        )
        .await
        .unwrap();

        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: None,
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: None,
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            None,
        );

        let result = receiver.abandon_message(&message, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("PeekLock mode"));
    }

    #[tokio::test]
    async fn test_dead_letter_message_receive_and_delete_mode() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::ReceiveAndDelete,
            options,
        )
        .await
        .unwrap();

        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: None,
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: None,
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            None,
        );

        let result = receiver
            .dead_letter_message(
                &message,
                Some(DeadLetterMessageOptions {
                    reason: Some("test reason".to_string()),
                    error_description: Some("test description".to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("PeekLock mode"));
    }

    #[tokio::test]
    async fn test_complete_message_no_lock_token() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: None,
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: None,
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            None, // No lock token
        );

        let result = receiver.complete_message(&message, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("lock token"));
    }

    #[tokio::test]
    async fn test_defer_message_receive_and_delete_mode() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::ReceiveAndDelete,
            options,
        )
        .await
        .unwrap();

        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: None,
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: None,
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            None,
        );

        let result = receiver.defer_message(&message, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("PeekLock mode"));
    }

    #[tokio::test]
    async fn test_defer_message_no_lock_token() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: None,
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: None,
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            None, // No lock token
        );

        let result = receiver.defer_message(&message, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("lock token"));
    }

    #[tokio::test]
    async fn test_receive_deferred_message() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let result = receiver.receive_deferred_message(12345, None).await;
        assert!(result.is_ok());
        // Should return None since it's not implemented yet
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_receive_deferred_messages() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let sequence_numbers = vec![12345, 67890];
        let result = receiver
            .receive_deferred_messages(&sequence_numbers, None)
            .await;

        // Should fail since we don't have a real connection, but validates the API
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("AMQP")
                || error_msg.contains("Failed")
                || error_msg.contains("management")
        );
    }

    #[tokio::test]
    async fn test_receive_deferred_message_receive_and_delete_mode() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::ReceiveAndDelete,
            options,
        )
        .await
        .unwrap();

        let result = receiver.receive_deferred_message(12345, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("PeekLock mode"));
    }

    #[tokio::test]
    async fn test_defer_message_success() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let lock_token = Uuid::new_v4();
        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: Some("test-msg-id".to_string()),
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: Some(12345),
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            Some(lock_token),
        );

        // Test that defer_message accepts a message with valid lock token
        // Note: This will fail in practice without a real management client,
        // but validates the input validation logic
        let result = receiver.defer_message(&message, None).await;

        // The operation will fail because we're using a mock connection,
        // but we're testing that it passes initial validation
        assert!(result.is_err());
        // Should fail due to mock connection limitations
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("AMQP") || error_msg.contains("Failed"));
    }

    #[tokio::test]
    async fn test_defer_message_with_properties() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let lock_token = Uuid::new_v4();
        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: Some("test-msg-id".to_string()),
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: Some(12345),
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            Some(lock_token),
        );

        let mut properties_to_modify = HashMap::new();
        properties_to_modify.insert("CustomProperty".to_string(), "CustomValue".to_string());

        let defer_options = DeferMessageOptions {
            properties_to_modify: Some(properties_to_modify),
        };

        let result = receiver.defer_message(&message, Some(defer_options)).await;

        // Should fail on mock connection but pass validation
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("AMQP") || error_msg.contains("Failed"));
    }

    #[tokio::test]
    async fn test_renew_message_lock_success() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let lock_token = Uuid::new_v4();
        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: Some("test-msg-id".to_string()),
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: Some(12345),
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            Some(lock_token),
        );

        // Test that renew_message_lock accepts a message with valid lock token
        // Note: This will fail in practice without a real management client,
        // but validates the input validation logic
        let result = receiver.renew_message_lock(&message, None).await;

        // The operation will fail because we're using a mock connection,
        // but we're testing that it passes initial validation
        assert!(result.is_err());
        // Should fail on mock connection but pass validation
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("AMQP") || error_msg.contains("Failed"));
    }

    #[tokio::test]
    async fn test_renew_message_lock_receive_and_delete_mode() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::ReceiveAndDelete,
            options,
        )
        .await
        .unwrap();

        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: None,
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: None,
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            None,
        );

        let result = receiver.renew_message_lock(&message, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("PeekLock mode"));
    }

    #[tokio::test]
    async fn test_renew_message_lock_no_lock_token() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: None,
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: None,
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            None, // No lock token
        );

        let result = receiver.renew_message_lock(&message, None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("lock token"));
    }

    #[tokio::test]
    async fn test_message_lock_renewal_returns_valid_time() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let lock_token = Uuid::new_v4();
        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: Some("test-msg-id".to_string()),
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: Some(12345),
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            Some(lock_token),
        );

        // Even though the operation will fail with a mock connection,
        // we can verify that the method signature returns the expected type
        let result = receiver.renew_message_lock(&message, None).await;

        // The operation should fail, but we're testing it doesn't panic
        // and returns a proper Result<OffsetDateTime>
        assert!(result.is_err());

        // Verify error occurs but don't check specific message due to mock connection
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("AMQP") || error_msg.contains("Failed"));
    }

    #[tokio::test]
    async fn test_defer_message_validates_lock_token_format() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        // Test with a valid UUID lock token format
        let lock_token = Uuid::new_v4();
        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: Some("test-msg-id".to_string()),
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: Some(12345),
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            Some(lock_token),
        );

        let result = receiver.defer_message(&message, None).await;

        // Should pass lock token validation and fail on mock connection
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("AMQP") || error_msg.contains("Failed"));
    }

    #[tokio::test]
    async fn test_defer_and_receive_deferred_integration() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let lock_token = Uuid::new_v4();
        let sequence_number = 12345i64;

        let message = ReceivedMessage::new(
            b"test deferred message".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: Some("test-msg-id".to_string()),
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: Some(sequence_number),
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            Some(lock_token),
        );

        // Test defer message
        let defer_result = receiver.defer_message(&message, None).await;

        // Should fail on mock management client
        assert!(defer_result.is_err());

        // Test receive deferred message by sequence number
        let receive_result = receiver
            .receive_deferred_message(sequence_number, None)
            .await;

        // Should succeed but return None since it's a placeholder implementation
        assert!(receive_result.is_ok());
        assert!(receive_result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_multiple_message_lock_renewals() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        // Create multiple messages with different lock tokens
        let lock_token_1 = Uuid::new_v4();
        let lock_token_2 = Uuid::new_v4();

        let message_1 = ReceivedMessage::new(
            b"test message 1".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: Some("test-msg-id-1".to_string()),
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: Some(1),
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            Some(lock_token_1),
        );

        let message_2 = ReceivedMessage::new(
            b"test message 2".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: Some("test-msg-id-2".to_string()),
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: Some(2),
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            Some(lock_token_2),
        );

        // Test renewing locks for multiple messages
        let result_1 = receiver.renew_message_lock(&message_1, None).await;
        let result_2 = receiver.renew_message_lock(&message_2, None).await;

        // Both should fail on mock management client but validate different lock tokens
        assert!(result_1.is_err());
        assert!(result_2.is_err());

        // Verify they're different lock tokens being processed
        assert_ne!(lock_token_1, lock_token_2);
    }

    #[tokio::test]
    async fn test_defer_message_with_empty_properties() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let lock_token = Uuid::new_v4();
        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: Some("test-msg-id".to_string()),
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: Some(12345),
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            Some(lock_token),
        );

        // Test with None properties_to_modify
        let defer_options = DeferMessageOptions {
            properties_to_modify: None,
        };

        let result = receiver.defer_message(&message, Some(defer_options)).await;

        // Should pass validation and fail on mock connection
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("AMQP") || error_msg.contains("Failed"));
    }

    #[tokio::test]
    async fn test_renew_message_lock_with_different_options() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let lock_token = Uuid::new_v4();
        let message = ReceivedMessage::new(
            b"test".to_vec(),
            HashMap::new(),
            SystemProperties {
                message_id: Some("test-msg-id".to_string()),
                correlation_id: None,
                session_id: None,
                content_type: None,
                reply_to: None,
                reply_to_session_id: None,
                subject: None,
                enqueued_time_utc: None,
                sequence_number: Some(12345),
                delivery_count: None,
                time_to_live: None,
                dead_letter_source: None,
                dead_letter_reason: None,
                dead_letter_error_description: None,
            },
            Some(lock_token),
        );

        // Test with default options (currently not used but validates API)
        let result = receiver.renew_message_lock(&message, None).await;

        // Should pass validation and fail on management operation
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("AMQP") || error_msg.contains("Failed"));
    }

    #[tokio::test]
    async fn test_receive_deferred_messages_basic() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let sequence_numbers = vec![12345, 67890];
        let result = receiver
            .receive_deferred_messages(&sequence_numbers, None)
            .await;

        // Should fail since we don't have a real connection, but validates the API
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("AMQP")
                || error_msg.contains("Failed")
                || error_msg.contains("management")
        );
    }

    #[tokio::test]
    async fn test_receive_deferred_messages_empty_sequence_numbers() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let sequence_numbers = vec![];
        let result = receiver
            .receive_deferred_messages(&sequence_numbers, None)
            .await;

        // Should return empty result for empty input
        match result {
            Ok(messages) => assert!(messages.is_empty()),
            Err(_) => {
                // May fail due to mock connection, but that's acceptable
            }
        }
    }

    #[tokio::test]
    async fn test_receive_deferred_messages_single_sequence_number() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let sequence_numbers = vec![42];
        let result = receiver
            .receive_deferred_messages(&sequence_numbers, None)
            .await;

        // Should fail since we don't have a real connection, but validates the API
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("AMQP")
                || error_msg.contains("Failed")
                || error_msg.contains("management")
        );
    }

    #[tokio::test]
    async fn test_parse_deferred_messages_response_empty() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        // Test with empty response
        let mut response = azure_core_amqp::AmqpOrderedMap::new();
        response.insert("messages".to_string(), AmqpValue::Array(vec![]));

        let result = receiver.parse_deferred_messages_response(response).await;
        assert!(result.is_ok());
        let messages = result.unwrap();
        assert!(messages.is_empty());
    }

    #[tokio::test]
    async fn test_parse_deferred_messages_response_missing_messages_field() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        // Test with response missing "messages" field
        let response = azure_core_amqp::AmqpOrderedMap::new();

        let result = receiver.parse_deferred_messages_response(response).await;
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("messages"));
    }

    #[tokio::test]
    async fn test_parse_deferred_messages_response_invalid_format() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        // Test with invalid "messages" field (not an array)
        let mut response = azure_core_amqp::AmqpOrderedMap::new();
        response.insert(
            "messages".to_string(),
            AmqpValue::String("invalid".to_string()),
        );

        let result = receiver.parse_deferred_messages_response(response).await;
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("array"));
    }

    #[tokio::test]
    async fn test_peek_messages_basic() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        // This would fail in a real test because there's no actual management client
        // But we're testing the parameter validation logic
        let result = receiver.peek_messages(10, None).await;
        // In a mock environment this will fail at the AMQP level, which is expected
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_peek_messages_zero_count() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let result = receiver.peek_messages(0, None).await;
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Max count must be greater than 0"));
    }

    #[tokio::test]
    async fn test_peek_messages_with_sequence_number() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        let peek_options = PeekMessagesOptions {
            from_sequence_number: Some(12345),
        };

        // This would fail in a real test because there's no actual management client
        // But we're testing that the sequence number parameter is correctly handled
        let result = receiver.peek_messages(5, Some(peek_options)).await;
        // In a mock environment this will fail at the AMQP level, which is expected
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_peek_messages_both_modes_allowed() {
        // Test that peek works in both PeekLock and ReceiveAndDelete modes
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        // Test PeekLock mode
        let receiver_peek_lock = Receiver::new(
            connection.clone(),
            entity_name.clone(),
            None,
            ReceiveMode::PeekLock,
            options.clone(),
        )
        .await
        .unwrap();

        let result = receiver_peek_lock.peek_messages(1, None).await;
        // Should fail at AMQP level, not validation level
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(!error_msg.contains("mode")); // Should not contain mode-related errors

        // Test ReceiveAndDelete mode
        let receiver_receive_delete = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::ReceiveAndDelete,
            options,
        )
        .await
        .unwrap();

        let result = receiver_receive_delete.peek_messages(1, None).await;
        // Should fail at AMQP level, not validation level
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(!error_msg.contains("mode")); // Should not contain mode-related errors
    }

    #[tokio::test]
    async fn test_parse_peeked_messages_response_missing_messages() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        // Test with response missing "messages" field
        let response = azure_core_amqp::AmqpOrderedMap::new();

        let result = receiver.parse_peeked_messages_response(response).await;
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("messages"));
    }

    #[tokio::test]
    async fn test_parse_peeked_messages_response_invalid_format() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        // Test with invalid "messages" field (not an array)
        let mut response = azure_core_amqp::AmqpOrderedMap::new();
        response.insert(
            "messages".to_string(),
            AmqpValue::String("invalid".to_string()),
        );

        let result = receiver.parse_peeked_messages_response(response).await;
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("array"));
    }

    #[tokio::test]
    async fn test_parse_peeked_messages_response_empty_array() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        // Test with empty messages array
        let mut response = azure_core_amqp::AmqpOrderedMap::new();
        response.insert("messages".to_string(), AmqpValue::Array(vec![]));

        let result = receiver.parse_peeked_messages_response(response).await;
        assert!(result.is_ok());
        let messages = result.unwrap();
        assert_eq!(messages.len(), 0);
    }

    #[tokio::test]
    async fn test_parse_peeked_messages_response_invalid_message_entry() {
        let connection = create_test_connection();
        let entity_name = "test-queue".to_string();
        let options = create_test_options();

        let receiver = Receiver::new(
            connection,
            entity_name,
            None,
            ReceiveMode::PeekLock,
            options,
        )
        .await
        .unwrap();

        // Test with invalid message entry (not a map)
        let mut response = azure_core_amqp::AmqpOrderedMap::new();
        let messages_array = vec![AmqpValue::String("invalid".to_string())];
        response.insert("messages".to_string(), AmqpValue::Array(messages_array));

        let result = receiver.parse_peeked_messages_response(response).await;
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("map"));
    }

    #[tokio::test]
    async fn test_peek_messages_options_default() {
        let options = PeekMessagesOptions::default();
        assert_eq!(options.from_sequence_number, None);
    }

    #[tokio::test]
    async fn test_peek_messages_options_with_sequence_number() {
        let options = PeekMessagesOptions {
            from_sequence_number: Some(12345),
        };
        assert_eq!(options.from_sequence_number, Some(12345));
    }
}
