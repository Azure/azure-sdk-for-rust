// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::PageBlobClient as GeneratedPageBlobClient,
    models::{
        format_http_range, PageBlobClientClearPagesOptions, PageBlobClientClearPagesResult,
        PageBlobClientCreateOptions, PageBlobClientCreateResult, PageBlobClientResizeOptions,
        PageBlobClientResizeResult, PageBlobClientUploadPagesOptions,
        PageBlobClientUploadPagesResult,
    },
    pipeline::StorageHeadersPolicy,
    BlobClientOptions, PageBlobClientOptions,
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

/// A client to interact with a specific Azure storage Page blob, although that blob may not yet exist.
pub struct PageBlobClient {
    pub(crate) endpoint: Url,
    pub(crate) client: GeneratedPageBlobClient,
}

impl PageBlobClient {
    /// Creates a new PageBlobClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container containing this Page blob.
    /// * `blob_name` - The name of the Page blob to interact with.
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: String,
        blob_name: String,
        credential: Arc<dyn TokenCredential>,
        options: Option<PageBlobClientOptions>,
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

        let client = GeneratedPageBlobClient::new(
            endpoint,
            credential.clone(),
            container_name.clone(),
            blob_name.clone(),
            Some(options),
        )?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            client,
        })
    }

    /// Gets the endpoint of the Storage account this client is connected to.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Gets the container name of the Storage account this client is connected to.
    pub fn container_name(&self) -> &str {
        &self.client.container_name
    }

    /// Gets the blob name of the Storage account this client is connected to.
    pub fn blob_name(&self) -> &str {
        &self.client.blob_name
    }

    /// Creates a new Page blob.
    ///
    /// # Arguments
    ///
    /// * `content_length` - The maximum size for the Page blob, up to 1TB. The page blob size must
    ///   be aligned to a 512-byte boundary.
    /// * `options` - Optional parameters for the request.
    pub async fn create(
        &self,
        content_length: u64,
        options: Option<PageBlobClientCreateOptions<'_>>,
    ) -> Result<Response<PageBlobClientCreateResult, NoFormat>> {
        self.client.create(content_length, options).await
    }

    /// Clears a range of pages.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of bytes to clear. See [`azure_storage_blob::models::format_http_range()`](crate::models::format_http_range) for help with the expected String format.
    /// * `options` - Optional parameters for the request.
    pub async fn clear_page(
        &self,
        range: String,
        options: Option<PageBlobClientClearPagesOptions<'_>>,
    ) -> Result<Response<PageBlobClientClearPagesResult, NoFormat>> {
        self.client.clear_pages(range, options).await
    }

    /// Resizes a Page blob to the specified size. If the specified value is less than
    /// the current size of the blob, then all pages above the specified value are cleared.
    ///
    /// # Arguments
    ///
    /// * `size` - Size used to resize the blob. Maximum size for a page Blob is up to 1TB. The
    ///   Page blob size must be aligned to a 512-byte boundary.
    /// * `options` - Optional parameters for the request.
    pub async fn resize(
        &self,
        size: u64,
        options: Option<PageBlobClientResizeOptions<'_>>,
    ) -> Result<Response<PageBlobClientResizeResult, NoFormat>> {
        self.client.resize(size, options).await
    }

    /// The Upload Pages operation writes a range of pages to a Page blob.
    ///
    /// # Arguments
    ///
    /// * `data` - The contents of the page.
    /// * `content_length` - Number of bytes to use for writing to a section of the blob. The
    ///   content_length specified must be a modulus of 512.
    /// * `range` - The range of the bytes to write. See [`azure_storage_blob::serialize::format_http_range()`](crate::serialize::format_http_range) for help with the expected String format.
    /// * `options` - Optional parameters for the request.
    pub async fn upload_page(
        &self,
        data: RequestContent<Bytes>,
        content_length: u64,
        range: String,
        options: Option<PageBlobClientUploadPagesOptions<'_>>,
    ) -> Result<Response<PageBlobClientUploadPagesResult, NoFormat>> {
        self.client
            .upload_pages(data, content_length, range, options)
            .await
    }
}
