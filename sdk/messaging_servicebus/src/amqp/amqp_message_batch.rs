use fe2o3_amqp_types::messaging::{message::__private::Serializable, Data, Message};
use serde_amqp::serialized_size;

use crate::{core::TransportMessageBatch, ServiceBusMessage};

use super::error::TryAddMessageError;

/// A set of <see cref="ServiceBusMessage" /> with size constraints known up-front, intended to be
/// sent to the Queue/Topic as a single batch. A <see cref="ServiceBusMessageBatch"/> can be
/// created using <see
/// cref="ServiceBusSender.CreateMessageBatchAsync(System.Threading.CancellationToken)"/>.
/// Messages can be added to the batch using the <see cref="TryAddMessage"/> method on the batch.
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
        let serializable_message = Serializable(message.amqp_message);
        let ssize = serialized_size(&serializable_message)?;

        let new_size = self.size_in_bytes + ssize as u64;
        if new_size > self.max_size_in_bytes {
            return Err(TryAddMessageError::BatchFull(ServiceBusMessage {
                amqp_message: serializable_message.0,
            }));
        }

        self.messages.push(serializable_message.0);
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
