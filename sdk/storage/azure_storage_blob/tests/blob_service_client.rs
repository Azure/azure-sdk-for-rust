// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    models::{AccountKind, BlobServiceClientGetAccountInfoResultHeaders},
    BlobServiceClientGetPropertiesOptions,
};
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
async fn test_get_account_info(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording)?;

    // Act
    let response = service_client.get_account_info(None).await?;

    // Assert
    let sku_name = response.sku_name()?;
    let account_kind = response.account_kind()?;

    assert!(sku_name.is_some());
    assert_eq!(AccountKind::StorageV2, account_kind.unwrap());

    Ok(())
}
