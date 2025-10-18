// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlockBlobClient as GeneratedBlockBlobClient,
    generated::models::{
        BlobClientDownloadResult, BlobClientGetPropertiesResult,
        BlockBlobClientCommitBlockListResult, BlockBlobClientStageBlockResult,
        BlockBlobClientUploadBlobFromUrlResult, BlockBlobClientUploadResult,
    },
    models::{
        BlobClientDeleteOptions, BlobClientDownloadOptions, BlobClientGetPropertiesOptions,
        BlobClientSetMetadataOptions, BlobClientSetPropertiesOptions, BlobClientSetTierOptions,
        BlockBlobClientCommitBlockListOptions, BlockBlobClientGetBlockListOptions,
        BlockBlobClientStageBlockOptions, BlockBlobClientUploadBlobFromUrlOptions,
        BlockBlobClientUploadOptions, BlockList, BlockListType, BlockLookupList,
    },
    pipeline::StorageHeadersPolicy,
    BlockBlobClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        NoFormat, Pipeline, RequestContent, Response, Url, XmlFormat,
    },
    tracing, Bytes, Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage Block blob, although that blob may not yet exist.
pub struct BlockBlobClient {
    pub(super) client: GeneratedBlockBlobClient,
}

impl GeneratedBlockBlobClient {
    /// Creates a new GeneratedBlockBlobClient from a block blob URL.
    ///
    /// # Arguments
    ///
    /// * `block_blob_url` - The full URL of the block blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.BlockBlob")]
    pub fn from_url(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlockBlobClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let per_retry_policies = if let Some(token_credential) = credential {
            if !blob_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{blob_url} must use http(s)"),
                ));
            }
            let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenCredentialPolicy::new(
                token_credential,
                vec!["https://storage.azure.com/.default"],
            ));
            vec![auth_policy]
        } else {
            Vec::default()
        };

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            per_retry_policies,
            None,
        );

        Ok(Self {
            endpoint: blob_url,
            version: options.version,
            pipeline,
        })
    }
}

impl BlockBlobClient {
    /// Creates a new BlockBlobClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container containing this Block blob.
    /// * `blob_name` - The name of the Block blob to interact with.
    /// * `credential` - An  optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: &str,
        blob_name: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlockBlobClientOptions>,
    ) -> Result<Self> {
        let mut url = Url::parse(endpoint)?;

        url.path_segments_mut()
            .expect("Invalid endpoint URL: Cannot append container_name and blob_name to the blob endpoint.")
            .extend([&container_name, &blob_name]);

        let client = GeneratedBlockBlobClient::from_url(url, credential, options)?;
        Ok(Self { client })
    }

    /// Creates a new BlockBlobClient from a Block blob URL.
    ///
    /// # Arguments
    ///
    /// * `block_blob_url` - The full URL of the Block blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn from_blob_url(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlockBlobClientOptions>,
    ) -> Result<Self> {
        let client = GeneratedBlockBlobClient::from_url(blob_url, credential, options)?;

        Ok(Self { client })
    }

    /// Gets the URL of the Storage account this client is connected to.
    pub fn url(&self) -> &Url {
        &self.client.endpoint
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
        body: RequestContent<Bytes, NoFormat>,
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

    /// Creates a new Block Blob where the content of the blob is read from a given URL. The default behavior is content of an existing blob is overwritten with the new blob.
    ///
    /// # Arguments
    ///
    /// * `copy_source` - A URL of up to 2 KB in length that specifies a file or blob. The value should be URL-encoded as it would appear in a request URI.
    ///   The source must either be public or must be authenticated via a shared access signature as part of the url or using the source_authorization keyword.
    ///   If the source is public, no authentication is required. Examples:
    ///   - `https://myaccount.blob.core.windows.net/mycontainer/myblob`
    ///   - `https://myaccount.blob.core.windows.net/mycontainer/myblob?snapshot=<DateTime>`
    ///   - `https://otheraccount.blob.core.windows.net/mycontainer/myblob?sastoken`
    /// * `options` - Optional configuration for the request.
    pub async fn upload_blob_from_url(
        &self,
        copy_source: String,
        options: Option<BlockBlobClientUploadBlobFromUrlOptions<'_>>,
    ) -> Result<Response<BlockBlobClientUploadBlobFromUrlResult, NoFormat>> {
        self.client.upload_blob_from_url(copy_source, options).await
    }
}
