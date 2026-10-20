// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition topology cache loading-mode tests.

use std::{borrow::Cow, num::NonZeroU32, sync::Arc};

use azure_core::http::Url;
use azure_data_cosmos_driver::{
    driver::CosmosDriver,
    in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
        VirtualRegion, WriteMode,
    },
    models::{
        AccountReference, ContainerReference, CosmosOperation, FeedRange, ItemReference,
        MaxItemCountHint, PartitionKey, PartitionKeyDefinition,
    },
    options::{
        DriverOptions, OperationOptions, PartitionFailoverOptions, PartitionTopologyCacheMode,
        PlanOptions,
    },
};

#[cfg(feature = "fault_injection")]
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};

use super::{host_recorder::HostRecorder, page_document_values, GATEWAY_URL};

fn account() -> AccountReference {
    AccountReference::with_account_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5")
}

fn emulator(recorder: Arc<HostRecorder>) -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Eventual);
    emulator_with_config(recorder, config)
}

fn emulator_with_config(
    recorder: Arc<HostRecorder>,
    config: VirtualAccountConfig,
) -> Arc<InMemoryEmulatorHttpClient> {
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

async fn create_items(driver: &CosmosDriver, container: &ContainerReference) {
    for id in ["one", "two", "three"] {
        let item = ItemReference::from_name(container, PartitionKey::from("key"), id.to_owned());
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(item).with_body(
                    serde_json::to_vec(&serde_json::json!({"id": id, "pk": "key"})).unwrap(),
                ),
                OperationOptions::default(),
            )
            .await
            .unwrap();
    }
}

fn logical_query(container: &ContainerReference) -> CosmosOperation {
    CosmosOperation::query_items(
        container.clone(),
        Some(FeedRange::for_partition(
            PartitionKey::from("key"),
            container.partition_key_definition(),
        )),
    )
    .with_body(br#"{"query":"SELECT * FROM c","parameters":[]}"#.to_vec())
    .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()))
}

