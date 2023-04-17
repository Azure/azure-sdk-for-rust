//! A wrapper around a vector of consumers.

use super::amqp_consumer::AmqpConsumer;

pub(crate) struct AmqpConsumerVec<RP> {
    consumers: Vec<AmqpConsumer<RP>>,
}
