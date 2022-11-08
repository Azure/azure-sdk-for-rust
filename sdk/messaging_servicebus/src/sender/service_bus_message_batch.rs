use crate::{core::TransportMessageBatch, ServiceBusMessage};

pub struct ServiceBusMessageBatch<T: TransportMessageBatch> {
    pub(crate) inner: T,
}

impl<T: TransportMessageBatch> ServiceBusMessageBatch<T> {
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

    /// Attempts to add a <see cref="ServiceBusMessage"/> to the <see cref="ServiceBusMessageBatch"/>.
    pub fn try_add_message(
        &mut self,
        message: impl Into<ServiceBusMessage>,
    ) -> Result<(), T::TryAddError> {
        self.inner.try_add_message(message.into())
    }

    /// Iterate over the messages in the batch.
    pub fn iter(&self) -> T::Iter<'_> {
        self.inner.iter()
    }

    /// Clears the batch, removing all messages and resetting the available size.
    pub fn clear(&mut self) {
        self.inner.clear()
    }
}
