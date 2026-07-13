// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobContainerClient, BlobContainerClientOptions};

use crate::{models::StorageErrorCode, BlobClient};
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
    /// Creates a new BlobContainerClient from a container URL.
    ///
    /// # Arguments
    ///
    /// * `container_url` - The full URL of the container, for example `https://myaccount.blob.core.windows.net/mycontainer`.
    ///   The caller is responsible for percent-encoding the URL correctly; it will be used as-is.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Container")]
    pub fn new(
        container_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        // Storage endpoints must be base URLs.
        if container_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{container_url} is not a valid base URL"),
            ));
        }

        let mut options = options.unwrap_or_default();
        super::apply_client_defaults(&mut options.client_options);

        let mut per_retry_policies: Vec<Arc<dyn Policy>> = Vec::default();
        if let Some(token_credential) = credential {
            if !container_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{container_url} must use https"),
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
    use super::*;
    use azure_core::{
        http::{
            headers::Headers, pager::PagerContinuation, AsyncRawResponse, ClientOptions,
            StatusCode, Transport,
        },
        Bytes,
    };
    use azure_core_test::http::MockHttpClient;
    use futures::{FutureExt as _, TryStreamExt as _};
    use std::sync::Arc;

    const LIST_BLOBS_PAGE: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<EnumerationResults ServiceEndpoint="https://example.blob.core.windows.net/" ContainerName="container">
  <Blobs>
    <Blob>
      <Name>blob1</Name>
      <Properties>
        <BlobType>BlockBlob</BlobType>
      </Properties>
    </Blob>
  </Blobs>
  <NextMarker>page-2</NextMarker>
</EnumerationResults>"#;

    #[test]
    fn from_url_rejects_cannot_be_a_base_url() {
        let url = Url::parse("data:text/plain,hello").unwrap();
        assert!(BlobContainerClient::new(url, None, None).is_err());
    }

    #[test]
    fn from_url_accepts_http_without_credential() {
        let url = Url::parse("http://127.0.0.1:10000/devstoreaccount1/container").unwrap();
        let container = BlobContainerClient::new(url, None, None).unwrap();
        assert_eq!(
            container.blob_client("blob").url().path(),
            "/devstoreaccount1/container/blob"
        );
    }

    #[test]
    fn from_url_accepts_https_custom_hostname() {
        // CDN / Front Door / private endpoint hostnames are still https URLs.
        let url = Url::parse("https://cdn.contoso.com/container").unwrap();
        assert!(BlobContainerClient::new(url, None, None).is_ok());
    }

    #[tokio::test]
    async fn list_blobs_page_keeps_body_for_into_model() -> Result<()> {
        let mock_client = Arc::new(MockHttpClient::new(|req| {
            assert_eq!(req.url().path(), "/container");
            assert!(req
                .url()
                .query()
                .is_some_and(|query| query.contains("comp=list")));
            async move {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    Bytes::from_static(LIST_BLOBS_PAGE),
                ))
            }
            .boxed()
        }));
        let client = BlobContainerClient::new(
            Url::parse("https://example.blob.core.windows.net/container").unwrap(),
            None,
            Some(BlobContainerClientOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(mock_client)),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )?;

        let mut pages = client.list_blobs(None)?.into_pages();
        let page = pages.try_next().await?.expect("expected a page");

        assert!(matches!(
            pages.continuation(),
            Some(PagerContinuation::Token(token)) if token == "page-2"
        ));

        let page = page.into_model()?;
        assert_eq!(page.next_marker.as_deref(), Some("page-2"));
        assert_eq!(page.blob_items.len(), 1);
        assert_eq!(page.blob_items[0].name.as_deref(), Some("blob1"));

        Ok(())
    }
}
