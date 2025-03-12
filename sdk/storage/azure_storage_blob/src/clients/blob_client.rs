// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlobClient as GeneratedBlobClient,
    models::{
        BlobBlobClientDownloadOptions, BlobBlobClientGetPropertiesOptions,
        BlobBlockBlobClientCommitBlockListOptions, BlobBlockBlobClientStageBlockOptions,
        BlobBlockBlobClientUploadOptions, BlobProperties, BlockLookupList,
    },
    pipeline::StorageHeadersPolicy,
    BlobClientOptions,
};
use azure_core::{
    base64, credentials::TokenCredential, BearerTokenCredentialPolicy, Bytes, Policy,
    RequestContent, Response, Result, Url,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage blob, although that blob may not yet exist.
pub struct BlobClient {
    endpoint: Url,
    container_name: String,
    blob_name: String,
    client: GeneratedBlobClient,
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

        let client = GeneratedBlobClient::new(endpoint, credential, Some(options))?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            container_name,
            blob_name,
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

    /// Gets the blob name of the Storage account this client is connected to.
    pub fn blob_name(&self) -> &str {
        &self.blob_name
    }

    /// Returns all user-defined metadata, standard HTTP properties, and system properties for the blob.
    /// The data returned does not include the content of the blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_blob_properties(
        &self,
        options: Option<BlobBlobClientGetPropertiesOptions<'_>>,
    ) -> Result<BlobProperties> {
        let response = self
            .client
            .get_blob_blob_client(self.container_name.clone(), self.blob_name.clone())
            .get_properties(options)
            .await?;

        let blob_properties: BlobProperties = response.headers().get()?;
        Ok(blob_properties)
    }

    /// Downloads a blob from the service, including its metadata and properties.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn download_blob(
        &self,
        options: Option<BlobBlobClientDownloadOptions<'_>>,
    ) -> Result<Response> {
        let response = self
            .client
            .get_blob_blob_client(self.container_name.clone(), self.blob_name.clone())
            .download(options)
            .await?;
        Ok(response)
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
    pub async fn upload_blob(
        &self,
        data: RequestContent<Bytes>,
        overwrite: bool,
        content_length: i64,
        options: Option<BlobBlockBlobClientUploadOptions<'_>>,
    ) -> Result<Response<()>> {
        let mut options = options.unwrap_or_default();

        if !overwrite {
            options.if_none_match = Some(String::from("*"));
        }

        let response = self
            .client
            .get_blob_block_blob_client(self.container_name.clone(), self.blob_name.clone())
            .upload(data, content_length, Some(options))
            .await?;
        Ok(response)
    }

    /// Creates a new block to be later committed as part of a blob.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The unique identifier for the block. The identifier should be less than or equal to 64 bytes in size.
    ///   For a given blob, the `block_id` must be the same size for each block.
    /// * `content_length` - Total length of the blob data to be staged.
    /// * `data` - The content of the blob.
    /// * `options` - Optional configuration for the request.
    pub async fn stage_block(
        &self,
        block_id: &str,
        content_length: i64,
        data: RequestContent<Bytes>,
        options: Option<BlobBlockBlobClientStageBlockOptions<'_>>,
    ) -> Result<Response<()>> {
        let block_id = base64::encode(block_id);
        let response = self
            .client
            .get_blob_block_blob_client(self.container_name.clone(), self.blob_name.clone())
            .stage_block(&block_id, content_length, data, options)
            .await?;
        Ok(response)
    }

    /// Writes to a blob based on blocks specified by the list of IDs and content that make up the blob.
    ///
    /// # Arguments
    ///
    /// * `blocks` - The list of Block Blobs to commit.
    /// * `options` - Optional configuration for the request.
    pub async fn commit_block_list(
        &self,
        blocks: RequestContent<BlockLookupList>,
        options: Option<BlobBlockBlobClientCommitBlockListOptions<'_>>,
    ) -> Result<Response<()>> {
        let response = self
            .client
            .get_blob_block_blob_client(self.container_name.clone(), self.blob_name.clone())
            .commit_block_list(blocks, options)
            .await?;
        Ok(response)
    }
}
