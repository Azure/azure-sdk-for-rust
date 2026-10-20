// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request-level hedge-feedback regressions without public inspection hooks.

use super::CosmosDriver;
use crate::{
    diagnostics::HedgeTerminalState,
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRuleBuilder,
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
