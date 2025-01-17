// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::{recorded, TestContext};
use azure_identity::DefaultAzureCredentialBuilder;
use azure_storage_blob::blob_client::BlobClientOptions;
use azure_storage_blob::clients::BlobClient;
use azure_storage_blob::models::blob_properties::{build_from_response_headers, BlobProperties};
use azure_storage_blob::BlobBlobClientGetPropertiesOptions;
use std::error::Error;

#[cfg(test)]
mod tests {

    use azure_storage_blob::models::blob_properties;

    use super::*;

    #[recorded::test(live)]
    async fn test_get_blob_properties() -> Result<(), Box<dyn Error>> {
        let credential = DefaultAzureCredentialBuilder::default().build().unwrap();
        let blob_client = BlobClient::new(
            String::from("https://vincenttranstock.blob.core.windows.net/"),
            String::from("testcontainer"),
            String::from("test_blob.txt"),
            credential,
            BlobClientOptions::default(),
        )
        .unwrap();
        let response = blob_client
            .get_blob_properties(Some(BlobBlobClientGetPropertiesOptions::default()))
            .await;

        println!("{:?}", response);
        Ok(())
    }

    #[recorded::test(live)]
    async fn test_get_blob_properties_invalid_container() -> Result<(), Box<dyn Error>> {
        let credential = DefaultAzureCredentialBuilder::default().build().unwrap();
        let blob_client = BlobClient::new(
            String::from("https://vincenttranstock.blob.core.windows.net/"),
            String::from("missingcontainer"),
            String::from("test_blob.txt"),
            credential,
            BlobClientOptions::default(),
        )
        .unwrap();
        let response = blob_client
            .get_blob_properties(Some(BlobBlobClientGetPropertiesOptions::default()))
            .await;
        println!("{:?}", response);
        Ok(())
    }
}
