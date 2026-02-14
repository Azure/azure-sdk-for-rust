// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Tests for Service Bus authentication methods.

mod common;

use azure_core::{time::Duration, Uuid};
use azure_core_test::{recorded, TestContext};
use azure_messaging_servicebus::{
    CreateReceiverOptions, Message, ReceiveMessageOptions, ReceiveMode, ServiceBusClient,
};
use common::{get_queue_name, get_servicebus_namespace};
use std::{env, error::Error};
use time::OffsetDateTime;

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
async fn test_send_verify_receive_verify_empty(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing send -> verify queue has message -> receive -> verify queue is empty for queue: {}",
        queue_name
    );

    // Create client with recording credential
    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Step 1: Purge any existing messages from the queue to ensure clean test
    println!("Purging any existing messages from queue");
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

    // Step 2: Verify queue is initially empty
    println!("Verifying queue is initially empty");
    let verify_receiver = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None,
            }),
        )
        .await?;

    let empty_check_options = ReceiveMessageOptions {
        max_message_count: 1,
        max_wait_time: Some(Duration::seconds(2)), // Short timeout to check if empty
    };
    let initial_messages = verify_receiver
        .receive_messages(1, Some(empty_check_options))
        .await?;
    assert!(
        initial_messages.is_empty(),
        "Queue should be empty at start, but found {} messages",
        initial_messages.len()
    );
    println!("✓ Confirmed queue is empty");

    // Step 3: Send a test message
    let message_id = Uuid::new_v4().to_string();
    let sender = client.create_sender(&queue_name, None).await?;

    let mut message = Message::from("Test message for send-verify-receive-verify cycle");
    message.set_message_id(&message_id);
    message.set_property("test_type", "send_verify_receive_verify");
    message.set_property("timestamp", OffsetDateTime::now_utc().to_string());

    sender.send_message(message, None).await?;
    println!("✓ Message sent successfully with ID: {}", message_id);

    // Step 4: Verify our message is in the queue
    println!("Verifying our message is in the queue");

    // Wait a moment for message to be available
    tokio::time::sleep(std::time::Duration::from_millis(3000)).await;

    // Create a peek receiver to verify without consuming the message
    let peek_receiver = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None,
            }),
        )
        .await?;

    let verify_options = ReceiveMessageOptions {
        max_message_count: 20, // Get multiple messages to search through
        max_wait_time: Some(Duration::seconds(10)), // Reasonable timeout
    };
    let verification_messages = peek_receiver
        .receive_messages(20, Some(verify_options))
        .await?;

    println!(
        "Found {} messages in queue during verification",
        verification_messages.len()
    );

    // Find our specific message by content
    let our_message = verification_messages.iter().find(|msg| {
        msg.body_as_string().unwrap_or_default()
            == "Test message for send-verify-receive-verify cycle"
    });

    assert!(
        our_message.is_some(),
        "Should find our message in the queue"
    );
    let found_message = our_message.unwrap();

    assert_eq!(
        found_message.body_as_string()?,
        "Test message for send-verify-receive-verify cycle",
        "Message body should match what we sent"
    );

    // Abandon all messages so they go back to the queue for the next step
    for msg in &verification_messages {
        peek_receiver.abandon_message(msg, None).await?;
    }
    println!("✓ Confirmed our message is in queue with correct content");

    // Step 5: Actually receive and complete our specific message
    println!("Receiving and completing our specific message");

    // Create a new receiver for actually processing the message
    let completion_receiver = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None,
            }),
        )
        .await?;

    let completion_options = ReceiveMessageOptions {
        max_message_count: 20, // Get multiple messages to search through
        max_wait_time: Some(Duration::seconds(10)),
    };
    let completion_messages = completion_receiver
        .receive_messages(20, Some(completion_options))
        .await?;

    // Find our specific message by content
    let our_message = completion_messages.iter().find(|msg| {
        msg.body_as_string().unwrap_or_default()
            == "Test message for send-verify-receive-verify cycle"
    });

    assert!(our_message.is_some(), "Should find our message to complete");
    let message_to_complete = our_message.unwrap();

    // Complete our specific message to remove it from the queue
    completion_receiver
        .complete_message(message_to_complete, None)
        .await?;

    // Abandon all other messages so they stay in the queue
    for msg in &completion_messages {
        if msg.body_as_string().unwrap_or_default()
            != "Test message for send-verify-receive-verify cycle"
        {
            completion_receiver.abandon_message(msg, None).await?;
        }
    }

    println!("✓ Our specific message completed successfully");

    // Step 6: Verify our specific message is no longer in the queue
    println!("Verifying our specific message is no longer in the queue");
    let final_verify_options = ReceiveMessageOptions {
        max_message_count: 20, // Get multiple messages to search through
        max_wait_time: Some(Duration::seconds(5)), // Reasonable timeout
    };
    let final_messages = completion_receiver
        .receive_messages(20, Some(final_verify_options))
        .await?;

    // Check that our specific message is not in the queue anymore
    let our_message_found = final_messages.iter().any(|msg| {
        msg.body_as_string().unwrap_or_default()
            == "Test message for send-verify-receive-verify cycle"
    });

    assert!(
        !our_message_found,
        "Our specific message should no longer be in the queue after completion"
    );

    // Abandon all other messages so they stay in the queue for other tests
    for msg in &final_messages {
        completion_receiver.abandon_message(msg, None).await?;
    }

    println!("✓ Confirmed our specific message is no longer in the queue");

    // Clean up
    peek_receiver.close().await?;
    completion_receiver.close().await?;
    sender.close().await?;
    client.close().await?;

    println!("Send-verify-receive-verify test completed successfully!");
    Ok(())
}

