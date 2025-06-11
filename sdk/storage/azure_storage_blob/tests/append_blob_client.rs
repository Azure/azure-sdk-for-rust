// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{BlobClientGetPropertiesResultHeaders, BlobType};
use azure_storage_blob_test::{get_blob_name, get_container_client};
use std::error::Error;

#[recorded::test]
async fn test_create_append_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();

    append_blob_client.create(None).await?;

    // Assert
    let blob_properties = blob_client.get_properties(None).await?;
    let blob_type = blob_properties.blob_type()?;
    let content_length = blob_properties.content_length()?;

    assert_eq!(0, content_length.unwrap());
    assert_eq!(BlobType::AppendBlob, blob_type.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}
