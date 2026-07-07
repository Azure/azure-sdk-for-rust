// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Read-feed integration tests for the in-memory emulator.

use super::*;
use azure_core::http::headers::{HeaderName, HeaderValue};
use azure_core::http::{Method, Request, StatusCode, Url};

static MAX_ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-max-item-count");
static CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
static ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-item-count");

#[tokio::test]
async fn read_database_feed_lists_databases() {
    let ctx = setup_single_region().await;

    let url = format!("{}/dbs", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, headers, body) = collect_response(response).await;

    assert_eq!(status, StatusCode::Ok);
    assert_eq!(headers.get_optional_str(&ITEM_COUNT), Some("1"));
    let databases = body["Databases"].as_array().unwrap();
    assert_eq!(databases.len(), 1);
    assert_eq!(databases[0]["id"], "testdb");
    assert_eq!(body["_count"], 1);
}

#[tokio::test]
async fn read_container_feed_lists_containers() {
    let ctx = setup_single_region().await;

    let url = format!("{}/dbs/testdb/colls", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, headers, body) = collect_response(response).await;

    assert_eq!(status, StatusCode::Ok);
    assert_eq!(headers.get_optional_str(&ITEM_COUNT), Some("1"));
    let containers = body["DocumentCollections"].as_array().unwrap();
    assert_eq!(containers.len(), 1);
    assert_eq!(containers[0]["id"], "testcoll");
    assert!(containers[0].get("partitionKey").is_some());
    assert_eq!(body["_count"], 1);
}

#[tokio::test]
async fn read_document_feed_paginates_documents() {
    let ctx = setup_single_region().await;

    for id in ["item1", "item2", "item3"] {
        let body = serde_json::json!({"id": id, "pk": "pk1", "value": id});
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
    }

    let url = format!("{}/dbs/testdb/colls/testcoll/docs", ctx.gateway_url);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    req.headers_mut()
        .insert(MAX_ITEM_COUNT.clone(), HeaderValue::from_static("2"));

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, headers, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(headers.get_optional_str(&ITEM_COUNT), Some("2"));
    assert_eq!(headers.get_optional_str(&CONTINUATION), Some("2"));
    let documents = body["Documents"].as_array().unwrap();
    assert_eq!(documents.len(), 2);
    assert_eq!(documents[0]["id"], "item1");
    assert_eq!(documents[1]["id"], "item2");

    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    req.headers_mut()
        .insert(MAX_ITEM_COUNT.clone(), HeaderValue::from_static("2"));
    req.headers_mut()
        .insert(CONTINUATION.clone(), HeaderValue::from_static("2"));

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, headers, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(headers.get_optional_str(&ITEM_COUNT), Some("1"));
    assert!(headers.get_optional_str(&CONTINUATION).is_none());
    let documents = body["Documents"].as_array().unwrap();
    assert_eq!(documents.len(), 1);
    assert_eq!(documents[0]["id"], "item3");
}

#[tokio::test]
async fn read_document_feed_rejects_invalid_continuation() {
    let ctx = setup_single_region().await;

    let url = format!("{}/dbs/testdb/colls/testcoll/docs", ctx.gateway_url);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    req.headers_mut().insert(
        CONTINUATION.clone(),
        HeaderValue::from_static("not-a-number"),
    );

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::BadRequest);
    assert_eq!(body["message"], "Invalid continuation token");
}
