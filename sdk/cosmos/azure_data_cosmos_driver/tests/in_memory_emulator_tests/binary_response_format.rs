// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response-format negotiation tests.
//!
//! These tests exercise the **response side** of Cosmos binary JSON
//! negotiation at the emulator: given a binary request body, what serialization
//! format does the service send *back*? The answer is driven entirely by the
//! `x-ms-cosmos-supported-serialization-formats` request header:
//!
//! - `JsonText,CosmosBinary` (or `CosmosBinary` alone) → the service replies
//!   with **binary** (body begins with the `0x80` preamble).
//! - `JsonText` alone → the service replies with **text**, even though the
//!   request body was binary.
//! - no header (binary encoding disabled) → **text**.
//!
//! Note the SDK-level `BinaryEncodingOptions::request_text_response` does **not**
//! send `JsonText` alone: point operations keep advertising `CosmosBinary` (so
//! the wire stays binary) and the *driver* transcodes the binary response to
//! text. These tests cover the underlying emulator format decision directly.
//!
//! The tests send a Cosmos-binary request body directly through the in-memory
//! emulator and inspect the **raw** response bytes, so they assert on the actual
//! wire format rather than a decoded value.

use super::*;
use azure_core::http::headers::HeaderValue;
use azure_data_cosmos_driver::binary_json::{self, PREAMBLE};

/// Builds a create-item POST whose body is Cosmos **binary** JSON, optionally
/// advertising a response-format via `x-ms-cosmos-supported-serialization-formats`.
fn create_binary_item_request(
    gateway_url: &str,
    db: &str,
    coll: &str,
    body: &serde_json::Value,
    pk: &str,
    serialization_formats: Option<&str>,
) -> Request {
    let url = format!("{}/dbs/{}/colls/{}/docs", gateway_url, db, coll);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);

    // The request body is binary — begins with the 0x80 preamble.
    let binary_body = binary_json::encode(body);
    assert_eq!(
        binary_body.first(),
        Some(&PREAMBLE),
        "test setup: request body must be binary",
    );
    req.set_body(binary_body);

    req.headers_mut()
        .insert(PARTITION_KEY.clone(), HeaderValue::from(pk.to_string()));
    req.headers_mut()
        .insert(CONTENT_RESPONSE.clone(), HeaderValue::from_static("True"));
    if let Some(formats) = serialization_formats {
        req.headers_mut().insert(
            SUPPORTED_SERIALIZATION_FORMATS.clone(),
            HeaderValue::from(formats.to_string()),
        );
    }
    req
}

/// Default negotiation (`JsonText,CosmosBinary`): the response body is binary.
///
/// This documents the *current* behavior — when binary encoding is enabled with
/// its default options, the service (emulator) sends the response back as
/// **binary**.
#[tokio::test]
async fn enabled_default_negotiation_yields_binary_response() {
    let ctx = setup_single_region().await;
    let body = serde_json::json!({ "id": "bin-1", "pk": "pk1", "value": 42 });

    let req = create_binary_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        Some("JsonText,CosmosBinary"),
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _headers, raw) = collect_raw_response(response).await;

    assert_eq!(status, StatusCode::Created);
    assert_eq!(
        raw.first(),
        Some(&PREAMBLE),
        "default negotiation must return a binary (0x80) response body",
    );
    assert!(
        binary_json::is_binary(&raw),
        "response body must be detected as binary",
    );

    // And it decodes back to the stored document.
    let decoded: serde_json::Value = binary_json::decode(&raw).unwrap();
    assert_eq!(decoded["id"], "bin-1");
    assert_eq!(decoded["value"], 42);
}

/// Explicit `JsonText`-only negotiation: the response body is **text**, even
/// though the request body was binary.
///
/// This asserts the emulator's underlying format decision when a request
/// advertises `JsonText` alone. Note this is **not** the SDK-level
/// `request_text_response` behavior (which keeps advertising `CosmosBinary` and
/// has the driver transcode the binary response); it is the lower-level
/// negotiation primitive that mode does not use.
#[tokio::test]
async fn jsontext_only_negotiation_yields_text_response_despite_binary_request() {
    let ctx = setup_single_region().await;
    let body = serde_json::json!({ "id": "text-1", "pk": "pk1", "value": 7 });

    let req = create_binary_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        Some("JsonText"),
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _headers, raw) = collect_raw_response(response).await;

    assert_eq!(status, StatusCode::Created);
    assert_ne!(
        raw.first(),
        Some(&PREAMBLE),
        "text negotiation must NOT return a binary (0x80) response body",
    );
    assert!(
        !binary_json::is_binary(&raw),
        "response body must NOT be detected as binary",
    );

    // The raw bytes are valid UTF-8 text JSON and decode directly.
    let text = std::str::from_utf8(&raw).expect("text response must be valid UTF-8");
    let decoded: serde_json::Value = serde_json::from_str(text).unwrap();
    assert_eq!(decoded["id"], "text-1");
    assert_eq!(decoded["value"], 7);
}

