// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::clients::{BlobClient, GeneratedBlobContainerClient};
use crate::models::{
    BlobContainerClientCreateOptions, BlobContainerClientCreateResult,
    BlobContainerClientDeleteOptions, BlobContainerClientDeleteResult,
    BlobContainerClientGetPropertiesOptions, BlobContainerClientGetPropertiesResult,
};
use crate::pipeline::StorageHeadersPolicy;
use crate::{BlobClientOptions, BlobContainerClientOptions};
use azure_core::{
    credentials::TokenCredential, BearerTokenCredentialPolicy, Policy, Response, Result, Url,
};
use std::sync::Arc;

pub struct BlobContainerClient {
    endpoint: Url,
    container_name: String,
    credential: Arc<dyn TokenCredential>, // This will be removed in future versions, but needed for now to spin up sub-clients.
    client: GeneratedBlobContainerClient,
}

impl BlobContainerClient {
    pub fn new(
        endpoint: &str,
        container_name: String,
        credential: Arc<dyn TokenCredential>,
        options: Option<BlobContainerClientOptions>,
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

        let client = GeneratedBlobContainerClient::new(
            endpoint,
            credential.clone(),
            container_name.clone(),
            Some(options),
        )?;

        Ok(Self {
            endpoint: endpoint.parse()?,
            container_name,
            credential,
            client,
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    pub fn credential(&self) -> Arc<dyn TokenCredential> {
        self.credential.clone()
    }

    pub fn get_blob_client(
        &self,
        client_options: Option<BlobClientOptions>,
        blob_name: &str,
    ) -> Result<BlobClient> {
        BlobClient::new(
            self.endpoint().as_str(),
            self.container_name().to_string(),
            blob_name.to_string(),
            self.credential().clone(),
            Some(client_options.unwrap_or_default()),
        )
    }

    pub async fn create_container(
        &self,
        options: Option<BlobContainerClientCreateOptions<'_>>,
    ) -> Result<Response<BlobContainerClientCreateResult>> {
        let response = self.client.create(options).await?;
        Ok(response)
    }

    pub async fn delete_container(
        &self,
        options: Option<BlobContainerClientDeleteOptions<'_>>,
    ) -> Result<Response<BlobContainerClientDeleteResult>> {
        let response = self.client.delete(options).await?;
        Ok(response)
    }

    pub async fn get_container_properties(
        &self,
        options: Option<BlobContainerClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<BlobContainerClientGetPropertiesResult>> {
        let response = self.client.get_properties(options).await?;

        Ok(response)
    }
}
