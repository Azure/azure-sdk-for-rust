// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobClient, BlobClientOptions};

use crate::{
    generated::clients::BlobClient as GeneratedBlobClient,
    logging::apply_storage_logging_defaults,
    models::{
        http_ranges::IntoRangeHeader, method_options::BlobClientManagedDownloadOptions,
        BlobClientDownloadOptions, BlobClientDownloadResult, BlockBlobClientUploadOptions,
        BlockBlobClientUploadResult, StorageErrorCode,
    },
    partitioned_transfer::{self, PartitionedDownloadBehavior, SendSlice},
    pipeline::StorageHeadersPolicy,
    AppendBlobClient, BlockBlobClient, PageBlobClient,
};
use async_trait::async_trait;
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        response::{AsyncResponse, PinnedStream},
        AsyncRawResponse, Etag, NoFormat, Pipeline, RequestContent, Response, StatusCode, Url,
        UrlExt,
    },
    tracing, Bytes, Result,
};
use std::sync::Arc;
use std::{num::NonZero, ops::Range};

impl BlobClient {
    /// Creates a new BlobClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container containing this blob.
    /// * `blob_name` - The name of the blob to interact with.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: &str,
        blob_name: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobClientOptions>,
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

    /// Creates a new BlobClient from a blob URL.
    ///
    /// # Arguments
    ///
    /// * `blob_url` - The full URL of the blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Blob")]
    pub fn from_url(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobClientOptions>,
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

    /// The managed download operation retrieves the content of an existing blob.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    pub async fn managed_download(
        &self,
        options: Option<BlobClientManagedDownloadOptions<'_>>,
    ) -> Result<PinnedStream> {
        let options = options.unwrap_or_default();
        let parallel = options.parallel.unwrap_or(DEFAULT_PARALLEL);
        let partition_size = options.partition_size.unwrap_or(DEFAULT_PARTITION_SIZE);
        // construct exhaustively to ensure we catch new options when added
        let get_range_options = BlobClientDownloadOptions {
            encryption_algorithm: options.encryption_algorithm,
            encryption_key: options.encryption_key,
            encryption_key_sha256: options.encryption_key_sha256,
            if_match: options.if_match,
            if_modified_since: options.if_modified_since,
            if_none_match: options.if_none_match,
            if_tags: options.if_tags,
            if_unmodified_since: options.if_unmodified_since,
            lease_id: options.lease_id,
            // TODO: method_options: options.method_options,
            range: None,
            range_get_content_crc64: options.range_get_content_crc64,
            range_get_content_md5: options.range_get_content_md5,
            snapshot: options.snapshot,
            structured_body_type: options.structured_body_type,
            timeout: options.timeout,
            version_id: options.version_id,
            ..Default::default()
        };

        let client = GeneratedBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        };
        let client = BlobClientDownloadBehavior::new(client, get_range_options);

        partitioned_transfer::download(options.range, parallel, partition_size, Arc::new(client))
            .await
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

    // TODO: Partitioned upload will obsolete this wrapper.
    /// Downloads a blob from the service, including its metadata and properties.
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn download(
        &self,
        options: Option<BlobClientDownloadOptions<'_>>,
    ) -> Result<AsyncResponse<BlobClientDownloadResult>> {
        self.download_internal(options).await
    }

    /// Creates a new blob from a data source.
    ///
    /// # Arguments
    ///
    /// * `data` - The blob data to upload.
    /// * `overwrite` - Whether the blob to be uploaded should overwrite the current data. If True, `upload()` will overwrite the existing data.
    ///   If False, the operation will fail with ResourceExistsError.
    /// * `content_length` - Total length of the blob data to be uploaded.
    /// * `options` - Optional configuration for the request.
    pub async fn upload(
        &self,
        data: RequestContent<Bytes, NoFormat>,
        overwrite: bool,
        content_length: u64,
        options: Option<BlockBlobClientUploadOptions<'_>>,
    ) -> Result<Response<BlockBlobClientUploadResult, NoFormat>> {
        let mut options = options.unwrap_or_default();

        if !overwrite {
            options.if_none_match = Some(Etag::from("*"));
        }

        self.block_blob_client()
            .upload_internal(data, content_length, Some(options))
            .await
    }

