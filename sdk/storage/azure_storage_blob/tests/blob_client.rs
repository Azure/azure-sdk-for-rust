// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::recorded;
use azure_identity::DefaultAzureCredential;
use azure_storage_blob::{BlobBlobClientGetPropertiesOptions, BlobClient};
use std::{env, error::Error};

#[recorded::test(live)]
async fn test_get_blob_properties() -> Result<(), Box<dyn Error>> {
    // Setup
    let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
    let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
    let credential = DefaultAzureCredential::new()?;

    // Act
    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainer"),
        String::from("test_blob.txt"),
        credential,
        None,
    )?;
    blob_client
        .get_blob_properties(Some(BlobBlobClientGetPropertiesOptions::default()))
        .await?;

    // Assert
    Ok(())
}

#[recorded::test(live)]
async fn test_get_blob_properties_invalid_container() -> Result<(), Box<dyn Error>> {
    // Setup
    let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
    let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
    let credential = DefaultAzureCredential::new()?;

    // Act
    let blob_client = BlobClient::new(
        &endpoint,
        String::from("missingcontainer"),
        String::from("test_blob.txt"),
        credential,
        None,
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
