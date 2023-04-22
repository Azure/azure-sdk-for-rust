use fe2o3_amqp_types::messaging::{message::__private::Serializable, Data, Message};
use serde_amqp::serialized_size;

use crate::{core::TransportEventBatch, producer::create_batch_options::CreateBatchOptions, EventData};

use super::{
    amqp_message_converter::{build_amqp_batch_from_messages, SendableEnvelope},
    error::TryAddError,
};

/// A set of [`EventData`] with size constraints known up-front, intended to be
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

    /// TODO: A phantom data type should probably be implemented to test the serialized data size
    /// in order to reduce number of clones.
    fn try_add(&mut self, event: EventData) -> Result<(), Self::TryAddError> {
        let EventData {amqp_message} = event.clone();
        self.events.push(amqp_message);

        let envelope = match build_amqp_batch_from_messages(
            self.events.iter().cloned(),
            self.options.partition_key.clone(),
        ) {
            Some(envelope) => envelope,
            None => unreachable!(),
        };

        let result = match envelope.sendable {
            SendableEnvelope::Single(sendable) => {
                serialized_size(&Serializable(sendable.message))
            }
            SendableEnvelope::Batch(sendable) => {
                serialized_size(&Serializable(sendable.message))
            }
        };
        let serialized_size = match result {
            Ok(size) => size as u64,
            Err(err) => return Err(Self::TryAddError::Codec { source: err, event }),
        };

        if serialized_size > self.max_size_in_bytes {
            let _ = self.events.pop();
            Err(Self::TryAddError::BatchFull(event))
        } else {
            self.size_in_bytes = serialized_size;
            Ok(())
        }
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

    use bytes::{BytesMut, BufMut};
    use serde_amqp::{ser::Serializer, serde::Serialize};

    use crate::amqp::amqp_message_converter::BatchEnvelope;

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

    #[test]
    fn try_add_sets_batch_size_in_bytes() {
        let options = CreateBatchOptions::default();
        let overhead = OVERHEAD_BYTES_SMALL_MESSAGE; // The events added are small
        let mut batch = AmqpEventBatch::new(1024, options);
        let message = EventData::new("hello world");

        let serializable = Serializable(&message.amqp_message);
        let size = serialized_size(&serializable).unwrap();
        println!("size: {}", size);

        assert!(size * 2 < 1024);

        assert!(batch.try_add(message.clone()).is_ok());
        assert_eq!(batch.size_in_bytes, (size + overhead) as u64);

        assert!(batch.try_add(message).is_ok());
        assert_eq!(batch.size_in_bytes, (size * 2 + overhead) as u64);
    }

    #[test]
    fn serialized_size_matches() {
        let options = CreateBatchOptions::default();
        let mut batch = AmqpEventBatch::new(64, options);
        let event = "abcdefg";

        while let Ok(_) = batch.try_add(EventData::from(event)) {}

        println!("batch size: {}", batch.size_in_bytes());

        let batch = build_amqp_batch_from_messages(batch.events.into_iter(), None).unwrap();
        let (ssize, payload, value) = match batch.sendable {
            SendableEnvelope::Single(sendable) => {
                let message = sendable.message;
                let serializable=  Serializable(message);
                let ssize = serialized_size(&serializable).unwrap();
                let mut payload = BytesMut::new();
                let mut serializer = Serializer::from((&mut payload).writer());
                serializable.serialize(&mut serializer).unwrap();

                let value = serde_amqp::to_value(&serializable).unwrap();

                (ssize, payload.freeze(), value)
            },
            SendableEnvelope::Batch(sendable) => {
                let message = sendable.message;
                let serializable=  Serializable(message);
                let ssize = serialized_size(&serializable).unwrap();
                let mut payload = BytesMut::new();
                let mut serializer = Serializer::from((&mut payload).writer());
                serializable.serialize(&mut serializer).unwrap();

                let value = serde_amqp::to_value(&serializable).unwrap();

                (ssize, payload.freeze(), value)
            },
        };
        println!("ssize: {}", ssize);
        println!("payload size: {}", payload.len());
        println!("value: {:?}", value);
    }

    // #[test]
    // fn try_add_accepts_message_smaller_than_max_size() {
    //     let mut batch = AmqpEventBatch::new(1024);
    //     let message = EventData::new("hello world");

    //     // Make sure the message is smaller than the max size
    //     let serializable = Serializable(message.amqp_message.clone());
    //     let message_size = serialized_size(&serializable).unwrap();
    //     assert!(message_size < 1024);

    //     assert!(batch.try_add(message).is_ok());
    // }

    // #[test]
    // fn try_add_does_not_accept_message_larger_than_max_size() {
    //     let mut batch = AmqpEventBatch::new(1024);
    //     let message = EventData::new(vec![0u8; 1025]);

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

    //     let message = EventData::new("hello world");

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
    //         .map(|i| EventData::new(format!("message {}", i)))
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
    //     let message = EventData::new("hello world");

    //     assert!(batch.try_add(message.clone()).is_ok());
    //     assert_eq!(batch.len(), 1);
    //     assert!(batch.size_in_bytes > 0);

    //     batch.clear();
    //     assert_eq!(batch.len(), 0);
    //     assert_eq!(batch.size_in_bytes, 0);
    // }
}
