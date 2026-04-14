// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared mock transport infrastructure for benchmarks.
//!
//! Both `point_read` and `heap_profile` bench binaries include this module via
//! `#[path = "common.rs"] mod common;`.

use std::sync::Arc;

use async_trait::async_trait;
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntimeBuilder,
    models::{AccountReference, ItemReference, PartitionKey},
    testing::{
        ConnectionPoolOptions, HttpClientConfig, HttpClientFactory, HttpRequest, HttpResponse,
        TransportClient, TransportError,
    },
    CosmosDriver,
};
use url::Url;

// ---------------------------------------------------------------------------
// Pre-canned JSON payloads
// ---------------------------------------------------------------------------

pub const ACCOUNT_PROPERTIES_PAYLOAD: &str = r#"{
    "_self": "",
    "id": "bench",
    "_rid": "bench.documents.azure.com",
    "media": "//media/",
    "addresses": "//addresses/",
    "_dbs": "//dbs/",
    "writableLocations": [
        { "name": "West US 2", "databaseAccountEndpoint": "https://bench-westus2.documents.azure.com:443/" }
    ],
    "readableLocations": [
        { "name": "West US 2", "databaseAccountEndpoint": "https://bench-westus2.documents.azure.com:443/" }
    ],
    "enableMultipleWriteLocations": false,
    "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
    "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
    "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
    "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
    "queryEngineConfiguration": "{}"
}"#;

pub const DATABASE_PROPERTIES_PAYLOAD: &str = r#"{
    "id": "benchdb",
    "_rid": "benchdb==",
    "_self": "dbs/benchdb==/",
    "_etag": "\"00000000-0000-0000-0000-000000000001\"",
    "_colls": "colls/",
    "_users": "users/",
    "_ts": 1
}"#;

pub const CONTAINER_PROPERTIES_PAYLOAD: &str = r#"{
    "id": "benchcontainer",
    "_rid": "benchcontainer==",
    "_self": "dbs/benchdb==/colls/benchcontainer==/",
    "_etag": "\"00000000-0000-0000-0000-000000000002\"",
    "partitionKey": { "paths": ["/pk"], "kind": "Hash", "version": 2 },
    "indexingPolicy": { "indexingMode": "consistent", "automatic": true },
    "_ts": 1,
    "_docs": "docs/",
    "_sprocs": "sprocs/",
    "_triggers": "triggers/",
    "_udfs": "udfs/",
    "_conflicts": "conflicts/"
}"#;

pub const ITEM_PAYLOAD: &str = r#"{
    "id": "item1",
    "pk": "pk1",
    "_rid": "benchitem==",
    "_self": "dbs/benchdb==/colls/benchcontainer==/docs/benchitem==/",
    "_etag": "\"00000000-0000-0000-0000-000000000003\"",
    "_attachments": "attachments/",
    "_ts": 1
}"#;

// ---------------------------------------------------------------------------
// Mock transport
// ---------------------------------------------------------------------------

/// In-memory transport that returns pre-canned responses based on the request URL path.
///
/// Path dispatch rules (checked by URL path segment depth):
/// - Depth 0 (path = `/`)          → account properties (200)
/// - Depth 2 (`/dbs/<name>`)       → database properties (200)
/// - Depth 4 (`/dbs/.../colls/<name>`) → container properties (200)
/// - All other depths              → item document (200)
///
/// An optional `latency` delay is injected before each response to simulate
/// network RTT. Typical values: `0` (no delay), `2 ms` (same-DC SLA), `10 ms`
/// (cross-region baseline).
#[derive(Debug)]
pub struct MockTransportClient {
    pub latency: std::time::Duration,
}

impl MockTransportClient {
    /// Creates a `MockTransportClient` with no artificial latency.
    pub fn new() -> Self {
        Self {
            latency: std::time::Duration::ZERO,
        }
    }

    /// Creates a `MockTransportClient` with the given simulated network latency.
    pub fn with_latency(latency: std::time::Duration) -> Self {
        Self { latency }
    }
}

#[async_trait]
impl TransportClient for MockTransportClient {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        if !self.latency.is_zero() {
            tokio::time::sleep(self.latency).await;
        }

        let depth = request
            .url
            .path()
            .split('/')
            .filter(|s| !s.is_empty())
            .count();

        let body: &[u8] = match depth {
            0 => ACCOUNT_PROPERTIES_PAYLOAD.as_bytes(),
            2 => DATABASE_PROPERTIES_PAYLOAD.as_bytes(),
            4 => CONTAINER_PROPERTIES_PAYLOAD.as_bytes(),
            _ => ITEM_PAYLOAD.as_bytes(),
        };

        let mut headers = azure_core::http::headers::Headers::new();
        headers.insert(azure_core::http::headers::CONTENT_TYPE, "application/json");
        headers.insert(
            azure_core::http::headers::HeaderName::from_static("x-ms-session-token"),
            "0:1",
        );
        headers.insert(
            azure_core::http::headers::HeaderName::from_static("x-ms-request-charge"),
            "1.0",
        );

        Ok(HttpResponse {
            status: 200,
            headers,
            body: body.to_vec(),
        })
    }
}

// ---------------------------------------------------------------------------
// Mock factory
// ---------------------------------------------------------------------------

/// HTTP client factory that always produces a [`MockTransportClient`] with a given latency.
#[derive(Debug)]
pub struct MockHttpClientFactory {
    latency: std::time::Duration,
}

impl MockHttpClientFactory {
    pub fn new(latency: std::time::Duration) -> Self {
        Self { latency }
    }
}

impl HttpClientFactory for MockHttpClientFactory {
    fn build(
        &self,
        _connection_pool: &ConnectionPoolOptions,
        _config: HttpClientConfig,
    ) -> azure_core::Result<Arc<dyn TransportClient>> {
        Ok(Arc::new(MockTransportClient::with_latency(self.latency)))
    }
}

// ---------------------------------------------------------------------------
// Shared setup
// ---------------------------------------------------------------------------

/// Builds a fully-initialized driver backed by the mock transport.
///
/// `latency` is injected into every mock response to simulate network RTT.
/// Use `Duration::ZERO` for pure CPU-overhead measurement, `Duration::from_millis(2)`
/// for same-DC SLA baseline, or `Duration::from_millis(10)` for cross-region baseline.
///
/// All cache priming (account metadata, container metadata) happens here so
/// that callers can benchmark or profile the fully-warm hot path only.
pub async fn setup(latency: std::time::Duration) -> (Arc<CosmosDriver>, ItemReference) {
    let factory = Arc::new(MockHttpClientFactory::new(latency));
    let runtime = CosmosDriverRuntimeBuilder::new()
        .with_mock_http_client_factory(factory)
        .build()
        .await
        .expect("failed to build runtime");

    let account = AccountReference::with_master_key(
        Url::parse("https://bench.documents.azure.com:443/").unwrap(),
        "dGVzdA==",
    );

    let driver = runtime
        .get_or_create_driver(account, None)
        .await
        .expect("failed to create driver");

    let container = driver
        .resolve_container("benchdb", "benchcontainer")
        .await
        .expect("failed to resolve container");

    let item_ref = ItemReference::from_name(&container, PartitionKey::from("pk1"), "item1");

    (driver, item_ref)
}
