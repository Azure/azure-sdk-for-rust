use fe2o3_amqp::Sendable;
use fe2o3_amqp_types::messaging::{message::__private::Serializable, Batch, Data, Message};
use serde_amqp::to_vec;

use crate::ServiceBusMessage;

use super::amqp_constants;

pub(crate) const LOCK_TOKEN_DELIVERY_ANNOTATION: &str = "x-opt-lock-token";

#[derive(Debug, Clone)]
pub(crate) enum SendableEnvelope {
    Single(Sendable<Data>),
    Batch(Sendable<Batch<Data>>),
}

#[derive(Debug, Clone)]
pub(crate) struct BatchEnvelope {
    pub batchable: bool,
    pub sendable: SendableEnvelope,
}

#[inline]
pub(crate) fn batch_service_bus_messages_as_amqp_message(
    source: impl Iterator<Item = ServiceBusMessage> + ExactSizeIterator,
    force_batch: bool,
) -> Option<BatchEnvelope> {
    let batch_messages = source.map(|m| m.amqp_message);
    build_amqp_batch_from_messages(batch_messages, force_batch)
}

/// Builds a batch from a set of messages.
///
/// # Parameters
///
/// * `source` - The set of messages to use as the body of the batch message.
/// * `force_batch` - Set to true to force creating as a batch even when only one message.
///
/// # Returns
///
/// The batch containing the source messages
pub(crate) fn build_amqp_batch_from_messages(
    mut source: impl Iterator<Item = Message<Data>> + ExactSizeIterator,
    force_batch: bool,
) -> Option<BatchEnvelope> {
    let total = source.len();

    match (total, force_batch) {
        (0, _) => None,
        (1, false) => {
            let mut message = source.next()?;
            message.properties = message.properties;
            let sendable = Sendable {
                message,
                message_format: Default::default(),
                settled: Default::default(),
            };
            Some(BatchEnvelope {
                batchable: false,
                sendable: SendableEnvelope::Single(sendable),
            })
        }
        _ => {
            let mut batch_data: Batch<Data> = Batch::from(Vec::with_capacity(total));

            let first_message = source.next()?;

            // Take selected fields from the first message properties and message annotations and
            // use it as the basis for the evelope
            let properties = first_message.properties.clone();
            let message_annotations = first_message.message_annotations.clone();

            let data = Data::from(to_vec(&Serializable(first_message)).ok()?);
            batch_data.push(data);

            while let Some(message) = source.next() {
                let data = Data::from(to_vec(&Serializable(message)).ok()?);
                batch_data.push(data);
            }

            let envelop = Message::builder()
                .body(batch_data)
                .properties(properties)
                .message_annotations(message_annotations)
                .build();

            let sendable = Sendable::builder()
                .message(envelop)
                .message_format(amqp_constants::AMQP_BATCHED_MESSAGE_FORMAT)
                .build();

            Some(BatchEnvelope {
                batchable: true,
                sendable: SendableEnvelope::Batch(sendable),
            })
        }
    }
}
