use std::{borrow::Cow, time::Duration as StdDuration};

use time::Duration as TimeSpan; // To avoid confusion with std::time::Duration

use fe2o3_amqp_types::{
    messaging::{
        annotations::AnnotationKey, Header, Message, MessageAnnotations, MessageId, Properties,
    },
    primitives::{BinaryRef, Symbol, Value},
};
use time::OffsetDateTime;

use crate::constants::{
    DEFAULT_OFFSET_DATE_TIME, MAX_MESSAGE_ID_LENGTH, MAX_PARTITION_KEY_LENGTH,
    MAX_SESSION_ID_LENGTH,
};

use super::{
    amqp_message_constants::{self, SCHEDULED_ENQUEUE_TIME_UTC_NAME},
    error::{
        MaxAllowedTtlExceededError, MaxLengthExceededError, SetMessageIdError, SetPartitionKeyError,
    },
};

pub(crate) trait AmqpMessageExt {
    fn message_id(&self) -> Option<Cow<'_, str>>;

    fn partition_key(&self) -> Option<&str>;

    fn via_partition_key(&self) -> Option<&str>;

    fn session_id(&self) -> Option<&str>;

    fn reply_to_session_id(&self) -> Option<&str>;

    fn time_to_live(&self) -> Option<StdDuration>;

    fn correlation_id(&self) -> Option<Cow<'_, str>>;

    fn subject(&self) -> Option<&str>;

    fn to(&self) -> Option<&str>;

    fn content_type(&self) -> Option<&str>;

    fn reply_to(&self) -> Option<&str>;

    fn scheduled_enqueue_time(&self) -> OffsetDateTime;
}

pub(crate) trait AmqpMessageMutExt {
    fn set_message_id(&mut self, message_id: impl Into<String>) -> Result<(), SetMessageIdError>;

    fn set_partition_key(
        &mut self,
        key: impl Into<Option<String>>,
    ) -> Result<(), SetPartitionKeyError>;

    fn set_via_partition_key(
        &mut self,
        key: impl Into<Option<String>>,
    ) -> Result<(), MaxLengthExceededError>;

    fn set_session_id(
        &mut self,
        session_id: impl Into<Option<String>>,
    ) -> Result<(), MaxLengthExceededError>;

    fn set_reply_to_session_id(
        &mut self,
        session_id: impl Into<Option<String>>,
    ) -> Result<(), MaxLengthExceededError>;

    fn set_time_to_live(
        &mut self,
        ttl: Option<StdDuration>,
    ) -> Result<(), MaxAllowedTtlExceededError>;

    fn set_correlation_id(&mut self, id: impl Into<Option<String>>);

    fn set_subject(&mut self, subject: impl Into<Option<String>>);

    fn set_to(&mut self, to: impl Into<Option<String>>);

    fn set_content_type(&mut self, content_type: impl Into<Option<String>>);

    fn set_reply_to(&mut self, reply_to: impl Into<Option<String>>);
}

impl<B> AmqpMessageExt for Message<B> {
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
        self.properties.as_ref()?.group_id.as_deref()
    }

    #[inline]
    fn reply_to_session_id(&self) -> Option<&str> {
        self.properties.as_ref()?.reply_to_group_id.as_deref()
    }

    #[inline]
    fn time_to_live(&self) -> Option<StdDuration> {
        self.header
            .as_ref()?
            .ttl
            .map(|millis| StdDuration::from_millis(millis as u64))
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
        self.properties.as_ref()?.subject.as_deref()
    }

    #[inline]
    fn to(&self) -> Option<&str> {
        self.properties.as_ref()?.to.as_deref()
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
        self.properties.as_ref()?.reply_to.as_deref()
    }

    /// The default `DateTimeOffset` value from dotnet is taken as the default value
    /// if the scheduled_enqueue_time is `None`
    #[inline]
    fn scheduled_enqueue_time(&self) -> OffsetDateTime {
        self.message_annotations
            .as_ref()
            .and_then(|m| m.get(&SCHEDULED_ENQUEUE_TIME_UTC_NAME as &dyn AnnotationKey))
            .map(|value| match value {
                Value::Timestamp(timestamp) => {
                    // let millis = timestamp.milliseconds();
                    // let duration = TimeSpan::milliseconds(millis);
                    let timespan = TimeSpan::from(timestamp.clone());
                    OffsetDateTime::UNIX_EPOCH + timespan
                }
                _ => unreachable!("Expecting a Timestamp"),
            })
            .unwrap_or(DEFAULT_OFFSET_DATE_TIME)
    }
}

impl<B> AmqpMessageMutExt for Message<B> {
    /// Returns `Err(_)` if message_id length exceeds [`MAX_MESSAGE_ID_LENGTH`]
    #[inline]
    fn set_message_id(&mut self, message_id: impl Into<String>) -> Result<(), SetMessageIdError> {
        let message_id = message_id.into();

        if message_id.is_empty() {
            return Err(SetMessageIdError::Empty);
        }

        if message_id.len() > MAX_MESSAGE_ID_LENGTH {
            return Err(
                MaxLengthExceededError::new(message_id.len(), MAX_MESSAGE_ID_LENGTH).into(),
            );
        }

        self.properties
            .get_or_insert(Properties::default())
            .message_id = Some(MessageId::String(message_id));
        Ok(())
    }

