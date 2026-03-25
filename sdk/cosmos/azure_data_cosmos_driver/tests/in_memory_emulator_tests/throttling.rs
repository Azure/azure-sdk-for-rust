// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Throughput throttling integration tests (429/3200).

use super::*;
use azure_core::http::HttpClient;

static RETRY_AFTER: azure_core::http::headers::HeaderName =
    azure_core::http::headers::HeaderName::from_static("x-ms-retry-after-ms");

/// Helper to set up a single-region emulator with throttling enabled and a
/// container provisioned at the given RU/s.
async fn setup_throttled(throughput_ru: u32) -> TestContext {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(GATEWAY_URL).unwrap(),
    )])
    .with_consistency(ConsistencyLevel::Session)
    .with_throttling_enabled(true);

    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();

    store.create_database("testdb");
    store.create_container_with_config(
        "testdb",
        "testcoll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
        azure_data_cosmos_driver::in_memory_emulator::ContainerConfig::new()
            .with_partition_count(1) // single partition for predictable budget
            .with_throughput(throughput_ru),
    );

    TestContext {
        emulator,
        gateway_url: GATEWAY_URL.to_string(),
    }
}

#[tokio::test]
async fn throttle_429_3200_when_exceeds_budget() {
    // 400 RU/s with 1 partition → all 400 RU goes to the single partition
    let ctx = setup_throttled(400).await;

    // First request should succeed (small doc ~ 5.8 RU create)
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

    // Exhaust the budget by creating many items
    for i in 2..200 {
        let body = serde_json::json!({"id": format!("item{}", i), "pk": "pk1", "value": i});
        let req = create_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            &body,
            r#"["pk1"]"#,
            false,
        );
        let response = ctx.emulator.execute_request(&req).await.unwrap();
        let status = response.status();
        if status == StatusCode::TooManyRequests {
            // Success — we got throttled
            let substatus = response
                .headers()
                .get_optional_str(&SUBSTATUS)
                .unwrap_or("0");
            assert_eq!(substatus, "3200");
            assert!(response.headers().get_optional_str(&RETRY_AFTER).is_some());
            return;
        }
        assert_eq!(status, StatusCode::Created);
    }

    panic!("Expected 429/3200 throttling but never hit the limit");
}

#[tokio::test]
async fn throttle_disabled_no_429() {
    // Set up WITHOUT throttling enabled (default)
    let ctx = setup_single_region().await;

    // Even with provisioned throughput, no throttling when disabled
    for i in 0..100 {
        let body = serde_json::json!({"id": format!("item{}", i), "pk": "pk1", "value": i});
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
}

#[tokio::test]
#[should_panic(expected = "provisioned throughput must be >= 400 RU/s")]
async fn container_creation_min_400() {
    azure_data_cosmos_driver::in_memory_emulator::ContainerConfig::new().with_throughput(100);
}
