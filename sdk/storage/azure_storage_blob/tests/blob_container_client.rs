// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{clients::ContainerClient, BlobClientOptions};
use std::{env, error::Error};

#[recorded::test]
async fn test_create_container(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let mut options = BlobClientOptions::default();
    recording.instrument(&mut options.client_options);

    // Setup
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    // Act
    let container_client = ContainerClient::new(
        &endpoint,
        String::from("testcontainer11"),
        recording.credential(),
        Some(options),
    )?;

    // Assert
    container_client.create_container(None).await?;

    container_client.delete_container(None).await?;
    Ok(())
}
