// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Query integration tests for the in-memory emulator.

use super::*;
use azure_core::http::headers::{HeaderName, HeaderValue, CONTENT_TYPE};
use azure_core::http::{Method, Request, StatusCode, Url};
use azure_data_cosmos_driver::models::{FeedRange, PartitionKey, PartitionKeyDefinition};

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
async fn non_session_query_response_preserves_incoming_region_progress() {
    let ctx = setup_single_region().await;
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "score": 1});
    let create = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let created = ctx.emulator.execute_request(&create).await.unwrap();
    let created_token = created
        .headers()
        .get_optional_str(&SESSION_TOKEN)
        .expect("create should return a session token");
    let (partition_range_id, token_value) = created_token
        .split_once(':')
        .expect("session token should include a partition range");
    let mut token_parts = token_value.split('#');
    let version = token_parts.next().expect("token should include a version");
    let global_lsn = token_parts
        .next()
        .expect("token should include a global LSN");
    let incoming_token = format!("{partition_range_id}:{version}#{global_lsn}#999=123");

    let query = serde_json::json!({
        "query": "SELECT * FROM c",
        "parameters": []
    });
    let mut request = query_request(&ctx.gateway_url, "/dbs/testdb/colls/testcoll/docs", query);
    request.headers_mut().insert(
        PARTITION_KEY.clone(),
        HeaderValue::from_static(r#"["pk1"]"#),
    );
    request
        .headers_mut()
        .insert(SESSION_TOKEN.clone(), HeaderValue::from(incoming_token));
    request.headers_mut().insert(
        HeaderName::from_static("x-ms-cosmos-read-consistency-strategy"),
        HeaderValue::from_static("LatestCommitted"),
    );

    let response = ctx.emulator.execute_request(&request).await.unwrap();
    assert_eq!(response.status(), StatusCode::Ok);
    let response_token = response
        .headers()
        .get_optional_str(&SESSION_TOKEN)
        .expect("query should return a session token");
    assert!(
        response_token.contains("#999=123"),
        "non-Session feed response must preserve incoming region progress: {response_token}"
    );
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
    let continuation = headers
        .get_optional_str(&CONTINUATION)
        .expect("first page should return a continuation")
        .to_owned();
    assert!(continuation.contains("document_feed_cursor_v1"));
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
        .insert(CONTINUATION.clone(), HeaderValue::from(continuation));

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
    assert_eq!(body["partitionedQueryExecutionInfoVersion"], 2);
    assert_eq!(body["queryInfo"]["top"], 2);
    assert_eq!(body["queryInfo"]["dCountInfo"], serde_json::Value::Null);
    assert_eq!(body["queryInfo"]["rewrittenQuery"], "");
    assert_eq!(body["queryInfo"]["hasSelectValue"], false);
    let ranges = body["queryRanges"].as_array().unwrap();
    assert_eq!(ranges.len(), 1);
    let pk_def: PartitionKeyDefinition = "/pk".into();
    let expected_range = FeedRange::for_partition(PartitionKey::from("pk1"), &pk_def);
    assert_eq!(ranges[0]["min"], expected_range.min_inclusive().to_hex());
    assert_eq!(ranges[0]["max"], expected_range.max_exclusive().to_hex());
    assert_eq!(ranges[0]["isMinInclusive"], true);
    assert_eq!(ranges[0]["isMaxInclusive"], true);
}

#[tokio::test]
async fn query_plan_returns_hpk_prefix_range_for_partial_where_clause() {
    let ctx = setup_single_region().await;
    let hpk_def: PartitionKeyDefinition = ("/tenant", "/user", "/session").into();
    ctx.emulator
        .store()
        .create_container("testdb", "hpk-coll", hpk_def.clone());

    let mut req = query_request(
        &ctx.gateway_url,
        "/dbs/testdb/colls/hpk-coll/docs",
        serde_json::json!({
            "query": "SELECT * FROM c WHERE c.tenant = @tenant",
            "parameters": [
                {"name": "@tenant", "value": "tenant-a"}
            ]
        }),
    );
    req.headers_mut()
        .insert(IS_QUERY_PLAN.clone(), HeaderValue::from_static("True"));

    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    let ranges = body["queryRanges"].as_array().unwrap();
    assert_eq!(ranges.len(), 1);
    let expected_range = FeedRange::for_partition(PartitionKey::from("tenant-a"), &hpk_def);
    assert_eq!(ranges[0]["min"], expected_range.min_inclusive().to_hex());
    assert_eq!(ranges[0]["max"], expected_range.max_exclusive().to_hex());
    assert_eq!(ranges[0]["isMinInclusive"], true);
    assert_eq!(ranges[0]["isMaxInclusive"], true);
}
