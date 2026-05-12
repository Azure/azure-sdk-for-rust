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
    models::{
        AccountReference, ContainerReference, CosmosOperation, DatabaseReference, ItemReference,
        PartitionKey,
    },
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

/// Stable etag the mock returns for every `/pkranges` response. The cache
/// echoes this value back as the `If-None-Match` header on subsequent
/// changefeed fetches; the mock detects that and returns 304 to terminate
/// the fetch loop.
const PKRANGES_ETAG: &str = "\"pkranges-bench-etag-v1\"";

/// Default payload returned by [`MockTransportClient`] for `/pkranges` requests
/// when no per-instance override is configured. A single-range partition map
/// is enough to satisfy the routing-cache lookup that
/// `CosmosDriver::execute_operation` performs as part of every point read,
/// which keeps the existing `point_read` benchmark functional.
const DEFAULT_PKRANGES_PAYLOAD: &str = r#"{
    "_rid": "benchcontainer==",
    "PartitionKeyRanges": [
        {
            "id": "0",
            "minInclusive": "",
            "maxExclusive": "FF",
            "status": "online"
        }
    ],
    "_count": 1
}"#;

/// Builds a `/pkranges` REST response body containing exactly `n` contiguous
/// partition key ranges that cover the full effective partition key space
/// `["", "FF")`.
///
/// Each range is assigned the integer id `0..n`, both endpoints use 8-char
/// uppercase hex strings, and the boundaries are spaced uniformly across the
/// 32-bit address space so that the resulting strings sort lexicographically
/// in the same order they appear in the slice.
///
/// `n` must be positive; sizes the existing benchmarks use are 1 / 10 / 100,
/// reflecting realistic per-container range counts for small, medium, and
/// large physical-partition setups.
pub fn build_pkranges_payload(n: u32) -> String {
    assert!(n > 0, "pkranges payload must contain at least one range");

    let total: u64 = 0x1_0000_0000;
    let n_u64 = n as u64;
    let boundary = |i: u32| -> String {
        if i == 0 {
            String::new()
        } else if i == n {
            "FF".to_string()
        } else {
            // 8-char uppercase hex so that lexicographic ordering matches numeric ordering
            // for all valid mid-range boundaries.
            format!("{:08X}", (total * i as u64 / n_u64) as u32)
        }
    };

    let mut ranges = String::with_capacity(96 * n as usize);
    for i in 0..n {
        if i > 0 {
            ranges.push(',');
        }
        ranges.push_str(&format!(
            r#"{{"id":"{id}","minInclusive":"{min}","maxExclusive":"{max}","status":"online"}}"#,
            id = i,
            min = boundary(i),
            max = boundary(i + 1),
        ));
    }

    format!(
        r#"{{"_rid":"benchcontainer==","PartitionKeyRanges":[{ranges}],"_count":{n}}}"#,
        ranges = ranges,
        n = n
    )
}

// ---------------------------------------------------------------------------
// Mock transport
// ---------------------------------------------------------------------------

/// In-memory transport that returns pre-canned responses based on the request URL path.
///
/// Path dispatch rules:
/// - Last segment `pkranges` (depth 5)  → pkranges payload (200) — defaults to a
///   single-range map; override per instance with [`Self::with_pkranges_payload`].
/// - Depth 0 (path = `/`)               → account properties (200)
/// - Depth 2 (`/dbs/<name>`)            → database properties (200)
/// - Depth 4 (`/dbs/.../colls/<name>`)  → container properties (200)
/// - All other depths                   → item document (200)
///
/// An optional `latency` delay is injected before each response to simulate
/// network RTT. Typical values: `0` (no delay), `2 ms` (same-DC SLA), `10 ms`
/// (cross-region baseline).
#[derive(Debug)]
pub struct MockTransportClient {
    pub latency: std::time::Duration,
    /// Body returned for any request whose path's final segment is `pkranges`.
    /// Cloned on each request to avoid copying for the common `Arc::clone` path.
    pkranges_payload: Arc<str>,
}

impl MockTransportClient {
    /// Creates a `MockTransportClient` with no artificial latency and the
    /// default single-range `/pkranges` payload.
    pub fn new() -> Self {
        Self {
            latency: std::time::Duration::ZERO,
            pkranges_payload: Arc::from(DEFAULT_PKRANGES_PAYLOAD),
        }
    }

    /// Creates a `MockTransportClient` with the given simulated network latency.
    pub fn with_latency(latency: std::time::Duration) -> Self {
        Self {
            latency,
            pkranges_payload: Arc::from(DEFAULT_PKRANGES_PAYLOAD),
        }
    }

