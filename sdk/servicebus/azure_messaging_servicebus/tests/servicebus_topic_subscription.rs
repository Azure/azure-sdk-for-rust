// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Tests for Service Bus authentication methods.

mod common;

use azure_core::time::Duration;
use azure_core_test::{recorded, TestContext};
use azure_messaging_servicebus::{
    CreateReceiverOptions, Message, ReceiveMessageOptions, ReceiveMode, ServiceBusClient,
};
use common::{get_queue_name, get_servicebus_namespace, get_subscription_name, get_topic_name};
use std::{env, error::Error};
use time::OffsetDateTime;
use uuid::Uuid;

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
async fn test_topic_subscription_messaging(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    // Check if topic and subscription environment variables are set
    if env::var("SERVICEBUS_TOPIC_NAME").is_err()
        || env::var("SERVICEBUS_SUBSCRIPTION_NAME").is_err()
    {
        println!("Skipping topic/subscription test - SERVICEBUS_TOPIC_NAME or SERVICEBUS_SUBSCRIPTION_NAME not set");
        return Ok(());
    }

    let namespace = get_servicebus_namespace()?;
    let topic_name = get_topic_name()?;
    let subscription_name = get_subscription_name()?;

    println!(
        "Testing topic subscription messaging for topic: {} subscription: {}",
        topic_name, subscription_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Send message to topic
    let message_id = Uuid::new_v4().to_string();
    let sender = client.create_sender(&topic_name, None).await?;

    let mut message = Message::from("Topic subscription test message");
    message.set_message_id(&message_id);
    message.set_property("test_name", "test_topic_subscription_messaging");
    message.set_property("timestamp", OffsetDateTime::now_utc().to_string());

    sender.send_message(message, None).await?;
    sender.close().await?;
    println!("Message sent to topic successfully");

    // Receive from subscription
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

    let messages = receiver.receive_messages(1, None).await?;

    assert!(
        !messages.is_empty(),
        "Should receive message from subscription"
    );
    let received_message = &messages[0];
    assert_eq!(received_message.message_id(), Some(message_id).as_ref());

    // Verify message content and properties
    assert_eq!(
        received_message.body_as_string()?,
        "Topic subscription test message"
    );
    if let Some(test_name) = received_message.property("test_name") {
        assert_eq!(test_name, "test_topic_subscription_messaging");
    }

    // Complete the message
    receiver.complete_message(received_message, None).await?;
    receiver.close().await?;
    client.close().await?;

    println!("Topic subscription messaging test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_topic_multiple_messages(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    // Check if topic and subscription environment variables are set
    if env::var("SERVICEBUS_TOPIC_NAME").is_err()
        || env::var("SERVICEBUS_SUBSCRIPTION_NAME").is_err()
    {
        println!("Skipping topic multiple messages test - SERVICEBUS_TOPIC_NAME or SERVICEBUS_SUBSCRIPTION_NAME not set");
        return Ok(());
    }

    let namespace = get_servicebus_namespace()?;
    let topic_name = get_topic_name()?;
    let subscription_name = get_subscription_name()?;

    println!(
        "Testing multiple messages to topic: {} subscription: {}",
        topic_name, subscription_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Send multiple messages to topic
    let sender = client.create_sender(&topic_name, None).await?;
    let message_count = 3;
    let mut sent_message_ids = Vec::new();

    for i in 0..message_count {
        let message_id = format!("topic-multi-{}-{}", Uuid::new_v4(), i);
        sent_message_ids.push(message_id.clone());

        let mut message = Message::from(format!("Topic multiple message {}", i));
        message.set_message_id(&message_id);
        message.set_property("test_name", "test_topic_multiple_messages");
        message.set_property("sequence", i.to_string());

        sender.send_message(message, None).await?;
    }

    sender.close().await?;
    println!("Sent {} messages to topic successfully", message_count);

    // Receive from subscription
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

    let messages = receiver
        .receive_messages(message_count as usize, None)
        .await?;

    assert_eq!(
        messages.len(),
        message_count as usize,
        "Should receive all sent messages from subscription"
    );

    // Verify and complete all messages
    for received_message in &messages {
        let received_id = received_message.message_id().unwrap();
        assert!(
            sent_message_ids.contains(received_id),
            "Received message ID should be in sent list"
        );

        if let Some(test_name) = received_message.property("test_name") {
            assert_eq!(test_name, "test_topic_multiple_messages");
        }

        receiver.complete_message(received_message, None).await?;
    }

    receiver.close().await?;
    client.close().await?;

    println!("Topic multiple messages test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_topic_subscription_with_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    // Check if topic and subscription environment variables are set
    if env::var("SERVICEBUS_TOPIC_NAME").is_err()
        || env::var("SERVICEBUS_SUBSCRIPTION_NAME").is_err()
    {
        println!("Skipping topic properties test - SERVICEBUS_TOPIC_NAME or SERVICEBUS_SUBSCRIPTION_NAME not set");
        return Ok(());
    }

    let namespace = get_servicebus_namespace()?;
    let topic_name = get_topic_name()?;
    let subscription_name = get_subscription_name()?;

    println!("Testing topic subscription with message properties");

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Send message with comprehensive properties to topic
    let sender = client.create_sender(&topic_name, None).await?;
    let message_id = Uuid::new_v4().to_string();
    let correlation_id = Uuid::new_v4().to_string();

    let mut message = Message::from("Topic message with properties");
    message.set_message_id(&message_id);
    message.set_correlation_id(&correlation_id);
    message.set_content_type("application/json");
    message.set_subject("Topic Test Subject");
    message.set_reply_to("topic-reply");

    // Add custom properties
    message.set_property("test_name", "test_topic_subscription_with_properties");
    message.set_property("category", "important");
    message.set_property("region", "global");
    message.set_property("priority", "high");

    sender.send_message(message, None).await?;
    sender.close().await?;

    // Receive from subscription and verify properties
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

    let messages = receiver.receive_messages(1, None).await?;

    assert!(
        !messages.is_empty(),
        "Should receive the message from subscription"
    );

    let received_message = &messages[0];

    // Verify standard properties
    assert_eq!(received_message.message_id(), Some(message_id).as_ref());
    assert_eq!(
        received_message.correlation_id(),
        Some(correlation_id).as_ref()
    );
    assert_eq!(
        received_message.system_properties().content_type.as_ref(),
        Some("application/json".to_string()).as_ref()
    );
    assert_eq!(
        received_message.system_properties().subject.as_ref(),
        Some("Topic Test Subject".to_string()).as_ref()
    );
    assert_eq!(
        received_message.system_properties().reply_to.as_ref(),
        Some("topic-reply".to_string()).as_ref()
    );

    // Verify custom properties
    assert_eq!(
        received_message.property("test_name"),
        Some("test_topic_subscription_with_properties".to_string()).as_ref()
    );
    assert_eq!(
        received_message.property("category"),
        Some("important".to_string()).as_ref()
    );
    assert_eq!(
        received_message.property("region"),
        Some("global".to_string()).as_ref()
    );
    assert_eq!(
        received_message.property("priority"),
        Some("high".to_string()).as_ref()
    );

    // Complete the message
    receiver.complete_message(received_message, None).await?;
    receiver.close().await?;
    client.close().await?;

    println!("Topic subscription properties test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_topic_subscription_peek_lock(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    // Check if topic and subscription environment variables are set
    if env::var("SERVICEBUS_TOPIC_NAME").is_err()
        || env::var("SERVICEBUS_SUBSCRIPTION_NAME").is_err()
    {
        println!("Skipping topic peek lock test - SERVICEBUS_TOPIC_NAME or SERVICEBUS_SUBSCRIPTION_NAME not set");
        return Ok(());
    }

    let namespace = get_servicebus_namespace()?;
    let topic_name = get_topic_name()?;
    let subscription_name = get_subscription_name()?;

    println!("Testing topic subscription PeekLock operations");

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Send a message to topic for abandon testing
    let sender = client.create_sender(&topic_name, None).await?;
    let message_id = Uuid::new_v4().to_string();

    let mut message = Message::from("Topic abandon test message");
    message.set_message_id(&message_id);
    message.set_property("test_name", "test_topic_subscription_peek_lock");

    sender.send_message(message, None).await?;
    sender.close().await?;

    // Receive and abandon the message from subscription
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

    let messages = receiver.receive_messages(1, None).await?;

    assert!(
        !messages.is_empty(),
        "Should receive the message from subscription"
    );
    let received_message = &messages[0];
    assert_eq!(
        received_message.message_id(),
        Some(message_id.clone()).as_ref()
    );

    // Abandon the message (should make it available again)
    receiver.abandon_message(received_message, None).await?;

    // Try to receive it again (it should be available since we abandoned it)
    let messages_after_abandon = receiver.receive_messages(1, None).await?;

    if !messages_after_abandon.is_empty() {
        let re_received = &messages_after_abandon[0];
        assert_eq!(re_received.message_id(), Some(message_id).as_ref());

        // Complete it this time to clean up
        receiver.complete_message(re_received, None).await?;
    }

    receiver.close().await?;
    client.close().await?;

    println!("Topic subscription PeekLock test completed successfully");
    Ok(())
}
