// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobClient, BlobClientOptions};

use crate::{
    blob_layout::{fetch_layout, LayoutCache, LayoutEndpoint, LayoutRoutingPolicy},
    generated::{
        clients::BlobClient as GeneratedBlobClient,
        models::{BlobClientDownloadInternalOptions, BlobClientListLayoutOptions},
    },
    models::{
        BlobClientDownloadIntoResult, BlobClientDownloadOptions, BlobClientDownloadResult,
        BlobClientUploadOptions, BlobClientUploadResult, BlobDownloadProperties, HttpRange,
        LayoutAwareRouting, StorageErrorCode,
    },
    partitioned_transfer::{self, PartitionedDownloadBehavior},
    AppendBlobClient, BlockBlobClient, PageBlobClient,
};
use async_trait::async_trait;
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        headers::Headers,
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        AsyncRawResponse, Etag, NoFormat, Pipeline, RequestContent, StatusCode, Url, UrlExt,
    },
    tracing, Bytes, Result,
};
use std::{
    ops::Range,
    sync::{Arc, OnceLock},
};

impl BlobClient {
    /// Creates a new BlobClient from a blob URL.
    ///
    /// # Arguments
    ///
    /// * `blob_url` - The full URL of the blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    ///   The caller is responsible for percent-encoding the URL correctly; it will be used as-is.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Blob")]
    pub fn new(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobClientOptions>,
    ) -> Result<Self> {
        // Storage endpoints must be base URLs.
        if blob_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{blob_url} is not a valid base URL"),
            ));
        }

        let mut options = options.unwrap_or_default();
        super::apply_client_defaults(&mut options.client_options);

        let mut per_retry_policies: Vec<Arc<dyn Policy>> = Vec::default();
        if let Some(token_credential) = credential {
            if !blob_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{blob_url} must use https"),
                ));
            }
            per_retry_policies.push(Arc::new(BearerTokenAuthorizationPolicy::new(
                token_credential,
                vec!["https://storage.azure.com/.default"],
            )));
        }

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            vec![Arc::new(LayoutRoutingPolicy)],
            per_retry_policies,
            None,
        );

        Ok(Self {
            endpoint: blob_url,
            version: options.version,
            pipeline,
        })
    }

    /// Returns a new instance of AppendBlobClient.
    pub fn append_blob_client(&self) -> AppendBlobClient {
        AppendBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Returns a new instance of BlockBlobClient.
    pub fn block_blob_client(&self) -> BlockBlobClient {
        BlockBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Returns a new instance of PageBlobClient.
    pub fn page_blob_client(&self) -> PageBlobClient {
        PageBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Gets the URL of the resource this client is configured for.
    pub fn url(&self) -> &Url {
        &self.endpoint
    }

    /// Creates a new BlobClient targeting a specific blob version.
    ///
    /// # Arguments
    ///
    /// * `version_id` - The version ID of the blob to target.
    pub fn with_version(&self, version_id: &str) -> Result<Self> {
        let mut versioned_endpoint = self.endpoint.clone();
        {
            let mut query_builder = versioned_endpoint.query_builder();
            query_builder.set_pair("versionid", version_id);
            query_builder.build();
        }

        Ok(Self {
            endpoint: versioned_endpoint,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        })
    }

    /// Creates a new BlobClient targeting a specific blob snapshot.
    ///
    /// # Arguments
    ///
    /// * `snapshot` - The snapshot ID of the blob to target.
    pub fn with_snapshot(&self, snapshot: &str) -> Result<Self> {
        let mut snapshot_endpoint = self.endpoint.clone();
        {
            let mut query_builder = snapshot_endpoint.query_builder();
            query_builder.set_pair("snapshot", snapshot);
            query_builder.build();
        }

        Ok(Self {
            endpoint: snapshot_endpoint,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        })
    }

    /// Downloads a blob and its contents from the service.
    ///
    /// This operation performs a managed (multi-part) download, splitting the blob into
    /// parallel range requests for better performance on large blobs. The returned
    /// [`BlobClientDownloadResult::body`] contains the complete blob data, while
    /// [`BlobClientDownloadResult::properties`] and [`BlobClientDownloadResult::headers`]
    /// reflect only the initial response's metadata and properties.
    ///
    /// If the streamed bytes of this blob are to be collected into contiguous memory,
    /// consider instead calling [`BlobClient::download_into`] with a pre-allocated buffer
    /// to avoid unnecessary copies and allocations.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    ///
    /// # Notes
    ///
    /// By default, storage clients create their HTTP transport via
    /// [`azure_core::http::new_http_client()`] with automatic decompression disabled.
    /// If you set a custom transport in [`BlobClientOptions`] without also disabling
    /// automatic decompression, partitioned downloads may not succeed.
    #[tracing::function("Storage.Blob.Blob.download")]
    pub async fn download(
        &self,
        options: Option<BlobClientDownloadOptions<'_>>,
    ) -> Result<BlobClientDownloadResult> {
        let options = options.unwrap_or_default();
        let parallel = options
            .parallel
            .unwrap_or_else(crate::partitioned_transfer::defaults::default_concurrency);
        let partition_size = options
            .partition_size
            .unwrap_or(crate::partitioned_transfer::defaults::DEFAULT_DOWNLOAD_PARTITION_SIZE);
        let inner_client = GeneratedBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        };
        let range = options.range.clone();
        let layout_aware_routing = options.layout_aware_routing;
        let behavior = BlobClientDownloadBehavior::new(
            inner_client,
            options.into(),
            layout_aware_routing,
            range.clone(),
        );
        let response =
            partitioned_transfer::download(range, parallel, partition_size, Arc::new(behavior))
                .await?;
        BlobClientDownloadResult::from_headers(response)
    }

    /// Downloads a blob and its contents from the service.
    ///
    /// This operation performs a managed (multi-part) download, splitting the blob into
    /// parallel range requests for better performance on large blobs. The downloaded bytes are
    /// written directly into the provided `buffer`.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Destination buffer to write the downloaded blob data into.
    /// * `options` - Optional configuration for the request.
    ///
    /// # Notes
    ///
    /// By default, storage clients create their HTTP transport via
    /// [`azure_core::http::new_http_client()`] with automatic decompression disabled.
    /// If you set a custom transport in [`BlobClientOptions`] without also disabling
    /// automatic decompression, partitioned downloads may not succeed.
    #[tracing::function("Storage.Blob.Blob.download_into")]
    pub async fn download_into(
        &self,
        buffer: &mut [u8],
        options: Option<BlobClientDownloadOptions<'_>>,
    ) -> Result<BlobClientDownloadIntoResult> {
        let options = options.unwrap_or_default();
        let parallel = options
            .parallel
            .unwrap_or_else(crate::partitioned_transfer::defaults::default_concurrency);
        let partition_size = options
            .partition_size
            .unwrap_or(crate::partitioned_transfer::defaults::DEFAULT_DOWNLOAD_PARTITION_SIZE);
        let inner_client = GeneratedBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        };
        let range = options.range.clone();
        let layout_aware_routing = options.layout_aware_routing;
        let behavior = BlobClientDownloadBehavior::new(
            inner_client,
            options.into(),
            layout_aware_routing,
            range.clone(),
        );
        let (_, headers, len) = partitioned_transfer::download_into(
            buffer,
            range,
            parallel,
            partition_size,
            Arc::new(behavior),
        )
        .await?;
        Ok(BlobClientDownloadIntoResult {
            len,
            properties: BlobDownloadProperties::from_headers(&headers)?,
            headers,
        })
    }

    /// Uploads content to a block blob, overwriting any existing blob by default.
    ///
    /// Updating an existing block blob overwrites any existing metadata on the blob. Use [`BlobClientUploadOptions::if_not_exists()`] to fail instead of overwriting.
    /// To perform a partial update of the content of a block blob, use [`BlockBlobClient::stage_block()`] and [`BlockBlobClient::commit_block_list()`] directly.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to upload.
    /// * `options` - Optional parameters for the request.
    pub async fn upload(
        &self,
        content: RequestContent<Bytes, NoFormat>,
        options: Option<BlobClientUploadOptions<'_>>,
    ) -> Result<BlobClientUploadResult> {
        self.block_blob_client().upload(content, options).await
    }

    /// Checks if the blob exists.
    ///
    /// Returns `true` if the blob exists, `false` if the blob does not exist, and propagates all other errors.
    pub async fn exists(&self) -> Result<bool> {
        match self.get_properties(None).await {
            Ok(_) => Ok(true),
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => match e.kind() {
                ErrorKind::HttpResponse {
                    error_code: Some(error_code),
                    ..
                } if error_code == StorageErrorCode::BlobNotFound.as_ref()
                    || error_code == StorageErrorCode::ContainerNotFound.as_ref() =>
                {
                    Ok(false)
                }
                // Propagate all other error types.
                _ => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

struct BlobClientDownloadBehavior<'a> {
    client: Arc<GeneratedBlobClient>,
    options: BlobClientDownloadInternalOptions<'a>,
    layout_aware_routing: LayoutAwareRouting,
    /// The caller-requested range, which `options.range` does not retain because it is rewritten for each partition.
    requested_range: Option<HttpRange>,
    layout_cache: OnceLock<Option<LayoutCache>>,
}

impl<'a> BlobClientDownloadBehavior<'a> {
    fn new(
        client: GeneratedBlobClient,
        options: BlobClientDownloadInternalOptions<'a>,
        layout_aware_routing: LayoutAwareRouting,
        requested_range: Option<HttpRange>,
    ) -> Self {
        Self {
            client: Arc::new(client),
            options,
            layout_aware_routing,
            requested_range,
            layout_cache: OnceLock::new(),
        }
    }

    fn layout_options(&self) -> BlobClientListLayoutOptions<'static> {
        BlobClientListLayoutOptions {
            encryption_algorithm: self.options.encryption_algorithm,
            encryption_key: self.options.encryption_key.clone(),
            encryption_key_sha256: self.options.encryption_key_sha256.clone(),
            if_match: self.options.if_match.clone(),
            if_modified_since: self.options.if_modified_since,
            if_none_match: self.options.if_none_match.clone(),
            if_tags: self.options.if_tags.clone(),
            if_unmodified_since: self.options.if_unmodified_since,
            lease_id: self.options.lease_id.clone(),
            range: self.requested_range.clone(),
            snapshot: self.options.snapshot.clone(),
            timeout: self.options.timeout,
            version_id: self.options.version_id.clone(),
            ..Default::default()
        }
    }
}

#[async_trait]
impl PartitionedDownloadBehavior for BlobClientDownloadBehavior<'_> {
    async fn transfer_range(
        &self,
        range: Option<Range<usize>>,
        etag_lock: Option<Etag>,
    ) -> Result<AsyncRawResponse> {
        let mut opt = self.options.clone();
        if let Some(Some(cache)) = self.layout_cache.get() {
            if let Some(range) = &range {
                if let Some(layout) = cache.current().await {
                    if let Some(endpoint) = layout.ideal_endpoint(range.start as i64) {
                        opt.method_options.context = opt
                            .method_options
                            .context
                            .clone()
                            .with_value(LayoutEndpoint(endpoint.to_owned()));
                    }
                }
            }
        }
        opt.range = range.map(HttpRange::from);
        if let Some(etag) = etag_lock {
            opt.if_match = Some(etag);
            opt.if_none_match = None;
            opt.if_modified_since = None;
            opt.if_unmodified_since = None;
            opt.if_tags = None;
        }
        self.client
            .download_internal(Some(opt))
            .await
            .map(AsyncRawResponse::from)
    }

    async fn prepare(&self, initial_headers: &Headers, etag_lock: Option<&Etag>) -> Result<()> {
        if matches!(self.layout_aware_routing, LayoutAwareRouting::Disabled) {
            return Ok(());
        }
        if initial_headers.get_optional_str(&"x-ms-download-hint".into()) != Some("layout") {
            let _ = self.layout_cache.set(None);
            return Ok(());
        }
        let mut layout_options = self.layout_options();
        if layout_options.if_match.is_none() {
            layout_options.if_match = etag_lock.cloned();
        }
        let context = self.options.method_options.context.clone();
        let cache = fetch_layout(&self.client, &context, &layout_options)
            .await?
            .map(|prefetch| {
                LayoutCache::new(
                    Arc::clone(&self.client),
                    layout_options,
                    Arc::new(prefetch.layout),
                )
            });
        let _ = self.layout_cache.set(cache);
        Ok(())
    }
}
