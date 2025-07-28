// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use azure_core_amqp::{message::AmqpMessageProperties, AmqpList};
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    models::{AmqpMessage, AmqpValue, EventData, MessageId},
    {
        ConsumerClient, EventDataBatchOptions, OpenReceiverOptions, ProducerClient, StartLocation,
        StartPosition,
    },
};
use futures::stream::StreamExt;
use std::{env, error::Error};
use tracing::info;

#[recorded::test(live)]
async fn test_round_trip_batch(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const EVENTHUB_PARTITION: &str = "1";
    const TEST_NAME: &str = "test_round_trip_batch";
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let credential = recording.credential();
    let producer = ProducerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    let partition_properties = producer
        .get_partition_properties(EVENTHUB_PARTITION)
        .await?;

    info!(
        "Start receiving messages from sequence: {:?}",
        partition_properties.last_enqueued_sequence_number
    );

    let start_sequence = partition_properties.last_enqueued_sequence_number;
    let batch = producer
        .create_batch(Some(EventDataBatchOptions {
            partition_id: Some(EVENTHUB_PARTITION.to_string()),
            partition_key: Some("My Partition Key.".to_string()),
            ..Default::default()
        }))
        .await?;

    assert!(batch.try_add_event_data(
        EventData::builder()
            .with_body(b"Hello, World!")
            .add_property("Message#".to_string(), 1)
            .with_message_id(1)
            .build(),
        None
    )?);

    assert!(batch.try_add_amqp_message(
        AmqpMessage::builder()
            .with_body(AmqpValue::from("Hello, World!"))
            .add_application_property("Message#".to_string(), 2)
            .with_properties(AmqpMessageProperties {
                message_id: Some(2.into()),
                ..Default::default()
            })
            .build(),
        None,
    )?);

    assert!(batch.try_add_amqp_message(
        AmqpMessage::builder()
            .with_body(AmqpList::from(vec![
                AmqpValue::from("Hello, World!"),
                3.into(),
                5.into()
            ]))
            .add_application_property("Message#".to_string(), 3)
            .with_properties(AmqpMessageProperties {
                message_id: Some(3.into()),
                ..Default::default()
            })
            .build(),
        None,
    )?);

    assert!(batch.try_add_amqp_message(
        AmqpMessage::builder()
            .add_application_property("Message#".to_string(), 4)
            .with_properties(AmqpMessageProperties {
                message_id: Some(4.into()),
                ..Default::default()
            })
            .build(),
        None
    )?);

    assert!(producer.send_batch(batch, None).await.is_ok());

    let credential = recording.credential();
    let consumer = ConsumerClient::builder()
        .with_application_id(TEST_NAME.to_string())
        .open(host.as_str(), eventhub, credential)
        .await?;
    let receiver = consumer
        .open_receiver_on_partition(
            EVENTHUB_PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(start_sequence),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    let receive_stream = receiver.stream_events();

    let mut message_index = 1;
    let received_messages = receive_stream.take(4);
    received_messages
        .for_each(|f| {
            assert!(f.is_ok());
            let received_event_data = f.unwrap();
            info!("Received: {:?}", received_event_data);
            assert!(received_event_data.sequence_number().is_some());
            assert_eq!(
                received_event_data.sequence_number().unwrap(),
                start_sequence + message_index
            );
            if let Some(message_id) = received_event_data.event_data().message_id() {
                match *message_id {
                    MessageId::Ulong(long) => {
                        assert_eq!(long, message_index as u64);
                    }
                    _ => {
                        panic!("Expected MessageId::Ulong");
                    }
                }
            }

            message_index += 1;
            if received_event_data.event_data().body().is_none() {
                info!("AMQP Body: {:?}", received_event_data.raw_amqp_message());
            }

            futures::future::ready(())
        })
        .await;

    Ok(())
}

