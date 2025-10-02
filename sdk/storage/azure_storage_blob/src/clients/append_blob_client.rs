// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::AppendBlobClient as GeneratedAppendBlobClient,
    models::{
        AppendBlobClientAppendBlockFromUrlOptions, AppendBlobClientAppendBlockFromUrlResult,
        AppendBlobClientAppendBlockOptions, AppendBlobClientAppendBlockResult,
        AppendBlobClientCreateOptions, AppendBlobClientCreateResult, AppendBlobClientSealOptions,
        AppendBlobClientSealResult,
    },
    parsers::parse_url_name_components,
    pipeline::StorageHeadersPolicy,
    AppendBlobClientOptions, BlobClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        NoFormat, RequestContent, Response, Url,
    },
    Bytes, Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage Append blob, although that blob may not yet exist.
pub struct AppendBlobClient {
    pub(super) endpoint: Url,
    pub(super) client: GeneratedAppendBlobClient,
    pub(super) container_name: String,
    pub(super) blob_name: String,
}

impl AppendBlobClient {
    /// Creates a new AppendBlobClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container containing this Append blob.
    /// * `blob_name` - The name of the Append blob to interact with.
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: String,
        blob_name: String,
        credential: Arc<dyn TokenCredential>,
        options: Option<AppendBlobClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let mut url = Url::parse(endpoint)?;
        if !url.scheme().starts_with("http") {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{url} must use http(s)"),
            ));
        }

        // Build Blob URL, Url crate handles encoding only path params
        url.path_segments_mut()
            .expect("Cannot be base")
            .extend([&container_name, &blob_name]);

        let client = GeneratedAppendBlobClient::new(url.as_str(), credential, Some(options))?;
        Ok(Self {
            endpoint: client.endpoint().clone(),
            client,
            container_name,
            blob_name,
        })
    }

    /// Gets the endpoint of the Storage account this client is connected to.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Gets the container name of the Storage account this client is connected to.
    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    /// Gets the blob name of the Storage account this client is connected to.
    pub fn blob_name(&self) -> &str {
        &self.blob_name
    }

    /// Creates a new Append blob.
    ///
    /// # Arguments
    ///
    /// * `content_length` - Total length of the blob data to be uploaded.
    /// * `options` - Optional configuration for the request.
    pub async fn create(
        &self,
        options: Option<AppendBlobClientCreateOptions<'_>>,
    ) -> Result<Response<AppendBlobClientCreateResult, NoFormat>> {
        self.client.create(options).await
    }

    /// Commits a new block of data to the end of an Append blob.
    ///
    /// # Arguments
    ///
    /// * `data` - The blob data to append.
    /// * `content_length` - Total length of the blob data to be appended.
    /// * `options` - Optional configuration for the request.
    pub async fn append_block(
        &self,
        data: RequestContent<Bytes, NoFormat>,
        content_length: u64,
        options: Option<AppendBlobClientAppendBlockOptions<'_>>,
    ) -> Result<Response<AppendBlobClientAppendBlockResult, NoFormat>> {
        self.client
            .append_block(data, content_length, options)
            .await
    }

    /// Creates a new block to be committed as part of an Append blob where the contents are
    /// read from a URL.
    ///
    /// # Arguments
    ///
    /// * `source_url` - The URL of the copy source.
    /// * `content_length` - Total length of the blob data to be appended.
    /// * `options` - Optional configuration for the request.
    pub async fn append_block_from_url(
        &self,
        source_url: String,
        content_length: u64,
        options: Option<AppendBlobClientAppendBlockFromUrlOptions<'_>>,
    ) -> Result<Response<AppendBlobClientAppendBlockFromUrlResult, NoFormat>> {
        self.client
            .append_block_from_url(source_url, content_length, options)
            .await
    }

    /// Seals the Append blob to make it read-only. Seal is supported only on version 2019-12-12 or later.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn seal(
        &self,
        options: Option<AppendBlobClientSealOptions<'_>>,
    ) -> Result<Response<AppendBlobClientSealResult, NoFormat>> {
        self.client.seal(options).await
    }
}
