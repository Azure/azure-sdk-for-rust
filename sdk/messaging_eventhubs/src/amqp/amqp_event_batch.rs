use fe2o3_amqp_types::messaging::{message::__private::Serializable, Data, Message};
use serde_amqp::serialized_size;

use crate::{
    core::transport_event_batch::TransportEventBatch,
    producer::create_batch_options::CreateBatchOptions, Event,
};

use super::{
    amqp_message_converter::{build_amqp_batch_from_messages, SendableEnvelope},
    error::TryAddError,
};

/// A set of [`Event`] with size constraints known up-front, intended to be
/// sent to the Queue/Topic as a single batch. A [`EventBatch`] can be
/// created using `ServiceBusSender::create_message_batch()`.
/// Messages can be added to the batch using the [`try_add`] method on the batch.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AmqpEventBatch {
    /// The maximum size of the batch, in bytes.
    pub(crate) max_size_in_bytes: u64,
    /// The list of events that will be sent as a batch.
    pub(crate) events: Vec<Message<Data>>,
    /// The size of the batch, in bytes.
    pub(crate) size_in_bytes: u64,

    pub(crate) options: CreateBatchOptions,
}

impl AmqpEventBatch {
    pub(crate) fn new(max_size_in_bytes: u64, options: CreateBatchOptions) -> Self {
        Self {
            max_size_in_bytes,
            events: Vec::new(),
            size_in_bytes: 0,
            options,
        }
    }
}

impl TransportEventBatch for AmqpEventBatch {
    type TryAddError = TryAddError;

    type Iter<'a> = std::slice::Iter<'a, Message<Data>>;

    fn max_size_in_bytes(&self) -> u64 {
        self.max_size_in_bytes
    }

    fn size_in_bytes(&self) -> u64 {
        self.size_in_bytes
    }

    fn len(&self) -> usize {
        self.events.len()
    }

    fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    fn try_add(&mut self, event: Event) -> Result<(), Self::TryAddError> {
        let serializable_message = Serializable(&event.amqp_message);

        // Initialize the size by reserving space for the batch envelope taking into account the
        // properties from the first message which will be used to populate properties on the batch
        // envelope.
        let new_size = if self.events.is_empty() {
            // TODO: avoid clone
            let reservce_overhead_message = std::iter::once(event.clone().amqp_message);
            // Force batching to get the overhead size
            let reserve_overhead_envelope = match build_amqp_batch_from_messages(
                reservce_overhead_message,
                self.options.partition_key.clone(),
            ) {
                Some(envelope) => envelope,
                None => unreachable!(), // force batching a single message should never fail
            };
            let result = match reserve_overhead_envelope.sendable {
                SendableEnvelope::Single(sendable) => {
                    serialized_size(&Serializable(sendable.message))
                }
                SendableEnvelope::Batch(sendable) => {
                    serialized_size(&Serializable(sendable.message))
                }
            };
            match result {
                Ok(size) => size as u64,
                Err(err) => return Err(Self::TryAddError::Codec { source: err, event }),
            }
        } else {
            let ssize = match serialized_size(&serializable_message) {
                Ok(size) => size,
                Err(err) => return Err(Self::TryAddError::Codec { source: err, event }),
            };

            self.size_in_bytes + ssize as u64
        };

        if new_size > self.max_size_in_bytes {
            return Err(TryAddError::BatchFull(event));
        }

        self.size_in_bytes = new_size;
        self.events.push(event.amqp_message);

        Ok(())
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.events.iter()
    }

