// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{sync::Arc, time::Duration};

use azure_data_cosmos::{
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    },
    options::{
        AvailabilityStrategy, HedgeThreshold, HedgingStrategy, ItemReadOptions,
        OperationOptionsBuilder, PartitionFailoverOptions, Region,
    },
    RoutingStrategy,
};

use crate::e2e_test_cases::{
    fixture::{build_client_with_customizer, ClientSetup, E2eTest, TestResult},
    support::{item, selected_scenario_profile, wait_for_item_replication_in_region, Item},
    topology_support::{with_topology_reset, TopologyManager},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn partition_breaker_avoids_then_probes_recovered_region() -> TestResult {
    let Some(profile) = selected_scenario_profile("resilience.partition-circuit-breaker").await?
    else {
        return Ok(());
    };
    let fault = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::new("East US"))
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("e2e-ppcb-east", fault)
            .with_condition(condition)
            .with_hit_limit(2)
            .build(),
    );
    rule.disable();
    let options = PartitionFailoverOptions::builder()
        .with_circuit_breaker_enabled(true)
        .with_read_failure_threshold(2)
        .with_counter_reset_window(Duration::from_secs(30))
        .with_partition_unavailability_duration(Duration::from_secs(1))
        .with_failback_sweep_interval(Duration::from_secs(1))
        .build()?;
    let setup = ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![
            Region::new("East US"),
            Region::new("West US"),
            Region::new("North Europe"),
        ]),
    )?;
    let client = build_client_with_customizer(setup, |builder| {
        Ok(builder
            .with_partition_failover_options(options)
            .with_fault_injection_rules(vec![Arc::clone(&rule)])?)
    })
    .await?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let expected = item("partition-breaker", "A", 81);
            fixture
                .container
                .create_item("A", &expected.id, &expected, None)
                .await?;
            wait_for_item_replication_in_region(
                &fixture.container,
                "A",
                &expected.id,
                &expected,
                Region::new("West US"),
                [
                    Region::new("East US"),
                    Region::new("West US"),
                    Region::new("North Europe"),
                ],
            )
            .await?;
            rule.enable();

            let mut operation = OperationOptionsBuilder::new()
                .with_availability_strategy(AvailabilityStrategy::Disabled)
                .build();
            operation.endpoint_unavailability_ttl = Some(Duration::ZERO);
            let first = fixture
                .container
                .read_item(
                    "A",
                    &expected.id,
                    Some(ItemReadOptions::default().with_operation_options(operation.clone())),
                )
                .await?;
            assert_eq!(first.into_model::<Item>()?, expected);
            assert_eq!(rule.hit_count(), 1);

            let second = fixture
                .container
                .read_item(
                    "A",
                    &expected.id,
                    Some(ItemReadOptions::default().with_operation_options(operation.clone())),
                )
                .await?;
            assert_eq!(second.into_model::<Item>()?, expected);
            assert_eq!(rule.hit_count(), 2);
            rule.disable();

            let avoided = fixture
                .container
                .read_item(
                    "A",
                    &expected.id,
                    Some(ItemReadOptions::default().with_operation_options(operation.clone())),
                )
                .await?;
            let avoided_diagnostics = avoided.diagnostics();
            assert_eq!(avoided.into_model::<Item>()?, expected);
            assert_eq!(
                avoided_diagnostics.regions_contacted().first(),
                Some(&Region::new("West US")),
                "tripped partition must avoid East US"
            );

            let deadline = tokio::time::Instant::now() + Duration::from_secs(6);
            tokio::time::sleep(Duration::from_secs(2)).await;
            loop {
                let probe = fixture
                    .container
                    .read_item(
                        "A",
                        &expected.id,
                        Some(ItemReadOptions::default().with_operation_options(operation.clone())),
                    )
                    .await?;
                let diagnostics = probe.diagnostics();
                assert_eq!(probe.into_model::<Item>()?, expected);
                if diagnostics.regions_contacted().first() == Some(&Region::new("East US")) {
                    break;
                }
                if tokio::time::Instant::now() >= deadline {
                    return Err("PPCB did not probe East US before the bounded deadline".into());
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            Ok(())
        })
        .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn hedge_completes_while_preferred_region_transitions_offline() -> TestResult {
    let Some(profile) = selected_scenario_profile("resilience.hedging-topology").await? else {
        return Ok(());
    };
    let delay = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_secs(1))
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::new("East US"))
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("e2e-topology-hedge", delay)
            .with_condition(condition)
            .build(),
    );
    let setup = ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![
            Region::new("East US"),
            Region::new("West US"),
            Region::new("North Europe"),
        ]),
    )?;
    let client = build_client_with_customizer(setup, |builder| {
        Ok(builder.with_fault_injection_rules(vec![Arc::clone(&rule)])?)
    })
    .await?;
    let manager = TopologyManager::from_env()?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            with_topology_reset(&manager, async {
                let expected = item("hedging-topology", "A", 82);
                fixture
                    .container
                    .create_item("A", &expected.id, &expected, None)
                    .await?;
                wait_for_item_replication_in_region(
                    &fixture.container,
                    "A",
                    &expected.id,
                    &expected,
                    Region::new("West US"),
                    [
                        Region::new("East US"),
                        Region::new("West US"),
                        Region::new("North Europe"),
                    ],
                )
                .await?;

                let operation = OperationOptionsBuilder::new()
                    .with_availability_strategy(AvailabilityStrategy::Hedging(
                        HedgingStrategy::new(
                            HedgeThreshold::new(Duration::from_millis(100))
                                .expect("non-zero hedge threshold"),
                        ),
                    ))
                    .build();
                let container = fixture.container.clone();
                let item_id = expected.id.clone();
                let read = tokio::spawn(async move {
                    container
                        .read_item(
                            "A",
                            &item_id,
                            Some(ItemReadOptions::default().with_operation_options(operation)),
                        )
                        .await
                });
                let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
                while rule.hit_count() == 0 {
                    if tokio::time::Instant::now() >= deadline {
                        return Err("preferred hedge leg did not enter its injected delay".into());
                    }
                    tokio::task::yield_now().await;
                }
                manager.offline("East US").await?;
                let response = read.await??;
                let diagnostics = response.diagnostics();
                assert_eq!(response.into_model::<Item>()?, expected);
                let hedge = diagnostics
                    .hedge_diagnostics()
                    .expect("delayed preferred leg must launch a hedge");
                assert_eq!(hedge.terminal_state().as_str(), "alternate_won");
                assert_eq!(hedge.primary_region(), &Region::new("East US"));
                assert_eq!(hedge.response_region(), Some(&Region::new("West US")));
                assert_eq!(manager.account().await?.write_region, "West US");
                Ok(())
            })
            .await
        })
        .await
}
