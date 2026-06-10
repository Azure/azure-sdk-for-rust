// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Throughput throttling integration tests (429/3200).

use super::*;

static RETRY_AFTER: azure_core::http::headers::HeaderName =
    azure_core::http::headers::HeaderName::from_static("x-ms-retry-after-ms");

/// Helper to set up a single-region emulator with throttling enabled and a
/// container provisioned at the given RU/s.
async fn setup_throttled(throughput_ru: u32) -> TestContext {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
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
            .with_throughput(throughput_ru)
            .build()
            .unwrap(),
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
async fn container_creation_min_400() {
    let err = azure_data_cosmos_driver::in_memory_emulator::ContainerConfig::new()
        .with_throughput(100)
        .build()
        .unwrap_err();
    assert!(err
        .to_string()
        .contains("provisioned throughput must be >= 400 RU/s"));
}

#[tokio::test]
async fn container_creation_zero_partitions_rejected() {
    let err = azure_data_cosmos_driver::in_memory_emulator::ContainerConfig::new()
        .with_partition_count(0)
        .build()
        .unwrap_err();
    assert!(err.to_string().contains("partition count must be > 0"));
}

#[tokio::test]
async fn throttled_create_does_not_persist_document() {
    // Verify that when a create is throttled, the document is NOT stored.
    // This tests the fix for the "write-then-check" bug where throttle checks
    // happened after the document was already persisted.

    // Use minimum throughput (400 RU/s) with 1 partition so we can exhaust it.
    let ctx = setup_throttled(400).await;

    // Exhaust the budget with successful creates.
    let mut throttled_id = None;
    for i in 0..200 {
        let id = format!("thr-item-{}", i);
        let body = serde_json::json!({"id": &id, "pk": "pk1", "value": i});
        let req = create_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            &body,
            r#"["pk1"]"#,
            false,
        );
        let response = ctx.emulator.execute_request(&req).await.unwrap();
        if response.status() == StatusCode::TooManyRequests {
            throttled_id = Some(id);
            break;
        }
        assert_eq!(response.status(), StatusCode::Created);
    }

    let throttled_id = throttled_id.expect("should have been throttled");

    // The throttled document must NOT be readable.
    let req = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &throttled_id,
        r#"["pk1"]"#,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::NotFound,
        "throttled create must not persist the document",
    );
}

#[tokio::test]
async fn throttled_replace_does_not_modify_document() {
    // Verify that when a replace is throttled, the original document is unchanged.
    let ctx = setup_throttled(400).await;

    // Create a seed item first (low RU, should succeed).
    let seed_body = serde_json::json!({"id": "seed", "pk": "pk1", "value": "original"});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &seed_body,
        r#"["pk1"]"#,
        true,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);

    // Exhaust the budget with more creates.
    for i in 1..200 {
        let body = serde_json::json!({"id": format!("fill-{}", i), "pk": "pk1", "value": i});
        let req = create_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            &body,
            r#"["pk1"]"#,
            false,
        );
        let response = ctx.emulator.execute_request(&req).await.unwrap();
        if response.status() == StatusCode::TooManyRequests {
            break;
        }
    }

    // Now try to replace the seed. This should be throttled.
    let replace_body = serde_json::json!({"id": "seed", "pk": "pk1", "value": "modified"});
    let req = replace_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "seed",
        &replace_body,
        r#"["pk1"]"#,
        None,
        true,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();

    // The fill loop above exhausts the per-second budget in the current
    // window, so this immediately-following replace must be throttled. Wait
    // for a fresh budget window before issuing the read-back.
    assert_eq!(
        response.status(),
        StatusCode::TooManyRequests,
        "replace immediately after a throttle-exhausting fill must be throttled (deterministic 1s budget window)"
    );
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let req = read_item_request(&ctx.gateway_url, "testdb", "testcoll", "seed", r#"["pk1"]"#);
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Ok);
    let body = read_response_body(response).await;
    assert_eq!(
        body["value"], "original",
        "throttled replace must not modify the document",
    );
}

#[tokio::test]
async fn throttled_delete_does_not_remove_document() {
    // Verify that when a delete is throttled, the document remains.
    let ctx = setup_throttled(400).await;

    // Create a seed item.
    let seed_body = serde_json::json!({"id": "keep-me", "pk": "pk1", "value": 42});
    let req = create_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        &seed_body,
        r#"["pk1"]"#,
        false,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);

    // Exhaust the budget.
    for i in 1..200 {
        let body = serde_json::json!({"id": format!("fill-{}", i), "pk": "pk1", "value": i});
        let req = create_item_request(
            &ctx.gateway_url,
            "testdb",
            "testcoll",
            &body,
            r#"["pk1"]"#,
            false,
        );
        let response = ctx.emulator.execute_request(&req).await.unwrap();
        if response.status() == StatusCode::TooManyRequests {
            break;
        }
    }

    // Try to delete the seed. Should be throttled.
    let req = delete_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "keep-me",
        r#"["pk1"]"#,
        None,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();

    assert_eq!(
        response.status(),
        StatusCode::TooManyRequests,
        "delete immediately after a throttle-exhausting fill must be throttled (deterministic 1s budget window)"
    );
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let req = read_item_request(
        &ctx.gateway_url,
        "testdb",
        "testcoll",
        "keep-me",
        r#"["pk1"]"#,
    );
    let response = ctx.emulator.execute_request(&req).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::Ok,
        "throttled delete must not remove the document",
    );
}

