use fe2o3_amqp::{Delivery, Sendable};
use fe2o3_amqp_types::{
    messaging::{
        annotations::{AnnotationKey, BorrowedKey},
        message::__private::Serializable,
        Batch, Body, Data, Message, MessageAnnotations, Properties,
    },
    primitives::SymbolRef,
};
use serde_amqp::{to_vec, Value};

use crate::{
    primitives::service_bus_received_message::ServiceBusReceivedMessage, ServiceBusMessage,
};

use super::{amqp_constants, amqp_message_constants};

/// <summary>
/// The size, in bytes, to use for extracting the delivery tag bytes into <see cref="Guid"/>.
/// </summary>
const GUID_SIZE_IN_BYTES: usize = 16;

/// <summary>The size, in bytes, to use as a buffer for stream operations.</summary>
const STREAM_BUFFER_SIZE_IN_BYTES: usize = 512;

const LOCK_TOKEN_DELIVERY_ANNOTATION: &str = "x-opt-lock-token";

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
pub(crate) fn build_amqp_batch_from_messages(
    mut source: impl Iterator<Item = Message<Data>> + ExactSizeIterator,
    force_batch: bool,
) -> Option<BatchEnvelope> {
    let total = source.len();

    match (total, force_batch) {
        (0, _) => None,
        (1, false) => {
            let mut message = source.next()?;
            message.properties = message.properties.map(filter_properties);
            message.message_annotations =
                message.message_annotations.map(filter_message_annotation);
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
            let properties = first_message.properties.clone().map(filter_properties);
            let message_annotations = first_message
                .message_annotations
                .clone()
                .map(filter_message_annotation);

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

pub(crate) struct InvalidLockTokenError {}

pub(crate) fn amqp_delivery_as_service_bus_received_message(
    delivery: Delivery<Body<Value>>,
    is_settled: bool,
) -> Result<ServiceBusReceivedMessage, InvalidLockTokenError> {
    use fe2o3_amqp::types::primitives::Uuid;

    let delivery_tag = delivery.delivery_tag().as_ref();
    let lock_token_uuid: fe2o3_amqp::types::primitives::Uuid = match Uuid::try_from(delivery_tag) {
        Ok(uuid) => uuid,
        Err(_) => match delivery
            .message()
            .delivery_annotations
            .as_ref()
            .and_then(|da| da.get(&LOCK_TOKEN_DELIVERY_ANNOTATION as &dyn AnnotationKey))
        {
            Some(Value::Uuid(uuid)) => uuid.clone(),
            _ => return Err(InvalidLockTokenError {}),
        },
    };

    Ok(ServiceBusReceivedMessage {
        is_settled,
        delivery,
        lock_token_uuid,
    })
}
