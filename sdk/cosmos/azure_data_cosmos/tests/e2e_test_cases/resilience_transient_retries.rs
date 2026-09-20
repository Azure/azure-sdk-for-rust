// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::Arc;

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    },
    options::{AvailabilityStrategy, ItemReadOptions, OperationOptionsBuilder, Region},
    RoutingStrategy, SubStatusCode,
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
async fn service_unavailable_is_retried_with_attempt_history() -> TestResult {
    run_retry_case(
        "resilience.service-retry",
        "e2e-service-retry",
        FaultInjectionErrorType::ServiceUnavailable,
        Some(StatusCode::ServiceUnavailable),
        3,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn response_timeout_is_retried_with_attempt_history() -> TestResult {
    run_retry_case(
        "resilience.transport-retry",
        "e2e-transport-retry",
        FaultInjectionErrorType::ResponseTimeout,
        None,
        2,
    )
    .await
}

async fn run_retry_case(
    scenario_id: &str,
    rule_id: &str,
    fault: FaultInjectionErrorType,
    expected_status: Option<StatusCode>,
    expected_attempts: usize,
) -> TestResult {
    let Some(profile) = selected_scenario_profile(scenario_id).await? else {
        return Ok(());
    };
    let result = FaultInjectionResultBuilder::new().with_error(fault).build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(rule_id, result)
            .with_condition(condition)
            .with_hit_limit(1)
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
            let expected = item(rule_id, "A", 41);
            fixture
                .container
                .create_item("A", &expected.id, &expected, None)
                .await?;
            let operation = OperationOptionsBuilder::new()
                .with_availability_strategy(AvailabilityStrategy::Disabled)
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
            assert_eq!(diagnostics.request_count(), expected_attempts);
            if let Some(expected_status) = expected_status {
                assert_eq!(
                    diagnostics.requests()[0].status().status_code(),
                    expected_status
                );
                assert_eq!(diagnostics.requests()[0].status().sub_status(), None);
                assert_eq!(diagnostics.requests()[0].region(), Some(&Region::EAST_US));
                assert!(diagnostics.requests()[0].request_sent().definitely_sent());

                assert_eq!(
                    diagnostics.requests()[1].status().status_code(),
                    StatusCode::NotFound
                );
                assert_eq!(
                    diagnostics.requests()[1].status().sub_status(),
                    Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE)
                );
                assert_eq!(diagnostics.requests()[1].region(), Some(&Region::WEST_US));
                assert!(diagnostics.requests()[1].request_sent().definitely_sent());

                assert_eq!(
                    diagnostics.requests()[2].status().status_code(),
                    StatusCode::Ok
                );
                assert_eq!(diagnostics.requests()[2].status().sub_status(), None);
                assert_eq!(diagnostics.requests()[2].region(), Some(&Region::EAST_US));
                assert!(diagnostics.requests()[2].request_sent().definitely_sent());
                assert_eq!(
                    diagnostics.regions_contacted(),
                    [Region::EAST_US, Region::WEST_US]
                );
            } else {
                assert_eq!(
                    diagnostics.requests()[0].status().sub_status(),
                    Some(SubStatusCode::TRANSPORT_GENERATED_503)
                );
                assert_eq!(diagnostics.requests()[0].region(), Some(&Region::EAST_US));
                let sent = diagnostics.requests()[0].request_sent();
                assert!(sent.may_have_been_sent());
                assert!(!sent.definitely_sent());
                assert!(!sent.definitely_not_sent());
                assert_eq!(
                    diagnostics.requests()[1].status().status_code(),
                    StatusCode::Ok
                );
                assert_eq!(diagnostics.requests()[1].status().sub_status(), None);
                assert_eq!(diagnostics.requests()[1].region(), Some(&Region::EAST_US));
                assert!(diagnostics.requests()[1].request_sent().definitely_sent());
                assert_eq!(diagnostics.regions_contacted(), [Region::EAST_US]);
            }
            assert_eq!(
                diagnostics
                    .requests()
                    .last()
                    .unwrap()
                    .status()
                    .status_code(),
                StatusCode::Ok
            );
            assert!(diagnostics.requests()[0]
                .fault_injection_evaluations()
                .iter()
                .any(|evaluation| evaluation.rule_id() == rule_id && evaluation.was_applied()));
            Ok(())
        })
        .await
}
