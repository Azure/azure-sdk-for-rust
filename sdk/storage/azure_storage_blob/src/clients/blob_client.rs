// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::GeneratedBlobClient,
    models::{
        BlobBlobClientDownloadOptions, BlobBlobClientGetPropertiesOptions,
        BlobBlockBlobClientUploadOptions, BlobProperties,
    },
    pipeline::StorageHeadersPolicy,
    BlobClientOptions,
};
use azure_core::{
    credentials::TokenCredential, BearerTokenCredentialPolicy, Bytes, Policy, RequestContent,
    Response, Result, Url,
};
use std::sync::Arc;

pub struct BlobClient {
    endpoint: Url,
    container_name: String,
    blob_name: String,
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

        let client = GeneratedBlobClient::new(endpoint, credential, Some(options))?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            container_name,
            blob_name,
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

    pub async fn get_blob_properties(
        &self,
        options: Option<BlobBlobClientGetPropertiesOptions<'_>>,
    ) -> Result<BlobProperties> {
        let response = self
            .client
            .get_blob_blob_client(self.container_name.clone(), self.blob_name.clone())
            .get_properties(options)
            .await?;

        let blob_properties: BlobProperties = response.headers().get()?;
        Ok(blob_properties)
    }

    pub async fn download_blob(
        &self,
        options: Option<BlobBlobClientDownloadOptions<'_>>,
    ) -> Result<Response> {
        let response = self
            .client
            .get_blob_blob_client(self.container_name.clone(), self.blob_name.clone())
            .download(options)
            .await?;
        Ok(response)
    }

    // For now, this is single-shot, block blob hot path only.
    pub async fn upload_blob(
        &self,
        data: RequestContent<Bytes>,
        overwrite: bool,
        content_length: i64,
        options: Option<BlobBlockBlobClientUploadOptions<'_>>,
    ) -> Result<Response<()>> {
        let mut options = options.unwrap_or_default();

        // Check if they want overwrite, by default overwrite=False
        if !overwrite {
            options.if_none_match = Some(String::from("*"));
        }

        let response = self
            .client
            .get_blob_block_blob_client(self.container_name.clone(), self.blob_name.clone())
            .upload(data, content_length, Some(options))
            .await?;
        Ok(response)
    }
}
