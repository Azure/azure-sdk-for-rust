use fe2o3_amqp::{session::SessionHandle, Receiver};

pub(crate) struct ConsumerLink {
    session_handle: SessionHandle<()>,
    receiver: Receiver,
    link_identifier: u32,
}
