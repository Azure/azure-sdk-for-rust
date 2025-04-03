// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    BlobServiceClient, BlobServiceClientGetPropertiesOptions, BlobServiceClientOptions,
};
use azure_storage_blob_test::recorded_test_setup;
use std::error::Error;

#[recorded::test]
async fn test_get_service_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording);

    let service_client_options = BlobServiceClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };

    // Act
    let service_client = BlobServiceClient::new(
        &endpoint,
        recording.credential(),
        Some(service_client_options),
    )?;
    let response = service_client
        .get_properties(Some(BlobServiceClientGetPropertiesOptions::default()))
        .await?;

    // Assert
    let storage_service_properties = response.into_body().await?;
    let hour_metrics = storage_service_properties.hour_metrics;
    assert!(hour_metrics.is_some());
    Ok(())
}
