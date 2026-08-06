// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Distributed transaction integration tests for the in-memory emulator.

use super::*;

static DTX_IDEMPOTENCY_TOKEN: HeaderName = HeaderName::from_static("x-ms-cosmos-idempotency-token");
static DTX_OPERATION_TYPE: HeaderName = HeaderName::from_static("x-ms-cosmos-operation-type");
static DTX_RESOURCE_TYPE: HeaderName = HeaderName::from_static("x-ms-cosmos-resource-type");
const DTX_RESOURCE_TYPE_VALUE: &str = "DistributedTransactionBatch";

fn dtx_request(gateway_url: &str, operations: serde_json::Value) -> Request {
    let url = format!("{gateway_url}/operations/dtc");
    let mut request = Request::new(Url::parse(&url).unwrap(), Method::Post);
    // The emulator requires the same DTX request headers as the live coordinator
    // (idempotency token, resource-type, operation-type). Derive the
    // operation-type from the payload so the declared transaction type matches
    // the operations: any non-`Read` operation makes it a write commit.
    let is_write = operations["operations"].as_array().is_some_and(|ops| {
        ops.iter().any(|op| {
            !op["operationType"]
                .as_str()
                .unwrap_or("")
                .eq_ignore_ascii_case("Read")
        })
    });
    let operation_type = if is_write {
        "CommitDistributedTransaction"
    } else {
        "Read"
    };
    request.headers_mut().insert(
        DTX_IDEMPOTENCY_TOKEN.clone(),
        HeaderValue::from_static("3f2504e0-4f89-41d3-9a0c-0305e82c3301"),
    );
    request.headers_mut().insert(
        DTX_RESOURCE_TYPE.clone(),
        HeaderValue::from_static(DTX_RESOURCE_TYPE_VALUE),
    );
    request.headers_mut().insert(
        DTX_OPERATION_TYPE.clone(),
        HeaderValue::from_static(operation_type),
    );
    request.set_body(serde_json::to_vec(&operations).unwrap());
    request
}

#[tokio::test]
async fn dtx_create_then_read() {
    let ctx = setup_single_region().await;
    let create_body = serde_json::json!({
        "operations": [{
            "databaseName": "testdb",
            "collectionName": "testcoll",
            "id": "item1",
            "partitionKey": ["pk1"],
            "index": 0,
            "resourceBody": {"id": "item1", "pk": "pk1", "value": 42},
            "operationType": "Create",
            "resourceType": "Document"
        }]
    });

    let response = ctx
        .emulator
        .execute_request(&dtx_request(&ctx.gateway_url, create_body))
        .await
        .unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    let results = body["operationResponses"].as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["index"], 0);
    assert_eq!(results[0]["statusCode"], 201);
    assert!(results[0]["sessionToken"].as_str().is_some());
    assert!(results[0].get("resourceBody").is_none());

    let read_body = serde_json::json!({
        "operations": [{
            "databaseName": "testdb",
            "collectionName": "testcoll",
            "id": "item1",
            "partitionKey": ["pk1"],
            "index": 0,
            "operationType": "Read",
            "resourceType": "Document"
        }]
    });

    let response = ctx
        .emulator
        .execute_request(&dtx_request(&ctx.gateway_url, read_body))
        .await
        .unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    let result = &body["operationResponses"][0];
    assert_eq!(result["index"], 0);
    assert_eq!(result["statusCode"], 200);
    assert_eq!(result["resourceBody"]["id"], "item1");
    assert_eq!(result["resourceBody"]["value"], 42);
}

