// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request-level hedge-feedback regressions without public inspection hooks.

use super::CosmosDriver;
use crate::{
    diagnostics::HedgeTerminalState,
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResult,
        FaultInjectionResultBuilder, FaultInjectionRule, FaultInjectionRuleBuilder,
        FaultOperationType,
    },
    in_memory_emulator::{
        ContainerConfig, InMemoryEmulatorHttpClient, ReplicationConfig, VirtualAccountConfig,
        VirtualRegion, WriteMode,
    },
    models::{AccountReference, CosmosOperation, ItemReference, PartitionKey},
    options::{
        AvailabilityStrategy, DriverOptions, HedgeThreshold, HedgingStrategy, OperationOptions,
        OperationOptionsBuilder, PartitionFailoverOptions, Region,
    },
};
use std::{collections::BTreeMap, sync::Arc, time::Duration};
use url::Url;

async fn read(
    driver: &CosmosDriver,
    item: &ItemReference,
    options: OperationOptions,
) -> HedgeTerminalState {
    let response = driver
        .execute_singleton_operation(CosmosOperation::read_item(item.clone()), options)
        .await
        .unwrap();
    response
        .diagnostics()
        .hedge_diagnostics()
        .unwrap()
        .terminal_state()
}

#[tokio::test(start_paused = true)]
async fn feedback_respects_ppcb_without_disabling_transport_hedging() {
    for enabled in [false, true] {
        let east = Url::parse("https://eastus.emulator.local").unwrap();
        let config = VirtualAccountConfig::new(vec![
            VirtualRegion::new("East US", east.clone()),
            VirtualRegion::new(
                "West US",
                Url::parse("https://westus.emulator.local").unwrap(),
            ),
        ])
        .unwrap()
        .with_write_mode(WriteMode::Single)
        .with_replication_config(ReplicationConfig::immediate());
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
        emulator.store().create_database("repro");
        emulator.store().create_container_with_config(
            "repro",
            "items",
            serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))
                .unwrap(),
            ContainerConfig::new()
                .with_partition_count(16)
                .build()
                .unwrap(),
        );
        let rule = Arc::new(
            FaultInjectionRuleBuilder::new(
                "repro-primary-delay",
                FaultInjectionResultBuilder::new()
                    .with_delay(Duration::from_millis(200))
                    .with_probability(1.0)
                    .build(),
            )
            .with_condition(
                FaultInjectionConditionBuilder::new()
                    .with_operation_type(FaultOperationType::ReadItem)
                    .with_region(Region::EAST_US)
                    .build(),
            )
            .build(),
        );
        rule.disable();
        let runtime = emulator
            .runtime_builder_with_fault_rules(vec![rule.clone()])
            .build()
            .await
            .unwrap();
        let driver = runtime
            .create_driver(
                DriverOptions::builder(AccountReference::with_account_key(
                    east,
                    "ZW11bGF0b3JrZXk=",
                ))
                .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
                .with_partition_failover_options(
                    PartitionFailoverOptions::builder()
                        .with_circuit_breaker_enabled(enabled)
                        .with_circuit_breaker_enabled_override(enabled)
                        .build()
                        .unwrap(),
                )
                .build(),
            )
            .await
            .unwrap();
        let container = driver
            .resolve_container("repro", "items", OperationOptions::default())
            .await
            .unwrap();
        let mut items = BTreeMap::new();
        for candidate in 0..10_000 {
            let key = format!("key-{candidate}");
            let pk = PartitionKey::from(key.clone());
            let ranges = driver
                .resolve_partition_key_ranges_for_key(&container, &pk, false)
                .await
                .unwrap()
                .unwrap();
            assert_eq!(ranges.len(), 1);
            if items.contains_key(&ranges[0].id) {
                continue;
            }
            let item = ItemReference::from_name(&container, pk, key.clone());
            driver
                .execute_singleton_operation(
                    CosmosOperation::create_item(item.clone()).with_body(
                        serde_json::to_vec(&serde_json::json!({"id":key,"pk":key})).unwrap(),
                    ),
                    OperationOptions::default(),
                )
                .await
                .unwrap();
            items.insert(ranges[0].id.clone(), item);
            if items.len() == 16 {
                break;
            }
        }
        assert_eq!(items.len(), 16, "must cover distinct physical ranges");
        let items: Vec<_> = items.into_values().collect();
        let options = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Hedging(HedgingStrategy::new(
                HedgeThreshold::new(Duration::from_millis(20)).unwrap(),
            )))
            .build();
        let store = &driver.location_state_store;
        assert_eq!(
            store
                .snapshot()
                .partitions
                .per_partition_circuit_breaker_enabled,
            enabled
        );

        // Warm the request path without leaving any alternate-win counters.
        for item in &items {
            assert_eq!(
                read(&driver, item, options.clone()).await,
                HedgeTerminalState::PrimaryWonPreThreshold
            );
        }
        assert!(store
            .snapshot()
            .partitions
            .consecutive_hedge_wins
            .is_empty());
        rule.enable();
        let hits = rule.hit_count();
        for item in &items[..8] {
            assert_eq!(
                read(&driver, item, options.clone()).await,
                HedgeTerminalState::AlternateWon
            );
        }
        assert_eq!(rule.hit_count() - hits, 8);
        rule.disable();
        let accumulated = store.snapshot();
        assert_eq!(
            accumulated.partitions.consecutive_hedge_wins.len(),
            if enabled { 8 } else { 0 }
        );
        assert!(accumulated.partitions.circuit_breaker_overrides.is_empty());

        let version = store.version_for_tests();
        for item in &items[8..] {
            assert_eq!(
                read(&driver, item, options.clone()).await,
                HedgeTerminalState::PrimaryWonPreThreshold
            );
        }
        let after = store.snapshot();
        assert_eq!(
            accumulated.partitions.consecutive_hedge_wins,
            after.partitions.consecutive_hedge_wins
        );
        assert_eq!(store.version_for_tests(), version);
        assert!(Arc::ptr_eq(&accumulated.partitions, &after.partitions));

        let disabled = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Disabled)
            .build();
        let version = store.version_for_tests();
        rule.enable();
        let hits = rule.hit_count();
        let response = driver
            .execute_singleton_operation(CosmosOperation::read_item(items[8].clone()), disabled)
            .await
            .unwrap();
        rule.disable();
        assert!(response.diagnostics().hedge_diagnostics().is_none());
        assert_eq!(rule.hit_count() - hits, 1);
        assert_eq!(store.version_for_tests(), version);
    }
}

