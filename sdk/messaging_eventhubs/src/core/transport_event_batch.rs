use crate::EventData;

pub trait TransportEventBatch {
    /// Error with adding a message to the batch.
    type TryAddError: std::error::Error + Send;

    /// An iterator type over the messages in the batch.
    type Iter<'a>: Iterator + ExactSizeIterator + 'a
    where
        Self: 'a;

    /// The maximum size of the batch, in bytes.
    fn max_size_in_bytes(&self) -> u64;

    /// The size of the batch, in bytes.
    fn size_in_bytes(&self) -> u64;

    /// Number of messages in the batch.
    fn len(&self) -> usize;

    /// Returns true if the batch is empty.
    fn is_empty(&self) -> bool;

    /// Attempts to add a [`EventData`] to the batch.
    fn try_add(&mut self, event: EventData) -> Result<(), Self::TryAddError>;

    /// Iterate over the messages in the batch.
    fn iter(&self) -> Self::Iter<'_>;

    /// Clears the batch, removing all messages and resetting the available size.
    fn clear(&mut self);
}
