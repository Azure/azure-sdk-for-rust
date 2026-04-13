// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Blob service client example for Azure Blob Storage.
//!
//! This sample demonstrates service-level operations:
//! 1. Create a container through [`BlobServiceClient`].
//! 2. Set and read service properties (CORS).
//! 3. List containers with a prefix.
//! 4. Search for blobs across containers using blob index tag filters.
//! 5. Delete the temporary container.
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
//! cargo run --package azure_storage_blob --example blob_service_client
//! ```

use std::{collections::HashMap, env};

use azure_core::http::RequestContent;
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::{
    models::{
        BlobServiceClientListContainersOptions, BlobServiceProperties, BlobTags, CorsRule,
        ListContainersIncludeType,
    },
    BlobServiceClient,
};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.blob.core.windows.net/", account);
    let container_name = "test-container-service-client";

    let credential = DeveloperToolsCredential::new(None)?;
    let service_client = BlobServiceClient::new(&endpoint, Some(credential), None)?;
    let container_client = service_client.blob_container_client(container_name);

    println!("Creating container '{container_name}'...");
    container_client.create(None).await?;
    container_client
        .set_metadata(
            &HashMap::from([("sample".to_string(), "service-client".to_string())]),
            None,
        )
        .await?;

    set_and_get_service_properties(&service_client).await?;
    list_containers(&service_client, container_name).await?;
    find_blobs_by_tags(&service_client, container_name).await?;

    container_client.delete(None).await?;
    println!("Deleted container '{container_name}'");

    Ok(())
}

/// Sets a CORS rule on the service, then reads back the properties to confirm.
async fn set_and_get_service_properties(
    service_client: &BlobServiceClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let properties = BlobServiceProperties {
        cors: Some(vec![CorsRule {
            allowed_origins: Some("https://example.com".to_string()),
            allowed_methods: Some("GET,PUT".to_string()),
            max_age_in_seconds: Some(3600),
            exposed_headers: Some("x-ms-meta-data".to_string()),
            allowed_headers: Some("x-ms-meta-target".to_string()),
        }]),
        ..Default::default()
    };
    service_client
        .set_properties(properties.try_into()?, None)
        .await?;
    println!("Updated blob service properties");

    let retrieved = service_client.get_properties(None).await?.into_model()?;
    println!(
        "Service properties loaded. CORS rules configured: {}",
        retrieved.cors.as_ref().map(Vec::len).unwrap_or(0)
    );

    Ok(())
}

/// Lists containers matching a prefix, printing the name and metadata for each.
async fn list_containers(
    service_client: &BlobServiceClient,
    prefix: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let options = BlobServiceClientListContainersOptions {
        prefix: Some(prefix.to_string()),
        include: Some(vec![ListContainersIncludeType::Metadata]),
        ..Default::default()
    };
    let mut containers = service_client.list_containers(Some(options))?;
    println!("Listing containers with prefix '{prefix}'...");
    while let Some(container) = containers.try_next().await? {
        println!("  Container: {}", container.name.unwrap_or_default());
        for (key, value) in container.metadata.unwrap_or_default() {
            println!("    {key}: {value}");
        }
    }

    Ok(())
}

/// Tags a blob in the test container, then searches for it across the account using
/// a tag filter expression.
///
/// Blob index tags are searchable without scanning blob content. Tag indexing typically
/// propagates within seconds but may take up to 30 seconds on some accounts.
async fn find_blobs_by_tags(
    service_client: &BlobServiceClient,
    container_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Upload a blob and set index tags on it.
    let blob_client = service_client.blob_client(container_name, "tagged-blob.txt");
    blob_client
        .upload(RequestContent::from(b"taggable content".to_vec()), None)
        .await?;
    let tags: BlobTags =
        HashMap::from([("sample".to_string(), "service-client".to_string())]).into();
    blob_client.set_tags(tags.try_into()?, None).await?;
    println!("Tagged 'tagged-blob.txt' with sample=service-client");

    // Tag names must be in double-quotes and values in single-quotes.
    let filter = "\"sample\" = 'service-client'";
    let segment = service_client
        .find_blobs_by_tags(filter, None)
        .await?
        .into_model()?;
    let blobs = segment.blobs.unwrap_or_default();
    println!("find_blobs_by_tags: {} result(s)", blobs.len());
    for item in blobs {
        println!(
            "  {}/{}",
            item.container_name.as_deref().unwrap_or("<?>"),
            item.name.as_deref().unwrap_or("<?>"),
        );
    }

    Ok(())
}