#[tokio::test]
async fn logical_reads_and_query_pages_preserve_eager_and_lazy_contracts() {
    for (mode, warm) in [
        (PartitionTopologyCacheMode::Lazy, false),
        (PartitionTopologyCacheMode::Lazy, true),
        (PartitionTopologyCacheMode::Eager, false),
    ] {
        let recorder = HostRecorder::new();
        let emulator = emulator(recorder.clone());
        let driver = driver(&emulator, mode).await;
        let container = driver
            .resolve_container("testdb", "testcoll", OperationOptions::default())
            .await
            .unwrap();
        assert_eq!(
            recorder.routing_metadata_count(),
            if mode == PartitionTopologyCacheMode::Eager {
                2
            } else {
                0
            }
        );
        recorder.clear();
        create_items(&driver, &container).await;
        assert_eq!(recorder.routing_metadata_count(), 0);
        if warm {
            assert_eq!(
                driver
                    .resolve_all_partition_key_ranges(&container, false)
                    .await
                    .unwrap()
                    .unwrap()
                    .len(),
                2
            );
            assert_eq!(recorder.routing_metadata_count(), 2);
            recorder.clear();
        }

        let item =
            ItemReference::from_name(&container, PartitionKey::from("key"), "one".to_owned());
        let response = driver
            .execute_singleton_operation(
                CosmosOperation::read_item(item),
                OperationOptions::default(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
        let document = super::parse_json_body(&response.into_body().single().unwrap()).unwrap();
        assert_eq!(document["id"], "one");
        assert_eq!(document["pk"], "key");
        let mut plan = driver
            .plan_operation(
                logical_query(&container),
                &OperationOptions::default(),
                None,
                &PlanOptions::default(),
            )
            .await
            .unwrap();
        assert_eq!(recorder.routing_metadata_count(), 0);
        let mut ids = Vec::new();
        while let Some(page) = driver
            .execute_plan(
                &mut plan,
                Some(container.clone()),
                OperationOptions::default(),
            )
            .await
            .unwrap()
        {
            ids.extend(
                page_document_values(page)
                    .into_iter()
                    .map(|item| item["id"].as_str().unwrap().to_owned()),
            );
            assert_eq!(recorder.routing_metadata_count(), 0);
        }
        ids.sort();
        assert_eq!(ids, ["one", "three", "two"]);
        assert_eq!(recorder.document_query_count(), 3);
    }
}

#[tokio::test]
async fn lazy_partition_features_load_topology_only_for_eligible_operations() {
    // (regions, multi-write, PPCB option, account failover, read, expected fetch)
    for (regions, multi, ppcb, ppaf, read, required) in [
        (1, false, true, false, true, false),
        (2, false, false, false, true, false),
        (2, false, true, false, true, true),
        (2, false, true, false, false, false),
        (2, true, true, false, false, true),
        (2, false, false, true, false, true),
        (2, false, false, true, true, true),
    ] {
        let recorder = HostRecorder::new();
        let locations = [
            VirtualRegion::new("East US", Url::parse(GATEWAY_URL).unwrap()),
            VirtualRegion::new(
                "West US",
                Url::parse("https://westus.emulator.local").unwrap(),
            ),
        ];
        let config = VirtualAccountConfig::new(locations.into_iter().take(regions).collect())
            .unwrap()
            .with_consistency(ConsistencyLevel::Eventual)
            .with_write_mode(if multi {
                WriteMode::Multi
            } else {
                WriteMode::Single
            })
            .with_per_partition_failover(ppaf);
        let emulator = emulator_with_config(recorder.clone(), config);
        let driver = emulator
            .runtime_builder()
            .build()
            .await
            .unwrap()
            .create_driver(
                DriverOptions::builder(account())
                    .with_partition_failover_options(
                        PartitionFailoverOptions::builder()
                            .with_partition_topology_cache_mode(PartitionTopologyCacheMode::Lazy)
                            .with_circuit_breaker_enabled(ppcb)
                            .build()
                            .unwrap(),
                    )
                    .build(),
            )
            .await
            .unwrap();
        let container = driver
            .resolve_container("testdb", "testcoll", OperationOptions::default())
            .await
            .unwrap();
        let item =
            ItemReference::from_name(&container, PartitionKey::from("key"), "one".to_owned());
        let operation = if read {
            CosmosOperation::read_item(item)
        } else {
            CosmosOperation::create_item(item).with_body(br#"{"id":"one","pk":"key"}"#.to_vec())
        };
        let result = driver
            .execute_singleton_operation(operation, OperationOptions::default())
            .await;
        if read {
            assert_eq!(
                result.unwrap_err().status().status_code(),
                azure_core::http::StatusCode::NotFound
            );
        } else {
            assert_eq!(
                result.unwrap().status(),
                azure_core::http::StatusCode::Created
            );
        }
        assert_eq!(
            recorder.routing_metadata_count(),
            if required { 2 } else { 0 },
            "regions={regions}, multi={multi}, ppcb={ppcb}, ppaf={ppaf}, read={read}"
        );
    }
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
async fn lazy_logical_topology_recovery_does_not_fetch_optional_identity() {
    let recorder = HostRecorder::new();
    let emulator = emulator(recorder.clone());
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "logical-range-gone",
            FaultInjectionResultBuilder::new()
                .with_error(FaultInjectionErrorType::PartitionIsGone)
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .build(),
        )
        .with_hit_limit(1)
        .build(),
    );
    let runtime = emulator
        .runtime_builder_with_fault_rules(vec![rule.clone()])
        .build()
        .await
        .unwrap();
    let driver = runtime
        .create_driver(options(PartitionTopologyCacheMode::Lazy))
        .await
        .unwrap();
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    create_items(&driver, &container).await;
    driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .unwrap()
        .unwrap();
    recorder.clear();

    let item = ItemReference::from_name(&container, PartitionKey::from("key"), "one".to_owned());
    let response = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(item),
            OperationOptions::default(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
    assert_eq!(rule.hit_count(), 1);
    assert_eq!(recorder.routing_metadata_count(), 0);
}

#[tokio::test(start_paused = true)]
async fn lazy_query_rechecks_feature_requirements_between_pages() {
    let recorder = HostRecorder::new();
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(GATEWAY_URL).unwrap()),
        VirtualRegion::new(
            "West US",
            Url::parse("https://westus.emulator.local").unwrap(),
        ),
    ])
    .unwrap()
    .with_consistency(ConsistencyLevel::Eventual);
    let emulator = emulator_with_config(recorder.clone(), config);
    let driver = driver(&emulator, PartitionTopologyCacheMode::Lazy).await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    create_items(&driver, &container).await;
    let mut plan = driver
        .plan_operation(
            logical_query(&container),
            &OperationOptions::default(),
            None,
            &PlanOptions::default(),
        )
        .await
        .unwrap();
    let first = driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(page_document_values(first).len(), 1);
    assert_eq!(recorder.routing_metadata_count(), 0);

    emulator.store().config().set_per_partition_failover(true);
    tokio::time::sleep(std::time::Duration::from_secs(600)).await;
    assert!(driver.is_per_partition_automatic_failover_enabled_for_testing());
    let second = driver
        .execute_plan(&mut plan, Some(container), OperationOptions::default())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(page_document_values(second).len(), 1);
    assert_eq!(recorder.routing_metadata_count(), 2);
}

