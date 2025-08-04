// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlockBlobClient as GeneratedBlockBlobClient,
    generated::models::{
        BlobClientDownloadResult, BlobClientGetPropertiesResult,
        BlockBlobClientCommitBlockListResult, BlockBlobClientStageBlockResult,
        BlockBlobClientUploadResult,
    },
    models::{
        BlobClientDeleteOptions, BlobClientDownloadOptions, BlobClientGetPropertiesOptions,
        BlobClientSetMetadataOptions, BlobClientSetPropertiesOptions, BlobClientSetTierOptions,
        BlockBlobClientCommitBlockListOptions, BlockBlobClientGetBlockListOptions,
        BlockBlobClientStageBlockOptions, BlockBlobClientUploadOptions, BlockList, BlockListType,
        BlockLookupList, StorageServiceProperties,
    },
    pipeline::StorageHeadersPolicy,
    BlobClientOptions, BlockBlobClientOptions,
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

/// A client to interact with a specific Azure storage Block blob, although that blob may not yet exist.
pub struct BlockBlobClient {
    pub(crate) endpoint: Url,
    pub(crate) client: GeneratedBlockBlobClient,
}

impl BlockBlobClient {
    /// Creates a new BlockBlobClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container containing this Block blob.
    /// * `blob_name` - The name of the Block blob to interact with.
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: String,
        blob_name: String,
        credential: Arc<dyn TokenCredential>,
        options: Option<BlockBlobClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let client = GeneratedBlockBlobClient::new(
            endpoint,
            credential,
            container_name,
            blob_name,
            Some(options),
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

    /// Writes to a blob based on blocks specified by the list of IDs and content that make up the blob.
    ///
    /// # Arguments
    ///
    /// * `blocks` - The list of Blob blocks to commit.
    /// * `options` - Optional configuration for the request.
    pub async fn commit_block_list(
        &self,
        blocks: RequestContent<BlockLookupList, XmlFormat>,
        options: Option<BlockBlobClientCommitBlockListOptions<'_>>,
    ) -> Result<Response<BlockBlobClientCommitBlockListResult, NoFormat>> {
        self.client.commit_block_list(blocks, options).await
    }

    /// Creates a new block to be later committed as part of a blob.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The unique identifier for the block. The identifier should be less than or equal to 64 bytes in size.
    ///   For a given blob, the `block_id` must be the same size for each block.
    /// * `content_length` - Total length of the blob data to be staged.
    /// * `data` - The content of the block.
    /// * `options` - Optional configuration for the request.
    pub async fn stage_block(
        &self,
        block_id: &[u8],
        content_length: u64,
        body: RequestContent<Bytes>,
        options: Option<BlockBlobClientStageBlockOptions<'_>>,
    ) -> Result<Response<BlockBlobClientStageBlockResult, NoFormat>> {
        self.client
            .stage_block(block_id, content_length, body, options)
            .await
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
    ) -> Result<Response<BlockList, XmlFormat>> {
        self.client.get_block_list(list_type, options).await
    }
}