#[tokio::test(start_paused = true)]
async fn hedge_win_with_hub_region_header_caches_hub_region() {
    let slow_primary = read_rule(
        "west-slow",
        Region::WEST_US,
        FaultInjectionResultBuilder::new()
            .with_delay(Duration::from_millis(200))
            .with_probability(1.0)
            .build(),
    );
    let session_unavailable = read_rule(
        "west-1002",
        Region::WEST_US,
        error_result(FaultInjectionErrorType::ReadSessionNotAvailable),
    );
    let (driver, item, range_id) = hub_fixture(
        &["East US", "West US"],
        vec![Region::WEST_US, Region::EAST_US],
        vec![slow_primary.clone(), session_unavailable.clone()],
    )
    .await;
    let store = &driver.location_state_store;

    // Without the hub-only header, the winning region says nothing about the hub.
    slow_primary.enable();
    assert_eq!(
        read(&driver, &item, hedging_options()).await,
        HedgeTerminalState::AlternateWon
    );
    slow_primary.disable();
    assert!(store.snapshot().partitions.failover_overrides.is_empty());

    // A fast 1002 on the primary latches the header before the alternate (write region) leg is built.
    session_unavailable.enable();
    assert_eq!(
        read(&driver, &item, hedging_options()).await,
        HedgeTerminalState::AlternateWon
    );
    session_unavailable.disable();
    assert_eq!(cached_hub(&driver, &range_id), Some(Region::EAST_US));
}

#[tokio::test(start_paused = true)]
async fn hedged_hub_only_403_continues_to_hub() {
    // A fast 1002 latches the header onto the alternate, which is not the hub and answers 403/3.
    let session_unavailable = read_rule(
        "west-1002",
        Region::WEST_US,
        error_result(FaultInjectionErrorType::ReadSessionNotAvailable),
    );
    let not_hub = read_rule(
        "central-403-3",
        Region::CENTRAL_US,
        error_result(FaultInjectionErrorType::WriteForbidden),
    );
    let (driver, item, range_id) = hub_fixture(
        &["East US", "West US", "Central US"],
        vec![Region::WEST_US, Region::CENTRAL_US, Region::EAST_US],
        vec![session_unavailable.clone(), not_hub.clone()],
    )
    .await;
    session_unavailable.enable();
    not_hub.enable();

    let response = driver
        .execute_singleton_operation(CosmosOperation::read_item(item), hedging_options())
        .await
        .expect("a hub-only 403/3 on a hedge leg must not end the read before the hub is tried");

    assert!(response
        .diagnostics()
        .regions_contacted()
        .contains(&Region::EAST_US));
    assert_eq!(cached_hub(&driver, &range_id), Some(Region::EAST_US));
}

