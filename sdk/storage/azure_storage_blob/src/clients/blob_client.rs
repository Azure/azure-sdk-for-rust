// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlobClient as GeneratedBlobClient,
    generated::models::{
        BlobClientDownloadResult, BlobClientGetPropertiesResult,
        BlockBlobClientCommitBlockListResult, BlockBlobClientStageBlockResult,
        BlockBlobClientUploadResult,
    },
    models::{
        AccessTierOptional, BlobClientDeleteOptions, BlobClientDownloadOptions,
        BlobClientGetPropertiesOptions, BlobClientSetMetadataOptions,
        BlobClientSetPropertiesOptions, BlobClientSetTierOptions,
        BlockBlobClientCommitBlockListOptions, BlockBlobClientUploadOptions, BlockList,
        BlockListType, BlockLookupList,
    },
    pipeline::StorageHeadersPolicy,
    AppendBlobClient, BlobClientOptions, BlockBlobClient, PageBlobClient,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        NoFormat, RequestContent, Response, Url, XmlFormat,
    },
    Bytes, Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage blob, although that blob may not yet exist.
pub struct BlobClient {
    pub(super) endpoint: Url,
    pub(super) client: GeneratedBlobClient,
}

impl BlobClient {
    /// Creates a new BlobClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container containing this blob.
    /// * `blob_name` - The name of the blob to interact with.
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: String,
        blob_name: String,
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

        let client = GeneratedBlobClient::new(
            endpoint,
            credential.clone(),
            container_name.clone(),
            blob_name.clone(),
            Some(options),
        )?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            client,
        })
    }

    /// Returns a new instance of AppendBlobClient.
    ///
    /// # Arguments
    ///
    pub fn append_blob_client(&self) -> AppendBlobClient {
        AppendBlobClient {
            endpoint: self.client.endpoint.clone(),
            client: self.client.get_append_blob_client(),
        }
    }

    /// Returns a new instance of BlockBlobClient.
    ///
    /// # Arguments
    ///
    pub fn block_blob_client(&self) -> BlockBlobClient {
        BlockBlobClient {
            endpoint: self.client.endpoint.clone(),
            client: self.client.get_block_blob_client(),
        }
    }

    /// Returns a new instance of PageBlobClient.
    ///
    /// # Arguments
    ///
    pub fn page_blob_client(&self) -> PageBlobClient {
        PageBlobClient {
            endpoint: self.client.endpoint.clone(),
            client: self.client.get_page_blob_client(),
        }
    }

    /// Gets the endpoint of the Storage account this client is connected to.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Gets the container name of the Storage account this client is connected to.
    pub fn container_name(&self) -> &str {
        &self.client.container_name
    }

    /// Gets the blob name of the Storage account this client is connected to.
    pub fn blob_name(&self) -> &str {
        &self.client.blob_name
    }

    /// Returns all user-defined metadata, standard HTTP properties, and system properties for the blob.
    /// The data returned does not include the content of the blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_properties(
        &self,
        options: Option<BlobClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<BlobClientGetPropertiesResult, NoFormat>> {
        self.client.get_properties(options).await
    }

    /// Sets system properties on the blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn set_properties(
        &self,
        options: Option<BlobClientSetPropertiesOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_properties(options).await
    }

    /// Downloads a blob from the service, including its metadata and properties.
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn download(
        &self,
        options: Option<BlobClientDownloadOptions<'_>>,
    ) -> Result<Response<BlobClientDownloadResult, NoFormat>> {
        self.client.download(options).await
    }

    /// Creates a new blob from a data source.
    ///
    /// # Arguments
    ///
    /// * `data` - The blob data to upload.
    /// * `overwrite` - Whether the blob to be uploaded should overwrite the current data. If True, `upload_blob` will overwrite the existing data.
    ///   If False, the operation will fail with ResourceExistsError.
    /// * `content_length` - Total length of the blob data to be uploaded.
    /// * `options` - Optional configuration for the request.
    pub async fn upload(
        &self,
        data: RequestContent<Bytes>,
        overwrite: bool,
        content_length: u64,
        options: Option<BlockBlobClientUploadOptions<'_>>,
    ) -> Result<Response<BlockBlobClientUploadResult, NoFormat>> {
        let mut options = options.unwrap_or_default();

        if !overwrite {
            options.if_none_match = Some(String::from("*"));
        }

        let block_blob_client = self.client.get_block_blob_client();

        block_blob_client
            .upload(data, content_length, Some(options))
            .await
    }

    /// Sets user-defined metadata for the specified blob as one or more name-value pairs. Each call to this operation
    /// replaces all existing metadata attached to the blob. To remove all metadata from the blob, call this operation with
    /// no metadata headers.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn set_metadata(
        &self,
        options: Option<BlobClientSetMetadataOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_metadata(options).await
    }

    /// Deletes the blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn delete(
        &self,
        options: Option<BlobClientDeleteOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.delete(options).await
    }

    /// Sets the tier on a blob. Standard tiers are only applicable for Block blobs, while Premium tiers are only applicable
    /// for Page blobs.
    ///
    /// # Arguments
    ///
    /// * `tier` - The tier to be set on the blob.
    /// * `options` - Optional configuration for the request.
    pub async fn set_tier(
        &self,
        tier: AccessTierOptional,
        options: Option<BlobClientSetTierOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_tier(tier, options).await
    }
}