/// No negotiation header (binary encoding disabled): the response is text.
#[tokio::test]
async fn no_negotiation_header_yields_text_response() {
    let ctx = setup_single_region().await;
    let body = serde_json::json!({ "id": "notneg-1", "pk": "pk1", "value": 1 });

    let req = create_binary_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        None,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _headers, raw) = collect_raw_response(response).await;

    assert_eq!(status, StatusCode::Created);
    assert!(
        !binary_json::is_binary(&raw),
        "absent negotiation header must yield a text response",
    );
    let decoded: serde_json::Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(decoded["id"], "notneg-1");
}

/// A read of a binary-written document, requested with `JsonText`, comes back as
/// text — proving the stored value is format-agnostic and the response format is
/// governed purely by the read's negotiation header.
#[tokio::test]
async fn text_read_of_binary_written_item_yields_text_response() {
    let ctx = setup_single_region().await;
    let body = serde_json::json!({ "id": "mixed-1", "pk": "pk1", "value": 314 });

    // Write with default (binary) negotiation → stored via the binary request
    // body, and (per the first test) echoed back as binary.
    let write = create_binary_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        Some("JsonText,CosmosBinary"),
    );
    let write_resp = ctx.emulator.execute_request(&write).await.unwrap();
    let (write_status, _h, write_raw) = collect_raw_response(write_resp).await;
    assert_eq!(write_status, StatusCode::Created);
    assert!(
        binary_json::is_binary(&write_raw),
        "write response should be binary under default negotiation",
    );

    // Read the same item, but advertise only JsonText → text response.
    let mut read = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "mixed-1",
        r#"["pk1"]"#,
    );
    read.headers_mut().insert(
        SUPPORTED_SERIALIZATION_FORMATS.clone(),
        HeaderValue::from_static("JsonText"),
    );
    let read_resp = ctx.emulator.execute_request(&read).await.unwrap();
    let (read_status, _h, read_raw) = collect_raw_response(read_resp).await;

    assert_eq!(read_status, StatusCode::Ok);
    assert!(
        !binary_json::is_binary(&read_raw),
        "text-negotiated read must return a text response even though the item was written binary",
    );
    let decoded: serde_json::Value = serde_json::from_slice(&read_raw).unwrap();
    assert_eq!(decoded["id"], "mixed-1");
    assert_eq!(decoded["value"], 314);
}

/// The same item read as an **untyped** [`serde_json::Value`] must be identical
/// whether the response arrived binary or text.
///
/// This is the assertion the rest of the suite structurally cannot make: the
/// fuzzer compares two *binary* decoders, and the perf corpus test reconciles
/// number variants through `numbers_equivalent`. Plain `==` on untyped `Value`s
/// is what pins it, since `serde_json::Number`'s `PartialEq` is
/// variant-sensitive (`PosInt(3) != Float(3.0)`).
///
/// Untyped matters: an integer field routes to `deserialize_integer`, which has
/// coerced since #4976, while `Value` routes to `deserialize_any`, which did
/// not. Every model in this repo is typed, which is why this went unnoticed on
/// the point-read path even though point operations have negotiated binary all
/// along.
#[tokio::test]
async fn untyped_read_agrees_between_binary_and_text_responses() {
    let ctx = setup_single_region().await;
    // Written as floats so the service (and emulator) hold them as `Double` —
    // the case where the binary and text spellings of one value can diverge.
    let body = serde_json::json!({
        "id": "untyped-1",
        "pk": "pk1",
        "small": 3.0,
        "negative": -7.0,
        "fractional": 2.5,
        "nested": { "arr": [1.0, 2.5, -3.0] },
    });

    let write = create_binary_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        Some("JsonText,CosmosBinary"),
    );
    let write_resp = ctx.emulator.execute_request(&write).await.unwrap();
    let (write_status, _h, _raw) = collect_raw_response(write_resp).await;
    assert_eq!(write_status, StatusCode::Created);

    // Read once negotiating binary, once negotiating text.
    let read_as = |formats: &'static str| {
        let mut req = read_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            "untyped-1",
            r#"["pk1"]"#,
        );
        req.headers_mut().insert(
            SUPPORTED_SERIALIZATION_FORMATS.clone(),
            HeaderValue::from_static(formats),
        );
        req
    };

    let binary_resp = ctx
        .emulator
        .execute_request(&read_as("JsonText,CosmosBinary"))
        .await
        .unwrap();
    let (binary_status, _h, binary_raw) = collect_raw_response(binary_resp).await;
    assert_eq!(binary_status, StatusCode::Ok);
    assert!(
        binary_json::is_binary(&binary_raw),
        "test setup: the binary-negotiated read must actually return binary",
    );

    let text_resp = ctx
        .emulator
        .execute_request(&read_as("JsonText"))
        .await
        .unwrap();
    let (text_status, _h, text_raw) = collect_raw_response(text_resp).await;
    assert_eq!(text_status, StatusCode::Ok);
    assert!(
        !binary_json::is_binary(&text_raw),
        "test setup: the text-negotiated read must actually return text",
    );

    // `from_slice` is the deserializer the driver hands binary bodies to, so
    // this exercises the real caller boundary rather than the reference decoder.
    let from_binary: serde_json::Value = binary_json::from_slice(&binary_raw).unwrap();
    let from_text: serde_json::Value = serde_json::from_slice(&text_raw).unwrap();

    assert_eq!(
        from_binary, from_text,
        "an untyped read must not depend on the response serialization format",
    );

    // Pin the direction of agreement, so the comparison above cannot be
    // satisfied by both sides being wrong together.
    assert!(
        from_binary["small"].is_u64(),
        "an integral double must read back as an integer, not a float",
    );
    assert!(
        from_binary["negative"].is_i64(),
        "a negative integral double must read back as a signed integer",
    );
    assert!(
        from_binary["fractional"].is_f64(),
        "a fractional double must stay floating point",
    );
}

