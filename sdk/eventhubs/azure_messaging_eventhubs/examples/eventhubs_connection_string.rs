// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! This sample demonstrates connection-string (Shared Access Signature)
//! authentication. It opens both a [`ProducerClient`] and a [`ConsumerClient`]
//! with `open_with_connection_string`, sends a uniquely tagged event, and reads
//! it back, confirming the broker accepts the SAS token end to end.
//!
//! Environment:
//!   EVENTHUBS_CONNECTION_STRING  required, e.g.
//!     `Endpoint=sb://<ns>.servicebus.windows.net/;SharedAccessKeyName=<policy>;SharedAccessKey=<key>`
//!   EVENTHUB_NAME                required only if the connection string has no `EntityPath`

use azure_core::{time::Duration, Uuid};
use azure_messaging_eventhubs::{
    ConsumerClient, OpenReceiverOptions, ProducerClient, SendEventOptions, StartLocation,
    StartPosition,
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = std::env::var("EVENTHUBS_CONNECTION_STRING")?;
    // `None` when the connection string already carries an `EntityPath`.
    let eventhub_name = std::env::var("EVENTHUB_NAME").ok();

    let producer = ProducerClient::builder()
        .open_with_connection_string(&connection_string, eventhub_name.as_deref())
        .await?;
    println!("Opened producer via connection string.");

    // Pick a partition and capture its current tail, so we read only events we
    // enqueue after this point (no history scan, no race with the send).
    let properties = producer.get_eventhub_properties().await?;
    let partition_id = properties.partition_ids[0].clone();
    let before = producer.get_partition_properties(&partition_id).await?;
    let start_sequence = before.last_enqueued_sequence_number;

    let marker = Uuid::new_v4().to_string();
    producer
        .send_event(
            marker.clone(),
            Some(SendEventOptions {
                partition_id: Some(partition_id.clone()),
            }),
        )
        .await?;
    println!("Sent event with marker {marker} to partition {partition_id}.");

    let consumer = ConsumerClient::builder()
        .open_with_connection_string(&connection_string, eventhub_name.clone())
        .await?;
    println!("Opened consumer via connection string.");

    let receiver = consumer
        .open_receiver_on_partition(
            partition_id.clone(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(start_sequence),
                    inclusive: false,
                }),
                receive_timeout: Some(Duration::seconds(30)),
                ..Default::default()
            }),
        )
        .await?;

    let mut stream = receiver.stream_events();
    let mut found = false;
    while let Some(event) = stream.next().await {
        let event = event?;
        if event.event_data().body() == Some(marker.as_bytes()) {
            found = true;
            println!("Received the marker event back.");
            break;
        }
    }

    consumer.close().await?;
    producer.close().await?;

    if found {
        println!("PASS: connection-string SAS authentication validated end to end.");
        Ok(())
    } else {
        Err("FAIL: did not receive the marker event within the timeout".into())
    }
}
