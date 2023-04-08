use std::collections::HashMap;

use crate::{EventHubConnection, amqp::{amqp_client::AmqpClient, amqp_producer::AmqpProducer}};

pub struct EventHubProducerClient {
    connection: EventHubConnection<AmqpClient>,
}


