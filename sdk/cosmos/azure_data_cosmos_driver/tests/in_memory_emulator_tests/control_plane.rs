// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Control-plane integration tests (database/container/PKRanges CRUD).

use super::*;
use azure_core::http::{headers::HeaderValue, HttpClient, Method, Request, StatusCode, Url};

#[tokio::test]
async fn create_database() {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )]);
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));

    let url = format!("{}/dbs", GATEWAY_URL);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);
    req.set_body(serde_json::to_vec(&serde_json::json!({"id": "mydb"})).unwrap());

    let response = emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);

    let body = read_response_body(response).await;
    assert_eq!(body["id"], "mydb");
    assert!(body.get("_rid").is_some());
    assert!(body.get("_etag").is_some());
}

#[tokio::test]
async fn read_database() {
    let ctx = setup_single_region().await;

    let url = format!("{}/dbs/testdb", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Ok);

    let body = read_response_body(response).await;
    assert_eq!(body["id"], "testdb");
}

#[tokio::test]
async fn delete_database_cascades() {
    let ctx = setup_single_region().await;

    // Create an item first
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

    // Delete database
    let url = format!("{}/dbs/testdb", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Delete);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NoContent);

    // Database should not exist anymore
    let url = format!("{}/dbs/testdb", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NotFound);
}

#[tokio::test]
async fn create_container_with_pk() {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )]);
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    emulator.store().create_database("testdb");

    let url = format!("{}/dbs/testdb/colls", GATEWAY_URL);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);
    req.set_body(
        serde_json::to_vec(&serde_json::json!({
            "id": "mycoll",
            "partitionKey": {
                "paths": ["/pk"],
                "kind": "Hash",
                "version": 2
            }
        }))
        .unwrap(),
    );

    let response = emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);

    let body = read_response_body(response).await;
    assert_eq!(body["id"], "mycoll");
    assert!(body.get("partitionKey").is_some());
}

#[tokio::test]
async fn read_container() {
    let ctx = setup_single_region().await;

    let url = format!("{}/dbs/testdb/colls/testcoll", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Ok);

    let body = read_response_body(response).await;
    assert_eq!(body["id"], "testcoll");
    assert!(body.get("partitionKey").is_some());
}

#[tokio::test]
async fn delete_container_cascades() {
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

    // Delete container
    let url = format!("{}/dbs/testdb/colls/testcoll", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Delete);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NoContent);

    // Container should be gone
    let url = format!("{}/dbs/testdb/colls/testcoll", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NotFound);
}

#[tokio::test]
async fn create_container_missing_pk_400() {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )]);
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    emulator.store().create_database("testdb");

    let url = format!("{}/dbs/testdb/colls", GATEWAY_URL);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);
    req.set_body(serde_json::to_vec(&serde_json::json!({"id": "nocoll"})).unwrap());

    let response = emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BadRequest);
}

#[tokio::test]
async fn read_nonexistent_database_404() {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )]);
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));

    let url = format!("{}/dbs/nope", GATEWAY_URL);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NotFound);
}

#[tokio::test]
async fn read_pkranges() {
    let ctx = setup_single_region().await;

    let url = format!("{}/dbs/testdb/colls/testcoll/pkranges", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Ok);

    let body = read_response_body(response).await;
    let ranges = body["PartitionKeyRanges"].as_array().unwrap();
    assert_eq!(ranges.len(), 4); // default 4 partitions
    assert_eq!(ranges[0]["id"], "0");
    assert_eq!(ranges[0]["status"], "online");
    assert!(ranges[0].get("minInclusive").is_some());
    assert!(ranges[0].get("maxExclusive").is_some());
}
