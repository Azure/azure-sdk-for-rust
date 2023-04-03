use super::amqp_connection_scope::AmqpConnectionScope;

pub(crate) struct AmqpClient {
    connection_scope: AmqpConnectionScope,
}
