// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Tests for Service Bus authentication methods.

mod common;

use azure_core::time::Duration;
use azure_core_test::{recorded, TestContext};
use azure_messaging_servicebus::{
    CreateMessageBatchOptions, CreateReceiverOptions, Message, ReceiveMessageOptions, ReceiveMode,
    ServiceBusClient,
};
use common::{get_queue_name, get_servicebus_namespace};
use std::{env, error::Error};
use uuid::Uuid;

#[recorded::test(live)]
async fn test_token_credential_batch_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing batch operations with TokenCredential");

    // Use recording credential for this test
    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Send multiple messages
    let sender = client.create_sender(&queue_name, None).await?;
    let message_count = 3;
    let mut messages = Vec::new();

    for i in 0..message_count {
        let message_id = format!("token-batch-{}-{}", Uuid::new_v4(), i);
        let mut message = Message::from(format!("TokenCredential batch message {}", i));
        message.set_message_id(&message_id);
        message.set_property("credential_type", "DeveloperToolsCredential");
        message.set_property("test_name", "test_token_credential_batch_operations");
        message.set_property("batch_index", i.to_string());
        messages.push(message);
    }

    sender.send_messages(messages, None).await?;
    println!(
        "Sent {} messages successfully with TokenCredential",
        message_count
    );

    // Receive all messages
    let receiver = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None,
            }),
        )
        .await?;
    let received_messages = receiver.receive_messages(message_count, None).await?;

    assert_eq!(
        received_messages.len(),
        message_count,
        "Should receive all sent messages"
    );

    // Verify and complete all messages
    for message in received_messages.iter() {
        if let Some(cred_type) = message.property("credential_type") {
            assert_eq!(cred_type, "DeveloperToolsCredential");
        }

        receiver.complete_message(message, None).await?;
    }

    println!(
        "Received and completed {} messages successfully",
        received_messages.len()
    );

    // Clean up
    receiver.close().await?;
    sender.close().await?;
    client.close().await?;

    println!("TokenCredential batch operations test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_message_batch_send_receive(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing message batch send and receive for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Create a batch and add multiple messages
    let mut batch = sender.create_message_batch(None).await?;
    let batch_id = Uuid::new_v4().to_string();
    let mut expected_messages = Vec::new();

    println!("Creating batch with batch ID: {}", batch_id);

    // Add 5 messages to the batch
    for i in 0..5 {
        let message_id = format!("batch-{}-msg-{}", batch_id, i);
        let message_body = format!("Batch message {} content", i);

        let mut message = Message::from(message_body.clone());
        message.set_message_id(&message_id);
        message.set_correlation_id(&batch_id);
        message.set_property("batch_id", &batch_id);
        message.set_property("sequence", i.to_string());
        message.set_property("test_type", "batch_send_receive");

        if batch.try_add_message(message) {
            expected_messages.push((message_id, message_body));
            println!("Added message {} to batch", i);
        } else {
            panic!("Failed to add message {} to batch", i);
        }
    }

    assert_eq!(batch.count(), 5);
    assert!(!batch.is_empty());
    println!(
        "Batch contains {} messages, size: {} bytes",
        batch.count(),
        batch.size_in_bytes()
    );

    // Send the entire batch
    sender.send_message_batch(batch, None).await?;
    println!("Batch sent successfully");

    // Create receiver and receive the messages
    let receiver = client.create_receiver(&queue_name, None).await?;

    // Receive messages - may come back in any order
    let mut received_messages = Vec::new();
    let mut attempts = 0;
    while received_messages.len() < 5 && attempts < 10 {
        let messages = receiver.receive_messages(5, None).await?;
        for msg in messages {
            if msg.correlation_id() == Some(&batch_id) {
                received_messages.push(msg);
            }
        }
        attempts += 1;
        if received_messages.len() < 5 {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    assert_eq!(
        received_messages.len(),
        5,
        "Should receive all 5 batch messages"
    );
    println!("Received {} messages from batch", received_messages.len());

    // Verify all expected messages were received
    for (expected_id, expected_body) in expected_messages {
        let found = received_messages
            .iter()
            .find(|msg| msg.message_id() == Some(&expected_id));
        assert!(found.is_some(), "Message with ID {} not found", expected_id);

        let msg = found.unwrap();
        assert_eq!(msg.body_as_string().unwrap(), expected_body);
        assert_eq!(msg.correlation_id(), Some(&batch_id));
        assert_eq!(msg.property("batch_id"), Some(&batch_id));

        // Complete the message
        receiver.complete_message(msg, None).await?;
        println!("Verified and completed message: {}", expected_id);
    }

    // Clean up
    receiver.close().await?;
    sender.close().await?;
    client.close().await?;

    println!("Message batch test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_batch_size_limits(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing batch size limits");

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Create a small batch to test size limits
    let options = CreateMessageBatchOptions {
        maximum_size_in_bytes: Some(2048),
    };

    let mut batch = sender.create_message_batch(Some(options)).await?;

    assert_eq!(batch.maximum_size_in_bytes(), 1024);
    println!(
        "Created batch with size limit: {} bytes",
        batch.maximum_size_in_bytes()
    );

    // Add messages until the batch is full
    let mut added_count = 0;
    let batch_id = Uuid::new_v4().to_string();

    for i in 0..100 {
        let message_body = format!(
            "Size limit test message {} with some content to use space",
            i
        );
        let mut message = Message::from(message_body);
        message.set_message_id(format!("size-test-{}-{}", batch_id, i));
        message.set_correlation_id(&batch_id);
        message.set_property("test_type", "size_limits");

        if batch.try_add_message(message) {
            added_count += 1;
            println!(
                "Added message {} (batch size: {} bytes)",
                i,
                batch.size_in_bytes()
            );
        } else {
            println!(
                "Batch full after {} messages, current size: {} bytes",
                added_count,
                batch.size_in_bytes()
            );
            break;
        }
    }

    // Should have added some messages but not all 100
    assert!(added_count > 0, "Should have added at least one message");
    assert!(
        added_count < 100,
        "Should not have added all 100 messages due to size limit"
    );

    // Send the batch if it's not empty
    if !batch.is_empty() {
        sender.send_message_batch(batch, None).await?;
        println!(
            "Size-limited batch sent successfully with {} messages",
            added_count
        );

        // Clean up the messages
        let receiver = client.create_receiver(&queue_name, None).await?;
        let messages = receiver
            .receive_messages(added_count as usize, None)
            .await?;

        for msg in messages {
            if msg.correlation_id() == Some(&batch_id) {
                receiver.complete_message(&msg, None).await?;
            }
        }
        receiver.close().await?;
    }

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Batch size limit test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_empty_batch_handling(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing empty batch handling");

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Create an empty batch
    let batch = sender.create_message_batch(None).await?;
    assert!(batch.is_empty());
    assert_eq!(batch.count(), 0);

    // Sending an empty batch should succeed but do nothing
    sender.send_message_batch(batch, None).await?;
    println!("Empty batch handled successfully");

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Empty batch test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_batch_with_different_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing batch creation with different options");

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Test default batch options
    let mut default_batch = sender.create_message_batch(None).await?;
    assert!(default_batch.maximum_size_in_bytes() > 0);

    // Test batch with custom size
    let custom_options = CreateMessageBatchOptions {
        maximum_size_in_bytes: Some(2048),
    };

    let mut custom_batch = sender.create_message_batch(Some(custom_options)).await?;
    assert_eq!(custom_batch.maximum_size_in_bytes(), 2048);

    // Test that we can add messages to both batches
    let mut message = Message::from("Test message for batch options");
    message.set_message_id(Uuid::new_v4().to_string());
    message.set_property("test_type", "batch_options");

    // Clone message for both batches
    let message_copy = Message::from("Test message for batch options");
    let mut message_copy = message_copy;
    message_copy.set_message_id(Uuid::new_v4().to_string());
    message_copy.set_property("test_type", "batch_options");

    assert!(default_batch.try_add_message(message));
    assert!(custom_batch.try_add_message(message_copy));

    // Send both batches
    sender.send_message_batch(default_batch, None).await?;
    sender.send_message_batch(custom_batch, None).await?;

    // Clean up the messages
    let receiver = client.create_receiver(&queue_name, None).await?;
    let messages = receiver.receive_messages(5, None).await?;

    for msg in messages {
        if msg.property("test_type") == Some(&"batch_options".to_string()) {
            receiver.complete_message(&msg, None).await?;
        }
    }

    // Clean up
    receiver.close().await?;
    sender.close().await?;
    client.close().await?;

    println!("Batch options test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_batch_overflow_handling(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing batch overflow handling");

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Create a very small batch to force overflow
    let options = CreateMessageBatchOptions {
        maximum_size_in_bytes: Some(512),
    };

    let mut batch = sender.create_message_batch(Some(options)).await?;

    let batch_id = Uuid::new_v4().to_string();
    let mut batches_sent = 0;
    let mut total_messages_sent = 0;

    // Try to add many messages, creating new batches when overflow occurs
    for i in 0..20 {
        let message_body = format!(
            "Overflow test message {} with substantial content to trigger overflow",
            i
        );
        let mut message = Message::from(message_body.clone());
        message.set_message_id(format!("overflow-{}-{}", batch_id, i));
        message.set_correlation_id(&batch_id);
        message.set_property("test_type", "overflow_handling");

        if !batch.try_add_message(message) {
            // Current batch is full, send it and create a new one
            if !batch.is_empty() {
                let message_count = batch.count();
                sender.send_message_batch(batch, None).await?;
                batches_sent += 1;
                total_messages_sent += message_count;
                println!("Sent batch {} with messages", batches_sent);
            }

            let options = azure_messaging_servicebus::CreateMessageBatchOptions {
                maximum_size_in_bytes: Some(2048),
            };

            // Create new batch and add the message
            batch = sender.create_message_batch(Some(options)).await?;

            let mut retry_message = Message::from(message_body.clone());
            retry_message.set_message_id(format!("overflow-{}-{}", batch_id, i));
            retry_message.set_correlation_id(&batch_id);
            retry_message.set_property("test_type", "overflow_handling");

            if !batch.try_add_message(retry_message) {
                println!("Message too large even for new batch, skipping");
            }
        }
    }

    // Send the final batch if it has messages
    if !batch.is_empty() {
        total_messages_sent += batch.count();
        sender.send_message_batch(batch, None).await?;
        batches_sent += 1;
    }

    println!(
        "Sent {} batches with total {} messages",
        batches_sent, total_messages_sent
    );

    // Clean up the messages
    let receiver = client.create_receiver(&queue_name, None).await?;
    let mut cleanup_attempts = 0;
    while cleanup_attempts < 5 {
        let cleanup_options = ReceiveMessageOptions {
            max_message_count: 10,
            max_wait_time: Some(Duration::seconds(5)), // Short timeout for cleanup
        };
        let messages = receiver.receive_messages(10, Some(cleanup_options)).await?;
        let mut found_test_messages = false;

        for msg in messages {
            if msg.correlation_id() == Some(&batch_id) {
                receiver.complete_message(&msg, None).await?;
                found_test_messages = true;
            }
        }

        if !found_test_messages {
            break;
        }
        cleanup_attempts += 1;
    }

    // Clean up
    receiver.close().await?;
    sender.close().await?;
    client.close().await?;

    println!("Batch overflow handling test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_multiple_senders_receivers(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing multiple senders and receivers from same client");

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Create multiple senders and receivers
    let sender1 = client.create_sender(&queue_name, None).await?;
    let sender2 = client.create_sender(&queue_name, None).await?;
    let receiver1 = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None,
            }),
        )
        .await?;
    let receiver2 = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None,
            }),
        )
        .await?;

    // Send messages from different senders
    let message_id_1 = Uuid::new_v4().to_string();
    let message_id_2 = Uuid::new_v4().to_string();

    let mut message1 = Message::from("Message from sender 1");
    message1.set_message_id(&message_id_1);

    let mut message2 = Message::from("Message from sender 2");
    message2.set_message_id(&message_id_2);

    sender1.send_message(message1, None).await?;
    sender2.send_message(message2, None).await?;

    // Receive messages with different receivers
    let messages1 = receiver1.receive_messages(1, None).await?;
    let messages2 = receiver2.receive_messages(1, None).await?;

    // Complete messages
    if let Some(msg) = messages1.first() {
        receiver1.complete_message(msg, None).await?;
    }
    if let Some(msg) = messages2.first() {
        receiver2.complete_message(msg, None).await?;
    }

    // Clean up
    receiver2.close().await?;
    receiver1.close().await?;
    sender2.close().await?;
    sender1.close().await?;
    client.close().await?;

    println!("Multiple senders/receivers test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_receiver_lifecycle(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing receiver lifecycle operations");

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Create and close receiver multiple times
    for i in 0..3 {
        // Send a message for this iteration
        let message_id = format!("receiver-lifecycle-{}-{}", Uuid::new_v4(), i);
        let mut message = Message::from(format!("Receiver lifecycle test {}", i));
        message.set_message_id(&message_id);
        message.set_property("test_type", "receiver_lifecycle");

        sender.send_message(message, None).await?;

        // Create receiver, receive message, and close
        let receiver = client
            .create_receiver(
                &queue_name,
                Some(CreateReceiverOptions {
                    receive_mode: ReceiveMode::PeekLock,
                    sub_queue: None,
                }),
            )
            .await?;
        let messages = receiver.receive_messages(1, None).await?;

        if let Some(received_message) = messages.first() {
            receiver.complete_message(received_message, None).await?;
        }

        receiver.close().await?;

        println!("Receiver lifecycle iteration {} completed", i);
    }

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Receiver lifecycle test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_basic_send_receive_round_trip(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing basic send/receive round trip for queue: {}",
        queue_name
    );

    // Create client with recording credential (consistent with EventHubs pattern)
    let credential = recording.credential();
    let client = ServiceBusClient::builder()
        .open(&namespace, credential.clone())
        .await?;

    // Purge any existing messages from the queue to ensure clean test
    let purge_receiver = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::ReceiveAndDelete,
                sub_queue: None,
            }),
        )
        .await?;

    loop {
        let purge_options = ReceiveMessageOptions {
            max_message_count: 10,
            max_wait_time: Some(Duration::seconds(2)), // Short timeout for cleanup
        };
        let purge_messages = purge_receiver
            .receive_messages(10, Some(purge_options))
            .await?;
        if purge_messages.is_empty() {
            break;
        }
        println!("Purged {} existing messages", purge_messages.len());
    }
    purge_receiver.close().await?;

    // Send a test message
    let message_id = Uuid::new_v4().to_string();
    let sender = client.create_sender(&queue_name, None).await?;

    let mut message = Message::from("Hello, Service Bus!");
    message.set_message_id(&message_id);
    message.set_property("test_property", "test_value");
    message.set_property("test_type", "round_trip");

    sender.send_message(message, None).await?;
    println!("Message sent successfully");

    // Receive the message
    let receiver = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None,
            }),
        )
        .await?;
    let messages = receiver.receive_messages(10, None).await?; // Try to get up to 10 messages

    println!("Received {} messages", messages.len());
    for (i, msg) in messages.iter().enumerate() {
        println!(
            "Message {}: ID = {:?}, Body = {:?}",
            i,
            msg.message_id(),
            msg.body_as_string()
        );
    }

    assert!(
        !messages.is_empty(),
        "Should receive the message we just sent"
    );

    // Find our message by content since IDs might not match due to recording framework
    let our_message = messages
        .iter()
        .find(|msg| msg.body_as_string().unwrap_or_default() == "Hello, Service Bus!");

    assert!(
        our_message.is_some(),
        "Should find our message by body content"
    );
    let received_message = our_message.unwrap();
    assert_eq!(
        received_message.body_as_string()?,
        "Hello, Service Bus!",
        "Message body should match"
    );
    assert_eq!(
        received_message.property("test_property"),
        Some("test_value".to_string()).as_ref(),
        "Custom property should match"
    );

    // Complete the message
    receiver.complete_message(received_message, None).await?;
    println!("Message completed successfully");

    // Clean up
    receiver.close().await?;
    sender.close().await?;
    client.close().await?;

    println!("Basic round trip test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_batch_send(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing batch send for queue: {}", queue_name);

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    let mut batch = sender.create_message_batch(None).await?;
    let batch_id = Uuid::new_v4().to_string();

    for i in 0..3 {
        let message_id = format!("batch-{}-{}", batch_id, i);
        let mut message = Message::from(format!("Batch message {}", i));
        message.set_message_id(&message_id);
        message.set_correlation_id(&batch_id);
        message.set_property("test_type", "batch_test");
        message.set_property("sequence", i.to_string());

        let added = batch.try_add_message(message);
        assert!(added, "Should be able to add message {} to batch", i);
    }

    assert_eq!(batch.count(), 3);
    println!("Batch contains {} messages", batch.count());

    sender.send_message_batch(batch, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Batch send test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_batch_partitioned_queue(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing partitioned queue batch send for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    let mut batch = sender.create_message_batch(None).await?;
    let session_id = format!("partition-session-{}", Uuid::new_v4());

    // Create messages with the same session ID to ensure they go to the same partition
    for i in 0..5 {
        let message_id = format!("partitioned-{}-{}", session_id, i);
        let mut message = Message::from(format!("Partitioned message {}", i));
        message.set_message_id(&message_id);
        message.set_session_id(&session_id);
        message.set_property("test_type", "partitioned_batch");
        message.set_property("sequence", i.to_string());

        let added = batch.try_add_message(message);
        assert!(
            added,
            "Should be able to add partitioned message {} to batch",
            i
        );
    }

    assert_eq!(batch.count(), 5);
    sender.send_message_batch(batch, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Partitioned queue batch test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_batch_size_limits_alt(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing batch size limits for queue: {}", queue_name);

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Create a batch with a small size limit for testing
    let options = azure_messaging_servicebus::CreateMessageBatchOptions {
        maximum_size_in_bytes: Some(2048),
    };

    let mut batch = sender.create_message_batch(Some(options)).await?;

    // Add messages until batch is full
    let mut added_count = 0;
    for i in 0..50 {
        let message_text = format!("Size test message {} with some content to use space", i);
        let mut message = Message::from(message_text);
        message.set_message_id(format!("size-test-{}", i));
        message.set_property("test_type", "size_limits");
        message.set_property("sequence", i.to_string());

        if batch.try_add_message(message) {
            added_count += 1;
        } else {
            println!("Batch became full after {} messages", added_count);
            break;
        }
    }

    assert!(
        added_count > 0,
        "Should have been able to add at least one message"
    );
    assert!(
        added_count < 50,
        "Batch should have size limits preventing all messages"
    );

    println!(
        "Successfully added {} messages to size-limited batch",
        added_count
    );
    sender.send_message_batch(batch, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Batch size limits test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_batch_empty_batch(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing empty batch send for queue: {}", queue_name);

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    let batch = sender.create_message_batch(None).await?;

    assert!(batch.is_empty());
    assert_eq!(batch.count(), 0);

    // Sending empty batch should succeed (no-op)
    sender.send_message_batch(batch, None).await?;
    println!("Empty batch sent successfully (no-op)");

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Empty batch test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_batch_single_message_batch(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing single message batch send for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    let mut batch = sender.create_message_batch(None).await?;

    let mut message = Message::from("Single message in batch test");
    message.set_message_id("single-batch-message-id");
    message.set_property("test_type", "single_message_batch");

    let added = batch.try_add_message(message);
    assert!(added, "Should be able to add single message to batch");
    assert_eq!(batch.count(), 1);
    assert!(!batch.is_empty());

    sender.send_message_batch(batch, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Single message batch test completed successfully");
    Ok(())
}
