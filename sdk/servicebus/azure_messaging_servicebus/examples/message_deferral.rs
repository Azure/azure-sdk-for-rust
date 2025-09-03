// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This example demonstrates how to defer and receive deferred messages in Service Bus.
//! It shows how to:
//! 1. Send a message to a queue
//! 2. Receive and defer the message
//! 3. Attempt to receive the deferred message using its sequence number

use azure_identity::DeveloperToolsCredential;
use azure_messaging_servicebus::{CreateReceiverOptions, Message, ReceiveMode, ServiceBusClient};
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

    // Step 1: Send a message to the queue
    println!("Creating sender for queue: {}", queue_name);
    let sender = client.create_sender(&queue_name, None).await?;

    let mut message = Message::from("This message will be deferred");
    message.set_message_id("defer-example-1");
    message.set_property("example", "message_deferral");

    println!("Sending message to queue: {}", queue_name);
    sender.send_message(message, None).await?;
    println!("Message sent successfully!");

    // Step 2: Receive the message and defer it
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

        let sequence_number = received_message.sequence_number();
        println!("Sequence number: {:?}", sequence_number);

        // Defer the message
        println!("Deferring the message...");
        receiver.defer_message(&received_message, None).await?;
        println!("Message deferred successfully!");

        // Step 3: Try to receive the deferred message using its sequence number
        if let Some(seq_num) = sequence_number {
            println!(
                "Attempting to receive deferred message with sequence number: {}",
                seq_num
            );

            match receiver.receive_deferred_message(seq_num, None).await? {
                Some(deferred_message) => {
                    println!("Received deferred message!");
                    println!("Message body: {}", deferred_message.body_as_string()?);
                    println!("Message ID: {:?}", deferred_message.message_id());

                    // Complete the deferred message
                    println!("Completing deferred message...");
                    receiver.complete_message(&deferred_message, None).await?;
                    println!("Deferred message completed successfully!");
                }
                None => {
                    println!("Deferred message not found (implementation incomplete)");
                    println!("Note: Deferred message retrieval is not yet fully implemented");
                }
            }
        } else {
            println!("Message did not have a sequence number");
        }

        // Example of receiving multiple deferred messages
        let sequence_numbers = vec![123, 456, 789]; // Example sequence numbers
        println!("Attempting to receive multiple deferred messages...");
        let deferred_messages = receiver
            .receive_deferred_messages(&sequence_numbers, None)
            .await?;
        println!(
            "Received {} deferred messages (implementation incomplete)",
            deferred_messages.len()
        );
    } else {
        println!("No message received");
    }

    // Clean up
    sender.close().await?;
    receiver.close().await?;
    client.close().await?;

    Ok(())
}
