// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlockBlobClient, BlockBlobClientOptions};

use crate::{
    logging::apply_storage_logging_defaults,
    models::{
        method_options::BlockBlobClientManagedUploadOptions, BlockBlobClientCommitBlockListOptions,
        BlockBlobClientStageBlockOptions, BlockBlobClientUploadOptions, BlockLookupList,
    },
    partitioned_transfer::{self, PartitionedUploadBehavior},
    pipeline::StorageHeadersPolicy,
};
use async_trait::async_trait;
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        Body, NoFormat, Pipeline, RequestContent, Url,
    },
    tracing, Bytes, Result, Uuid,
};
use futures::lock::Mutex;
use std::{num::NonZero, sync::Arc};

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

        Self::from_url(url, credential, options)
    }

    /// Creates a new BlockBlobClient from a block blob URL.
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

        if let Some(token_credential) = credential {
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
            options.client_options.per_try_policies.push(auth_policy);
        }

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            Vec::default(),
            None,
        );

        Ok(Self {
            endpoint: blob_url,
            version: options.version,
            pipeline,
        })
    }

    /// Gets the URL of the blob.
    pub fn url(&self) -> &Url {
        &self.endpoint
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
        let oneshot_options = BlockBlobClientUploadOptions {
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

// unwrap evaluated at compile time
const DEFAULT_PARALLEL: NonZero<usize> = NonZero::new(4).unwrap();
const DEFAULT_PARTITION_SIZE: NonZero<usize> = NonZero::new(4 * 1024 * 1024).unwrap();

struct BlockInfo {
    offset: u64,
    block_id: Uuid,
}

struct BlockBlobClientUploadBehavior<'c, 'opt> {
    client: &'c BlockBlobClient,
    oneshot_options: BlockBlobClientUploadOptions<'opt>,
    stage_block_options: BlockBlobClientStageBlockOptions<'opt>,
    commit_block_list_options: BlockBlobClientCommitBlockListOptions<'opt>,
    blocks: Mutex<Vec<BlockInfo>>,
}

impl<'c, 'opt> BlockBlobClientUploadBehavior<'c, 'opt> {
    fn new(
        client: &'c BlockBlobClient,
        oneshot_options: BlockBlobClientUploadOptions<'opt>,
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

#[async_trait]
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
