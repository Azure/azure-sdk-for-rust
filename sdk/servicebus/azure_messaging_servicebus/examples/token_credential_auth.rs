// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! This example demonstrates how to authenticate with Service Bus using TokenCredential.
//!
//! This example shows various ways to create a Service Bus client using different
//! credential types from the azure_identity crate, including:
//! - DeveloperToolsCredential (recommended for production)
//! - ClientSecretCredential (for service principals)
//! - ManagedIdentityCredential (for Azure resources)
//! - AzureCliCredential (for development with Azure CLI)
//!
//! The TokenCredential authentication automatically handles:
//! - Token acquisition and caching
//! - Automatic token refresh before expiration
//! - Claims-Based Security (CBS) authorization for AMQP
//! - Path-specific authorization for queues and topics

use azure_identity::{
    AzureCliCredential, ClientSecretCredential, DeveloperToolsCredential, ManagedIdentityCredential,
};

use azure_messaging_servicebus::{Message, ServiceBusClient};
use std::env;
use time::OffsetDateTime;

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

    // Example 1: DeveloperToolsCredential (Recommended for production)
    // This credential type tries multiple authentication methods in order:
    // 1. Environment variables (client secret, certificate, username/password)
    // 2. Managed Identity (if running on Azure)
    // 3. Azure CLI credentials (if logged in)
    // 4. Azure PowerShell credentials (if logged in)
    // 5. Interactive browser authentication (as fallback)
    println!("\n=== Example 1: DeveloperToolsCredential (Builder Pattern) ===");
    let default_credential = DeveloperToolsCredential::new(None)?;
    let client = ServiceBusClient::builder()
        .open(&namespace, default_credential.clone())
        .await?;

    match send_test_message(&client, &queue_name, "DeveloperToolsCredential").await {
        Ok(_) => println!("✅ DeveloperToolsCredential authentication successful!"),
        Err(e) => println!("❌ DeveloperToolsCredential authentication failed: {}", e),
    }

    // Close the client
    client.close().await?;

    // Example 2: ClientSecretCredential (For service principals)
    // This is useful when you have a registered application in Azure AD
    // and want to authenticate using client ID and secret
    if let (Ok(tenant_id), Ok(client_id), Ok(client_secret)) = (
        env::var("AZURE_TENANT_ID"),
        env::var("AZURE_CLIENT_ID"),
        env::var("AZURE_CLIENT_SECRET"),
    ) {
        println!("\n=== Example 2: ClientSecretCredential (Builder Pattern) ===");
        let client_secret_credential =
            ClientSecretCredential::new(&tenant_id, client_id, client_secret.into(), None)?;
        let client = ServiceBusClient::builder()
            .open(&namespace, client_secret_credential.clone())
            .await?;

        match send_test_message(&client, &queue_name, "ClientSecretCredential").await {
            Ok(_) => println!("✅ ClientSecretCredential authentication successful!"),
            Err(e) => println!("❌ ClientSecretCredential authentication failed: {}", e),
        }

        client.close().await?;
    } else {
        println!("\n=== Example 2: ClientSecretCredential (Skipped) ===");
        println!("Set AZURE_TENANT_ID, AZURE_CLIENT_ID, and AZURE_CLIENT_SECRET to test ClientSecretCredential");
    }

    // Example 3: ManagedIdentityCredential (For Azure resources)
    // This is useful when running on Azure resources like VMs, App Service, or AKS
    // that have a managed identity assigned
    println!(
        "
=== Example 3: ManagedIdentityCredential (Builder Pattern) ==="
    );
    let managed_identity_credential = ManagedIdentityCredential::new(None)?;
    let client = ServiceBusClient::builder()
        .open(&namespace, managed_identity_credential.clone())
        .await?;

    match send_test_message(&client, &queue_name, "ManagedIdentityCredential").await {
        Ok(_) => println!("✅ ManagedIdentityCredential authentication successful!"),
        Err(e) => {
            println!("❌ ManagedIdentityCredential authentication failed: {}", e);
            println!(
                "   This is expected if not running on an Azure resource with managed identity"
            );
        }
    }

    client.close().await?;

    // Example 4: AzureCliCredential (For development)
    // This uses the credentials from Azure CLI (az login)
    // Useful for local development when you're logged in via Azure CLI
    println!("\n=== Example 4: AzureCliCredential (Builder Pattern) ===");
    let cli_credential = AzureCliCredential::new(None)?;
    let client = ServiceBusClient::builder()
        .open(&namespace, cli_credential.clone())
        .await?;
    match send_test_message(&client, &queue_name, "AzureCliCredential").await {
        Ok(_) => println!("✅ AzureCliCredential authentication successful!"),
        Err(e) => {
            println!("❌ AzureCliCredential authentication failed: {}", e);
            println!("   Make sure you're logged in with 'az login'");
        }
    }

    client.close().await?;

    // Example 5: Receiving messages with TokenCredential
    println!(
        "
=== Example 5: DeveloperToolsCredential (Builder Pattern) ==="
    );
    let credential = DeveloperToolsCredential::new(None)?;
    let client = ServiceBusClient::builder()
        .open(&namespace, credential.clone())
        .await?;
    match receive_test_messages(&client, &queue_name).await {
        Ok(count) => println!("✅ Received {} messages successfully!", count),
        Err(e) => println!("❌ Failed to receive messages: {}", e),
    }

    client.close().await?;

    println!("\n🎉 TokenCredential authentication examples completed!");
    println!(
        "\nNote: Different credential types may succeed or fail depending on your environment:"
    );
    println!("- DeveloperToolsCredential: Should work in most environments");
    println!("- ClientSecretCredential: Requires service principal setup");
    println!("- ManagedIdentityCredential: Only works on Azure resources");
    println!("- AzureCliCredential: Requires 'az login'");

    Ok(())
}

