// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Blob client example for Azure Blob Storage.
//!
//! This sample demonstrates blob-level operations on a [`BlobClient`]:
//! 1. Check existence with `exists`.
//! 2. Set and read blob metadata via `set_metadata` / `get_properties`.
//! 3. Set and retrieve blob index tags (searchable server-side without downloading).
//! 4. Create a snapshot and download its frozen content via `create_snapshot` / `with_snapshot`.
//! 5. Move a blob to a different access tier with `set_tier`.
//! 6. Acquire a timed lease to demonstrate exclusive write access.
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
//! cargo run --package azure_storage_blob --example blob_client
//! ```

use std::{collections::HashMap, env};

use azure_core::http::RequestContent;
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::{
    models::{
        AccessTier, BlobClientAcquireLeaseResultHeaders, BlobClientCreateSnapshotResultHeaders,
        BlobClientGetPropertiesResultHeaders, BlobClientSetMetadataOptions, BlobTags,
    },
    BlobContainerClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.blob.core.windows.net/", account);
    let container_name = "test-container-blob-client";
    let blob_name = "sample.txt";

    let credential = DeveloperToolsCredential::new(None)?;
    let container_client =
        BlobContainerClient::new(&endpoint, container_name, Some(credential), None)?;

    container_client.create(None).await?;
    println!("Created container '{container_name}'");

    let blob_client = container_client.blob_client(blob_name);

    // exists() returns false before the blob is uploaded.
    println!("Blob exists before upload: {}", blob_client.exists().await?);

    // Upload a block blob.
    blob_client
        .upload(
            RequestContent::from(b"Hello from blob client example".to_vec()),
            None,
        )
        .await?;
    println!("Uploaded '{blob_name}'");

    // exists() now returns true.
    println!("Blob exists after upload:  {}", blob_client.exists().await?);

    // Set blob metadata - arbitrary key/value pairs stored with the blob.
    blob_client
        .set_metadata(
            &HashMap::from([
                ("sample".to_string(), "blob-client".to_string()),
                ("language".to_string(), "rust".to_string()),
            ]),
            None,
        )
        .await?;
    println!("Set metadata on '{blob_name}'");

    // get_properties returns response headers carrying the blob type, size, metadata, and more.
    let props = blob_client.get_properties(None).await?;
    println!("Blob type:      {:?}", props.blob_type()?);
    println!("Content-length: {:?}", props.content_length()?);
    println!("Metadata:       {:?}", props.metadata()?);

    // Set blob index tags - searchable across the account via find_blobs_by_tags.
    // The From<HashMap<String, String>> for BlobTags impl makes construction ergonomic.
    let tags: BlobTags = HashMap::from([
        ("project".to_string(), "azure-sdk-rust".to_string()),
        ("env".to_string(), "sample".to_string()),
    ])
    .into();
    blob_client.set_tags(tags.try_into()?, None).await?;
    println!("Set index tags on '{blob_name}'");

    // Read tags back - into_model()?.into() converts BlobTags -> HashMap via the From impl.
    let retrieved: HashMap<String, String> = blob_client.get_tags(None).await?.into_model()?.into();
    println!("Tags: {retrieved:?}");

    // Snapshot the blob to preserve its current content as an immutable point-in-time copy.
    let snap_resp = blob_client.create_snapshot(None).await?;
    let snapshot_id = snap_resp
        .snapshot()?
        .ok_or("service did not return a snapshot ID")?;
    println!("Created snapshot: {snapshot_id}");

    // Overwrite the live blob...
    blob_client
        .upload(
            RequestContent::from(b"Updated content - snapshot holds original".to_vec()),
            None,
        )
        .await?;

    // ...but with_snapshot returns a BlobClient pointed at the frozen snapshot.
    let snapshot_client = blob_client.with_snapshot(&snapshot_id)?;
    let (_, _, body) = snapshot_client.download(None).await?.deconstruct();
    let data = body.collect().await?;
    println!("Snapshot content: {}", String::from_utf8_lossy(&data));

    // Move the blob to Cool tier (lower storage cost for infrequently-accessed data).
    // Requires a general-purpose v2 or Blob Storage account.
    blob_client.set_tier(AccessTier::Cool, None).await?;
    let props = blob_client.get_properties(None).await?;
    println!("Access tier after set_tier: {:?}", props.access_tier()?);

    // Acquire a 30-second lease - an exclusive write lock on this blob.
    // Use -1 instead of 30 for an infinite lease that must be released explicitly.
    let lease_resp = blob_client.acquire_lease(30, None).await?;
    let lease_id = lease_resp
        .lease_id()?
        .ok_or("service did not return a lease ID")?;
    println!("Acquired blob lease: {lease_id}");

    // Any mutating call without the lease ID is rejected (412 Precondition Failed).
    match blob_client
        .set_metadata(&HashMap::from([("k".to_string(), "v".to_string())]), None)
        .await
    {
        Ok(_) => println!("Unexpected success - blob should be locked"),
        Err(err) => println!(
            "Write without lease rejected (expected): {}",
            err.http_status()
                .map(|s| s.to_string())
                .unwrap_or_else(|| err.to_string())
        ),
    }

    // Supplying the lease ID in options lets the write succeed.
    let set_meta_opts = BlobClientSetMetadataOptions {
        lease_id: Some(lease_id.clone()),
        ..Default::default()
    };
    blob_client
        .set_metadata(
            &HashMap::from([("locked-by".to_string(), "sample".to_string())]),
            Some(set_meta_opts),
        )
        .await?;
    println!("Set metadata while holding lease");

    blob_client.release_lease(lease_id, None).await?;
    println!("Released blob lease");

    container_client.delete(None).await?;
    println!("Deleted container '{container_name}'");

    Ok(())
}
