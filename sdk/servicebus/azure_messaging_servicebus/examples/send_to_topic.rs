// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This example demonstrates how to send messages to a Service Bus topic.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_servicebus::{Message, ServiceBusClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Get the topic name from environment variable
    let topic_name = env::var("SERVICEBUS_TOPIC_NAME")
        .expect("SERVICEBUS_TOPIC_NAME environment variable must be set");

    println!("Creating Service Bus client...");
    let credential = DeveloperToolsCredential::new(None)?;
    let client = ServiceBusClient::builder()
        .open("myservicebus.servicebus.windows.net", credential.clone())
        .await?;

    println!("Creating sender for topic: {}", topic_name);
    let sender = client.create_sender(&topic_name, None).await?;

    // Send multiple messages
    for i in 1..=5 {
        let mut message = Message::from(format!("Message {} to topic", i));
        message.set_message_id(format!("topic-message-{}", i));
        message.set_property("message_number", i.to_string());
        message.set_property("example", "send_to_topic");
        message.set_subject("example-message");

        println!("Sending message {} to topic: {}", i, topic_name);
        sender.send_message(message, None).await?;
    }

    println!("All messages sent successfully to topic!");

    // Close the sender
    sender.close().await?;

    // Close the client
    client.close().await?;

    Ok(())
}
