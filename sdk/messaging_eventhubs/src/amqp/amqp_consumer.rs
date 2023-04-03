use fe2o3_amqp::{session::SessionHandle, Receiver};

pub(crate) struct AmqpConsumer {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) receiver: Receiver,
    pub(crate) link_identifier: u32,
}
