//! Implements `ServiceBusMessageBatch`.

use fe2o3_amqp_types::messaging::{Data, Message};

use crate::{
    amqp::{amqp_message_batch::AmqpMessageBatch, error::TryAddMessageError},
    core::TransportMessageBatch,
    ServiceBusMessage,
};

// Conditional import for docs.rs
#[cfg(docsrs)]
use crate::ServiceBusSender;

/// The set of options that can be specified to influence the way in which an service bus message
/// batch behaves and is sent to the Queue/Topic.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateMessageBatchOptions {
    /// The maximum size of the batch, in bytes.
    pub max_size_in_bytes: Option<u64>,
}

/// A set of [`ServiceBusMessage`] with size constraints known up-front, intended to be sent to the
/// Queue/Topic as a single batch.
///
/// A [`ServiceBusMessageBatch`] can be created using [`ServiceBusSender::create_message_batch`].
/// Messages can be added to the batch using the [`Self::try_add_message`] method on the batch.
///
/// # Examples
///
/// ```rust,ignore
/// use azure_messaging_servicebus::CreateMessageBatchOptions;
///
/// let options = CreateMessageBatchOptions::default();
/// let mut message_batch = sender.create_message_batch(options).unwrap();
/// message_batch.try_add_message("Message 1").unwrap();
/// message_batch.try_add_message("Message 2").unwrap();
/// message_batch.try_add_message("Message 3").unwrap();
///
/// sender.send_message_batch(message_batch).await.unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServiceBusMessageBatch {
    pub(crate) inner: AmqpMessageBatch,
}

impl ServiceBusMessageBatch {
    /// The maximum size of the batch, in bytes.
    pub fn max_size_in_bytes(&self) -> u64 {
        self.inner.max_size_in_bytes()
    }

    /// The size of the batch, in bytes.
    pub fn size_in_bytes(&self) -> u64 {
        self.inner.size_in_bytes()
    }

    /// Number of messages in the batch.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the batch contains no messages.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Attempts to add a [`ServiceBusMessage`] to the [`ServiceBusMessageBatch`].
    ///
    /// Returns an error if the message is too large to fit in the batch or
    /// if the message fails to serialize. The original message can be recovered
    /// from the error.
    pub fn try_add_message(
        &mut self,
        message: impl Into<ServiceBusMessage>,
    ) -> Result<(), TryAddMessageError> {
        self.inner.try_add_message(message.into())
    }

    /// Iterate over the messages in the batch.
    pub fn iter(&self) -> std::slice::Iter<'_, Message<Data>> {
        self.inner.iter()
    }

    /// Clears the batch, removing all messages and resetting the available size.
    pub fn clear(&mut self) {
        self.inner.clear()
    }
}
