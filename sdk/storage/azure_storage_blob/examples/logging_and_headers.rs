// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This example demonstrates how to enable distributed tracing and configure
//! which headers are logged when making requests to Azure Blob Storage.
//!
//! By default, Azure SDK clients sanitize headers to avoid logging sensitive information.
//! You can configure additional headers to be logged using `LoggingOptions`.
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
//! $env:AZURE_STORAGE_ACCOUNT_NAME="<your-storage-account>"
//! $env:RUST_LOG="<log-level>"
//! cargo run --package azure_storage_blob --example logging_and_headers
//! ```

use azure_core::http::{LoggingOptions, RequestContent};
use azure_identity::AzureCliCredential;
use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
use std::env;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber to see HTTP requests and responses.
    // Uses RUST_LOG environment variable if set, otherwise defaults to "trace" level
    // to ensure detailed HTTP request/response logs are visible when running this example.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace")),
        )
        .init();

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

    // Configure logging to show additional Azure Storage headers
    // By default, most headers are sanitized to avoid logging sensitive data.
    // Here we can explicitly allow certain Azure Storage headers to be logged.
    let mut client_options = BlobContainerClientOptions::default();
    client_options.client_options.logging = LoggingOptions {
        additional_allowed_header_names: vec![
            "x-ms-version".into(),
            "x-ms-blob-type".into(),
            "content-md5".into(),
            "x-ms-server-encrypted".into(),
            "x-ms-lease-state".into(),
            "accept-ranges".into(),
            "x-ms-lease-status".into(),
            "x-ms-creation-time".into(),
        ],
        additional_allowed_query_params: vec![],
    };

    let container_client = BlobContainerClient::new(
        &endpoint,
        container_name,
        Some(credential),
        Some(client_options),
    )?;

    // Create BlobClient
    let blob_client = container_client.blob_client(blob_name);

    // Create container if does not exist
    println!("Creating container '{}'...", container_name);
    match container_client.create_container(None).await {
        Ok(_) => println!("Container created successfully"),
        Err(e) => {
            if e.to_string().contains("ContainerAlreadyExists") || e.to_string().contains("409") {
                println!("Container already exists, continuing...");
            } else {
                return Err(e.into());
            }
        }
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

    println!(
        "\nNote: With RUST_LOG=trace, you should see detailed HTTP request/response logs above."
    );
    println!(
        "The configured headers (x-ms-version, x-ms-blob-type, etc.) are now visible in the logs."
    );

    Ok(())
}