    fn clear(&mut self) {
        self.events.clear();
        self.size_in_bytes = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::Event;

    use super::*;

    /// The amount of bytes to reserve as overhead for a small message.
    const OVERHEAD_BYTES_SMALL_MESSAGE: usize = 5;

    /// The amount of bytes to reserve as overhead for a large message.
    const OVERHEAD_BYTES_LARGE_MESSAGE: usize = 8;

    /// The maximum number of bytes that a message may be to be considered small.
    const MAXIMUM_BYTES_SMALL_MESSAGE: usize = 255;

    #[test]
    fn new_sets_max_size_in_bytes() {
        let options = CreateBatchOptions::default();
        let batch = AmqpEventBatch::new(1024, options);
        assert_eq!(batch.max_size_in_bytes, 1024);
    }

    // #[test]
    // fn try_add_sets_batch_size_in_bytes() {
    //     let overhead = OVERHEAD_BYTES_SMALL_MESSAGE; // The events added are small
    //     let mut batch = AmqpEventBatch::new(1024);
    //     let message = Event::new("hello world");

    //     let serializable = Serializable(&message.amqp_message);
    //     let size = serialized_size(&serializable).unwrap();

    //     assert!(size * 2 < 1024);

    //     assert!(batch.try_add(message.clone()).is_ok());
    //     assert_eq!(batch.size_in_bytes, (size + overhead) as u64);

    //     assert!(batch.try_add(message).is_ok());
    //     assert_eq!(batch.size_in_bytes, (size * 2 + overhead) as u64);
    // }

    // #[test]
    // fn try_add_accepts_message_smaller_than_max_size() {
    //     let mut batch = AmqpEventBatch::new(1024);
    //     let message = Event::new("hello world");

    //     // Make sure the message is smaller than the max size
    //     let serializable = Serializable(message.amqp_message.clone());
    //     let message_size = serialized_size(&serializable).unwrap();
    //     assert!(message_size < 1024);

    //     assert!(batch.try_add(message).is_ok());
    // }

    // #[test]
    // fn try_add_does_not_accept_message_larger_than_max_size() {
    //     let mut batch = AmqpEventBatch::new(1024);
    //     let message = Event::new(vec![0u8; 1025]);

    //     // Make sure the message is larger than the max size
    //     let serializable = Serializable(message.amqp_message.clone());
    //     let message_size = serialized_size(&serializable).unwrap();
    //     assert!(message_size > 1024);

    //     assert!(batch.try_add(message).is_err());
    // }

    // #[test]
    // fn try_add_accepts_message_until_batch_is_full() {
    //     let max_size_in_bytes = 1024;
    //     let overhead = match max_size_in_bytes > MAXIMUM_BYTES_SMALL_MESSAGE as u64 {
    //         true => OVERHEAD_BYTES_LARGE_MESSAGE,
    //         false => OVERHEAD_BYTES_SMALL_MESSAGE,
    //     };
    //     let mut batch = AmqpEventBatch::new(max_size_in_bytes);

    //     let message = Event::new("hello world");

    //     let mut cumulated_size_in_bytes = 0;
    //     loop {
    //         let serializable = Serializable(message.amqp_message.clone());
    //         let message_size = serialized_size(&serializable).unwrap();
    //         cumulated_size_in_bytes += message_size;
    //         if (cumulated_size_in_bytes + overhead) as u64 > max_size_in_bytes {
    //             break;
    //         }

    //         assert!(batch.try_add(message.clone()).is_ok());
    //     }

    //     assert!(batch.try_add(message).is_err());
    // }

    // #[test]
    // fn iter_returns_iterator_over_added_messages() {
    //     let mut batch = AmqpEventBatch::new(1024);

    //     let events: Vec<_> = (0..5)
    //         .map(|i| Event::new(format!("message {}", i)))
    //         .collect();
    //     for message in events.iter() {
    //         assert!(batch.try_add(message.clone()).is_ok());
    //     }

    //     let iter = batch.iter();
    //     for (original, added) in events.into_iter().zip(iter) {
    //         assert_eq!(original.amqp_message, *added);
    //     }
    // }

    // #[test]
    // fn clear_resets_batch_len_and_size_in_bytes() {
    //     let mut batch = AmqpEventBatch::new(1024);
    //     let message = Event::new("hello world");

    //     assert!(batch.try_add(message.clone()).is_ok());
    //     assert_eq!(batch.len(), 1);
    //     assert!(batch.size_in_bytes > 0);

    //     batch.clear();
    //     assert_eq!(batch.len(), 0);
    //     assert_eq!(batch.size_in_bytes, 0);
    // }
}
