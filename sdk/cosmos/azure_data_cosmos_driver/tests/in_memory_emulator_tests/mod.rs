// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared test helpers for the in-memory emulator integration tests.

pub mod control_plane;
pub mod driver_end_to_end;
pub mod error_cases;
pub mod multi_region;
pub mod point_operations;
pub mod split_merge;
pub mod throttling;

use azure_core::http::{
    headers::{HeaderName, HeaderValue, Headers},
    AsyncRawResponse, HttpClient, Method, Request, StatusCode, Url,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, ReplicationConfig, VirtualAccountConfig,
    VirtualRegion, WriteMode,
};
use std::sync::Arc;

/// Default gateway URL for single-region tests.
pub const GATEWAY_URL: &str = "https://eastus.emulator.local";

/// Creates a single-region emulator with a pre-provisioned database and container.
pub async fn setup_single_region() -> TestContext {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .with_consistency(ConsistencyLevel::Session);

    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();

    store.create_database("testdb");
    store.create_container(
        "testdb",
        "testcoll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );

    TestContext {
        emulator,
        gateway_url: GATEWAY_URL.to_string(),
    }
}

/// Creates a multi-region emulator.
pub async fn setup_multi_region(write_mode: WriteMode) -> MultiRegionTestContext {
    let east_url = "https://eastus.emulator.local";
    let west_url = "https://westus.emulator.local";

    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(east_url).unwrap()),
        VirtualRegion::new("West US", Url::parse(west_url).unwrap()),
    ])
    .with_write_mode(write_mode)
    .with_consistency(ConsistencyLevel::Session)
    .with_replication_config(ReplicationConfig::immediate());

    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();

    store.create_database("testdb");
    store.create_container(
        "testdb",
        "testcoll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );

    MultiRegionTestContext {
        emulator,
        east_url: east_url.to_string(),
        west_url: west_url.to_string(),
    }
}

/// Test context for single-region tests.
pub struct TestContext {
    pub emulator: Arc<InMemoryEmulatorHttpClient>,
    pub gateway_url: String,
}

/// Test context for multi-region tests.
pub struct MultiRegionTestContext {
    pub emulator: Arc<InMemoryEmulatorHttpClient>,
    pub east_url: String,
    pub west_url: String,
}

/// Header name constants for assertions.
pub static ETAG: HeaderName = HeaderName::from_static("etag");
pub static REQUEST_CHARGE: HeaderName = HeaderName::from_static("x-ms-request-charge");
pub static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
pub static SUBSTATUS: HeaderName = HeaderName::from_static("x-ms-substatus");
pub static ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
pub static PARTITION_KEY: HeaderName = HeaderName::from_static("x-ms-documentdb-partitionkey");
pub static IS_UPSERT: HeaderName = HeaderName::from_static("x-ms-documentdb-is-upsert");
pub static CONTENT_RESPONSE: HeaderName =
    HeaderName::from_static("x-ms-cosmos-populate-content-response-on-write");
pub static IF_MATCH: HeaderName = HeaderName::from_static("if-match");

/// Sends a request and collects the response body.
pub async fn send_request(
    client: &Arc<InMemoryEmulatorHttpClient>,
    request: Request,
) -> (AsyncRawResponse, Vec<u8>) {
    let response = client.execute_request(&request).await.unwrap();
    // Collect the body by reading the raw response
    (response, vec![])
}

/// Helper to create a POST request to create a document.
pub fn create_item_request(
    gateway_url: &str,
    db: &str,
    coll: &str,
    body: &serde_json::Value,
    pk: &str,
    content_response: bool,
) -> Request {
    let url = format!("{}/dbs/{}/colls/{}/docs", gateway_url, db, coll);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);
    let body_bytes = serde_json::to_vec(body).unwrap();
    req.set_body(body_bytes);
    req.headers_mut()
        .insert(PARTITION_KEY.clone(), HeaderValue::from(pk.to_string()));
    if content_response {
        req.headers_mut()
            .insert(CONTENT_RESPONSE.clone(), HeaderValue::from_static("True"));
    }
    req
}

