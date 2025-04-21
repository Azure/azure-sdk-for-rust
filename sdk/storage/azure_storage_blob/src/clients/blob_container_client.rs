// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlobContainerClient as GeneratedBlobContainerClient,
    generated::models::BlobContainerClientGetPropertiesResult, pipeline::StorageHeadersPolicy,
    BlobClient, BlobClientOptions, BlobContainerClientCreateOptions,
    BlobContainerClientDeleteOptions, BlobContainerClientGetPropertiesOptions,
    BlobContainerClientOptions, BlobContainerClientSetMetadataOptions,
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
    credential: Arc<dyn TokenCredential>,
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

    /// Returns a new instance of BlobClient.
    ///
    /// # Arguments
    ///
    /// * `blob_name` - The name of the blob.
    /// * `options` - Optional configuration for the client.
    pub fn get_blob_client(
        &self,
        blob_name: String,
        options: Option<BlobClientOptions>,
    ) -> Result<BlobClient> {
        BlobClient::new(
            self.endpoint().as_str(),
            self.container_name().to_string(),
            blob_name,
            self.credential.clone(),
            options,
        )
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
        self.client.create(options).await
    }

    /// Sets user-defined metadata for the specified container as one or more name-value pairs. Each call to this operation
    /// replaces all existing metadata attached to the container. To remove all metadata from the container, call this operation with
    /// no metadata headers.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn set_metadata(
        &self,
        options: Option<BlobContainerClientSetMetadataOptions<'_>>,
    ) -> Result<Response<()>> {
        self.client.set_metadata(options).await
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
        self.client.delete(options).await
    }

    /// Returns all user-defined metadata and system properties for the specified container.
    /// The data returned does not include the container's list of blobs.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_properties(
        &self,
        options: Option<BlobContainerClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<BlobContainerClientGetPropertiesResult>> {
        self.client.get_properties(options).await
    }
}
