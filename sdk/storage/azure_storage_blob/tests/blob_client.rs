// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{headers::HeaderName, RequestContent, StatusCode};
use azure_core_test::recorded;
use azure_identity::DefaultAzureCredentialBuilder;
use azure_storage_blob::{
    clients::{BlobClient, BlobContainerClient},
    models::{BlobBlobClientGetPropertiesOptions, BlobBlockBlobClientUploadOptions, BlobType},
    BlobClientOptions,
};
use std::{env, error::Error};

#[recorded::test(live)]
async fn test_get_blob_properties() -> Result<(), Box<dyn Error>> {
    // Setup
    let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
    let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
    let credential = DefaultAzureCredentialBuilder::default().build()?;

    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        String::from("testcontainer1"),
        credential.clone(),
        None,
    )?;
    container_client.create_container(None).await?;

    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainer1"),
        String::from("test_blob.txt"),
        credential,
        Some(BlobClientOptions::default()),
    )?;
    let data = b"hello rusty world";
    blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            true,
            data.len() as i64,
            None,
        )
        .await?;
    let response = blob_client
        .get_blob_properties(Some(BlobBlobClientGetPropertiesOptions::default()))
        .await;

    // Assert
    assert!(response.is_ok());
    let blob_properties = response?;
    assert_eq!(blob_properties.blob_type, Some(BlobType::BlockBlob));
    assert_eq!(blob_properties.content_length, Some(17));

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test(live)]
async fn test_get_blob_properties_invalid_container() -> Result<(), Box<dyn Error>> {
    // Setup
    let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
    let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
    let credential = DefaultAzureCredentialBuilder::default().build()?;

    // Act
    let blob_client = BlobClient::new(
        &endpoint,
        String::from("missingcontainer"),
        String::from("test_blob.txt"),
        credential,
        Some(BlobClientOptions::default()),
    )?;
    let response = blob_client
        .get_blob_properties(Some(BlobBlobClientGetPropertiesOptions::default()))
        .await;

    // Assert
    assert_eq!(
        String::from("HttpResponse(NotFound, \"ContainerNotFound\")"),
        response.unwrap_err().kind().to_string()
    );

    Ok(())
}

#[recorded::test(live)]
async fn test_download_blob() -> Result<(), Box<dyn Error>> {
    // Setup

    use azure_storage_blob::models::BlobBlobClientDownloadOptions;
    let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
    let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
    let credential = DefaultAzureCredentialBuilder::default().build()?;

    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        String::from("testcontainer2"),
        credential.clone(),
        None,
    )?;
    container_client.create_container(None).await?;

    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainer2"),
        String::from("test_blob.txt"),
        credential,
        Some(BlobClientOptions::default()),
    )?;
    let data = b"test download content";
    blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            true,
            data.len() as i64,
            None,
        )
        .await?;
    let response = blob_client
        .download_blob(Some(BlobBlobClientDownloadOptions::default()))
        .await;

    // Assert
    assert!(response.is_ok());
    let (status_code, headers, response_body) = response.unwrap().deconstruct();
    assert!(status_code.is_success());
    assert_eq!(
        "21",
        headers.get_str(&HeaderName::from_static("content-length"))?
    );
    assert_eq!(
        "test download content",
        response_body.collect_string().await?
    );

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test(live)]
async fn test_upload_blob() -> Result<(), Box<dyn Error>> {
    // Setup
    let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
    let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
    let credential = DefaultAzureCredentialBuilder::default().build()?;

    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        String::from("testcontainer3"),
        credential.clone(),
        None,
    )?;
    container_client.create_container(None).await?;

    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainer3"),
        String::from("test_upload_blob.txt"),
        credential,
        Some(BlobClientOptions::default()),
    )?;

    let data = b"hello rusty world";
    let response = blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            false, // overwrite=True to make re-running easier
            data.len() as i64,
            None,
        )
        .await?;

    // Assert
    assert_eq!(response.status(), StatusCode::Created);

    container_client.delete_container(None).await?;
    Ok(())
}
