// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobContainerClient, BlobContainerClientOptions};

use crate::{
    logging::apply_storage_logging_defaults, models::StorageErrorCode,
    pipeline::StorageHeadersPolicy, BlobClient,
};
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        Pipeline, StatusCode, Url,
    },
    tracing, Result,
};
use std::sync::Arc;

impl BlobContainerClient {
    /// Creates a new BlobContainerClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        let mut url = Url::parse(endpoint)?;

        {
            let mut path_segments = url.path_segments_mut().map_err(|_| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "Invalid endpoint URL: Failed to parse out path segments from provided endpoint URL.",
                )
            })?;
            path_segments.extend([container_name]);
        }

        Self::from_url(url, credential, options)
    }

    /// Creates a new BlobContainerClient from a container URL.
    ///
    /// # Arguments
    ///
    /// * `container_url` - The full URL of the container, for example `https://myaccount.blob.core.windows.net/mycontainer`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Container")]
    pub fn from_url(
        container_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();
        apply_storage_logging_defaults(&mut options.client_options);

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        if let Some(token_credential) = credential {
            if !container_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{container_url} must use https"),
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
            endpoint: container_url,
            version: options.version,
            pipeline,
        })
    }

    /// Returns a new instance of BlobClient.
    ///
    /// # Arguments
    ///
    /// * `blob_name` - The name of the blob.
    pub fn blob_client(&self, blob_name: &str) -> BlobClient {
        let mut blob_url = self.url().clone();
        blob_url
            .path_segments_mut()
            // This should not fail as container URL has already been validated on client construction.
            .expect("Invalid endpoint URL: Cannot append blob_name to the blob endpoint.")
            .extend([blob_name]);

        BlobClient {
            endpoint: blob_url,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Gets the URL of the container.
    pub fn url(&self) -> &Url {
        &self.endpoint
    }

    /// Checks if the container exists.
    ///
    /// Returns `true` if the container exists, `false` if the container does not exist, and propagates all other errors.
    pub async fn exists(&self) -> Result<bool> {
        match self.get_properties(None).await {
            Ok(_) => Ok(true),
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => match e.kind() {
                ErrorKind::HttpResponse {
                    error_code: Some(error_code),
                    ..
                } if error_code == StorageErrorCode::ContainerNotFound.as_ref() => Ok(false),
                // Propagate all other error types.
                _ => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BlobContainerClient, BlobContainerClientOptions};
    use azure_core::http::Url;
    use azure_core_test::credentials::MockCredential;
    use std::sync::Arc;

    #[test]
    fn new_requires_https_with_credential() {
        let credential = Arc::new(MockCredential);
        let err = BlobContainerClient::new(
            "http://myaccount.blob.core.windows.net/",
            "mycontainer",
            Some(credential),
            None,
        )
        .err()
        .unwrap();
        assert!(
            err.to_string().contains("must use https"),
            "Expected 'must use https' error, got: {err}"
        );
    }

    #[test]
    fn from_url_requires_https_with_credential() {
        let credential = Arc::new(MockCredential);
        let url =
            Url::parse("http://myaccount.blob.core.windows.net/mycontainer").unwrap();
        let err = BlobContainerClient::from_url(url, Some(credential), None)
            .err()
            .unwrap();
        assert!(
            err.to_string().contains("must use https"),
            "Expected 'must use https' error, got: {err}"
        );
    }

    #[test]
    fn new_allows_http_without_credential() {
        BlobContainerClient::new(
            "http://myaccount.blob.core.windows.net/",
            "mycontainer",
            None,
            None,
        )
        .unwrap();
    }

    #[test]
    fn new_allows_https_with_credential() {
        let credential = Arc::new(MockCredential);
        BlobContainerClient::new(
            "https://myaccount.blob.core.windows.net/",
            "mycontainer",
            Some(credential),
            Some(BlobContainerClientOptions::default()),
        )
        .unwrap();
    }
}
