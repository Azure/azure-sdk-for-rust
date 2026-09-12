// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobServiceClient, BlobServiceClientOptions};

use crate::{BlobClient, BlobContainerClient, SessionOptions};
use azure_core::{credentials::TokenCredential, http::Url, tracing, Result};
use std::sync::Arc;

impl BlobServiceClient {
    /// Creates a new BlobServiceClient from a service URL.
    ///
    /// # Arguments
    ///
    /// * `service_url` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`.
    ///   The caller is responsible for percent-encoding the URL correctly; it will be used as-is.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Service")]
    pub fn new(
        service_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobServiceClientOptions>,
    ) -> Result<Self> {
        // Storage endpoints must be base URLs.
        if service_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{service_url} is not a valid base URL."),
            ));
        }
        let options = options.unwrap_or_default();
        let pipeline = super::build_pipeline(&service_url, credential, None, &options)?;

        Ok(Self {
            endpoint: service_url,
            pipeline,
            version: options.version,
        })
    }

    /// Creates a new BlobServiceClient that authenticates eligible blob downloads with session tokens.
    ///
    /// # Arguments
    ///
    /// * `service_url` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`.
    ///   The caller is responsible for percent-encoding the URL correctly; it will be used as-is.
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `session_options` - Configuration for session token authentication.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Service")]
    pub fn new_with_session(
        service_url: Url,
        credential: Arc<dyn TokenCredential>,
        session_options: SessionOptions,
        options: Option<BlobServiceClientOptions>,
    ) -> Result<Self> {
        // Storage endpoints must be base URLs.
        if service_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{service_url} is not a valid base URL."),
            ));
        }
        let options = options.unwrap_or_default();
        let pipeline = super::build_pipeline(
            &service_url,
            Some(credential),
            Some(&session_options),
            &options,
        )?;

        Ok(Self {
            endpoint: service_url,
            pipeline,
            version: options.version,
        })
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

    /// Returns a new instance of BlobClient.
    ///
    /// # Arguments
    ///
    /// * `container_name` - The name of the container.
    /// * `blob_name` - The name of the blob.
    pub fn blob_client(&self, container_name: &str, blob_name: &str) -> BlobClient {
        let mut blob_url = self.url().clone();
        blob_url
            .path_segments_mut()
            // This should not fail as service URL has already been validated on client construction.
            .expect("Cannot be a base URL.")
            .extend([container_name, blob_name]);

        BlobClient {
            endpoint: blob_url,
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
