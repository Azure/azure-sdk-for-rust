// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::PageBlobClient as GeneratedPageBlobClient,
    models::{
        PageBlobClientClearPagesOptions, PageBlobClientClearPagesResult,
        PageBlobClientCreateOptions, PageBlobClientCreateResult,
        PageBlobClientGetPageRangesOptions, PageBlobClientResizeOptions,
        PageBlobClientResizeResult, PageBlobClientSetSequenceNumberOptions,
        PageBlobClientSetSequenceNumberResult, PageBlobClientUploadPagesFromUrlOptions,
        PageBlobClientUploadPagesFromUrlResult, PageBlobClientUploadPagesOptions,
        PageBlobClientUploadPagesResult, PageList, SequenceNumberActionType,
    },
    pipeline::StorageHeadersPolicy,
    BlobClientOptions, PageBlobClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        NoFormat, RequestContent, Response, Url, XmlFormat,
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

        let client = GeneratedPageBlobClient::new(
            endpoint,
            credential,
            container_name,
            blob_name,
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
    /// * `options` - Optional configuration for the request.
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
    /// * `range` - The range of bytes to clear. See [`format_page_range()`](crate::format_page_range) for help with the expected String format.
    /// * `options` - Optional configuration for the request.
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
    /// * `options` - Optional configuration for the request.
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
    /// * `range` - The range of the bytes to write. See [`format_page_range()`](crate::format_page_range) for help with the expected String format.
    /// * `options` - Optional configuration for the request.
    pub async fn upload_page(
        &self,
        data: RequestContent<Bytes, NoFormat>,
        content_length: u64,
        range: String,
        options: Option<PageBlobClientUploadPagesOptions<'_>>,
    ) -> Result<Response<PageBlobClientUploadPagesResult, NoFormat>> {
        self.client
            .upload_pages(data, content_length, range, options)
            .await
    }

    /// Sets the blob's sequence number. The operation will fail if the specified sequence
    /// number is less than the current sequence number of the blob.
    ///
    /// # Arguments
    ///
    /// * `sequence_number_action` - This property indicates how the service should modify the blob's sequence number. See
    ///   [SequenceNumberActionType](crate::models::SequenceNumberActionType) for more information.
    /// * `options` - Optional parameters for the request.
    pub async fn set_sequence_number(
        &self,
        sequence_number_action: SequenceNumberActionType,
        options: Option<PageBlobClientSetSequenceNumberOptions<'_>>,
    ) -> Result<Response<PageBlobClientSetSequenceNumberResult, NoFormat>> {
        self.client
            .set_sequence_number(sequence_number_action, options)
            .await
    }

    /// Writes a range of pages to a Page blob where the contents are read from a URL.
    ///
    /// # Arguments
    ///
    /// * `source_url` - The URL of the copy source.
    /// * `source_range` - Range of bytes from the source to be uploaded. See [`format_page_range()`](crate::format_page_range) for help with the expected String format.
    /// * `content_length` - Total length of the blob data to be uploaded.
    /// * `range` - Range of bytes where the source data should be written on the destination Page blob. See [`format_page_range()`](crate::format_page_range) for help with the expected String format.
    /// * `options` - Optional parameters for the request.
    pub async fn upload_pages_from_url(
        &self,
        source_url: String,
        source_range: String,
        content_length: u64,
        range: String,
        options: Option<PageBlobClientUploadPagesFromUrlOptions<'_>>,
    ) -> Result<Response<PageBlobClientUploadPagesFromUrlResult, NoFormat>> {
        self.client
            .upload_pages_from_url(source_url, source_range, content_length, range, options)
            .await
    }

    /// Returns the list of valid page ranges for a Page blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn get_page_ranges(
        &self,
        options: Option<PageBlobClientGetPageRangesOptions<'_>>,
    ) -> Result<Response<PageList, XmlFormat>> {
        self.client.get_page_ranges(options).await
    }
}
