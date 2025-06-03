// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::RequestContent;
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{BlobServiceClientGetPropertiesOptions, StorageServiceProperties};
use azure_storage_blob_test::get_blob_service_client;
use std::error::Error;

#[recorded::test]
async fn test_get_service_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording)?;

    let response = service_client
        .get_properties(Some(BlobServiceClientGetPropertiesOptions::default()))
        .await?;

    // Assert
    let storage_service_properties = response.into_body().await?;
    let hour_metrics = storage_service_properties.hour_metrics;
    assert!(hour_metrics.is_some());
    Ok(())
}

#[recorded::test]
async fn test_set_service_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording)?;

    // Storage Service Properties
    let storage_service_properties = StorageServiceProperties {
        default_service_version: Some("2022-11-02".to_string()),
        ..Default::default()
    };
    let request_content: RequestContent<StorageServiceProperties> =
        storage_service_properties.try_into()?;

    service_client.set_properties(request_content, None).await?;

    // Assert
    let response = service_client.get_properties(None).await?;
    let storage_service_properties = response.into_body().await?;
    let default_service_version = storage_service_properties.default_service_version;
    assert_eq!("2022-11-02".to_string(), default_service_version.unwrap());
    Ok(())
}
