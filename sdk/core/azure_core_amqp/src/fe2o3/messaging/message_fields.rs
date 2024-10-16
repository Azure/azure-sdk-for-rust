// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp servicebus eventhub mgmt

use crate::{
    messaging::{
        AmqpAnnotationKey, AmqpAnnotations, AmqpApplicationProperties, AmqpMessageHeader,
        AmqpMessageId, AmqpMessageProperties,
    },
    value::{AmqpOrderedMap, AmqpValue},
};

impl From<fe2o3_amqp_types::messaging::MessageId> for AmqpMessageId {
    fn from(message_id: fe2o3_amqp_types::messaging::MessageId) -> Self {
        match message_id {
            fe2o3_amqp_types::messaging::MessageId::String(message_id) => {
                AmqpMessageId::String(message_id)
            }
            fe2o3_amqp_types::messaging::MessageId::Uuid(message_id) => {
                AmqpMessageId::Uuid(message_id.into())
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

#[test]
fn test_message_id_conversion() {
    use crate::Uuid;

    {
        let message_id = fe2o3_amqp_types::messaging::MessageId::String("test".into());
        let amqp_message_id: AmqpMessageId = message_id.clone().into();
        assert_eq!(amqp_message_id, AmqpMessageId::String("test".into()));
        let fe2o3_message_id: fe2o3_amqp_types::messaging::MessageId = amqp_message_id.into();
        assert_eq!(fe2o3_message_id, message_id);
    }

    {
        let uuid = Uuid::new_v4();
        let message_id = fe2o3_amqp_types::messaging::MessageId::Uuid(uuid.into());
        let amqp_message_id: AmqpMessageId = message_id.clone().into();
        assert_eq!(amqp_message_id, AmqpMessageId::from(uuid));

        let fe2o3_message_id: fe2o3_amqp_types::messaging::MessageId = amqp_message_id.into();
        assert_eq!(fe2o3_message_id, message_id);
    }

    {
        let message_id = fe2o3_amqp_types::messaging::MessageId::Binary(vec![1, 2, 3].into());
        let amqp_message_id: AmqpMessageId = message_id.clone().into();
        assert_eq!(amqp_message_id, AmqpMessageId::Binary(vec![1, 2, 3]));
        let fe2o3_message_id: fe2o3_amqp_types::messaging::MessageId = amqp_message_id.into();
        assert_eq!(fe2o3_message_id, message_id);
    }

    {
        let message_id = fe2o3_amqp_types::messaging::MessageId::Ulong(1);
        let amqp_message_id: AmqpMessageId = message_id.clone().into();
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
        let amqp_round_trip: AmqpMessageId = message_id.into();
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
        let amqp_round_trip: AmqpMessageId = message_id.into();
        assert_eq!(amqp_round_trip, amqp_message_id);
    }
    {
        let amqp_message_id = AmqpMessageId::Binary(vec![1, 2, 3]);
        let message_id: fe2o3_amqp_types::messaging::MessageId = amqp_message_id.clone().into();
        assert_eq!(
            message_id,
            fe2o3_amqp_types::messaging::MessageId::Binary(vec![1, 2, 3].into())
        );
        let amqp_round_trip: AmqpMessageId = message_id.into();
        assert_eq!(amqp_round_trip, amqp_message_id);
    }
}

impl From<fe2o3_amqp_types::messaging::ApplicationProperties>
    for crate::messaging::AmqpApplicationProperties
{
    fn from(application_properties: fe2o3_amqp_types::messaging::ApplicationProperties) -> Self {
        let mut properties = AmqpOrderedMap::<String, AmqpValue>::new();
        for (key, value) in application_properties.0 {
            properties.insert(key, value);
        }
        AmqpApplicationProperties(properties)
    }
}

impl From<fe2o3_amqp_types::messaging::Header> for AmqpMessageHeader {
    fn from(header: fe2o3_amqp_types::messaging::Header) -> Self {
        println!("Source Header: {:?}", header);
        let rv = AmqpMessageHeader::builder()
            .with_durable(header.durable)
            .with_priority(header.priority.into())
            .with_time_to_live(
                header
                    .ttl
                    .map(|t| std::time::Duration::from_millis(t as u64)),
            )
            .with_first_acquirer(header.first_acquirer)
            .with_delivery_count(header.delivery_count)
            .build();
        println!("Converted Header: {:?}", rv);
        rv
    }
}

impl From<AmqpMessageHeader> for fe2o3_amqp_types::messaging::Header {
    fn from(header: AmqpMessageHeader) -> Self {
        fe2o3_amqp_types::messaging::Header::builder()
            .durable(header.durable())
            .priority(fe2o3_amqp_types::messaging::Priority(header.priority()))
            .ttl(header.time_to_live().map(|t| t.as_millis() as u32))
            .first_acquirer(header.first_acquirer())
            .delivery_count(header.delivery_count())
            .build()
    }
}

impl From<crate::messaging::AmqpApplicationProperties>
    for fe2o3_amqp_types::messaging::ApplicationProperties
{
    fn from(application_properties: AmqpApplicationProperties) -> Self {
        let mut properties_builder = fe2o3_amqp_types::messaging::ApplicationProperties::builder();
        for (key, value) in application_properties.0 {
            properties_builder = properties_builder.insert(key, value);
        }
        properties_builder.build()
    }
}

impl From<crate::messaging::AmqpAnnotationKey>
    for fe2o3_amqp_types::messaging::annotations::OwnedKey
{
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

impl From<fe2o3_amqp_types::messaging::annotations::OwnedKey>
    for crate::messaging::AmqpAnnotationKey
{
    fn from(key: fe2o3_amqp_types::messaging::annotations::OwnedKey) -> Self {
        match key {
            fe2o3_amqp_types::messaging::annotations::OwnedKey::Ulong(key) => {
                crate::messaging::AmqpAnnotationKey::Ulong(key)
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
        let amqp_key = AmqpAnnotationKey::from(fe2o3_key.clone());

        assert_eq!(amqp_key, AmqpAnnotationKey::Ulong(1995));
        let round_trip_key = fe2o3_amqp_types::messaging::annotations::OwnedKey::from(amqp_key);
        assert_eq!(round_trip_key, fe2o3_key);
    }

    {
        let fe2o3_key =
            fe2o3_amqp_types::messaging::annotations::OwnedKey::Symbol("OwnedSymbol".into());
        let amqp_key = AmqpAnnotationKey::from(fe2o3_key.clone());

        assert_eq!(amqp_key, AmqpAnnotationKey::Symbol("OwnedSymbol".into()));
        let round_trip_key = fe2o3_amqp_types::messaging::annotations::OwnedKey::from(amqp_key);
        assert_eq!(round_trip_key, fe2o3_key);
    }
}

impl From<crate::messaging::AmqpAnnotations> for fe2o3_amqp_types::messaging::Annotations {
    fn from(annotations: AmqpAnnotations) -> Self {
        let mut message_annotations = fe2o3_amqp_types::messaging::Annotations::new();
        for (key, value) in annotations.0 {
            message_annotations.insert(key.into(), value.into());
        }
        message_annotations
    }
}

impl From<AmqpAnnotations> for fe2o3_amqp_types::messaging::DeliveryAnnotations {
    fn from(annotations: AmqpAnnotations) -> Self {
        fe2o3_amqp_types::messaging::DeliveryAnnotations(
            annotations
                .0
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl From<AmqpAnnotations> for fe2o3_amqp_types::messaging::MessageAnnotations {
    fn from(annotations: AmqpAnnotations) -> Self {
        fe2o3_amqp_types::messaging::MessageAnnotations(
            annotations
                .0
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl From<AmqpAnnotations> for fe2o3_amqp_types::messaging::Footer {
    fn from(annotations: AmqpAnnotations) -> Self {
        fe2o3_amqp_types::messaging::Footer(
            annotations
                .0
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl From<fe2o3_amqp_types::messaging::Annotations> for AmqpAnnotations {
    fn from(annotations: fe2o3_amqp_types::messaging::Annotations) -> Self {
        let mut amqp_annotations = AmqpOrderedMap::<AmqpAnnotationKey, AmqpValue>::new();
        for (key, value) in annotations {
            amqp_annotations.insert(key, value);
        }
        AmqpAnnotations(amqp_annotations)
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

        let amqp_round_trip: AmqpAnnotations = fe2o3_annotations.into();
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

        let annotations = AmqpAnnotations::from(fe2o3_annotations.clone());
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

impl From<fe2o3_amqp_types::messaging::Properties> for AmqpMessageProperties {
    fn from(properties: fe2o3_amqp_types::messaging::Properties) -> Self {
        let mut amqp_message_properties_builder = AmqpMessageProperties::builder();

        if let Some(message_id) = properties.message_id {
            amqp_message_properties_builder =
                amqp_message_properties_builder.with_message_id(message_id);
        }
        if let Some(user_id) = properties.user_id {
            amqp_message_properties_builder =
                amqp_message_properties_builder.with_user_id(user_id.to_vec());
        }
        if let Some(to) = properties.to {
            amqp_message_properties_builder = amqp_message_properties_builder.with_to(to);
        }
        if let Some(subject) = properties.subject {
            amqp_message_properties_builder = amqp_message_properties_builder.with_subject(subject);
        }
        if let Some(reply_to) = properties.reply_to {
            amqp_message_properties_builder =
                amqp_message_properties_builder.with_reply_to(reply_to);
        }
        if let Some(correlation_id) = properties.correlation_id {
            amqp_message_properties_builder =
                amqp_message_properties_builder.with_correlation_id(correlation_id);
        }
        if let Some(content_type) = properties.content_type {
            amqp_message_properties_builder =
                amqp_message_properties_builder.with_content_type(content_type);
        }
        if let Some(content_encoding) = properties.content_encoding {
            amqp_message_properties_builder =
                amqp_message_properties_builder.with_content_encoding(content_encoding);
        }
        if let Some(absolute_expiry_time) = properties.absolute_expiry_time {
            amqp_message_properties_builder =
                amqp_message_properties_builder.with_absolute_expiry_time(absolute_expiry_time);
        }
        if let Some(creation_time) = properties.creation_time {
            amqp_message_properties_builder =
                amqp_message_properties_builder.with_creation_time(creation_time);
        }
        if let Some(group_id) = properties.group_id {
            amqp_message_properties_builder =
                amqp_message_properties_builder.with_group_id(group_id);
        }
        if let Some(group_sequence) = properties.group_sequence {
            amqp_message_properties_builder =
                amqp_message_properties_builder.with_group_sequence(group_sequence);
        }
        if let Some(reply_to_group_id) = properties.reply_to_group_id {
            amqp_message_properties_builder =
                amqp_message_properties_builder.with_reply_to_group_id(reply_to_group_id);
        }
        amqp_message_properties_builder.build()
    }
}

impl From<AmqpMessageProperties> for fe2o3_amqp_types::messaging::Properties {
    fn from(properties: AmqpMessageProperties) -> Self {
        let mut properties_builder = fe2o3_amqp_types::messaging::Properties::builder();

        if let Some(message_id) = properties.message_id() {
            properties_builder = properties_builder.message_id(message_id.clone());
        }
        if let Some(user_id) = properties.user_id() {
            properties_builder = properties_builder.user_id(user_id.clone());
        }
        if let Some(to) = properties.to() {
            properties_builder = properties_builder.to(to.clone());
        }
        if let Some(subject) = properties.subject() {
            properties_builder = properties_builder.subject(subject.clone());
        }
        if let Some(reply_to) = properties.reply_to() {
            properties_builder = properties_builder.reply_to(reply_to.clone());
        }
        if let Some(correlation_id) = properties.correlation_id() {
            properties_builder = properties_builder.correlation_id(correlation_id.clone());
        }
        if let Some(content_type) = properties.content_type() {
            properties_builder = properties_builder.content_type(content_type.clone());
        }
        if let Some(content_encoding) = properties.content_encoding() {
            properties_builder = properties_builder.content_encoding(content_encoding.clone());
        }
        if let Some(absolute_expiry_time) = properties.absolute_expiry_time() {
            properties_builder =
                properties_builder.absolute_expiry_time(Some(absolute_expiry_time.clone().into()));
        }
        if let Some(creation_time) = properties.creation_time() {
            properties_builder =
                properties_builder.creation_time(Some(creation_time.clone().into()));
        }
        if let Some(group_id) = properties.group_id() {
            properties_builder = properties_builder.group_id(group_id.clone());
        }
        if let Some(group_sequence) = properties.group_sequence() {
            properties_builder = properties_builder.group_sequence(*group_sequence);
        }
        if let Some(reply_to_group_id) = properties.reply_to_group_id() {
            properties_builder = properties_builder.reply_to_group_id(reply_to_group_id.clone());
        }
        properties_builder.build()
    }
}

#[test]
fn test_properties_conversion() {
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

        let amqp_properties = AmqpMessageProperties::from(properties.clone());
        let roundtrip_properties = fe2o3_amqp_types::messaging::Properties::from(amqp_properties);
        assert_eq!(properties, roundtrip_properties);
    }

    {
        let time_now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // Round trip time_now through milliseconds to round down from nanoseconds.
        let time_now: std::time::SystemTime =
            std::time::UNIX_EPOCH + std::time::Duration::from_millis(time_now as u64);

        let properties = AmqpMessageProperties::builder()
            .with_absolute_expiry_time(time_now)
            .with_content_encoding("content_encoding")
            .with_content_type("content_type")
            .with_correlation_id("correlation_id")
            .with_creation_time(time_now)
            .with_group_id("group_id")
            .with_group_sequence(3)
            .with_message_id("test")
            .with_reply_to("reply_to")
            .with_reply_to_group_id("reply_to_group_id")
            .with_subject("subject")
            .with_to("to")
            .with_user_id(vec![1, 2, 3])
            .build();

        let fe2o3_properties: fe2o3_amqp_types::messaging::Properties = properties.clone().into();

        let amqp_round_trip = AmqpMessageProperties::from(fe2o3_properties);
        assert_eq!(properties, amqp_round_trip);
    }
}
