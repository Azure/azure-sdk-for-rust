use fe2o3_amqp::{connection::ConnectionHandle};

pub(crate) struct AmqpConnection {
    inner: ConnectionHandle<()>,
}
