// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

pub(crate) mod message_fields;
pub(crate) mod message_source;
pub(crate) mod message_target;
pub(crate) mod messaging_types;

use crate::{
    messaging::{AmqpMessage, AmqpMessageBody, AmqpMessageProperties},
    value::AmqpValue,
};

use azure_core::{error::ErrorKind, Error};
use fe2o3_amqp_types::messaging::{message::EmptyBody, IntoBody};
use serde_amqp::{extensions::TransparentVec, Value};
use tracing::info;

impl TryInto<AmqpValue> for fe2o3_amqp_types::messaging::Data {
    type Error = std::fmt::Error;

    fn try_into(self) -> Result<AmqpValue, Self::Error> {
        Err(std::fmt::Error)
    }
}

impl TryInto<AmqpValue> for TransparentVec<fe2o3_amqp_types::messaging::Data> {
    type Error = std::fmt::Error;

    fn try_into(self) -> Result<AmqpValue, Self::Error> {
        Err(std::fmt::Error)
    }
}

impl TryInto<AmqpValue> for fe2o3_amqp_types::messaging::message::EmptyBody {
    type Error = std::fmt::Error;

    fn try_into(self) -> Result<AmqpValue, Self::Error> {
        Err(std::fmt::Error)
    }
}

impl TryInto<AmqpValue> for Vec<Vec<serde_amqp::Value>> {
    type Error = std::fmt::Error;

    fn try_into(self) -> Result<AmqpValue, Self::Error> {
        Err(std::fmt::Error)
    }
}

/*
 * Convert a fe2o3 message into an AMQP message.
 *
 * Note that this API can be used for four different scenarios:
 * 1) Body is empty
 * 2) Body is an array of binary blobs
 * 3) Body is an AMQP value
 * 4) Body is a sequence of AMQP values.
 *
 * In order to satisfy all four of these, the method is specialized on the type of body element.
 * Since the actual body is either <nothing>, a Vec<Vec<u8>> or AmqpValue or Vec<AmqpValue>
 * the T specialization is declared as being TryInto<AmqpValue>. That way, when processing the
 * empty body or the binary body, we won't call Into<AmqpValue> on the body, and when it is
 * a Vec<AmqpValue> or an AmqpValue, we will.
 *
 * TryInto<T> has a specialization where Into<T> exists, which returns an immutable error
 * and in this case there is an fe2o3 Value Into AmqpValue specialization, which means that the call to convert
 * the T value into an AmqpValue will always succeed.
 */
impl From<&fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<Value>>>
    for AmqpMessage
{
    fn from(
        message: &fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<Value>>,
    ) -> Self {
        let mut amqp_message_builder = AmqpMessage::builder();

        if let Some(application_properties) = &message.application_properties {
            amqp_message_builder =
                amqp_message_builder.with_application_properties(application_properties.into());
        }

        let body = &message.body;
        if body.is_empty() {
            let body = AmqpMessageBody::Empty;
            amqp_message_builder = amqp_message_builder.with_body(body);
        } else if body.is_data() {
            let data = body.try_as_data().unwrap();
            let body = AmqpMessageBody::Binary(data.map(|x| x.to_vec()).collect());
            amqp_message_builder = amqp_message_builder.with_body(body);
        } else if body.is_value() {
            let value = body.try_as_value().unwrap();
            // Because a conversion exists between fe2o3 values and AmqpValue types,
            // this try_into will always succeed.
            let value = Into::<AmqpValue>::into(value);
            amqp_message_builder = amqp_message_builder.with_body(AmqpMessageBody::Value(value));
        } else if body.is_sequence() {
            let sequence = body.try_as_sequence().unwrap();

            let body = AmqpMessageBody::Sequence(
                sequence
                    .map(|x| {
                        x.iter()
                            .map(|v| {
                                // Because a conversion exists between fe2o3 values and AmqpValue types,
                                // this into will always succeed.
                                Into::<AmqpValue>::into(v)
                            })
                            .collect()
                    })
                    .collect(),
            );
            amqp_message_builder = amqp_message_builder.with_body(body);
        }

        if let Some(header) = &message.header {
            amqp_message_builder = amqp_message_builder.with_header(header.into());
        }

        if let Some(properties) = &message.properties {
            amqp_message_builder =
                amqp_message_builder.with_properties(AmqpMessageProperties::from(properties));
        }

        if let Some(delivery_annotations) = &message.delivery_annotations {
            amqp_message_builder =
                amqp_message_builder.with_delivery_annotations((&delivery_annotations.0).into());
        }

        if let Some(message_annotations) = &message.message_annotations {
            amqp_message_builder =
                amqp_message_builder.with_message_annotations((&message_annotations.0).into());
        }

        if let Some(footer) = &message.footer {
            amqp_message_builder = amqp_message_builder.with_footer((&footer.0).into());
        }

        amqp_message_builder.build()
    }
}

