// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This example demonstrates how to create a Service Bus client using the builder pattern.

use azure_identity::DefaultAzureCredential;

use azure_messaging_servicebus::{
    CompleteMessageOptions, CreateReceiverOptions, CreateSenderOptions, Message,
    ReceiveMessageOptions, SendMessageOptions, ServiceBusClient,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Get the namespace from environment variable
    let namespace = env::var("SERVICEBUS_NAMESPACE")
        .expect("SERVICEBUS_NAMESPACE environment variable must be set");

    // Get the queue name from environment variable
    let queue_name = env::var("SERVICEBUS_QUEUE_NAME")
        .expect("SERVICEBUS_QUEUE_NAME environment variable must be set");

    println!("Creating Service Bus client using builder pattern...");

    // Create a credential
    let credential = DefaultAzureCredential::new()?;

    // Create client using builder pattern with custom retry options
    let client = ServiceBusClient::builder()
        .open(&namespace, credential.clone())
        .await?;

    println!("Creating sender for queue: {}", queue_name);
    let sender = client
        .create_sender(&queue_name, Some(CreateSenderOptions::default()))
        .await?;

    // Create a message
    let mut message = Message::from_string("Hello, Service Bus from Rust Builder Pattern!");
    message.set_message_id("builder-example-message-1");
    message.set_property("language", "rust");
    message.set_property("example", "builder_pattern");
    message.set_property("pattern", "builder");

    println!("Sending message to queue: {}", queue_name);
    let send_options = SendMessageOptions::default();
    sender.send_message(message, Some(send_options)).await?;

    println!("Message sent successfully using builder pattern!");

    // Create a receiver to verify the message was sent
    println!("Creating receiver for queue: {}", queue_name);
    let receiver = client
        .create_receiver(&queue_name, Some(CreateReceiverOptions::default()))
        .await?;

    println!("Receiving message...");
    let receive_options = ReceiveMessageOptions::default();
    if let Some(received_message) = receiver.receive_message(Some(receive_options)).await? {
        println!("Received message: {}", received_message.body_as_string()?);
        println!("Message ID: {:?}", received_message.message_id());

        // Print custom properties
        for (key, value) in received_message.properties() {
            println!("Property {}: {}", key, value);
        }

        // Complete the message
        let complete_options = CompleteMessageOptions::default();
        receiver
            .complete_message(&received_message, Some(complete_options))
            .await?;
        println!("Message completed successfully!");
    } else {
        println!("No message received");
    }

    // Close the sender and receiver
    sender.close().await?;
    receiver.close().await?;

    // Close the client
    client.close().await?;

    Ok(())
}