fn hedging_options() -> OperationOptions {
    OperationOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Hedging(HedgingStrategy::new(
            HedgeThreshold::new(Duration::from_millis(20)).unwrap(),
        )))
        .build()
}

fn error_result(error: FaultInjectionErrorType) -> FaultInjectionResult {
    FaultInjectionResultBuilder::new()
        .with_error(error)
        .with_probability(1.0)
        .build()
}

/// A disabled fault rule for item reads in `region`.
fn read_rule(id: &str, region: Region, result: FaultInjectionResult) -> Arc<FaultInjectionRule> {
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(id, result)
            .with_condition(
                FaultInjectionConditionBuilder::new()
                    .with_operation_type(FaultOperationType::ReadItem)
                    .with_region(region)
                    .build(),
            )
            .build(),
    );
    rule.disable();
    rule
}

/// The region the hub cache holds for `range_id`, if any.
fn cached_hub(driver: &CosmosDriver, range_id: &str) -> Option<Region> {
    driver
        .location_state_store
        .snapshot()
        .partitions
        .failover_overrides
        .get(range_id)
        .and_then(|entry| entry.current_endpoint.region().cloned())
}

/// Single-partition PPAF account whose first region is the write region, with one seeded
/// item and a reader preferring `preferred`. Returns the reader, the item, and its range id.
async fn hub_fixture(
    regions: &[&'static str],
    preferred: Vec<Region>,
    rules: Vec<Arc<FaultInjectionRule>>,
) -> (Arc<CosmosDriver>, ItemReference, String) {
    let endpoint = |name: &str| {
        Url::parse(&format!(
            "https://{}.emulator.local",
            name.replace(' ', "").to_lowercase()
        ))
        .unwrap()
    };
    let config = VirtualAccountConfig::new(
        regions
            .iter()
            .map(|name| VirtualRegion::new(name, endpoint(name)))
            .collect(),
    )
    .unwrap()
    .with_write_mode(WriteMode::Single)
    .with_replication_config(ReplicationConfig::immediate())
    .with_per_partition_failover(true);
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    emulator.store().create_database("repro");
    emulator.store().create_container_with_config(
        "repro",
        "items",
        serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))
            .unwrap(),
        ContainerConfig::new()
            .with_partition_count(1)
            .build()
            .unwrap(),
    );
    let runtime = emulator
        .runtime_builder_with_fault_rules(rules)
        .build()
        .await
        .unwrap();
    let account = AccountReference::with_account_key(endpoint(regions[0]), "ZW11bGF0b3JrZXk=");
    // Seeding through the reader would record a PPAF write override, pre-filling the cache under test.
    let writer = runtime
        .create_driver(
            DriverOptions::builder(account.clone())
                .with_preferred_regions(vec![Region::new(regions[0])])
                .build(),
        )
        .await
        .unwrap();
    let driver = runtime
        .create_driver(
            DriverOptions::builder(account)
                .with_preferred_regions(preferred)
                .build(),
        )
        .await
        .unwrap();
    let container = driver
        .resolve_container("repro", "items", OperationOptions::default())
        .await
        .unwrap();
    let pk = PartitionKey::from("key");
    let item = ItemReference::from_name(&container, pk.clone(), "key");
    writer
        .execute_singleton_operation(
            CosmosOperation::create_item(item.clone()).with_body(
                serde_json::to_vec(&serde_json::json!({"id":"key","pk":"key"})).unwrap(),
            ),
            OperationOptions::default(),
        )
        .await
        .unwrap();
    let range_id = driver
        .resolve_partition_key_ranges_for_key(&container, &pk, false)
        .await
        .unwrap()
        .unwrap()[0]
        .id
        .clone();
    (driver, item, range_id)
}
