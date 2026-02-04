// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    logging::apply_storage_logging_defaults, pipeline::StorageHeadersPolicy, BlobContainerClient,
};
use azure_core::{
    credentials::TokenCredential,
    fmt::SafeDebug,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        ClientOptions, Pipeline, Url,
    },
    tracing, Result,
};
use std::sync::Arc;

pub use crate::generated::clients::BlobServiceClient;

/// Options used when creating a [`BlobServiceClient`].
#[derive(Clone, SafeDebug)]
pub struct BlobServiceClientOptions {
    /// Allows customization of the client.
    pub client_options: ClientOptions,
    /// Specifies the version of the operation to use for this request.
    pub version: String,
}

impl Default for BlobServiceClientOptions {
    fn default() -> Self {
        Self {
            client_options: ClientOptions::default(),
            version: String::from("2026-04-06"),
        }
    }
}

impl BlobServiceClient {
    /// Creates a new BlobServiceClient from the URL of the Azure storage account.
    ///
    /// # Arguments
    ///
    /// * `blob_service_url` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Service")]
    pub fn from_url(
        blob_service_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobServiceClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();
        apply_storage_logging_defaults(&mut options.client_options);

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let per_retry_policies = if let Some(token_credential) = credential {
            if !blob_service_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{blob_service_url} must use https"),
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
            endpoint: blob_service_url,
            version: options.version,
            pipeline,
        })
    }

    /// Creates a new BlobServiceClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobServiceClientOptions>,
    ) -> Result<Self> {
        let url = Url::parse(endpoint)?;
        Self::from_url(url, credential, options)
    }

    /// Returns a new instance of BlobContainerClient.
    ///
    /// # Arguments
    ///
    /// * `container_name` - The name of the container.
    pub fn blob_container_client(&self, container_name: &str) -> BlobContainerClient {
        let mut container_url = self.url().clone();
        container_url
            .path_segments_mut()
            // This should not fail as service URL has already been validated on client construction.
            .expect("Cannot be a base URL.")
            .push(container_name);

        BlobContainerClient {
            endpoint: container_url,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Gets the URL of the resource this client is configured for.
    pub fn url(&self) -> &Url {
        &self.endpoint
    }
}
