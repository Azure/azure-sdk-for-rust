use fe2o3_amqp::{session::SessionHandle, Sender};

pub(crate) struct AmqpProducer {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) sender: Sender,
    pub(crate) link_identifier: u32,
}
