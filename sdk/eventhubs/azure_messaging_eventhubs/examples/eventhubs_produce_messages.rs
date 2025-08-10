// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! This sample demonstrates how to send AMQP messages to an Event Hub partition using the `ProducerClient`.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::{
    models::{AmqpMessage, AmqpValue},
    ProducerClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the Event Hub client
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    let credential = DeveloperToolsCredential::new(None)?;

    let client = ProducerClient::builder()
        .open(
            eventhub_namespace.as_str(),
            eventhub_name.as_str(),
            credential.clone(),
        )
        .await?;

    println!("Created producer client.");

    // Send a message to an eventhub instance directly. The message will be sent to a random partition.
    client
        .send_message(
            AmqpMessage::builder()
                .with_body(AmqpValue::from("Hello, Event Hubs from AMQP!"))
                .build(),
            None,
        )
        .await?;

    // Send an AMQP message whose body is an array of bytes to a random partition of the Event Hubs instance.
    client
        .send_message(
            AmqpMessage::builder().with_body(vec![2, 13, 8, 16]).build(),
            None,
        )
        .await?;

    // Send an AMQP message whose body is an AMQP Value to a random partition.
    client
        .send_message(
            AmqpMessage::builder()
                .with_body(AmqpValue::from("String Value"))
                .build(),
            None,
        )
        .await?;

    println!("Sent messages. Closing client.");

    client.close().await?;
    Ok(())
}
