// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::{
        clients::BlockBlobClient as GeneratedBlockBlobClient,
        models::{
            BlockBlobClientCommitBlockListResult, BlockBlobClientStageBlockFromUrlResult,
            BlockBlobClientStageBlockResult, BlockBlobClientUploadBlobFromUrlResult,
        },
    },
    logging::apply_storage_logging_defaults,
    models::{
        method_options::BlockBlobClientManagedUploadOptions, BlockBlobClientCommitBlockListOptions,
        BlockBlobClientGetBlockListOptions, BlockBlobClientStageBlockFromUrlOptions,
        BlockBlobClientStageBlockOptions, BlockBlobClientUploadBlobFromUrlOptions,
        BlockBlobClientUploadInternalOptions, BlockList, BlockListType, BlockLookupList,
    },
    partitioned_transfer::{self, PartitionedUploadBehavior},
    pipeline::StorageHeadersPolicy,
};
use async_trait::async_trait;
use azure_core::{
    credentials::TokenCredential,
    fmt::SafeDebug,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        Body, ClientOptions, NoFormat, Pipeline, RequestContent, Response, Url, XmlFormat,
    },
    tracing, Bytes, Result,
};
use futures::lock::Mutex;
use std::{num::NonZero, sync::Arc};
use uuid::Uuid;

/// Options used when creating a [`BlockBlobClient`].
#[derive(Clone, SafeDebug)]
pub struct BlockBlobClientOptions {
    /// Allows customization of the client.
    pub client_options: ClientOptions,
    /// Specifies the version of the operation to use for this request.
    pub version: String,
}

impl Default for BlockBlobClientOptions {
    fn default() -> Self {
        Self {
            client_options: ClientOptions::default(),
            version: String::from("2026-04-06"),
        }
    }
}

/// A client to interact with a specific Azure storage Block blob, although that blob may not yet exist.
pub struct BlockBlobClient {
    pub(crate) client: GeneratedBlockBlobClient,
}

