// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    messaging::{
        AmqpAnnotationKey, AmqpAnnotations, AmqpApplicationProperties, AmqpMessageHeader,
        AmqpMessageId, AmqpMessageProperties,
    },
    value::{AmqpOrderedMap, AmqpValue},
};
use azure_core::time::Duration;

impl From<&fe2o3_amqp_types::messaging::MessageId> for AmqpMessageId {
    fn from(message_id: &fe2o3_amqp_types::messaging::MessageId) -> Self {
        match message_id {
            fe2o3_amqp_types::messaging::MessageId::String(message_id) => {
                AmqpMessageId::String(message_id.clone())
            }
            fe2o3_amqp_types::messaging::MessageId::Uuid(message_id) => {
                AmqpMessageId::Uuid(azure_core::Uuid::from_bytes(*message_id.as_inner()))
            }
            fe2o3_amqp_types::messaging::MessageId::Binary(message_id) => {
                AmqpMessageId::Binary(message_id.to_vec())
            }
            fe2o3_amqp_types::messaging::MessageId::Ulong(message_id) => {
                AmqpMessageId::Ulong(*message_id)
            }
        }
    }
}

impl From<fe2o3_amqp_types::messaging::MessageId> for AmqpMessageId {
    fn from(message_id: fe2o3_amqp_types::messaging::MessageId) -> Self {
        match message_id {
            fe2o3_amqp_types::messaging::MessageId::String(message_id) => {
                AmqpMessageId::String(message_id)
            }
            fe2o3_amqp_types::messaging::MessageId::Uuid(message_id) => {
                AmqpMessageId::Uuid(azure_core::Uuid::from_bytes(message_id.into_inner()))
            }
            fe2o3_amqp_types::messaging::MessageId::Binary(message_id) => {
                AmqpMessageId::Binary(message_id.to_vec())
            }
            fe2o3_amqp_types::messaging::MessageId::Ulong(message_id) => {
                AmqpMessageId::Ulong(message_id)
            }
        }
    }
}

impl From<AmqpMessageId> for fe2o3_amqp_types::messaging::MessageId {
    fn from(message_id: AmqpMessageId) -> Self {
        match message_id {
            AmqpMessageId::String(message_id) => {
                fe2o3_amqp_types::messaging::MessageId::String(message_id)
            }
            AmqpMessageId::Uuid(message_id) => fe2o3_amqp_types::messaging::MessageId::Uuid(
                fe2o3_amqp_types::primitives::Uuid::from(message_id),
            ),
            AmqpMessageId::Binary(message_id) => fe2o3_amqp_types::messaging::MessageId::Binary(
                serde_bytes::ByteBuf::from(message_id),
            ),
            AmqpMessageId::Ulong(message_id) => {
                fe2o3_amqp_types::messaging::MessageId::Ulong(message_id)
            }
        }
    }
}

impl From<&AmqpMessageId> for fe2o3_amqp_types::messaging::MessageId {
    fn from(message_id: &AmqpMessageId) -> Self {
        match message_id {
            AmqpMessageId::String(message_id) => {
                fe2o3_amqp_types::messaging::MessageId::String(message_id.clone())
            }
            AmqpMessageId::Uuid(message_id) => fe2o3_amqp_types::messaging::MessageId::Uuid(
                fe2o3_amqp_types::primitives::Uuid::from(*message_id),
            ),
            AmqpMessageId::Binary(message_id) => fe2o3_amqp_types::messaging::MessageId::Binary(
                serde_bytes::ByteBuf::from(message_id.as_slice()),
            ),
            AmqpMessageId::Ulong(message_id) => {
                fe2o3_amqp_types::messaging::MessageId::Ulong(*message_id)
            }
        }
    }
}