/// Regression: container creation requests carrying `x-ms-offer-throughput`
/// must produce a container that actually honors the requested RU/s.
///
/// Before the fix, `handle_create_container` ignored the header and built the
/// container with `ContainerConfig::default()` (no provisioned RU/s), so
/// throttling never engaged regardless of what the caller requested. This
/// test creates a container over the wire with a 400 RU/s offer header on a
/// throttling-enabled account, then drives writes hard enough to exhaust the
/// per-second budget. A 429/3200 response (with `x-ms-retry-after-ms`) proves
/// the offer was applied. The companion negative case (no header → no
/// throttling) is covered by [`throttle_disabled_no_429`] above for the
/// no-throttling-enabled scenario; here we focus on the positive flow.
#[tokio::test]
async fn create_container_honors_offer_throughput_header() {
    use azure_core::http::headers::{HeaderName, HeaderValue};
    use azure_core::http::{Method, Request, Url};

    static OFFER_THROUGHPUT: HeaderName = HeaderName::from_static("x-ms-offer-throughput");

    // Throttling-enabled single-region account, no container provisioned yet.
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session)
    .with_throttling_enabled(true);
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    emulator.store().create_database("testdb");

    // Create the container over the wire, with a 400 RU/s throughput header
    // and a single partition (so the entire budget concentrates on one
    // partition and can be exhausted deterministically).
    let url = format!("{}/dbs/testdb/colls", GATEWAY_URL);
    let mut req = Request::new(Url::parse(&url).unwrap(), Method::Post);
    req.set_body(
        serde_json::to_vec(&serde_json::json!({
            "id": "throughput_coll",
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
    let response = emulator.execute_request(&req).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);

    // The default 4-partition container would split 400 RU/s into 100 RU per
    // partition, but `x-ms-offer-throughput` does not control partition
    // count — it only sets the total RU/s. To make the assertion robust
    // against the default partition count, pump enough creates to exhaust
    // even the per-partition budget.
    let mut throttled = false;
    for i in 0..1000 {
        let body = serde_json::json!({"id": format!("item{}", i), "pk": "pk1", "value": i});
        let req = create_item_request(
            GATEWAY_URL,
            "testdb",
            "throughput_coll",
            &body,
            r#"["pk1"]"#,
            false,
        );
        let response = emulator.execute_request(&req).await.unwrap();
        if response.status() == StatusCode::TooManyRequests {
            assert_eq!(
                response
                    .headers()
                    .get_optional_str(&SUBSTATUS)
                    .unwrap_or("0"),
                "3200"
            );
            assert!(response.headers().get_optional_str(&RETRY_AFTER).is_some());
            throttled = true;
            break;
        }
        assert_eq!(response.status(), StatusCode::Created);
    }
    assert!(
        throttled,
        "expected x-ms-offer-throughput=400 to engage throttling within 1000 creates"
    );
}

/// End-to-end validation that the configurable 429 (throttle) retry budget —
/// [`ThrottlingRetryOptions::max_retry_count`](azure_data_cosmos_driver::options::ThrottlingRetryOptions::max_retry_count)
/// — is honored when driving the **full driver pipeline** against the
/// in-memory emulator.
///
/// Unlike the raw-HTTP throttling tests above (which call
/// [`InMemoryEmulatorHttpClient::execute_request`] directly and therefore
/// bypass the driver's transport retry loop), this test wires the emulator in
/// as the driver's transport via
/// [`InMemoryEmulatorHttpClient::runtime_builder`] and layers a fault-injection
/// rule on top. The rule injects an HTTP 429 (`TooManyRequests`) on **every**
/// `ReadItem` attempt — short-circuiting the emulator entirely — so the only
/// thing limiting the number of read attempts that reach the (faulted)
/// transport is the configured throttle-retry budget.
///
/// Because the injected 429 carries no sub-status it is a *generic* throttle:
/// the operation pipeline does not re-retry it (a generic 429 routes straight
/// to `Abort`), so the entire budget is spent inside a single
/// transport-pipeline invocation and `rule.hit_count()` equals the exact
/// number of read attempts on the wire:
///
/// * **Total = `N + 1`** attempts (1 initial + N retries) for any N,
///   including `N == 0`. The one-shot forced-final-retry safety net in
///   `execute_transport_pipeline` is gated on `attempt_count < max_attempts`,
///   so once the count budget is exhausted it is suppressed too — matching
///   the .NET-parity `MaxRetryAttemptsOnRateLimitedRequests` semantic.
/// * The forced-final retry still fires when the *cumulative-wait* budget
///   (rather than the count) is the limiter; this test sets a generous
///   300-second wait so the count is the sole limiter.
///
/// This is the in-memory analog of the live-emulator test
/// `emulator_tests::driver_fault_injection::fault_injection_429_honors_configurable_throttle_retry_count`
/// and the unit-level
/// `transport_pipeline::tests::execute_transport_pipeline_honors_configured_max_throttle_attempts`.
#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn fault_injection_429_honors_configurable_throttle_retry_count() {
    use azure_data_cosmos_driver::fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    };
    use azure_data_cosmos_driver::models::{
        AccountReference, CosmosOperation, ItemReference, PartitionKey,
    };
    use azure_data_cosmos_driver::options::{
        OperationOptions, OperationOptionsBuilder, ThrottlingRetryOptionsBuilder,
    };

    // A single-region emulator with a database + container provisioned. No
    // throttling is enabled on the store itself — the 429s come purely from
    // the fault-injection rule, keeping the attempt count deterministic.
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

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

    let account = AccountReference::with_master_key(
        azure_core::http::Url::parse(GATEWAY_URL).unwrap(),
        "ZW11bGF0b3Ita2V5",
    );

    // (configured throttle-retry budget, expected total ReadItem attempts).
    // Total = 1 initial + N retries (the forced-final retry is suppressed
    // once the count budget is exhausted; it only fires when the
    // cumulative-wait budget is the limiter).
    for (max_throttle_retry_count, expected_hits) in [(0_u32, 1_u32), (1, 2), (3, 4), (5, 6)] {
        let rule = Arc::new(
            FaultInjectionRuleBuilder::new(
                "always-429",
                FaultInjectionResultBuilder::new()
                    .with_error(FaultInjectionErrorType::TooManyRequests)
                    .with_probability(1.0)
                    .build(),
            )
            .with_condition(
                FaultInjectionConditionBuilder::new()
                    .with_operation_type(FaultOperationType::ReadItem)
                    .build(),
            )
            .build(),
        );

        // Pin the throttle-retry budget at the runtime layer. A generous
        // cumulative-wait budget keeps the attempt count the sole limiter,
        // and no end-to-end latency policy is set so the forced-final retry
        // is immediate.
        let operation_options = OperationOptionsBuilder::new()
            .with_throttling_retry_options(
                ThrottlingRetryOptionsBuilder::new()
                    .with_max_retry_count(max_throttle_retry_count)
                    .with_max_retry_wait_time(std::time::Duration::from_secs(300))
                    .build(),
            )
            .build();

        let runtime = emulator
            .runtime_builder()
            .with_fault_injection_rules(vec![Arc::clone(&rule)])
            .expect("fault injection rules should register")
            .with_default_operation_options(operation_options)
            .build()
            .await
            .expect("runtime should build against the in-memory emulator");

        let driver = runtime
            .get_or_create_driver(account.clone(), None)
            .await
            .expect("driver should initialize against the in-memory emulator");

        let container = driver
            .resolve_container_by_name("testdb", "testcoll")
            .await
            .expect("container should resolve");

        // Seed a unique item per iteration via the driver. The fault rule
        // targets only ReadItem, so this CreateItem is never faulted.
        let item_id = format!("item-{max_throttle_retry_count}");
        let create_body =
            format!(r#"{{"id": "{item_id}", "pk": "pk1", "value": "throttle-test"}}"#);
        let create_ref =
            ItemReference::from_name(&container, PartitionKey::from("pk1"), item_id.clone());
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(create_ref).with_body(create_body.into_bytes()),
                OperationOptions::default(),
            )
            .await
            .expect("seeding create must succeed (rule targets ReadItem only)");

        // The read always observes the injected 429 and ultimately fails once
        // the throttle budget is exhausted.
        let read_ref =
            ItemReference::from_name(&container, PartitionKey::from("pk1"), item_id.clone());
        let read_result = driver
            .execute_singleton_operation(
                CosmosOperation::read_item(read_ref),
                OperationOptions::default(),
            )
            .await;
        assert!(
            read_result.is_err(),
            "read must fail once the throttle budget is exhausted \
             (max_throttle_retry_count={max_throttle_retry_count})",
        );

        assert_eq!(
            rule.hit_count(),
            expected_hits,
            "max_throttle_retry_count={max_throttle_retry_count} must yield {expected_hits} \
             ReadItem attempts on the wire, but the 429 fault rule fired {} time(s)",
            rule.hit_count(),
        );
    }
}
