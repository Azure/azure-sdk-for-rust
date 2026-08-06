// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Multi-region integration tests.

use super::*;
use azure_core::http::headers::HeaderName;

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

/// Control-plane writes must honor the single-write-region rule too: database
/// and container create/delete plus offer replace all return `403 / 3
/// (WriteForbidden)` on a read-only region, matching data-plane writes and the
/// real service. The region guard short-circuits before any resource lookup,
/// so nonexistent ids still surface the region error rather than 404.
#[tokio::test]
async fn control_plane_write_forbidden_403_3_on_read_region() {
    use azure_core::http::{Method, Request, Url};

    let ctx = setup_multi_region(WriteMode::Single).await;
    let west = &ctx.west_url;

    // Each control-plane write verb, aimed at the read region.
    let cases = [
        (Method::Post, format!("{west}/dbs")),
        (Method::Delete, format!("{west}/dbs/testdb")),
        (Method::Post, format!("{west}/dbs/testdb/colls")),
        (Method::Delete, format!("{west}/dbs/testdb/colls/testcoll")),
        (Method::Put, format!("{west}/offers/some_offer")),
    ];

    for (method, url) in cases {
        let mut req = Request::new(Url::parse(&url).unwrap(), method);
        // A well-formed body keeps create requests valid; it is never parsed
        // because the write-region guard rejects the request first.
        req.set_body(serde_json::to_vec(&serde_json::json!({"id": "x"})).unwrap());
        let response = ctx.emulator.execute_request(&req).await.unwrap();
        assert_eq!(
            response.status(),
            StatusCode::Forbidden,
            "{url} must be forbidden on a read region"
        );
        let substatus = response
            .headers()
            .get_optional_str(&SUBSTATUS)
            .unwrap_or("0");
        assert_eq!(substatus, "3", "{url} must carry WriteForbidden substatus");
    }
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

/// Validates that a createItem sent to a read-only region returns 403 with
/// substatus 3, and that all expected Cosmos headers and error body fields
/// are present and correct.
#[tokio::test]
async fn create_item_on_read_region_returns_403_3_with_correct_headers() {
    let ctx = setup_multi_region(WriteMode::Single).await;

    // Header name constants for fields not in the shared test helpers.
    let activity_id: HeaderName = HeaderName::from_static("x-ms-activity-id");
    let request_charge: HeaderName = HeaderName::from_static("x-ms-request-charge");
    let version: HeaderName = HeaderName::from_static("x-ms-version");
    let content_type: HeaderName = HeaderName::from_static("content-type");
    let date: HeaderName = HeaderName::from_static("date");
    let server_duration_ms: HeaderName = HeaderName::from_static("x-ms-request-duration-ms");

    // Attempt createItem against the read-only region (West US).
    let body = serde_json::json!({"id": "forbidden-item", "pk": "pk1", "value": 1});
    let req = create_item_request(
        &ctx.west_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    let (status, headers, body_json) =
        collect_response(ctx.emulator.execute_request(&req).await.unwrap()).await;

    // ── Status & substatus ───────────────────────────────────────
    assert_eq!(status, StatusCode::Forbidden, "HTTP status should be 403");
    let substatus_val = headers
        .get_optional_str(&SUBSTATUS)
        .expect("x-ms-substatus header should be present");
    assert_eq!(substatus_val, "3", "substatus should be 3 (WriteForbidden)");

    // ── Activity ID (unique per request) ─────────────────────────
    let aid = headers
        .get_optional_str(&activity_id)
        .expect("x-ms-activity-id should be present");
    assert!(
        !aid.is_empty(),
        "x-ms-activity-id should be a non-empty UUID",
    );

    // ── Request charge (should be 0 for rejected writes) ─────────
    let charge_str = headers
        .get_optional_str(&request_charge)
        .expect("x-ms-request-charge should be present");
    let charge: f64 = charge_str
        .parse()
        .expect("request charge should be a number");
    assert!(
        charge >= 0.0,
        "x-ms-request-charge should be non-negative, got {charge}",
    );

    // ── Session token (omitted for rejected writes) ──────────────
    // Rejected writes do not advance the session vector, so the
    // emulator omits the `x-ms-session-token` response header rather
    // than emitting an empty value (which would mislead drivers into
    // treating it as a no-progress checkpoint). When the header IS
    // present its value must be non-empty.
    if let Some(tok) = headers.get_optional_str(&SESSION_TOKEN) {
        assert!(
            !tok.is_empty(),
            "x-ms-session-token, if present, must be non-empty (got '')",
        );
    }

    // ── Standard Cosmos response headers ─────────────────────────
    assert!(
        headers.get_optional_str(&content_type).is_some(),
        "content-type header should be present",
    );
    assert!(
        headers.get_optional_str(&date).is_some(),
        "date header should be present",
    );
    assert!(
        headers.get_optional_str(&version).is_some(),
        "x-ms-version header should be present",
    );
    let duration_str = headers
        .get_optional_str(&server_duration_ms)
        .expect("x-ms-request-duration-ms should be present");
    let duration: f64 = duration_str
        .parse()
        .expect("server duration should be a number");
    assert!(
        duration >= 0.0,
        "x-ms-request-duration-ms should be non-negative, got {duration}",
    );

    // ── Error body structure ─────────────────────────────────────
    assert!(body_json.is_object(), "Error body should be a JSON object");
    let code = body_json["code"]
        .as_str()
        .expect("Error body should have a 'code' field");
    assert_eq!(code, "Forbidden", "Error code should be 'Forbidden'");
    let message = body_json["message"]
        .as_str()
        .expect("Error body should have a 'message' field");
    assert!(
        message.contains("not allowed") || message.contains("Write"),
        "Error message should describe write-forbidden, got: {message}",
    );

    // ── No etag or lsn on error responses ────────────────────────
    // The real service does not return etag or lsn for 403.3 errors.
    assert!(
        headers.get_optional_str(&ETAG).is_none(),
        "etag should not be present on 403.3 error",
    );
}

#[tokio::test]
async fn paused_replication_buffer_full_returns_429_3075() {
    use azure_core::http::Url;
    use azure_data_cosmos_driver::in_memory_emulator::ReplicationConfig;

    // Build a 2-region account where the replication buffer for any target
    // region is capped at 3 entries — small enough to fill quickly.
    let east_url = "https://eastus.emulator.local";
    let west_url = "https://westus.emulator.local";
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(east_url).unwrap()),
        VirtualRegion::new("West US", Url::parse(west_url).unwrap()),
    ])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session)
    .with_replication_config(ReplicationConfig::immediate().with_max_buffered_replications(3));
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
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

    // Pause West US so the replication buffer fills with each East US write.
    store.pause_replication("West US");

    // Three writes succeed — buffer fills exactly to the cap.
    for i in 0..3 {
        let body = serde_json::json!({"id": format!("item-{i}"), "pk": "pk1", "value": i});
        let req = create_item_request(east_url, "testdb", "testcoll", &body, r#"["pk1"]"#, false);
        let response = emulator.execute_request(&req).await.unwrap();
        assert_eq!(
            response.status(),
            StatusCode::Created,
            "write {i} should succeed (buffer not yet full)"
        );
    }

    // Fourth write must short-circuit with 429/3075 — back-pressure from a
    // paused, saturated target.
    let body = serde_json::json!({"id": "overflow", "pk": "pk1", "value": 4});
    let req = create_item_request(east_url, "testdb", "testcoll", &body, r#"["pk1"]"#, false);
    let response = emulator.execute_request(&req).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::TooManyRequests,
        "write must be rejected when target replication queue is saturated"
    );
    let substatus = response
        .headers()
        .get_optional_str(&SUBSTATUS)
        .unwrap_or("0");
    assert_eq!(substatus, "3075", "substatus must signal queue overflow");

    // Resume — buffer drains, subsequent writes succeed again.
    store.resume_replication("West US");
    let body = serde_json::json!({"id": "after-resume", "pk": "pk1", "value": 5});
    let req = create_item_request(east_url, "testdb", "testcoll", &body, r#"["pk1"]"#, false);
    let response = emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);
}
