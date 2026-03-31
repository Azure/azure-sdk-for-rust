// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlockBlobClient, BlockBlobClientOptions};

use crate::{
    generated::clients::BlobClient as GeneratedBlobClient,
    generated::models::{
        BlobClientDownloadInternalOptions, BlockBlobClientCommitBlockListResultHeaders,
        BlockBlobClientUploadInternalOptions, BlockBlobClientUploadInternalResultHeaders,
        CopyStatus,
    },
    logging::apply_storage_logging_defaults,
    models::{
        http_ranges::IntoRangeHeader,
        method_options::{BlockBlobClientDownloadOptions, BlockBlobClientUploadOptions},
        BlockBlobClientCommitBlockListOptions, BlockBlobClientDownloadResult,
        BlockBlobClientStageBlockOptions, BlockBlobClientUploadResult, BlockLookupList,
    },
    partitioned_transfer::{self, PartitionedDownloadBehavior, PartitionedUploadBehavior},
    pipeline::StorageHeadersPolicy,
};
use async_trait::async_trait;
use azure_core::{
    base64,
    credentials::TokenCredential,
    http::{
        headers::HeaderName,
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        response::AsyncResponseBody,
        AsyncRawResponse, Body, ClientMethodOptions, NoFormat, Pipeline, RequestContent, Url,
    },
    time::parse_rfc7231,
    tracing, Bytes, Result, Uuid,
};
use futures::lock::Mutex;
use std::collections::HashMap;
use std::{num::NonZero, ops::Range, sync::Arc};

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

    /// Uploads content to a block blob, overwriting any existing blob by default.
    ///
    /// Updating an existing block blob overwrites any existing metadata on the blob. Use [`BlockBlobClientUploadOptions::with_if_not_exists()`] to fail instead of overwriting.
    /// To perform a partial update of the content of a block blob, use [`stage_block`](Self::stage_block) and [`commit_block_list`](Self::commit_block_list) directly.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to upload.
    /// * `options` - Optional parameters for the request.
    #[tracing::function("Storage.Blob.BlockBlob.upload")]
    pub async fn upload(
        &self,
        content: RequestContent<Bytes, NoFormat>,
        options: Option<BlockBlobClientUploadOptions<'_>>,
    ) -> Result<BlockBlobClientUploadResult> {
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
        let behavior = BlockBlobClientUploadBehavior::new(
            self,
            oneshot_options,
            stage_block_options,
            commit_block_list_options,
        );
        partitioned_transfer::upload(content.into(), parallel, partition_size, &behavior).await?;
        behavior.result.into_inner().ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "Upload completed without setting result.",
            )
        })
    }

    /// Downloads a blob and its contents from the service.
    ///
    /// This operation performs a managed (multi-part) download, splitting the blob into
    /// parallel range requests for better performance on large blobs.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    #[tracing::function("Storage.Blob.BlockBlob.download")]
    pub async fn download(
        &self,
        options: Option<BlockBlobClientDownloadOptions<'_>>,
    ) -> Result<BlockBlobClientDownloadResult> {
        let options = options.unwrap_or_default();
        let parallel = options.parallel.unwrap_or(DEFAULT_PARALLEL);
        let partition_size = options.partition_size.unwrap_or(DEFAULT_PARTITION_SIZE);
        // construct exhaustively to ensure we catch new options when added
        let get_range_options = BlobClientDownloadInternalOptions {
            encryption_algorithm: options.encryption_algorithm,
            encryption_key: options.encryption_key,
            encryption_key_sha256: options.encryption_key_sha256,
            if_match: options.if_match,
            if_modified_since: options.if_modified_since,
            if_none_match: options.if_none_match,
            if_tags: options.if_tags,
            if_unmodified_since: options.if_unmodified_since,
            lease_id: options.lease_id,
            // requires into_owned due to BlockBlobClientDownloadBehavior w/ 'static Behavior
            method_options: ClientMethodOptions {
                context: options.method_options.context.into_owned(),
            },
            range: None,
            range_get_content_crc64: options.range_get_content_crc64,
            range_get_content_md5: options.range_get_content_md5,
            snapshot: options.snapshot,
            structured_body_type: options.structured_body_type,
            timeout: options.timeout,
            version_id: options.version_id,
        };
        let generated_blob_client = GeneratedBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        };
        let behavior =
            BlockBlobClientDownloadBehavior::new(generated_blob_client, get_range_options);
        let (raw_response, body) = partitioned_transfer::download(
            options.range,
            parallel,
            partition_size,
            Arc::new(behavior),
        )
        .await?;
        let headers = raw_response.headers();
        let etag = headers.get_optional_as(&HeaderName::from_static("etag"))?;
        let last_modified = headers
            .get_optional_with(&HeaderName::from_static("last-modified"), |h| {
                parse_rfc7231(h.as_str())
            })?;
        let content_length = headers.get_optional_as(&HeaderName::from_static("content-length"))?;
        let content_type = headers.get_optional_as(&HeaderName::from_static("content-type"))?;
        let cache_control = headers.get_optional_as(&HeaderName::from_static("cache-control"))?;
        let content_disposition =
            headers.get_optional_as(&HeaderName::from_static("content-disposition"))?;
        let content_encoding =
            headers.get_optional_as(&HeaderName::from_static("content-encoding"))?;
        let content_language =
            headers.get_optional_as(&HeaderName::from_static("content-language"))?;
        let content_range = headers.get_optional_as(&HeaderName::from_static("content-range"))?;
        let content_hash = headers
            .get_optional_with(&HeaderName::from_static("content-md5"), |h| {
                base64::decode(h.as_str())
            })?;
        let content_crc64 = headers
            .get_optional_with(&HeaderName::from_static("x-ms-content-crc64"), |h| {
                base64::decode(h.as_str())
            })?;
        let blob_content_hash = headers
            .get_optional_with(&HeaderName::from_static("x-ms-blob-content-md5"), |h| {
                base64::decode(h.as_str())
            })?;
        let blob_type = headers.get_optional_as(&HeaderName::from_static("x-ms-blob-type"))?;
        let blob_sequence_number =
            headers.get_optional_as(&HeaderName::from_static("x-ms-blob-sequence-number"))?;
        let blob_committed_block_count =
            headers.get_optional_as(&HeaderName::from_static("x-ms-blob-committed-block-count"))?;
        let is_sealed = headers.get_optional_as(&HeaderName::from_static("x-ms-blob-sealed"))?;
        let is_server_encrypted =
            headers.get_optional_as(&HeaderName::from_static("x-ms-server-encrypted"))?;
        let encryption_scope =
            headers.get_optional_as(&HeaderName::from_static("x-ms-encryption-scope"))?;
        let encryption_key_sha256 =
            headers.get_optional_as(&HeaderName::from_static("x-ms-encryption-key-sha256"))?;
        let version_id = headers.get_optional_as(&HeaderName::from_static("x-ms-version-id"))?;
        let created_on = headers
            .get_optional_with(&HeaderName::from_static("x-ms-creation-time"), |h| {
                parse_rfc7231(h.as_str())
            })?;
        let last_accessed = headers
            .get_optional_with(&HeaderName::from_static("x-ms-last-access-time"), |h| {
                parse_rfc7231(h.as_str())
            })?;
        let lease_state = headers.get_optional_as(&HeaderName::from_static("x-ms-lease-state"))?;
        let lease_status =
            headers.get_optional_as(&HeaderName::from_static("x-ms-lease-status"))?;
        let lease_duration =
            headers.get_optional_as(&HeaderName::from_static("x-ms-lease-duration"))?;
        let legal_hold = headers.get_optional_as(&HeaderName::from_static("x-ms-legal-hold"))?;
        let immutability_policy_mode =
            headers.get_optional_as(&HeaderName::from_static("x-ms-immutability-policy-mode"))?;
        let immutability_policy_expires_on = headers.get_optional_with(
            &HeaderName::from_static("x-ms-immutability-policy-until-date"),
            |h| parse_rfc7231(h.as_str()),
        )?;
        let copy_completed_on = headers
            .get_optional_with(&HeaderName::from_static("x-ms-copy-completion-time"), |h| {
                parse_rfc7231(h.as_str())
            })?;
        let copy_id = headers.get_optional_as(&HeaderName::from_static("x-ms-copy-id"))?;
        let copy_progress =
            headers.get_optional_as(&HeaderName::from_static("x-ms-copy-progress"))?;
        let copy_source = headers.get_optional_as(&HeaderName::from_static("x-ms-copy-source"))?;
        let copy_status: Option<CopyStatus> =
            headers.get_optional_as(&HeaderName::from_static("x-ms-copy-status"))?;
        let copy_status_description =
            headers.get_optional_as(&HeaderName::from_static("x-ms-copy-status-description"))?;
        let object_replication_policy_id =
            headers.get_optional_as(&HeaderName::from_static("x-ms-or-policy-id"))?;
        let tag_count = headers.get_optional_as(&HeaderName::from_static("x-ms-tag-count"))?;
        let mut metadata = HashMap::new();
        let mut object_replication_rules = HashMap::new();
        const META_PREFIX: &str = "x-ms-meta-";
        const OR_PREFIX: &str = "x-ms-or-";
        for (name, value) in headers.iter() {
            let name = name.as_str();
            if name.len() > META_PREFIX.len() && name.starts_with(META_PREFIX) {
                metadata.insert(
                    name[META_PREFIX.len()..].to_owned(),
                    value.as_str().to_owned(),
                );
            } else if name.len() > OR_PREFIX.len()
                && name.starts_with(OR_PREFIX)
                && name != "x-ms-or-policy-id"
            {
                object_replication_rules.insert(
                    name[OR_PREFIX.len()..].to_owned(),
                    value.as_str().to_owned(),
                );
            }
        }
        Ok(BlockBlobClientDownloadResult {
            body: AsyncResponseBody::new(body),
            etag,
            last_modified,
            created_on,
            last_accessed,
            content_length,
            content_type,
            cache_control,
            content_disposition,
            content_encoding,
            content_language,
            content_range,
            content_hash,
            content_crc64,
            blob_content_hash,
            blob_type,
            blob_sequence_number,
            blob_committed_block_count,
            is_sealed,
            metadata,
            is_server_encrypted,
            encryption_scope,
            encryption_key_sha256,
            version_id,
            lease_state,
            lease_status,
            lease_duration,
            legal_hold,
            immutability_policy_mode,
            immutability_policy_expires_on,
            copy_completed_on,
            copy_id,
            copy_progress,
            copy_source,
            copy_status,
            copy_status_description,
            object_replication_policy_id,
            object_replication_rules,
            tag_count,
            raw_response,
        })
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
    oneshot_options: BlockBlobClientUploadInternalOptions<'opt>,
    stage_block_options: BlockBlobClientStageBlockOptions<'opt>,
    commit_block_list_options: BlockBlobClientCommitBlockListOptions<'opt>,
    blocks: Mutex<Vec<BlockInfo>>,
    result: Mutex<Option<BlockBlobClientUploadResult>>,
}

