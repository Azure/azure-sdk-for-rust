// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Hello-world sample for Azure Queue Storage.
//!
//! This is the quickest path to sending and receiving messages:
//! 1. Create a [`QueueClient`] authenticated with Microsoft Entra ID.
//! 2. Create a queue.
//! 3. Send a message.
//! 4. Receive (and print) the message.
//! 5. Delete the queue.
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
//! cargo run --package azure_storage_queue --example queue_hello_world
//! ```

use azure_identity::DeveloperToolsCredential;
use azure_storage_queue::{models::QueueMessage, QueueClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = std::env::var("AZURE_QUEUE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_QUEUE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.queue.core.windows.net/", account);
    let queue_name = "hello-world-queue";

    let credential = DeveloperToolsCredential::new(None)?;
    let queue_client = QueueClient::new(&endpoint, queue_name, Some(credential), None)?;

    // Create the queue.
    queue_client.create(None).await?;
    println!("Created queue '{queue_name}'");

    // Send a message.
    let body = QueueMessage {
        message_text: Some("Hello, Azure Queue Storage!".to_string()),
    };
    queue_client.send_message(body.try_into()?, None).await?;
    println!("Sent message");

    // Receive messages (default: up to 1 message with 30-second visibility timeout).
    let response = queue_client.receive_messages(None).await?;
    let messages = response.into_model()?;
    for msg in messages.items.unwrap_or_default() {
        println!(
            "Received: {}",
            msg.message_text.as_deref().unwrap_or("<empty>")
        );
    }

    // Delete the queue.
    queue_client.delete(None).await?;
    println!("Deleted queue '{queue_name}'");

    Ok(())
}
