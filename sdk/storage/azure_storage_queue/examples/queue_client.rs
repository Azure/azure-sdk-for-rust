// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Queue client example for Azure Queue Storage.
//!
//! This sample shows a small end-to-end queue workflow:
//! 1. Create a queue.
//! 2. Set and read queue metadata.
//! 3. Send a visible message and a deferred message with time-to-live/visibility options.
//! 4. Peek visible messages.
//! 5. Receive and delete visible messages.
//! 6. Clear any remaining messages and delete the queue.
//!
//! # Prerequisites
//!
//! - Set `AZURE_QUEUE_STORAGE_ACCOUNT_NAME` to your storage account name.
//! - Sign in with `az login` (or any other credential flow supported by [`DeveloperToolsCredential`]).
//!
//! # Usage
//!
//! ```bash
//! az login
//! export AZURE_QUEUE_STORAGE_ACCOUNT_NAME="<your-storage-account>"
//! cargo run --package azure_storage_queue --example queue_client
//! ```

use std::{collections::HashMap, env};

use azure_identity::DeveloperToolsCredential;
use azure_storage_queue::{
    models::{
        QueueClientGetPropertiesResultHeaders, QueueClientPeekMessagesOptions,
        QueueClientReceiveMessagesOptions, QueueClientSendMessageOptions, QueueMessage,
    },
    QueueClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = env::var("AZURE_QUEUE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_QUEUE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.queue.core.windows.net/", account);
    let queue_name = random_queue_name();

    let credential = DeveloperToolsCredential::new(None)?;
    let queue_client = QueueClient::new(&endpoint, &queue_name, Some(credential), None)?;

    println!("Creating queue '{queue_name}'...");
    queue_client.create(None).await?;

    let metadata = HashMap::from([
        ("sample".to_string(), "queue-client".to_string()),
        ("language".to_string(), "rust".to_string()),
    ]);
    queue_client.set_metadata(&metadata, None).await?;

    let properties = queue_client.get_properties(None).await?;
    let stored_metadata = properties.metadata()?;
    println!("Queue metadata:");
    for (key, value) in stored_metadata {
        println!("  {key}: {value}");
    }

    let visible_message = QueueMessage {
        message_text: Some("Hello from the queue client example!".to_string()),
    };
    queue_client
        .send_message(visible_message.try_into()?, None)
        .await?;
    println!("Sent one visible message");

    let deferred_message = QueueMessage {
        message_text: Some("This message becomes visible later".to_string()),
    };
    let send_options = QueueClientSendMessageOptions {
        visibility_timeout: Some(10),
        message_time_to_live: Some(60),
        ..Default::default()
    };
    queue_client
        .send_message(deferred_message.try_into()?, Some(send_options))
        .await?;
    println!("Sent one deferred message with visibility timeout and TTL");

    let peek_options = QueueClientPeekMessagesOptions {
        number_of_messages: Some(5),
        ..Default::default()
    };
    let peeked = queue_client.peek_messages(Some(peek_options)).await?;
    let peeked_messages = peeked.into_model()?.items.unwrap_or_default();
    println!("Peeked {} visible message(s):", peeked_messages.len());
    for message in &peeked_messages {
        println!("  {}", message.message_text.as_deref().unwrap_or("<empty>"));
    }

    let receive_options = QueueClientReceiveMessagesOptions {
        number_of_messages: Some(5),
        visibility_timeout: Some(30),
        ..Default::default()
    };
    let received = queue_client.receive_messages(Some(receive_options)).await?;
    let received_messages = received.into_model()?.items.unwrap_or_default();
    println!("Received {} visible message(s)", received_messages.len());

    for message in &received_messages {
        println!(
            "  Message {}: {}",
            message.message_id.as_deref().unwrap_or("<no-id>"),
            message.message_text.as_deref().unwrap_or("<empty>")
        );

        if let (Some(message_id), Some(pop_receipt)) = (
            message.message_id.as_deref(),
            message.pop_receipt.as_deref(),
        ) {
            queue_client
                .delete_message(message_id, pop_receipt, None)
                .await?;
            println!("  Deleted message {message_id}");
        }
    }

    queue_client.clear(None).await?;
    println!("Cleared any remaining messages");

    queue_client.delete(None).await?;
    println!("Deleted queue '{queue_name}'");

    Ok(())
}

fn random_queue_name() -> String {
    use rand::RngExt;

    let mut rng = rand::rng();
    let random_suffix: u32 = rng.random_range(1000..9999);
    format!("sdk-test-queue-{random_suffix}")
}
