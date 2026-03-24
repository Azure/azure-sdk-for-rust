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
async fn test_receive_single_message(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing single message receive for queue: {}", queue_name);

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Send a test message first
    let sender = client.create_sender(&queue_name, None).await?;
    let message_id = Uuid::new_v4().to_string();
    let mut message = Message::from("Single receive test message");
    message.set_message_id(&message_id);
    message.set_property("test_type", "single_receive");

    sender.send_message(message, None).await?;
    sender.close().await?;

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
    let messages = receiver.receive_messages(1, None).await?;

    assert!(!messages.is_empty(), "Should receive the message we sent");

    let received_message = &messages[0];
    assert_eq!(received_message.message_id(), Some(message_id).as_ref());
    assert_eq!(
        received_message.body_as_string()?,
        "Single receive test message"
    );

    // Complete the message
    receiver.complete_message(received_message, None).await?;

    // Clean up
    receiver.close().await?;
    client.close().await?;

    println!("Single message receive test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_receive_multiple_messages(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing multiple message receive for queue: {}", queue_name);

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Send multiple test messages
    let sender = client.create_sender(&queue_name, None).await?;
    let message_count = 5;
    let mut sent_message_ids = Vec::new();

    for i in 0..message_count {
        let message_id = format!("multi-receive-{}-{}", Uuid::new_v4(), i);
        sent_message_ids.push(message_id.clone());

        let mut message = Message::from(format!("Multiple receive test message {}", i));
        message.set_message_id(&message_id);
        message.set_property("test_type", "multiple_receive");
        message.set_property("sequence", i.to_string());

        sender.send_message(message, None).await?;
    }

    sender.close().await?;

    // Receive the messages
    let receiver = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None,
            }),
        )
        .await?;
    let messages = receiver
        .receive_messages(message_count as usize, None)
        .await?;

    assert_eq!(
        messages.len(),
        message_count as usize,
        "Should receive all sent messages"
    );

    // Verify and complete all messages
    for received_message in &messages {
        let received_id = received_message.message_id().unwrap();
        assert!(
            sent_message_ids.contains(received_id),
            "Received message ID should be in sent list"
        );

        receiver.complete_message(received_message, None).await?;
    }

    // Clean up
    receiver.close().await?;
    client.close().await?;

    println!("Multiple message receive test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_receive_and_delete_mode(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing ReceiveAndDelete mode for queue: {}", queue_name);

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Send a test message
    let sender = client.create_sender(&queue_name, None).await?;
    let message_id = Uuid::new_v4().to_string();
    let mut message = Message::from("ReceiveAndDelete test message");
    message.set_message_id(&message_id);
    message.set_property("test_type", "receive_and_delete");

    sender.send_message(message, None).await?;
    sender.close().await?;

    // Receive with ReceiveAndDelete mode
    let receiver = client
        .create_receiver(
            &queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::ReceiveAndDelete,
                sub_queue: None,
            }),
        )
        .await?;
    let messages = receiver.receive_messages(1, None).await?;

    assert!(!messages.is_empty(), "Should receive the message");
    let received_message = &messages[0];
    assert_eq!(received_message.message_id(), Some(message_id).as_ref());

    // In ReceiveAndDelete mode, messages are automatically deleted
    // No need to call complete_message()

    // Clean up
    receiver.close().await?;
    client.close().await?;

    println!("ReceiveAndDelete mode test completed successfully");
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
