// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::recorded;
use azure_identity::DefaultAzureCredentialBuilder;
use azure_storage_blob::{
    clients::BlobServiceClient, models::BlobServiceClientGetPropertiesOptions, BlobClientOptions,
};
use std::{env, error::Error};

#[cfg(test)]
mod tests {
    use super::*;

    #[recorded::test(live)]
    async fn test_get_service_properties() -> Result<(), Box<dyn Error>> {
        // Setup
        let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
            .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
        let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
        let credential = DefaultAzureCredentialBuilder::default().build()?;

        // Act
        let service_client =
            BlobServiceClient::new(&endpoint, credential, Some(BlobClientOptions::default()))?;
        let response = service_client
            .get_service_properties(Some(BlobServiceClientGetPropertiesOptions::default()))
            .await?;

        // Assert
        let storage_service_properties = response.into_body().await?;
        assert!(storage_service_properties.default_service_version.is_some());
        Ok(())
    }
}
