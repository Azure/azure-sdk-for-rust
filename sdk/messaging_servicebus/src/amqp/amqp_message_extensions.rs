use std::time::Duration;

use fe2o3_amqp_types::{
    messaging::{annotations::AnnotationKey, Body, Data, Message, MessageId},
    primitives::Value,
};
use time::OffsetDateTime;

use super::{
    amqp_message_constants,
    error::{not_supported_error, Error},
};

pub(crate) struct AmqpMessageExtensions {}

impl AmqpMessageExtensions {
    // /// TODO: is this really necessary?
    // pub fn to_amqp_message(message: ServiceBusMessage) -> AmqpMessage {
    //     message.amqp_message
    // }

    // /// TODO: returns `impl Iterator<Item = Data>`
    // ///
    // /// TODO: Multiple Data section is not supported yet
    // pub fn as_amqp_data(binary_data: impl Into<Vec<u8>>) -> Data {
    //     Data(Binary::from(binary_data))
    // }

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

    pub fn get_body_mut<T>(message: &mut Message<T>) -> Result<&mut [u8], Error> {
        match &mut message.body {
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

    // pub fn get_message_id<T>(message: &Message<T>) ->
}

pub(crate) trait AmqpMessageExt {
    fn body(&self) -> Result<&[u8], Error>;

    fn message_id(&self) -> Option<Result<&str, Error>>;

    fn partition_key(&self) -> Option<&str>;

    fn via_partition_key(&self) -> Option<&str>;

    fn session_id(&self) -> Option<&str>;

    fn reply_to_session_id(&self) -> Option<&str>;

    fn time_to_live(&self) -> Option<Duration>;

    fn correlation_id(&self) -> Option<Result<&str, Error>>;

    fn subject(&self) -> Option<&str>;

    fn to(&self) -> Option<&str>;

    fn content_type(&self) -> Option<&str>;

    fn reply_to(&self) -> Option<&str>;

    // TODO: `time::OffsetDateTime` doesn't implement `Default`
    fn scheduled_enqueue_time(&self) -> Option<Result<OffsetDateTime, Error>>;
}

pub(crate) trait AmqpMessageMutExt {
    fn body_mut(&mut self) -> Result<&mut [u8], Error>;

    fn set_body(&mut self, body: impl Into<Vec<u8>>);

    fn message_id_mut(&mut self) -> Option<Result<&mut str, Error>>;

    fn set_message_id(&mut self, message_id: impl Into<String>);

    fn partition_key_mut(&mut self) -> Option<&mut str>;

    fn set_partition_key(&mut self, key: impl Into<String>);

    fn via_partition_key_mut(&mut self) -> Option<&mut str>;

    fn set_via_partition_key(&mut self, key: impl Into<String>);

    fn session_id_mut(&mut self) -> Option<&mut str>;

    fn set_session_id(&mut self, session_id: impl Into<String>);

    fn reply_to_session_id_mut(&mut self) -> Option<&mut str>;

    fn set_reply_to_session_id(&mut self, id: impl Into<String>);

    fn set_time_to_live(&self, ttl: Duration);

    fn correlation_id_mut(&mut self) -> Option<Result<&mut str, Error>>;

    fn set_correlation_id(&mut self, id: impl Into<String>);

    fn subject_mut(&mut self) -> Option<&mut str>;

    fn set_subject(&mut self, subject: impl Into<String>);

    fn to_mut(&mut self) -> Option<&mut str>;

    fn set_to(&mut self, to: impl Into<String>);

    fn content_type_mut(&mut self) -> Option<&mut str>;

    fn set_content_type(&mut self, content_type: impl Into<String>);

    fn reply_to_mut(&mut self) -> Option<&mut str>;

    fn set_reply_to(&mut self, reply_to: impl Into<String>);
}

impl<T> AmqpMessageExt for Message<T> {
    #[inline]
    fn body(&self) -> Result<&[u8], Error> {
        match &self.body {
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

    #[inline]
    fn message_id(&self) -> Option<Result<&str, Error>> {
        match self.properties.as_ref()?.message_id.as_ref()? {
            MessageId::String(val) => Some(Ok(val)),
            MessageId::ULong(_) => Some(Err(not_supported_error(
                "MessageId::ULong",
                "message_id()",
                "raw_amqp_message()",
            ))),
            MessageId::Uuid(_) => Some(Err(not_supported_error(
                "MessageId::Uuid",
                "message_id()",
                "raw_amqp_message()",
            ))),
            MessageId::Binary(_) => Some(Err(not_supported_error(
                "MessageId::Binary",
                "message_id()",
                "raw_amqp_message()",
            ))),
        }
    }

    #[inline]
    fn partition_key(&self) -> Option<&str> {
        self.message_annotations
            .as_ref()?
            .get(&amqp_message_constants::PARTITION_KEY_NAME as &dyn AnnotationKey)
            .and_then(|value| match value {
                Value::String(s) => Some(s.as_str()),
                _ => None,
            })
    }

    #[inline]
    fn via_partition_key(&self) -> Option<&str> {
        self.message_annotations
            .as_ref()?
            .get(&amqp_message_constants::VIA_PARTITION_KEY_NAME as &dyn AnnotationKey)
            .and_then(|value| match value {
                Value::String(s) => Some(s.as_str()),
                _ => None,
            })
    }

    #[inline]
    fn session_id(&self) -> Option<&str> {
        self.properties
            .as_ref()?
            .group_id
            .as_ref()
            .map(|s| s.as_str())
    }

    #[inline]
    fn reply_to_session_id(&self) -> Option<&str> {
        self.properties
            .as_ref()?
            .reply_to_group_id
            .as_ref()
            .map(|s| s.as_str())
    }

    #[inline]
    fn time_to_live(&self) -> Option<Duration> {
        self.header
            .as_ref()?
            .ttl
            .map(|millis| Duration::from_millis(millis as u64))
    }

    #[inline]
    fn correlation_id(&self) -> Option<Result<&str, Error>> {
        match self.properties.as_ref()?.correlation_id.as_ref()? {
            MessageId::String(val) => Some(Ok(val)),
            MessageId::ULong(_) => Some(Err(not_supported_error(
                "MessageId::ULong",
                "message_id()",
                "raw_amqp_message()",
            ))),
            MessageId::Uuid(_) => Some(Err(not_supported_error(
                "MessageId::Uuid",
                "message_id()",
                "raw_amqp_message()",
            ))),
            MessageId::Binary(_) => Some(Err(not_supported_error(
                "MessageId::Binary",
                "message_id()",
                "raw_amqp_message()",
            ))),
        }
    }

    #[inline]
    fn subject(&self) -> Option<&str> {
        self.properties
            .as_ref()?
            .subject
            .as_ref()
            .map(|s| s.as_str())
    }

    #[inline]
    fn to(&self) -> Option<&str> {
        self.properties.as_ref()?.to.as_ref().map(|s| s.as_str())
    }

    #[inline]
    fn content_type(&self) -> Option<&str> {
        self.properties
            .as_ref()?
            .content_type
            .as_ref()
            .map(|s| s.as_str())
    }

    #[inline]
    fn reply_to(&self) -> Option<&str> {
        self.properties
            .as_ref()?
            .reply_to
            .as_ref()
            .map(|s| s.as_str())
    }

    #[inline]
    fn scheduled_enqueue_time(&self) -> Option<Result<OffsetDateTime, Error>> {
        self.message_annotations
            .as_ref()?
            .get(&amqp_message_constants::SCHEDULED_ENQUEUE_TIME_UTC_NAME as &dyn AnnotationKey)
            .map(|value| match value {
                Value::Timestamp(timestamp) => {
                    let millis = timestamp.milliseconds();
                    if millis >= 0 {
                        let duration = Duration::from_millis(millis as u64);
                        Ok(OffsetDateTime::UNIX_EPOCH + duration)
                    } else {
                        let duration = Duration::from_millis((-millis) as u64);
                        Ok(OffsetDateTime::UNIX_EPOCH - duration)
                    }
                }
                _ => Err(Error::InvalidValueType),
            })
    }
}

impl<T> AmqpMessageMutExt for Message<T> {
    #[inline]
    fn body_mut(&mut self) -> Result<&mut [u8], Error> {
        match &mut self.body {
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

    #[inline]
    fn set_body(&mut self, body: impl Into<Vec<u8>>) {
        todo!()
    }

    #[inline]
    fn message_id_mut(&mut self) -> Option<Result<&mut str, Error>> {
        match self.properties.as_mut()?.message_id.as_mut()? {
            MessageId::String(val) => Some(Ok(val)),
            MessageId::ULong(_) => Some(Err(not_supported_error(
                "MessageId::ULong",
                "message_id()",
                "raw_amqp_message()",
            ))),
            MessageId::Uuid(_) => Some(Err(not_supported_error(
                "MessageId::Uuid",
                "message_id()",
                "raw_amqp_message()",
            ))),
            MessageId::Binary(_) => Some(Err(not_supported_error(
                "MessageId::Binary",
                "message_id()",
                "raw_amqp_message()",
            ))),
        }
    }

    #[inline]
    fn set_message_id(&mut self, message_id: impl Into<String>) {
        todo!()
    }

    #[inline]
    fn partition_key_mut(&mut self) -> Option<&mut str> {
        self.message_annotations
            .as_mut()?
            .get_mut(&amqp_message_constants::PARTITION_KEY_NAME as &dyn AnnotationKey)
            .and_then(|value| match value {
                Value::String(s) => Some(s.as_mut_str()),
                _ => None,
            })
    }

    #[inline]
    fn set_partition_key(&mut self, key: impl Into<String>) {
        todo!()
    }

    #[inline]
    fn via_partition_key_mut(&mut self) -> Option<&mut str> {
        self.message_annotations
            .as_mut()?
            .get_mut(&amqp_message_constants::VIA_PARTITION_KEY_NAME as &dyn AnnotationKey)
            .and_then(|value| match value {
                Value::String(s) => Some(s.as_mut_str()),
                _ => None,
            })
    }

    #[inline]
    fn session_id_mut(&mut self) -> Option<&mut str> {
        self.properties
            .as_mut()?
            .group_id
            .as_mut()
            .map(|s| s.as_mut_str())
    }

    #[inline]
    fn reply_to_session_id_mut(&mut self) -> Option<&mut str> {
        self.properties
            .as_mut()?
            .reply_to_group_id
            .as_mut()
            .map(|s| s.as_mut_str())
    }

    #[inline]
    fn correlation_id_mut(&mut self) -> Option<Result<&mut str, Error>> {
        match self.properties.as_mut()?.correlation_id.as_mut()? {
            MessageId::String(val) => Some(Ok(val)),
            MessageId::ULong(_) => Some(Err(not_supported_error(
                "MessageId::ULong",
                "message_id()",
                "raw_amqp_message()",
            ))),
            MessageId::Uuid(_) => Some(Err(not_supported_error(
                "MessageId::Uuid",
                "message_id()",
                "raw_amqp_message()",
            ))),
            MessageId::Binary(_) => Some(Err(not_supported_error(
                "MessageId::Binary",
                "message_id()",
                "raw_amqp_message()",
            ))),
        }
    }

    #[inline]
    fn subject_mut(&mut self) -> Option<&mut str> {
        self.properties
            .as_mut()?
            .subject
            .as_mut()
            .map(|s| s.as_mut_str())
    }

    #[inline]
    fn to_mut(&mut self) -> Option<&mut str> {
        self.properties
            .as_mut()?
            .to
            .as_mut()
            .map(|s| s.as_mut_str())
    }

    #[inline]
    fn content_type_mut(&mut self) -> Option<&mut str> {
        self.properties
            .as_mut()?
            .content_type
            .as_mut()
            .map(|s| s.as_mut_str())
    }

    #[inline]
    fn reply_to_mut(&mut self) -> Option<&mut str> {
        self.properties
            .as_mut()?
            .reply_to
            .as_mut()
            .map(|s| s.as_mut_str())
    }

    #[inline]
    fn set_via_partition_key(&mut self, key: impl Into<String>) {
        todo!()
    }

    #[inline]
    fn set_session_id(&mut self, session_id: impl Into<String>) {
        todo!()
    }

    #[inline]
    fn set_reply_to_session_id(&mut self, id: impl Into<String>) {
        todo!()
    }

    #[inline]
    fn set_time_to_live(&self, ttl: Duration) {
        todo!()
    }

    #[inline]
    fn set_correlation_id(&mut self, id: impl Into<String>) {
        todo!()
    }

    #[inline]
    fn set_subject(&mut self, subject: impl Into<String>) {
        todo!()
    }

    #[inline]
    fn set_to(&mut self, to: impl Into<String>) {
        todo!()
    }

    #[inline]
    fn set_content_type(&mut self, content_type: impl Into<String>) {
        todo!()
    }

    #[inline]
    fn set_reply_to(&mut self, reply_to: impl Into<String>) {
        todo!()
    }
}
