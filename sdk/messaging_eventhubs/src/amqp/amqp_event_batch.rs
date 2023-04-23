use fe2o3_amqp_types::messaging::{Batch, Data, Message};

use crate::{core::TransportEventBatch, EventData};

use super::{
    amqp_message_converter::create_empty_phantom_envelope,
    amqp_phantom_message::{Phantom, PhantomMessage},
    error::TryAddError,
};

/// A set of [`EventData`] with size constraints known up-front, intended to be
/// sent to the Queue/Topic as a single batch. A [`EventDataBatch`] can be
/// created using `ServiceBusSender::create_message_batch()`.
/// Messages can be added to the batch using the [`try_add`] method on the batch.
#[derive(Debug, Clone)]
pub struct AmqpEventBatch {
    /// The maximum size of the batch, in bytes.
    pub(crate) max_size_in_bytes: u64,
    /// The list of events that will be sent as a batch.
    pub(crate) events: Vec<Message<Data>>,

    pub(crate) phantom_envelope: PhantomMessage<Batch<Data>>,
}

impl AmqpEventBatch {
    pub(crate) fn new(
        max_size_in_bytes: u64,
        partition_key: Option<String>,
    ) -> Result<Self, serde_amqp::Error> {
        let phantom_envelope = create_empty_phantom_envelope(partition_key)?;

        Ok(Self {
            max_size_in_bytes,
            events: Vec::new(),
            phantom_envelope,
        })
    }
}

impl TransportEventBatch for AmqpEventBatch {
    type TryAddError = TryAddError;

    type Iter<'a> = std::slice::Iter<'a, Message<Data>>;

    fn max_size_in_bytes(&self) -> u64 {
        self.max_size_in_bytes
    }

    fn size_in_bytes(&self) -> u64 {
        // usize will be either u32 or u64, so converting to u64 should be safe.
        self.phantom_envelope.serialized_size() as u64
    }

    fn len(&self) -> usize {
        self.events.len()
    }

    fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    fn try_add(&mut self, event: EventData) -> Result<(), Self::TryAddError> {
        let phantom_event = match Phantom::try_from(&event.amqp_message) {
            Ok(phantom_event) => phantom_event,
            Err(err) => return Err(TryAddError::Codec { source: err, event }),
        };
        self.phantom_envelope.body.push(phantom_event);

        if self.phantom_envelope.serialized_size() as u64 > self.max_size_in_bytes {
            // reset the size back to the previous value
            self.phantom_envelope.body.pop(phantom_event);
            Err(Self::TryAddError::BatchFull(event))
        } else {
            self.events.push(event.amqp_message);
            Ok(())
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.events.iter()
    }

    fn clear(&mut self) {
        self.events.clear();
        self.phantom_envelope.body.clear();
    }
}

#[cfg(test)]
mod tests {

    use bytes::{BufMut, BytesMut};
    use fe2o3_amqp_types::messaging::message::__private::Serializable;
    use serde_amqp::{ser::Serializer, serde::Serialize, serialized_size};

    use crate::{
        amqp::amqp_message_converter::{build_amqp_batch_from_messages, SendableEnvelope},
        producer::CreateBatchOptions,
    };

    use super::*;

    /// The amount of bytes to reserve as overhead for a small message.
    const OVERHEAD_BYTES_SMALL_MESSAGE: usize = 5;

    #[test]
    fn new_sets_max_size_in_bytes() {
        let options = CreateBatchOptions::default();
        let batch = AmqpEventBatch::new(1024, options.partition_key).unwrap();
        assert_eq!(batch.max_size_in_bytes, 1024);
    }

    #[test]
    fn try_add_sets_batch_size_in_bytes() {
        let options = CreateBatchOptions::default();
        let overhead = OVERHEAD_BYTES_SMALL_MESSAGE; // The events added are small
        let mut batch = AmqpEventBatch::new(1024, options.partition_key).unwrap();
        let message = EventData::new("hello world");

        let serializable = Serializable(&message.amqp_message);
        let size = serialized_size(&serializable).unwrap();

        assert!(size * 2 < 1024);

        assert!(batch.try_add(message.clone()).is_ok());
        assert_eq!(batch.size_in_bytes(), (size + overhead) as u64);

        assert!(batch.try_add(message).is_ok());
        assert_eq!(batch.size_in_bytes(), 2 * (size + overhead) as u64);
    }

