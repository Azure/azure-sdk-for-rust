// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Blob container client example for Azure Blob Storage.
//!
//! This sample shows a small end-to-end container workflow:
//! 1. Create a container.
//! 2. Set and read container metadata.
//! 3. Upload blobs and list them (with the `include` option to request metadata).
//! 4. Set and read a stored access policy (for SAS delegation).
//! 5. Delete the container.
//!
//! # Prerequisites
//!
//! - Set `AZURE_STORAGE_ACCOUNT_NAME` to your storage account name.
//! - Sign in with `az login` (or any other credential flow supported by [`DeveloperToolsCredential`]).
//!
//! # Usage
//!
//! ```bash
//! az login
//! export AZURE_STORAGE_ACCOUNT_NAME="<your-storage-account>"
//! cargo run --package azure_storage_blob --example blob_container_client
//! ```

use std::{collections::HashMap, env};

use azure_core::{http::RequestContent, time::OffsetDateTime};
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::{
    models::{
        AccessPolicy, BlobContainerClientGetPropertiesResultHeaders,
        BlobContainerClientListBlobsOptions, ListBlobsIncludeItem, SignedIdentifiers,
    },
    BlobContainerClient,
};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.blob.core.windows.net/", account);
    let container_name = "test-container-lifecycle";

    let credential = DeveloperToolsCredential::new(None)?;
    let container_client =
        BlobContainerClient::new(&endpoint, container_name, Some(credential), None)?;

    println!("Creating container '{container_name}'...");
    container_client.create(None).await?;

    set_and_get_metadata(&container_client).await?;
    upload_and_list_blobs(&container_client).await?;
    set_and_get_access_policy(&container_client).await?;

    container_client.delete(None).await?;
    println!("Deleted container '{container_name}'");

    Ok(())
}

/// Sets two metadata keys on the container, then reads them back via `get_properties`.
async fn set_and_get_metadata(
    container_client: &BlobContainerClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = HashMap::from([
        ("sample".to_string(), "blob-container-client".to_string()),
        ("language".to_string(), "rust".to_string()),
    ]);
    container_client.set_metadata(&metadata, None).await?;

    let properties = container_client.get_properties(None).await?;
    let stored_metadata = properties.metadata()?;
    println!("Container metadata:");
    for (key, value) in stored_metadata {
        println!("  {key}: {value}");
    }

    Ok(())
}

/// Uploads three small blobs, then lists them, demonstrating the `include` option.
async fn upload_and_list_blobs(
    container_client: &BlobContainerClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let blob_names = ["blob-a.txt", "blob-b.txt", "blob-c.txt"];
    for name in &blob_names {
        let content = format!("content of {name}");
        container_client
            .blob_client(name)
            .upload(RequestContent::from(content.into_bytes()), None)
            .await?;
    }
    println!("Uploaded {} blobs", blob_names.len());

    // Use the `include` option to request additional blob metadata in the listing.
    let options = BlobContainerClientListBlobsOptions {
        include: Some(vec![ListBlobsIncludeItem::Metadata]),
        ..Default::default()
    };
    let mut blobs = container_client.list_blobs(Some(options))?;
    let mut total = 0usize;
    println!("Listing blobs:");
    while let Some(blob) = blobs.try_next().await? {
        println!("  {}", blob.name.unwrap_or_default());
        total += 1;
    }
    println!("Found {total} blob(s) total");

    Ok(())
}

/// Sets an access policy named `"read-list"` on the container, then retrieves
/// and prints it, and finally clears all policies.
///
/// Access policies can be referenced by SAS tokens so that permissions can be
/// revoked or extended without regenerating the SAS token itself.
async fn set_and_get_access_policy(
    container_client: &BlobContainerClient,
) -> Result<(), Box<dyn std::error::Error>> {
    // expiry: 2027-01-01T00:00:00Z (unix timestamp 1 798 761 600)
    let expiry =
        OffsetDateTime::from_unix_timestamp(1_798_761_600).expect("hardcoded timestamp is valid");
    let policy = AccessPolicy {
        start: None,
        expiry: Some(expiry),
        // r = read, l = list
        permission: Some("rl".to_string()),
    };
    // Build SignedIdentifiers from a HashMap via the From impl - one entry per policy ID.
    let identifiers: SignedIdentifiers = HashMap::from([("read-list".to_string(), policy)]).into();

    container_client
        .set_access_policy(identifiers.try_into()?, None)
        .await?;
    println!("Set access policy 'read-list' on the container");

    // Read back the policies to confirm.
    let response = container_client.get_access_policy(None).await?;
    let policies = response.into_model()?;
    for identifier in policies.items.unwrap_or_default() {
        let id = identifier.id.as_deref().unwrap_or("<unnamed>");
        if let Some(ap) = &identifier.access_policy {
            println!(
                "Policy '{}': permissions={}, expiry={:?}",
                id,
                ap.permission.as_deref().unwrap_or(""),
                ap.expiry,
            );
        }
    }

    // Clear all policies by converting an empty HashMap.
    let empty: SignedIdentifiers = HashMap::<String, AccessPolicy>::new().into();
    container_client
        .set_access_policy(empty.try_into()?, None)
        .await?;
    println!("Cleared all access policies on the container");

    Ok(())
}
