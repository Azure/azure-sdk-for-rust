use fe2o3_amqp::{session::SessionHandle, Receiver};

use crate::core::transport_consumer::TransportConsumer;

pub struct AmqpConsumer {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) receiver: Receiver,
    pub(crate) link_identifier: u32,
    pub(crate) invalidate_consumer_when_partition_stolen: bool,
}

impl TransportConsumer for AmqpConsumer {

}
