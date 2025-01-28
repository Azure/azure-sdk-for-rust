// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::recorded;
use azure_identity::DefaultAzureCredentialBuilder;
use azure_storage_blob::{
    BlobBlobClientDownloadOptions, BlobBlobClientGetPropertiesOptions, BlobClient,
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
    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainer"),
        String::from("test_blob.txt"),
        credential,
        Some(BlobClientOptions::default()),
    )?;
    let response = blob_client
        .get_blob_properties(Some(BlobBlobClientGetPropertiesOptions::default()))
        .await;

    // Assert
    assert!(response.is_ok());
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

    use azure_core::headers::HeaderName;
    let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
    let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
    let credential = DefaultAzureCredentialBuilder::default().build()?;

    // Act
    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainer"),
        String::from("test_blob.txt"),
        credential,
        Some(BlobClientOptions::default()),
    )?;
    let response = blob_client
        .download_blob(Some(BlobBlobClientDownloadOptions::default()))
        .await;

    // Assert
    assert!(response.is_ok());
    let (status_code, headers, response_body) = response.unwrap().deconstruct();
    assert!(status_code.is_success());
    assert_eq!(
        "11",
        headers.get_str(&HeaderName::from_static("content-length"))?
    );
    assert_eq!("hello world", response_body.collect_string().await?);
    Ok(())
}
