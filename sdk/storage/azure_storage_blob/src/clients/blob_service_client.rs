// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::{BlobContainerClient, GeneratedBlobServiceClient},
    models::{BlobServiceClientGetPropertiesOptions, StorageServiceProperties},
    pipeline::StorageHeadersPolicy,
    BlobContainerClientOptions, BlobServiceClientOptions,
};
use azure_core::{
    credentials::TokenCredential, BearerTokenCredentialPolicy, Policy, Response, Result, Url,
};
use std::sync::Arc;

pub struct BlobServiceClient {
    endpoint: Url,
    credential: Arc<dyn TokenCredential>, // This will be removed in future versions, but needed for now to spin up sub-clients.
    client: GeneratedBlobServiceClient,
}

impl BlobServiceClient {
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<BlobServiceClientOptions>,
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

        let client = GeneratedBlobServiceClient::new(endpoint, credential.clone(), Some(options))?;

        Ok(Self {
            endpoint: endpoint.parse()?,
            credential,
            client,
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn credential(&self) -> Arc<dyn TokenCredential> {
        self.credential.clone()
    }

    pub fn get_blob_container_client(
        &self,
        client_options: Option<BlobContainerClientOptions>,
        container_name: &str,
    ) -> Result<BlobContainerClient> {
        BlobContainerClient::new(
            self.endpoint().as_str(),
            container_name.to_string(),
            self.credential().clone(),
            Some(client_options.unwrap_or_default()),
        )
    }

    pub async fn get_service_properties(
        &self,
        options: Option<BlobServiceClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<StorageServiceProperties>> {
        let response = self.client.get_properties(options).await?;
        Ok(response)
    }
}
