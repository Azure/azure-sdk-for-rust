// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#[cfg_attr(target_os = "macos", allow(unused_imports))]
use azure_core::http::{ClientOptions, HttpClient, Method, Request, Transport, Url};
#[cfg_attr(target_os = "macos", allow(unused_imports))]
use azure_core_examples::{
    client::{TestServiceClient, TestServiceClientOptions, HTTP_ENDPOINT},
    identity::MockCredential,
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::Arc;

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

        let endpoint = HTTP_ENDPOINT;
        let credential = MockCredential::new().unwrap();
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

        let endpoint = HTTP_ENDPOINT;
        let credential = MockCredential::new().unwrap();
        let transport = new_reqwest_client_disable_connection_pool();
        let options = TestServiceClientOptions {
            client_options: ClientOptions {
                transport: Some(Transport::new(transport)),
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
        let endpoint = HTTP_ENDPOINT;

        let http_client = new_default_reqwest_client();

        let url = Url::parse(&format!("{}/get", endpoint)).unwrap();

        // Benchmark GET and POST requests
        c.bench_function("baseline_http_pipeline_test", |b| {
            b.to_async(&rt).iter(|| {
                // Clone the Url for this iteration so the async block can take ownership.
                let url = url.clone();
                let http_client = http_client.clone();
                async move {
                    let request = Request::new(url, Method::Get);
                    let response = http_client.execute_request(&request).await;
                    assert!(response.is_ok());
                    let response = response.unwrap();
                    assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
                }
            });
        });
    }
}

#[cfg_attr(target_os = "macos", allow(unused_variables))]
pub fn raw_reqwest_http_transport_test(c: &mut Criterion) {
    #[cfg(target_os = "macos")]
    return;

    #[cfg(not(target_os = "macos"))]
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let endpoint = HTTP_ENDPOINT;

        let client = ::reqwest::Client::new();

        // Benchmark GET and POST requests
        c.bench_function("raw_http_pipeline_test", |b| {
            b.to_async(&rt).iter(|| async {
                let request = client.get(format!("{}/get", endpoint));
                let response = request.send().await;
                assert!(response.is_ok());
                let response = response.unwrap();
                assert_eq!(response.status(), reqwest::StatusCode::OK);
            });
        });
    }
}

// Main benchmark configuration
criterion_group!(name=http_transport_benchmarks;
    config=Criterion::default()
        .sample_size(500)
        .warm_up_time(std::time::Duration::new(10, 0))
        .measurement_time(std::time::Duration::new(60, 0));
    targets=simple_http_transport_test, disable_pooling_http_transport_test, baseline_http_transport_test, raw_reqwest_http_transport_test
);

criterion_main!(http_transport_benchmarks);
