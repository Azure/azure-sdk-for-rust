// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response-format negotiation tests.
//!
//! These tests exercise the **response side** of Cosmos binary JSON
//! negotiation at the emulator: given a binary request body, what serialization
//! format does the service send *back*? The answer is driven entirely by the
//! `x-ms-cosmos-supported-serialization-formats` request header:
//!
//! - `JsonText,CosmosBinary` (what the SDK advertises whenever binary encoding
//!   is enabled) → the service replies with **binary** (body begins with the
//!   `0x80` preamble).
//! - `JsonText` alone → the service replies with **text**, even though the
//!   request body was binary.
//! - no header (binary encoding disabled) → **text**.
//!
//! Note the SDK-level `BinaryEncodingOptions::request_text_response` does **not**
//! send `JsonText` alone: it keeps advertising `CosmosBinary` (so the wire stays
//! binary) and has the *driver* transcode the binary response to text. These
//! tests cover the underlying emulator format decision that both modes rely on.
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

/// `request_text_response` (advertises only `JsonText`): the response body is
/// **text**, even though the request body was binary.
#[tokio::test]
async fn request_text_response_yields_text_response_despite_binary_request() {
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
