// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{RequestContent, StatusCode},
    Bytes,
};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    AccessTierOptional, BlobClientDownloadResultHeaders, BlobClientGetPropertiesResultHeaders,
    BlobClientSetMetadataOptions, BlobClientSetPropertiesOptions, BlockBlobClientUploadOptions,
    LeaseState,
};
use azure_storage_blob_test::{create_test_blob, get_blob_name, get_container_client};
use std::{collections::HashMap, error::Error};

#[recorded::test]
async fn test_get_blob_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));

    let hns_file_client = blob_client.file_hns_client();
    let hns_directory_client = blob_client.directory_hns_client();

    Ok(())
}