    /// Replaces the body returned for `/pkranges` requests. Intended for the
    /// pkrange cache benchmark, which needs to vary the number of ranges per
    /// scenario without rebuilding the driver runtime.
    pub fn with_pkranges_payload(mut self, payload: Arc<str>) -> Self {
        self.pkranges_payload = payload;
        self
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

        let segments: Vec<&str> = request
            .url
            .path()
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let is_pkranges = segments.last() == Some(&"pkranges");

        // The pkrange cache uses changefeed semantics with `If-None-Match` for
        // incremental refreshes. After the cache primes itself with the first
        // 200 response, every subsequent fetch sends back our etag — we must
        // return 304 Not Modified to terminate the cache's fetch loop. Without
        // this, the cache merges the same payload N times and trips its
        // overlap detector.
        let if_none_match =
            request
                .headers
                .get_optional_str(&azure_core::http::headers::HeaderName::from_static(
                    "if-none-match",
                ));
        let return_not_modified = is_pkranges && if_none_match == Some(PKRANGES_ETAG);

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

        if return_not_modified {
            headers.insert(
                azure_core::http::headers::HeaderName::from_static("etag"),
                PKRANGES_ETAG,
            );
            return Ok(HttpResponse {
                status: 304,
                headers,
                body: Vec::new(),
            });
        }

        let body: Vec<u8> = if is_pkranges {
            // Always advertise the same etag so the next changefeed fetch
            // round-trips it back via `If-None-Match`.
            headers.insert(
                azure_core::http::headers::HeaderName::from_static("etag"),
                PKRANGES_ETAG,
            );
            self.pkranges_payload.as_bytes().to_vec()
        } else {
            match segments.len() {
                0 => ACCOUNT_PROPERTIES_PAYLOAD.as_bytes().to_vec(),
                2 => DATABASE_PROPERTIES_PAYLOAD.as_bytes().to_vec(),
                4 => CONTAINER_PROPERTIES_PAYLOAD.as_bytes().to_vec(),
                _ => ITEM_PAYLOAD.as_bytes().to_vec(),
            }
        };

        Ok(HttpResponse {
            status: 200,
            headers,
            body,
        })
    }
}

// ---------------------------------------------------------------------------
// Mock factory
// ---------------------------------------------------------------------------

/// HTTP client factory that always produces a [`MockTransportClient`].
///
/// By default, clients are built with the single-range `/pkranges` payload.
/// Use [`Self::with_pkranges_payload`] to swap in a multi-range payload —
/// this is the lever the pkrange cache benchmark uses to size the routing
/// map per scenario.
#[derive(Debug)]
pub struct MockHttpClientFactory {
    pkranges_payload: Arc<str>,
}

impl MockHttpClientFactory {
    pub fn new() -> Self {
        Self {
            pkranges_payload: Arc::from(DEFAULT_PKRANGES_PAYLOAD),
        }
    }

    /// Replaces the `/pkranges` body returned by every transport this factory builds.
    pub fn with_pkranges_payload(mut self, payload: Arc<str>) -> Self {
        self.pkranges_payload = payload;
        self
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
        Ok(Arc::new(
            MockTransportClient::new().with_pkranges_payload(self.pkranges_payload.clone()),
        ))
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

/// Builds a mock-backed driver whose `/pkranges` responses describe `n`
/// contiguous partition key ranges, and resolves the bench container so the
/// caller can drive `resolve_*partition_key_range*` calls without any further
/// setup.
///
/// The returned `PartitionKey` is a stable single-string key (`pk1`) that
/// hashes deterministically into the same routing-map cell across iterations,
/// which keeps the bench measurement focused on the cache-lookup path itself.
pub async fn setup_with_pkranges(n: u32) -> (Arc<CosmosDriver>, ContainerReference, PartitionKey) {
    let payload: Arc<str> = Arc::from(build_pkranges_payload(n));
    let factory = Arc::new(MockHttpClientFactory::new().with_pkranges_payload(payload));
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

    (driver, container, PartitionKey::from("pk1"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Asserts that `build_pkranges_payload` produces a syntactically valid
    /// changefeed body whose ranges form a contiguous, fully-covering
    /// partition of the EPK space `["", "FF")`. Without this, a regression
    /// in the helper would silently break the `pkrange_cache` benchmark
    /// (the routing-map cache would reject the response with
    /// `RoutingMapError::IncompleteRanges` or `OverlappingRanges`).
    #[test]
    fn build_pkranges_payload_covers_full_space() {
        for &n in &[1u32, 10, 100] {
            let body = build_pkranges_payload(n);
            let parsed: serde_json::Value =
                serde_json::from_str(&body).expect("payload must be valid JSON");
            let arr = parsed["PartitionKeyRanges"]
                .as_array()
                .expect("PartitionKeyRanges must be an array");
            assert_eq!(arr.len(), n as usize, "range count for n={n}");

            // First range starts at the empty-string sentinel.
            assert_eq!(arr[0]["minInclusive"].as_str(), Some(""), "n={n} first min");
            // Last range ends at the "FF" sentinel.
            let last_max = arr[(n - 1) as usize]["maxExclusive"].as_str();
            assert_eq!(last_max, Some("FF"), "n={n} last max");

            // Every internal boundary must be byte-equal between adjacent
            // ranges — that's what `validate_and_build_index` checks for.
            for i in 1..n as usize {
                let prev_max = arr[i - 1]["maxExclusive"].as_str().unwrap();
                let next_min = arr[i]["minInclusive"].as_str().unwrap();
                assert_eq!(
                    prev_max,
                    next_min,
                    "n={n}: gap between range {} (max={prev_max}) and range {} (min={next_min})",
                    i - 1,
                    i
                );
            }
        }
    }
}
