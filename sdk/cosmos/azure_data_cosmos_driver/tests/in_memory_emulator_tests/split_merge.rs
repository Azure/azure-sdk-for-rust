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
    store.drain_pending_control_plane().await;

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

    // Discover which partition the test PK routes to. V2 hashing distributes
    // `""` to partition 3 (not 0), so we cannot hard-code partition 0 — we
    // do a seed create first, parse the session token to learn the pkrange
    // id, then split *that* partition and verify the next write to the same
    // PK is rejected with 410/1007.
    let probe_body = serde_json::json!({"id": "probe", "pk": "", "value": 0});
    let probe_req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &probe_body,
        r#"[""]"#,
        false,
    );
    let probe_resp = ctx.emulator.execute_request(&probe_req).await.unwrap();
    assert_eq!(probe_resp.status(), StatusCode::Created);
    let probe_token = probe_resp
        .headers()
        .get_optional_str(&SESSION_TOKEN)
        .expect("probe create should return a session token")
        .to_string();
    let pkrange_id: u32 = probe_token
        .split(':')
        .next()
        .and_then(|s| s.parse().ok())
        .expect("session token must start with a numeric pkrange id");

    // Split that partition with 500ms lock so the subsequent write hits the
    // locked window deterministically.
    store.split_partition("testdb", "testcoll", pkrange_id, Duration::from_millis(500));

    // Immediately try to write to the locked partition — should get 410/1007.
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
    store.drain_pending_control_plane().await;

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
    store.drain_pending_control_plane().await;

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
async fn merge_rejects_non_adjacent_partitions() {
    let ctx = setup_single_region().await;
    let store = ctx.emulator.store();

    let mut routed_item = None;
    for index in 0..128 {
        let item_id = format!("item{index}");
        let pk_value = format!("pk{index}");
        let body = serde_json::json!({"id": item_id, "pk": pk_value, "value": index});
        let pk_header = serde_json::json!([body["pk"].as_str().unwrap()]).to_string();
        let req = create_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            &body,
            &pk_header,
            false,
        );
        let response = ctx.emulator.execute_request(&req).await.unwrap();
        assert_eq!(response.status(), StatusCode::Created);

        let partition_id = response
            .headers()
            .get_optional_str(&SESSION_TOKEN)
            .and_then(|token| token.split(':').next())
            .and_then(|prefix| prefix.parse::<u32>().ok())
            .expect("create should return a numeric partition id in the session token");
        if partition_id == 0 || partition_id == 2 {
            routed_item = Some((item_id, pk_header));
            break;
        }
    }

    let (item_id, pk_header) =
        routed_item.expect("expected to route at least one item into partition 0 or 2");

    store.merge_partitions("testdb", "testcoll", 0, 2, Duration::ZERO);
    store.drain_pending_control_plane().await;

    let url = format!("{}/dbs/testdb/colls/testcoll/pkranges", ctx.gateway_url);
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (_, _, body) = collect_response(response).await;
    let ranges = body["PartitionKeyRanges"].as_array().unwrap();
    assert_eq!(
        ranges.len(),
        4,
        "non-adjacent merge should not change topology"
    );
    assert!(ranges.iter().all(|range| {
        range["parents"]
            .as_array()
            .expect("PKRange should always contain parents array")
            .is_empty()
    }));

    let req = read_item_request(&ctx.gateway_url, "testdb", "testcoll", &item_id, &pk_header);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let (status, _, doc) = collect_response(response).await;
    assert_eq!(
        status,
        StatusCode::Ok,
        "invalid merge must not leave parent ranges locked"
    );
    assert_eq!(doc["id"], item_id);
}