impl GeneratedBlockBlobClient {
    /// Creates a new GeneratedBlockBlobClient from a block blob URL.
    ///
    /// # Arguments
    ///
    /// * `blob_url` - The full URL of the block blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.BlockBlob")]
    pub fn from_url(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlockBlobClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();
        apply_storage_logging_defaults(&mut options.client_options);

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let per_retry_policies = if let Some(token_credential) = credential {
            if !blob_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{blob_url} must use https"),
                ));
            }
            let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenAuthorizationPolicy::new(
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

    #[tracing::function("Storage.Blob.BlockBlob.managedUpload")]
    pub async fn managed_upload(
        &self,
        content: RequestContent<Bytes, NoFormat>,
        options: Option<BlockBlobClientManagedUploadOptions<'_>>,
    ) -> Result<()> {
        let options = options.unwrap_or_default();
        let parallel = options.parallel.unwrap_or(DEFAULT_PARALLEL);
        let partition_size = options.partition_size.unwrap_or(DEFAULT_PARTITION_SIZE);
        // construct exhaustively to ensure we catch new options when added
        let oneshot_options = BlockBlobClientUploadInternalOptions {
            blob_cache_control: options.blob_cache_control.clone(),
            blob_content_disposition: options.blob_content_disposition.clone(),
            blob_content_encoding: options.blob_content_encoding.clone(),
            blob_content_language: options.blob_content_language.clone(),
            blob_content_md5: options.blob_content_md5.clone(),
            blob_content_type: options.blob_content_type.clone(),
            blob_tags_string: options.blob_tags_string.clone(),
            encryption_algorithm: options.encryption_algorithm,
            encryption_key: options.encryption_key.clone(),
            encryption_key_sha256: options.encryption_key_sha256.clone(),
            encryption_scope: options.encryption_scope.clone(),
            if_match: options.if_match.clone(),
            if_modified_since: options.if_modified_since,
            if_none_match: options.if_none_match.clone(),
            if_tags: options.if_tags.clone(),
            if_unmodified_since: options.if_unmodified_since,
            immutability_policy_expiry: options.immutability_policy_expiry,
            immutability_policy_mode: options.immutability_policy_mode,
            lease_id: options.lease_id.clone(),
            legal_hold: options.legal_hold,
            metadata: options.metadata.clone(),
            method_options: options.method_options.clone(),
            structured_body_type: None,
            structured_content_length: None,
            tier: options.tier.clone(),
            timeout: options.per_request_timeout,
            transactional_content_crc64: None,
            transactional_content_md5: None,
        };
        let stage_block_options = BlockBlobClientStageBlockOptions {
            encryption_algorithm: options.encryption_algorithm,
            encryption_key: options.encryption_key.clone(),
            encryption_key_sha256: options.encryption_key_sha256.clone(),
            encryption_scope: options.encryption_scope.clone(),
            lease_id: options.lease_id.clone(),
            method_options: options.method_options.clone(),
            structured_body_type: None,
            structured_content_length: None,
            timeout: options.per_request_timeout,
            transactional_content_crc64: None,
            transactional_content_md5: None,
        };
        let commit_block_list_options = BlockBlobClientCommitBlockListOptions {
            blob_cache_control: options.blob_cache_control,
            blob_content_disposition: options.blob_content_disposition,
            blob_content_encoding: options.blob_content_encoding,
            blob_content_language: options.blob_content_language,
            blob_content_md5: options.blob_content_md5,
            blob_content_type: options.blob_content_type,
            blob_tags_string: options.blob_tags_string,
            encryption_algorithm: options.encryption_algorithm,
            encryption_key: options.encryption_key,
            encryption_key_sha256: options.encryption_key_sha256,
            encryption_scope: options.encryption_scope,
            if_match: options.if_match,
            if_modified_since: options.if_modified_since,
            if_none_match: options.if_none_match,
            if_tags: options.if_tags,
            if_unmodified_since: options.if_unmodified_since,
            immutability_policy_expiry: options.immutability_policy_expiry,
            immutability_policy_mode: options.immutability_policy_mode,
            lease_id: options.lease_id,
            legal_hold: options.legal_hold,
            metadata: options.metadata,
            method_options: options.method_options,
            tier: options.tier,
            timeout: options.per_request_timeout,
            transactional_content_crc64: None,
            transactional_content_md5: None,
        };
        partitioned_transfer::upload(
            content.into(),
            parallel,
            partition_size,
            &BlockBlobClientUploadBehavior::new(
                self,
                oneshot_options,
                stage_block_options,
                commit_block_list_options,
            ),
        )
        .await
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
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: &str,
        blob_name: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlockBlobClientOptions>,
    ) -> Result<Self> {
        let mut url = Url::parse(endpoint)?;

        {
            let mut path_segments = url.path_segments_mut().map_err(|_| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "Invalid endpoint URL: Failed to parse out path segments from provided endpoint URL.",
                )
            })?;
            path_segments.extend([container_name, blob_name]);
        }

        let client = GeneratedBlockBlobClient::from_url(url, credential, options)?;
        Ok(Self { client })
    }

    /// Creates a new BlockBlobClient from a Block blob URL.
    ///
    /// # Arguments
    ///
    /// * `blob_url` - The full URL of the Block blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn from_url(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlockBlobClientOptions>,
    ) -> Result<Self> {
        let client = GeneratedBlockBlobClient::from_url(blob_url, credential, options)?;

        Ok(Self { client })
    }

    /// Gets the URL of the resource this client is configured for.
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
    /// * `block_id` - A unique identifier for the block (up to 64 bytes). The SDK will Base64-encode this value
    ///   before sending to the service. For a given blob, the `block_id` must be the same size for each block.
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

    /// The Stage Block From URL operation creates a new block to be committed as part of a blob where the contents are read from
    /// a URL.
    ///
    /// # Arguments
    ///
    /// * `block_id` - A unique identifier for the block (up to 64 bytes). This value will be base64-encoded automatically.
    ///   For a given blob, the `block_id` must be the same size for each block.
    /// * `content_length` - The length of the request.
    /// * `source_url` - Specify a URL to the copy source.
    /// * `options` - Optional configuration for the request.
    pub async fn stage_block_from_url(
        &self,
        block_id: &[u8],
        content_length: u64,
        source_url: String,
        options: Option<BlockBlobClientStageBlockFromUrlOptions<'_>>,
    ) -> Result<Response<BlockBlobClientStageBlockFromUrlResult, NoFormat>> {
        self.client
            .stage_block_from_url(block_id, content_length, source_url, options)
            .await
    }

    /// The managed upload operation updates the content of an existing block blob. Updating an existing block blob overwrites
    /// any existing metadata on the blob. Partial updates are not supported; the content of the existing blob is
    /// overwritten with the content of the new blob. To perform a partial update of the content of a block blob, use the Put
    /// Block List operation.
    ///
    /// # Arguments
    ///
    /// * `body` - The body of the request.
    /// * `options` - Optional parameters for the request.
    pub async fn managed_upload(
        &self,
        content: RequestContent<Bytes, NoFormat>,
        options: Option<BlockBlobClientManagedUploadOptions<'_>>,
    ) -> Result<()> {
        self.client.managed_upload(content, options).await
    }
}

