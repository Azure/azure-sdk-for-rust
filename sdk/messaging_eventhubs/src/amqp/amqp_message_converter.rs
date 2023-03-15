use fe2o3_amqp_types::messaging::{Message, Data};

use crate::Event;

use super::amqp_property;

use fe2o3_amqp::{
    link::{delivery::DeliveryFut, SendError},
    Sendable,
};
use fe2o3_amqp_types::messaging::{
    message::__private::Serializable, Batch, Outcome,
};
use serde_amqp::to_vec;

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

pub(crate) fn create_envelope_from_event(event: Event, partition_key: Option<String>) -> BatchEnvelope {
    let message = build_amqp_message_from_event(event, partition_key);
    let sendable = Sendable {
        message,
        message_format: Default::default(),
        settled: Default::default(),
    };

    BatchEnvelope {
        state: BatchEnvelopeState::NotSent,
        sendable: SendableEnvelope::Single(sendable),
    }
}

pub(crate) fn create_envelope_from_events(events: impl Iterator<Item = Event> + ExactSizeIterator, partition_key: Option<String>) -> Option<BatchEnvelope> {
    build_amqp_batch_from_events(events, partition_key)
}

#[inline]
fn build_amqp_message_from_event(event: Event, partition_key: Option<String>) -> Message<Data> {
    let mut message = event.amqp_message;

    // add partition key to message annotation
    if let Some(partition_key) = partition_key {
        if !partition_key.is_empty() {
            message.message_annotations
                .get_or_insert(Default::default())
                .insert(amqp_property::PARTITION_KEY.into(), partition_key.into());
        }
    }

    message
}

#[inline]
fn build_amqp_batch_from_events(events: impl Iterator<Item = Event> + ExactSizeIterator, partition_key: Option<String>) -> Option<BatchEnvelope> {
    let partition_key_clone = partition_key.clone();
    let messages = events.map(|event| build_amqp_message_from_event(event, partition_key.clone()));
    build_amqp_batch_from_messages(messages, partition_key_clone)
}


/// Builds a batch from a set of messages. Returns the batch containing the source messages.
///
/// If `force_batch` is set to true, then a batch will be created even if there is only one message.
fn build_amqp_batch_from_messages(
    mut source: impl Iterator<Item = Message<Data>> + ExactSizeIterator,
    partition_key: Option<String>
) -> Option<BatchEnvelope> {
    let total = source.len();

    match total {
        0 => None,
        1 => {
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

            for message in source {
                // No need to set each message as batchable, as the field batchable is in the Transfer
                // performative
                let data = Data::from(to_vec(&Serializable(message)).ok()?);
                batch_data.push(data);
            }

            let mut envelope = Message::builder()
                .body(batch_data)
                .build();

            if let Some(partition_key) = partition_key {
                if !partition_key.is_empty() {
                    envelope.message_annotations
                        .get_or_insert(Default::default())
                        .insert(amqp_property::PARTITION_KEY.into(), partition_key.into());
                }
            }

            let sendable = Sendable::builder()
                .message(envelope)
                .message_format(amqp_constants::AMQP_BATCHED_MESSAGE_FORMAT)
                .build();

            Some(BatchEnvelope {
                state: BatchEnvelopeState::NotSent,
                sendable: SendableEnvelope::Batch(sendable),
            })
        }
    }
}
