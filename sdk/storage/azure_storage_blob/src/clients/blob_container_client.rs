// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::clients::GeneratedBlobContainerClient;
use crate::models::{
    BlobContainerClientCreateOptions, BlobContainerClientCreateResult,
    BlobContainerClientDeleteOptions, BlobContainerClientDeleteResult,
    BlobContainerClientGetPropertiesOptions, BlobContainerClientGetPropertiesResult,
};
use crate::pipeline::StorageHeadersPolicy;
use crate::BlobContainerClientOptions;
use azure_core::{
    credentials::TokenCredential, BearerTokenCredentialPolicy, Policy, Response, Result, Url,
};
use std::sync::Arc;

pub struct BlobContainerClient {
    endpoint: Url,
    container_name: String,
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
            credential,
            container_name.clone(),
            Some(options),
        )?;

        Ok(Self {
            endpoint: endpoint.parse()?,
            container_name,
            client,
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn container_name(&self) -> &str {
        &self.container_name
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