#[tokio::test]
async fn dtx_failed_write_does_not_partially_commit() {
    let ctx = setup_single_region().await;
    let body = serde_json::json!({
        "operations": [
            {
                "databaseName": "testdb",
                "collectionName": "testcoll",
                "id": "created-before-failure",
                "partitionKey": ["pk1"],
                "index": 0,
                "resourceBody": {"id": "created-before-failure", "pk": "pk1", "value": 1},
                "operationType": "Create",
                "resourceType": "Document"
            },
            {
                "databaseName": "testdb",
                "collectionName": "testcoll",
                "id": "missing-doc",
                "partitionKey": ["pk1"],
                "index": 1,
                "operationType": "Delete",
                "resourceType": "Document"
            }
        ]
    });

    let response = ctx
        .emulator
        .execute_request(&dtx_request(&ctx.gateway_url, body))
        .await
        .unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::from(452_u16));
    let results = body["operationResponses"].as_array().unwrap();
    assert_eq!(results.len(), 2);
    // The create voted "Yes" (prepared) but was rolled back: 453 / 5415.
    assert_eq!(results[0]["statusCode"], 453);
    assert_eq!(results[0]["subStatusCode"], 5415);
    // The delete of a missing document voted "No": it keeps its real code.
    assert_eq!(results[1]["statusCode"], 404);

    let response = ctx
        .emulator
        .execute_request(&read_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            "created-before-failure",
            r#"["pk1"]"#,
        ))
        .await
        .unwrap();
    let (status, _, _) = collect_response(response).await;
    assert_eq!(status, StatusCode::NotFound);
}

#[tokio::test]
async fn dtx_read_snapshot_failure_rewrites_successful_reads() {
    let ctx = setup_single_region().await;

    // Seed one document; the second read targets a document that does not exist.
    let create = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &serde_json::json!({"id": "present", "pk": "pk1", "value": 7}),
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&create).await.unwrap();
    let (status, _, _) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);

    let read_body = serde_json::json!({
        "operations": [
            {
                "databaseName": "testdb",
                "collectionName": "testcoll",
                "id": "present",
                "partitionKey": ["pk1"],
                "index": 0,
                "operationType": "Read",
                "resourceType": "Document"
            },
            {
                "databaseName": "testdb",
                "collectionName": "testcoll",
                "id": "absent",
                "partitionKey": ["pk1"],
                "index": 1,
                "operationType": "Read",
                "resourceType": "Document"
            }
        ]
    });

    let response = ctx
        .emulator
        .execute_request(&dtx_request(&ctx.gateway_url, read_body))
        .await
        .unwrap();
    let (status, _, body) = collect_response(response).await;

    // A single distinct failure code (404) promotes to the envelope; the read
    // that individually succeeded is rewritten to 424 FailedDependency with its
    // body stripped so no unconfirmed snapshot data leaks to the caller.
    assert_eq!(status, StatusCode::NotFound);
    assert_eq!(body["isRetriable"], false);
    let results = body["operationResponses"].as_array().unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0]["statusCode"], 424);
    assert!(results[0].get("resourceBody").is_none());
    assert_eq!(results[1]["statusCode"], 404);
}

#[tokio::test]
async fn dtx_patch_with_filter_predicate_updates_item() {
    let ctx = setup_single_region().await;

    let create = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &serde_json::json!({"id": "patch-target", "pk": "pk1", "status": "pending"}),
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&create).await.unwrap();
    let (status, _, _) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);

    let patch_body = serde_json::json!({
        "operations": [{
            "databaseName": "testdb",
            "collectionName": "testcoll",
            "id": "patch-target",
            "partitionKey": ["pk1"],
            "index": 0,
            "operationType": "Patch",
            "resourceType": "Document",
            "resourceBody": {
                "condition": "from c where c.status = 'pending'",
                "operations": [{"op": "set", "path": "/status", "value": "done"}]
            }
        }]
    });

    let response = ctx
        .emulator
        .execute_request(&dtx_request(&ctx.gateway_url, patch_body))
        .await
        .unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    let result = &body["operationResponses"][0];
    assert_eq!(result["statusCode"], 200);
    assert!(result.get("resourceBody").is_none());

    let response = ctx
        .emulator
        .execute_request(&read_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            "patch-target",
            r#"["pk1"]"#,
        ))
        .await
        .unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::Ok);
    assert_eq!(body["status"], "done");
}

