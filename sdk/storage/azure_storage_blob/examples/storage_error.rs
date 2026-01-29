// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This example demonstrates how to use the `StorageError` model to get programmatic
//! access to error details returned by Azure Blob Storage.
//!
//! The `StorageError` model provides structured access to error information including:
//! - HTTP status code
//! - Storage-specific error code (e.g., `BlobNotFound`, `ContainerNotFound`)
//! - Error message with details
//! - Request ID for troubleshooting
//! - Copy source error information (for copy operations)
//! - Additional error info that may be returned by the service
//!
//! # Prerequisites
//!
//! - Set the `AZURE_STORAGE_ACCOUNT_NAME` environment variable to your storage account name
//! - Authenticate using Azure CLI: `az login`
//!
//! # Usage
//!
//! ```bash
//! az login
//! AZURE_STORAGE_ACCOUNT_NAME="<your-storage-account>" cargo run --package azure_storage_blob --example storage_error
//! ```

use azure_core::error::ErrorKind;
use azure_identity::AzureCliCredential;
use azure_storage_blob::{
    models::{StorageError, StorageErrorCode},
    BlobClient,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get Azure Storage Account name from environment variable
    let account = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.blob.core.windows.net", account);
    let container_name = "nonexistent-container";
    let blob_name = "nonexistent-blob.txt";

    // Create OAuth credentials using Azure CLI
    println!("Authenticating with Azure CLI...");
    let credential = AzureCliCredential::new(None)?;

    // Create a BlobClient pointing to a blob that doesn't exist
    let blob_client =
        BlobClient::new(&endpoint, container_name, blob_name, Some(credential), None)?;

    // Attempt to download a blob that doesn't exist to force an error
    println!("Attempting to download a blob that doesn't exist...");
    let result = blob_client.download(None).await;

    match result {
        Ok(_) => {
            println!("Blob downloaded successfully (unexpected)");
        }
        Err(error) => {
            // Check if this is an HTTP response error
            if matches!(error.kind(), ErrorKind::HttpResponse { .. }) {
                // Convert the azure_core::Error to a StorageError for programmatic access
                let storage_error: StorageError = error.try_into()?;

                // StorageError implements Display
                println!("\n=== StorageError (Display) ===");
                println!("{storage_error}");

                // For programmatic error handling, access fields directly:
                println!("\n=== Programmatic Access ===");
                println!("HTTP Status Code: {}", storage_error.status_code);

                if let Some(error_code) = &storage_error.error_code {
                    // Handle specific error codes
                    match error_code {
                        StorageErrorCode::BlobNotFound => {
                            println!("The blob does not exist.");
                        }
                        StorageErrorCode::ContainerNotFound => {
                            println!("The container does not exist.");
                        }
                        StorageErrorCode::AuthorizationFailure => {
                            println!("Authorization failed. Check your permissions.");
                        }
                        StorageErrorCode::AuthenticationFailed => {
                            println!("Authentication failed. Verify your credentials.");
                        }
                        _ => {
                            println!("Other error: {error_code}");
                        }
                    }
                }

                // Request ID is useful for Azure support troubleshooting
                if let Some(request_id) = &storage_error.request_id {
                    println!("Request ID: {request_id}");
                }
            } else {
                // Handle non-HTTP errors (e.g., network errors, timeouts)
                println!("Non-HTTP error occurred: {:?}", error);
            }
        }
    }

    Ok(())
}
