// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! This sample demonstrates how to send events to an Event Hub partition using the `ProducerClient` using a SAS token.

use azure_core::Uuid;
use azure_messaging_eventhubs::{models::EventData, ProducerClient, SendEventOptions};
use core::f32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Event Hub client
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    let sas_key_name = std::env::var("EVENTHUBS_SAS_KEY_NAME")?;
    let sas_key = std::env::var("EVENTHUBS_SAS_KEY")?;

    let client = ProducerClient::builder()
        .open_sas(
            eventhub_namespace.as_str(),
            eventhub_name.as_str(),
            sas_key_name,
            sas_key,
        )
        .await?;

    println!("Created producer client.");

    // Send an event to an eventhub instance directly. The message will be sent to a random partition.
    // Note that this uses an implicit builder to create the EventData being sent to the service.
    client.send_event("Hello, Event Hub!", None).await?;

    // Send an array of bytes to partition 0 of the Event Hubs instance.
    // Note that this uses an implicit builder to create the EventData being sent to the service.
    client
        .send_event(
            vec![2, 4, 8, 16],
            Some(SendEventOptions {
                partition_id: Some("0".to_string()),
            }),
        )
        .await?;

    // Send an event built using the `EventData` builder which allows for more control over the event.
    // This message will be sent to a random partition.
    client
        .send_event(
            EventData::builder()
                .with_content_type("text/plain".to_string())
                .with_correlation_id(Uuid::new_v4())
                .with_body("This is some text")
                .add_property("Event Property".to_string(), "Property Value")
                .add_property("Pi".to_string(), f32::consts::PI)
                .add_property("Binary".to_string(), vec![0x08, 0x09, 0x0A])
                .build(),
            None,
        )
        .await?;

    println!("Sent messages. Closing client.");

    client.close().await?;
    Ok(())
}
