use crate::{amqp::amqp_receiver::AmqpReceiver, core::TransportReceiver};

pub(crate) enum InnerReceiver {
    Amqp(AmqpReceiver),
}

impl TransportReceiver for InnerReceiver {}