#[test]
fn test_message_id_conversion() {
    use azure_core::Uuid;
    {
        let message_id = fe2o3_amqp_types::messaging::MessageId::String("test".into());
        let amqp_message_id: AmqpMessageId = (&message_id).into();
        assert_eq!(amqp_message_id, AmqpMessageId::String("test".into()));
        let fe2o3_message_id: fe2o3_amqp_types::messaging::MessageId = amqp_message_id.into();
        assert_eq!(fe2o3_message_id, message_id);
    }

    {
        let uuid = Uuid::new_v4();
        let message_id = fe2o3_amqp_types::messaging::MessageId::Uuid(uuid.into());
        let amqp_message_id: AmqpMessageId = (&message_id).into();
        assert_eq!(amqp_message_id, AmqpMessageId::from(uuid));

        let fe2o3_message_id: fe2o3_amqp_types::messaging::MessageId = amqp_message_id.into();
        assert_eq!(fe2o3_message_id, message_id);
    }

    {
        let message_id = fe2o3_amqp_types::messaging::MessageId::Binary(vec![1, 2, 3].into());
        let amqp_message_id: AmqpMessageId = (&message_id).into();
        assert_eq!(amqp_message_id, AmqpMessageId::Binary(vec![1, 2, 3]));
        let fe2o3_message_id: fe2o3_amqp_types::messaging::MessageId = amqp_message_id.into();
        assert_eq!(fe2o3_message_id, message_id);
    }

    {
        let message_id = fe2o3_amqp_types::messaging::MessageId::Ulong(1);
        let amqp_message_id: AmqpMessageId = (&message_id).into();
        assert_eq!(amqp_message_id, AmqpMessageId::Ulong(1));
        let fe2o3_message_id: fe2o3_amqp_types::messaging::MessageId = amqp_message_id.into();
        assert_eq!(fe2o3_message_id, message_id);
    }

    {
        let amqp_message_id = AmqpMessageId::String("test".into());
        let message_id: fe2o3_amqp_types::messaging::MessageId = amqp_message_id.clone().into();
        assert_eq!(
            message_id,
            fe2o3_amqp_types::messaging::MessageId::String("test".into())
        );
        let amqp_round_trip: AmqpMessageId = (&message_id).into();
        assert_eq!(amqp_round_trip, amqp_message_id);
    }
    {
        let uuid = Uuid::new_v4();
        let amqp_message_id = AmqpMessageId::from(uuid);
        let message_id: fe2o3_amqp_types::messaging::MessageId = amqp_message_id.clone().into();
        assert_eq!(
            message_id,
            fe2o3_amqp_types::messaging::MessageId::Uuid(fe2o3_amqp_types::primitives::Uuid::from(
                uuid
            ))
        );
        let amqp_round_trip: AmqpMessageId = (&message_id).into();
        assert_eq!(amqp_round_trip, amqp_message_id);
    }
    {
        let amqp_message_id = AmqpMessageId::Binary(vec![1, 2, 3]);
        let message_id: fe2o3_amqp_types::messaging::MessageId = amqp_message_id.clone().into();
        assert_eq!(
            message_id,
            fe2o3_amqp_types::messaging::MessageId::Binary(vec![1, 2, 3].into())
        );
        let amqp_round_trip: AmqpMessageId = (&message_id).into();
        assert_eq!(amqp_round_trip, amqp_message_id);
    }
}

impl From<&fe2o3_amqp_types::messaging::ApplicationProperties> for AmqpApplicationProperties {
    fn from(application_properties: &fe2o3_amqp_types::messaging::ApplicationProperties) -> Self {
        Self(
            application_properties
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        )
    }
}

impl From<fe2o3_amqp_types::messaging::ApplicationProperties> for AmqpApplicationProperties {
    fn from(application_properties: fe2o3_amqp_types::messaging::ApplicationProperties) -> Self {
        Self(
            application_properties
                .0
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        )
    }
}

impl From<AmqpApplicationProperties> for fe2o3_amqp_types::messaging::ApplicationProperties {
    fn from(application_properties: AmqpApplicationProperties) -> Self {
        Self(
            application_properties
                .0
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        )
    }
}

