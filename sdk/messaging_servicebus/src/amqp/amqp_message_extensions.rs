use std::borrow::Cow;

use time::Duration as TimeSpan; // To avoid confusion with std::time::Duration

use fe2o3_amqp_types::{
    messaging::{
        annotations::AnnotationKey, Body, Data, Header, Message, MessageAnnotations, MessageId,
        Properties,
    },
    primitives::{Binary, BinaryRef, Symbol, Value},
};
use time::OffsetDateTime;

use crate::constants::{MAX_MESSAGE_ID_LENGTH, MAX_PARTITION_KEY_LENGTH, MAX_SESSION_ID_LENGTH};

use super::{
    amqp_message_constants,
    error::{not_supported_error, Error},
};

pub(crate) trait AmqpMessageExt {
    fn body(&self) -> Result<&[u8], Error>;

    fn message_id(&self) -> Option<Cow<'_, str>>;

    fn partition_key(&self) -> Option<&str>;

    fn via_partition_key(&self) -> Option<&str>;

    fn session_id(&self) -> Option<&str>;

    fn reply_to_session_id(&self) -> Option<&str>;

    fn time_to_live(&self) -> Option<TimeSpan>;

    fn correlation_id(&self) -> Option<Cow<'_, str>>;

    fn subject(&self) -> Option<&str>;

    fn to(&self) -> Option<&str>;

    fn content_type(&self) -> Option<&str>;

    fn reply_to(&self) -> Option<&str>;

    // TODO: `time::OffsetDateTime` doesn't implement `Default`
    fn scheduled_enqueue_time(&self) -> Option<OffsetDateTime>;
}

pub(crate) trait AmqpMessageMutExt {
    fn body_mut(&mut self) -> Result<&mut Vec<u8>, Error>;

    fn set_body(&mut self, body: impl Into<Vec<u8>>);

    fn message_id_mut(&mut self) -> Option<&mut MessageId>;

    fn set_message_id(&mut self, message_id: impl Into<String>);

    fn partition_key_mut(&mut self) -> Option<&mut String>;

    fn set_partition_key(&mut self, key: impl Into<String>) -> Result<(), Error>;

    fn via_partition_key_mut(&mut self) -> Option<&mut String>;

    fn set_via_partition_key(&mut self, key: impl Into<String>);

    fn session_id_mut(&mut self) -> Option<&mut String>;

    fn set_session_id(&mut self, session_id: impl Into<String>);

    fn reply_to_session_id_mut(&mut self) -> Option<&mut String>;

    fn set_reply_to_session_id(&mut self, session_id: Option<impl Into<String>>);

    fn set_time_to_live(&mut self, ttl: Option<TimeSpan>);

    fn correlation_id_mut(&mut self) -> Option<&mut MessageId>;

    fn set_correlation_id(&mut self, id: Option<impl Into<String>>);

    fn subject_mut(&mut self) -> Option<&mut String>;

    fn set_subject(&mut self, subject: Option<impl Into<String>>);

    fn to_mut(&mut self) -> Option<&mut String>;

    fn set_to(&mut self, to: Option<impl Into<String>>);

    fn content_type_mut(&mut self) -> Option<&mut String>;

    fn set_content_type(&mut self, content_type: Option<impl Into<String>>);

    fn reply_to_mut(&mut self) -> Option<&mut String>;