impl From<fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<Value>>>
    for AmqpMessage
{
    fn from(
        message: fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<Value>>,
    ) -> Self {
        let mut amqp_message_builder = AmqpMessage::builder();

        if let Some(application_properties) = message.application_properties {
            amqp_message_builder =
                amqp_message_builder.with_application_properties(application_properties.into());
        }

        let body = message.body;
        if body.is_empty() {
            let body = AmqpMessageBody::Empty;
            amqp_message_builder = amqp_message_builder.with_body(body);
        } else if body.is_data() {
            let data = body.try_into_data().unwrap();
            let body = AmqpMessageBody::Binary(data.map(|x| x.to_vec()).collect());
            amqp_message_builder = amqp_message_builder.with_body(body);
        } else if body.is_value() {
            let value = body.try_into_value().unwrap();
            // Because a conversion exists between fe2o3 values and AmqpValue types,
            // this try_into will always succeed.
            let value = Into::<AmqpValue>::into(value);
            amqp_message_builder = amqp_message_builder.with_body(AmqpMessageBody::Value(value));
        } else if body.is_sequence() {
            let sequence = body.try_into_sequence().unwrap();

            let body = AmqpMessageBody::Sequence(
                sequence
                    .map(|x| {
                        x.iter()
                            .map(|v| {
                                // Because a conversion exists between fe2o3 values and AmqpValue types,
                                // this into will always succeed.
                                Into::<AmqpValue>::into(v)
                            })
                            .collect()
                    })
                    .collect(),
            );
            amqp_message_builder = amqp_message_builder.with_body(body);
        }

        if let Some(header) = message.header {
            amqp_message_builder = amqp_message_builder.with_header(header.into());
        }

        if let Some(properties) = message.properties {
            amqp_message_builder =
                amqp_message_builder.with_properties(AmqpMessageProperties::from(properties));
        }

        if let Some(delivery_annotations) = message.delivery_annotations {
            amqp_message_builder =
                amqp_message_builder.with_delivery_annotations(delivery_annotations.0.into());
        }

        if let Some(message_annotations) = message.message_annotations {
            amqp_message_builder =
                amqp_message_builder.with_message_annotations(message_annotations.0.into());
        }

        if let Some(footer) = message.footer {
            amqp_message_builder = amqp_message_builder.with_footer(footer.0.into());
        }

        amqp_message_builder.build()
    }
}

impl From<AmqpMessage>
    for fe2o3_amqp_types::messaging::Message<
        fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
    >
{
    fn from(message: AmqpMessage) -> Self {
        let message_builder = fe2o3_amqp_types::messaging::Message::builder()
            .application_properties(message.application_properties.map(Into::into))
            .properties(message.properties.map(Into::into))
            .header(message.header.map(Into::into))
            .delivery_annotations(message.delivery_annotations.map(Into::into))
            .message_annotations(message.message_annotations.map(Into::into))
            .footer(message.footer.map(Into::into));

        match message.body {
            AmqpMessageBody::Empty => message_builder
                .body(fe2o3_amqp_types::messaging::Body::Empty)
                .build(),
            AmqpMessageBody::Value(value) => {
                let value: fe2o3_amqp_types::primitives::Value = value.into();
                let value = fe2o3_amqp_types::messaging::Body::Value(value.into_body());
                let message_builder = message_builder.body(value);
                message_builder.build()
            }
            AmqpMessageBody::Binary(data) => {
                let message_builder =
                    message_builder.body(fe2o3_amqp_types::messaging::Body::Data(
                        data.into_iter()
                            .map(fe2o3_amqp_types::messaging::Data::from)
                            .collect::<TransparentVec<fe2o3_amqp_types::messaging::Data>>(),
                    ));
                message_builder.build()
            }
            AmqpMessageBody::Sequence(sequence) => {
                let message_builder =
                    message_builder.body(fe2o3_amqp_types::messaging::Body::Sequence(
                        sequence
                            .into_iter()
                            .map(|x| {
                                fe2o3_amqp_types::messaging::AmqpSequence(
                                    x.0.into_iter()
                                        .map(Into::<fe2o3_amqp_types::primitives::Value>::into)
                                        .collect(),
                                )
                            })
                            .collect(),
                    ));
                message_builder.build()
            }
        }
    }
}

