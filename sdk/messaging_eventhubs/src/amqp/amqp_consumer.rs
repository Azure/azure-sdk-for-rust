use fe2o3_amqp::{session::SessionHandle, Receiver};

use crate::core::transport_consumer::TransportConsumer;

pub(crate) struct AmqpConsumer {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) receiver: Receiver,
    pub(crate) link_identifier: u32,
}

impl TransportConsumer for AmqpConsumer {

}