    #[test]
    fn serialized_size_matches() {
        let options = CreateBatchOptions::default();
        let mut batch = AmqpEventBatch::new(262144, options.partition_key).unwrap();
        let event = "abcdefg";

        while let Ok(_) = batch.try_add(EventData::from(event)) {}
        let batch_size_in_bytes = batch.size_in_bytes();

        let batch = build_amqp_batch_from_messages(batch.events.into_iter(), None).unwrap();
        let (ssize, payload, _value) = match batch.sendable {
            SendableEnvelope::Single(sendable) => {
                let message = sendable.message;
                let serializable = Serializable(message);
                let ssize = serialized_size(&serializable).unwrap();
                let mut payload = BytesMut::new();
                let mut serializer = Serializer::from((&mut payload).writer());
                serializable.serialize(&mut serializer).unwrap();

                let value = serde_amqp::to_value(&serializable).unwrap();

                (ssize, payload.freeze(), value)
            }
            SendableEnvelope::Batch(sendable) => {
                let message = sendable.message;
                let serializable = Serializable(message);
                let ssize = serialized_size(&serializable).unwrap();
                let mut payload = BytesMut::new();
                let mut serializer = Serializer::from((&mut payload).writer());
                serializable.serialize(&mut serializer).unwrap();

                let value = serde_amqp::to_value(&serializable).unwrap();

                (ssize, payload.freeze(), value)
            }
        };

        assert_eq!(payload.len(), ssize);
        assert_eq!(ssize as u64, batch_size_in_bytes);
    }

    #[test]
    fn try_add_accepts_message_smaller_than_max_size() {
        let mut batch = AmqpEventBatch::new(1024, None).unwrap();
        let message = EventData::new("hello world");

        // Make sure the message is smaller than the max size
        let serializable = Serializable(message.amqp_message.clone());
        let message_size = serialized_size(&serializable).unwrap();
        assert!(message_size < 1024);

        assert!(batch.try_add(message).is_ok());
    }

    #[test]
    fn try_add_does_not_accept_message_larger_than_max_size() {
        let mut batch = AmqpEventBatch::new(1024, None).unwrap();
        let message = EventData::new(vec![0u8; 1025]);

        // Make sure the message is larger than the max size
        let serializable = Serializable(message.amqp_message.clone());
        let message_size = serialized_size(&serializable).unwrap();
        assert!(message_size > 1024);

        assert!(batch.try_add(message).is_err());
    }

    #[test]
    fn try_add_accepts_message_until_batch_is_full() {
        let max_size_in_bytes = 1024;
        let data = "abcdefg";
        let event = EventData::from(data);
        let mut batch = AmqpEventBatch::new(max_size_in_bytes, None).unwrap();

        while let Ok(_) = batch.try_add(event.clone()) {}

        assert!(batch.try_add(event).is_err());
    }

    #[test]
    fn iter_returns_iterator_over_added_messages() {
        let mut batch = AmqpEventBatch::new(1024, None).unwrap();

        let events: Vec<_> = (0..5)
            .map(|i| EventData::new(format!("message {}", i)))
            .collect();
        for message in events.iter() {
            assert!(batch.try_add(message.clone()).is_ok());
        }

        let iter = batch.iter();
        for (original, added) in events.into_iter().zip(iter) {
            assert_eq!(original.amqp_message, *added);
        }
    }

    #[test]
    fn clear_resets_batch_len_and_size_in_bytes() {
        let mut batch = AmqpEventBatch::new(1024, None).unwrap();
        let message = EventData::new("hello world");

        assert!(batch.try_add(message.clone()).is_ok());
        assert_eq!(batch.len(), 1);
        assert!(batch.size_in_bytes() > 0);

        batch.clear();
        assert_eq!(batch.len(), 0);
        assert_eq!(batch.size_in_bytes(), 0);
    }
}