impl From<&AmqpMessage>
    for fe2o3_amqp_types::messaging::Message<
        fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
    >
{
    fn from(message: &AmqpMessage) -> Self {
        let message_builder = fe2o3_amqp_types::messaging::Message::builder()
            .application_properties(message.application_properties.as_ref().map(Into::into))
            .properties(message.properties.as_ref().map(Into::into))
            .header(message.header.as_ref().map(Into::into))
            .delivery_annotations(message.delivery_annotations.as_ref().map(Into::into))
            .message_annotations(message.message_annotations.as_ref().map(Into::into))
            .footer(message.footer.as_ref().map(Into::into));

        match &(message.body) {
            AmqpMessageBody::Empty => message_builder
                .body(fe2o3_amqp_types::messaging::Body::Empty)
                .build(),
            AmqpMessageBody::Value(value) => {
                let value: fe2o3_amqp_types::primitives::Value = value.into();
                let value = fe2o3_amqp_types::messaging::Body::Value(value.into_body());
                let message_builder = message_builder.body(value);
                message_builder.build()
            }
            AmqpMessageBody::Binary(data) => {
                let message_builder =
                    message_builder.body(fe2o3_amqp_types::messaging::Body::Data(
                        data.iter()
                            .map(|d| fe2o3_amqp_types::messaging::Data::from(d.as_slice()))
                            .collect::<TransparentVec<fe2o3_amqp_types::messaging::Data>>(),
                    ));
                message_builder.build()
            }
            AmqpMessageBody::Sequence(sequence) => {
                let message_builder =
                    message_builder.body(fe2o3_amqp_types::messaging::Body::Sequence(
                        sequence
                            .iter()
                            .map(|x| {
                                fe2o3_amqp_types::messaging::AmqpSequence(
                                    x.0.iter()
                                        .map(Into::<fe2o3_amqp_types::primitives::Value>::into)
                                        .collect(),
                                )
                            })
                            .collect(),
                    ));
                message_builder.build()
            }
        }
    }
}

impl
    From<
        &fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<TransparentVec<fe2o3_amqp_types::messaging::Data>>,
        >,
    > for AmqpMessage
{
    fn from(
        message: &fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<TransparentVec<fe2o3_amqp_types::messaging::Data>>,
        >,
    ) -> Self {
        let mut amqp_message_builder = AmqpMessage::builder();

        if let Some(application_properties) = &message.application_properties {
            amqp_message_builder =
                amqp_message_builder.with_application_properties(application_properties.into());
        }

        let body = &message.body;
        if body.is_empty() {
            let body = AmqpMessageBody::Empty;
            amqp_message_builder = amqp_message_builder.with_body(body);
        } else if body.is_data() {
            let data = body
                .try_as_data()
                .map_err(|_| {
                    Error::with_message(
                        ErrorKind::DataConversion,
                        "Could not convert AMQP Message Body to data.",
                    )
                })
                .unwrap();
            let body = AmqpMessageBody::Binary(data.map(|x| x.to_vec()).collect());
            amqp_message_builder = amqp_message_builder.with_body(body);
        }

        if let Some(header) = &message.header {
            amqp_message_builder = amqp_message_builder.with_header(header.into());
        }

        if let Some(properties) = &message.properties {
            amqp_message_builder =
                amqp_message_builder.with_properties(AmqpMessageProperties::from(properties));
        }

        if let Some(delivery_annotations) = &message.delivery_annotations {
            amqp_message_builder =
                amqp_message_builder.with_delivery_annotations((&delivery_annotations.0).into());
        }

        if let Some(message_annotations) = &message.message_annotations {
            amqp_message_builder =
                amqp_message_builder.with_message_annotations((&message_annotations.0).into());
        }

        if let Some(footer) = &message.footer {
            amqp_message_builder = amqp_message_builder.with_footer((&footer.0).into());
        }

        amqp_message_builder.build()
    }
}

