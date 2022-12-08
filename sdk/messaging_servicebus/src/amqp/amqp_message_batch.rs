use fe2o3_amqp_types::messaging::{message::__private::Serializable, Data, Message};
use serde_amqp::serialized_size;

use crate::core::TransportMessageBatch;

use super::error::TryAddMessageError;

/// A set of <see cref="ServiceBusMessage" /> with size constraints known up-front, intended to be
/// sent to the Queue/Topic as a single batch. A <see cref="ServiceBusMessageBatch"/> can be
/// created using <see
/// cref="ServiceBusSender.CreateMessageBatchAsync(System.Threading.CancellationToken)"/>.
/// Messages can be added to the batch using the <see cref="TryAddMessage"/> method on the batch.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AmqpMessageBatch {
    /// The maximum size of the batch, in bytes.
    pub(crate) max_size_in_bytes: u64,
    /// The list of messages that will be sent as a batch.
    pub(crate) messages: Vec<Message<Data>>,
    /// The size of the batch, in bytes.
    pub(crate) size_in_bytes: u64,
}

impl AmqpMessageBatch {
    pub(crate) fn new(max_size_in_bytes: u64) -> Self {
        Self {
            max_size_in_bytes,
            messages: Vec::new(),
            size_in_bytes: 0,
        }
    }
}

impl TransportMessageBatch for AmqpMessageBatch {
    type TryAddError = TryAddMessageError;

    type Iter<'a> = std::slice::Iter<'a, Message<Data>>;

    fn max_size_in_bytes(&self) -> u64 {
        self.max_size_in_bytes
    }

    fn size_in_bytes(&self) -> u64 {
        self.size_in_bytes
    }

    fn len(&self) -> usize {
        self.messages.len()
    }

    fn try_add_message(
        &mut self,
        message: crate::ServiceBusMessage,
    ) -> Result<(), Self::TryAddError> {
        let serializable_message = Serializable(&message.amqp_message);
        let ssize = match serialized_size(&serializable_message) {
            Ok(size) => size,
            Err(err) => {
                return Err(Self::TryAddError::Codec {
                    source: err,
                    message,
                })
            }
        };

        let new_size = self.size_in_bytes + ssize as u64;
        if new_size > self.max_size_in_bytes {
            return Err(TryAddMessageError::BatchFull(message));
        }

        self.size_in_bytes = new_size;
        self.messages.push(message.amqp_message);
        Ok(())
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.messages.iter()
    }

    fn clear(&mut self) {
        self.messages.clear();
        self.size_in_bytes = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::ServiceBusMessage;

    use super::*;

    #[test]
    fn new_sets_max_size_in_bytes() {
        let batch = AmqpMessageBatch::new(1024);
        assert_eq!(batch.max_size_in_bytes, 1024);
    }

    #[test]
    fn try_add_sets_batch_size_in_bytes() {
        let mut batch = AmqpMessageBatch::new(1024);
        let message = ServiceBusMessage::new("hello world");

        let serializable = Serializable(&message.amqp_message);
        let size = serialized_size(&serializable).unwrap();

        assert!(size * 2 < 1024);

        assert!(batch.try_add_message(message.clone()).is_ok());
        assert_eq!(batch.size_in_bytes, size as u64);

        assert!(batch.try_add_message(message).is_ok());
        assert_eq!(batch.size_in_bytes, size as u64 * 2);
    }

    #[test]
    fn try_add_accepts_message_smaller_than_max_size() {
        let mut batch = AmqpMessageBatch::new(1024);
        let message = ServiceBusMessage::new("hello world");

        // Make sure the message is smaller than the max size
        let serializable = Serializable(message.amqp_message.clone());
        let message_size = serialized_size(&serializable).unwrap();
        assert!(message_size < 1024);

        assert!(batch.try_add_message(message).is_ok());
    }

    #[test]
    fn try_add_does_not_accept_message_larger_than_max_size() {
        let mut batch = AmqpMessageBatch::new(1024);
        let message = ServiceBusMessage::new(vec![0u8; 1025]);

        // Make sure the message is larger than the max size
        let serializable = Serializable(message.amqp_message.clone());
        let message_size = serialized_size(&serializable).unwrap();
        assert!(message_size > 1024);

        assert!(batch.try_add_message(message).is_err());
    }

    #[test]
    fn try_add_accepts_message_until_batch_is_full() {
        let max_size_in_bytes = 1024;
        let mut batch = AmqpMessageBatch::new(max_size_in_bytes);

        let message = ServiceBusMessage::new("hello world");

        let mut cumulated_size_in_bytes = 0;
        loop {
            let serializable = Serializable(message.amqp_message.clone());
            let message_size = serialized_size(&serializable).unwrap();
            cumulated_size_in_bytes += message_size;
            if cumulated_size_in_bytes as u64 > max_size_in_bytes {
                break;
            }

            assert!(batch.try_add_message(message.clone()).is_ok());
        }

        assert!(batch.try_add_message(message).is_err());
    }

    #[test]
    fn iter_returns_iterator_over_added_messages() {
        let mut batch = AmqpMessageBatch::new(1024);

        let messages: Vec<_> = (0..5)
            .map(|i| ServiceBusMessage::new(format!("message {}", i)))
            .collect();
        for message in messages.iter() {
            assert!(batch.try_add_message(message.clone()).is_ok());
        }

        let iter = batch.iter();
        for (original, added) in messages.into_iter().zip(iter) {
            assert_eq!(original.amqp_message, *added);
        }
    }

    #[test]
    fn clear_resets_batch_len_and_size_in_bytes() {
        let mut batch = AmqpMessageBatch::new(1024);
        let message = ServiceBusMessage::new("hello world");

        assert!(batch.try_add_message(message.clone()).is_ok());
        assert_eq!(batch.len(), 1);
        assert!(batch.size_in_bytes > 0);

        batch.clear();
        assert_eq!(batch.len(), 0);
        assert_eq!(batch.size_in_bytes, 0);
    }
}
