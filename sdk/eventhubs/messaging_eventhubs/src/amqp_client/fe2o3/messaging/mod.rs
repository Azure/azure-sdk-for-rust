// Copyright (c) Microsoft Corp. All Rights Reserved.

// cspell: words amqp servicebus eventhub mgmt

pub mod message_fields;
pub mod message_source;
pub mod message_target;
pub mod messaging_types;

use fe2o3_amqp_types::messaging::{message::EmptyBody, IntoBody};
use serde_amqp::{extensions::TransparentVec, Value};

use crate::amqp_client::{
    messaging::{AmqpMessage, AmqpMessageBody},
    value::AmqpValue,
};

impl TryInto<AmqpValue> for fe2o3_amqp_types::messaging::Data {
    type Error = std::fmt::Error;

    fn try_into(self) -> Result<AmqpValue, Self::Error> {
        Err(std::fmt::Error::default())
    }
}

impl TryInto<AmqpValue> for TransparentVec<fe2o3_amqp_types::messaging::Data> {
    type Error = std::fmt::Error;

    fn try_into(self) -> Result<AmqpValue, Self::Error> {
        Err(std::fmt::Error::default())
    }
}

impl TryInto<AmqpValue> for fe2o3_amqp_types::messaging::message::EmptyBody {
    type Error = std::fmt::Error;

    fn try_into(self) -> Result<AmqpValue, Self::Error> {
        Err(std::fmt::Error::default())
    }
}

impl TryInto<AmqpValue> for Vec<Vec<serde_amqp::Value>> {
    type Error = std::fmt::Error;

    fn try_into(self) -> Result<AmqpValue, Self::Error> {
        Err(std::fmt::Error::default())
    }
}

