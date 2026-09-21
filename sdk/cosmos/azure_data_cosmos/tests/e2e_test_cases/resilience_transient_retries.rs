// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::Arc;

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    },
    feed::FeedScope,
    options::{
        AvailabilityStrategy, ItemReadOptions, OperationOptionsBuilder, QueryOptions, Region,
    },
    Query, RoutingStrategy, SubStatusCode,
};
use futures::StreamExt;

use crate::e2e_test_cases::{
    fixture::{build_client_with_customizer, ClientSetup, E2eTest, TestResult},
    support::{
        item, selected_scenario_profile, wait_for_item_replication, with_replication_paused_if,
        Item,
    },
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
        true,
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
        false,
        None,
        2,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn request_timeout_retries_respect_failover_budget() -> TestResult {
    run_request_timeout_case("e2e-request-timeout-retry", 1, true).await?;
    run_request_timeout_case("e2e-request-timeout-terminal", 0, false).await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn partition_topology_change_refreshes_and_retries_query() -> TestResult {
    let Some(profile) = selected_scenario_profile("resilience.partition-topology-retry").await?
    else {
        return Ok(());
    };
    let rule_id = "e2e-partition-topology-retry";
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::PartitionIsGone)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::QueryItem)
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
        Ok(builder.with_fault_injection_rules(vec![Arc::clone(&rule)])?)
    })
    .await?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let expected = item(rule_id, "A", 43);
            fixture
                .container
                .create_item("A", &expected.id, &expected, None)
                .await?;

            let operation = OperationOptionsBuilder::new()
                .with_availability_strategy(AvailabilityStrategy::Disabled)
                .build();
            let mut pages = fixture
                .container
                .query_items::<Item>(
                    Query::from("SELECT * FROM c"),
                    FeedScope::partition("A"),
                    Some(QueryOptions::default().with_operation_options(operation)),
                )
                .await?
                .into_pages();
            let page = pages
                .next()
                .await
                .expect("topology-retried query must yield a page")?;

            assert_eq!(page.items(), std::slice::from_ref(&expected));
            assert_eq!(rule.hit_count(), 1);
            let diagnostics = page.diagnostics();
            assert_eq!(diagnostics.request_count(), 2);
            assert_eq!(
                diagnostics.requests()[0].status().status_code(),
                StatusCode::Gone
            );
            assert_eq!(
                diagnostics.requests()[0].status().sub_status(),
                Some(SubStatusCode::PARTITION_KEY_RANGE_GONE)
            );
            assert_eq!(
                diagnostics.requests()[1].status().status_code(),
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

async fn run_request_timeout_case(
    rule_id: &str,
    max_failover_retry_count: u32,
    expect_success: bool,
) -> TestResult {
    let Some(profile) = selected_scenario_profile("resilience.request-timeout-retry").await? else {
        return Ok(());
    };
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::Timeout)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(rule_id, result)
            .with_condition(condition)
            .with_hit_limit(1)
            .build(),
    );
    rule.disable();
    let setup = ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![Region::EAST_US, Region::WEST_US]),
    )?;
    let client = build_client_with_customizer(setup, |builder| {
        Ok(builder.with_fault_injection_rules(vec![Arc::clone(&rule)])?)
    })
    .await?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let expected = item(rule_id, "A", 42);
            fixture
                .container
                .create_item("A", &expected.id, &expected, None)
                .await?;
            wait_for_item_replication(&fixture.container, &expected.id, &expected).await?;
            rule.enable();

            let operation = OperationOptionsBuilder::new()
                .with_availability_strategy(AvailabilityStrategy::Disabled)
                .with_max_failover_retry_count(max_failover_retry_count)
                .build();
            let result = fixture
                .container
                .read_item(
                    "A",
                    &expected.id,
                    Some(ItemReadOptions::default().with_operation_options(operation)),
                )
                .await;

            if expect_success {
                let response = result?;
                let diagnostics = response.diagnostics();
                assert_eq!(response.into_model::<Item>()?, expected);
                assert_eq!(diagnostics.request_count(), 2);
                assert_eq!(
                    diagnostics
                        .requests()
                        .iter()
                        .map(|request| (request.status().status_code(), request.region()))
                        .collect::<Vec<_>>(),
                    [
                        (StatusCode::RequestTimeout, Some(&Region::EAST_US)),
                        (StatusCode::Ok, Some(&Region::WEST_US)),
                    ]
                );
                assert!(diagnostics.requests()[0]
                    .fault_injection_evaluations()
                    .iter()
                    .any(|evaluation| {
                        evaluation.rule_id() == rule_id && evaluation.was_applied()
                    }));
            } else {
                let error = result.expect_err("zero failover budget must return the injected 408");
                assert_eq!(error.status().status_code(), StatusCode::RequestTimeout);
                let diagnostics = error
                    .diagnostics()
                    .expect("terminal request timeout must retain diagnostics");
                assert_eq!(diagnostics.request_count(), 1);
                assert!(diagnostics.requests()[0]
                    .fault_injection_evaluations()
                    .iter()
                    .any(|evaluation| {
                        evaluation.rule_id() == rule_id && evaluation.was_applied()
                    }));
            }
            Ok(())
        })
        .await
}

async fn run_retry_case(
    scenario_id: &str,
    rule_id: &str,
    fault: FaultInjectionErrorType,
    needs_stale_replica: bool,
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
            with_replication_paused_if(needs_stale_replica, "West US", async {
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
        })
        .await
}
