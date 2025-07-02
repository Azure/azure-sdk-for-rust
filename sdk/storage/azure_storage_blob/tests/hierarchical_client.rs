// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::{recorded, TestContext};
use azure_storage_blob_test::{get_blob_name, get_container_client};
use std::error::Error;

#[recorded::test]
async fn test(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;
    container_client.create_container(None).await?;
    let blob_client_1 = container_client.blob_client(get_blob_name(recording));
    let blob_client_2 = container_client.blob_client("gonna/get/complex".to_string());

    let hns_file_client = blob_client_1.file_hns_client();
    let hns_directory_client = blob_client_2.directory_hns_client();

    hns_file_client.create(None).await?;
    hns_directory_client.create(None).await?;

    Ok(())
}
