// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Point operation integration tests.

use super::*;
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