    /// Returns error if key length exceeds [`MAX_PARTITION_KEY_LENGTH`]
    #[inline]
    fn set_partition_key(
        &mut self,
        key: impl Into<Option<String>>,
    ) -> Result<(), SetPartitionKeyError> {
        let key: Option<String> = key.into();
        match key.as_ref().map(|k| k.len()) {
            Some(len) if len > MAX_PARTITION_KEY_LENGTH => {
                return Err(MaxLengthExceededError::new(len, MAX_PARTITION_KEY_LENGTH).into())
            }
            _ => {}
        }

        match (key.as_ref(), self.session_id()) {
            (Some(key), Some(session_id)) if key != session_id => {
                return Err(SetPartitionKeyError::PartitionKeyAndSessionIdAreDifferent);
            }
            _ => {}
        }

        self.message_annotations
            .get_or_insert(MessageAnnotations::default())
            .insert(
                amqp_message_constants::PARTITION_KEY_NAME.into(),
                key.map(Value::String).unwrap_or(Value::Null),
            );
        Ok(())
    }

    /// Returns error if key length exceeds [`MAX_PARTITION_KEY_LENGTH`]
    #[inline]
    fn set_via_partition_key(
        &mut self,
        key: impl Into<Option<String>>,
    ) -> Result<(), MaxLengthExceededError> {
        let key = key.into();

        match key.as_ref().map(|k| k.len()) {
            Some(len) if len > MAX_PARTITION_KEY_LENGTH => {
                return Err(MaxLengthExceededError::new(len, MAX_PARTITION_KEY_LENGTH))
            }
            _ => {}
        }

        self.message_annotations
            .get_or_insert(MessageAnnotations::default())
            .insert(
                amqp_message_constants::VIA_PARTITION_KEY_NAME.into(),
                key.map(Value::String).unwrap_or(Value::Null),
            );
        Ok(())
    }

    /// Returns error if key length exceeds [`MAX_SESSION_ID_LENGTH`]
    #[inline]
    fn set_session_id(
        &mut self,
        session_id: impl Into<Option<String>>,
    ) -> Result<(), MaxLengthExceededError> {
        let session_id = session_id.into();
        match session_id.as_ref().map(|s| s.len()) {
            Some(session_id_len) if session_id_len > MAX_SESSION_ID_LENGTH => {
                return Err(MaxLengthExceededError::new(
                    session_id_len,
                    MAX_SESSION_ID_LENGTH,
                ))
            }
            _ => {}
        }

        // If the PartitionKey was already set to a different value, override it with the SessionId,
        if let Some(partition_key) = self.message_annotations.as_mut().and_then(|m| {
            m.get_mut(&amqp_message_constants::PARTITION_KEY_NAME as &dyn AnnotationKey)
        }) {
            *partition_key = session_id.clone().map(Value::String).unwrap_or(Value::Null);
        }

        self.properties
            .get_or_insert(Properties::default())
            .group_id = session_id;
        Ok(())
    }

    /// Returns error if key length exceeds [`MAX_SESSION_ID_LENGTH`]
    #[inline]
    fn set_reply_to_session_id(
        &mut self,
        session_id: impl Into<Option<String>>,
    ) -> Result<(), MaxLengthExceededError> {
        let session_id = session_id
            .into()
            .map(|s| {
                if s.len() > MAX_SESSION_ID_LENGTH {
                    Result::<String, _>::Err(MaxLengthExceededError::new(
                        s.len(),
                        MAX_SESSION_ID_LENGTH,
                    ))
                } else {
                    Ok(s)
                }
            })
            .transpose()?;
        self.properties
            .get_or_insert(Properties::default())
            .reply_to_group_id = session_id;
        Ok(())
    }

    /// Returns error if `ttl.whole_milliseconds()` exceeds valid range of u32
    #[inline]
    fn set_time_to_live(
        &mut self,
        ttl: Option<StdDuration>,
    ) -> Result<(), MaxAllowedTtlExceededError> {
        let millis = ttl
            .map(|t| {
                let millis = t.as_millis();
                if millis > u32::MAX as u128 {
                    Result::<u32, _>::Err(MaxAllowedTtlExceededError {})
                } else {
                    Ok(millis as u32)
                }
            })
            .transpose()?;
        self.header.get_or_insert(Header::default()).ttl = millis;
        Ok(())
    }

    #[inline]
    fn set_correlation_id(&mut self, id: impl Into<Option<String>>) {
        let correlation_id = id.into().map(MessageId::String);
        self.properties
            .get_or_insert(Properties::default())
            .correlation_id = correlation_id;
    }

    #[inline]
    fn set_subject(&mut self, subject: impl Into<Option<String>>) {
        let subject = subject.into();
        self.properties.get_or_insert(Properties::default()).subject = subject;
    }

    #[inline]
    fn set_to(&mut self, to: impl Into<Option<String>>) {
        let to = to.into();
        self.properties.get_or_insert(Properties::default()).to = to;
    }

    #[inline]
    fn set_content_type(&mut self, content_type: impl Into<Option<String>>) {
        let content_type = content_type.into();
        self.properties
            .get_or_insert(Properties::default())
            .content_type = content_type.map(Symbol::from);
    }

    #[inline]
    fn set_reply_to(&mut self, reply_to: impl Into<Option<String>>) {
        let reply_to = reply_to.into();
        self.properties
            .get_or_insert(Properties::default())
            .reply_to = reply_to;
    }
}
