// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This example demonstrates how to receive messages from a Service Bus queue.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_servicebus::{CreateReceiverOptions, ReceiveMode, ServiceBusClient};
use std::env;
use tokio::time::{sleep, Duration};

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

    println!("Creating receiver for queue: {}", queue_name);
    let receiver = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None,
            }),
        )
        .await?;

    println!("Listening for messages on queue: {}", queue_name);
    println!("Press Ctrl+C to stop...");

    // Receive messages in a loop
    loop {
        match receiver.receive_messages(5, None).await {
            Ok(messages) => {
                if messages.is_empty() {
                    println!("No messages received, waiting...");
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }

                println!("Received {} messages", messages.len());

                for message in messages {
                    println!("Message ID: {:?}", message.message_id());
                    println!("Message body: {}", message.body_as_string()?);
                    println!("Sequence number: {:?}", message.sequence_number());
                    println!("Enqueued time: {:?}", message.enqueued_time_utc());

                    // Print custom properties
                    for (key, value) in message.properties() {
                        println!("Property {}: {}", key, value);
                    }

                    // Complete the message to remove it from the queue
                    println!("Completing message...");
                    receiver.complete_message(&message, None).await?;
                    println!("Message completed successfully");
                    println!("---");
                }
            }
            Err(e) => {
                eprintln!("Error receiving messages: {}", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
