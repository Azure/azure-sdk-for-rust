// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlobServiceClient as GeneratedBlobServiceClient,
    models::{
        BlobServiceClientGetPropertiesOptions, BlobServiceClientGetServiceStatsOptions,
        BlobServiceClientGetUserDelegationKeyOptions,
        BlobServiceClientListContainersSegmentOptions, KeyInfo, ListContainersSegmentResponse,
        StorageServiceProperties, StorageServiceStats, UserDelegationKey,
    },
    pipeline::StorageHeadersPolicy,
    BlobContainerClient, BlobServiceClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        PageIterator, RequestContent, Response, Url, XmlFormat,
    },
    Result,
};
use std::sync::Arc;

/// A client to interact with an Azure storage account.
pub struct BlobServiceClient {
    pub(super) endpoint: Url,
    pub(super) client: GeneratedBlobServiceClient,
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

        let client = GeneratedBlobServiceClient::new(endpoint, credential.clone(), Some(options))?;

        Ok(Self {
            endpoint: endpoint.parse()?,
            client,
        })
    }

    /// Returns a new instance of BlobContainerClient.
    ///
    /// # Arguments
    ///
    /// * `container_name` - The name of the container.
    pub fn blob_container_client(&self, container_name: String) -> BlobContainerClient {
        BlobContainerClient {
            endpoint: self.client.endpoint.clone(),
            client: self.client.get_blob_container_client(container_name),
        }
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
    pub async fn get_properties(
        &self,
        options: Option<BlobServiceClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<StorageServiceProperties, XmlFormat>> {
        self.client.get_properties(options).await
    }

    /// Returns a list of the containers under the specified Storage account.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub fn list_containers(
        &self,
        options: Option<BlobServiceClientListContainersSegmentOptions<'_>>,
    ) -> Result<PageIterator<Response<ListContainersSegmentResponse, XmlFormat>>> {
        self.client.list_containers_segment(options)
    }

    /// Retrieves statistics related to replication for the Blob service. It is only available on the secondary location endpoint
    /// when read-access geo-redundant replication is enabled for the Storage account.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_service_stats(
        &self,
        options: Option<BlobServiceClientGetServiceStatsOptions<'_>>,
    ) -> Result<Response<StorageServiceStats, XmlFormat>> {
        self.client.get_service_stats(options).await
    }
}