impl From<&AmqpApplicationProperties> for fe2o3_amqp_types::messaging::ApplicationProperties {
    fn from(application_properties: &AmqpApplicationProperties) -> Self {
        Self(
            application_properties
                .0
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        )
    }
}

impl From<&fe2o3_amqp_types::messaging::Header> for AmqpMessageHeader {
    fn from(header: &fe2o3_amqp_types::messaging::Header) -> Self {
        AmqpMessageHeader {
            durable: header.durable,
            priority: header.priority.into(),
            time_to_live: header.ttl.map(|t| Duration::milliseconds(t as i64)),
            first_acquirer: (header.first_acquirer),
            delivery_count: (header.delivery_count),
        }
    }
}
impl From<&AmqpMessageHeader> for fe2o3_amqp_types::messaging::Header {
    fn from(header: &AmqpMessageHeader) -> Self {
        fe2o3_amqp_types::messaging::Header {
            durable: header.durable,
            priority: fe2o3_amqp_types::messaging::Priority(header.priority),
            ttl: header.time_to_live.map(|t| t.whole_milliseconds() as u32),
            first_acquirer: header.first_acquirer,
            delivery_count: header.delivery_count,
        }
    }
}

impl From<fe2o3_amqp_types::messaging::Header> for AmqpMessageHeader {
    fn from(header: fe2o3_amqp_types::messaging::Header) -> Self {
        Self::from(&header)
    }
}

impl From<AmqpMessageHeader> for fe2o3_amqp_types::messaging::Header {
    fn from(header: AmqpMessageHeader) -> Self {
        Self::from(&header)
    }
}

impl From<AmqpAnnotationKey> for fe2o3_amqp_types::messaging::annotations::OwnedKey {
    fn from(key: AmqpAnnotationKey) -> Self {
        match key {
            AmqpAnnotationKey::Ulong(key) => {
                fe2o3_amqp_types::messaging::annotations::OwnedKey::Ulong(key)
            }
            AmqpAnnotationKey::Symbol(key) => {
                fe2o3_amqp_types::messaging::annotations::OwnedKey::Symbol(key.into())
            }
        }
    }
}

impl From<&AmqpAnnotationKey> for fe2o3_amqp_types::messaging::annotations::OwnedKey {
    fn from(key: &AmqpAnnotationKey) -> Self {
        match key {
            AmqpAnnotationKey::Ulong(key) => {
                fe2o3_amqp_types::messaging::annotations::OwnedKey::Ulong(*key)
            }
            AmqpAnnotationKey::Symbol(key) => {
                fe2o3_amqp_types::messaging::annotations::OwnedKey::Symbol(key.into())
            }
        }
    }
}

impl From<&fe2o3_amqp_types::messaging::annotations::OwnedKey>
    for crate::messaging::AmqpAnnotationKey
{
    fn from(key: &fe2o3_amqp_types::messaging::annotations::OwnedKey) -> Self {
        match key {
            fe2o3_amqp_types::messaging::annotations::OwnedKey::Ulong(key) => {
                crate::messaging::AmqpAnnotationKey::Ulong(*key)
            }
            fe2o3_amqp_types::messaging::annotations::OwnedKey::Symbol(key) => {
                crate::messaging::AmqpAnnotationKey::Symbol(key.into())
            }
        }
    }
}

#[test]
fn test_owned_key_conversion() {
    {
        let fe2o3_key = fe2o3_amqp_types::messaging::annotations::OwnedKey::Ulong(1995);
        let amqp_key = AmqpAnnotationKey::from(&fe2o3_key);

        assert_eq!(amqp_key, AmqpAnnotationKey::Ulong(1995));
        let round_trip_key = fe2o3_amqp_types::messaging::annotations::OwnedKey::from(amqp_key);
        assert_eq!(round_trip_key, fe2o3_key);
    }

    {
        let fe2o3_key =
            fe2o3_amqp_types::messaging::annotations::OwnedKey::Symbol("OwnedSymbol".into());
        let amqp_key = AmqpAnnotationKey::from(&fe2o3_key);

        assert_eq!(amqp_key, AmqpAnnotationKey::Symbol("OwnedSymbol".into()));
        let round_trip_key = fe2o3_amqp_types::messaging::annotations::OwnedKey::from(amqp_key);
        assert_eq!(round_trip_key, fe2o3_key);
    }
}

