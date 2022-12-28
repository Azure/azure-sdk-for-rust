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

/// Builds a batch from a set of messages. Returns the batch containing the source messages.
///
/// If `force_batch` is set to true, then a batch will be created even if there is only one message.
pub(crate) fn build_amqp_batch_from_messages(
    mut source: impl Iterator<Item = Message<Data>> + ExactSizeIterator,
    force_batch: bool,
) -> Option<BatchEnvelope> {
    let total = source.len();

    match (total, force_batch) {
        (0, _) => None,
        (1, false) => {
            let mut message = source.next()?;
            generate_message_id_if_not_present(&mut message); // TODO: temp workaround for duplicate detection
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

            let mut first_message = source.next()?;
            generate_message_id_if_not_present(&mut first_message); // TODO: temp workaround for duplicate detection

            // Take selected fields from the first message properties and message annotations and
            // use it as the basis for the evelope
            //
            // TODO: should AmqpMessageBatch also follow the same pattern? (where first message's
            // metadata is used as the basis for the batch)
            let properties = first_message.properties.clone();
            let message_annotations = first_message.message_annotations.clone();

            let data = Data::from(to_vec(&Serializable(first_message)).ok()?);
            batch_data.push(data);

            for mut message in source {
                generate_message_id_if_not_present(&mut message); // TODO: temp workaround for duplicate detection
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

/// Generates a message id if one is not present.
///
/// TODO: This is a workaround to allow the service to perform duplicate detection. The current
/// retry policy would make retrying a message as if it were a new message.
///
/// Upon establishing a sender link, the service will have `rcv_settle_mode` set to `First` which
/// settles the message without needing a disposition from the sender.
fn generate_message_id_if_not_present(message: &mut Message<Data>) {
    use super::amqp_message_extensions::{AmqpMessageExt, AmqpMessageMutExt};

    if message.message_id().is_none() {
        // UUID length won't exceed MAX_MESSAGE_ID_LENGTH
        let message_id = uuid::Uuid::new_v4().to_string();
        println!("Generated message id: {}", message_id);
        let _ = message.set_message_id(message_id);
    }
}
