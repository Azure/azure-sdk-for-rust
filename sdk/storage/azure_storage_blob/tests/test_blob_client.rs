// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::recorded;
use azure_identity::DefaultAzureCredentialBuilder;
use azure_storage_blob::blob_blob_client::BlobBlobClientGetPropertiesOptions;
use azure_storage_blob::blob_client::BlobClientOptions;
use azure_storage_blob::clients::BlobClient;
use std::env;
use std::error::Error;

#[cfg(test)]
mod tests {

    use super::*;

    #[recorded::test(live)]
    async fn test_get_blob_properties() -> Result<(), Box<dyn Error>> {
        let storage_account_name =
            env::var("STORAGE_ACCOUNT_NAME").unwrap_or("storagename".to_string());
        let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
        let credential = DefaultAzureCredentialBuilder::default().build()?;

        let blob_client = BlobClient::new(
            endpoint,
            String::from("testcontainer"),
            String::from("test_blob.txt"),
            credential,
            Some(BlobClientOptions::default()),
        )?;
        let response = blob_client
            .get_blob_properties(Some(BlobBlobClientGetPropertiesOptions::default()))
            .await;

        assert!(response.is_ok());
        Ok(())
    }

    #[recorded::test(live)]
    async fn test_get_blob_properties_invalid_container() -> Result<(), Box<dyn Error>> {
        let storage_account_name =
            env::var("STORAGE_ACCOUNT_NAME").unwrap_or("storagename".to_string());
        let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
        let credential = DefaultAzureCredentialBuilder::default().build()?;

        let blob_client = BlobClient::new(
            endpoint,
            String::from("missingcontainer"),
            String::from("test_blob.txt"),
            credential,
            Some(BlobClientOptions::default()),
        )?;
        let response = blob_client
            .get_blob_properties(Some(BlobBlobClientGetPropertiesOptions::default()))
            .await;

        assert!(response.is_err());
        Ok(())
    }
}