/// Helper to create a GET request to read a document.
pub fn read_item_request(
    gateway_url: &str,
    db: &str,
    coll: &str,
    doc_id: &str,
    pk: &str,
) -> Request {
    let url = format!("{}/dbs/{}/colls/{}/docs/{}", gateway_url, db, coll, doc_id);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    req.headers_mut()
        .insert(PARTITION_KEY.clone(), HeaderValue::from(pk.to_string()));
    req
}

/// Helper to create a PUT request to replace a document.
pub fn replace_item_request(
    gateway_url: &str,
    db: &str,
    coll: &str,
    doc_id: &str,
    body: &serde_json::Value,
    pk: &str,
    if_match: Option<&str>,
    content_response: bool,
) -> Request {
    let url = format!("{}/dbs/{}/colls/{}/docs/{}", gateway_url, db, coll, doc_id);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Put);
    let body_bytes = serde_json::to_vec(body).unwrap();
    req.set_body(body_bytes);
    req.headers_mut()
        .insert(PARTITION_KEY.clone(), HeaderValue::from(pk.to_string()));
    if let Some(etag) = if_match {
        req.headers_mut()
            .insert(IF_MATCH.clone(), HeaderValue::from(etag.to_string()));
    }
    if content_response {
        req.headers_mut()
            .insert(CONTENT_RESPONSE.clone(), HeaderValue::from_static("True"));
    }
    req
}

/// Helper to create a DELETE request.
pub fn delete_item_request(
    gateway_url: &str,
    db: &str,
    coll: &str,
    doc_id: &str,
    pk: &str,
    if_match: Option<&str>,
) -> Request {
    let url = format!("{}/dbs/{}/colls/{}/docs/{}", gateway_url, db, coll, doc_id);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Delete);
    req.headers_mut()
        .insert(PARTITION_KEY.clone(), HeaderValue::from(pk.to_string()));
    if let Some(etag) = if_match {
        req.headers_mut()
            .insert(IF_MATCH.clone(), HeaderValue::from(etag.to_string()));
    }
    req
}

/// Helper to create an upsert request.
pub fn upsert_item_request(
    gateway_url: &str,
    db: &str,
    coll: &str,
    body: &serde_json::Value,
    pk: &str,
    content_response: bool,
) -> Request {
    let url = format!("{}/dbs/{}/colls/{}/docs", gateway_url, db, coll);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);
    let body_bytes = serde_json::to_vec(body).unwrap();
    req.set_body(body_bytes);
    req.headers_mut()
        .insert(PARTITION_KEY.clone(), HeaderValue::from(pk.to_string()));
    req.headers_mut()
        .insert(IS_UPSERT.clone(), HeaderValue::from_static("True"));
    if content_response {
        req.headers_mut()
            .insert(CONTENT_RESPONSE.clone(), HeaderValue::from_static("True"));
    }
    req
}

/// Reads the response body as JSON. Consumes the response.
pub async fn read_response_body(response: AsyncRawResponse) -> serde_json::Value {
    let raw = response.try_into_raw_response().await.unwrap();
    let body_bytes: &[u8] = raw.body().as_ref();
    if body_bytes.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::from_slice(body_bytes).unwrap_or(serde_json::Value::Null)
    }
}

/// Fully buffers a response and returns status, headers, and body JSON.
pub async fn collect_response(
    response: AsyncRawResponse,
) -> (StatusCode, Headers, serde_json::Value) {
    let raw = response.try_into_raw_response().await.unwrap();
    let status = raw.status();
    let headers = raw.headers().clone();
    let body_bytes: &[u8] = raw.body().as_ref();
    let body = if body_bytes.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::from_slice(body_bytes).unwrap_or(serde_json::Value::Null)
    };
    (status, headers, body)
}
