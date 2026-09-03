// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore pkmut
//! Point operation integration tests.

use super::*;
use azure_core::http::headers::HeaderValue;
use azure_data_cosmos_driver::{
    models::{AccountReference, CosmosOperation, ItemReference, PartitionKey},
    options::{DriverOptions, OperationOptions},
};

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
async fn item_id_with_literal_percent_round_trips_through_driver() {
    let ctx = setup_single_region().await;
    let runtime = ctx
        .emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build against the in-memory emulator");
    let account =
        AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5");
    let driver = runtime
        .create_driver(DriverOptions::builder(account).build())
        .await
        .expect("driver should initialize");
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container should resolve");
    let item_id = "item%41";
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), item_id.to_string());
    let body = serde_json::json!({"id": item_id, "pk": "pk1", "value": 42});

    driver
        .execute_singleton_operation(
            CosmosOperation::create_item(item).with_body(serde_json::to_vec(&body).unwrap()),
            OperationOptions::default(),
        )
        .await
        .expect("item should be created");

    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), item_id.to_string());
    let response = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(item),
            OperationOptions::default(),
        )
        .await
        .expect("literal percent item should be read");
    let bytes = response.into_body().single().expect("point read body");
    let document = parse_json_body(&bytes).expect("body should be JSON");

    assert_eq!(document["id"], item_id);
}

/// Guards the **default**: a driver operation with no binary option must still
/// put binary on the wire — the request body carries the `0x80` preamble and
/// advertises `CosmosBinary`. Without this, silently flipping the default off
/// passes every other wire-level test.
#[tokio::test]
async fn default_operation_negotiates_binary_on_the_wire() {
    #[derive(Debug, Default)]
    struct BinaryRequestRecorder {
        formats: std::sync::Mutex<Vec<Option<String>>>,
        body_is_binary: std::sync::Mutex<Vec<bool>>,
    }

    impl azure_data_cosmos_driver::in_memory_emulator::RequestObserver for BinaryRequestRecorder {
        fn on_request(&self, request: &Request) {
            if request.method() != Method::Post || !request.url().path().ends_with("/docs") {
                return;
            }
            let formats = request
                .headers()
                .get_optional_str(&SUPPORTED_SERIALIZATION_FORMATS)
                .map(str::to_string);
            self.formats.lock().unwrap().push(formats);
            let is_binary = matches!(
                request.body(),
                azure_core::http::request::Body::Bytes(bytes)
                    if azure_data_cosmos_driver::binary_json::is_binary(bytes)
            );
            self.body_is_binary.lock().unwrap().push(is_binary);
        }
    }

    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);
    let recorder = Arc::new(BinaryRequestRecorder::default());
    let emulator = Arc::new(
        InMemoryEmulatorHttpClient::new(config).with_request_observer(Arc::clone(&recorder)
            as Arc<dyn azure_data_cosmos_driver::in_memory_emulator::RequestObserver>),
    );
    let store = emulator.store();
    store.create_database("testdb");
    store.create_container(
        "testdb",
        "testcoll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );

    let runtime = emulator.runtime_builder().build().await.unwrap();
    let account =
        AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5");
    let driver = runtime
        .create_driver(DriverOptions::builder(account).build())
        .await
        .expect("driver should initialize");
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container should resolve");

    let body = serde_json::json!({"id": "d1", "pk": "pk1", "value": 1});
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "d1".to_string());
    driver
        .execute_singleton_operation(
            CosmosOperation::create_item(item).with_body(serde_json::to_vec(&body).unwrap()),
            OperationOptions::default(),
        )
        .await
        .expect("create must succeed");

    let formats = recorder.formats.lock().unwrap();
    let bodies = recorder.body_is_binary.lock().unwrap();
    assert!(!formats.is_empty(), "expected at least one docs request");
    assert!(
        formats.iter().all(|f| f.as_deref() == Some("CosmosBinary")),
        "default point op must advertise CosmosBinary; saw {formats:?}",
    );
    assert!(
        bodies.iter().all(|b| *b),
        "default point op request body must carry the 0x80 preamble",
    );
}

#[tokio::test]
async fn conditional_read_reports_item_lsn_not_partition_lsn() {
    let ctx = setup_single_region().await;

    let first = serde_json::json!({"id": "item1", "pk": "pk1", "value": 1});
    let response = ctx
        .emulator
        .execute_request(&create_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            &first,
            r#"["pk1"]"#,
            true,
        ))
        .await
        .unwrap();
    let (_, first_headers, first_body) = collect_response(response).await;
    let first_etag = first_body["_etag"].as_str().unwrap().to_owned();
    let first_item_lsn = first_headers
        .get_optional_str(&ITEM_LSN)
        .unwrap()
        .to_owned();

    let second = serde_json::json!({"id": "item2", "pk": "pk1", "value": 2});
    let response = ctx
        .emulator
        .execute_request(&create_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            &second,
            r#"["pk1"]"#,
            false,
        ))
        .await
        .unwrap();
    let (_, second_headers, _) = collect_response(response).await;
    assert_ne!(
        second_headers.get_optional_str(&LSN),
        Some(first_item_lsn.as_str()),
        "the second write must advance the partition LSN"
    );

    let mut request = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "item1",
        r#"["pk1"]"#,
    );
    request
        .headers_mut()
        .insert(IF_NONE_MATCH.clone(), HeaderValue::from(first_etag));
    let response = ctx.emulator.execute_request(&request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NotModified);
    assert_eq!(
        response.headers().get_optional_str(&ITEM_LSN),
        Some(first_item_lsn.as_str())
    );
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
