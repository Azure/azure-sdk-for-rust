use fe2o3_amqp::{
    link::{delivery::DeliveryFut, SendError},
    Sendable,
};
use fe2o3_amqp_types::messaging::{
    message::__private::Serializable, Batch, Data, Message, Outcome,
};
use serde_amqp::to_vec;

use crate::ServiceBusMessage;

use super::amqp_constants;

pub(crate) const LOCK_TOKEN_DELIVERY_ANNOTATION: &str = "x-opt-lock-token";

/// State of a batch envelope. Delivery is not considered complete until the envelope is settled.
///
/// The AMQP sender will keep the message in its internal unsettled map until the delivery is
/// settled by the receiver. All unsettled messages will be resent upon re-attaching the sender.
/// This is why a sent but not settled message should not be retried.
pub(crate) enum BatchEnvelopeState {
    /// The envelope has not been sent yet.
    NotSent,

    /// The envelope has been sent but not settled by the receiver.
    Sent(DeliveryFut<Result<Outcome, SendError>>),

    /// The envelope has been settled by the receiver.
    Settled,
}

impl std::fmt::Debug for BatchEnvelopeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatchEnvelopeState::NotSent => write!(f, "NotSent"),
            BatchEnvelopeState::Sent(_) => write!(f, "Sent"),
            BatchEnvelopeState::Settled => write!(f, "Settled"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum SendableEnvelope {
    Single(Sendable<Data>),
    Batch(Sendable<Batch<Data>>),
}

#[derive(Debug)]
pub(crate) struct BatchEnvelope {
    pub state: BatchEnvelopeState,
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
            let message = source.next()?;
            let sendable = Sendable {
                message,
                message_format: Default::default(),
                settled: Default::default(),
            };
            Some(BatchEnvelope {
                state: BatchEnvelopeState::NotSent,
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

            for message in source {
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
                state: BatchEnvelopeState::NotSent,
                sendable: SendableEnvelope::Batch(sendable),
            })
        }
    }
}
