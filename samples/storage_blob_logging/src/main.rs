// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This sample demonstrates how to enable logging and OpenTelemetry distributed tracing
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
//! - Set `RUST_LOG` to control log level (optional, defaults to `trace` in this sample):
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
//! cargo run --manifest-path samples/storage_blob_logging/Cargo.toml -- <ACCOUNT_NAME>
//! ```
//!
//! The `<ACCOUNT_NAME>` argument can also be provided via the `AZURE_STORAGE_ACCOUNT_NAME`
//! environment variable.
//!
//! To enable OpenTelemetry tracing (outputs spans to stdout):
//!
//! ```bash
//! cargo run --manifest-path samples/storage_blob_logging/Cargo.toml -- <ACCOUNT_NAME> --otel
//! ```

use azure_core::{
    http::{ClientOptions, InstrumentationOptions, RequestContent, Url},
    tracing::TracerProvider,
};
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use azure_identity::AzureCliCredential;
use azure_storage_blob::{BlobServiceClient, BlobServiceClientOptions};
use clap::Parser;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let otel_enabled = args.otel;

    let default_level = if otel_enabled { "warn" } else { "trace" };
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level));
    println!("RUST_LOG filter: {}", env_filter);
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
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

    let account = &args.account_name;
    let container_name = "test-container";
    let blob_name = "hello_world.txt";
    let content = b"Hello, World!";

    println!("Authenticating with Azure CLI...");
    let credential = AzureCliCredential::new(None)?;

    let endpoint = format!("https://{}.blob.core.windows.net", account);

    let client_options = BlobServiceClientOptions {
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

    let service_client = BlobServiceClient::new(
        Url::parse(&endpoint)?,
        Some(credential),
        Some(client_options),
    )?;
    let container_client = service_client.blob_container_client(container_name);
    let blob_client = container_client.blob_client(blob_name);

    println!("Creating container '{}'...", container_name);
    if container_client.exists().await? {
        println!("Container already exists, continuing...");
    } else {
        container_client.create(None).await?;
        println!("Container created successfully");
    }

    println!("\nUploading blob '{}'...", blob_name);
    blob_client
        .upload(RequestContent::from(content.to_vec()), None)
        .await?;
    println!("Blob uploaded successfully");

    println!("\nDownloading blob '{}'...", blob_name);
    let response = blob_client.download(None).await?;
    let downloaded_content = response.body.collect().await?;

    println!("\n=== File Contents ===");
    println!("{}", String::from_utf8_lossy(&downloaded_content));
    println!("=====================");

    if let Some(provider) = otel_provider {
        let _ = provider.shutdown();
        println!("\nOpenTelemetry spans flushed.");
    }

    println!("Pass --otel to see OpenTelemetry spans. Use RUST_LOG=trace for detailed HTTP logs.");

    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    /// Azure Storage account name.
    ///
    /// Can also be set via the `AZURE_STORAGE_ACCOUNT_NAME` environment variable.
    #[arg(env = "AZURE_STORAGE_ACCOUNT_NAME")]
    account_name: String,

    /// Enable OpenTelemetry distributed tracing.
    #[arg(long)]
    otel: bool,
}
