// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::num::NonZero;

use async_trait::async_trait;
use azure_core::http::Body;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::{
    generated::BlockBlobClient,
    models::{
        BlockBlobClientCommitBlockListOptions, BlockBlobClientManagedUploadOptions,
        BlockBlobClientStageBlockOptions, BlockLookupList,
    },
    partitioned_transfer::{self, PartitionedUploadBehavior},
};

type AzureResult<T> = azure_core::Result<T>;

// unwrap evaluated at compile time
const DEFAULT_PARALLEL: NonZero<usize> = NonZero::new(4).unwrap();
const DEFAULT_PARTITION_SIZE: NonZero<usize> = NonZero::new(4 * 1024 * 1024).unwrap();

// implement this on handwritten client for now
impl crate::BlockBlobClient {
    pub async fn managed_upload(
        &self,
        body: Body,
        options: Option<BlockBlobClientManagedUploadOptions<'_>>,
    ) -> AzureResult<()> {
        self.client.managed_upload(body, options).await
    }
}

impl BlockBlobClient {
    pub async fn managed_upload(
        &self,
        body: Body,
        options: Option<BlockBlobClientManagedUploadOptions<'_>>,
    ) -> AzureResult<()> {
        let options = options.unwrap_or_default();
        let parallel = options.parallel.unwrap_or(DEFAULT_PARALLEL);
        let partition_size = options.partition_size.unwrap_or(DEFAULT_PARTITION_SIZE);
        // construct exhaustively to ensure we catch new options when added
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
            body,
            parallel,
            partition_size,
            &BlockBlobClientUploadBehavior::new(
                self,
                stage_block_options,
                commit_block_list_options,
            ),
        )
        .await
    }
}

struct BlockInfo {
    offset: u64,
    block_id: Uuid,
}

struct BlockBlobClientUploadBehavior<'c, 'opt> {
    client: &'c BlockBlobClient,
    stage_block_options: BlockBlobClientStageBlockOptions<'opt>,
    commit_block_list_options: BlockBlobClientCommitBlockListOptions<'opt>,
    blocks: Mutex<Vec<BlockInfo>>,
}

impl<'c, 'opt> BlockBlobClientUploadBehavior<'c, 'opt> {
    fn new(
        client: &'c BlockBlobClient,
        stage_block_options: BlockBlobClientStageBlockOptions<'opt>,
        commit_block_list_options: BlockBlobClientCommitBlockListOptions<'opt>,
    ) -> Self {
        Self {
            client,
            stage_block_options,
            commit_block_list_options,
            blocks: Mutex::new(vec![]),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl PartitionedUploadBehavior for BlockBlobClientUploadBehavior<'_, '_> {
    async fn transfer_oneshot(&self, content: Body) -> AzureResult<()> {
        let content_len = content.len().try_into().unwrap();
        self.client
            .upload(content.into(), content_len, None)
            .await?;
        Ok(())
    }

    async fn transfer_partition(&self, offset: usize, content: Body) -> AzureResult<()> {
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

    async fn initialize(&self, _content_len: usize) -> AzureResult<()> {
        Ok(())
    }

    async fn finalize(&self) -> AzureResult<()> {
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