impl<'c, 'opt> BlockBlobClientUploadBehavior<'c, 'opt> {
    fn new(
        client: &'c BlockBlobClient,
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
            result: Mutex::new(None),
        }
    }
}

#[async_trait]
impl PartitionedUploadBehavior for BlockBlobClientUploadBehavior<'_, '_> {
    async fn transfer_oneshot(&self, content: Body) -> Result<()> {
        let content_len = content.len();
        let rsp = self
            .client
            .upload_internal(
                content.into(),
                content_len,
                Some(self.oneshot_options.clone()),
            )
            .await?;
        *self.result.lock().await = Some(BlockBlobClientUploadResult {
            content_md5: rsp.content_md5()?,
            content_crc64: rsp.content_crc64()?,
            encryption_key_sha256: rsp.encryption_key_sha256()?,
            encryption_scope: rsp.encryption_scope()?,
            etag: rsp.etag()?,
            is_server_encrypted: rsp.is_server_encrypted()?,
            last_modified: rsp.last_modified()?,
            version_id: rsp.version_id()?,
            raw_response: rsp.to_raw_response(),
        });
        Ok(())
    }

    async fn transfer_partition(&self, offset: usize, content: Body) -> Result<()> {
        let block_id = Uuid::new_v4();
        let content_len = content.len();
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

    async fn initialize(&self, _content_len: u64) -> Result<()> {
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
        let rsp = self
            .client
            .commit_block_list(
                blocklist.try_into()?,
                Some(self.commit_block_list_options.clone()),
            )
            .await?;
        *self.result.lock().await = Some(BlockBlobClientUploadResult {
            content_md5: rsp.content_md5()?,
            content_crc64: rsp.content_crc64()?,
            encryption_key_sha256: rsp.encryption_key_sha256()?,
            encryption_scope: rsp.encryption_scope()?,
            etag: rsp.etag()?,
            is_server_encrypted: rsp.is_server_encrypted()?,
            last_modified: rsp.last_modified()?,
            version_id: rsp.version_id()?,
            raw_response: rsp.to_raw_response(),
        });
        Ok(())
    }
}

struct BlockBlobClientDownloadBehavior<'a> {
    client: GeneratedBlobClient,
    options: BlobClientDownloadInternalOptions<'a>,
}

impl<'a> BlockBlobClientDownloadBehavior<'a> {
    fn new(client: GeneratedBlobClient, options: BlobClientDownloadInternalOptions<'a>) -> Self {
        Self { client, options }
    }
}

#[async_trait]
impl PartitionedDownloadBehavior for BlockBlobClientDownloadBehavior<'_> {
    async fn transfer_range(&self, range: Option<Range<usize>>) -> Result<AsyncRawResponse> {
        let mut opt = self.options.clone();
        opt.range = range.map(|r| r.as_range_header());
        self.client
            .download_internal(Some(opt))
            .await
            .map(AsyncRawResponse::from)
    }
}
