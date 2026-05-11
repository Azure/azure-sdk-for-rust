// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobServiceClient, BlobServiceClientOptions};

use crate::{BlobClient, BlobContainerClient};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        Pipeline, Url,
    },
    tracing, Result,
};
use std::sync::Arc;

impl BlobServiceClient {
    /// Creates a new BlobServiceClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `account_url` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Service")]
    pub fn new(
        account_url: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobServiceClientOptions>,
    ) -> Result<Self> {
        let account_url = Url::parse(account_url)?;
        // Storage endpoints must be base URLs.
        if account_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{account_url} is not a valid base URL"),
            ));
        }
        let mut options = options.unwrap_or_default();
        super::apply_client_defaults(&mut options.client_options);

        let mut per_retry_policies: Vec<Arc<dyn Policy>> = Vec::default();
        if let Some(token_credential) = credential {
            if !account_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{account_url} must use https"),
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
            Vec::default(),
            per_retry_policies,
            None,
        );

        Ok(Self {
            endpoint: account_url,
            version: options.version,
            pipeline,
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
