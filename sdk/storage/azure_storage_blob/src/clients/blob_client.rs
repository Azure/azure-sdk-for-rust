// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlobClient as GeneratedBlobClient,
    generated::models::{
        BlobClientDownloadResult, BlobClientGetPropertiesResult,
        BlockBlobClientCommitBlockListResult, BlockBlobClientStageBlockResult,
        BlockBlobClientUploadResult,
    },
    models::{BlockList, BlockListType, BlockLookupList},
    pipeline::StorageHeadersPolicy,
    BlobClientDownloadOptions, BlobClientGetPropertiesOptions, BlobClientOptions,
    BlockBlobClientCommitBlockListOptions, BlockBlobClientGetBlockListOptions,
    BlockBlobClientStageBlockOptions, BlockBlobClientUploadOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        RequestContent, Response, Url,
    },
    Bytes, Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage blob, although that blob may not yet exist.
pub struct BlobClient {
    endpoint: Url,
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

        let client = GeneratedBlobClient::new(
            endpoint,
            credential.clone(),
            container_name.clone(),
            blob_name.clone(),
            Some(options.clone()),
        )?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            client,
        })
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
    pub async fn get_blob_properties(
        &self,
        options: Option<BlobClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<BlobClientGetPropertiesResult>> {
        let response = self.client.get_properties(options).await?;
        Ok(response)
    }

    /// Downloads a blob from the service, including its metadata and properties.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn download_blob(
        &self,
        options: Option<BlobClientDownloadOptions<'_>>,
    ) -> Result<Response<BlobClientDownloadResult>> {
        let response = self.client.download(options).await?;
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
        content_length: u64,
        options: Option<BlockBlobClientUploadOptions<'_>>,
    ) -> Result<Response<BlockBlobClientUploadResult>> {
        let mut options = options.unwrap_or_default();

        if !overwrite {
            options.if_none_match = Some(String::from("*"));
        }

        let block_blob_client = self.client.get_block_blob_client();

        let response = block_blob_client
            .upload(data, content_length, Some(options))
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
        options: Option<BlockBlobClientCommitBlockListOptions<'_>>,
    ) -> Result<Response<BlockBlobClientCommitBlockListResult>> {
        let block_blob_client = self.client.get_block_blob_client();
        let response = block_blob_client.commit_block_list(blocks, options).await?;
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
        block_id: Vec<u8>,
        content_length: u64,
        body: RequestContent<Bytes>,
        options: Option<BlockBlobClientStageBlockOptions<'_>>,
    ) -> Result<Response<BlockBlobClientStageBlockResult>> {
        let block_blob_client = self.client.get_block_blob_client();
        let response = block_blob_client
            .stage_block(block_id, content_length, body, options)
            .await?;
        Ok(response)
    }

    /// Retrieves the list of blocks that have been uploaded as part of a block blob.
    ///
    /// # Arguments
    ///
    /// * `list_type` - Specifies whether to return the list of committed blocks, uncommitted blocks, or both lists together.
    /// * `options` - Optional configuration for the request.
    pub async fn get_block_list(
        &self,
        list_type: BlockListType,
        options: Option<BlockBlobClientGetBlockListOptions<'_>>,
    ) -> Result<Response<BlockList>> {
        let block_blob_client = self.client.get_block_blob_client();
        let response = block_blob_client.get_block_list(list_type, options).await?;
        Ok(response)
    }
}
