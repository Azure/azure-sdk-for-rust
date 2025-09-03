// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This example demonstrates how to work with dead letter queues in Service Bus using the SubQueue enum.
//! It shows how to:
//! 1. Send a message to a queue
//! 2. Receive and dead letter the message
//! 3. Use SubQueue::DeadLetter to receive the dead lettered message from the dead letter queue

use azure_identity::DeveloperToolsCredential;
use azure_messaging_servicebus::{
    CreateReceiverOptions, DeadLetterMessageOptions, Message, ReceiveMode, ServiceBusClient,
    SubQueue,
};
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

    // Step 1: Send a message to the queue
    println!("Creating sender for queue: {}", queue_name);
    let sender = client.create_sender(&queue_name, None).await?;

    let mut message = Message::from("This message will be dead lettered");
    message.set_message_id("dead-letter-example-1");
    message.set_property("example", "dead_letter_queue");

    println!("Sending message to queue: {}", queue_name);
    sender.send_message(message, None).await?;
    println!("Message sent successfully!");

    // Step 2: Receive the message and dead letter it
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

    println!("Receiving message from queue...");
    if let Some(received_message) = receiver.receive_message(None).await? {
        println!("Received message: {}", received_message.body_as_string()?);
        println!("Message ID: {:?}", received_message.message_id());

        // Dead letter the message with a reason
        println!("Dead lettering the message...");
        let dead_letter_options = DeadLetterMessageOptions {
            reason: Some("ProcessingFailed".to_string()),
            error_description: Some(
                "Message could not be processed due to invalid format".to_string(),
            ),
            properties_to_modify: None,
        };
        receiver
            .dead_letter_message(&received_message, Some(dead_letter_options))
            .await?;
        println!("Message dead lettered successfully!");
    } else {
        println!("No message received");
    }

    // Step 3: Receive the dead lettered message from the dead letter queue using SubQueue
    println!("Creating dead letter receiver for queue: {}", queue_name);
    let dead_letter_receiver = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: Some(SubQueue::DeadLetter),
            }),
        )
        .await?;

    println!("Checking dead letter queue for messages...");
    // Wait a bit for the message to appear in the dead letter queue
    sleep(Duration::from_secs(2)).await;

    match dead_letter_receiver.receive_message(None).await? {
        Some(dead_letter_message) => {
            println!("Received dead lettered message!");
            println!("Message body: {}", dead_letter_message.body_as_string()?);
            println!("Message ID: {:?}", dead_letter_message.message_id());
            println!(
                "Dead letter reason: {:?}",
                dead_letter_message.system_properties().dead_letter_reason
            );
            println!(
                "Dead letter description: {:?}",
                dead_letter_message
                    .system_properties()
                    .dead_letter_error_description
            );

            // Complete the dead lettered message to remove it from the dead letter queue
            println!("Completing dead lettered message...");
            dead_letter_receiver
                .complete_message(&dead_letter_message, None)
                .await?;
            println!("Dead lettered message completed successfully!");
        }
        None => {
            println!("No dead lettered messages found");
        }
    }

    // Clean up
    sender.close().await?;
    receiver.close().await?;
    dead_letter_receiver.close().await?;
    client.close().await?;

    Ok(())
}
