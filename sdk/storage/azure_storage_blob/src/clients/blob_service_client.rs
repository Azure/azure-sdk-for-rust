// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    models::{BlobServiceClientGetPropertiesOptions, StorageServiceProperties},
    pipeline::StorageHeadersPolicy,
    BlobServiceClientOptions, GeneratedBlobServiceClient,
};
use azure_core::{
    credentials::TokenCredential, BearerTokenCredentialPolicy, Policy, Response, Result, Url,
};
use std::sync::Arc;

/// A client to interact with an Azure storage account.
pub struct BlobServiceClient {
    endpoint: Url,
    client: GeneratedBlobServiceClient,
}

impl BlobServiceClient {
    /// Creates a new BlobServiceClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
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

        let client = GeneratedBlobServiceClient::new(endpoint, credential, Some(options))?;

        Ok(Self {
            endpoint: endpoint.parse()?,
            client,
        })
    }

    /// Gets the endpoint of the Storage account this client is connected to.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Gets the properties of a Storage account's Blob service, including Azure Storage Analytics.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_service_properties(
        &self,
        options: Option<BlobServiceClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<StorageServiceProperties>> {
        let response = self.client.get_properties(options).await?;
        Ok(response)
    }
}
