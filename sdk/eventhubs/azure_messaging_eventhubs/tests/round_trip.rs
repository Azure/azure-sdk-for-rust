// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//cspell: words eventhubs eventhub eventdata amqp

#![cfg(all(test, feature = "test_e2e"))] // to run this, do: `cargo test --features test_e2e`

use async_std::stream::StreamExt;
use azure_core_amqp::{
    messaging::{AmqpMessage, AmqpMessageProperties},
    value::{AmqpList, AmqpValue},
};
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
use azure_messaging_eventhubs::{
    consumer::{ConsumerClient, ConsumerClientOptions, ReceiveOptions},
    models::{EventData, MessageId, StartPosition},
    producer::{batch::EventDataBatchOptions, ProducerClient, ProducerClientOptions},
};
use futures::pin_mut;
use std::env;
use tracing::info;

mod common;

#[tokio::test]
async fn test_round_trip_batch() {
    const EVENTHUB_PARTITION: &str = "1";
    const TEST_NAME: &str = "test_round_trip_batch";
    common::setup();
    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();
    let producer = ProducerClient::new(
        host.clone(),
        eventhub.clone(),
        DefaultAzureCredential::create(TokenCredentialOptions::default()).unwrap(),
        ProducerClientOptions::builder()
            .with_application_id(TEST_NAME)
            .build(),
    );

    assert!(producer.open().await.is_ok());

    let partition_properties = producer
        .get_partition_properties(EVENTHUB_PARTITION)
        .await
        .unwrap();

    info!(
        "Start receiving messages from sequence: {:?}",
        partition_properties.last_enqueued_sequence_number
    );

    let start_sequence = partition_properties.last_enqueued_sequence_number;
    let mut batch = producer
        .create_batch(Some(
            EventDataBatchOptions::builder()
                .with_partition_key("My Partition Key.")
                .with_partition_id(EVENTHUB_PARTITION)
                .build(),
        ))
        .await
        .unwrap();

    assert!(batch
        .try_add_event_data(
            EventData::builder()
                .with_body(b"Hello, World!")
                .add_property("Message#", 1)
                .with_message_id(1)
                .build(),
            None
        )
        .unwrap());

    assert!(batch
        .try_add_amqp_message(
            AmqpMessage::builder()
                .with_body(AmqpValue::from("Hello, World!"))
                .add_application_property("Message#", 2)
                .with_properties(AmqpMessageProperties::builder().with_message_id(2).build())
                .build(),
            None,
        )
        .unwrap());

    assert!(batch
        .try_add_amqp_message(
            AmqpMessage::builder()
                .with_body(AmqpList::from(vec![
                    AmqpValue::from("Hello, World!"),
                    3.into(),
                    5.into()
                ]))
                .add_application_property("Message#", 3)
                .with_properties(AmqpMessageProperties::builder().with_message_id(3).build())
                .build(),
            None,
        )
        .unwrap());

    assert!(batch
        .try_add_amqp_message(
            AmqpMessage::builder()
                .add_application_property("Message#", 4)
                .with_properties(AmqpMessageProperties::builder().with_message_id(4).build())
                .build(),
            None
        )
        .unwrap());

    assert!(producer.submit_batch(&batch).await.is_ok());

    let consumer = ConsumerClient::new(
        host,
        eventhub,
        None,
        DefaultAzureCredential::create(TokenCredentialOptions::default()).unwrap(),
        Some(
            ConsumerClientOptions::builder()
                .with_application_id(TEST_NAME)
                .build(),
        ),
    );

    assert!(consumer.open().await.is_ok());

    let receive_stream = consumer
        .receive_events_on_partition(
            EVENTHUB_PARTITION,
            Some(
                ReceiveOptions::builder()
                    .with_start_position(
                        StartPosition::builder()
                            .with_sequence_number(start_sequence)
                            .build(),
                    )
                    .build(),
            ),
        )
        .await;

    pin_mut!(receive_stream);

    let mut message_index = 1;
    let received_messages = receive_stream.take(4);
    received_messages
        .for_each(|f| {
            assert!(f.is_ok());
            let received_event_data = f.unwrap();
            info!("Received: {:?}", received_event_data);
            assert_eq!(
                received_event_data.sequence_number(),
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
        })
        .await;
}
