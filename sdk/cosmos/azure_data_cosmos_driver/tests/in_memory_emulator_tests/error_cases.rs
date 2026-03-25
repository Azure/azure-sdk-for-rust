// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Error case integration tests (404, 409, 412, 404/1002).

use super::*;
use azure_core::http::HttpClient;

#[tokio::test]
async fn read_nonexistent_404() {
    let ctx = setup_single_region().await;
    let req = read_item_request(&ctx.gateway_url, "testdb", "testcoll", "nope", r#"["pk1"]"#);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NotFound);
}

#[tokio::test]
async fn create_duplicate_409() {
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
    assert_eq!(response.status(), StatusCode::Created);

    // Duplicate
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Conflict);
}

#[tokio::test]
async fn replace_nonexistent_404() {
    let ctx = setup_single_region().await;

    let body = serde_json::json!({"id": "nope", "pk": "pk1", "value": 1});
    let req = replace_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "nope",
        &body,
        r#"["pk1"]"#,
        None,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NotFound);
}

#[tokio::test]
async fn delete_nonexistent_404() {
    let ctx = setup_single_region().await;
    let req = delete_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "nope",
        r#"["pk1"]"#,
        None,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NotFound);
}

#[tokio::test]
async fn replace_stale_etag_412() {
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

    // Replace with stale etag
    let new_body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 99});
    let req = replace_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        &new_body,
        r#"["pk1"]"#,
        Some("\"stale-etag\""),
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::PreconditionFailed);
}

#[tokio::test]
async fn forced_session_not_available() {
    let ctx = setup_single_region().await;

    // Create an item
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

    // Force session not available
    ctx.emulator
        .store()
        .force_session_not_available("East US", r#"["pk1"]"#);

    // Read should return 404 with substatus 1002
    let req = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        r#"["pk1"]"#,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NotFound);
    let substatus = response
        .headers()
        .get_optional_str(&SUBSTATUS)
        .unwrap_or("0");
    assert_eq!(substatus, "1002");

    // One-shot: next read should succeed
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
async fn session_not_available_404_1002() {
    let ctx = setup_single_region().await;

    // Request with a session token ahead of current LSN
    let req = {
        let url = format!("{}/dbs/testdb/colls/testcoll/docs/item1", ctx.gateway_url);
        let mut req = azure_core::http::Request::new(
            azure_core::http::Url::parse(&url).unwrap(),
            azure_core::http::Method::Get,
        );
        req.headers_mut().insert(
            PARTITION_KEY.clone(),
            azure_core::http::headers::HeaderValue::from(r#"["pk1"]"#.to_string()),
        );
        // Set session token ahead of current LSN (partition 0, LSN 999)
        req.headers_mut().insert(
            SESSION_TOKEN.clone(),
            azure_core::http::headers::HeaderValue::from("0:-1#999".to_string()),
        );
        req
    };

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NotFound);
    let substatus = response
        .headers()
        .get_optional_str(&SUBSTATUS)
        .unwrap_or("0");
    assert_eq!(substatus, "1002");
}
