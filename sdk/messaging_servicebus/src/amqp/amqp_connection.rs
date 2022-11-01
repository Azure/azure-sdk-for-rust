use super::CONNECTION_IDENTIFIER;

pub(crate) struct AmqpConnection {
    pub identifier: u32,
    pub handle: fe2o3_amqp::connection::ConnectionHandle<()>,
}

impl AmqpConnection {
    pub(crate) fn new(handle: fe2o3_amqp::connection::ConnectionHandle<()>) -> Self {
        Self {
            identifier: CONNECTION_IDENTIFIER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            handle,
        }
    }
}
