use fe2o3_amqp::{session::SessionHandle, Sender};

pub(crate) struct ProducerLink {
    session_handle: SessionHandle<()>,
    session_identifier: u32,
    sender: Sender,
    link_identifier: u32,
}
