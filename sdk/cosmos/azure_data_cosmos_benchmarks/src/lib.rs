// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cSpell:ignore benchdb benchcontainer benchitem

//! Shared transport infrastructure for `azure_data_cosmos_benchmarks`.
//!
//! Provides both a mock transport (zero-latency, in-memory) and a live-transport
//! setup backed by a real Cosmos DB endpoint. The active mode is controlled by the
//! `AZURE_BENCH_MODE` environment variable.
//!
//! Re-used by both the Criterion benchmarks (`benches/`) and the Valgrind
//! example (`examples/`).

use std::sync::Arc;

use async_trait::async_trait;
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntimeBuilder,
    models::{AccountReference, CosmosOperation, DatabaseReference, ItemReference, PartitionKey},
    options::OperationOptions,
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

impl Default for MockTransportClient {
    fn default() -> Self {
        Self::new()
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
pub struct MockHttpClientFactory;

impl MockHttpClientFactory {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MockHttpClientFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClientFactory for MockHttpClientFactory {
    fn build(
        &self,
        _connection_pool: &ConnectionPoolOptions,
        _config: HttpClientConfig,
    ) -> azure_core::Result<Arc<dyn TransportClient>> {
        Ok(Arc::new(MockTransportClient::new()))
    }
}

// ---------------------------------------------------------------------------
// Shared setup
// ---------------------------------------------------------------------------

/// Benchmark run mode.
///
/// Controlled by the `AZURE_BENCH_MODE` environment variable. Use
/// [`load_bench_config`] to read the active mode at startup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchConfig {
    /// Use the in-memory mock transport (default).
    Mock,
    /// Use a real Cosmos DB endpoint configured via environment variables.
    Live,
}

/// Reads `AZURE_BENCH_MODE` and returns the corresponding [`BenchConfig`].
///
/// Returns [`BenchConfig::Live`] when the variable is set to `live`
/// (case-insensitive). Any other value, or an absent variable, returns
/// [`BenchConfig::Mock`].
pub fn load_bench_config() -> BenchConfig {
    match std::env::var("AZURE_COSMOS_BENCH_MODE") {
        Ok(v) if v.eq_ignore_ascii_case("live") => BenchConfig::Live,
        _ => BenchConfig::Mock,
    }
}

/// Builds a fully-initialized driver backed by the real Cosmos DB transport.
///
/// Reads configuration from environment variables:
///
/// | Variable | Required | Default |
/// |---|---|---|
/// | `AZURE_COSMOS_ENDPOINT` | yes | — |
/// | `AZURE_COSMOS_KEY` | yes | — |
/// | `AZURE_COSMOS_DATABASE` | no | `bench` |
/// | `AZURE_COSMOS_CONTAINER` | no | `bench` |
/// | `AZURE_COSMOS_PARTITION_KEY` | no | `pk1` |
/// | `AZURE_COSMOS_ITEM_ID` | no | `item1` |
///
/// Creates the target database, container, and item if they do not already
/// exist (409 Conflict responses are silently ignored). All cache priming
/// happens here so that callers benchmark the fully-warm hot path only.
pub async fn setup_live() -> (Arc<CosmosDriver>, ItemReference) {
    let endpoint = std::env::var("AZURE_COSMOS_ENDPOINT")
        .expect("AZURE_COSMOS_ENDPOINT must be set for live benchmarks");
    let key = std::env::var("AZURE_COSMOS_KEY")
        .expect("AZURE_COSMOS_KEY must be set for live benchmarks");
    let database = std::env::var("AZURE_COSMOS_DATABASE").unwrap_or_else(|_| "bench".to_string());
    let container = std::env::var("AZURE_COSMOS_CONTAINER").unwrap_or_else(|_| "bench".to_string());
    let pk_value =
        std::env::var("AZURE_COSMOS_PARTITION_KEY").unwrap_or_else(|_| "pk1".to_string());
    let item_id = std::env::var("AZURE_COSMOS_ITEM_ID").unwrap_or_else(|_| "item1".to_string());

    let endpoint_url = Url::parse(&endpoint).expect("AZURE_COSMOS_ENDPOINT is not a valid URL");
    let account = AccountReference::with_master_key(endpoint_url, key.clone());

    let runtime = CosmosDriverRuntimeBuilder::new()
        .build()
        .await
        .expect("failed to build runtime");

    let driver = runtime
        .get_or_create_driver(account.clone(), None)
        .await
        .expect("failed to create driver");

    // Create the database if it doesn't exist.
    let db_body = format!(r#"{{"id": "{}"}}"#, database);
    ignore_conflict(
        driver
            .execute_operation(
                CosmosOperation::create_database(account.clone()).with_body(db_body.into_bytes()),
                OperationOptions::default(),
            )
            .await,
    )
    .expect("failed to create database");

    let database_ref = DatabaseReference::from_name(account.clone(), database.clone());

    // Create the container if it doesn't exist.
    let container_body = format!(
        r#"{{"id": "{}", "partitionKey": {{"paths": ["/pk"], "kind": "Hash", "version": 2}}}}"#,
        container
    );
    ignore_conflict(
        driver
            .execute_operation(
                CosmosOperation::create_container(database_ref)
                    .with_body(container_body.into_bytes()),
                OperationOptions::default(),
            )
            .await,
    )
    .expect("failed to create container");

    // Resolve the container (primes the container cache).
    let container_ref = driver
        .resolve_container(database.as_str(), container.as_str())
        .await
        .expect("failed to resolve container");

    // Create the benchmark item if it doesn't already exist.
    let item_body = format!(r#"{{"id": "{}", "pk": "{}"}}"#, item_id, pk_value);
    let item_ref = ItemReference::from_name(
        &container_ref,
        PartitionKey::from(pk_value.clone()),
        item_id.clone(),
    );
    ignore_conflict(
        driver
            .execute_operation(
                CosmosOperation::create_item(item_ref).with_body(item_body.into_bytes()),
                OperationOptions::default(),
            )
            .await,
    )
    .expect("failed to create benchmark item");

    let item_ref = ItemReference::from_name(
        &container_ref,
        PartitionKey::from(pk_value.clone()),
        item_id.clone(),
    );

    (driver, item_ref)
}

/// Returns `Ok(())` for a successful result or a 409 Conflict error.
///
/// Used during setup to ignore "resource already exists" responses when
/// creating the benchmark database, container, and item.
fn ignore_conflict(
    result: azure_core::Result<azure_data_cosmos_driver::CosmosResponse>,
) -> azure_core::Result<()> {
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            if let azure_core::error::ErrorKind::HttpResponse { status, .. } = e.kind() {
                if *status == azure_core::http::StatusCode::Conflict {
                    return Ok(());
                }
            }
            Err(e)
        }
    }
}

/// Builds a fully-initialized driver backed by the mock transport.
///
/// All cache priming (account metadata, container metadata) happens here so
/// that callers can benchmark or profile the fully-warm hot path only.
pub async fn setup() -> (Arc<CosmosDriver>, ItemReference) {
    let factory = Arc::new(MockHttpClientFactory::new());
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