#[recorded::test(live)]
async fn test_send_single_message(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing single message send for queue: {}", queue_name);

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    let message_id = Uuid::new_v4().to_string();
    let mut message = Message::from("Single message test");
    message.set_message_id(&message_id);
    message.set_property("test_type", "single_send");

    sender.send_message(message, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Single message send test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_send_multiple_messages(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing multiple message send for queue: {}", queue_name);

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    let message_count = 5;
    let mut messages = Vec::new();

    for i in 0..message_count {
        let message_id = format!("multi-send-{}-{}", Uuid::new_v4(), i);
        let mut message = Message::from(format!("Multiple send test message {}", i));
        message.set_message_id(&message_id);
        message.set_property("test_type", "multiple_send");
        message.set_property("sequence", i.to_string());
        messages.push(message);
    }

    sender.send_messages(messages, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Multiple message send test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_send_message_with_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing message send with properties for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    let message_id = Uuid::new_v4().to_string();
    let correlation_id = Uuid::new_v4().to_string();

    let mut message = Message::from("Message with comprehensive properties");
    message.set_message_id(&message_id);
    message.set_correlation_id(&correlation_id);
    message.set_content_type("text/plain");
    message.set_subject("Test Subject");
    message.set_reply_to("reply-queue");

    // Add custom properties
    message.set_property("test_type", "properties_test");
    message.set_property("priority", "high");
    message.set_property("region", "us-west");
    message.set_property("version", "1.0");
    message.set_property("timestamp", OffsetDateTime::now_utc().to_string());

    sender.send_message(message, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Message properties send test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_sender_lifecycle(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing sender lifecycle operations");

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Create and close sender multiple times
    for i in 0..3 {
        let sender = client.create_sender(&queue_name, None).await?;

        let message_id = format!("sender-lifecycle-{}-{}", Uuid::new_v4(), i);
        let mut message = Message::from(format!("Sender lifecycle test {}", i));
        message.set_message_id(&message_id);
        message.set_property("test_type", "sender_lifecycle");

        sender.send_message(message, None).await?;
        sender.close().await?;

        println!("Sender lifecycle iteration {} completed", i);
    }

    // Clean up
    client.close().await?;

    println!("Sender lifecycle test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_send_message_id(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing message ID handling for queue: {}", queue_name);

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    let custom_message_id = "test-message-id-12345";
    let mut message = Message::from("Hello world with custom ID");
    message.set_message_id(custom_message_id);
    message.set_property("test_type", "message_id_test");

    sender.send_message(message, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Message ID test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_send_amqp_annotated_message(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing AMQP annotated message send for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Create message with comprehensive AMQP properties
    let message_id = "amqp-annotated-id-12345";
    let correlation_id = "correlation-98765";
    let mut message = Message::from("AMQP annotated message with all properties");

    // Set standard AMQP properties
    message.set_message_id(message_id);
    message.set_correlation_id(correlation_id);
    message.set_content_type("application/json");
    message.set_subject("AMQP Test Subject");
    message.set_reply_to("reply-queue");
    message.set_reply_to_session_id("reply-session-123");

    // Set custom application properties of different types
    message.set_property("test_type", "amqp_annotated");
    message.set_property("string-prop", "string-value");
    message.set_property("number-prop", "42");
    message.set_property("bool-prop", "true");
    message.set_property("float-prop", "3.14159");
    message.set_property("timestamp", OffsetDateTime::now_utc().to_string());

    sender.send_message(message, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("AMQP annotated message test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_send_amqp_value_body(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing AMQP value body message send for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Create a message with structured JSON data (simulating AMQP Value body)
    let structured_data = serde_json::json!({
        "id": 123,
        "name": "test-entity",
        "properties": {
            "key1": "value1",
            "key2": 42,
            "nested": {
                "level": 2,
                "data": [1, 2, 3, 4, 5]
            }
        },
        "timestamp": OffsetDateTime::now_utc().to_string()
    });

    let mut message = Message::from(structured_data.to_string());
    message.set_message_id("amqp-value-body-id");
    message.set_content_type("application/json");
    message.set_property("test_type", "amqp_value_body");

    sender.send_message(message, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("AMQP value body test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_send_amqp_sequence_body(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing AMQP sequence body message send for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Create a message with sequence-like data
    let sequence_data = vec![
        serde_json::json!({"item": "first", "value": 1}),
        serde_json::json!({"item": "second", "value": 2}),
        serde_json::json!({"item": "third", "value": 3}),
        serde_json::json!({"item": "fourth", "value": 4}),
    ];

    let mut message = Message::from(serde_json::to_string(&sequence_data)?);
    message.set_message_id("amqp-sequence-body-id");
    message.set_content_type("application/json");
    message.set_property("test_type", "amqp_sequence_body");
    message.set_property("sequence_length", sequence_data.len().to_string());

    sender.send_message(message, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("AMQP sequence body test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_send_multiple_byte_slices(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing multiple byte slices message send for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;
    let sender = client.create_sender(&queue_name, None).await?;

    // Simulate sending binary data in multiple chunks
    let chunk1 = b"First chunk of binary data containing important information";
    let chunk2 = b"Second chunk with more binary content and metadata";
    let chunk3 = b"Third and final chunk completing the binary message";

    let combined_data = [chunk1.as_slice(), chunk2.as_slice(), chunk3.as_slice()].concat();
    let mut message = Message::new(combined_data);
    message.set_message_id("multi-byte-slices-id");
    message.set_content_type("application/octet-stream");
    message.set_property("test_type", "multiple_byte_slices");
    message.set_property("chunk_count", "3");

    sender.send_message(message, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Multiple byte slices test completed successfully");
    Ok(())
}
