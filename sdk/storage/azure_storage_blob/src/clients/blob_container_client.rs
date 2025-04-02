// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::{
        BlobContainerClient as GeneratedBlobContainerClient, BlobContainerClientOptions,
    },
    models::{
        BlobContainerClientCreateOptions, BlobContainerClientDeleteOptions,
        BlobContainerClientGetPropertiesOptions, BlobContainerClientGetPropertiesResult,
    },
    pipeline::StorageHeadersPolicy,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        Response, Url,
    },
    Result,
};
use std::sync::Arc;

/// A client to interact with a specified Azure storage container.
pub struct BlobContainerClient {
    endpoint: Url,
    container_name: String,
    client: GeneratedBlobContainerClient,
}

impl BlobContainerClient {
    /// Creates a new BlobContainerClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container.
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
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

    /// Gets the endpoint of the Storage account this client is connected to.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Gets the container name of the Storage account this client is connected to.
    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    /// Creates a new container under the specified account. If the container with the same name already exists, the operation fails.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn create_container(
        &self,
        options: Option<BlobContainerClientCreateOptions<'_>>,
    ) -> Result<Response<()>> {
        let response = self.client.create(options).await?;
        Ok(response)
    }

    /// Marks the specified container for deletion. The container and any blobs contained within are later deleted during garbage collection.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn delete_container(
        &self,
        options: Option<BlobContainerClientDeleteOptions<'_>>,
    ) -> Result<Response<()>> {
        let response = self.client.delete(options).await?;
        Ok(response)
    }

    /// Returns all user-defined metadata and system properties for the specified container.
    /// The data returned does not include the container's list of blobs.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_container_properties(
        &self,
        options: Option<BlobContainerClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<BlobContainerClientGetPropertiesResult>> {
        let response = self.client.get_properties(options).await?;

        Ok(response)
    }
}
