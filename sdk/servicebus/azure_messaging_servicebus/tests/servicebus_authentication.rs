// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Tests for Service Bus authentication methods.

mod common;
use azure_core::Uuid;
use azure_core_test::{recorded, TestContext};
use azure_messaging_servicebus::{CreateReceiverOptions, Message, ReceiveMode, ServiceBusClient};
use common::{get_queue_name, get_servicebus_namespace};
use std::{env, error::Error};

#[recorded::test(live)]
async fn test_token_credential_message_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing message properties with TokenCredential");

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Create message with comprehensive properties
    let message_id = Uuid::new_v4().to_string();
    let correlation_id = Uuid::new_v4().to_string();

    let mut message = Message::from("TokenCredential properties test");
    message.set_message_id(&message_id);
    message.set_correlation_id(&correlation_id);
    message.set_content_type("application/json");
    message.set_subject("TokenCredential Test");
    message.set_reply_to("reply-queue");

    // Add custom properties
    message.set_property("credential_type", "DeveloperToolsCredential");
    message.set_property("test_name", "test_token_credential_message_properties");
    message.set_property("environment", "live_test");
    message.set_property("number_value", "123");
    message.set_property("boolean_value", "true");

    // Send message
    let sender = client.create_sender(&queue_name, None).await?;
    sender.send_message(message, None).await?;

    // Receive and validate message
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

    // Validate standard properties
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
        Some("TokenCredential Test".to_string()).as_ref()
    );
    assert_eq!(
        received_message.system_properties().reply_to.as_ref(),
        Some("reply-queue".to_string()).as_ref()
    );

    // Validate custom properties
    assert_eq!(
        received_message.property("credential_type"),
        Some("DeveloperToolsCredential".to_string()).as_ref()
    );
    assert_eq!(
        received_message.property("test_name"),
        Some("test_token_credential_message_properties".to_string()).as_ref()
    );
    assert_eq!(
        received_message.property("environment"),
        Some("live_test".to_string()).as_ref()
    );
    assert_eq!(
        received_message.property("number_value"),
        Some("123".to_string()).as_ref()
    );
    assert_eq!(
        received_message.property("boolean_value"),
        Some("true".to_string()).as_ref()
    );

    println!("All message properties validated successfully");

    // Complete message
    receiver.complete_message(received_message, None).await?;

    // Clean up
    receiver.close().await?;
    sender.close().await?;
    client.close().await?;

    println!("TokenCredential message properties test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_client_with_token_credential(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing client creation with DeveloperToolsCredential for namespace: {}",
        namespace
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    // Test basic client operations
    let sender = client.create_sender(&queue_name, None).await?;
    let receiver = client.create_receiver(&queue_name, None).await?;

    // Clean up
    receiver.close().await?;
    sender.close().await?;
    client.close().await?;

    println!("Client TokenCredential test completed successfully");
    Ok(())
}