#[tokio::test]
async fn lazy_fanout_query_and_change_feed_still_load_topology() {
    for change_feed in [false, true] {
        let recorder = HostRecorder::new();
        let emulator = emulator(recorder.clone());
        let driver = driver(&emulator, PartitionTopologyCacheMode::Lazy).await;
        let container = driver
            .resolve_container("testdb", "testcoll", OperationOptions::default())
            .await
            .unwrap();
        let operation = if change_feed {
            CosmosOperation::change_feed(container.clone(), Some(FeedRange::full()))
        } else {
            CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
                .with_body(br#"{"query":"SELECT * FROM c","parameters":[]}"#.to_vec())
        };
        let mut plan = driver
            .plan_operation(
                operation,
                &OperationOptions::default(),
                None,
                &PlanOptions::default(),
            )
            .await
            .unwrap();
        assert_eq!(recorder.routing_metadata_count(), 2);
        driver
            .execute_plan(&mut plan, Some(container), OperationOptions::default())
            .await
            .unwrap();
        assert_eq!(recorder.routing_metadata_count(), 2);
    }
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn lazy_topology_failure_only_affects_required_resolution() {
    for regions in [1, 2] {
        let recorder = HostRecorder::new();
        let locations = [
            VirtualRegion::new("East US", Url::parse(GATEWAY_URL).unwrap()),
            VirtualRegion::new(
                "West US",
                Url::parse("https://westus.emulator.local").unwrap(),
            ),
        ];
        let config = VirtualAccountConfig::new(locations.into_iter().take(regions).collect())
            .unwrap()
            .with_consistency(ConsistencyLevel::Eventual);
        let emulator = emulator_with_config(recorder, config);
        let rule = topology_failure_rule();
        let runtime = emulator
            .runtime_builder_with_fault_rules(vec![rule.clone()])
            .build()
            .await
            .unwrap();
        let driver = runtime
            .create_driver(
                DriverOptions::builder(account())
                    .with_partition_failover_options(
                        PartitionFailoverOptions::builder()
                            .with_partition_topology_cache_mode(PartitionTopologyCacheMode::Lazy)
                            .with_circuit_breaker_enabled(true)
                            .build()
                            .unwrap(),
                    )
                    .build(),
            )
            .await
            .unwrap();
        let container = driver
            .resolve_container("testdb", "testcoll", OperationOptions::default())
            .await
            .unwrap();
        create_items(&driver, &container).await;
        assert_eq!(
            rule.hit_count(),
            0,
            "single-writer creates do not require PPCB topology"
        );

        let item =
            ItemReference::from_name(&container, PartitionKey::from("key"), "one".to_owned());
        let response = driver
            .execute_singleton_operation(
                CosmosOperation::read_item(item),
                OperationOptions::default(),
            )
            .await
            .expect("logical routing survives metadata failures");
        assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
        let document = super::parse_json_body(&response.into_body().single().unwrap()).unwrap();
        assert_eq!(document["id"], "one");
        if regions == 1 {
            assert_eq!(rule.hit_count(), 0);
        } else {
            assert!(
                rule.hit_count() > 0,
                "eligible PPCB read must attempt topology resolution"
            );
        }
    }
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
