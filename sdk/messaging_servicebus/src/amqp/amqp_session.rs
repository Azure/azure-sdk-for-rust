use super::SESSION_IDENTIFIER;

pub(crate) struct AmqpSession {
    pub identifier: u32,
    pub handle: fe2o3_amqp::session::SessionHandle<()>,
}

impl AmqpSession {
    pub(crate) fn new(handle: fe2o3_amqp::session::SessionHandle<()>) -> Self {
        Self {
            identifier: SESSION_IDENTIFIER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            handle,
        }
    }
}
