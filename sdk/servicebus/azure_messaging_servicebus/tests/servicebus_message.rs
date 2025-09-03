// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Tests for Service Bus message properties and handling.

mod common;

use azure_core_test::{recorded, TestContext};
use azure_messaging_servicebus::{CreateReceiverOptions, Message, ReceiveMode, ServiceBusClient};
use common::{get_queue_name, get_servicebus_namespace};
use std::error::Error;
use time::OffsetDateTime;
use uuid::Uuid;

#[recorded::test(live)]
async fn test_message_properties_preservation(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing message properties preservation for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Send message with comprehensive properties
    let sender = client.create_sender(&queue_name, None).await?;
    let message_id = Uuid::new_v4().to_string();
    let correlation_id = Uuid::new_v4().to_string();

    let mut message = Message::from("Properties preservation test");
    message.set_message_id(&message_id);
    message.set_correlation_id(&correlation_id);
    message.set_content_type("application/json");
    message.set_subject("Test Subject");
    message.set_reply_to("reply-queue");
    message.set_property("custom_prop_1", "value1");
    message.set_property("custom_prop_2", "value2");
    message.set_property("number_prop", "42");

    sender.send_message(message, None).await?;
    sender.close().await?;

    // Receive and verify properties
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

    assert!(!messages.is_empty(), "Should receive the message");

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
        Some("Test Subject".to_string()).as_ref()
    );
    assert_eq!(
        received_message.system_properties().reply_to.as_ref(),
        Some("reply-queue".to_string()).as_ref()
    );

    // Verify custom properties
    assert_eq!(
        received_message.property("custom_prop_1"),
        Some("value1".to_string()).as_ref()
    );
    assert_eq!(
        received_message.property("custom_prop_2"),
        Some("value2".to_string()).as_ref()
    );
    assert_eq!(
        received_message.property("number_prop"),
        Some("42".to_string()).as_ref()
    );

    // Complete message
    receiver.complete_message(received_message, None).await?;

    // Clean up
    receiver.close().await?;
    client.close().await?;

    println!("Message properties preservation test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_message_properties_comprehensive(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing comprehensive message properties for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    let sender = client.create_sender(&queue_name, None).await?;

    let mut message = Message::from("Comprehensive properties test message");

    // Set all possible standard AMQP properties
    message.set_message_id("comprehensive-prop-test-id");
    message.set_correlation_id("correlation-comprehensive-123");
    message.set_session_id("session-comprehensive-456");
    message.set_reply_to("comprehensive-reply-queue");
    message.set_reply_to_session_id("reply-session-comprehensive-789");
    message.set_content_type("application/json");
    message.set_subject("Comprehensive Properties Test Message");

    // Set comprehensive custom application properties
    message.set_property("test_type", "comprehensive_properties");
    message.set_property("string_prop", "comprehensive_string_value");
    message.set_property("int_prop", "12345");
    message.set_property("bool_prop", "true");
    message.set_property("float_prop", "99.99");
    message.set_property("timestamp_prop", OffsetDateTime::now_utc().to_string());
    message.set_property("version", "2.0.1");
    message.set_property("environment", "test");
    message.set_property("region", "us-east-1");
    message.set_property("category", "integration-test");

    sender.send_message(message, None).await?;

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Comprehensive properties test completed successfully");
    Ok(())
}
