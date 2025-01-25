// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::recorded;
use azure_identity::DefaultAzureCredentialBuilder;
use azure_storage_blob::{
    BlobClientOptions, BlobContainerClient, BlobContainerClientGetPropertiesOptions,
};
use std::{env, error::Error};

#[cfg(test)]
mod tests {

    use super::*;

    #[recorded::test(live)]
    async fn test_get_container_properties() -> Result<(), Box<dyn Error>> {
        // Setup
        let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
            .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
        let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
        let credential = DefaultAzureCredentialBuilder::default().build()?;

        // Act
        let container_client = BlobContainerClient::new(
            endpoint,
            String::from("testcontainer"),
            credential,
            Some(BlobClientOptions::default()),
        )
        .unwrap();
        let response = container_client
            .get_container_properties(Some(BlobContainerClientGetPropertiesOptions::default()))
            .await;

        // Assert
        assert!(response.is_ok());
        println!("{:?}", response.unwrap());
        Ok(())
    }

    #[recorded::test(live)]
    async fn test_get_container_properties_invalid_container() -> Result<(), Box<dyn Error>> {
        // Setup
        let storage_account_name = env::var("AZURE_STORAGE_ACCOUNT_NAME")
            .expect("Failed to get environment variable: AZURE_STORAGE_ACCOUNT_NAME");
        let endpoint = format!("https://{}.blob.core.windows.net/", storage_account_name);
        let credential = DefaultAzureCredentialBuilder::default().build()?;

        // Act
        let container_client = BlobContainerClient::new(
            endpoint,
            String::from("missingcontainer"),
            credential,
            Some(BlobClientOptions::default()),
        )
        .unwrap();
        let response = container_client
            .get_container_properties(Some(BlobContainerClientGetPropertiesOptions::default()))
            .await;

        // Assert
        assert_eq!(
            String::from("HttpResponse(NotFound, \"ContainerNotFound\")"),
            response.unwrap_err().kind().to_string()
        );

        Ok(())
    }
}
