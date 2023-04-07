use crate::{EventHubConnection, amqp::amqp_client::AmqpClient};

pub struct EventHubProducerClient {
    connection: EventHubConnection<AmqpClient>,
}
