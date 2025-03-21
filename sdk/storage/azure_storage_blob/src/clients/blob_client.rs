// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::{BlockBlobClient, GeneratedBlobClient},
    models::{
        BlobClientDownloadOptions, BlobClientDownloadResult, BlobClientGetPropertiesOptions,
        BlobClientGetPropertiesResult, BlockBlobClientCommitBlockListOptions,
        BlockBlobClientCommitBlockListResult, BlockBlobClientStageBlockOptions,
        BlockBlobClientStageBlockResult, BlockBlobClientUploadOptions, BlockBlobClientUploadResult,
        BlockLookupList,
    },
    pipeline::StorageHeadersPolicy,
    BlobClientOptions, BlockBlobClientOptions,
};
use azure_core::{
    base64, credentials::TokenCredential, BearerTokenCredentialPolicy, Bytes, Policy,
    RequestContent, Response, Result, Url,
};
use std::sync::Arc;

pub struct BlobClient {
    endpoint: Url,
    credential: Arc<dyn TokenCredential>, // This will be removed in future versions, but needed for now to spin up sub-clients.
    options: BlobClientOptions, // This will be removed in future versions, but needed for now to spin up sub-clients.
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
            "2025-05-05".to_string(),
            container_name.clone(),
            blob_name.clone(),
            Some(options.clone()),
        )?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            credential,
            options,
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
        &self.client.blob
    }

    pub fn credential(&self) -> Arc<dyn TokenCredential> {
        self.credential.clone()
    }

    pub fn options(&self) -> BlobClientOptions {
        self.options.clone()
    }

    pub fn get_block_blob_client(
        &self,
        client_options: Option<BlockBlobClientOptions>,
    ) -> Result<BlockBlobClient> {
        // Take user input OR inherit from current client
        let options = client_options.or(Some(BlockBlobClientOptions {
            client_options: self.options().client_options,
        }));
        BlockBlobClient::new(
            self.endpoint().as_str(),
            self.container_name().to_string(),
            self.blob_name().to_string(),
            self.credential().clone(),
            options,
        )
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

    // For now, this is single-shot, block blob hot path only.
    pub async fn upload_blob(
        &self,
        data: RequestContent<Bytes>,
        overwrite: bool,
        content_length: u64,
        client_options: Option<BlockBlobClientOptions>,
        options: Option<BlockBlobClientUploadOptions<'_>>,
    ) -> Result<Response<BlockBlobClientUploadResult>> {
        let mut options = options.unwrap_or_default();

        // Check if they want overwrite, by default overwrite=False
        if !overwrite {
            options.if_none_match = Some(String::from("*"));
        }

        let block_blob_client = self.get_block_blob_client(client_options)?;

        let response = block_blob_client
            .upload_blob(data, content_length, Some(options))
            .await?;
        Ok(response)
    }

    pub async fn commit_block_list(
        &self,
        blocks: RequestContent<BlockLookupList>,
        client_options: Option<BlockBlobClientOptions>,
        options: Option<BlockBlobClientCommitBlockListOptions<'_>>,
    ) -> Result<Response<BlockBlobClientCommitBlockListResult>> {
        let block_blob_client = self.get_block_blob_client(client_options)?;
        let response = block_blob_client.commit_block_list(blocks, options).await?;
        Ok(response)
    }

    pub async fn stage_block(
        &self,
        block_id: &str,
        content_length: u64,
        body: RequestContent<Bytes>,
        client_options: Option<BlockBlobClientOptions>,
        options: Option<BlockBlobClientStageBlockOptions<'_>>,
    ) -> Result<Response<BlockBlobClientStageBlockResult>> {
        let block_id = base64::encode(block_id);
        let block_blob_client = self.get_block_blob_client(client_options)?;

        let response = block_blob_client
            .stage_block(&block_id, content_length, body, options)
            .await?;
        Ok(response)
    }
}
