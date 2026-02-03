// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Re-export the generated BlobClient as the public type.
pub use crate::generated::clients::BlobClient;

use crate::{
    generated::clients::AppendBlobClient as GeneratedAppendBlobClient,
    generated::clients::BlockBlobClient as GeneratedBlockBlobClient,
    generated::clients::PageBlobClient as GeneratedPageBlobClient,
    logging::apply_storage_logging_defaults,
    models::{
        BlobClientDownloadOptions, BlobClientDownloadResult, BlockBlobClientUploadOptions,
        BlockBlobClientUploadResult, StorageErrorCode,
    },
    pipeline::StorageHeadersPolicy,
    AppendBlobClient, BlockBlobClient, PageBlobClient,
};
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    fmt::SafeDebug,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        AsyncResponse, ClientOptions, NoFormat, Pipeline, RequestContent, Response, StatusCode,
        Url, UrlExt,
    },
    tracing, Bytes, Result,
};
use std::sync::Arc;

/// Options used when creating a [`BlobClient`].
#[derive(Clone, SafeDebug)]
pub struct BlobClientOptions {
    /// Allows customization of the client.
    pub client_options: ClientOptions,
    /// Specifies the version of the operation to use for this request.
    pub version: String,
}

impl Default for BlobClientOptions {
    fn default() -> Self {
        Self {
            client_options: ClientOptions::default(),
            version: String::from("2026-04-06"),
        }
    }
}

impl BlobClient {
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

        let per_retry_policies = if let Some(token_credential) = credential {
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
            vec![auth_policy]
        } else {
            Vec::default()
        };

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            per_retry_policies,
            None,
        );

        Ok(Self {
            endpoint: blob_url,
            version: options.version,
            pipeline,
        })
    }

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

    /// Returns a new instance of AppendBlobClient.
    pub fn append_blob_client(&self) -> AppendBlobClient {
        AppendBlobClient {
            client: GeneratedAppendBlobClient {
                endpoint: self.endpoint.clone(),
                pipeline: self.pipeline.clone(),
                version: self.version.clone(),
                tracer: self.tracer.clone(),
            },
        }
    }

    /// Returns a new instance of BlockBlobClient.
    pub fn block_blob_client(&self) -> BlockBlobClient {
        BlockBlobClient {
            client: GeneratedBlockBlobClient {
                endpoint: self.endpoint.clone(),
                pipeline: self.pipeline.clone(),
                version: self.version.clone(),
                tracer: self.tracer.clone(),
            },
        }
    }

    /// Returns a new instance of PageBlobClient.
    pub fn page_blob_client(&self) -> PageBlobClient {
        PageBlobClient {
            client: GeneratedPageBlobClient {
                endpoint: self.endpoint.clone(),
                pipeline: self.pipeline.clone(),
                version: self.version.clone(),
                tracer: self.tracer.clone(),
            },
        }
    }

    // TODO: Can we just rename endpoint() on generated to this?
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
            options.if_none_match = Some(String::from("*"));
        }

        self.block_blob_client()
            .client
            .upload_internal(data, content_length, Some(options))
            .await
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
