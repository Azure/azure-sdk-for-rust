// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Point operation integration tests.

use super::*;
use azure_core::http::headers::HeaderValue;
use azure_core::http::HttpClient;

#[tokio::test]
async fn create_new_item() {
    let ctx = setup_single_region().await;
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        true,
    );

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, headers, doc) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);

    assert!(headers.get_optional_str(&ETAG).is_some());
    assert!(headers.get_optional_str(&REQUEST_CHARGE).is_some());
    assert!(headers.get_optional_str(&SESSION_TOKEN).is_some());

    assert_eq!(doc["id"], "item1");
    assert_eq!(doc["value"], 42);
    assert!(doc.get("_rid").is_some());
    assert!(doc.get("_etag").is_some());
    assert!(doc.get("_ts").is_some());
    assert!(doc.get("_self").is_some());
}

#[tokio::test]
async fn read_existing_item() {
    let ctx = setup_single_region().await;

    // Create first
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);

    // Read
    let req = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        r#"["pk1"]"#,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, doc) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(doc["id"], "item1");
    assert_eq!(doc["value"], 42);
    assert!(doc.get("_rid").is_some());
    assert!(doc.get("_etag").is_some());
}

#[tokio::test]
async fn replace_existing_item() {
    let ctx = setup_single_region().await;

    // Create
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        true,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (_, _, doc) = collect_response(response).await;
    let etag = doc["_etag"].as_str().unwrap().to_string();

    // Replace
    let new_body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 99});
    let req = replace_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        &new_body,
        r#"["pk1"]"#,
        Some(&etag),
        true,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, replaced) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);

    assert_eq!(replaced["value"], 99);
    assert_ne!(replaced["_etag"].as_str().unwrap(), &etag);
}

#[tokio::test]
async fn replace_rejects_body_id_mismatch() {
    let ctx = setup_single_region().await;

    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        true,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (_, _, created) = collect_response(response).await;
    let etag = created["_etag"].as_str().unwrap().to_string();

    let replacement = serde_json::json!({"id": "item2", "pk": "pk1", "value": 99});
    let req = replace_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        &replacement,
        r#"["pk1"]"#,
        Some(&etag),
        true,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::BadRequest);
    assert_eq!(
        body["message"],
        "Document id in request body must match the resource id in the request URI"
    );

    let req = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        r#"["pk1"]"#,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, doc) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(doc["id"], "item1");
    assert_eq!(doc["value"], 42);
}

#[tokio::test]
async fn replace_rejects_partition_key_mutation() {
    // Replacing an item with a body whose partition-key value differs from
    // the existing item's PK must fail with 400 BadRequest. Without this
    // guard the new body could route to a different physical partition while
    // the original document remained orphaned on the old partition (silent
    // divergence).
    let ctx = setup_single_region().await;

    let body = serde_json::json!({"id": "item-pkmut", "pk": "pk-original", "value": 1});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk-original"]"#,
        true,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, created) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);
    let etag = created["_etag"].as_str().unwrap().to_string();

    // Realistic PK-mutation attempt: header carries the EXISTING PK so
    // the request routes to the right partition and the existing doc is
    // located, but the body's pk field disagrees. Real Cosmos rejects
    // this with 400 BadRequest because partition-key values are immutable
    // on Replace.
    let replacement = serde_json::json!({"id": "item-pkmut", "pk": "pk-different", "value": 2});
    let req = replace_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item-pkmut",
        &replacement,
        r#"["pk-original"]"#,
        Some(&etag),
        true,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(
        status,
        StatusCode::BadRequest,
        "PK mutation must be rejected; got body={body}",
    );
    let msg = body["message"].as_str().unwrap_or("");
    assert!(
        msg.contains("Partition key") || msg.contains("partition key"),
        "error message should mention partition key, got: {msg}",
    );

    // Original document must still be readable on its original PK.
    let req = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item-pkmut",
        r#"["pk-original"]"#,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, doc) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(doc["pk"], "pk-original");
    assert_eq!(doc["value"], 1);
}

#[tokio::test]
async fn echoes_request_activity_id() {
    let ctx = setup_single_region().await;

    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let mut req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        true,
    );
    req.headers_mut().insert(
        ACTIVITY_ID.clone(),
        HeaderValue::from("test-activity-id".to_string()),
    );

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, headers, _) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);
    assert_eq!(
        headers.get_optional_str(&ACTIVITY_ID),
        Some("test-activity-id")
    );
}

#[tokio::test]
async fn upsert_new_item() {
    let ctx = setup_single_region().await;

    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = upsert_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        true,
    );

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, doc) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);
    assert_eq!(doc["id"], "item1");
    assert_eq!(doc["value"], 42);
}

#[tokio::test]
async fn upsert_existing_item() {
    let ctx = setup_single_region().await;

    // Create via upsert
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = upsert_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);

    // Update via upsert
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 99});
    let req = upsert_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        true,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, doc) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(doc["value"], 99);
}

#[tokio::test]
async fn upsert_without_content_response() {
    let ctx = setup_single_region().await;

    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = upsert_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, response_body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);
    assert_eq!(response_body, serde_json::Value::Null);

    let req = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        r#"["pk1"]"#,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, doc) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(doc["value"], 42);
}

#[tokio::test]
async fn delete_existing_item() {
    let ctx = setup_single_region().await;

    // Create
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);

    // Delete
    let req = delete_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        r#"["pk1"]"#,
        None,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NoContent);

    // Verify deleted
    let req = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        r#"["pk1"]"#,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NotFound);
}

#[tokio::test]
async fn create_without_content_response() {
    let ctx = setup_single_region().await;

    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);
    assert_eq!(body, serde_json::Value::Null);

    // But the item should still exist
    let req = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        r#"["pk1"]"#,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Ok);
}

#[tokio::test]
async fn replace_without_content_response() {
    let ctx = setup_single_region().await;

    // Create with content response to get the etag
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        true,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (_, _, doc) = collect_response(response).await;
    let etag = doc["_etag"].as_str().unwrap().to_string();

    // Replace without content response
    let new_body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 99});
    let req = replace_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        &new_body,
        r#"["pk1"]"#,
        Some(&etag),
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(body, serde_json::Value::Null);
}