    fn set_reply_to(&mut self, reply_to: Option<impl Into<String>>);
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
    fn message_id(&self) -> Option<Cow<'_, str>> {
        match self.properties.as_ref()?.message_id.as_ref()? {
            MessageId::String(val) => Some(Cow::Borrowed(val)),
            MessageId::ULong(val) => Some(Cow::Owned(val.to_string())),
            MessageId::Uuid(uuid) => Some(Cow::Owned(format!("{:x}", uuid))),
            MessageId::Binary(bytes) => {
                let binary_ref = BinaryRef::from(bytes);
                Some(Cow::Owned(format!("{:X}", binary_ref)))
            }
        }
    }

    #[inline]
    fn partition_key(&self) -> Option<&str> {
        self.message_annotations
            .as_ref()?
            .get(&amqp_message_constants::PARTITION_KEY_NAME as &dyn AnnotationKey)
            .map(|value| match value {
                Value::String(s) => s.as_str(),
                _ => unreachable!("Expecting a String"),
            })
    }

    #[inline]
    fn via_partition_key(&self) -> Option<&str> {
        self.message_annotations
            .as_ref()?
            .get(&amqp_message_constants::VIA_PARTITION_KEY_NAME as &dyn AnnotationKey)
            .map(|value| match value {
                Value::String(s) => s.as_str(),
                _ => unreachable!("Expecting a String"),
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
    fn time_to_live(&self) -> Option<TimeSpan> {
        self.header
            .as_ref()?
            .ttl
            .map(|millis| TimeSpan::milliseconds(millis as i64))
    }

    #[inline]
    fn correlation_id(&self) -> Option<Cow<'_, str>> {
        match self.properties.as_ref()?.correlation_id.as_ref()? {
            MessageId::String(val) => Some(Cow::Borrowed(val)),
            MessageId::ULong(val) => Some(Cow::Owned(val.to_string())),
            MessageId::Uuid(uuid) => Some(Cow::Owned(format!("{:x}", uuid))),
            MessageId::Binary(bytes) => {
                let binary_ref = BinaryRef::from(bytes);
                Some(Cow::Owned(format!("{:X}", binary_ref)))
            }
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
    fn scheduled_enqueue_time(&self) -> Option<OffsetDateTime> {
        self.message_annotations
            .as_ref()?
            .get(&amqp_message_constants::SCHEDULED_ENQUEUE_TIME_UTC_NAME as &dyn AnnotationKey)
            .map(|value| match value {
                Value::Timestamp(timestamp) => {
                    let millis = timestamp.milliseconds();
                    let duration = TimeSpan::milliseconds(millis);
                    OffsetDateTime::UNIX_EPOCH + duration
                }
                _ => unreachable!("Expecting a Timestamp"),
            })
    }
}

impl<T> AmqpMessageMutExt for Message<T> {
    #[inline]
    fn body_mut(&mut self) -> Result<&mut Vec<u8>, Error> {
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
        self.body = Body::Data(Data(Binary::from(body)))
    }

    #[inline]
    fn message_id_mut(&mut self) -> Option<&mut MessageId> {
        self.properties.as_mut()?.message_id.as_mut()
    }

    /// # Panic
    ///
    /// Panics if message_id length exceeds [`MAX_MESSAGE_ID_LENGTH`]
    #[inline]
    fn set_message_id(&mut self, message_id: impl Into<String>) {
        let message_id = message_id.into();
        assert!(message_id.len() < MAX_MESSAGE_ID_LENGTH);
        self.properties
            .get_or_insert(Properties::default())
            .message_id = Some(MessageId::String(message_id));
    }

    #[inline]
    fn partition_key_mut(&mut self) -> Option<&mut String> {
        self.message_annotations
            .as_mut()?
            .get_mut(&amqp_message_constants::PARTITION_KEY_NAME as &dyn AnnotationKey)
            .map(|value| match value {
                Value::String(s) => s,
                _ => unreachable!("Expecting a String"),
            })
    }

    /// # Panic
    ///
    /// Panics if key length exceeds [`MAX_PARTITION_KEY_LENGTH`]
    #[inline]
    fn set_partition_key(&mut self, key: impl Into<String>) -> Result<(), Error> {
        let key = key.into();
        assert!(key.len() < MAX_PARTITION_KEY_LENGTH);

        if key
            != self
                .session_id()
                .ok_or(Error::PartitionKeyAndSessionIdAreDifferent)?
        {
            return Err(Error::PartitionKeyAndSessionIdAreDifferent);
        }

        self.message_annotations
            .get_or_insert(MessageAnnotations::default())
            .insert(
                amqp_message_constants::PARTITION_KEY_NAME.into(),
                key.into(),
            );
        Ok(())
    }

    #[inline]
    fn via_partition_key_mut(&mut self) -> Option<&mut String> {
        self.message_annotations
            .as_mut()?
            .get_mut(&amqp_message_constants::VIA_PARTITION_KEY_NAME as &dyn AnnotationKey)
            .map(|value| match value {
                Value::String(s) => s,
                _ => unreachable!("Expecting a String"),
            })
    }

    #[inline]
    fn session_id_mut(&mut self) -> Option<&mut String> {
        self.properties.as_mut()?.group_id.as_mut()
    }

    #[inline]
    fn reply_to_session_id_mut(&mut self) -> Option<&mut String> {
        self.properties.as_mut()?.reply_to_group_id.as_mut()
    }

    #[inline]
    fn correlation_id_mut(&mut self) -> Option<&mut MessageId> {
        self.properties.as_mut()?.correlation_id.as_mut()
    }

    #[inline]
    fn subject_mut(&mut self) -> Option<&mut String> {
        self.properties.as_mut()?.subject.as_mut()
    }

    #[inline]
    fn to_mut(&mut self) -> Option<&mut String> {
        self.properties.as_mut()?.to.as_mut()
    }

    #[inline]
    fn content_type_mut(&mut self) -> Option<&mut String> {
        self.properties
            .as_mut()?
            .content_type
            .as_mut()
            .map(|s| &mut s.0)
    }

    #[inline]
    fn reply_to_mut(&mut self) -> Option<&mut String> {
        self.properties.as_mut()?.reply_to.as_mut()
    }

    /// # Panic
    ///
    /// Panics if key length exceeds [`MAX_PARTITION_KEY_LENGTH`]
    #[inline]
    fn set_via_partition_key(&mut self, key: impl Into<String>) {
        let key = key.into();

        assert!(key.len() < MAX_PARTITION_KEY_LENGTH);

        self.message_annotations
            .get_or_insert(MessageAnnotations::default())
            .insert(
                amqp_message_constants::VIA_PARTITION_KEY_NAME.into(),
                key.into(),
            );
    }

    /// # Panic
    ///
    /// Panics if key length exceeds [`MAX_SESSION_ID_LENGTH`]
    #[inline]
    fn set_session_id(&mut self, session_id: impl Into<String>) {
        let session_id = session_id.into();
        assert!(session_id.len() < MAX_SESSION_ID_LENGTH);

        // If the PartitionKey was already set to a different value, override it with the SessionId,
        if let Some(partition_key) = self.partition_key_mut() {
            *partition_key = session_id.clone();
        }

        self.properties
            .get_or_insert(Properties::default())
            .group_id = Some(session_id);
    }

    /// # Panic
    ///
    /// Panics if key length exceeds [`MAX_SESSION_ID_LENGTH`]
    #[inline]
    fn set_reply_to_session_id(&mut self, session_id: Option<impl Into<String>>) {
        let session_id = session_id.map(Into::into).map(|s| {
            assert!(s.len() < MAX_SESSION_ID_LENGTH);
            s
        });
        self.properties
            .get_or_insert(Properties::default())
            .reply_to_group_id = session_id;
    }

    /// # Panic
    ///
    /// Panics if `ttl.whole_milliseconds()` exceeds valid range of u32
    ///
    /// TODO: use std::time::Duration instead?
    #[inline]
    fn set_time_to_live(&mut self, ttl: Option<TimeSpan>) {
        let millis = ttl.map(|t| {
            let millis = t.whole_milliseconds();
            assert!(millis <= u32::MAX as i128 && millis >= u32::MIN as i128);
            millis as u32
        });
        self.header.get_or_insert(Header::default()).ttl = millis;
    }

    #[inline]
    fn set_correlation_id(&mut self, id: Option<impl Into<String>>) {
        let correlation_id = id.map(|s| MessageId::String(s.into()));
        self.properties
            .get_or_insert(Properties::default())
            .correlation_id = correlation_id;
    }

    #[inline]
    fn set_subject(&mut self, subject: Option<impl Into<String>>) {
        let subject = subject.map(Into::into);
        self.properties.get_or_insert(Properties::default()).subject = subject;
    }

    #[inline]
    fn set_to(&mut self, to: Option<impl Into<String>>) {
        let to = to.map(Into::into);
        self.properties.get_or_insert(Properties::default()).to = to;
    }

    #[inline]
    fn set_content_type(&mut self, content_type: Option<impl Into<String>>) {
        let content_type = content_type.map(|s| Symbol(s.into()));
        self.properties
            .get_or_insert(Properties::default())
            .content_type = content_type;
    }

    #[inline]
    fn set_reply_to(&mut self, reply_to: Option<impl Into<String>>) {
        let reply_to = reply_to.map(Into::into);
        self.properties
            .get_or_insert(Properties::default())
            .reply_to = reply_to;
    }
}
