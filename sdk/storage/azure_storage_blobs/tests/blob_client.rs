// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{base64, headers::HeaderName, Bytes, Request, RequestContent, StatusCode};
use azure_core_test::recorded;
use azure_identity::DefaultAzureCredentialBuilder;
use azure_storage_blobs::{
    clients::{BlobClient, BlobContainerClient},
    models::{
        BlobBlobClientDownloadOptions, BlobBlobClientGetPropertiesOptions,
        BlobBlockBlobClientCommitBlockListOptions, BlobBlockBlobClientGetBlockListOptions,
        BlobBlockBlobClientStageBlockOptions, BlobType, BlockList, BlockListType, BlockLookupList,
    },
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
            i64::try_from(data.len())?,
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
    assert!(response.is_err());
    let error = response.unwrap_err().http_status();
    assert_eq!(Some(StatusCode::NotFound), error);

    Ok(())
}

#[recorded::test(live)]
async fn test_download_blob() -> Result<(), Box<dyn Error>> {
    // Setup

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
            i64::try_from(data.len())?,
            None,
        )
        .await?;
    let response = blob_client
        .download_blob(Some(BlobBlobClientDownloadOptions::default()))
        .await?;

    // Assert
    let (status_code, headers, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(
        "21",
        headers.get_str(&HeaderName::from_static("content-length"))?
    );
    assert_eq!(Bytes::from_static(data), response_body.collect().await?);

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
            false,
            i64::try_from(data.len())?,
            None,
        )
        .await?;

    // Assert
    assert_eq!(response.status(), StatusCode::Created);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test(live)]
async fn test_upload_blob_overwrite() -> Result<(), Box<dyn Error>> {
    // Setup
    let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
    let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
    let credential = DefaultAzureCredentialBuilder::default().build()?;

    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        String::from("testcontainer4"),
        credential.clone(),
        None,
    )?;
    container_client.create_container(None).await?;

    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainer4"),
        String::from("test_upload_blob_overwrite.txt"),
        credential,
        Some(BlobClientOptions::default()),
    )?;

    let data = b"hello rusty world";
    blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            false,
            i64::try_from(data.len())?,
            None,
        )
        .await?;

    let data2 = b"hello overwritten rusty world";
    let response = blob_client
        .upload_blob(
            RequestContent::from(data2.to_vec()),
            true,
            i64::try_from(data2.len())?,
            None,
        )
        .await?;

    // Assert
    assert_eq!(response.status(), StatusCode::Created);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test(live)]
async fn test_put_block_list() -> Result<(), Box<dyn Error>> {
    // Setup
    let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
    let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
    let credential = DefaultAzureCredentialBuilder::default().build()?;

    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        String::from("testcontainerpbl"),
        credential.clone(),
        None,
    )?;
    container_client.create_container(None).await?;

    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainerpbl"),
        String::from("testblob"),
        credential,
        Some(BlobClientOptions::default()),
    )?;

    let block_1 = b"AAA";
    let block_2 = b"BBB";
    let block_3 = b"CCC";

    blob_client
        .stage_block(
            "1",
            i64::try_from(block_1.len())?,
            RequestContent::from(block_1.to_vec()),
            Some(BlobBlockBlobClientStageBlockOptions::default()),
        )
        .await?;

    blob_client
        .stage_block(
            "2",
            i64::try_from(block_2.len())?,
            RequestContent::from(block_2.to_vec()),
            Some(BlobBlockBlobClientStageBlockOptions::default()),
        )
        .await?;
    blob_client
        .stage_block(
            "3",
            i64::try_from(block_3.len())?,
            RequestContent::from(block_3.to_vec()),
            Some(BlobBlockBlobClientStageBlockOptions::default()),
        )
        .await?;

    let latest_blocks: Vec<String> = vec![
        base64::encode("1"),
        base64::encode("2"),
        base64::encode("3"),
    ];
    let block_lookup_list = BlockLookupList {
        committed: None,
        latest: Some(latest_blocks),
        uncommitted: None,
    };

    let request_content = RequestContent::try_from(block_lookup_list)?;

    blob_client
        .commit_block_list(
            request_content,
            Some(BlobBlockBlobClientCommitBlockListOptions::default()),
        )
        .await?;

    let response = blob_client
        .download_blob(Some(BlobBlobClientDownloadOptions::default()))
        .await?;

    // Assert
    let (status_code, headers, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(
        "9",
        headers.get_str(&HeaderName::from_static("content-length"))?
    );
    assert_eq!(
        Bytes::from_static(b"AAABBBCCC"),
        response_body.collect().await?
    );

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test(live)]
async fn test_get_block_list() -> Result<(), Box<dyn Error>> {
    // Setup
    let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
    let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
    let credential = DefaultAzureCredentialBuilder::default().build()?;

    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        String::from("testcontainergbl"),
        credential.clone(),
        None,
    )?;
    container_client.create_container(None).await?;

    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainergbl"),
        String::from("testblob"),
        credential,
        Some(BlobClientOptions::default()),
    )?;

    let block_1 = b"AAA";
    let block_2 = b"BBB";
    let block_3 = b"CCC";

    blob_client
        .stage_block(
            "1",
            i64::try_from(block_1.len())?,
            RequestContent::from(block_1.to_vec()),
            Some(BlobBlockBlobClientStageBlockOptions::default()),
        )
        .await?;

    blob_client
        .stage_block(
            "2",
            i64::try_from(block_2.len())?,
            RequestContent::from(block_2.to_vec()),
            Some(BlobBlockBlobClientStageBlockOptions::default()),
        )
        .await?;
    blob_client
        .stage_block(
            "3",
            i64::try_from(block_3.len())?,
            RequestContent::from(block_3.to_vec()),
            Some(BlobBlockBlobClientStageBlockOptions::default()),
        )
        .await?;

    // Three staged blocks
    let mut block_list = blob_client
        .get_block_list(
            BlockListType::All,
            Some(BlobBlockBlobClientGetBlockListOptions::default()),
        )
        .await?
        .into_body()
        .await?;
    assert!(block_list.uncommitted_blocks.is_some());
    assert!(block_list.committed_blocks.is_none());

    let latest_blocks: Vec<String> = vec![
        base64::encode("1"),
        base64::encode("2"),
        base64::encode("3"),
    ];
    let block_lookup_list = BlockLookupList {
        committed: None,
        latest: Some(latest_blocks),
        uncommitted: None,
    };

    let request_content = RequestContent::try_from(block_lookup_list)?;

    blob_client
        .commit_block_list(
            request_content,
            Some(BlobBlockBlobClientCommitBlockListOptions::default()),
        )
        .await?;

    // Three committed blocks
    block_list = blob_client
        .get_block_list(
            BlockListType::All,
            Some(BlobBlockBlobClientGetBlockListOptions::default()),
        )
        .await?
        .into_body()
        .await?;
    assert!(block_list.uncommitted_blocks.is_none());
    assert!(block_list.committed_blocks.is_some());

    container_client.delete_container(None).await?;
    Ok(())
}
