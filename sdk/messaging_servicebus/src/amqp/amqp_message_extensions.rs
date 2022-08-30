use std::time::Duration;

use fe2o3_amqp_types::messaging::{Data, Message};
use time::OffsetDateTime;

use crate::{primitives::service_bus_message::AmqpMessage, ServiceBusMessage};

pub struct AmqpMessageExtensions {}

impl AmqpMessageExtensions {
    pub fn to_amqp_message(message: ServiceBusMessage) -> AmqpMessage {
        todo!()
    }

    // TODO: returns `impl Iterator<Item = Data>`
    pub fn as_amqp_data(binary_data: impl Iterator<Item = Vec<u8>>) -> Vec<Data> {
        todo!()
    }

    pub fn get_partition_key<T>(message: &Message<T>) -> Option<&str> {
        todo!()
    }

    pub fn get_via_partition_key<T>(message: &Message<T>) -> Option<&str> {
        todo!()
    }

    pub fn get_time_to_live<T>(message: &Message<T>) -> Option<Duration> {
        todo!()
    }

    pub fn get_scheduled_enqueue_time<T>(message: &Message<T>) -> Option<OffsetDateTime> {
        todo!()
    }

    pub fn get_body<T>(message: &Message<T>) -> Option<&[u8]> {
        todo!()
    }
}