/// Helper function to send a test message using the provided client
async fn send_test_message(
    client: &ServiceBusClient,
    queue_name: &str,
    credential_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating sender for queue: {}", queue_name);
    let sender = client.create_sender(queue_name, None).await?;

    // Create a message with credential type information
    let mut message = Message::from(format!(
        "Hello from Service Bus Rust SDK using {}!",
        credential_type
    ));
    message.set_message_id(format!(
        "example-{}-{}",
        credential_type.to_lowercase(),
        OffsetDateTime::now_utc().unix_timestamp()
    ));
    message.set_property("credential_type", credential_type);
    message.set_property("example", "token_credential_auth");
    message.set_property("timestamp", OffsetDateTime::now_utc().to_string());

    println!("Sending message using {}...", credential_type);
    sender.send_message(message, None).await?;

    println!("Message sent successfully using {}!", credential_type);

    // Close the sender
    sender.close().await?;

    Ok(())
}

/// Helper function to receive test messages using the provided client
async fn receive_test_messages(
    client: &ServiceBusClient,
    queue_name: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    println!("Creating receiver for queue: {}", queue_name);
    let receiver = client.create_receiver(queue_name, None).await?;

    println!("Receiving messages...");
    let messages = receiver.receive_messages(5, None).await?;
    let count = messages.len() as u32;

    for (i, message) in messages.iter().enumerate() {
        println!("Message {}: {}", i + 1, message.body_as_string()?);

        // Get custom properties if they exist
        if let Some(credential_type) = message.property("credential_type") {
            println!("  - Credential type: {}", credential_type);
        }
        if let Some(timestamp) = message.property("timestamp") {
            println!("  - Timestamp: {}", timestamp);
        }
    }

    // Complete all messages (remove them from the queue)
    for message in messages {
        receiver.complete_message(&message, None).await?;
    }

    println!("Completed {} messages", count);

    // Close the receiver
    receiver.close().await?;

    Ok(count)
}
