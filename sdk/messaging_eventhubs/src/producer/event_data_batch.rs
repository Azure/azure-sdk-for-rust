use fe2o3_amqp_types::messaging::{Data, Message};

use crate::{
    amqp::{amqp_event_batch::AmqpEventBatch, error::TryAddError},
    core::TransportEventBatch,
    EventData,
};

/// A set of [`EventData`] with size constraints known up-front, intended to be sent to the Event
/// Hubs service in a single operation. When published, the result is atomic; either all events that
/// belong to the batch were successful or all have failed. Partial success is not possible.
#[derive(Debug)]
pub struct EventDataBatch {
    pub(crate) inner: AmqpEventBatch,
}

impl EventDataBatch {
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
    pub fn try_add(&mut self, message: impl Into<EventData>) -> Result<(), TryAddError> {
        self.inner.try_add(message.into())
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
