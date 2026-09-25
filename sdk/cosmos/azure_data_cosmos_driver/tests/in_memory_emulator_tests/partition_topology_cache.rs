// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition topology cache loading-mode tests.

use std::{borrow::Cow, sync::Arc};

use azure_core::http::Url;
use azure_data_cosmos_driver::{
    driver::CosmosDriver,
    in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
        VirtualRegion,
    },
    models::{AccountReference, PartitionKeyDefinition},
    options::{
        DriverOptions, OperationOptions, PartitionFailoverOptions, PartitionTopologyCacheMode,
    },
};

#[cfg(feature = "fault_injection")]
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};

use super::{host_recorder::HostRecorder, GATEWAY_URL};

fn account() -> AccountReference {
    AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5")
}

fn emulator(recorder: Arc<HostRecorder>) -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Eventual);
    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(recorder));
    emulator.store().create_database("testdb");
    emulator.store().create_container_with_config(
        "testdb",
        "testcoll",
        PartitionKeyDefinition::new(vec![Cow::Borrowed("/pk")]),
        ContainerConfig::new()
            .with_partition_count(2)
            .build()
            .unwrap(),
    );
    emulator
}

fn options(mode: PartitionTopologyCacheMode) -> DriverOptions {
    DriverOptions::builder(account())
        .with_partition_failover_options(
            PartitionFailoverOptions::builder()
                .with_partition_topology_cache_mode(mode)
                .with_circuit_breaker_enabled(false)
                .build()
                .unwrap(),
        )
        .build()
}

async fn driver(
    emulator: &Arc<InMemoryEmulatorHttpClient>,
    mode: PartitionTopologyCacheMode,
) -> Arc<CosmosDriver> {
    emulator
        .runtime_builder()
        .build()
        .await
        .unwrap()
        .create_driver(options(mode))
        .await
        .unwrap()
}

#[tokio::test]
async fn eager_mode_primes_topology_during_name_resolution() {
    let recorder = HostRecorder::new();
    let emulator = emulator(recorder.clone());
    let driver = driver(&emulator, PartitionTopologyCacheMode::Eager).await;

    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();

    assert!(recorder.routing_metadata_count() > 0);
    recorder.clear();

    let ranges = driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .unwrap()
        .unwrap();
    assert!(!ranges.is_empty());
    assert_eq!(recorder.routing_metadata_count(), 0);
}

#[tokio::test]
async fn lazy_mode_defers_topology_until_first_use() {
    let recorder = HostRecorder::new();
    let emulator = emulator(recorder.clone());
    let driver = driver(&emulator, PartitionTopologyCacheMode::Lazy).await;

    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();

    assert_eq!(recorder.routing_metadata_count(), 0);

    let ranges = driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .unwrap()
        .unwrap();
    assert!(!ranges.is_empty());
    assert!(recorder.routing_metadata_count() > 0);
}

#[cfg(feature = "fault_injection")]
fn topology_failure_rule() -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::MetadataPartitionKeyRanges)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();
    Arc::new(
        FaultInjectionRuleBuilder::new("partition-topology-load", result)
            .with_condition(condition)
            .build(),
    )
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn eager_mode_fails_name_and_rid_resolution_when_topology_load_fails() {
    let recorder = HostRecorder::new();
    let emulator = emulator(recorder);
    let rule = topology_failure_rule();
    rule.disable();
    let runtime = emulator
        .runtime_builder_with_fault_rules(vec![rule.clone()])
        .build()
        .await
        .unwrap();
    let lazy_driver = runtime
        .create_driver(options(PartitionTopologyCacheMode::Lazy))
        .await
        .unwrap();
    let container = lazy_driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    let rid = container.rid().to_owned();
    rule.enable();

    let lazy_with_fault = runtime
        .create_driver(options(PartitionTopologyCacheMode::Lazy))
        .await
        .unwrap();
    let lazy_container = lazy_with_fault
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("lazy resolution should not load topology");
    assert!(lazy_with_fault
        .resolve_all_partition_key_ranges(&lazy_container, false)
        .await
        .unwrap()
        .is_none());

    let eager_by_name = runtime
        .create_driver(options(PartitionTopologyCacheMode::Eager))
        .await
        .unwrap();
    let name_error = eager_by_name
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap_err();
    assert_eq!(
        name_error.status(),
        azure_data_cosmos_driver::error::status_codes::CLIENT_TOPOLOGY_RESOLUTION_FAILED
    );

    let eager_by_rid = runtime
        .create_driver(options(PartitionTopologyCacheMode::Eager))
        .await
        .unwrap();
    let rid_error = eager_by_rid
        .resolve_container_by_rid(&rid, OperationOptions::default())
        .await
        .unwrap_err();
    assert_eq!(
        rid_error.status(),
        azure_data_cosmos_driver::error::status_codes::CLIENT_TOPOLOGY_RESOLUTION_FAILED
    );
}