impl
    From<
        &fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::messaging::message::EmptyBody>,
        >,
    > for AmqpMessage
{
    fn from(
        message: &fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::messaging::message::EmptyBody>,
        >,
    ) -> Self {
        let mut amqp_message_builder = AmqpMessage::builder();

        amqp_message_builder = amqp_message_builder.with_body(AmqpMessageBody::Empty);

        if let Some(application_properties) = &message.application_properties {
            amqp_message_builder =
                amqp_message_builder.with_application_properties(application_properties.into());
        }

        if let Some(header) = &message.header {
            amqp_message_builder = amqp_message_builder.with_header(header.into());
        }

        if let Some(properties) = &message.properties {
            info!("Converting properties to AmqpMessageProperties");
            amqp_message_builder = amqp_message_builder.with_properties(properties);
        }

        if let Some(delivery_annotations) = &message.delivery_annotations {
            amqp_message_builder =
                amqp_message_builder.with_delivery_annotations((&delivery_annotations.0).into());
        }

        if let Some(message_annotations) = &message.message_annotations {
            amqp_message_builder =
                amqp_message_builder.with_message_annotations((&message_annotations.0).into());
        }

        if let Some(footer) = &message.footer {
            amqp_message_builder = amqp_message_builder.with_footer((&footer.0).into());
        }

        amqp_message_builder.build()
    }
}

impl From<AmqpMessage> for fe2o3_amqp_types::messaging::Message<EmptyBody> {
    fn from(message: AmqpMessage) -> Self {
        let message_builder = fe2o3_amqp_types::messaging::Message::builder()
            .application_properties(message.application_properties.map(Into::into))
            .header(message.header.map(Into::into))
            .delivery_annotations(message.delivery_annotations.map(Into::into))
            .message_annotations(message.message_annotations.map(Into::into))
            .footer(message.footer.map(Into::into));
        match message.body {
            AmqpMessageBody::Empty => message_builder.body(EmptyBody {}).build(),
            _ => panic!("Expected EmptyBody"),
        }
    }
}

impl From<AmqpMessage>
    for fe2o3_amqp_types::messaging::Message<
        TransparentVec<
            fe2o3_amqp_types::messaging::AmqpSequence<fe2o3_amqp_types::primitives::Value>,
        >,
    >
{
    fn from(message: AmqpMessage) -> Self {
        let message_builder = fe2o3_amqp_types::messaging::Message::builder()
            .application_properties(message.application_properties.map(Into::into))
            .header(message.header.map(Into::into))
            .delivery_annotations(message.delivery_annotations.map(Into::into))
            .message_annotations(message.message_annotations.map(Into::into))
            .footer(message.footer.map(Into::into));

        match message.body {
            AmqpMessageBody::Sequence(sequence) => {
                let sequence: Vec<
                    fe2o3_amqp_types::primitives::List<fe2o3_amqp_types::primitives::Value>,
                > = sequence
                    .into_iter()
                    .map(|x| {
                        x.0.into_iter()
                            .map(Into::<fe2o3_amqp_types::primitives::Value>::into)
                            .collect()
                    })
                    .collect();
                let message_builder = message_builder.sequence_batch(sequence);
                message_builder.build()
            }
            _ => panic!("Expected AmqpSequence"),
        }
    }
}

