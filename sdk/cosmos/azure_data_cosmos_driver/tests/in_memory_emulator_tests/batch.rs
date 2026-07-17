// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Transactional batch integration tests for the in-memory emulator.

use super::*;
use azure_core::http::headers::{HeaderName, HeaderValue};
use azure_core::http::{Method, Request, StatusCode, Url};

static IS_BATCH: HeaderName = HeaderName::from_static("x-ms-cosmos-is-batch-request");
static LSN: HeaderName = HeaderName::from_static("lsn");

fn batch_request(gateway_url: &str, operations: serde_json::Value, pk: &str) -> Request {
    let url = format!("{}/dbs/testdb/colls/testcoll/docs", gateway_url);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);
    req.set_body(serde_json::to_vec(&operations).unwrap());
    req.headers_mut()
        .insert(IS_BATCH.clone(), HeaderValue::from_static("True"));
    req.headers_mut()
        .insert(PARTITION_KEY.clone(), HeaderValue::from(pk.to_string()));
    req.headers_mut()
        .insert(CONTENT_RESPONSE.clone(), HeaderValue::from_static("True"));
    req
}

#[tokio::test]
async fn batch_create_and_read_uses_one_lsn() {
    let ctx = setup_single_region().await;
    let operations = serde_json::json!([
        {"operationType": "Create", "resourceBody": {"id": "item1", "pk": "pk1", "value": 1}},
        {"operationType": "Create", "resourceBody": {"id": "item2", "pk": "pk1", "value": 2}},
        {"operationType": "Read", "id": "item1"}
    ]);

    let req = batch_request(&ctx.gateway_url, operations, r#"["pk1"]"#);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, headers, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(headers.get_optional_str(&LSN), Some("1"));

    let results = body.as_array().unwrap();
    let status_codes: Vec<u64> = results
        .iter()
        .map(|r| r["statusCode"].as_u64().unwrap())
        .collect();
    assert_eq!(status_codes, vec![201, 201, 200]);
    assert_eq!(results[2]["resourceBody"]["id"], "item1");

    let read1 = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        r#"["pk1"]"#,
    );
    let read2 = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item2",
        r#"["pk1"]"#,
    );
    let response1 = ctx.emulator.execute_request(&read1).await.unwrap();
    let response2 = ctx.emulator.execute_request(&read2).await.unwrap();
    let (status1, headers1, _) = collect_response(response1).await;
    let (status2, headers2, _) = collect_response(response2).await;
    assert_eq!(status1, StatusCode::Ok);
    assert_eq!(status2, StatusCode::Ok);
    assert_eq!(headers1.get_optional_str(&LSN), Some("1"));
    assert_eq!(headers2.get_optional_str(&LSN), Some("1"));
}

#[tokio::test]
async fn read_only_batch_does_not_advance_lsn() {
    let ctx = setup_single_region().await;
    let seed = serde_json::json!({"id": "item1", "pk": "pk1", "value": 1});
    let create = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &seed,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&create).await.unwrap();
    let (status, headers, _) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);
    assert_eq!(headers.get_optional_str(&LSN), Some("1"));

    let operations = serde_json::json!([
        {"operationType": "Read", "id": "item1"},
        {"operationType": "Read", "id": "item1"}
    ]);
    let req = batch_request(&ctx.gateway_url, operations, r#"["pk1"]"#);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, headers, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(headers.get_optional_str(&LSN), Some("1"));

    let results = body.as_array().unwrap();
    let status_codes: Vec<u64> = results
        .iter()
        .map(|r| r["statusCode"].as_u64().unwrap())
        .collect();
    assert_eq!(status_codes, vec![200, 200]);
    assert_eq!(results[0]["resourceBody"]["id"], "item1");
    assert_eq!(results[1]["resourceBody"]["id"], "item1");
}

#[tokio::test]
async fn batch_rolls_back_when_operation_fails() {
    let ctx = setup_single_region().await;
    let operations = serde_json::json!([
        {"operationType": "Create", "resourceBody": {"id": "item2", "pk": "pk1", "value": 2}},
        {"operationType": "Delete", "id": "missing"}
    ]);

    let req = batch_request(&ctx.gateway_url, operations, r#"["pk1"]"#);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::MultiStatus);
    let results = body.as_array().unwrap();
    let status_codes: Vec<u64> = results
        .iter()
        .map(|r| r["statusCode"].as_u64().unwrap())
        .collect();
    assert_eq!(status_codes, vec![424, 404]);

    let read = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item2",
        r#"["pk1"]"#,
    );
    let response = ctx.emulator.execute_request(&read).await.unwrap();
    assert_eq!(response.status(), StatusCode::NotFound);
}

#[tokio::test]
async fn batch_rejects_body_partition_key_mismatch() {
    let ctx = setup_single_region().await;
    let operations = serde_json::json!([
        {"operationType": "Create", "resourceBody": {"id": "item1", "pk": "different", "value": 1}}
    ]);

    let req = batch_request(&ctx.gateway_url, operations, r#"["pk1"]"#);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::BadRequest);
    assert_eq!(
        body["message"],
        "Transactional batch operations must use the batch partition key"
    );
}

#[tokio::test]
async fn batch_rejects_more_than_100_operations() {
    let ctx = setup_single_region().await;
    let operations: Vec<_> = (0..101)
        .map(|i| {
            serde_json::json!({
                "operationType": "Create",
                "resourceBody": {"id": format!("item{i}"), "pk": "pk1", "value": i}
            })
        })
        .collect();

    let req = batch_request(
        &ctx.gateway_url,
        serde_json::Value::Array(operations),
        r#"["pk1"]"#,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::BadRequest);
    assert_eq!(
        body["message"],
        "Transactional batch cannot exceed 100 operations"
    );
}
