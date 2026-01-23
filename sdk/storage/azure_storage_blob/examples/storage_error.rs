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
//! $env:AZURE_STORAGE_ACCOUNT_NAME="<your-storage-account>"
//! cargo run --package azure_storage_blob --example storage_error
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

                println!("\n=== StorageError Details ===");

                // HTTP Status Code
                println!("HTTP Status Code: {}", storage_error.status_code());

                // Storage Error Code Model - Use this for programmatic error handling
                if let Some(error_code) = storage_error.error_code() {
                    println!("Storage Error Code: {:?}", error_code);

                    // Example: Handle specific error codes
                    match error_code {
                        StorageErrorCode::BlobNotFound => {
                            println!("  -> The blob does not exist.");
                        }
                        StorageErrorCode::ContainerNotFound => {
                            println!("  -> The container does not exist.");
                        }
                        StorageErrorCode::AuthorizationFailure => {
                            println!("  -> Authorization failed. Check your permissions.");
                        }
                        StorageErrorCode::AuthenticationFailed => {
                            println!("  -> Authentication failed. Verify your credentials.");
                        }
                        _ => {
                            println!("  -> Other error: {:?}", error_code);
                        }
                    }
                }

                // Error message
                if let Some(message) = storage_error.message() {
                    println!("Error Message: {}", message);
                }

                // Request ID - Useful for Azure support troubleshooting
                if let Some(request_id) = storage_error.request_id() {
                    println!("Request ID: {}", request_id);
                }

                // Copy source error details (for copy operations)
                if let Some(copy_source_status) = storage_error.copy_source_status_code() {
                    println!("\n=== Copy Source Error Details ===");
                    println!("Copy Source Status Code: {}", copy_source_status);

                    if let Some(code) = storage_error.copy_source_error_code() {
                        println!("Copy Source Error Code: {}", code);
                    }
                    if let Some(message) = storage_error.copy_source_error_message() {
                        println!("Copy Source Error Message: {}", message);
                    }
                }

                // Additional error info from the Service (if any)
                let additional_info = storage_error.additional_error_info();
                if !additional_info.is_empty() {
                    println!("\n=== Additional Error Information ===");
                    for (key, value) in additional_info {
                        println!("  {}: {}", key, value);
                    }
                }

                // StorageError implements Display for easy logging
                println!("\n=== Full Error Display ===");
                println!("{}", storage_error);
            } else {
                // Handle non-HTTP errors (e.g., network errors, timeouts)
                println!("Non-HTTP error occurred: {:?}", error);
            }
        }
    }

    Ok(())
}
