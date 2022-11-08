use crate::ServiceBusMessage;

pub trait TransportMessageBatch {
    type TryAddError: std::error::Error + Send;
    type Iter<'a>: Iterator
    where
        Self: 'a;

    /// The maximum size of the batch, in bytes.
    fn max_size_in_bytes(&self) -> u64;

    /// The size of the batch, in bytes.
    fn size_in_bytes(&self) -> u64;

    /// Number of messages in the batch.
    fn len(&self) -> usize;

    /// Attempts to add a <see cref="ServiceBusMessage"/> to the <see cref="ServiceBusMessageBatch"/>.
    fn try_add_message(&mut self, message: ServiceBusMessage) -> Result<(), Self::TryAddError>;

    /// Iterate over the messages in the batch.
    fn iter(&self) -> Self::Iter<'_>;

    /// Clears the batch, removing all messages and resetting the available size.
    fn clear(&mut self);
}