#[tokio::test]
async fn dtx_patch_filter_failure_rolls_back_siblings() {
    let ctx = setup_single_region().await;

    let create = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &serde_json::json!({"id": "patch-target", "pk": "pk1", "status": "done"}),
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&create).await.unwrap();
    let (status, _, _) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);

    let body = serde_json::json!({
        "operations": [
            {
                "databaseName": "testdb",
                "collectionName": "testcoll",
                "id": "rolled-back-create",
                "partitionKey": ["pk1"],
                "index": 0,
                "resourceBody": {"id": "rolled-back-create", "pk": "pk1", "value": 1},
                "operationType": "Create",
                "resourceType": "Document"
            },
            {
                "databaseName": "testdb",
                "collectionName": "testcoll",
                "id": "patch-target",
                "partitionKey": ["pk1"],
                "index": 1,
                "operationType": "Patch",
                "resourceType": "Document",
                "resourceBody": {
                    "condition": "from c where c.status = 'pending'",
                    "operations": [{"op": "set", "path": "/status", "value": "patched"}]
                }
            }
        ]
    });

    let response = ctx
        .emulator
        .execute_request(&dtx_request(&ctx.gateway_url, body))
        .await
        .unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::from(452_u16));
    let results = body["operationResponses"].as_array().unwrap();
    assert_eq!(results[0]["statusCode"], 453);
    assert_eq!(results[0]["subStatusCode"], 5415);
    assert_eq!(results[1]["statusCode"], 412);
    assert_eq!(results[1]["subStatusCode"], 1110);

    let response = ctx
        .emulator
        .execute_request(&read_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            "rolled-back-create",
            r#"["pk1"]"#,
        ))
        .await
        .unwrap();
    let (status, _, _) = collect_response(response).await;
    assert_eq!(status, StatusCode::NotFound);
}

#[tokio::test]
async fn dtx_create_body_id_mismatch_aborts_without_commit() {
    let ctx = setup_single_region().await;
    let body = serde_json::json!({
        "operations": [{
            "databaseName": "testdb",
            "collectionName": "testcoll",
            "id": "outer-id",
            "partitionKey": ["pk1"],
            "index": 0,
            "resourceBody": {"id": "inner-id", "pk": "pk1", "value": 1},
            "operationType": "Create",
            "resourceType": "Document"
        }]
    });

    let response = ctx
        .emulator
        .execute_request(&dtx_request(&ctx.gateway_url, body))
        .await
        .unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::from(452_u16));
    assert_eq!(body["operationResponses"][0]["statusCode"], 400);

    let response = ctx
        .emulator
        .execute_request(&read_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            "inner-id",
            r#"["pk1"]"#,
        ))
        .await
        .unwrap();
    let (status, _, _) = collect_response(response).await;
    assert_eq!(status, StatusCode::NotFound);
}

#[tokio::test]
async fn dtx_all_not_modified_read_is_completed() {
    let ctx = setup_single_region().await;
    let create = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &serde_json::json!({"id": "item-with-etag", "pk": "pk1", "value": 1}),
        r#"["pk1"]"#,
        true,
    );
    let response = ctx.emulator.execute_request(&create).await.unwrap();
    let (status, headers, _) = collect_response(response).await;
    assert_eq!(status, StatusCode::Created);
    let etag = headers.get_optional_str(&ETAG).unwrap();

    let read_body = serde_json::json!({
        "operations": [{
            "databaseName": "testdb",
            "collectionName": "testcoll",
            "id": "item-with-etag",
            "partitionKey": ["pk1"],
            "index": 0,
            "operationType": "Read",
            "resourceType": "Document",
            "ifNoneMatch": etag
        }]
    });

    let response = ctx
        .emulator
        .execute_request(&dtx_request(&ctx.gateway_url, read_body))
        .await
        .unwrap();
    let (status, _, body) = collect_response(response).await;
    assert_eq!(status, StatusCode::NotModified);
    assert_eq!(body["operationResponses"][0]["statusCode"], 304);
}