impl From<AmqpAnnotations> for fe2o3_amqp_types::messaging::Annotations {
    fn from(annotations: AmqpAnnotations) -> Self {
        annotations
            .0
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect::<fe2o3_amqp_types::messaging::Annotations>()
    }
}

impl From<&AmqpAnnotations> for fe2o3_amqp_types::messaging::Annotations {
    fn from(annotations: &AmqpAnnotations) -> Self {
        annotations
            .0
            .iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect::<fe2o3_amqp_types::messaging::Annotations>()
    }
}

impl From<&fe2o3_amqp_types::messaging::Annotations> for AmqpAnnotations {
    fn from(annotations: &fe2o3_amqp_types::messaging::Annotations) -> Self {
        AmqpAnnotations(
            annotations
                .iter()
                .map(|(k, v)| (AmqpAnnotationKey::from(k), AmqpValue::from(v)))
                .collect::<AmqpOrderedMap<AmqpAnnotationKey, AmqpValue>>(),
        )
    }
}

impl From<fe2o3_amqp_types::messaging::Annotations> for AmqpAnnotations {
    fn from(annotations: fe2o3_amqp_types::messaging::Annotations) -> Self {
        AmqpAnnotations(
            annotations
                .iter()
                .map(|(k, v)| (AmqpAnnotationKey::from(k), AmqpValue::from(v)))
                .collect::<AmqpOrderedMap<AmqpAnnotationKey, AmqpValue>>(),
        )
    }
}

impl From<AmqpAnnotations> for fe2o3_amqp_types::messaging::DeliveryAnnotations {
    fn from(annotations: AmqpAnnotations) -> Self {
        fe2o3_amqp_types::messaging::DeliveryAnnotations(annotations.into())
    }
}
impl From<&AmqpAnnotations> for fe2o3_amqp_types::messaging::DeliveryAnnotations {
    fn from(annotations: &AmqpAnnotations) -> Self {
        fe2o3_amqp_types::messaging::DeliveryAnnotations(annotations.into())
    }
}

impl From<AmqpAnnotations> for fe2o3_amqp_types::messaging::MessageAnnotations {
    fn from(annotations: AmqpAnnotations) -> Self {
        fe2o3_amqp_types::messaging::MessageAnnotations(annotations.into())
    }
}

impl From<&AmqpAnnotations> for fe2o3_amqp_types::messaging::MessageAnnotations {
    fn from(annotations: &AmqpAnnotations) -> Self {
        fe2o3_amqp_types::messaging::MessageAnnotations(annotations.into())
    }
}

impl From<AmqpAnnotations> for fe2o3_amqp_types::messaging::Footer {
    fn from(annotations: AmqpAnnotations) -> Self {
        fe2o3_amqp_types::messaging::Footer(annotations.into())
    }
}

impl From<&AmqpAnnotations> for fe2o3_amqp_types::messaging::Footer {
    fn from(annotations: &AmqpAnnotations) -> Self {
        fe2o3_amqp_types::messaging::Footer(annotations.into())
    }
}

