// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Example service client for use in `azure_core` examples and tests.

/// A public HTTP endpoint used for transport examples and benchmarks.
pub const HTTP_ENDPOINT: &str = "https://azuresdkforcpp.azurewebsites.net";

use crate::credentials::TokenCredential;
use azure_core::{
    fmt::SafeDebug,
    http::{ClientMethodOptions, ClientOptions, Pipeline, RawResponse, Request, Url},
    Result,
};
use std::sync::Arc;

/// Options for configuring a [`TestServiceClient`].
#[derive(Clone, SafeDebug)]
pub struct TestServiceClientOptions {
    /// Options passed through to the underlying HTTP client.
    pub client_options: ClientOptions,
    /// The API version to use for requests.
    pub api_version: Option<String>,
}

impl Default for TestServiceClientOptions {
    fn default() -> Self {
        Self {
            client_options: ClientOptions::default(),
            api_version: Some("2023-10-01".to_string()),
        }
    }
}

/// A minimal example service client backed by an [`azure_core`] [`Pipeline`].
pub struct TestServiceClient {
    endpoint: Url,
    api_version: String,
    pipeline: Pipeline,
}

/// Per-call options for [`TestServiceClient::get`].
#[derive(Default, SafeDebug)]
pub struct TestServiceClientGetMethodOptions<'a> {
    /// Common per-call options.
    pub method_options: ClientMethodOptions<'a>,
}

impl TestServiceClient {
    /// Create a new `TestServiceClient`.
    pub fn new(
        endpoint: &str,
        _credential: Arc<dyn TokenCredential>,
        options: Option<TestServiceClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        let mut endpoint = Url::parse(endpoint)?;
        if !endpoint.scheme().starts_with("http") {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{endpoint} must use http(s)"),
            ));
        }
        endpoint.set_query(None);

        Ok(Self {
            endpoint,
            api_version: options.api_version.unwrap_or_default(),
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options.client_options,
                Vec::default(),
                Vec::default(),
                None,
            ),
        })
    }

    /// Returns the URL associated with this client.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Sends a GET request to `path` relative to the configured endpoint.
    pub async fn get(
        &self,
        path: &str,
        options: Option<TestServiceClientGetMethodOptions<'_>>,
    ) -> Result<RawResponse> {
        let options = options.unwrap_or_default();
        let mut url = self.endpoint.clone();
        url.set_path(path);
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);

        let mut request = Request::new(url, azure_core::http::Method::Get);

        let response = self
            .pipeline
            .send(&options.method_options.context, &mut request, None)
            .await?;
        if !response.status().is_success() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::HttpResponse {
                    status: response.status(),
                    error_code: None,
                    raw_response: None,
                },
                format!("Failed to GET {}: {}", request.url(), response.status()),
            ));
        }
        Ok(response)
    }
}
