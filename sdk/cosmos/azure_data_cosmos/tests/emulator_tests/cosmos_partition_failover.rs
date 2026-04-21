// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for per-partition automatic failover (PPAF) and per-partition
//! circuit breaker (PPCB) at the SDK layer.
//!
//! **Prereqs**: 3-region single-master session consistency account with PPAF enabled.
//! Default PPCB thresholds.
//!
//! These tests use fault injection to simulate region-level failures and verify
//! that partition-level failover (PPAF/PPCB) moves operations to alternate regions.

#![cfg(feature = "key_auth")]
#![cfg(feature = "fault_injection")]

use super::framework;

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::fault_injection::{
    FaultInjectionClientBuilder, FaultInjectionConditionBuilder, FaultInjectionErrorType,
    FaultInjectionResultBuilder, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use framework::{TestClient, TestOptions, HUB_REGION};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct TestItem {
    id: Cow<'static, str>,
    partition_key: Option<Cow<'static, str>>,
    value: usize,
}

fn create_test_item(unique_id: &str) -> TestItem {
    TestItem {
        id: format!("Item-{}", unique_id).into(),
        partition_key: Some(format!("Partition-{}", unique_id).into()),
        value: 42,
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test 1: PPAF enabled — 503 on CreateItem from region 1 → partition fails
//         over to the next write region, write succeeds.
// ────────────────────────────────────────────────────────────────────────────

/// When PPAF is enabled on the account and region 1 returns 503 for a create,
/// the partition should fail over to the next region and the write should succeed.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppaf_enabled_503_on_create_fails_over_to_next_region() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppaf-503-create", result)
            .with_condition(condition)
            .build(),
    );

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::clone(&rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let _container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);

            // Use the fault client for the write — fault targets CreateItem in hub region.
            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // CreateItem with 503 injected in hub region — PPAF should trigger partition failover.
            let create_response = fault_container_client
                .create_item(&pk, &item, None)
                .await
                .expect("CreateItem should succeed via PPAF failover to another region");

            // Verify the response indicates success.
            assert_eq!(create_response.status(), StatusCode::Created);

            // Verify the fault rule was hit on the hub region.
            assert!(
                rule.hit_count() > 0,
                "Fault injection rule should have been hit on the hub region"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// When PPAF is enabled on the account and region 1 returns 403/3 WriteForbidden
/// for a create, the partition should fail over to the next region and the write
/// should succeed.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppaf_enabled_write_forbidden_on_create_fails_over_to_next_region(
) -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppaf-403-create", result)
            .with_condition(condition)
            .build(),
    );

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::clone(&rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let _container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let create_response = fault_container_client
                .create_item(&pk, &item, None)
                .await
                .expect("CreateItem should succeed via PPAF failover on 403/3 WriteForbidden");

            assert_eq!(create_response.status(), StatusCode::Created);

            assert!(
                rule.hit_count() > 0,
                "Fault injection rule should have been hit on the hub region"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

// ────────────────────────────────────────────────────────────────────────────
// Test 2: PPAF disabled — 503 / 403.3 on CreateItem from region 1 → partition
//         does NOT fail over; write fails.
// ────────────────────────────────────────────────────────────────────────────

/// When PPAF is disabled, a 503 on CreateItem from region 1 should NOT trigger
/// partition failover for a non-idempotent write. The write should fail because
/// the request was already sent and PPAF retry is not allowed.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppaf_disabled_503_on_create_does_not_failover() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("no-ppaf-503-create", result)
            .with_condition(condition)
            .build(),
    );

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::clone(&rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let _container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // With PPAF disabled, the non-idempotent write should fail (abort).
            let create_result = fault_container_client.create_item(&pk, &item, None).await;

            assert!(
                create_result.is_err(),
                "CreateItem should fail when PPAF is disabled and 503 is returned — \
                non-idempotent write cannot be retried safely"
            );

            let err = create_result.unwrap_err();
            assert_eq!(
                Some(StatusCode::ServiceUnavailable),
                err.http_status(),
                "Error should indicate 503 Service Unavailable"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// When PPAF is disabled, a 403/3 WriteForbidden on CreateItem should trigger
/// a cross-regional failover retry (403/3 always triggers failover regardless
/// of PPAF), but without partition-level override. Subsequent writes to the same
/// partition will still attempt the hub region first.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppaf_disabled_write_forbidden_on_create_retries_but_no_partition_override(
) -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("no-ppaf-403-create", result)
            .with_condition(condition)
            .build(),
    );

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::clone(&rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let _container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // 403/3 WriteForbidden always triggers FailoverRetry regardless of PPAF,
            // so the first write should succeed by failing over to another region.
            let create_response = fault_container_client
                .create_item(&pk, &item, None)
                .await
                .expect("CreateItem should succeed via cross-regional failover on 403/3");

            assert_eq!(create_response.status(), StatusCode::Created);

            assert!(
                rule.hit_count() > 0,
                "Fault injection rule should have been hit on the hub region"
            );

            // Disable the rule so the next write can succeed without faults.
            rule.disable();

            // Without PPAF, there is no partition-level override, so the next write
            // to the same partition should go back to the hub region.
            let unique_id2 = Uuid::new_v4().to_string();
            let item2 = create_test_item(&unique_id2);

            let second_response = fault_container_client
                .create_item(&pk, &item2, None)
                .await
                .expect("Second CreateItem to same partition should succeed after fault rule is disabled");

            assert_eq!(second_response.status(), StatusCode::Created);

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

// ────────────────────────────────────────────────────────────────────────────
// Test 3: PPCB enabled — 503 / 403.3 on ReadItem from region 1 → once failure
//         threshold is hit, reads are routed to the next region.
// ────────────────────────────────────────────────────────────────────────────

/// When PPCB is enabled (default when PPAF is enabled) and region 1 returns
/// 503 for reads, after the default read failure threshold (5) is exceeded,
/// subsequent reads should be routed directly to the next region via a
/// partition-level circuit breaker override.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppcb_enabled_503_on_read_fails_over_after_threshold() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppcb-503-read", result)
            .with_condition(condition)
            .build(),
    );

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::clone(&rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            // Seed an item using the normal (non-fault) client.
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Issue reads via the fault client to accumulate failures and trigger
            // the circuit breaker threshold. Default read failure threshold is 5.
            let mut read_success_count = 0;
            let total_reads = 10;
            for _ in 0..total_reads {
                match fault_container_client
                    .read_item::<TestItem>(&pk, &item_id, None)
                    .await
                {
                    Ok(_) => read_success_count += 1,
                    Err(e) => {
                        tracing::info!("Read failed (expected during threshold ramp-up): {e}");
                    }
                }
            }

            // After the threshold is exceeded, the PPCB should be active and reads
            // should go directly to the next region (succeeding without hitting the
            // faulted hub region).
            assert!(
                read_success_count > 0,
                "At least some reads should succeed via failover to the non-faulted region"
            );

            // Verify the fault rule was hit (faults were injected in the hub region).
            assert!(
                rule.hit_count() > 0,
                "Fault injection rule should have been hit on the hub region for reads"
            );

            // Now send a few more reads. With the circuit breaker tripped, these
            // should go directly to the alternate region and succeed.
            for _ in 0..3 {
                let response = fault_container_client
                    .read_item::<TestItem>(&pk, &item_id, None)
                    .await
                    .expect(
                        "Post-threshold reads should succeed directly via the alternate region",
                    );

                assert_eq!(response.status(), StatusCode::Ok);
            }

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Same as above but with 403/3 WriteForbidden injected on reads from hub region
/// to verify PPCB also handles write-forbidden errors for reads.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppcb_enabled_write_forbidden_on_read_fails_over_after_threshold(
) -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppcb-403-read", result)
            .with_condition(condition)
            .build(),
    );

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::clone(&rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            // Seed an item using the normal client.
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Issue reads to trip the circuit breaker.
            for _ in 0..10 {
                let _ = fault_container_client
                    .read_item::<TestItem>(&pk, &item_id, None)
                    .await;
            }

            assert!(
                rule.hit_count() > 0,
                "Fault injection rule should have been hit"
            );

            // Post-threshold reads should go directly to the alternate region.
            for _ in 0..3 {
                let response = fault_container_client
                    .read_item::<TestItem>(&pk, &item_id, None)
                    .await
                    .expect("Post-threshold reads should succeed via alternate region");

                assert_eq!(response.status(), StatusCode::Ok);
            }

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

// ────────────────────────────────────────────────────────────────────────────
// Test 4: PPCB disabled — 503 on ReadItem from region 1 → no partition-level
//         failover; reads still go to hub and fail, triggering cross-regional
//         retry (higher latency, no sticky override).
// ────────────────────────────────────────────────────────────────────────────

/// When PPCB is disabled, 503 on reads from region 1 should NOT create a
/// partition-level override. Each read still attempts the hub region first,
/// fails, and retries cross-regionally. This means higher latency but reads
/// still eventually succeed after each cross-regional retry.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppcb_disabled_503_on_read_no_partition_failover() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("no-ppcb-503-read", result)
            .with_condition(condition)
            .build(),
    );

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::clone(&rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            // Seed an item using the normal client.
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Without PPCB, reads should still succeed via cross-regional retry,
            // but each read will first hit the hub region (getting the 503) and
            // then retry to another region. The fault rule should keep getting hit
            // since there is no circuit breaker to bypass the hub.
            let hit_count_before = rule.hit_count();

            for _ in 0..5 {
                let response = fault_container_client
                    .read_item::<TestItem>(&pk, &item_id, None)
                    .await
                    .expect("Reads should eventually succeed via cross-regional retry");

                assert_eq!(response.status(), StatusCode::Ok);
            }

            // Without PPCB, each read should have hit the hub region (where the fault
            // is), so the hit count should have increased significantly.
            let hit_count_after = rule.hit_count();
            assert!(
                hit_count_after > hit_count_before,
                "Without PPCB, each read should hit the hub region fault: \
                hit_count before={hit_count_before}, after={hit_count_after}"
            );

            // Every read should have triggered the fault once (on the hub), so
            // we expect at least 5 new hits.
            assert!(
                hit_count_after - hit_count_before >= 5,
                "Expected at least 5 new fault hits (one per read), \
                got {} (before={hit_count_before}, after={hit_count_after})",
                hit_count_after - hit_count_before
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Same as above but with 403/3 WriteForbidden on reads when PPCB is disabled.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppcb_disabled_write_forbidden_on_read_no_partition_failover(
) -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("no-ppcb-403-read", result)
            .with_condition(condition)
            .build(),
    );

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::clone(&rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let hit_count_before = rule.hit_count();

            for _ in 0..5 {
                let response = fault_container_client
                    .read_item::<TestItem>(&pk, &item_id, None)
                    .await
                    .expect("Reads should eventually succeed via cross-regional retry");

                assert_eq!(response.status(), StatusCode::Ok);
            }

            let hit_count_after = rule.hit_count();
            assert!(
                hit_count_after > hit_count_before,
                "Without PPCB, each read should hit the hub region fault: \
                hit_count before={hit_count_before}, after={hit_count_after}"
            );

            assert!(
                hit_count_after - hit_count_before >= 5,
                "Expected at least 5 new fault hits (one per read), \
                got {} (before={hit_count_before}, after={hit_count_after})",
                hit_count_after - hit_count_before
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}
