// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This example demonstrates how to work with sub-queues in Service Bus using the SubQueue enum.
//! It shows how to:
//! 1. Send a message to a queue
//! 2. Receive and dead letter the message
//! 3. Use SubQueue enum to receive from the dead letter queue
//! 4. Demonstrate both queue and subscription sub-queues

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

    // Demonstrate queue sub-queues
    demonstrate_queue_subqueues(&client, &queue_name).await?;

    // Uncomment the following lines if you have topic/subscription configured
    // let topic_name = env::var("SERVICEBUS_TOPIC_NAME").unwrap_or_default();
    // let subscription_name = env::var("SERVICEBUS_SUBSCRIPTION_NAME").unwrap_or_default();
    // if !topic_name.is_empty() && !subscription_name.is_empty() {
    //     demonstrate_subscription_subqueues(&client, &topic_name, &subscription_name).await?;
    // }

    client.close().await?;
    Ok(())
}

async fn demonstrate_queue_subqueues(
    client: &ServiceBusClient,
    queue_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Demonstrating Queue Sub-Queues ===");

    // Step 1: Send a message to the queue
    println!("Creating sender for queue: {}", queue_name);
    let sender = client.create_sender(queue_name, None).await?;

    let mut message = Message::from("This message will demonstrate SubQueue enum usage");
    message.set_message_id("subqueue-example-1");
    message.set_property("example", "subqueue_demo");

    println!("Sending message to queue: {}", queue_name);
    sender.send_message(message, None).await?;
    println!("Message sent successfully!");

    // Step 2: Receive the message from the main queue
    println!("Creating receiver for main queue: {}", queue_name);
    let receiver = client
        .create_receiver(
            queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None, // Main queue (no sub-queue)
            }),
        )
        .await?;

    println!("Receiving message from main queue...");
    if let Some(received_message) = receiver.receive_message(None).await? {
        println!("Received message: {}", received_message.body_as_string()?);
        println!("Message ID: {:?}", received_message.message_id());

        // Dead letter the message with a reason
        println!("Dead lettering the message...");
        let dead_letter_options = DeadLetterMessageOptions {
            reason: Some("SubQueueDemo".to_string()),
            error_description: Some(
                "Message dead lettered to demonstrate SubQueue enum".to_string(),
            ),
            properties_to_modify: None,
        };
        receiver
            .dead_letter_message(&received_message, Some(dead_letter_options))
            .await?;
        println!("Message dead lettered successfully!");
    } else {
        println!("No message received from main queue");
    }

    // Step 3: Receive the message from the dead letter queue using SubQueue enum
    println!("Creating receiver for dead letter queue using SubQueue::DeadLetter...");
    let dead_letter_receiver = client
        .create_receiver(
            queue_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: Some(SubQueue::DeadLetter), // Using SubQueue enum!
            }),
        )
        .await?;

    println!("Checking dead letter queue for messages...");
    // Wait a bit for the message to appear in the dead letter queue
    sleep(Duration::from_secs(2)).await;

    match dead_letter_receiver.receive_message(None).await? {
        Some(dead_letter_message) => {
            println!("Successfully received dead lettered message using SubQueue::DeadLetter!");
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

    Ok(())
}

#[allow(dead_code)]
async fn demonstrate_subscription_subqueues(
    client: &ServiceBusClient,
    topic_name: &str,
    subscription_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Demonstrating Subscription Sub-Queues ===");

    // Step 1: Send a message to the topic
    println!("Creating sender for topic: {}", topic_name);
    let sender = client.create_sender(topic_name, None).await?;

    let mut message = Message::from("This message demonstrates subscription sub-queues");
    message.set_message_id("subqueue-subscription-example-1");
    message.set_property("example", "subscription_subqueue_demo");

    println!("Sending message to topic: {}", topic_name);
    sender.send_message(message, None).await?;
    println!("Message sent successfully!");

    // Step 2: Receive the message from the subscription
    println!(
        "Creating receiver for subscription: {}/{}",
        topic_name, subscription_name
    );
    let receiver = client
        .create_receiver_for_subscription(
            topic_name,
            subscription_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: None, // Main subscription (no sub-queue)
            }),
        )
        .await?;

    println!("Receiving message from subscription...");
    if let Some(received_message) = receiver.receive_message(None).await? {
        println!("Received message: {}", received_message.body_as_string()?);
        println!("Message ID: {:?}", received_message.message_id());

        // Dead letter the message
        println!("Dead lettering the subscription message...");
        let dead_letter_options = DeadLetterMessageOptions {
            reason: Some("SubscriptionSubQueueDemo".to_string()),
            error_description: Some(
                "Subscription message dead lettered to demonstrate SubQueue enum".to_string(),
            ),
            properties_to_modify: None,
        };
        receiver
            .dead_letter_message(&received_message, Some(dead_letter_options))
            .await?;
        println!("Subscription message dead lettered successfully!");
    } else {
        println!("No message received from subscription");
    }

    // Step 3: Receive from the subscription's dead letter queue using SubQueue enum
    println!("Creating receiver for subscription dead letter queue using SubQueue::DeadLetter...");
    let subscription_dead_letter_receiver = client
        .create_receiver_for_subscription(
            topic_name,
            subscription_name,
            Some(CreateReceiverOptions {
                receive_mode: ReceiveMode::PeekLock,
                sub_queue: Some(SubQueue::DeadLetter), // Using SubQueue enum for subscription!
            }),
        )
        .await?;

    println!("Checking subscription dead letter queue for messages...");
    sleep(Duration::from_secs(2)).await;

    match subscription_dead_letter_receiver
        .receive_message(None)
        .await?
    {
        Some(dead_letter_message) => {
            println!(
                "Successfully received subscription dead lettered message using SubQueue::DeadLetter!"
            );
            println!("Message body: {}", dead_letter_message.body_as_string()?);
            println!("Message ID: {:?}", dead_letter_message.message_id());
            println!(
                "Dead letter reason: {:?}",
                dead_letter_message.system_properties().dead_letter_reason
            );

            // Complete the message
            println!("Completing subscription dead lettered message...");
            subscription_dead_letter_receiver
                .complete_message(&dead_letter_message, None)
                .await?;
            println!("Subscription dead lettered message completed successfully!");
        }
        None => {
            println!("No subscription dead lettered messages found");
        }
    }

    // Clean up
    sender.close().await?;
    receiver.close().await?;
    subscription_dead_letter_receiver.close().await?;

    Ok(())
}