fn amqp_message_from_fe2o3_message<T>(
    message: fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<T>>,
) -> AmqpMessage
where
    T: std::fmt::Debug + Clone + TryInto<AmqpValue>,
    <T as TryInto<AmqpValue>>::Error: std::fmt::Debug,
{
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
        let value = value.try_into().unwrap();
        amqp_message_builder = amqp_message_builder.with_body(AmqpMessageBody::Value(value.into()));
    } else if body.is_sequence() {
        let sequence = body.try_into_sequence().unwrap();
        let body = AmqpMessageBody::Sequence(
            sequence
                .map(|x| {
                    x.iter()
                        .map(|v| {
                            let v: AmqpValue = v.clone().try_into().unwrap();
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
        amqp_message_builder = amqp_message_builder.with_properties(properties.into());
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

impl From<fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<Value>>>
    for AmqpMessage
{
    fn from(
        message: fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<Value>>,
    ) -> Self {
        amqp_message_from_fe2o3_message(message)
    }
}

impl
    From<
        fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<TransparentVec<fe2o3_amqp_types::messaging::Data>>,
        >,
    > for AmqpMessage
{
    fn from(
        message: fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<TransparentVec<fe2o3_amqp_types::messaging::Data>>,
        >,
    ) -> Self {
        amqp_message_from_fe2o3_message(message)
    }
}

impl
    From<
        fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<
                Vec<fe2o3_amqp_types::primitives::List<fe2o3_amqp_types::primitives::Value>>,
            >,
        >,
    > for AmqpMessage
{
    fn from(
        message: fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<
                Vec<fe2o3_amqp_types::primitives::List<fe2o3_amqp_types::primitives::Value>>,
            >,
        >,
    ) -> Self {
        amqp_message_from_fe2o3_message(message)
    }
}

impl
    From<
        fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::messaging::message::EmptyBody>,
        >,
    > for AmqpMessage
{
    fn from(
        message: fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::messaging::message::EmptyBody>,
        >,
    ) -> Self {
        let mut amqp_message_builder = AmqpMessage::builder().with_body(AmqpMessageBody::Empty);

        if let Some(application_properties) = message.application_properties {
            amqp_message_builder =
                amqp_message_builder.with_application_properties(application_properties.into());
        }

        if let Some(header) = message.header {
            amqp_message_builder = amqp_message_builder.with_header(header.into());
        }

        if let Some(properties) = message.properties {
            amqp_message_builder = amqp_message_builder.with_properties(properties.into());
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
            .application_properties(message.application_properties().map(|x| x.clone().into()))
            .header(message.header().map(|x| x.clone().into()))
            .delivery_annotations(message.delivery_annotations().map(|x| x.clone().into()))
            .message_annotations(message.message_annotations().map(|x| x.clone().into()))
            .footer(message.footer().map(|x| x.clone().into()));

        match message.body() {
            AmqpMessageBody::Value(value) => {
                let value: fe2o3_amqp_types::primitives::Value = value.clone().into();
                let value = fe2o3_amqp_types::messaging::Body::Value(value.into_body());
                let message_builder = message_builder.body(value);
                let message = message_builder.build();
                return message;
            }
            AmqpMessageBody::Binary(data) => {
                let data: Vec<serde_bytes::ByteBuf> = data
                    .clone()
                    .into_iter()
                    .map(|x| serde_bytes::ByteBuf::from(x))
                    .collect();
                let message_builder =
                    message_builder.body(fe2o3_amqp_types::messaging::Body::Data(
                        data.into_iter().map(|x| x.into()).collect(),
                    ));
                let message = message_builder.build();
                return message;
            }
            AmqpMessageBody::Empty => {
                let message = message_builder
                    .body(fe2o3_amqp_types::messaging::Body::Empty)
                    .build();
                return message;
            }
            AmqpMessageBody::Sequence(sequence) => {
                let sequence: TransparentVec<
                    fe2o3_amqp_types::primitives::List<fe2o3_amqp_types::primitives::Value>,
                > = sequence
                    .into_iter()
                    .map(|x| {
                        let mut l = fe2o3_amqp_types::primitives::List::new();
                        let c =
                            x.clone().0.into_iter().map(|v| {
                                Into::<fe2o3_amqp_types::primitives::Value>::into(v.clone())
                            });
                        for v in c {
                            l.push(v);
                        }
                        l
                    })
                    .collect();
                let amqp_sequence = TransparentVec::<
                    fe2o3_amqp_types::messaging::AmqpSequence<fe2o3_amqp_types::primitives::Value>,
                >::new(
                    sequence
                        .into_iter()
                        .map(|x| {
                            let iter = x.into_iter().map(|y| y.into());
                            iter.collect::<fe2o3_amqp_types::primitives::List<
                                fe2o3_amqp_types::primitives::Value,
                            >>()
                            .into()
                        })
                        .collect::<Vec<
                            fe2o3_amqp_types::messaging::AmqpSequence<
                                fe2o3_amqp_types::primitives::Value,
                            >,
                        >>(),
                );

                let message_builder = message_builder
                    .body(fe2o3_amqp_types::messaging::Body::Sequence(amqp_sequence));
                let message = message_builder.build();
                return message;
            }
        };
    }
}

impl From<AmqpMessage> for fe2o3_amqp_types::messaging::Message<EmptyBody> {
    fn from(message: AmqpMessage) -> Self {
        let message_builder = fe2o3_amqp_types::messaging::Message::builder()
            .application_properties(message.application_properties().map(|x| x.clone().into()))
            .header(message.header().map(|x| x.clone().into()))
            .delivery_annotations(message.delivery_annotations().map(|x| x.clone().into()))
            .message_annotations(message.message_annotations().map(|x| x.clone().into()))
            .footer(message.footer().map(|x| x.clone().into()));
        match message.body() {
            AmqpMessageBody::Empty => {
                let message = message_builder.body(EmptyBody {}).build();
                return message;
            }
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
            .application_properties(message.application_properties().map(|x| x.clone().into()))
            .header(message.header().map(|x| x.clone().into()))
            .delivery_annotations(message.delivery_annotations().map(|x| x.clone().into()))
            .message_annotations(message.message_annotations().map(|x| x.clone().into()))
            .footer(message.footer().map(|x| x.clone().into()));

        match message.body() {
            AmqpMessageBody::Sequence(sequence) => {
                let sequence: Vec<
                    fe2o3_amqp_types::primitives::List<fe2o3_amqp_types::primitives::Value>,
                > = sequence
                    .into_iter()
                    .map(|x| {
                        let mut l = fe2o3_amqp_types::primitives::List::new();
                        let c =
                            x.clone().0.into_iter().map(|v| {
                                Into::<fe2o3_amqp_types::primitives::Value>::into(v.clone())
                            });
                        for v in c {
                            l.push(v);
                        }
                        l
                    })
                    .collect();
                let message_builder = message_builder.sequence_batch(sequence);
                let message = message_builder.build();
                return message;
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
            .application_properties(message.application_properties().map(|x| x.clone().into()))
            .header(message.header().map(|x| x.clone().into()))
            .delivery_annotations(message.delivery_annotations().map(|x| x.clone().into()))
            .message_annotations(message.message_annotations().map(|x| x.clone().into()))
            .footer(message.footer().map(|x| x.clone().into()));

        match message.body() {
            AmqpMessageBody::Binary(data) => {
                let data: Vec<serde_bytes::ByteBuf> = data
                    .clone()
                    .into_iter()
                    .map(|x| serde_bytes::ByteBuf::from(x))
                    .collect();
                let message = message_builder.data_batch(data).build();
                message
            }
            _ => panic!("Expected Data"),
        }
    }
}

#[cfg(test)]
mod tests {

    use fe2o3_amqp_types::messaging::Data;
    use fe2o3_amqp_types::messaging::MessageAnnotations;

    use super::*;
    #[test]
    fn convert_empty_message_to_amqp_message() {
        let body: fe2o3_amqp_types::messaging::Body<EmptyBody> =
            fe2o3_amqp_types::messaging::Body::Empty;
        let fe2o3_message = fe2o3_amqp_types::messaging::Message::builder()
            .body(body)
            .build();

        let amqp_message: AmqpMessage = fe2o3_message.into();
        assert_eq!(*amqp_message.body(), AmqpMessageBody::Empty);
        assert!(amqp_message.application_properties().is_none());
        assert!(amqp_message.header().is_none());
        assert!(amqp_message.delivery_annotations().is_none());
        assert!(amqp_message.message_annotations().is_none());
        assert!(amqp_message.footer().is_none());

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

            let amqp_message: AmqpMessage = fe2o3_message.into();
            assert_eq!(
                *(amqp_message.body()),
                AmqpMessageBody::Binary(vec![vec![1, 2, 3]])
            );

            assert!(amqp_message.application_properties().is_none());
            assert!(amqp_message.header().is_none());
            assert!(amqp_message.delivery_annotations().is_none());
            assert!(amqp_message.message_annotations().is_none());
            assert!(amqp_message.footer().is_none());

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

            let amqp_message = Into::<AmqpMessage>::into(fe2o3_message);
            assert_eq!(
                *(amqp_message.body()),
                AmqpMessageBody::Binary(vec![vec![1, 2, 3]])
            );
            assert!(amqp_message.application_properties().is_none());
            assert!(amqp_message.header().is_none());
            assert!(amqp_message.delivery_annotations().is_none());
            assert!(amqp_message.message_annotations().is_some());
            assert!(amqp_message.footer().is_none());

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

        let amqp_message: AmqpMessage = fe2o3_message.into();
        assert_eq!(
            *(amqp_message.body()),
            AmqpMessageBody::Value(AmqpValue::String("hello".to_string()))
        );
        assert!(amqp_message.application_properties().is_none());
        assert!(amqp_message.header().is_none());
        assert!(amqp_message.delivery_annotations().is_none());
        assert!(amqp_message.message_annotations().is_none());
        assert!(amqp_message.footer().is_none());

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
                        let iter = x.into_iter().map(|y| y.into());
                        iter.collect::<fe2o3_amqp_types::primitives::List<fe2o3_amqp_types::primitives::Value>>().into()
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

        let amqp_message: AmqpMessage = fe2o3_message.into();

        // assert_eq!(
        //     *(amqp_message.body()),
        //     AmqpMessageBody::Sequence(
        //         test_body
        //             .into_iter()
        //             .map(|x| x.into_iter().map(|y| y.into()).collect())
        //             .collect()
        //     )
        // );
        assert!(amqp_message.application_properties().is_none());
        assert!(amqp_message.header().is_none());
        assert!(amqp_message.delivery_annotations().is_none());
        assert!(amqp_message.message_annotations().is_none());
        assert!(amqp_message.footer().is_none());
        let round_trip: fe2o3_amqp_types::messaging::Message<
            fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>,
        > = amqp_message.into();

        assert!(round_trip.body.is_sequence());
    }
}
