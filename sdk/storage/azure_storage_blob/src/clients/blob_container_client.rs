// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::clients::GeneratedBlobClient;
use crate::models::{
    BlobContainerClientCreateOptions, BlobContainerClientDeleteOptions,
    BlobContainerClientGetPropertiesOptions, ContainerProperties,
};
use crate::pipeline::StorageHeadersPolicy;
use crate::BlobClientOptions;
use azure_core::{
    credentials::TokenCredential, BearerTokenCredentialPolicy, Policy, Response, Result, Url,
};
use std::sync::Arc;
pub struct BlobContainerClient {
    endpoint: Url,
    container_name: String,
    client: GeneratedBlobClient,
}

impl BlobContainerClient {
    pub fn new(
        endpoint: &str,
        container_name: String,
        credential: Arc<dyn TokenCredential>,
        options: Option<BlobClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let oauth_token_policy = BearerTokenCredentialPolicy::new(
            credential.clone(),
            ["https://storage.azure.com/.default"],
        );
        options
            .client_options
            .per_try_policies
            .push(Arc::new(oauth_token_policy) as Arc<dyn Policy>);

        let client = GeneratedBlobClient::new(endpoint, credential, Some(options))?;

        Ok(Self {
            endpoint: endpoint.parse()?,
            container_name,
            client,
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn container_name(&self) -> &String {
        &self.container_name
    }

    pub async fn create_container(
        &self,
        options: Option<BlobContainerClientCreateOptions<'_>>,
    ) -> Result<Response<()>> {
        let response = self
            .client
            .get_blob_container_client(self.container_name.clone())
            .create(options)
            .await?;
        Ok(response)
    }

    pub async fn delete_container(
        &self,
        options: Option<BlobContainerClientDeleteOptions<'_>>,
    ) -> Result<Response<()>> {
        let response = self
            .client
            .get_blob_container_client(self.container_name.clone())
            .delete(options)
            .await?;
        Ok(response)
    }

    pub async fn get_container_properties(
        &self,
        options: Option<BlobContainerClientGetPropertiesOptions<'_>>,
    ) -> Result<ContainerProperties> {
        let response = self
            .client
            .get_blob_container_client(self.container_name.clone())
            .get_properties(options)
            .await?;

        let container_properties: Option<ContainerProperties> =
            response.headers().get_optional()?;
        Ok(container_properties.unwrap())
    }
}
