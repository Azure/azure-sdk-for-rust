// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Query integration tests for the in-memory emulator.

use super::*;
use azure_core::http::headers::{HeaderName, HeaderValue, CONTENT_TYPE};
use azure_core::http::{Method, Request, StatusCode, Url};

static IS_QUERY: HeaderName = HeaderName::from_static("x-ms-documentdb-isquery");
static IS_QUERY_PLAN: HeaderName = HeaderName::from_static("x-ms-cosmos-is-query-plan-request");
static MAX_ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-max-item-count");
static CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");

fn query_request(gateway_url: &str, path: &str, body: serde_json::Value) -> Request {
    let url = format!("{}{}", gateway_url, path);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);
    req.set_body(serde_json::to_vec(&body).unwrap());
    req.headers_mut()
        .insert(IS_QUERY.clone(), HeaderValue::from_static("True"));
    req.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/query+json"),
    );
    req
}

#[tokio::test]
async fn query_items_filters_projects_and_paginates() {
    let ctx = setup_single_region().await;

    for (id, value) in [("item1", 1), ("item2", 2), ("item3", 3)] {
        let body = serde_json::json!({"id": id, "pk": "pk1", "score": value});
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

    let query = serde_json::json!({
        "query": "SELECT c.id FROM c WHERE c.pk = @pk AND c.score >= @min",
        "parameters": [
            {"name": "@pk", "value": "pk1"},
            {"name": "@min", "value": 2}
        ]
    });
    let mut req = query_request(&ctx.gateway_url, "/dbs/testdb/colls/testcoll/docs", query);
    req.headers_mut()
        .insert(MAX_ITEM_COUNT.clone(), HeaderValue::from_static("1"));

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, headers, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(headers.get_optional_str(&CONTINUATION), Some("1"));
    let docs = body["Documents"].as_array().unwrap();
    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0], serde_json::json!({"id": "item2"}));

    let query = serde_json::json!({
        "query": "SELECT c.id FROM c WHERE c.pk = @pk AND c.score >= @min",
        "parameters": [
            {"name": "@pk", "value": "pk1"},
            {"name": "@min", "value": 2}
        ]
    });
    let mut req = query_request(&ctx.gateway_url, "/dbs/testdb/colls/testcoll/docs", query);
    req.headers_mut()
        .insert(MAX_ITEM_COUNT.clone(), HeaderValue::from_static("1"));
    req.headers_mut()
        .insert(CONTINUATION.clone(), HeaderValue::from_static("1"));

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, headers, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert!(headers.get_optional_str(&CONTINUATION).is_none());
    let docs = body["Documents"].as_array().unwrap();
    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0], serde_json::json!({"id": "item3"}));
}

#[tokio::test]
async fn metadata_queries_filter_database_and_container_feeds() {
    let ctx = setup_single_region().await;

    let req = query_request(
        &ctx.gateway_url,
        "/dbs",
        serde_json::json!({
            "query": "SELECT * FROM c WHERE c.id = @id",
            "parameters": [{"name": "@id", "value": "testdb"}]
        }),
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    let databases = body["Databases"].as_array().unwrap();
    assert_eq!(databases.len(), 1);
    assert_eq!(databases[0]["id"], "testdb");

    let req = query_request(
        &ctx.gateway_url,
        "/dbs/testdb/colls",
        serde_json::json!({
            "query": "SELECT c.id FROM c WHERE c.id = @id",
            "parameters": [{"name": "@id", "value": "testcoll"}]
        }),
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    let containers = body["DocumentCollections"].as_array().unwrap();
    assert_eq!(containers.len(), 1);
    assert_eq!(containers[0], serde_json::json!({"id": "testcoll"}));
}

#[tokio::test]
async fn query_items_rejects_invalid_sql() {
    let ctx = setup_single_region().await;
    let req = query_request(
        &ctx.gateway_url,
        "/dbs/testdb/colls/testcoll/docs",
        serde_json::json!({"query": "SELECT FROM", "parameters": []}),
    );

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::BadRequest);
    let message = body["message"].as_str().unwrap_or("");
    assert!(
        message.contains("failed to parse query") || message.contains("parse"),
        "unexpected error body: {body}",
    );
}

#[tokio::test]
async fn query_plan_returns_gateway_shaped_local_plan() {
    let ctx = setup_single_region().await;
    let mut req = query_request(
        &ctx.gateway_url,
        "/dbs/testdb/colls/testcoll/docs",
        serde_json::json!({
            "query": "SELECT TOP @n * FROM c WHERE c.pk = @pk",
            "parameters": [
                {"name": "@n", "value": 2},
                {"name": "@pk", "value": "pk1"}
            ]
        }),
    );
    req.headers_mut()
        .insert(IS_QUERY_PLAN.clone(), HeaderValue::from_static("True"));

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(body["partitionedQueryExecutionInfoVersion"], 1);
    assert_eq!(body["queryInfo"]["top"], 2);
    assert_eq!(body["queryInfo"]["hasSelectValue"], false);
    let ranges = body["queryRanges"].as_array().unwrap();
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0]["min"], "");
    assert_eq!(ranges[0]["max"], "FF");
    assert_eq!(ranges[0]["isMinInclusive"], true);
    assert_eq!(ranges[0]["isMaxInclusive"], false);
}