/// Builds a `POST .../docs` query request (`application/query+json`), optionally
/// advertising a response-format via `x-ms-cosmos-supported-serialization-formats`.
fn query_items_request(
    gateway_url: &str,
    db: &str,
    coll: &str,
    query: &str,
    serialization_formats: Option<&str>,
) -> Request {
    let url = format!("{}/dbs/{}/colls/{}/docs", gateway_url, db, coll);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);
    let body = serde_json::json!({ "query": query, "parameters": [] });
    req.set_body(serde_json::to_vec(&body).unwrap());
    req.headers_mut().insert(
        HeaderName::from_static("x-ms-documentdb-isquery"),
        HeaderValue::from_static("True"),
    );
    req.headers_mut().insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/query+json"),
    );
    req.headers_mut().insert(
        HeaderName::from_static("x-ms-documentdb-query-enablecrosspartition"),
        HeaderValue::from_static("True"),
    );
    if let Some(formats) = serialization_formats {
        req.headers_mut().insert(
            SUPPORTED_SERIALIZATION_FORMATS.clone(),
            HeaderValue::from(formats.to_string()),
        );
    }
    req
}

/// A query advertising `CosmosBinary` gets a binary feed body, asserted on the
/// raw wire bytes — the response half of query negotiation.
#[tokio::test]
async fn binary_query_yields_binary_response() {
    let ctx = setup_single_region().await;

    // Seed one item so the query returns a non-empty feed.
    let seed = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &serde_json::json!({ "id": "q-1", "pk": "pk1", "value": 11 }),
        r#"["pk1"]"#,
        false,
    );
    let seed_resp = ctx.emulator.execute_request(&seed).await.unwrap();
    let (seed_status, _h, _b) = collect_raw_response(seed_resp).await;
    assert_eq!(seed_status, StatusCode::Created);

    let req = query_items_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "SELECT * FROM c",
        Some("CosmosBinary"),
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _headers, raw) = collect_raw_response(response).await;

    assert_eq!(status, StatusCode::Ok);
    assert_eq!(
        raw.first(),
        Some(&PREAMBLE),
        "a query advertising CosmosBinary must return a binary (0x80) feed body",
    );
    assert!(
        binary_json::is_binary(&raw),
        "query response body must be detected as binary",
    );
    // The binary feed envelope decodes back to the seeded document.
    let decoded: serde_json::Value = binary_json::decode(&raw).unwrap();
    assert_eq!(decoded["Documents"][0]["id"], "q-1");
}

/// Counter-case: a `JsonText`-only query gets a text feed body.
#[tokio::test]
async fn jsontext_query_yields_text_response() {
    let ctx = setup_single_region().await;

    let seed = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &serde_json::json!({ "id": "q-2", "pk": "pk1", "value": 22 }),
        r#"["pk1"]"#,
        false,
    );
    let seed_resp = ctx.emulator.execute_request(&seed).await.unwrap();
    let (seed_status, _h, _b) = collect_raw_response(seed_resp).await;
    assert_eq!(seed_status, StatusCode::Created);

    let req = query_items_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "SELECT * FROM c",
        Some("JsonText"),
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _headers, raw) = collect_raw_response(response).await;

    assert_eq!(status, StatusCode::Ok);
    assert!(
        !binary_json::is_binary(&raw),
        "a JsonText query must return a text feed body",
    );
    let text = std::str::from_utf8(&raw).expect("text response must be valid UTF-8");
    let decoded: serde_json::Value = serde_json::from_str(text).unwrap();
    assert_eq!(decoded["Documents"][0]["id"], "q-2");
}
