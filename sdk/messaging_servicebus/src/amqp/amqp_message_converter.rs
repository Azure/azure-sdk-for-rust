use fe2o3_amqp::Sendable;
use fe2o3_amqp_types::{
    messaging::{
        annotations::{AnnotationKey, BorrowedKey},
        Batch, Data, IntoBody, Message, MessageAnnotations, Properties,
    },
    primitives::{OrderedMap, SymbolRef},
};

use crate::ServiceBusMessage;

use super::{amqp_constants, amqp_message_constants};

/// <summary>
/// The size, in bytes, to use for extracting the delivery tag bytes into <see cref="Guid"/>.
/// </summary>
const GUID_SIZE_IN_BYTES: usize = 16;

/// <summary>The size, in bytes, to use as a buffer for stream operations.</summary>
const STREAM_BUFFER_SIZE_IN_BYTES: usize = 512;

pub(crate) enum SendableEnvelope {
    Single(Sendable<Data>),
    Batch(Sendable<Batch<Data>>),
}

pub(crate) struct BatchEnvelope {
    batchable: bool,
    sendable: SendableEnvelope,
}

#[inline]
pub(crate) fn batch_service_bus_messages_as_amqp_message(
    source: impl Iterator<Item = ServiceBusMessage> + ExactSizeIterator,
    force_batch: Option<bool>,
) -> Option<BatchEnvelope> {
    let batch_messages = source.map(|m| m.amqp_message);
    build_amqp_batch_from_messages(batch_messages, force_batch.unwrap_or(false))
}

fn filter_properties(properties: Properties) -> Properties {
    Properties {
        message_id: properties.message_id,
        group_id: properties.group_id,
        ..Default::default()
    }
}

fn filter_message_annotation(message_annotations: MessageAnnotations) -> MessageAnnotations {
    let map = message_annotations
        .0
        .into_iter()
        .filter(|(k, _)| {
            k.key()
                == BorrowedKey::Symbol(SymbolRef::from(amqp_message_constants::PARTITION_KEY_NAME))
                || k.key()
                    == BorrowedKey::Symbol(SymbolRef::from(
                        amqp_message_constants::VIA_PARTITION_KEY_NAME,
                    ))
        })
        .collect();
    MessageAnnotations(map)
}

/// <summary>
///   Builds a batch <see cref="AmqpMessage" /> from a set of <see cref="AmqpMessage" />.
/// </summary>
///
/// <param name="source">The set of messages to use as the body of the batch message.</param>
/// <param name="forceBatch">Set to true to force creating as a batch even when only one message.</param>
///
/// <returns>The batch <see cref="AmqpMessage" /> containing the source messages.</returns>
///
fn build_amqp_batch_from_messages(
    mut source: impl Iterator<Item = Message<Data>> + ExactSizeIterator,
    force_batch: bool,
) -> Option<BatchEnvelope> {
    let total = source.len();

    match (total, force_batch) {
        (0, _) => None,
        (1, false) => source
            .next()
            .map(|mut map| {
                map.properties = map.properties.map(filter_properties);
                map.message_annotations = map.message_annotations.map(filter_message_annotation);
                Sendable::from(map)
            })
            .map(|s| BatchEnvelope {
                batchable: false,
                sendable: SendableEnvelope::Single(s),
            }),
        _ => {
            let mut batch_data: Batch<Data> = Batch::from(Vec::with_capacity(total));

            let first_message = source.next()?;
            batch_data.push(first_message.body);

            while let Some(message) = source.next() {
                batch_data.push(message.body);
            }

            let properties = first_message.properties.map(filter_properties);

            let message_annotations = first_message
                .message_annotations
                .map(filter_message_annotation);

            let message_envelop = Message::builder()
                .data_batch(batch_data)
                .properties(properties)
                .message_annotations(message_annotations)
                .build();

            let sendable = Sendable::builder()
                .message(message_envelop)
                .message_format(amqp_constants::AMQP_BATCHED_MESSAGE_FORMAT)
                .build();
            Some(BatchEnvelope {
                batchable: true,
                sendable: SendableEnvelope::Batch(sendable),
            })
        }
    }
}
