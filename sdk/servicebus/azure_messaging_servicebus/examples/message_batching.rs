// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This example demonstrates how to use message batching with Azure Service Bus
//! to efficiently send multiple messages in a single operation.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_servicebus::{CreateMessageBatchOptions, Message, ServiceBusClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for better debugging
    tracing_subscriber::fmt::init();

    let queue_name = env::var("SERVICEBUS_QUEUE_NAME")
        .expect("SERVICEBUS_QUEUE_NAME environment variable is required");

    println!("🚀 Service Bus Message Batching Example");
    println!("Queue: {}", queue_name);

    // Create Service Bus client
    let credential = DeveloperToolsCredential::new(None)?;
    let client = ServiceBusClient::builder()
        .open("myservicebus.servicebus.windows.net", credential.clone())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Example 1: Basic message batching
    println!("\n📦 Example 1: Basic Message Batching");
    basic_message_batching(&sender).await?;

    // Example 2: Size-limited batching
    println!("\n📏 Example 2: Size-Limited Batching");
    size_limited_batching(&sender).await?;

    // Example 3: Handling batch overflow
    println!("\n🔄 Example 3: Handling Batch Overflow");
    batch_overflow_handling(&sender).await?;

    // Example 4: Batching with message properties
    println!("\n🏷️  Example 4: Batching with Message Properties");
    batching_with_properties(&sender).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("\n✅ All batching examples completed successfully!");
    Ok(())
}

/// Demonstrates basic message batching functionality
async fn basic_message_batching(
    sender: &azure_messaging_servicebus::Sender,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a batch with default settings
    let mut batch = sender.create_message_batch(None).await?;

    println!(
        "  📊 Created batch with max size: {} bytes",
        batch.maximum_size_in_bytes()
    );

    // Add multiple messages to the batch
    let messages = [
        "Hello, batch message 1!",
        "Hello, batch message 2!",
        "Hello, batch message 3!",
        "Hello, batch message 4!",
        "Hello, batch message 5!",
    ];

    for (i, message_text) in messages.iter().enumerate() {
        let message = Message::from(*message_text);

        if batch.try_add_message(message) {
            println!("  ✅ Added message {}: '{}'", i + 1, message_text);
        } else {
            println!("  ⚠️  Failed to add message {}: batch is full", i + 1);
            break;
        }
    }

    println!(
        "  📈 Batch contains {} messages, total size: {} bytes",
        batch.count(),
        batch.size_in_bytes()
    );

    // Send the entire batch in one operation
    sender.send_message_batch(batch, None).await?;
    println!("  🚀 Batch sent successfully!");

    Ok(())
}

/// Demonstrates size-limited batching
async fn size_limited_batching(
    sender: &azure_messaging_servicebus::Sender,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a batch with a small size limit for demonstration
    let options = CreateMessageBatchOptions {
        maximum_size_in_bytes: Some(1024),
    };
    let mut batch = sender.create_message_batch(Some(options)).await?;

    println!(
        "  📏 Created size-limited batch: {} bytes max",
        batch.maximum_size_in_bytes()
    );

    // Try to add messages until the batch is full
    let mut message_count = 0;
    for i in 0..50 {
        let message_text = format!(
            "Size-limited message {} with some additional content to use more space",
            i
        );
        let message = Message::from(message_text);

        if batch.try_add_message(message) {
            message_count += 1;
            println!(
                "  ✅ Added message {} (batch size: {} bytes)",
                i,
                batch.size_in_bytes()
            );
        } else {
            println!(
                "  🛑 Batch full after {} messages at {} bytes",
                message_count,
                batch.size_in_bytes()
            );
            break;
        }
    }

    if !batch.is_empty() {
        sender.send_message_batch(batch, None).await?;
        println!("  🚀 Size-limited batch sent successfully!");
    }

    Ok(())
}

/// Demonstrates handling batch overflow by creating multiple batches
async fn batch_overflow_handling(
    sender: &azure_messaging_servicebus::Sender,
) -> Result<(), Box<dyn std::error::Error>> {
    let total_messages = 20;
    let batch_options = CreateMessageBatchOptions {
        maximum_size_in_bytes: Some(2048),
    };

    let mut current_batch = sender
        .create_message_batch(Some(batch_options.clone()))
        .await?;
    let mut batch_count = 1;
    let mut messages_sent = 0;

    println!(
        "  📦 Sending {} messages using automatic batch overflow handling",
        total_messages
    );

    for i in 0..total_messages {
        let message_text = format!("Overflow handling message {} with content", i);
        let message = Message::from(message_text);

        if !current_batch.try_add_message(message.clone()) {
            // Current batch is full, send it and create a new one
            if !current_batch.is_empty() {
                let batch_message_count = current_batch.count();
                sender.send_message_batch(current_batch, None).await?;
                println!(
                    "  🚀 Sent batch {} with {} messages",
                    batch_count, batch_message_count
                );
                messages_sent += batch_message_count;
                batch_count += 1;
            }

            // Create a new batch and add the message that didn't fit
            current_batch = sender
                .create_message_batch(Some(batch_options.clone()))
                .await?;

            if !current_batch.try_add_message(message) {
                println!("  ❌ Message {} is too large for an empty batch!", i);
                continue;
            }
        }

        println!("  ✅ Added message {} to batch {}", i, batch_count);
    }

    // Send the final batch if it has messages
    if !current_batch.is_empty() {
        let final_batch_count = current_batch.count();
        messages_sent += final_batch_count;
        sender.send_message_batch(current_batch, None).await?;
        println!(
            "  🚀 Sent final batch {} with {} messages",
            batch_count, final_batch_count
        );
    }

    println!(
        "  📊 Total: {} batches sent with {} messages",
        batch_count, messages_sent
    );
    Ok(())
}

/// Demonstrates batching messages with properties
async fn batching_with_properties(
    sender: &azure_messaging_servicebus::Sender,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut batch = sender.create_message_batch(None).await?;
    let batch_id = azure_core::Uuid::new_v4().to_string();

    println!("  🏷️  Creating batch with ID: {}", batch_id);

    // Add messages with various properties
    for i in 0..3 {
        let message_text = format!("Property message {} content", i);
        let mut message = Message::from(message_text);

        // Set standard properties
        message.set_message_id(format!("prop-msg-{}-{}", batch_id, i));
        message.set_correlation_id(&batch_id);
        message.set_content_type("text/plain");
        message.set_subject(format!("Batch Message {}", i));

        // Set custom properties
        message.set_property("batch_id", &batch_id);
        message.set_property("sequence", i.to_string());
        message.set_property("priority", if i % 2 == 0 { "high" } else { "normal" });
        message.set_property("category", "demo");

        if batch.try_add_message(message) {
            println!("  ✅ Added message {} with properties", i);
        } else {
            println!("  ⚠️  Failed to add message {}: batch is full", i);
            break;
        }
    }

    println!(
        "  📈 Batch with properties contains {} messages, size: {} bytes",
        batch.count(),
        batch.size_in_bytes()
    );

    sender.send_message_batch(batch, None).await?;
    println!("  🚀 Batch with properties sent successfully!");

    Ok(())
}
