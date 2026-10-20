// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{sync::Arc, time::Duration};

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    },
    options::{
        AvailabilityStrategy, ItemReadOptions, OperationOptionsBuilder, Region,
        ThrottlingRetryOptionsBuilder,
    },
    RoutingStrategy,
};

use crate::e2e_test_cases::{
    fixture::{build_client_with_customizer, ClientSetup, E2eTest, TestResult},
    support::{item, selected_scenario_profile, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn bounded_throttling_retries_succeed_with_attempt_history() -> TestResult {
    let Some(profile) = selected_scenario_profile("resilience.throttling-retry").await? else {
        return Ok(());
    };
    let fault = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::TooManyRequests)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("e2e-throttle-twice", fault)
            .with_condition(condition)
            .with_hit_limit(2)
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
            let expected = item("retry-429", "A", 1);
            fixture
                .container
                .create_item("A", &expected.id, &expected, None)
                .await?;

            let operation = OperationOptionsBuilder::new()
                .with_availability_strategy(AvailabilityStrategy::Disabled)
                .with_throttling_retry_options(
                    ThrottlingRetryOptionsBuilder::new()
                        .with_max_retry_count(2)
                        .with_max_retry_wait_time(Duration::from_secs(10))
                        .build(),
                )
                .build();
            let response = fixture
                .container
                .read_item(
                    "A",
                    &expected.id,
                    Some(ItemReadOptions::default().with_operation_options(operation)),
                )
                .await?;
            assert_eq!(response.status().status_code(), StatusCode::Ok);
            let diagnostics = response.diagnostics();
            assert_eq!(response.into_model::<Item>()?, expected);
            assert_eq!(diagnostics.request_count(), 3);
            assert_eq!(
                diagnostics
                    .requests()
                    .iter()
                    .map(|request| request.status().status_code())
                    .collect::<Vec<_>>(),
                [
                    StatusCode::TooManyRequests,
                    StatusCode::TooManyRequests,
                    StatusCode::Ok
                ]
            );
            assert!(diagnostics.fault_injection_enabled());
            for request in &diagnostics.requests()[..2] {
                assert!(request
                    .fault_injection_evaluations()
                    .iter()
                    .any(|evaluation| {
                        evaluation.rule_id() == "e2e-throttle-twice" && evaluation.was_applied()
                    }));
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
async fn operation_throttling_retry_limit_is_enforced() -> TestResult {
    let Some(profile) = selected_scenario_profile("resilience.throttling-retry").await? else {
        return Ok(());
    };
    let fault = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::TooManyRequests)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("e2e-throttle-exhausted", fault)
            .with_condition(condition)
            .with_hit_limit(2)
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
            let expected = item("retry-429-exhausted", "A", 2);
            fixture
                .container
                .create_item("A", &expected.id, &expected, None)
                .await?;

            let operation = OperationOptionsBuilder::new()
                .with_availability_strategy(AvailabilityStrategy::Disabled)
                .with_throttling_retry_options(
                    ThrottlingRetryOptionsBuilder::new()
                        .with_max_retry_count(1)
                        .with_max_retry_wait_time(Duration::from_secs(10))
                        .build(),
                )
                .build();
            let error = fixture
                .container
                .read_item(
                    "A",
                    &expected.id,
                    Some(ItemReadOptions::default().with_operation_options(operation)),
                )
                .await
                .expect_err("one retry must be exhausted by two throttling responses");
            assert_eq!(error.status().status_code(), StatusCode::TooManyRequests);
            let diagnostics = error
                .diagnostics()
                .expect("terminal throttling error must retain diagnostics");
            assert_eq!(diagnostics.request_count(), 2);
            assert!(diagnostics.requests().iter().all(|request| {
                request.status().status_code() == StatusCode::TooManyRequests
                    && request
                        .fault_injection_evaluations()
                        .iter()
                        .any(|evaluation| {
                            evaluation.rule_id() == "e2e-throttle-exhausted"
                                && evaluation.was_applied()
                        })
            }));
            Ok(())
        })
        .await
}
