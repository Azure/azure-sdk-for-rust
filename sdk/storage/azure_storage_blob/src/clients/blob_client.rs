// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobClient, BlobClientOptions};

use crate::{
    generated::clients::BlobClient as GeneratedBlobClient,
    generated::models::BlobClientDownloadInternalOptions,
    logging::apply_storage_logging_defaults,
    models::{
        http_ranges::IntoRangeHeader, BlobClientDownloadOptions, BlobClientDownloadResult,
        BlobClientUploadOptions, BlobClientUploadResult, StorageErrorCode,
    },
    partitioned_transfer::{self, PartitionedDownloadBehavior},
    pipeline::StorageHeadersPolicy,
    AppendBlobClient, BlockBlobClient, PageBlobClient,
};
use async_trait::async_trait;
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        AsyncRawResponse, ClientMethodOptions, NoFormat, Pipeline, RequestContent, StatusCode, Url,
        UrlExt,
    },
    tracing, Bytes, Result,
};
use std::{num::NonZero, ops::Range, sync::Arc};

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
        super::apply_client_defaults(&mut options.client_options);
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

    /// Downloads a blob directly into a caller-provided buffer.
    ///
    /// Unlike [`BlobClient::download`], which allocates and returns the blob data, this method
    /// writes the content directly into `buffer`. The blob is fetched in parallel range requests and assembled in-place.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to write the blob content into. Must be large enough to hold the requested range or the entire blob.
    /// * `options` - Optional parameters for the request.
    pub async fn download_into(
        &self,
        buffer: &mut [u8],
        options: Option<BlobClientDownloadOptions<'_>>,
    ) -> Result<usize> {
        let options = options.unwrap_or_default();
        let parallel = options.parallel.unwrap_or(DEFAULT_DOWNLOAD_PARALLEL);
        let partition_size = options
            .partition_size
            .unwrap_or(DEFAULT_DOWNLOAD_PARTITION_SIZE);
        // Construct exhaustively to catch new options.
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

        let client = GeneratedBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        };
        let client = BlobClientDownloadBehavior::new(client, get_range_options);

        partitioned_transfer::download_into(
            buffer,
            options.range,
            parallel,
            partition_size,
            Arc::new(client),
        )
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

    /// Downloads a blob and its contents from the service.
    ///
    /// This operation performs a managed (multi-part) download, splitting the blob into
    /// parallel range requests for better performance on large blobs. The returned
    /// [`BlobClientDownloadResult::body`] contains the complete blob data, while
    /// [`BlobClientDownloadResult::properties`] and /// [`BlobClientDownloadResult::headers`]
    /// reflect only the initial response's metadata and properties.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    #[tracing::function("Storage.Blob.Blob.download")]
    pub async fn download(
        &self,
        options: Option<BlobClientDownloadOptions<'_>>,
    ) -> Result<BlobClientDownloadResult> {
        let options = options.unwrap_or_default();
        let parallel = options.parallel.unwrap_or(DEFAULT_DOWNLOAD_PARALLEL);
        let partition_size = options
            .partition_size
            .unwrap_or(DEFAULT_DOWNLOAD_PARTITION_SIZE);
        // Construct exhaustively to catch new options.
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
            // requires into_owned due to BlobClientDownloadBehavior w/ 'static Behavior
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
        let inner_client = GeneratedBlobClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        };
        let behavior = BlobClientDownloadBehavior::new(inner_client, get_range_options);
        let response = partitioned_transfer::download(
            options.range,
            parallel,
            partition_size,
            Arc::new(behavior),
        )
        .await?;
        BlobClientDownloadResult::from_headers(response)
    }

    /// Uploads content to a block blob, overwriting any existing blob by default.
    ///
    /// Updating an existing block blob overwrites any existing metadata on the blob. Use [`BlobClientUploadOptions::with_if_not_exists()`] to fail instead of overwriting.
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

// unwrap evaluated at compile time
const DEFAULT_DOWNLOAD_PARALLEL: NonZero<usize> = NonZero::new(4).unwrap();
const DEFAULT_DOWNLOAD_PARTITION_SIZE: NonZero<usize> = NonZero::new(4 * 1024 * 1024).unwrap();

struct BlobClientDownloadBehavior<'a> {
    client: GeneratedBlobClient,
    options: BlobClientDownloadInternalOptions<'a>,
}

impl<'a> BlobClientDownloadBehavior<'a> {
    fn new(client: GeneratedBlobClient, options: BlobClientDownloadInternalOptions<'a>) -> Self {
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
