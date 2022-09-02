use std::time::Duration;

use fe2o3_amqp_types::{
    messaging::{annotations::AnnotationKey, Body, Data, Message},
    primitives::{Timestamp, Value},
};
use time::OffsetDateTime;

use crate::{primitives::service_bus_message::AmqpMessage, ServiceBusMessage};

use super::{
    amqp_message_constants,
    error::{not_supported_error, Error},
};

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
        message
            .message_annotations
            .as_ref()?
            .get(&amqp_message_constants::PARTITION_KEY_NAME as &dyn AnnotationKey)
            .and_then(|value| match value {
                Value::String(s) => Some(s.as_str()),
                _ => None,
            })
    }

    pub fn get_via_partition_key<T>(message: &Message<T>) -> Option<&str> {
        message
            .message_annotations
            .as_ref()?
            .get(&amqp_message_constants::VIA_PARTITION_KEY_NAME as &dyn AnnotationKey)
            .and_then(|value| match value {
                Value::String(s) => Some(s.as_str()),
                _ => None,
            })
    }

    pub fn get_time_to_live<T>(message: &Message<T>) -> Option<Duration> {
        message
            .header
            .as_ref()
            .and_then(|h| h.ttl)
            .map(|millis| Duration::from_millis(millis as u64))
    }

    pub fn get_scheduled_enqueue_time<T>(message: &Message<T>) -> Option<OffsetDateTime> {
        message
            .message_annotations
            .as_ref()?
            .get(&amqp_message_constants::SCHEDULED_ENQUEUE_TIME_UTC_NAME as &dyn AnnotationKey)
            .and_then(|value| match value {
                Value::Timestamp(timestamp) => {
                    let millis = timestamp.milliseconds();
                    if millis >= 0 {
                        let duration = Duration::from_millis(millis as u64);
                        Some(OffsetDateTime::UNIX_EPOCH + duration)
                    } else {
                        let duration = Duration::from_millis((-millis) as u64);
                        Some(OffsetDateTime::UNIX_EPOCH - duration)
                    }
                }
                _ => None,
            })
    }

    pub fn get_body<T>(message: &Message<T>) -> Result<&[u8], Error> {
        match &message.body {
            Body::Data(Data(buf)) => Ok(buf),
            Body::Sequence(_) => Err(not_supported_error(
                "Body::Sequence",
                "body()",
                "raw_amqp_message()",
            )),
            Body::Value(_) => Err(not_supported_error(
                "Body::Value",
                "body()",
                "raw_amqp_message()",
            )),
            Body::Empty => Err(not_supported_error(
                "Body::Empty",
                "body()",
                "raw_amqp_message()",
            )),
        }
    }
}
