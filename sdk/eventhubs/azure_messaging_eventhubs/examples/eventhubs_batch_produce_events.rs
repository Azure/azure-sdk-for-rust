// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! This sample demonstrates how to send events to all partitions using a batch sender.

use azure_core::{time::Duration, Uuid};
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::{
    models::EventData, EventDataBatchOptions, ProducerClient, RetryOptions,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Event Hubs client
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::builder()
        .with_retry_options(RetryOptions {
            initial_delay: Duration::milliseconds(100),
            ..Default::default()
        })
        .open(
            eventhub_namespace.as_str(),
            eventhub_name.as_str(),
            credential.clone(),
        )
        .await?;

    // Get the partition IDs
    let properties = client.get_eventhub_properties().await?;
    println!("EventHub Properties: {:?}", properties);

    // Create a message to send
    let message = "Hello, Event Hub!";

    // Send the message to each partition using a batch sender.
    for partition_id in properties.partition_ids {
        let batch = client
            .create_batch(Some(EventDataBatchOptions {
                partition_id: Some(partition_id.clone()),
                ..Default::default()
            }))
            .await?;
        if batch.try_add_event_data(message, None)? {
            println!("String message sent to partition: {}", partition_id);
        }

        if batch.try_add_event_data(vec![2, 4, 8, 16, 32], None)? {
            println!("Array Message sent to partition: {}", partition_id);
        }

        // Send an event built using the `EventData` builder which allows for more control over the event.
        if batch.try_add_event_data(
            EventData::builder()
                .with_content_type("text/plain".to_string())
                .with_correlation_id(Uuid::new_v4())
                .with_body("This is some text")
                .add_property("Event Property".to_string(), "Property Value")
                .add_property("Pi".to_string(), std::f32::consts::PI)
                .add_property("Binary".to_string(), vec![0x08, 0x09, 0x0A])
                .build(),
            None,
        )? {
            println!("EventData message sent to partition: {}", partition_id);
        }

        client.send_batch(&batch, None).await?;
    }

    Ok(())
}