// unwrap evaluated at compile time
const DEFAULT_PARALLEL: NonZero<usize> = NonZero::new(4).unwrap();
const DEFAULT_PARTITION_SIZE: NonZero<usize> = NonZero::new(4 * 1024 * 1024).unwrap();

struct BlockInfo {
    offset: u64,
    block_id: Uuid,
}

struct BlockBlobClientUploadBehavior<'c, 'opt> {
    client: &'c GeneratedBlockBlobClient,
    oneshot_options: BlockBlobClientUploadInternalOptions<'opt>,
    stage_block_options: BlockBlobClientStageBlockOptions<'opt>,
    commit_block_list_options: BlockBlobClientCommitBlockListOptions<'opt>,
    blocks: Mutex<Vec<BlockInfo>>,
}

impl<'c, 'opt> BlockBlobClientUploadBehavior<'c, 'opt> {
    fn new(
        client: &'c GeneratedBlockBlobClient,
        oneshot_options: BlockBlobClientUploadInternalOptions<'opt>,
        stage_block_options: BlockBlobClientStageBlockOptions<'opt>,
        commit_block_list_options: BlockBlobClientCommitBlockListOptions<'opt>,
    ) -> Self {
        Self {
            client,
            oneshot_options,
            stage_block_options,
            commit_block_list_options,
            blocks: Mutex::new(vec![]),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl PartitionedUploadBehavior for BlockBlobClientUploadBehavior<'_, '_> {
    async fn transfer_oneshot(&self, content: Body) -> Result<()> {
        let content_len = content.len() as u64;
        self.client
            .upload_internal(
                content.into(),
                content_len,
                Some(self.oneshot_options.clone()),
            )
            .await?;
        Ok(())
    }

    async fn transfer_partition(&self, offset: usize, content: Body) -> Result<()> {
        let block_id = Uuid::new_v4();
        let content_len = content.len().try_into().unwrap();
        {
            self.blocks.lock().await.push(BlockInfo {
                offset: offset as u64,
                block_id,
            });
        }
        self.client
            .stage_block(
                block_id.as_bytes(),
                content_len,
                content.into(),
                Some(self.stage_block_options.clone()),
            )
            .await?;
        Ok(())
    }

    async fn initialize(&self, _content_len: usize) -> Result<()> {
        Ok(())
    }

    async fn finalize(&self) -> Result<()> {
        let mut blocks = self.blocks.lock().await;
        blocks.sort_by(|left, right| left.offset.cmp(&right.offset));
        let blocklist = BlockLookupList {
            latest: Some(
                blocks
                    .iter()
                    .map(|bi| bi.block_id.as_bytes().to_vec())
                    .collect(),
            ),
            ..Default::default()
        };
        self.client
            .commit_block_list(
                blocklist.try_into()?,
                Some(self.commit_block_list_options.clone()),
            )
            .await?;

        Ok(())
    }
}
