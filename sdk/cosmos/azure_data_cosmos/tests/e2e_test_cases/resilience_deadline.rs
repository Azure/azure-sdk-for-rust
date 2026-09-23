// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{sync::Arc, time::Duration};

use azure_data_cosmos::{
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRuleBuilder,
        FaultOperationType,
    },
    options::{
        AvailabilityStrategy, EndToEndOperationLatencyPolicy, ItemReadOptions,
        OperationOptionsBuilder, Region,
    },
    RoutingStrategy, SubStatusCode,
};

use crate::e2e_test_cases::{
    fixture::{build_client_with_customizer, ClientSetup, E2eTest, TestResult},
    support::{item, selected_scenario_profile},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn operation_deadline_preempts_delayed_response() -> TestResult {
    let Some(profile) = selected_scenario_profile("resilience.deadline").await? else {
        return Ok(());
    };
    let delayed = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_millis(1500))
        .with_probability(1.0)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("e2e-deadline-delay", delayed)
            .with_condition(condition)
            .build(),
    );
    let observed_rule = Arc::clone(&rule);
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
            let expected = item("deadline", "A", 2);
            fixture
                .container
                .create_item("A", &expected.id, &expected, None)
                .await?;

            let operation = OperationOptionsBuilder::new()
                .with_availability_strategy(AvailabilityStrategy::Disabled)
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
                .expect_err("the deadline must preempt the delayed response");
            let elapsed = started.elapsed();
            assert!(elapsed >= Duration::from_millis(900));
            assert!(elapsed < Duration::from_secs(2));
            assert_eq!(
                error.status().sub_status(),
                Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT)
            );
            let diagnostics = error
                .diagnostics()
                .expect("deadline failure must carry diagnostics");
            assert!(diagnostics.request_count() >= 1);
            assert!(diagnostics.fault_injection_enabled());
            assert_eq!(observed_rule.hit_count(), 1);
            assert!(diagnostics.regions_contacted().contains(&Region::EAST_US));
            assert!(diagnostics.requests().iter().any(|request| {
                request
                    .fault_injection_evaluations()
                    .iter()
                    .any(|evaluation| {
                        evaluation.rule_id() == "e2e-deadline-delay" && evaluation.was_applied()
                    })
            }));
            Ok(())
        })
        .await
}
