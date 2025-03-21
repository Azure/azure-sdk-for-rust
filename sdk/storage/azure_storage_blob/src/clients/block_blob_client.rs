// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::GeneratedBlockBlobClient,
    models::{
        BlockBlobClientCommitBlockListOptions, BlockBlobClientCommitBlockListResult,
        BlockBlobClientStageBlockOptions, BlockBlobClientStageBlockResult,
        BlockBlobClientUploadOptions, BlockBlobClientUploadResult, BlockLookupList,
    },
    pipeline::StorageHeadersPolicy,
    BlockBlobClientOptions,
};
use azure_core::{
    credentials::TokenCredential, BearerTokenCredentialPolicy, Bytes, Policy, RequestContent,
    Response, Result, Url,
};
use std::sync::Arc;

pub struct BlockBlobClient {
    endpoint: Url,
    container_name: String,
    blob_name: String,
    credential: Arc<dyn TokenCredential>,
    client: GeneratedBlockBlobClient,
}

impl BlockBlobClient {
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

        let oauth_token_policy = BearerTokenCredentialPolicy::new(
            credential.clone(),
            ["https://storage.azure.com/.default"],
        );
        options
            .client_options
            .per_try_policies
            .push(Arc::new(oauth_token_policy) as Arc<dyn Policy>);

        let client = GeneratedBlockBlobClient::new(
            endpoint,
            credential.clone(),
            "2025-05-05".to_string(),
            container_name.clone(),
            blob_name.clone(),
            Some(options),
        )?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            container_name,
            blob_name,
            credential,
            client,
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    pub fn blob_name(&self) -> &str {
        &self.blob_name
    }

    pub fn credential(&self) -> Arc<dyn TokenCredential> {
        self.credential.clone()
    }

    // For now, this is single-shot, block blob hot path only.
    pub async fn upload_blob(
        &self,
        data: RequestContent<Bytes>,
        content_length: u64,
        options: Option<BlockBlobClientUploadOptions<'_>>,
    ) -> Result<Response<BlockBlobClientUploadResult>> {
        let response = self
            .client
            .upload(data, content_length, Some(options.unwrap_or_default()))
            .await?;
        Ok(response)
    }

    pub async fn commit_block_list(
        &self,
        blocks: RequestContent<BlockLookupList>,
        options: Option<BlockBlobClientCommitBlockListOptions<'_>>,
    ) -> Result<Response<BlockBlobClientCommitBlockListResult>> {
        let response = self.client.commit_block_list(blocks, options).await?;
        Ok(response)
    }

    pub async fn stage_block(
        &self,
        block_id: &str,
        content_length: u64,
        body: RequestContent<Bytes>,
        options: Option<BlockBlobClientStageBlockOptions<'_>>,
    ) -> Result<Response<BlockBlobClientStageBlockResult>> {
        let response = self
            .client
            .stage_block(block_id, content_length, body, options)
            .await?;
        Ok(response)
    }
}
