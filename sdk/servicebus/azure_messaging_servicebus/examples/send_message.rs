// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This example demonstrates how to send a message to a Service Bus queue.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_servicebus::{Message, ServiceBusClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Get the queue name from environment variable
    let queue_name = env::var("SERVICEBUS_QUEUE_NAME")
        .expect("SERVICEBUS_QUEUE_NAME environment variable must be set");

    println!("Creating Service Bus client...");
    let credential = DeveloperToolsCredential::new(None)?;
    let client = ServiceBusClient::builder()
        .open("myservicebus.servicebus.windows.net", credential.clone())
        .await?;

    println!("Creating sender for queue: {}", queue_name);
    let sender = client.create_sender(&queue_name, None).await?;

    // Create a message
    let mut message = Message::from("Hello, Service Bus from Rust!");
    message.set_message_id("example-message-1");
    message.set_property("language", "rust");
    message.set_property("example", "send_message");

    println!("Sending message to queue: {}", queue_name);
    sender.send_message(message, None).await?;

    println!("Message sent successfully!");

    // Close the sender
    sender.close().await?;

    // Close the client
    client.close().await?;

    Ok(())
}