    /// Downloads a blob directly into a caller-provided buffer using the Azure Core pipeline.
    ///
    /// Uses a large initial partition size (default 256MB) so small/medium blobs download in a
    /// single HTTP request. Remaining data is downloaded in parallel chunks written directly to
    /// the buffer at their correct offsets, with no ordering overhead.
    ///
    /// Returns the number of bytes written to the buffer.
    pub async fn managed_download_to(
        &self,
        buffer: &mut [u8],
        options: Option<BlobClientManagedDownloadOptions<'_>>,
    ) -> Result<usize> {
        let options = options.unwrap_or_default();
        let parallel = options.parallel.unwrap_or(DEFAULT_DOWNLOAD_TO_PARALLEL);
        let initial_partition_size = options
            .initial_partition_size
            .unwrap_or(DEFAULT_INITIAL_PARTITION_SIZE);
        let partition_size = options.partition_size.unwrap_or(DEFAULT_PARTITION_SIZE);

        let get_range_options = BlobClientDownloadOptions {
            encryption_algorithm: options.encryption_algorithm,
            encryption_key: options.encryption_key,
            encryption_key_sha256: options.encryption_key_sha256,
            if_match: options.if_match,
            if_modified_since: options.if_modified_since,
            if_none_match: options.if_none_match,
            if_tags: options.if_tags,
            if_unmodified_since: options.if_unmodified_since,
            lease_id: options.lease_id,
            range: None,
            range_get_content_crc64: options.range_get_content_crc64,
            range_get_content_md5: options.range_get_content_md5,
            snapshot: options.snapshot,
            structured_body_type: options.structured_body_type,
            timeout: options.timeout,
            version_id: options.version_id,
            ..Default::default()
        };

        let client = GeneratedBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        };
        let client = BlobClientDownloadBehavior::new(client, get_range_options);

        partitioned_transfer::download_to(
            buffer,
            options.range,
            parallel,
            initial_partition_size,
            partition_size,
            Arc::new(client),
        )
        .await
    }

    /// Downloads a blob directly into a caller-provided buffer using reqwest, bypassing the
    /// Azure Core pipeline for maximum performance.
    ///
    /// The caller is responsible for acquiring a bearer token (e.g. via `credential.get_token()`)
    /// and passing it as a string. A shared `reqwest::Client` should be provided for connection
    /// pooling across calls.
    ///
    /// Uses `tokio::spawn` for true OS-thread-level parallelism, streaming response bodies
    /// directly into non-overlapping buffer sub-slices via unsafe `SendSlice`.
    ///
    /// Returns the number of bytes written to the buffer.
    pub async fn reqwest_download_to(
        &self,
        buffer: &mut [u8],
        bearer_token: &str,
        http_client: &reqwest::Client,
        options: Option<BlobClientManagedDownloadOptions<'_>>,
    ) -> Result<usize> {
        let options = options.unwrap_or_default();
        let parallel = options
            .parallel
            .unwrap_or(DEFAULT_DOWNLOAD_TO_PARALLEL)
            .get();
        let initial_partition_size = options
            .initial_partition_size
            .unwrap_or(DEFAULT_INITIAL_PARTITION_SIZE)
            .get();
        let partition_size = options
            .partition_size
            .unwrap_or(DEFAULT_PARTITION_SIZE)
            .get();

        let max_download_range = options.range.unwrap_or(0..usize::MAX);
        if max_download_range.is_empty() {
            return Ok(0);
        }

        let url: Arc<str> = Arc::from(self.url().as_str());
        let auth_header: Arc<str> = Arc::from(format!("Bearer {bearer_token}").as_str());
        let version: Arc<str> = Arc::from(self.version.as_str());

        // Initial request with large initial_partition_size.
        let initial_range_end = std::cmp::min(
            max_download_range.end,
            max_download_range
                .start
                .saturating_add(initial_partition_size),
        );
        let mut initial_response = http_client
            .get(&*url)
            .header("Authorization", &*auth_header)
            .header("x-ms-version", &*version)
            .header(
                "Range",
                format!(
                    "bytes={}-{}",
                    max_download_range.start,
                    initial_range_end - 1
                ),
            )
            .send()
            .await
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Io, e))?;

        if !initial_response.status().is_success() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::HttpResponse {
                    status: StatusCode::from(initial_response.status().as_u16()),
                    error_code: None,
                    raw_response: None,
                },
                format!(
                    "Initial download request failed with status {}",
                    initial_response.status()
                ),
            ));
        }

        // Parse Content-Range to determine total blob size and remaining ranges.
        let (remainder_start, remainder_end) = if let Some(content_range) =
            initial_response.headers().get("content-range")
        {
            let cr_str = content_range.to_str().map_err(|e| {
                azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e)
            })?;
            parse_content_range_for_remainder(cr_str, &max_download_range)?
        } else {
            (0, 0) // No Content-Range means entire blob fit in response
        };

        // Stream initial response body directly into buffer (no intermediate Bytes).
        let mut write_offset = 0usize;
        while let Some(chunk) = initial_response
            .chunk()
            .await
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Io, e))?
        {
            let end = write_offset + chunk.len();
            if end > buffer.len() {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Io,
                    format!(
                        "Buffer too small: size {} but need at least {} bytes",
                        buffer.len(),
                        end
                    ),
                ));
            }
            buffer[write_offset..end].copy_from_slice(&chunk);
            write_offset += chunk.len();
        }
        let mut total_written = write_offset;

        // Compute remaining chunk ranges.
        let ranges: Vec<Range<usize>> = (remainder_start..remainder_end)
            .step_by(partition_size)
            .map(|i| i..std::cmp::min(i.saturating_add(partition_size), remainder_end))
            .collect();

        if ranges.is_empty() {
            return Ok(total_written);
        }

        // Use raw pointer for non-overlapping SendSlice instances.
        let buffer_ptr = buffer.as_mut_ptr();
        let buffer_len = buffer.len();

        let http_client = http_client.clone(); // reqwest::Client clones cheaply (internal Arc)
        let semaphore = Arc::new(tokio::sync::Semaphore::new(parallel));
        let mut join_set = tokio::task::JoinSet::new();

        for r in ranges {
            let client = http_client.clone();
            let sem = semaphore.clone();
            let u = url.clone();
            let auth = auth_header.clone();
            let v = version.clone();
            let buf_offset = r.start - max_download_range.start;
            let chunk_max_len = r.end - r.start;

            if buf_offset + chunk_max_len > buffer_len {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Io,
                    format!(
                        "Buffer too small: size {} but download requires {} bytes",
                        buffer_len,
                        buf_offset + chunk_max_len
                    ),
                ));
            }

            // SAFETY: Each range is non-overlapping and within buffer bounds.
            // The buffer outlives all spawned tasks (we join/shutdown before returning).
            let mut send_slice = unsafe { SendSlice::from_raw(buffer_ptr.add(buf_offset), chunk_max_len) };

            join_set.spawn(async move {
                let _permit = sem.acquire().await.map_err(|e| {
                    azure_core::Error::new(azure_core::error::ErrorKind::Other, e)
                })?;

                let mut resp = client
                    .get(&*u)
                    .header("Authorization", &*auth)
                    .header("x-ms-version", &*v)
                    .header("Range", format!("bytes={}-{}", r.start, r.end - 1))
                    .send()
                    .await
                    .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Io, e))?;

                // Stream response directly into buffer slice.
                let mut written = 0usize;
                // SAFETY: No other task accesses this slice region concurrently.
                let slice = unsafe { send_slice.as_mut_slice() };
                while let Some(chunk) = resp
                    .chunk()
                    .await
                    .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Io, e))?
                {
                    slice[written..written + chunk.len()].copy_from_slice(&chunk);
                    written += chunk.len();
                }
                Ok::<usize, azure_core::Error>(written)
            });
        }

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(written)) => total_written += written,
                Ok(Err(e)) => {
                    join_set.shutdown().await;
                    return Err(e);
                }
                Err(join_err) => {
                    join_set.shutdown().await;
                    return Err(azure_core::Error::new(
                        azure_core::error::ErrorKind::Other,
                        join_err,
                    ));
                }
            }
        }

        Ok(total_written)
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

