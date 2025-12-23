// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! This sample demonstrates how to consume events from an Event Hub partition using the [`ConsumerClient`] using a SAS token.

use azure_core::time::Duration;
use azure_messaging_eventhubs::{
    ConsumerClient, OpenReceiverOptions, StartLocation, StartPosition,
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Event Hub client
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    let sas_key_name = std::env::var("EVENTHUBS_SAS_KEY_NAME")?;
    let sas_key = std::env::var("EVENTHUBS_SAS_KEY")?;

    let consumer = ConsumerClient::builder()
        .open_sas(
            eventhub_namespace.as_str(),
            eventhub_name,
            sas_key_name,
            sas_key,
        )
        .await?;

    println!("Opened consumer client");

    // Get the partition IDs
    let properties = consumer.get_eventhub_properties().await?;
    println!("EventHub Properties: {:?}", properties);

    // The default is to receive messages from the end of the partition, so specify a start position at the start of the partition.
    let receiver = consumer
        .open_receiver_on_partition(
            properties.partition_ids[0].clone(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::Earliest,
                    ..Default::default()
                }),
                receive_timeout: Some(Duration::seconds(5)),
                ..Default::default()
            }),
        )
        .await?;

    println!("Created receiver");

    // Create a stream of events from the receiver
    let mut receive_stream = receiver.stream_events();

    println!("Created receive stream");

    // Receive events until the receive_timeout has been reached.
    while let Some(event) = receive_stream.next().await {
        let event = event?;
        println!("Received: {:?}", event);

        println!("Partition ID: {:?}", event.partition_key());
        println!("Event offset: {:?}", event.offset());
    }

    consumer.close().await?;

    Ok(())
}
