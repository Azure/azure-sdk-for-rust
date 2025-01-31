// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::blob_client::BlobClientOptions;
use crate::generated::clients::blob_client::BlobClient as GeneratedBlobClient;
use crate::models::ContainerProperties;
use crate::pipeline::StorageHeadersPolicy;
use crate::BlobContainerClientGetPropertiesOptions;
use azure_core::{credentials::TokenCredential, BearerTokenCredentialPolicy, Policy, Result};
use std::sync::Arc;
pub struct BlobContainerClient {
    pub endpoint: String,
    pub container_name: String,
    client: GeneratedBlobClient,
}

impl BlobContainerClient {
    pub fn new(
        endpoint: String,
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

        let client = GeneratedBlobClient::new(&endpoint, credential, Some(options))?;

        Ok(Self {
            endpoint,
            container_name,
            client,
        })
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

        Ok(ContainerProperties::build_from_response_headers(
            response.headers(),
        ))
    }
}