// unwrap evaluated at compile time
const DEFAULT_PARALLEL: NonZero<usize> = NonZero::new(4).unwrap();
const DEFAULT_PARTITION_SIZE: NonZero<usize> = NonZero::new(4 * 1024 * 1024).unwrap();
const DEFAULT_DOWNLOAD_TO_PARALLEL: NonZero<usize> = NonZero::new(5).unwrap();
const DEFAULT_INITIAL_PARTITION_SIZE: NonZero<usize> = NonZero::new(256 * 1024 * 1024).unwrap();

/// Parses a Content-Range header like "bytes 0-4194303/10485760" and returns
/// the (remainder_start, remainder_end) range for subsequent chunk downloads.
fn parse_content_range_for_remainder(
    cr_str: &str,
    max_download_range: &Range<usize>,
) -> Result<(usize, usize)> {
    // Format: "bytes start-end/total" where end is inclusive
    let parts = cr_str.strip_prefix("bytes ").unwrap_or(cr_str);
    let (range_part, total_part) = parts.split_once('/').ok_or_else(|| {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::DataConversion,
            format!("Invalid Content-Range header: {cr_str}"),
        )
    })?;
    let (_start_str, end_str) = range_part.split_once('-').ok_or_else(|| -> azure_core::Error {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::DataConversion,
            format!("Invalid Content-Range header: {cr_str}"),
        )
    })?;

    let received_end: usize = end_str.parse::<usize>().map_err(|e| {
        azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e)
    })?;
    let total: usize = total_part.parse::<usize>().map_err(|e| {
        azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e)
    })?;

    // received_end is inclusive, so remainder starts at received_end + 1
    let remainder_start = received_end + 1;
    let remainder_end = std::cmp::min(max_download_range.end, total);

    Ok((remainder_start, remainder_end))
}

struct BlobClientDownloadBehavior<'a> {
    client: GeneratedBlobClient,
    options: BlobClientDownloadOptions<'a>,
}
impl<'a> BlobClientDownloadBehavior<'a> {
    fn new(client: GeneratedBlobClient, options: BlobClientDownloadOptions<'a>) -> Self {
        Self { client, options }
    }
}

#[async_trait]
impl PartitionedDownloadBehavior for BlobClientDownloadBehavior<'_> {
    async fn transfer_range(&self, range: Option<Range<usize>>) -> Result<AsyncRawResponse> {
        let mut opt = self.options.clone();
        opt.range = range.map(|r| r.as_range_header());
        self.client
            .download_internal(Some(opt))
            .await
            .map(AsyncRawResponse::from)
    }
}