#[test]
fn test_message_annotation_conversion() {
    {
        let annotations = AmqpAnnotations::from(vec![
            (AmqpAnnotationKey::Ulong(1), "test"),
            (AmqpAnnotationKey::Symbol("test".into()), "test"),
        ]);

        let fe2o3_annotations = fe2o3_amqp_types::messaging::Annotations::from(annotations.clone());

        // There does not appear to be a From<OrderedMap<>> for Annotations.
        let mut annotations_to_test = fe2o3_amqp_types::messaging::Annotations::new();
        annotations_to_test.insert(
            fe2o3_amqp_types::messaging::annotations::OwnedKey::Ulong(1),
            "test".into(),
        );
        annotations_to_test.insert(
            fe2o3_amqp_types::messaging::annotations::OwnedKey::Symbol("test".into()),
            "test".into(),
        );
        assert_eq!(fe2o3_annotations, annotations_to_test);

        let amqp_round_trip: AmqpAnnotations = (&fe2o3_annotations).into();
        assert_eq!(amqp_round_trip, annotations);
    }

    {
        let mut fe2o3_annotations = fe2o3_amqp_types::messaging::Annotations::new();
        fe2o3_annotations.insert(
            fe2o3_amqp_types::messaging::annotations::OwnedKey::Ulong(1),
            "test".into(),
        );
        fe2o3_annotations.insert(
            fe2o3_amqp_types::messaging::annotations::OwnedKey::Symbol("test".into()),
            "test".into(),
        );

        let annotations = AmqpAnnotations::from(&fe2o3_annotations);
        assert_eq!(
            annotations,
            AmqpAnnotations::from(vec![
                (AmqpAnnotationKey::Ulong(1), "test"),
                (AmqpAnnotationKey::Symbol("test".into()), "test"),
            ])
        );

        let fe2o3_round_trip: fe2o3_amqp_types::messaging::Annotations = annotations.into();
        assert_eq!(fe2o3_round_trip, fe2o3_annotations);
    }
}

impl From<&fe2o3_amqp_types::messaging::Properties> for AmqpMessageProperties {
    fn from(properties: &fe2o3_amqp_types::messaging::Properties) -> Self {
        let amqp_message_properties = AmqpMessageProperties {
            message_id: properties.message_id.as_ref().map(Into::into),
            user_id: properties.user_id.as_ref().map(|u| u.to_vec()),
            to: properties.to.clone(),
            subject: properties.subject.clone(),
            reply_to: properties.reply_to.clone(),
            correlation_id: properties.correlation_id.as_ref().map(Into::into),
            content_type: properties.content_type.as_ref().map(Into::into),
            content_encoding: properties.content_encoding.as_ref().map(Into::into),
            absolute_expiry_time: properties.absolute_expiry_time.as_ref().map(Into::into),
            creation_time: properties.creation_time.as_ref().map(Into::into),
            group_id: properties.group_id.clone(),
            group_sequence: properties.group_sequence,
            reply_to_group_id: properties.reply_to_group_id.clone(),
        };
        amqp_message_properties
    }
}

impl From<fe2o3_amqp_types::messaging::Properties> for AmqpMessageProperties {
    fn from(properties: fe2o3_amqp_types::messaging::Properties) -> Self {
        AmqpMessageProperties {
            message_id: properties.message_id.map(Into::into),
            user_id: properties.user_id.map(|u| u.to_vec()),
            to: properties.to,
            subject: properties.subject,
            reply_to: properties.reply_to,
            correlation_id: properties.correlation_id.map(Into::into),
            content_type: properties.content_type.map(Into::into),
            content_encoding: properties.content_encoding.map(Into::into),
            absolute_expiry_time: properties.absolute_expiry_time.map(Into::into),
            creation_time: properties.creation_time.map(Into::into),
            group_id: properties.group_id,
            group_sequence: properties.group_sequence,
            reply_to_group_id: properties.reply_to_group_id,
        }
    }
}

