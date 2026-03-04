// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This example demonstrates how to enable logging and OpenTelemetry distributed tracing
//! when making requests to Azure Blob Storage.
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
//! - Public API calls (e.g., `Storage.Blob.Container.exists`)
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
//! cargo run --package azure_storage_blob --example blob_storage_logging
//! ```
//!
//! To enable OpenTelemetry tracing (outputs spans to stdout):
//!
//! ```bash
//! cargo run --package azure_storage_blob --example blob_storage_logging -- --otel
//! ```

use azure_core::{
    http::{ClientOptions, InstrumentationOptions, RequestContent},
    tracing::TracerProvider,
};
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use azure_identity::AzureCliCredential;
use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
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

    let container_name = "test-container";
    let blob_name = "hello_world.txt";
    let content = b"Hello, World!";

    // Create OAuth credentials using Azure CLI
    println!("Authenticating with Azure CLI...");
    let credential = AzureCliCredential::new(None)?;

    // Create BlobContainerClient
    let endpoint = format!("https://{}.blob.core.windows.net", account);

    // Configure client options with optional OpenTelemetry tracing.
    // Azure Storage headers (x-ms-version, x-ms-request-id, etc.) are logged by default.
    let client_options = BlobContainerClientOptions {
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

    let container_client = BlobContainerClient::new(
        &endpoint,
        container_name,
        Some(credential),
        Some(client_options),
    )?;

    // Create BlobClient
    let blob_client = container_client.blob_client(blob_name);

    // Create container if it does not exist
    println!("Creating container '{}'...", container_name);
    if container_client.exists().await? {
        println!("Container already exists, continuing...");
    } else {
        container_client.create(None).await?;
        println!("Container created successfully");
    }

    // Upload the file
    println!("\nUploading blob '{}'...", blob_name);
    blob_client
        .upload(
            RequestContent::from(content.to_vec()),
            true, // overwrite if exists
            content.len() as u64,
            None,
        )
        .await?;
    println!("Blob uploaded successfully");

    // Download the file
    println!("\nDownloading blob '{}'...", blob_name);
    let response = blob_client.download(None).await?;
    let (_, _, body) = response.deconstruct();
    let downloaded_content = body.collect().await?;

    // Print the contents to stdout
    println!("\n=== File Contents ===");
    println!("{}", String::from_utf8_lossy(&downloaded_content));
    println!("=====================");

    // Shutdown OpenTelemetry tracer provider to flush remaining spans.
    if let Some(provider) = otel_provider {
        let _ = provider.shutdown();
        println!("\nOpenTelemetry spans flushed.");
    }

    println!("Pass --otel to see OpenTelemetry spans. Use RUST_LOG=trace for detailed HTTP logs.");

    Ok(())
}
