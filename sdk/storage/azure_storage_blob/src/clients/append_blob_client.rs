// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::AppendBlobClient as GeneratedAppendBlobClient,
    logging::StorageLoggingExt,
    models::{
        AppendBlobClientAppendBlockFromUrlOptions, AppendBlobClientAppendBlockFromUrlResult,
        AppendBlobClientAppendBlockOptions, AppendBlobClientAppendBlockResult,
        AppendBlobClientCreateOptions, AppendBlobClientCreateResult, AppendBlobClientSealOptions,
        AppendBlobClientSealResult,
    },
    pipeline::StorageHeadersPolicy,
    AppendBlobClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        NoFormat, Pipeline, RequestContent, Response, Url,
    },
    tracing, Bytes, Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage Append blob, although that blob may not yet exist.
pub struct AppendBlobClient {
    pub(super) client: GeneratedAppendBlobClient,
}

impl GeneratedAppendBlobClient {
    /// Creates a new GeneratedAppendBlobClient from a blob URL.
    ///
    /// # Arguments
    ///
    /// * `blob_url` - The full URL of the Append blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.AppendBlob")]
    pub fn from_url(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<AppendBlobClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();
        options.client_options.apply_storage_logging_defaults();

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
}

impl AppendBlobClient {
    /// Creates a new AppendBlobClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container containing this Append blob.
    /// * `blob_name` - The name of the Append blob to interact with.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: &str,
        blob_name: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<AppendBlobClientOptions>,
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

        let client = GeneratedAppendBlobClient::from_url(url, credential, options)?;
        Ok(Self { client })
    }

    /// Creates a new AppendBlobClient from a blob URL.
    ///
    /// # Arguments
    ///
    /// * `blob_url` - The full URL of the Append blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn from_url(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<AppendBlobClientOptions>,
    ) -> Result<Self> {
        let client = GeneratedAppendBlobClient::from_url(blob_url, credential, options)?;

        Ok(Self { client })
    }

    /// Gets the URL of the resource this client is configured for.
    pub fn url(&self) -> &Url {
        &self.client.endpoint
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
