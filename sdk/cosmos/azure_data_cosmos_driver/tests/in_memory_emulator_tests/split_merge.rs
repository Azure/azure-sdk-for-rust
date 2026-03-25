// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition split and merge integration tests.

use super::*;
use azure_core::http::{HttpClient, Method, Request, Url};
use std::time::Duration;

#[tokio::test]
async fn split_creates_two_children() {
    let ctx = setup_single_region().await;
    let store = ctx.emulator.store();

    // Create an item in partition 0
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

    // Verify initial partition count via PKRanges
    let url = format!("{}/dbs/testdb/colls/testcoll/pkranges", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (_, _, body) = collect_response(response).await;
    let ranges = body["PartitionKeyRanges"].as_array().unwrap();
    assert_eq!(ranges.len(), 4); // default 4 partitions

    // Split partition 0 with zero min_lock_duration
    store.split_partition("testdb", "testcoll", 0, Duration::ZERO);

    // Give tokio a moment to complete the split
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Verify partition count increased
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (_, _, body) = collect_response(response).await;
    let ranges = body["PartitionKeyRanges"].as_array().unwrap();
    assert_eq!(ranges.len(), 5); // 4 - 1 + 2 = 5

    // Verify children have parents
    let children: Vec<&serde_json::Value> = ranges
        .iter()
        .filter(|r| {
            let parents = r["parents"].as_array().unwrap();
            !parents.is_empty()
        })
        .collect();
    assert_eq!(children.len(), 2);
    for child in &children {
        let parents = child["parents"].as_array().unwrap();
        assert_eq!(parents.len(), 1);
        assert_eq!(parents[0], "0");
    }
}

#[tokio::test]
async fn split_locked_returns_410_1007() {
    let ctx = setup_single_region().await;
    let store = ctx.emulator.store();

    // Split partition 0 with 500ms lock
    store.split_partition("testdb", "testcoll", 0, Duration::from_millis(500));

    // Immediately try to write to partition 0 — should get 410/1007
    // (The item's EPK routes to partition 0 since it's in the [MIN, boundary) range)
    let body = serde_json::json!({"id": "locked_item", "pk": "", "value": 1});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"[""]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Gone);
    let substatus = response
        .headers()
        .get_optional_str(&SUBSTATUS)
        .unwrap_or("0");
    assert_eq!(substatus, "1007");
}

#[tokio::test]
async fn split_preserves_vector_clock_version() {
    let ctx = setup_single_region().await;
    let store = ctx.emulator.store();

    // Create an item first (so LSN advances)
    let body = serde_json::json!({"id": "item1", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body,
        r#"["pk1"]"#,
        false,
    );
    ctx.emulator.execute_request(&req).await.unwrap();

    // Split partition 0
    store.split_partition("testdb", "testcoll", 0, Duration::ZERO);
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Check PKRanges — children should have same vectorClockVersion as parent (0)
    let url = format!("{}/dbs/testdb/colls/testcoll/pkranges", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (_, _, body) = collect_response(response).await;
    let ranges = body["PartitionKeyRanges"].as_array().unwrap();

    let children: Vec<&serde_json::Value> = ranges
        .iter()
        .filter(|r| !r["parents"].as_array().unwrap().is_empty())
        .collect();

    for child in &children {
        assert_eq!(child["vectorClockVersion"], 0);
    }
}

#[tokio::test]
async fn merge_adjacent_partitions() {
    let ctx = setup_single_region().await;
    let store = ctx.emulator.store();

    // Merge partitions 0 and 1 (adjacent in the EPK space)
    store.merge_partitions("testdb", "testcoll", 0, 1, Duration::ZERO);
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Verify partition count decreased
    let url = format!("{}/dbs/testdb/colls/testcoll/pkranges", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (_, _, body) = collect_response(response).await;
    let ranges = body["PartitionKeyRanges"].as_array().unwrap();
    assert_eq!(ranges.len(), 3); // 4 - 2 + 1 = 3

    // Verify child has both parents
    let child = ranges
        .iter()
        .find(|r| {
            let parents = r["parents"].as_array().unwrap();
            parents.len() == 2
        })
        .expect("should have a child with 2 parents");

    let parents = child["parents"].as_array().unwrap();
    assert!(parents.contains(&serde_json::json!("0")));
    assert!(parents.contains(&serde_json::json!("1")));
}

#[tokio::test]
async fn merge_increments_vector_clock_version() {
    let ctx = setup_single_region().await;
    let store = ctx.emulator.store();

    // Merge partitions 0 and 1
    store.merge_partitions("testdb", "testcoll", 0, 1, Duration::ZERO);
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Check child's vectorClockVersion — should be max(parent_versions) + 1 = 1
    let url = format!("{}/dbs/testdb/colls/testcoll/pkranges", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (_, _, body) = collect_response(response).await;
    let ranges = body["PartitionKeyRanges"].as_array().unwrap();

    let child = ranges
        .iter()
        .find(|r| r["parents"].as_array().unwrap().len() == 2)
        .unwrap();
    assert_eq!(child["vectorClockVersion"], 1);
    // LSN should restart at 1
    assert_eq!(child["_lsn"], 1);
}

#[tokio::test]
async fn session_token_uses_v2_format() {
    let ctx = setup_single_region().await;

    // Create an item and check the session token format
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

    let token = response
        .headers()
        .get_optional_str(&SESSION_TOKEN)
        .unwrap()
        .to_string();

    // V2 format: {pkrangeId}:{version}#{globalLSN}#{regionId}={localLSN}
    // Should NOT contain ":-1#" (V1 format)
    assert!(
        !token.contains(":-1#"),
        "Expected V2 session token but got V1: {}",
        token
    );

    // Should contain version#lsn#region=lsn pattern
    let parts: Vec<&str> = token.split(':').collect();
    assert_eq!(parts.len(), 2, "Token should have pkrangeId:rest format");
    let rest = parts[1];
    let hash_parts: Vec<&str> = rest.split('#').collect();
    assert!(
        hash_parts.len() >= 3,
        "V2 token should have version#globalLSN#region=lsn: {}",
        token
    );
}

#[tokio::test]
async fn read_after_split_succeeds() {
    let ctx = setup_single_region().await;
    let store = ctx.emulator.store();

    // Create items
    let body1 = serde_json::json!({"id": "item1", "pk": "pk1", "value": 1});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &body1,
        r#"["pk1"]"#,
        false,
    );
    ctx.emulator.execute_request(&req).await.unwrap();

    // Split the partition that contains this item
    // First find which partition has the item
    let url = format!("{}/dbs/testdb/colls/testcoll/pkranges", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (_, _, body) = collect_response(response).await;
    let first_pk_id: u32 = body["PartitionKeyRanges"][0]["id"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();

    // Split the first partition
    store.split_partition("testdb", "testcoll", first_pk_id, Duration::ZERO);
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Read the item — should still work after split
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
    assert_eq!(doc["value"], 1);
}
