// Copyright (c) Microsoft Corp. All Rights Reserved.

// cspell: words amqp servicebus eventhub mgmt

use fe2o3_amqp_types::messaging::annotations::OwnedKey;

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
        AmqpMessageHeader::builder()
            .with_durable(header.durable)
            .with_priority(header.priority.into())
            .with_time_to_live(std::time::Duration::from_millis(
                header.ttl.unwrap_or(0) as u64
            ))
            .with_first_acquirer(header.first_acquirer)
            .with_delivery_count(header.delivery_count)
            .build()
    }
}

impl From<AmqpMessageHeader> for fe2o3_amqp_types::messaging::Header {
    fn from(header: AmqpMessageHeader) -> Self {
        let mut builder = fe2o3_amqp_types::messaging::Header::builder();

        if let Some(durable) = header.durable() {
            builder = builder.durable(*durable);
        }
        if let Some(priority) = header.priority() {
            builder = builder.priority(fe2o3_amqp_types::messaging::Priority(*priority));
        }
        if let Some(time_to_live) = header.time_to_live() {
            builder = builder.ttl(Some(time_to_live.as_millis() as u32));
        }
        if let Some(first_acquirer) = header.first_acquirer() {
            builder = builder.first_acquirer(*first_acquirer);
        }
        if let Some(delivery_count) = header.delivery_count() {
            builder = builder.delivery_count(*delivery_count);
        }
        builder.build()
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

impl From<crate::messaging::AmqpAnnotationKey> for OwnedKey {
    fn from(key: AmqpAnnotationKey) -> Self {
        match key {
            AmqpAnnotationKey::Ulong(key) => OwnedKey::Ulong(key),
            AmqpAnnotationKey::Symbol(key) => OwnedKey::Symbol(key.into()),
        }
    }
}

impl From<OwnedKey> for crate::messaging::AmqpAnnotationKey {
    fn from(key: OwnedKey) -> Self {
        match key {
            OwnedKey::Ulong(key) => crate::messaging::AmqpAnnotationKey::Ulong(key),
            OwnedKey::Symbol(key) => crate::messaging::AmqpAnnotationKey::Symbol(key.into()),
        }
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
                amqp_message_properties_builder.with_content_encoding(content_encoding.into());
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
