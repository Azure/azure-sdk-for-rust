// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Offer API integration tests for the in-memory emulator.

use super::*;
use azure_core::http::headers::{HeaderName, HeaderValue, CONTENT_TYPE};
use azure_core::http::{Method, Request, StatusCode, Url};

static IS_QUERY: HeaderName = HeaderName::from_static("x-ms-documentdb-isquery");
static OFFER_THROUGHPUT: HeaderName = HeaderName::from_static("x-ms-offer-throughput");
static OFFER_REPLACE_PENDING: HeaderName = HeaderName::from_static("x-ms-offer-replace-pending");

fn query_offers_request(gateway_url: &str, resource_id: &str) -> Request {
    let url = format!("{}/offers", gateway_url);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);
    req.set_body(
        serde_json::to_vec(&serde_json::json!({
            "query": "SELECT * FROM c WHERE c.offerResourceId = @rid",
            "parameters": [{"name": "@rid", "value": resource_id}]
        }))
        .unwrap(),
    );
    req.headers_mut()
        .insert(IS_QUERY.clone(), HeaderValue::from_static("True"));
    req.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/query+json"),
    );
    req
}

#[tokio::test]
async fn offer_query_read_and_replace_round_trip() {
    let ctx = setup_single_region().await;

    let url = format!("{}/dbs/testdb/colls", ctx.gateway_url);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);
    req.set_body(
        serde_json::to_vec(&serde_json::json!({
            "id": "throughput_collection",
            "partitionKey": {
                "paths": ["/pk"],
                "kind": "Hash",
                "version": 2
            }
        }))
        .unwrap(),
    );
    req.headers_mut()
        .insert(OFFER_THROUGHPUT.clone(), HeaderValue::from_static("400"));
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, container) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);
    let container_rid = container["_rid"].as_str().unwrap();

    let req = query_offers_request(&ctx.gateway_url, container_rid);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    let offers = body["Offers"].as_array().unwrap();
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0]["offerResourceId"], container_rid);
    assert_eq!(offers[0]["content"]["offerThroughput"], 400);
    let offer_id = offers[0]["id"].as_str().unwrap();

    let url = format!("{}/offers/{}", ctx.gateway_url, offer_id);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, offer) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(offer["id"], offer_id);
    assert_eq!(offer["content"]["offerThroughput"], 400);

    let mut updated = offer.clone();
    updated["content"]["offerThroughput"] = serde_json::json!(500);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Put);
    req.set_body(serde_json::to_vec(&updated).unwrap());
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, headers, offer) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(
        headers.get_optional_str(&OFFER_REPLACE_PENDING),
        Some("false")
    );
    assert_eq!(offer["content"]["offerThroughput"], 500);
}
