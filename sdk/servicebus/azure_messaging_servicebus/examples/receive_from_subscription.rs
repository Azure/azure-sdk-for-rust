// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This example demonstrates how to receive messages from a Service Bus topic subscription.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_servicebus::{CreateReceiverOptions, ReceiveMode, ServiceBusClient};
use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Get the topic and subscription names from environment variables
    let topic_name = env::var("SERVICEBUS_TOPIC_NAME")
        .expect("SERVICEBUS_TOPIC_NAME environment variable must be set");
    let subscription_name = env::var("SERVICEBUS_SUBSCRIPTION_NAME")
        .expect("SERVICEBUS_SUBSCRIPTION_NAME environment variable must be set");

    println!("Creating Service Bus client...");
    let credential = DeveloperToolsCredential::new(None)?;
    let client = ServiceBusClient::builder()
        .open("myservicebus.servicebus.windows.net", credential.clone())
        .await?;

    println!(
        "Creating receiver for topic: {} subscription: {}",
        topic_name, subscription_name
    );
    let receiver = client
        .create_receiver_for_subscription(
            &topic_name,
            &subscription_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None,
            }),
        )
        .await?;

    println!(
        "Listening for messages on topic: {} subscription: {}",
        topic_name, subscription_name
    );
    println!("Press Ctrl+C to stop...");

    // Receive messages in a loop
    loop {
        match receiver.receive_messages(10, None).await {
            Ok(messages) => {
                if messages.is_empty() {
                    println!("No messages received, waiting...");
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }

                println!("Received {} messages from subscription", messages.len());

                for message in messages {
                    println!("Message ID: {:?}", message.message_id());
                    println!("Subject: {:?}", message.system_properties().subject);
                    println!("Message body: {}", message.body_as_string()?);
                    println!("Sequence number: {:?}", message.sequence_number());
                    println!("Enqueued time: {:?}", message.enqueued_time_utc());

                    // Print custom properties
                    for (key, value) in message.properties() {
                        println!("Property {}: {}", key, value);
                    }

                    // Complete the message to remove it from the subscription
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
