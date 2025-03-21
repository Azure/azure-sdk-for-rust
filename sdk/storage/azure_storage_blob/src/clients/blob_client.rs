// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    models::{
        BlobClientDownloadOptions, BlobClientDownloadResult, BlobClientGetPropertiesOptions,
        BlobClientGetPropertiesResult, BlockBlobClientCommitBlockListOptions,
        BlockBlobClientCommitBlockListResult, BlockBlobClientStageBlockOptions,
        BlockBlobClientStageBlockResult, BlockBlobClientUploadOptions, BlockBlobClientUploadResult,
        BlockLookupList,
    },
    pipeline::StorageHeadersPolicy,
    BlobClientOptions, GeneratedBlobClient,
};
use azure_core::{
    credentials::TokenCredential, BearerTokenCredentialPolicy, Bytes, Policy, RequestContent,
    Response, Result, Url,
};
use std::sync::Arc;

pub struct BlobClient {
    endpoint: Url,
    client: GeneratedBlobClient,
}

impl BlobClient {
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

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn container_name(&self) -> &str {
        &self.client.container_name
    }

    pub fn blob_name(&self) -> &str {
        &self.client.blob_name
    }

    pub async fn get_blob_properties(
        &self,
        options: Option<BlobClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<BlobClientGetPropertiesResult>> {
        let response = self.client.get_properties(options).await?;
        Ok(response)
    }

    pub async fn download_blob(
        &self,
        options: Option<BlobClientDownloadOptions<'_>>,
    ) -> Result<Response<BlobClientDownloadResult>> {
        let response = self.client.download(options).await?;
        Ok(response)
    }

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

    pub async fn commit_block_list(
        &self,
        blocks: RequestContent<BlockLookupList>,
        options: Option<BlockBlobClientCommitBlockListOptions<'_>>,
    ) -> Result<Response<BlockBlobClientCommitBlockListResult>> {
        let block_blob_client = self.client.get_block_blob_client();
        let response = block_blob_client.commit_block_list(blocks, options).await?;
        Ok(response)
    }

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
}
