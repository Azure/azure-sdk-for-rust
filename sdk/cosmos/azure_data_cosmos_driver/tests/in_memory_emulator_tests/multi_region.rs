// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Multi-region integration tests.

use super::*;
use azure_core::http::HttpClient;

#[tokio::test]
async fn write_forbidden_403_3() {
    let ctx = setup_multi_region(WriteMode::Single).await;

    // Write to non-write region (West US) should return 403
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.west_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Forbidden);
    let substatus = response
        .headers()
        .get_optional_str(&SUBSTATUS)
        .unwrap_or("0");
    assert_eq!(substatus, "3");
}

#[tokio::test]
async fn write_to_write_region_succeeds() {
    let ctx = setup_multi_region(WriteMode::Single).await;

    // Write to write region (East US) should succeed
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.east_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);
}

#[tokio::test]
async fn multi_write_any_region() {
    let ctx = setup_multi_region(WriteMode::Multi).await;

    // Both regions should accept writes
    let body1 = serde_json::json!({"id": "item1", "pk": "pk1", "value": 1});
    let req = create_item_request(
        &ctx.east_url,
        "testdb",
        "testcoll",
        &body1,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);

    let body2 = serde_json::json!({"id": "item2", "pk": "pk2", "value": 2});
    let req = create_item_request(
        &ctx.west_url,
        "testdb",
        "testcoll",
        &body2,
        r#"["pk2"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);
}

#[tokio::test]
async fn immediate_replication_cross_region() {
    let ctx = setup_multi_region(WriteMode::Single).await;

    // Write to East US
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.east_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);

    // Read from West US — should be available due to immediate replication
    let req = read_item_request(&ctx.west_url, "testdb", "testcoll", "item1", r#"["pk1"]"#);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Ok);

    let doc = read_response_body(response).await;
    assert_eq!(doc["value"], 42);
}

#[tokio::test]
async fn account_properties_reflect_config() {
    let ctx = setup_multi_region(WriteMode::Single).await;

    let url = format!("{}/", ctx.east_url);
    let req = azure_core::http::Request::new(
        azure_core::http::Url::parse(&url).unwrap(),
        azure_core::http::Method::Get,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Ok);

    let body = read_response_body(response).await;
    assert_eq!(body["enableMultipleWriteLocations"], false);

    let readable = body["readableLocations"].as_array().unwrap();
    assert_eq!(readable.len(), 2);

    let writable = body["writableLocations"].as_array().unwrap();
    assert_eq!(writable.len(), 1);
    assert_eq!(writable[0]["name"], "East US");
}

#[tokio::test]
async fn pause_resume_replication() {
    let ctx = setup_multi_region(WriteMode::Single).await;

    // Pause replication to West US
    ctx.emulator.store().pause_replication("West US");

    // Write to East US
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.east_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);

    // Read from West US — should NOT be available (replication paused)
    let req = read_item_request(&ctx.west_url, "testdb", "testcoll", "item1", r#"["pk1"]"#);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NotFound);

    // Resume replication — should drain the buffer
    ctx.emulator.store().resume_replication("West US");

    // Now read from West US should succeed
    let req = read_item_request(&ctx.west_url, "testdb", "testcoll", "item1", r#"["pk1"]"#);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Ok);

    let doc = read_response_body(response).await;
    assert_eq!(doc["value"], 42);
}
