// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Tests for Service Bus scheduled message functionality.

mod common;

use azure_core::{
    time::{Duration, OffsetDateTime},
    Uuid,
};
use azure_core_test::{recorded, TestContext};
use azure_messaging_servicebus::{Message, ServiceBusClient};
use common::{get_queue_name, get_servicebus_namespace};
use std::error::Error;

#[recorded::test(live)]
async fn test_scheduled_message_send(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing scheduled message send for queue: {}", queue_name);

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    let sender = client.create_sender(&queue_name, None).await?;

    // Schedule message for 2 seconds in the future
    let scheduled_time = OffsetDateTime::now_utc() + Duration::seconds(2);
    let message_id = Uuid::new_v4().to_string();

    let mut message = Message::from("Scheduled message test");
    message.set_message_id(&message_id);
    message.set_property("test_type", "scheduled_send");

    let sequence_number = sender
        .schedule_message(message, scheduled_time, None)
        .await?;

    println!(
        "Scheduled message with sequence number: {}",
        sequence_number
    );

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Scheduled message send test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_schedule_message(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!("Testing schedule message for queue: {}", queue_name);

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    let sender = client.create_sender(&queue_name, None).await?;

    let message_id = "scheduled-test-id-12345";
    let mut message = Message::from("Scheduled message test");
    message.set_message_id(message_id);
    message.set_property("test_type", "schedule_test");

    let scheduled_time = OffsetDateTime::now_utc() + Duration::seconds(300); // 5 minutes
    let sequence_number = sender
        .schedule_message(message, scheduled_time, None)
        .await?;

    assert!(sequence_number > 0);
    println!(
        "Scheduled message with sequence number: {}",
        sequence_number
    );

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Schedule message test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_schedule_message_then_cancel(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing schedule then cancel message for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    let sender = client.create_sender(&queue_name, None).await?;

    let message_id = "cancel-test-id-12345";
    let mut message = Message::from("Cancel scheduled message test");
    message.set_message_id(message_id);
    message.set_property("test_type", "schedule_cancel_test");

    let scheduled_time = OffsetDateTime::now_utc() + Duration::seconds(3600); // 1 hour
    let sequence_number = sender
        .schedule_message(message, scheduled_time, None)
        .await?;

    assert!(sequence_number > 0);
    println!(
        "Scheduled message with sequence number: {}",
        sequence_number
    );

    // Cancel the scheduled message
    sender
        .cancel_scheduled_message(sequence_number, None)
        .await?;
    println!("Cancelled scheduled message: {}", sequence_number);

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Schedule then cancel test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn test_schedule_multiple_messages(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let namespace = get_servicebus_namespace()?;
    let queue_name = get_queue_name()?;

    println!(
        "Testing multiple scheduled messages for queue: {}",
        queue_name
    );

    let client = ServiceBusClient::builder()
        .open(&namespace, recording.credential())
        .await?;

    let sender = client.create_sender(&queue_name, None).await?;

    let base_time = OffsetDateTime::now_utc() + Duration::seconds(600); // 10 minutes
    let mut sequence_numbers = Vec::new();

    for i in 0..3 {
        let message_id = format!("scheduled-multi-{}", i);
        let mut message = Message::from(format!("Scheduled message {}", i));
        message.set_message_id(&message_id);
        message.set_property("test_type", "schedule_multiple");
        message.set_property("sequence", i.to_string());

        let scheduled_time = base_time + Duration::minutes(i); // 1 minute apart
        let sequence_number = sender
            .schedule_message(message, scheduled_time, None)
            .await?;

        assert!(sequence_number > 0);
        sequence_numbers.push(sequence_number);
        println!(
            "Scheduled message {} with sequence number: {}",
            i, sequence_number
        );
    }

    // Verify all sequence numbers are unique
    let mut sorted_numbers = sequence_numbers.clone();
    sorted_numbers.sort();
    sorted_numbers.dedup();
    assert_eq!(
        sorted_numbers.len(),
        3,
        "All sequence numbers should be unique"
    );

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("Multiple scheduled messages test completed successfully");
    Ok(())
}
