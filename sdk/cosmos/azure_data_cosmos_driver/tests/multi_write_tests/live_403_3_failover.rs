// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live-account regression test for `403/3 (WriteForbidden)` failover on a
//! multi-write account: the SDK must retry the write against the next
//! preferred region rather than bubble the substatus up to the caller.
//!
//! Requires a live multi-write account (East US 2 + West US 3); gated by
//! `test_category = "multi_write"`.

#![cfg(feature = "fault_injection")]

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::options::{OperationOptions, Region};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

/// Pins live 403/3 handling: existing failover to West US 3 must keep working.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn live_403_3_create_item_triggers_failover() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(Region::EAST_US_2)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("live-403-3-create-east2", result)
            .with_condition(condition)
            .with_hit_limit(1)
            .build(),
    );
    // Start disabled so the warmup phase can prime both regions without
    // tripping the fault.
    rule.disable();
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_hedging(
        rules,
        OperationOptions::default(),
        vec![Region::EAST_US_2, Region::WEST_US_3],
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // Warm both regions to avoid a failover racing control-plane replication.
            let warmup_json = br#"{"id": "warmup", "pk": "pk1", "value": "warmup"}"#;
            context
                .create_item(&container, "warmup", "pk1", warmup_json)
                .await?;
            context.read_item(&container, "warmup", "pk1").await?;
            tokio::time::sleep(Duration::from_secs(5)).await;

            rule.enable();

            let item_json = br#"{"id": "live-403-3-item", "pk": "pk1", "value": "test"}"#;
            let response = context
                .create_item(&container, "live-403-3-item", "pk1", item_json)
                .await
                .expect(
                    "regression guard: with both regions warmed up, 403/3 on East US 2 must \
                     fail over to West US 3 and succeed",
                );

            assert!(
                rule.hit_count() >= 1,
                "fault rule must have fired at least once against East US 2",
            );

            let diagnostics = response.diagnostics();
            let requests = diagnostics.requests();
            assert!(
                requests.len() >= 2,
                "SDK must retry CreateItem after 403/3; observed request_count={}, \
                 requests={:?}",
                requests.len(),
                requests,
            );

            // The failover attempt must have actually targeted a different
            // region than the faulted attempt.
            let first_region = requests[0].region().cloned();
            let last_region = requests.last().and_then(|r| r.region()).cloned();
            assert_ne!(
                first_region, last_region,
                "post-403/3 retry must target a different region; first={:?}, last={:?}",
                first_region, last_region,
            );

            Ok(())
        },
    )
    .await
}
