// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Simple example showing TokenCredential authentication with Azure Service Bus.
//!
//! This example demonstrates the most basic usage of TokenCredential authentication
//! using DeveloperToolsCredential to send a message to a Service Bus queue.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_servicebus::{Message, ServiceBusClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Get the Service Bus namespace from environment variable
    let namespace = env::var("SERVICEBUS_NAMESPACE")
        .expect("SERVICEBUS_NAMESPACE environment variable must be set (e.g., 'mybus.servicebus.windows.net')");

    // Get the queue name from environment variable
    let queue_name = env::var("SERVICEBUS_QUEUE_NAME")
        .expect("SERVICEBUS_QUEUE_NAME environment variable must be set");

    println!("Service Bus namespace: {}", namespace);
    println!("Queue name: {}", queue_name);

    // Create a DeveloperToolsCredential (tries multiple auth methods automatically)
    let credential = DeveloperToolsCredential::new(None)?;

    // Create the Service Bus client with TokenCredential authentication
    let client = ServiceBusClient::builder()
        .open(&namespace, credential.clone())
        .await?;

    println!("✅ Successfully created Service Bus client with TokenCredential!");

    // Create a sender for the queue
    let sender = client.create_sender(&queue_name, None).await?;

    println!("✅ Successfully created sender for queue: {}", queue_name);

    // Create and send a message
    let mut message = Message::from("Hello from Azure Service Bus with TokenCredential!");
    message.set_message_id("simple-auth-example");
    message.set_property("authentication", "TokenCredential");
    message.set_property("example", "simple_auth");

    println!("Sending message...");
    sender.send_message(message, None).await?;

    println!("✅ Message sent successfully!");

    // Clean up
    sender.close().await?;
    client.close().await?;

    println!("🎉 Example completed successfully!");
    println!("\nNote: This example uses DeveloperToolsCredential which automatically tries:");
    println!("  1. Environment variables (AZURE_CLIENT_ID, AZURE_CLIENT_SECRET, etc.)");
    println!("  2. Managed Identity (if running on Azure)");
    println!("  3. Azure CLI authentication (if logged in with 'az login')");
    println!("  4. Azure PowerShell authentication");
    println!("  5. Interactive browser authentication (as fallback)");

    Ok(())
}
