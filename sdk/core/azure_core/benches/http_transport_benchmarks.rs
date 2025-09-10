// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#[cfg_attr(target_os = "macos", allow(unused_imports))]
use azure_core::{
    credentials::TokenCredential,
    fmt::SafeDebug,
    http::{
        BufResponse, ClientMethodOptions, ClientOptions, HttpClient, Method, Pipeline, Request,
        TransportOptions, Url,
    },
    Result,
};
#[cfg_attr(target_os = "macos", allow(unused_imports))]
use azure_identity::DeveloperToolsCredential;
use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::Arc;

#[derive(Clone, SafeDebug)]
pub struct TestServiceClientOptions {
    pub client_options: ClientOptions,
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

pub struct TestServiceClient {
    endpoint: Url,
    api_version: String,
    pipeline: Pipeline,
}

#[derive(Default, SafeDebug)]
pub struct TestServiceClientGetMethodOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

impl TestServiceClient {
    pub fn new(
        endpoint: &str,
        _credential: Arc<dyn TokenCredential>,
        options: Option<TestServiceClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        let mut endpoint = Url::parse(endpoint)?;
        if !endpoint.scheme().starts_with("http") {
            return Err(azure_core::Error::message(
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

    /// Returns the Url associated with this client.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Returns the result of a Get verb against the configured endpoint with the specified path.
    ///
    /// This method demonstrates a service client which does not have per-method spans but which will create
    /// HTTP client spans if the `InstrumentationOptions` are configured in the client options.
    ///
    pub async fn get(
        &self,
        path: &str,
        options: Option<TestServiceClientGetMethodOptions<'_>>,
    ) -> Result<BufResponse> {
        let options = options.unwrap_or_default();
        let mut url = self.endpoint.clone();
        url.set_path(path);
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);

        let mut request = Request::new(url, azure_core::http::Method::Get);

        let response = self
            .pipeline
            .send(&options.method_options.context, &mut request)
            .await?;
        if !response.status().is_success() {
            return Err(azure_core::Error::message(
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

pub fn new_reqwest_client_disable_connection_pool() -> Arc<dyn HttpClient> {
    let client = ::reqwest::ClientBuilder::new()
        .pool_max_idle_per_host(0)
        .build()
        .expect("failed to build `reqwest` client");

    Arc::new(client)
}

pub fn new_default_reqwest_client() -> Arc<dyn HttpClient> {
    let client = ::reqwest::Client::new();

    Arc::new(client)
}

#[cfg_attr(target_os = "macos", allow(unused_variables))]
pub fn simple_http_transport_test(c: &mut Criterion) {
    #[cfg(target_os = "macos")]
    return;

    #[cfg(not(target_os = "macos"))]
    {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let endpoint = "https://azuresdkforcpp.azurewebsites.net";
        let credential = DeveloperToolsCredential::new(None).unwrap();
        let options = TestServiceClientOptions::default();

        let client = TestServiceClient::new(endpoint, credential, Some(options)).unwrap();

        // Benchmark GET and POST requests
        c.bench_function("default_http_pipeline_test", |b| {
            b.to_async(&rt).iter(|| async {
                let response = client.get("get", None).await;
                assert!(response.is_ok());
                let response = response.unwrap();
                assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
            });
        });
    }
}

#[cfg_attr(target_os = "macos", allow(unused_variables))]
pub fn disable_pooling_http_transport_test(c: &mut Criterion) {
    #[cfg(target_os = "macos")]
    return;

    #[cfg(not(target_os = "macos"))]
    {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let endpoint = "https://azuresdkforcpp.azurewebsites.net";
        let credential = DeveloperToolsCredential::new(None).unwrap();
        let transport = new_reqwest_client_disable_connection_pool();
        let options = TestServiceClientOptions {
            client_options: ClientOptions {
                transport: Some(TransportOptions::new(transport)),
                ..Default::default()
            },
            ..Default::default()
        };

        let client = TestServiceClient::new(endpoint, credential, Some(options)).unwrap();

        // Benchmark GET and POST requests
        c.bench_function("disable_pooling_http_pipeline_test", |b| {
            b.to_async(&rt).iter(|| async {
                let response = client.get("get", None).await;
                assert!(response.is_ok());
                let response = response.unwrap();
                assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
            });
        });
    }
}

#[cfg_attr(target_os = "macos", allow(unused_variables))]
pub fn baseline_http_transport_test(c: &mut Criterion) {
    #[cfg(target_os = "macos")]
    return;

    #[cfg(not(target_os = "macos"))]
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let endpoint = "https://azuresdkforcpp.azurewebsites.net";

        let http_client = new_default_reqwest_client();

        // Benchmark GET and POST requests
        c.bench_function("baseline_http_pipeline_test", |b| {
            b.to_async(&rt).iter(|| async {
                let request = Request::new(
                    Url::parse(&format!("{}/get", endpoint)).unwrap(),
                    Method::Get,
                );
                let response = http_client.execute_request(&request).await;
                assert!(response.is_ok());
                let response = response.unwrap();
                assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
            });
        });
    }
}

// Main benchmark configuration
criterion_group!(name=http_transport_benchmarks;
    config=Criterion::default()
        .sample_size(100)
        .warm_up_time(std::time::Duration::new(10, 0))
        .measurement_time(std::time::Duration::new(50, 0));
    targets=simple_http_transport_test, disable_pooling_http_transport_test, baseline_http_transport_test
);

criterion_main!(http_transport_benchmarks);
