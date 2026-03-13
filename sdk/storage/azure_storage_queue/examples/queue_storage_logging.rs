// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This example demonstrates how to enable logging and OpenTelemetry distributed tracing
//! when making requests to Azure Queue Storage.
//!
//! # Regular Logging
//!
//! By default, Azure SDK clients sanitize headers to avoid logging sensitive information.
//! You can configure additional headers to be logged using `LoggingOptions`.
//! Logs are written to stderr.
//!
//! # OpenTelemetry Distributed Tracing
//!
//! Pass `--otel` to enable OpenTelemetry distributed tracing, which emits spans for:
//! - Public API calls (e.g., `Storage.Queues.Queue.sendMessage`)
//! - HTTP requests (method, URL, status code, etc.)
//!
//! When `--otel` is enabled, the default log level is set to `warn` to reduce noise.
//! OpenTelemetry spans are written to stdout. You can override with `RUST_LOG=trace`
//! to see both detailed logs (stderr) and spans (stdout).
//!
//! # Prerequisites
//!
//! - Set the `AZURE_STORAGE_ACCOUNT_NAME` environment variable to your storage account name
//! - Authenticate using Azure CLI: `az login`
//! - Set `RUST_LOG` to control log level (optional, defaults to `trace` in this example):
//!   - `error` - Only errors
//!   - `warn` - Warnings and errors
//!   - `info` - Info, warnings, and errors
//!   - `debug` - Debug and above
//!   - `trace` - All logs including detailed HTTP requests/responses (default)
//!
//! # Usage
//!
//! ```bash
//! az login
//! export AZURE_STORAGE_ACCOUNT_NAME="<your-storage-account>"
//! export RUST_LOG="<log-level>"
//! cargo run --package azure_storage_queue --example queue_storage_logging
//! ```
//!
//! To enable OpenTelemetry tracing (outputs spans to stdout):
//!
//! ```bash
//! cargo run --package azure_storage_queue --example queue_storage_logging -- --otel
//! ```

use azure_core::{
    http::{ClientOptions, InstrumentationOptions},
    tracing::TracerProvider,
};
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use azure_identity::AzureCliCredential;
use azure_storage_queue::{models::QueueMessage, QueueServiceClient, QueueServiceClientOptions};
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::{env, sync::Arc};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check for --otel flag to enable OpenTelemetry distributed tracing.
    let otel_enabled = env::args().any(|arg| arg == "--otel" || arg == "-otel");

    // Initialize tracing subscriber to see HTTP requests and responses.
    // When --otel is enabled, default to "warn" to reduce noise and let spans be visible.
    // When --otel is disabled, default to "trace" to show detailed HTTP logs.
    let default_level = if otel_enabled { "warn" } else { "trace" };
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level));
    println!("RUST_LOG filter: {}", env_filter);
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr) // Write logs to stderr so they don't interleave with otel stdout
        .init();

    let otel_provider = if otel_enabled {
        println!("OpenTelemetry tracing ENABLED (--otel flag)");
        Some(Arc::new(
            SdkTracerProvider::builder()
                .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
                .build(),
        ))
    } else {
        println!("OpenTelemetry tracing DISABLED (pass --otel to enable)");
        None
    };

    // Get Azure Storage Account name from environment variable
    let account = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_STORAGE_ACCOUNT_NAME environment variable");

    let queue_name = "test-logging-queue";
    let message_text = "Hello from azure_storage_queue logging example!";

    // Create OAuth credentials using Azure CLI
    println!("Authenticating with Azure CLI...");
    let credential = AzureCliCredential::new(None)?;

    let endpoint = format!("https://{}.queue.core.windows.net", account);

    // Configure client options with optional OpenTelemetry tracing.
    // Azure Queue Storage headers (x-ms-version, x-ms-approximate-messages-count, etc.)
    // are logged by default.
    let client_options = QueueServiceClientOptions {
        client_options: ClientOptions {
            instrumentation: InstrumentationOptions {
                tracer_provider: otel_provider.as_ref().map(|p| {
                    OpenTelemetryTracerProvider::new(p.clone()) as Arc<dyn TracerProvider>
                }),
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let service_client =
        QueueServiceClient::new(&endpoint, Some(credential), Some(client_options))?;
    let queue_client = service_client.queue_client(queue_name)?;

    // Create queue if it doesn't exist
    println!("\nCreating queue '{}'...", queue_name);
    if queue_client.exists().await? {
        println!("Queue already exists, continuing...");
    } else {
        queue_client.create(None).await?;
        println!("Queue created successfully");
    }

    // Send a message
    println!("\nSending message to queue '{}'...", queue_name);
    let message = QueueMessage {
        message_text: Some(message_text.to_string()),
    };
    let sent = queue_client.send_message(message.try_into()?, None).await?;
    let sent_message = sent.into_model()?;
    println!(
        "Message sent. ID: {}",
        sent_message.message_id.as_deref().unwrap_or("")
    );

    // Receive the message back. Receiving dequeues the message and returns a fresh
    // pop receipt that must be used for the subsequent delete.
    println!("\nReceiving messages from queue '{}'...", queue_name);
    let received = queue_client.receive_messages(None).await?;
    let message_list = received.into_model()?;
    let messages = message_list.items.unwrap_or_default();
    println!("Received {} message(s)", messages.len());
    for msg in &messages {
        println!(
            "  Message ID: {}, Text: {}",
            msg.message_id.as_deref().unwrap_or(""),
            msg.message_text.as_deref().unwrap_or("")
        );
    }

    // Delete using the pop receipt returned by receive_messages (not by send_message).
    let first = messages.first().ok_or("no messages received")?;
    let message_id = first.message_id.as_deref().unwrap_or("");
    let pop_receipt = first.pop_receipt.as_deref().unwrap_or("");
    println!("\nDeleting message '{}'...", message_id);
    queue_client
        .delete_message(message_id, pop_receipt, None)
        .await?;
    println!("Message deleted successfully");

    // Cleanup: delete the queue
    println!("\nDeleting queue '{}'...", queue_name);
    queue_client.delete(None).await?;
    println!("Queue deleted successfully");

    // Shutdown OpenTelemetry tracer provider to flush remaining spans.
    if let Some(provider) = otel_provider {
        let _ = provider.shutdown();
        println!("\nOpenTelemetry spans flushed.");
    }

    println!(
        "\nPass --otel to see OpenTelemetry spans. Use RUST_LOG=trace for detailed HTTP logs."
    );

    Ok(())
}
