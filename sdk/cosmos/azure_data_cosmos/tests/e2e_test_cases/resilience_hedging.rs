// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{sync::Arc, time::Duration};

use azure_data_cosmos::{
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    },
    options::{
        AvailabilityStrategy, EndToEndOperationLatencyPolicy, HedgeThreshold, HedgingStrategy,
        ItemReadOptions, OperationOptionsBuilder, Region,
    },
    RoutingStrategy,
};

use crate::e2e_test_cases::{
    fixture::{build_client_with_customizer, ClientSetup, E2eTest, TestResult},
    support::{item, selected_scenario_profile, wait_for_item_replication, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn alternate_region_wins_delayed_primary_hedge() -> TestResult {
    let Some(profile) = selected_scenario_profile("resilience.hedging").await? else {
        return Ok(());
    };
    let delayed = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_millis(500))
        .with_probability(1.0)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("e2e-hedge-primary-delay", delayed)
            .with_condition(condition)
            .build(),
    );
    let setup = ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![Region::EAST_US, Region::WEST_US]),
    )?;
    let client = build_client_with_customizer(setup, |builder| {
        Ok(builder.with_fault_injection_rules(vec![rule])?)
    })
    .await?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let expected = item("hedging", "A", 3);
            fixture
                .container
                .create_item("A", &expected.id, &expected, None)
                .await?;
            wait_for_item_replication(&fixture.container, &expected.id, &expected).await?;

            let operation = OperationOptionsBuilder::new()
                .with_availability_strategy(AvailabilityStrategy::Hedging(HedgingStrategy::new(
                    HedgeThreshold::new(Duration::from_millis(50)).expect("non-zero threshold"),
                )))
                .build();
            let response = fixture
                .container
                .read_item(
                    "A",
                    &expected.id,
                    Some(ItemReadOptions::default().with_operation_options(operation)),
                )
                .await?;
            let diagnostics = response.diagnostics();
            assert_eq!(response.into_model::<Item>()?, expected);
            let hedge = diagnostics
                .hedge_diagnostics()
                .expect("delayed primary must produce hedge diagnostics");
            assert_eq!(hedge.terminal_state().as_str(), "alternate_won");
            assert_eq!(hedge.primary_region(), &Region::EAST_US);
            assert_eq!(hedge.alternate_region(), Some(&Region::WEST_US));
            assert_eq!(hedge.response_region(), Some(&Region::WEST_US));
            assert!(diagnostics.regions_contacted().contains(&Region::WEST_US));
            Ok(())
        })
        .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn deadline_cancels_active_hedge_awaiting_partner() -> TestResult {
    let Some(profile) = selected_scenario_profile("resilience.hedge-deadline").await? else {
        return Ok(());
    };
    let primary_fault = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_delay(Duration::from_millis(200))
        .with_probability(1.0)
        .build();
    let primary_condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let primary_rule = Arc::new(
        FaultInjectionRuleBuilder::new("e2e-hedge-deadline-primary-503", primary_fault)
            .with_condition(primary_condition)
            .with_hit_limit(1)
            .build(),
    );
    primary_rule.disable();

    let alternate_fault = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_secs(2))
        .with_probability(1.0)
        .build();
    let alternate_condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::WEST_US)
        .build();
    let alternate_rule = Arc::new(
        FaultInjectionRuleBuilder::new("e2e-hedge-deadline-alternate-delay", alternate_fault)
            .with_condition(alternate_condition)
            .with_hit_limit(1)
            .build(),
    );
    alternate_rule.disable();

    let observed_primary = Arc::clone(&primary_rule);
    let observed_alternate = Arc::clone(&alternate_rule);
    let setup = ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![Region::EAST_US, Region::WEST_US]),
    )?;
    let client = build_client_with_customizer(setup, |builder| {
        Ok(builder.with_fault_injection_rules(vec![primary_rule, alternate_rule])?)
    })
    .await?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let expected = item("hedge-deadline", "A", 4);
            fixture
                .container
                .create_item("A", &expected.id, &expected, None)
                .await?;
            wait_for_item_replication(&fixture.container, &expected.id, &expected).await?;

            observed_primary.enable();
            observed_alternate.enable();
            let operation = OperationOptionsBuilder::new()
                .with_availability_strategy(AvailabilityStrategy::Hedging(HedgingStrategy::new(
                    HedgeThreshold::new(Duration::from_millis(50)).expect("non-zero threshold"),
                )))
                .with_max_failover_retry_count(0)
                .with_end_to_end_latency_policy(EndToEndOperationLatencyPolicy::new(
                    Duration::from_secs(1),
                ))
                .build();
            let started = std::time::Instant::now();
            let error = fixture
                .container
                .read_item(
                    "A",
                    &expected.id,
                    Some(ItemReadOptions::default().with_operation_options(operation)),
                )
                .await
                .expect_err("the deadline must cancel the active hedge race");
            let elapsed = started.elapsed();

            assert!(elapsed >= Duration::from_millis(900));
            assert!(elapsed < Duration::from_secs(2));
            assert_eq!(
                error.status().sub_status(),
                Some(azure_data_cosmos_driver::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT)
            );
            assert_eq!(observed_primary.hit_count(), 1);
            assert_eq!(observed_alternate.hit_count(), 1);
            let diagnostics = error
                .diagnostics()
                .expect("hedged deadline failure must carry diagnostics");
            assert!(diagnostics.regions_contacted().contains(&Region::EAST_US));
            assert!(diagnostics.regions_contacted().contains(&Region::WEST_US));
            assert!(diagnostics.requests().iter().any(|request| {
                request.region() == Some(&Region::EAST_US)
                    && request.status().status_code()
                        == azure_core::http::StatusCode::ServiceUnavailable
            }));
            let hedge = diagnostics
                .hedge_diagnostics()
                .expect("post-dispatch deadline must carry hedge diagnostics");
            assert_eq!(
                hedge.terminal_state().as_str(),
                "cancelled_awaiting_partner"
            );
            for rule_id in [
                "e2e-hedge-deadline-primary-503",
                "e2e-hedge-deadline-alternate-delay",
            ] {
                assert!(
                    diagnostics.requests().iter().any(|request| {
                        request
                            .fault_injection_evaluations()
                            .iter()
                            .any(|evaluation| {
                                evaluation.rule_id() == rule_id && evaluation.was_applied()
                            })
                    }),
                    "diagnostics must retain applied fault rule '{rule_id}'"
                );
            }
            Ok(())
        })
        .await
}
