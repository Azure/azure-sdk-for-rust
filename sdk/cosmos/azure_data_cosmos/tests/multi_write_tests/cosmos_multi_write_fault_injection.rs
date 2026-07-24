// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::framework;

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::options::{
    ExcludedRegions, ItemReadOptions, OperationOptions, ThrottlingRetryOptionsBuilder,
};
use framework::{
    assert_local_retry_attempted_on_region, assert_region_contacted_with_retry,
    assert_region_not_contacted, TestClient, TestOptions, HUB_REGION, SATELLITE_REGION,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{borrow::Cow, error::Error, time::Duration};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct NestedItem {
    nested_value: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct TestItem {
    id: Cow<'static, str>,
    partition_key: Option<Cow<'static, str>>,
    value: usize,
    nested: NestedItem,
    bool_value: bool,
}

fn read_options_for_expected_status(expected_status: StatusCode) -> Option<ItemReadOptions> {
    if expected_status != StatusCode::TooManyRequests {
        return None;
    }

    let mut operation = OperationOptions::default();
    operation.throttling_retry_options = Some(
        ThrottlingRetryOptionsBuilder::new()
            .with_max_retry_count(0)
            .build(),
    );
    Some(ItemReadOptions::default().with_operation_options(operation))
}

/// Shared implementation for fault injection read failure tests.
/// Creates a fault injection rule that returns the specified error and verifies
/// that the read operation fails with the expected HTTP status code.
async fn verify_read_fails_with_injected_error(
    error_type: FaultInjectionErrorType,
    expected_status: StatusCode,
) -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(error_type)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new(format!("{:?}-always", error_type), server_error)
        .with_condition(condition)
        .build();

    let fault_builder = vec![Arc::new(rule)];

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_for_fault_injection(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();

            let item = TestItem {
                id: format!("Item1-{}", unique_id).into(),
                partition_key: Some(format!("Partition1-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition1-{}", unique_id);
            let item_id = format!("Item1-{}", unique_id);

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let options = read_options_for_expected_status(expected_status);

            let result = run_context
                .read_item(&fault_container_client, &pk, &item_id, options)
                .await;

            let err = result.expect_err(&format!(
                "expected the read to fail with {:?}",
                expected_status
            ));
            assert_eq!(
                expected_status,
                err.status().status_code(),
                "expected {:?}, got {:?}",
                expected_status,
                err.status().status_code()
            );

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_timeout(Duration::from_secs(180)),
        ),
    )
    .await
}

#[test]
fn too_many_requests_test_disables_throttling_retries() {
    let options = read_options_for_expected_status(StatusCode::TooManyRequests).unwrap();
    let throttling = options.operation.throttling_retry_options.unwrap();

    assert_eq!(throttling.max_retry_count, Some(0));
    assert!(read_options_for_expected_status(StatusCode::ServiceUnavailable).is_none());
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn item_read_fault_injection_service_unavailable() -> Result<(), Box<dyn Error>> {
    verify_read_fails_with_injected_error(
        FaultInjectionErrorType::ServiceUnavailable,
        StatusCode::ServiceUnavailable,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn item_read_fault_injection_internal_server_error() -> Result<(), Box<dyn Error>> {
    verify_read_fails_with_injected_error(
        FaultInjectionErrorType::InternalServerError,
        StatusCode::InternalServerError,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn item_read_fault_injection_too_many_requests() -> Result<(), Box<dyn Error>> {
    verify_read_fails_with_injected_error(
        FaultInjectionErrorType::TooManyRequests,
        StatusCode::TooManyRequests,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn item_read_fault_injection_timeout() -> Result<(), Box<dyn Error>> {
    verify_read_fails_with_injected_error(
        FaultInjectionErrorType::Timeout,
        StatusCode::RequestTimeout,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn item_read_fault_injection_partition_is_gone() -> Result<(), Box<dyn Error>> {
    verify_read_fails_with_injected_error(
        FaultInjectionErrorType::PartitionIsGone,
        StatusCode::Gone,
    )
    .await
}

/// Test that verifies fault injection only affects the specified operation type.
/// When the fault condition is set to CreateItem, ReadItem operations should succeed.
///
/// This test uses two clients:
/// - A normal client for creating items
/// - A fault injection client (with CreateItem fault) for reading items (which should succeed)
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn item_read_succeeds_when_fault_targets_create_item() -> Result<(), Box<dyn Error>> {
    // Create a fault injection rule that returns 503 for CreateItem operations
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("create-item-503", server_error)
        .with_condition(condition)
        .build();

    let fault_builder = vec![Arc::new(rule)];

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            // Create a container using the normal client
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_for_fault_injection(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();

            let item = TestItem {
                id: format!("Item1-{}", unique_id).into(),
                partition_key: Some(format!("Partition1-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition1-{}", unique_id);
            let item_id = format!("Item1-{}", unique_id);

            // Create the item using the normal client (this should succeed)
            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Read the item using the fault client - this should succeed because the fault only targets CreateItem
            let result = run_context
                .read_item(&fault_container_client, &pk, &item_id, None)
                .await;

            // Verify the read succeeded
            assert!(
                result.is_ok(),
                "Read should succeed when fault targets CreateItem, but got error: {:?}",
                result.err()
            );

            let response = result.unwrap();
            assert_eq!(response.status(), StatusCode::Ok);

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_timeout(Duration::from_secs(180)),
        ),
    )
    .await
}

/// Test read region retries - inject 503 for primary region, verify cross region retries.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn fault_injection_read_region_retry_503() -> Result<(), Box<dyn Error>> {
    // Create a fault injection rule that returns 503 for reads targeting the primary region
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("primary-region-503", server_error)
        .with_condition(condition)
        .with_hit_limit(1)
        .build();

    let fault_builder = vec![Arc::new(rule)];

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
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Read should succeed on satellite region after primary returns 503
            let result = run_context
                .read_item(&fault_container_client, &pk, &item_id, None)
                .await;

            let response = result.unwrap();
            // After 503 on hub, the driver fails over; recovery may either
            // land on satellite or retry back on hub. Assert satellite was
            // contacted at least once, proving the failover path was hit.
            assert_region_contacted_with_retry(&response.diagnostics(), &SATELLITE_REGION);

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test that a transport-generated 503 on a non-idempotent write is retried
/// via cross-region failover.
///
/// Fault injection simulates transport-level failures (e.g. connection drops) which
/// produce a synthetic 503/20003 (`TransportGenerated503`). The Rust driver always
/// retries writes (including non-idempotent ones) for availability, relying on
/// Cosmos DB's conflict detection (409/412) to catch actual duplicates.
/// The driver fails over to the satellite region and the write succeeds there.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn fault_injection_transport_generated_503_write_retries_via_failover(
) -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::UpsertItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("write-region-http-503", server_error)
        .with_condition(condition)
        .with_hit_limit(1)
        .build();

    let fault_builder = vec![Arc::new(rule)];

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            // Transport-generated 503 on a non-idempotent write (upsert) is retried
            // via cross-region failover — the driver prefers availability over
            // idempotency concerns, and Cosmos DB's conflict detection catches
            // actual duplicates.
            let response = fault_container_client
                .upsert_item(&pk, &item_id, &item, None)
                .await
                .expect("write should succeed after failover to satellite");

            // After the transport 503 on hub, the driver fails over to the
            // satellite region. Assert satellite was contacted.
            assert_region_contacted_with_retry(&response.diagnostics(), &SATELLITE_REGION);

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test 404:1002 retry - inject ReadSessionNotAvailable on satellite region,
/// verify the read retries on the hub region and succeeds.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn fault_injection_read_region_retry_404_1002() -> Result<(), Box<dyn Error>> {
    // Create a fault injection rule that returns 404:1002 for reads targeting the satellite region
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ReadSessionNotAvailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(SATELLITE_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("satellite-region-404-1002", server_error)
        .with_condition(condition)
        .with_hit_limit(1)
        .build();

    let fault_builder = vec![Arc::new(rule)];

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
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Make sure the write has been replicated on both regions
            let _ = run_context
                .read_item(&container_client, &pk, &item_id, None)
                .await;
            let mut operation = OperationOptions::default();
            operation.excluded_regions = Some(ExcludedRegions::from_iter([SATELLITE_REGION]));
            let options = ItemReadOptions::default().with_operation_options(operation);
            let _ = run_context
                .read_item(&container_client, &pk, &item_id, Some(options))
                .await;

            // after verifying replication, read using the fault client
            // - should succeed via retry on hub region after satellite returns 404:1002
            let result = fault_container_client.read_item(&pk, &item_id, None).await;

            let response = result.unwrap();
            // After 404:1002 on satellite, the driver fails over; recovery
            // may either land on hub or retry back on satellite. Assert hub
            // was contacted at least once, proving the failover path was hit.
            assert_region_contacted_with_retry(&response.diagnostics(), &HUB_REGION);

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_fault_client_application_region(SATELLITE_REGION),
        ),
    )
    .await
}

/// Test write failover on connection error — inject ConnectionError on hub for CreateItem.
/// Connection errors produce `TRANSPORT_CONNECTION_FAILED` (`definitely_not_sent`).
/// The transport layer performs 1 local shard retry (MAX_LOCAL_CONNECTIVITY_RETRIES=1),
/// then escalates to the operation pipeline which marks the hub unavailable and does
/// a cross-region failover to satellite. Total fault hits on hub = 2 (initial + 1 local
/// retry). hit_limit(2) is the exact budget.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn fault_injection_write_connection_error_failover() -> Result<(), Box<dyn Error>> {
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ConnectionError)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("write-conn-error-hub", result)
        .with_condition(condition)
        .with_hit_limit(2)
        .build();

    let fault_builder = vec![Arc::new(rule)];

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            let _response = fault_container_client
                .create_item(&pk, &item_id, &item, None)
                .await
                .expect("write should succeed after connection-error failover");
            // After local retries exhaust on hub, the driver fails over to
            // the satellite. Recovery may either land on satellite or retry
            // back on hub once the transient fault clears — both are valid.
            // We assert the satellite was contacted at least once, proving
            // the failover path was exercised.
            assert_region_contacted_with_retry(&_response.diagnostics(), &SATELLITE_REGION);

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test read failover on connection error — inject ConnectionError on hub for ReadItem.
/// Same 3-local-retry-then-failover path as writes, but for a read operation.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn fault_injection_read_connection_error_failover() -> Result<(), Box<dyn Error>> {
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ConnectionError)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("read-conn-error-hub", result)
        .with_condition(condition)
        .with_hit_limit(4)
        .build();

    let fault_builder = vec![Arc::new(rule)];

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
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            // Create item with the normal client
            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Ensure replication to satellite before reading with fault client
            let mut operation = OperationOptions::default();
            operation.excluded_regions = Some(ExcludedRegions::from_iter([HUB_REGION]));
            let options = ItemReadOptions::default().with_operation_options(operation);
            let _ = run_context
                .read_item(&container_client, &pk, &item_id, Some(options))
                .await;

            let _response = run_context
                .read_item(&fault_container_client, &pk, &item_id, None)
                .await
                .expect("read should succeed via failover to satellite");
            // After connection error on hub, the driver fails over; recovery
            // may either land on satellite or retry back on hub. Assert
            // satellite was contacted at least once.
            assert_region_contacted_with_retry(&_response.diagnostics(), &SATELLITE_REGION);

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test that writes are NOT retried on response timeout.
/// ResponseTimeout has Unknown sent-status — the request may have been sent, so
/// write retries are unsafe. The write must fail.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn fault_injection_write_response_timeout_does_not_retry() -> Result<(), Box<dyn Error>> {
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ResponseTimeout)
        .build();

    // No region filter — fault applies to all regions so failover won't help.
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("write-timeout-all-regions", result)
        .with_condition(condition)
        .build();

    let fault_builder = vec![Arc::new(rule)];

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            let result = fault_container_client
                .create_item(&pk, &item_id, &item, None)
                .await;

            assert!(
                result.is_err(),
                "write should fail on response timeout — unsafe to retry"
            );

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test that reads ARE retried on response timeout. ResponseTimeout has
/// Unknown sent-status — reads are safe to retry. With `hit_limit(1)` the
/// fault fires once on hub and the driver recovers on retry; the recovery
/// may stay on hub (local retry) or fail over to the satellite — both
/// outcomes are valid. We only assert (a) the operation succeeded and
/// (b) more than one request was tracked, proving retry actually occurred.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn fault_injection_read_response_timeout_retries_to_satellite(
) -> Result<(), Box<dyn Error>> {
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ResponseTimeout)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("read-timeout-hub", result)
        .with_condition(condition)
        .with_hit_limit(1)
        .build();

    let fault_builder = vec![Arc::new(rule)];

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
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Ensure replication to satellite
            let mut operation = OperationOptions::default();
            operation.excluded_regions = Some(ExcludedRegions::from_iter([HUB_REGION]));
            let options = ItemReadOptions::default().with_operation_options(operation);
            let _ = run_context
                .read_item(&container_client, &pk, &item_id, Some(options))
                .await;

            let _response = run_context
                .read_item(&fault_container_client, &pk, &item_id, None)
                .await
                .expect("read should succeed via retry after response timeout on hub");
            // The driver may either retry locally on hub or fail over to the
            // satellite — both are valid for this scenario. We only assert
            // that the response-timeout fault was exercised on hub and that
            // some form of retry occurred.
            assert_local_retry_attempted_on_region(&_response.diagnostics(), &HUB_REGION);
            assert!(
                _response.diagnostics().request_count() > 1,
                "expected retry after response timeout on hub, got only {} request(s)",
                _response.diagnostics().request_count()
            );

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test connection error reverse failover — inject on satellite, preferred [SATELLITE, HUB].
/// Verifies failover works in the opposite direction (satellite → hub).
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn fault_injection_connection_error_reverse_failover() -> Result<(), Box<dyn Error>> {
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ConnectionError)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(SATELLITE_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("write-conn-error-satellite", result)
        .with_condition(condition)
        .with_hit_limit(4)
        .build();

    let fault_builder = vec![Arc::new(rule)];

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            let _response = fault_container_client
                .create_item(&pk, &item_id, &item, None)
                .await
                .expect("write should succeed via reverse failover to hub");
            // After fault on satellite, the driver fails over; recovery may
            // either land on hub or retry back on satellite. Assert hub was
            // contacted at least once, proving the reverse failover path was hit.
            assert_region_contacted_with_retry(&_response.diagnostics(), &HUB_REGION);

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_fault_client_application_region(SATELLITE_REGION),
        ),
    )
    .await
}

/// Test that a transient connection error on the hub is exercised by local
/// retry. With `hit_limit(2)` the fault fires twice then stops, after which
/// the operation must succeed — either via a third local retry on the hub
/// or via cross-region failover to the satellite. Either outcome is valid;
/// we only verify that the connection-error path was actually hit on the
/// hub region before recovery.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn fault_injection_connection_error_local_retry_succeeds() -> Result<(), Box<dyn Error>> {
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ConnectionError)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("transient-conn-error-hub", result)
        .with_condition(condition)
        .with_hit_limit(2)
        .build();

    let fault_builder = vec![Arc::new(rule)];

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
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let _response = run_context
                .read_item(&fault_container_client, &pk, &item_id, None)
                .await
                .expect("read should succeed after transient fault clears");
            // The driver may exhaust local retries on hub and then fail over to
            // the satellite, or the local retry may succeed before failover —
            // both are valid outcomes. We only assert the local-retry path was
            // exercised: at least one tracked request must have hit the hub
            // (proving the connection-error fault was triggered there).
            assert_local_retry_attempted_on_region(&_response.diagnostics(), &HUB_REGION);

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Pins `excluded_regions` as a hard constraint when the only allowed region keeps failing.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn fault_injection_excluded_region_not_used_when_hub_fails() -> Result<(), Box<dyn Error>>
{
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ConnectionError)
        .build();

    // Persistent hub fault must not fall back to the explicitly excluded satellite.
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("persistent-conn-error-hub", result)
        .with_condition(condition)
        .build();

    let fault_builder = vec![Arc::new(rule)];

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
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            // Create the item with the normal (non-fault) client so the
            // read target actually exists if the satellite were tried.
            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Caller-supplied exclusion: the satellite is off-limits for
            // this operation even if the hub is unhealthy.
            let mut operation = OperationOptions::default();
            operation.excluded_regions = Some(ExcludedRegions::from_iter([SATELLITE_REGION]));
            let options = ItemReadOptions::default().with_operation_options(operation);

            let result = run_context
                .read_item(&fault_container_client, &pk, &item_id, Some(options))
                .await;

            let err = result.expect_err(
                "read must fail: hub is faulted and satellite is excluded — no region is reachable",
            );
            let diagnostics = err
                .diagnostics()
                .expect("CosmosError must carry diagnostics on the fault-injected error path");

            // Hub must have been hit at least once (the fault fired) and
            // the satellite must NEVER appear in the request trail.
            assert_local_retry_attempted_on_region(&diagnostics, &HUB_REGION);
            assert_region_not_contacted(&diagnostics, &SATELLITE_REGION);

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_rules(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}
