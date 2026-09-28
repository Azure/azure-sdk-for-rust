// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{ClientOptions, HttpClient, Method, Request, Transport, Url};
use azure_core_examples::{
    client::{httpbin_endpoint, TestServiceClient, TestServiceClientOptions},
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

pub fn simple_http_transport_test(c: &mut Criterion, endpoint: &Url) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let credential = MockCredential::new().unwrap();
    let options = TestServiceClientOptions::default();

    let client = TestServiceClient::new(endpoint.as_str(), credential, Some(options)).unwrap();

    c.bench_function("default_http_pipeline_test", |b| {
        b.to_async(&rt).iter(|| async {
            let response = client
                .get("get", None)
                .await
                .expect("GET /get failed; check AZSDKRUSTTEST_HTTPBIN_URL and the httpbin service");
            assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
        });
    });
}

pub fn disable_pooling_http_transport_test(c: &mut Criterion, endpoint: &Url) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let credential = MockCredential::new().unwrap();
    let transport = new_reqwest_client_disable_connection_pool();
    let options = TestServiceClientOptions {
        client_options: ClientOptions {
            transport: Some(Transport::new(transport)),
            ..Default::default()
        },
        ..Default::default()
    };

    let client = TestServiceClient::new(endpoint.as_str(), credential, Some(options)).unwrap();

    c.bench_function("disable_pooling_http_pipeline_test", |b| {
        b.to_async(&rt).iter(|| async {
            let response = client
                .get("get", None)
                .await
                .expect("GET /get failed; check AZSDKRUSTTEST_HTTPBIN_URL and the httpbin service");
            assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
        });
    });
}

pub fn baseline_http_transport_test(c: &mut Criterion, endpoint: &Url) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let http_client = new_default_reqwest_client();
    let url = endpoint.join("get").unwrap();

    c.bench_function("baseline_http_pipeline_test", |b| {
        b.to_async(&rt).iter(|| {
            let url = url.clone();
            let http_client = http_client.clone();
            async move {
                let request = Request::new(url, Method::Get);
                let response = http_client.execute_request(&request).await.expect(
                    "GET /get failed; check AZSDKRUSTTEST_HTTPBIN_URL and the httpbin service",
                );
                assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
            }
        });
    });
}

pub fn raw_reqwest_http_transport_test(c: &mut Criterion, endpoint: &Url) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let url = endpoint.join("get").unwrap();
    let client = ::reqwest::Client::new();

    c.bench_function("raw_http_pipeline_test", |b| {
        b.to_async(&rt).iter(|| async {
            let request = client.get(url.clone());
            let response = request
                .send()
                .await
                .expect("GET /get failed; check AZSDKRUSTTEST_HTTPBIN_URL and the httpbin service");
            assert_eq!(response.status(), reqwest::StatusCode::OK);
        });
    });
}

fn configured_http_transport_benchmarks(c: &mut Criterion) {
    let Some(endpoint) = httpbin_endpoint().expect("Invalid live HTTP benchmark configuration")
    else {
        eprintln!(
            "Live HTTP transport benchmarks not enabled: set AZSDKRUSTTEST_HTTPBIN_URL to an httpbin origin. No HTTP requests were tested."
        );
        return;
    };

    if cfg!(target_os = "macos") {
        eprintln!(
            "Live HTTP transport benchmarks are disabled on macOS. No HTTP requests were tested."
        );
        return;
    }

    simple_http_transport_test(c, &endpoint);
    disable_pooling_http_transport_test(c, &endpoint);
    baseline_http_transport_test(c, &endpoint);
    raw_reqwest_http_transport_test(c, &endpoint);
}

criterion_group!(name=http_transport_benchmarks;
    config=Criterion::default()
        .sample_size(500)
        .warm_up_time(std::time::Duration::new(10, 0))
        .measurement_time(std::time::Duration::new(60, 0));
    targets=configured_http_transport_benchmarks
);

criterion_main!(http_transport_benchmarks);