#[tokio::test]
async fn merge_increments_vector_clock_version() {
    let ctx = setup_single_region().await;
    let store = ctx.emulator.store();

    // Merge partitions 0 and 1
    store.merge_partitions("testdb", "testcoll", 0, 1, Duration::ZERO);
    store.drain_pending_control_plane().await;

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
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    let routed_partition_id: u32 = response
        .headers()
        .get_optional_str(&SESSION_TOKEN)
        .and_then(|token| token.split(':').next())
        .and_then(|prefix| prefix.parse().ok())
        .expect("create should return a session token with a numeric partition id");

    // Split the partition that actually owns the item.
    store.split_partition("testdb", "testcoll", routed_partition_id, Duration::ZERO);
    store.drain_pending_control_plane().await;

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

/// Splitting a V1-hash container must succeed and route post-split reads
/// correctly. Before V1-aware split midpoint support, `compute_epk_midpoint`
/// rejected V1 boundaries and `split_partition` aborted with the parent
/// re-unlocked but no children created — leaving V1 containers permanently
/// non-splittable.
///
/// This test creates a V1 container, splits a partition that actually owns
/// at least one previously-inserted document, drains the split task, and
/// then verifies (a) the partition count grew by exactly 1 and (b) the
/// pre-split document remains readable on its (new) child partition. (a)
/// catches the regression where the split silently no-ops; (b) catches a
/// regression where the V1 midpoint is computed in the wrong space and
/// docs end up on the side of the boundary that no longer covers them.
#[tokio::test]
async fn split_v1_container_creates_two_children_and_routes_reads() {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();
    store.create_database("testdb");
    store.create_container(
        "testdb",
        "v1coll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 1
        }))
        .unwrap(),
    );

    let gateway_url = GATEWAY_URL;

    // Insert a handful of distinct PKs so multiple partitions become
    // populated; pick one whose routed partition has docs to verify
    // post-split routing.
    let mut routed: std::collections::HashMap<u32, Vec<String>> = Default::default();
    for i in 0..16 {
        let id = format!("v1-item-{i}");
        let pk = format!("pk-{i}");
        let body = serde_json::json!({"id": &id, "pk": &pk, "value": i});
        let pk_header = format!("[\"{pk}\"]");
        let req = create_item_request(gateway_url, "testdb", "v1coll", &body, &pk_header, false);
        let response = emulator.execute_request(&req).await.unwrap();
        assert_eq!(response.status(), StatusCode::Created);
        let pid: u32 = response
            .headers()
            .get_optional_str(&SESSION_TOKEN)
            .and_then(|token| token.split(':').next())
            .and_then(|prefix| prefix.parse().ok())
            .expect("create should return a session token with a numeric partition id");
        routed.entry(pid).or_default().push(id);
    }

    // Pick a partition that owns at least one document so the post-split
    // read assertion is meaningful.
    let (target_pid, target_ids) = routed
        .into_iter()
        .find(|(_, ids)| !ids.is_empty())
        .expect("at least one partition should have received a document");

    // Snapshot the pre-split partition count.
    let url = format!("{gateway_url}/dbs/testdb/colls/v1coll/pkranges");
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = emulator.execute_request(&req).await.unwrap();
    let (_, _, body) = collect_response(response).await;
    let pre_count = body["PartitionKeyRanges"].as_array().unwrap().len();

    // Split the partition we know owns documents.
    store.split_partition("testdb", "v1coll", target_pid, Duration::ZERO);
    store.drain_pending_control_plane().await;

    // Topology must have grown by exactly 1 (one parent removed, two children added).
    let req = Request::new(Url::parse(&url).unwrap(), Method::Get);
    let response = emulator.execute_request(&req).await.unwrap();
    let (_, _, body) = collect_response(response).await;
    let ranges = body["PartitionKeyRanges"].as_array().unwrap();
    assert_eq!(
        ranges.len(),
        pre_count + 1,
        "V1 split must produce exactly two children from one parent",
    );
    // And exactly two of those ranges must list the parent id.
    let target_pid_str = target_pid.to_string();
    let children: Vec<&serde_json::Value> = ranges
        .iter()
        .filter(|r| {
            r["parents"]
                .as_array()
                .map(|p| p.iter().any(|x| x == &serde_json::json!(target_pid_str)))
                .unwrap_or(false)
        })
        .collect();
    assert_eq!(
        children.len(),
        2,
        "split must produce two children with parent={target_pid}",
    );

    // Every previously-inserted doc on the parent must remain readable.
    for id in &target_ids {
        // Recover the original PK suffix from the doc id ("v1-item-{i}" → "pk-{i}").
        let suffix = id
            .strip_prefix("v1-item-")
            .expect("test-controlled id format");
        let pk_header = format!("[\"pk-{suffix}\"]");
        let req = read_item_request(gateway_url, "testdb", "v1coll", id, &pk_header);
        let response = emulator.execute_request(&req).await.unwrap();
        assert_eq!(
            response.status(),
            StatusCode::Ok,
            "post-split read of {id} must succeed",
        );
    }
}