impl From<AmqpMessage>
    for fe2o3_amqp_types::messaging::Message<TransparentVec<fe2o3_amqp_types::messaging::Data>>
{
    fn from(message: AmqpMessage) -> Self {
        let message_builder = fe2o3_amqp_types::messaging::Message::builder()
            .application_properties(message.application_properties.map(Into::into))
            .header(message.header.map(Into::into))
            .delivery_annotations(message.delivery_annotations.map(Into::into))
            .message_annotations(message.message_annotations.map(Into::into))
            .footer(message.footer.map(Into::into));

        match message.body {
            AmqpMessageBody::Binary(data) => {
                let data: Vec<serde_bytes::ByteBuf> = data
                    .iter()
                    .map(|b| serde_bytes::ByteBuf::from(b.as_slice()))
                    .collect();
                message_builder.data_batch(data).build()
            }
            _ => panic!("Expected Data"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::messaging::{
        AmqpAnnotationKey, AmqpAnnotations, AmqpMessageHeader, AmqpMessageProperties,
    };
    use crate::value::AmqpSymbol;
    use azure_core::time::Duration;
    use fe2o3_amqp_types::messaging::{Data, MessageAnnotations};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn round_trip_message_amqp_to_fe2o3() {
        // Fe2o3->Amqp
        {
            let body: fe2o3_amqp_types::messaging::Body<Value> =
                fe2o3_amqp_types::messaging::Body::Value(fe2o3_amqp_types::messaging::AmqpValue(
                    "hello".into(),
                ));
            let fe2o3_message = fe2o3_amqp_types::messaging::Message::builder()
                .body(body)
                .header(Some(
                    fe2o3_amqp_types::messaging::Header::builder()
                        .durable(true)
                        .ttl(1000)
                        .priority(3)
                        .build(),
                ))
                .properties(
                    fe2o3_amqp_types::messaging::Properties::builder()
                        .absolute_expiry_time(
                            fe2o3_amqp_types::primitives::Timestamp::from_milliseconds(25),
                        )
                        .content_encoding("utf-8")
                        .content_type("text/json")
                        .correlation_id(String::from("correlation_id"))
                        .creation_time(fe2o3_amqp_types::primitives::Timestamp::from_milliseconds(
                            25,
                        ))
                        .group_id(Some(String::from("group_id")))
                        .group_sequence(5)
                        .message_id(fe2o3_amqp_types::messaging::MessageId::String(
                            "message_id".into(),
                        ))
                        .reply_to("reply_to")
                        .reply_to_group_id(Some(String::from("reply_to_group_id")))
                        .subject("subject")
                        .to("to")
                        .user_id(vec![1, 2, 3])
                        .build(),
                )
                .footer(Some(
                    fe2o3_amqp_types::messaging::Footer::builder()
                        .insert("foo", 123)
                        .insert("bar", 95)
                        .build(),
                ))
                .delivery_annotations(Some(
                    fe2o3_amqp_types::messaging::DeliveryAnnotations::builder()
                        .insert("foo", 123)
                        .insert("bar", 95)
                        .build(),
                ))
                .message_annotations(Some(
                    fe2o3_amqp_types::messaging::MessageAnnotations::builder()
                        .insert("foo", 123)
                        .insert("bar", 95)
                        .build(),
                ))
                .application_properties(Some(
                    fe2o3_amqp_types::messaging::ApplicationProperties::builder()
                        .insert("foo", 123)
                        .insert("bar", 95)
                        .build(),
                ))
                .build();

            let amqp_message = AmqpMessage::from(&fe2o3_message);
            let round_trip: fe2o3_amqp_types::messaging::Message<
                fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
            > = amqp_message.into();

            assert_eq!(round_trip, fe2o3_message);
        }
        // Amqp->Fe2o3
        {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;

            // Round trip timestamp through milliseconds to round down from nanoseconds.
            let timestamp: SystemTime = UNIX_EPOCH + Duration::milliseconds(timestamp);

            let amqp_message = AmqpMessage::builder()
                .add_application_property("abc".to_string(), "23 skiddoo")
                .add_application_property("What?".to_string(), 29.5)
                .with_body(AmqpValue::from("hello"))
                .with_properties(AmqpMessageProperties {
                    absolute_expiry_time: Some(timestamp.into()),
                    content_encoding: Some(AmqpSymbol::from("utf-8")),
                    content_type: Some(AmqpSymbol::from("text/plain")),
                    correlation_id: Some("abc".into()),
                    creation_time: Some(timestamp.into()),
                    group_id: Some("group".to_string()),
                    group_sequence: Some(5),
                    message_id: Some("message".into()),
                    reply_to: Some("reply".to_string()),
                    reply_to_group_id: Some("reply_group".to_string()),
                    subject: Some("subject".to_string()),
                    to: Some("to".to_string()),
                    user_id: Some(vec![39, 20, 54]),
                })
                .with_header(AmqpMessageHeader {
                    delivery_count: 95,
                    first_acquirer: true,
                    durable: true,
                    time_to_live: Some(Duration::milliseconds(1000)),
                    priority: 3,
                })
                .with_delivery_annotations(AmqpAnnotations::from(vec![
                    (AmqpAnnotationKey::from(93), 123),
                    (AmqpAnnotationKey::from(128), 95),
                ]))
                .with_message_annotations(AmqpAnnotations::from(vec![
                    (AmqpAnnotationKey::from(AmqpSymbol::from("foo")), 123),
                    (AmqpAnnotationKey::from(AmqpSymbol::from("bar")), 95),
                ]))
                .with_footer(AmqpAnnotations::from(vec![
                    (AmqpAnnotationKey::from(AmqpSymbol::from("foo")), 123),
                    (AmqpAnnotationKey::from(AmqpSymbol::from("bar")), 95),
                ]))
                .build();

            let fe2o3_message = fe2o3_amqp_types::messaging::Message::<
                fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
            >::from(amqp_message.clone());

            let round_trip = AmqpMessage::from(&fe2o3_message);
            assert_eq!(amqp_message, round_trip);
        }
    }

    #[test]
    fn convert_empty_message_to_amqp_message() {
        let body: fe2o3_amqp_types::messaging::Body<EmptyBody> =
            fe2o3_amqp_types::messaging::Body::Empty;
        let fe2o3_message = fe2o3_amqp_types::messaging::Message::builder()
            .body(body)
            .build();

        let amqp_message: AmqpMessage = (&fe2o3_message).into();
        assert_eq!(amqp_message.body, AmqpMessageBody::Empty);
        assert!(amqp_message.application_properties.is_none());
        assert!(amqp_message.header.is_none());
        assert!(amqp_message.delivery_annotations.is_none());
        assert!(amqp_message.message_annotations.is_none());
        assert!(amqp_message.footer.is_none());

        let round_trip: fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        > = amqp_message.into();

        assert!(round_trip.body.is_empty());
    }

    #[test]
    fn convert_data_message_to_amqp_message() {
        {
            let mut data = TransparentVec::new(Vec::<fe2o3_amqp_types::messaging::Data>::new());
            data.push(Data::from(vec![1, 2, 3]));

            let data: fe2o3_amqp_types::messaging::Body<
                TransparentVec<fe2o3_amqp_types::messaging::Data>,
            > = fe2o3_amqp_types::messaging::Body::Data(data);

            let fe2o3_message = fe2o3_amqp_types::messaging::Message::builder() //<
                //            fe2o3_amqp_types::messaging::Body<Vec<Data>>,
                //>::builder()
                .body(data)
                .build();

            let amqp_message: AmqpMessage = (&fe2o3_message).into();
            assert_eq!(
                amqp_message.body,
                AmqpMessageBody::Binary(vec![vec![1, 2, 3]])
            );

            assert!(amqp_message.application_properties.is_none());
            assert!(amqp_message.header.is_none());
            assert!(amqp_message.delivery_annotations.is_none());
            assert!(amqp_message.message_annotations.is_none());
            assert!(amqp_message.footer.is_none());

            let round_trip: fe2o3_amqp_types::messaging::Message<
                fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
            > = amqp_message.into();

            assert!(round_trip.body.is_data());
        }
        {
            let mut data = TransparentVec::new(Vec::<fe2o3_amqp_types::messaging::Data>::new());
            data.push(Data::from(vec![1, 2, 3]));

            let data: fe2o3_amqp_types::messaging::Body<
                TransparentVec<fe2o3_amqp_types::messaging::Data>,
            > = fe2o3_amqp_types::messaging::Body::Data(data);

            let fe2o3_message = fe2o3_amqp_types::messaging::Message::builder() //<
                //            fe2o3_amqp_types::messaging::Body<Vec<Data>>,
                //>::builder()
                .body(data)
                .message_annotations(Some(
                    MessageAnnotations::builder()
                        .insert("foo", 123)
                        .insert("bar", 95)
                        .build(),
                ))
                .build();

            let amqp_message = Into::<AmqpMessage>::into(&fe2o3_message);
            assert_eq!(
                amqp_message.body,
                AmqpMessageBody::Binary(vec![vec![1, 2, 3]])
            );
            assert!(amqp_message.application_properties.is_none());
            assert!(amqp_message.header.is_none());
            assert!(amqp_message.delivery_annotations.is_none());
            assert!(amqp_message.message_annotations.is_some());
            assert!(amqp_message.footer.is_none());

            let round_trip: fe2o3_amqp_types::messaging::Message<
                fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
            > = amqp_message.into();

            assert!(round_trip.body.is_data());
            assert!(round_trip.message_annotations.is_some());
        }
    }

    #[test]
    fn convert_value_message_to_amqp_message() {
        let body: fe2o3_amqp_types::messaging::Body<Value> =
            fe2o3_amqp_types::messaging::Body::Value(fe2o3_amqp_types::messaging::AmqpValue(
                "hello".into(),
            ));
        let fe2o3_message = fe2o3_amqp_types::messaging::Message::builder()
            .body(body)
            .build();

        let amqp_message: AmqpMessage = (&fe2o3_message).into();
        assert_eq!(
            amqp_message.body,
            AmqpMessageBody::Value(AmqpValue::String("hello".to_string()))
        );
        assert!(amqp_message.application_properties.is_none());
        assert!(amqp_message.header.is_none());
        assert!(amqp_message.delivery_annotations.is_none());
        assert!(amqp_message.message_annotations.is_none());
        assert!(amqp_message.footer.is_none());

        let round_trip: fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        > = amqp_message.into();

        assert!(round_trip.body.is_value());
    }

    #[test]
    fn convert_sequence_message_to_amqp_message() {
        let test_body = vec![vec![3, 5, 7], vec![11, 13, 17]];
        let mut seq =
            Vec::<fe2o3_amqp_types::primitives::List<fe2o3_amqp_types::primitives::Value>>::new();
        for v in test_body {
            seq.push(
                v.into_iter()
                    .map(fe2o3_amqp_types::primitives::Value::from)
                    .collect(),
            );
        }

        let amqp_seq =
            TransparentVec::<
                fe2o3_amqp_types::messaging::AmqpSequence<fe2o3_amqp_types::primitives::Value>,
            >::new(
                seq.into_iter()
                    .map(|x| {
                        x.into_iter().collect::<fe2o3_amqp_types::primitives::List<fe2o3_amqp_types::primitives::Value>>().into()
                    })
                    .collect::<Vec<
                        fe2o3_amqp_types::messaging::AmqpSequence<
                            fe2o3_amqp_types::primitives::Value,
                        >,
                    >>(),
            );

        let body = fe2o3_amqp_types::messaging::Body::Sequence(amqp_seq);

        let fe2o3_message = fe2o3_amqp_types::messaging::Message::builder()
            .body(body)
            .build();

        let amqp_message: AmqpMessage = (&fe2o3_message).into();

        // assert_eq!(
        //     *(amqp_message.body()),
        //     AmqpMessageBody::Sequence(
        //         test_body
        //             .into_iter()
        //             .map(|x| x.into_iter().map(Into::into).collect())
        //             .collect()
        //     )
        // );
        assert!(amqp_message.application_properties.is_none());
        assert!(amqp_message.header.is_none());
        assert!(amqp_message.delivery_annotations.is_none());
        assert!(amqp_message.message_annotations.is_none());
        assert!(amqp_message.footer.is_none());
        let round_trip: fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        > = amqp_message.into();

        assert!(round_trip.body.is_sequence());
    }
}
