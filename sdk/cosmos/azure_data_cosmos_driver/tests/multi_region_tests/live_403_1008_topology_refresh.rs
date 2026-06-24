// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live-account regression test for `403/1008 (DatabaseAccountNotFound)`
//! handling: the SDK must refresh account topology and retry to another
//! region rather than bubble the substatus up to the caller.
//!
//! Requires a live multi-region account (East US 2 + West US 3); gated by
//! `test_category = "multi_region"`.

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

/// Pins live 403/1008 handling: refresh topology and retry to another region.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn live_403_1008_create_item_triggers_refresh_and_retry() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(Region::EAST_US_2)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::DatabaseAccountNotFound)
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("live-403-1008-create-east2", result)
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

            let item_json = br#"{"id": "live-1008-item", "pk": "pk1", "value": "test"}"#;
            let result = context
                .create_item(&container, "live-1008-item", "pk1", item_json)
                .await;

            assert!(
                rule.hit_count() >= 1,
                "fault rule must have fired at least once against East US 2",
            );

            match result {
                // Post-fix path: refresh + retry succeeded.
                Ok(response) => {
                    let diagnostics = response.diagnostics();
                    let requests = diagnostics.requests();
                    assert!(
                        requests.len() >= 2,
                        "post-fix: SDK must retry after 403/1008; observed request_count={}, \
                         requests={:?}",
                        requests.len(),
                        requests,
                    );
                }
                // Pre-fix path: 1008 bubbled up.
                Err(err) => {
                    panic!(
                        "403/1008 reproduction confirmed against a real account: 403/1008 \
                         bubbled up to the caller (no refresh + retry). Error: {err}"
                    );
                }
            }

            Ok(())
        },
    )
    .await
}