impl From<AmqpMessageProperties> for fe2o3_amqp_types::messaging::Properties {
    fn from(properties: AmqpMessageProperties) -> Self {
        Self {
            message_id: properties.message_id.map(Into::into),
            user_id: properties.user_id.map(Into::into),
            to: properties.to,
            subject: properties.subject,
            reply_to: properties.reply_to,
            correlation_id: properties.correlation_id.map(Into::into),
            content_type: properties.content_type.map(Into::into),
            content_encoding: properties.content_encoding.map(Into::into),
            absolute_expiry_time: properties.absolute_expiry_time.map(Into::into),
            creation_time: properties.creation_time.map(Into::into),
            group_id: properties.group_id,
            group_sequence: properties.group_sequence,
            reply_to_group_id: properties.reply_to_group_id,
        }
    }
}

impl From<&AmqpMessageProperties> for fe2o3_amqp_types::messaging::Properties {
    fn from(properties: &AmqpMessageProperties) -> Self {
        Self {
            message_id: properties.message_id.as_ref().map(Into::into),
            user_id: properties
                .user_id
                .as_ref()
                .map(|u| serde_bytes::ByteBuf::from(u.clone())),
            to: properties.to.clone(),
            subject: properties.subject.clone(),
            reply_to: properties.reply_to.clone(),
            correlation_id: properties.correlation_id.as_ref().map(Into::into),
            content_type: properties.content_type.as_ref().map(Into::into),
            content_encoding: properties.content_encoding.as_ref().map(Into::into),
            absolute_expiry_time: properties.absolute_expiry_time.as_ref().map(Into::into),
            creation_time: properties.creation_time.as_ref().map(Into::into),
            group_id: properties.group_id.clone(),
            group_sequence: properties.group_sequence,
            reply_to_group_id: properties.reply_to_group_id.clone(),
        }
    }
}

#[test]
fn test_properties_conversion() {
    use std::time::{SystemTime, UNIX_EPOCH};

    {
        let properties = fe2o3_amqp_types::messaging::Properties {
            message_id: Some(fe2o3_amqp_types::messaging::MessageId::String(
                "test".into(),
            )),
            user_id: Some(vec![1, 2, 3].into()),
            to: Some("to".into()),
            subject: Some("subject".into()),
            reply_to: Some("reply_to".into()),
            correlation_id: Some("correlation_id".to_string().into()),
            content_type: Some("content_type".into()),
            content_encoding: Some("content_encoding".into()),
            absolute_expiry_time: Some(fe2o3_amqp_types::primitives::Timestamp::from(1)),
            creation_time: Some(fe2o3_amqp_types::primitives::Timestamp::from(2)),
            group_id: Some("group_id".into()),
            group_sequence: Some(3),
            reply_to_group_id: Some("reply_to_group_id".into()),
        };

        let amqp_properties = AmqpMessageProperties::from(&properties);
        let roundtrip_properties = fe2o3_amqp_types::messaging::Properties::from(amqp_properties);
        assert_eq!(properties, roundtrip_properties);
    }

    {
        let time_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // Round trip time_now through milliseconds to round down from nanoseconds.
        let time_now: SystemTime = UNIX_EPOCH + Duration::milliseconds(time_now);

        let properties = AmqpMessageProperties {
            absolute_expiry_time: Some(time_now.into()),
            content_encoding: Some(crate::value::AmqpSymbol("content_encoding".to_string())),
            content_type: Some(crate::value::AmqpSymbol("content_type".to_string())),
            correlation_id: Some("correlation_id".into()),
            creation_time: Some(time_now.into()),
            group_id: Some("group_id".to_string()),
            group_sequence: Some(3),
            message_id: Some("test".into()),
            reply_to: Some("reply_to".to_string()),
            reply_to_group_id: Some("reply_to_group_id".to_string()),
            subject: Some("subject".to_string()),
            to: Some("to".to_string()),
            user_id: Some(vec![1, 2, 3]),
        };

        let fe2o3_properties: fe2o3_amqp_types::messaging::Properties = properties.clone().into();

        let amqp_round_trip = AmqpMessageProperties::from(&fe2o3_properties);
        assert_eq!(properties, amqp_round_trip);
    }
}
