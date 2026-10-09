// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg(all(feature = "fault_injection", feature = "__internal_in_memory_emulator"))]

use azure_data_cosmos_driver::{
    driver::CosmosDriver,
    error::status_codes,
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
    },
    models::{ChangeFeedStartFrom, ContainerReference, CosmosOperation, FeedRange},
    options::{OperationOptions, PlanOptions},
};
use std::{error::Error, sync::Arc};

fn require(condition: bool, message: &'static str) -> Result<(), Box<dyn Error>> {
    if !condition {
        return Err(message.into());
    }
    Ok(())
}

fn split_rule(limit: Option<u32>) -> Arc<FaultInjectionRule> {
    let mut builder = FaultInjectionRuleBuilder::new(
        "diagnostics-split-repro",
        FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::PartitionIsGone)
            .build(),
    )
    .with_condition(
        FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::ChangeFeedItem)
            .build(),
    );
    if let Some(limit) = limit {
        builder = builder.with_hit_limit(limit);
    }
    Arc::new(builder.build())
}

async fn run_repro(
    driver: &CosmosDriver,
    container: ContainerReference,
    rule: &FaultInjectionRule,
    exhaustion: bool,
) -> Result<(), Box<dyn Error>> {
    let operation = CosmosOperation::change_feed(container.clone(), Some(FeedRange::full()))
        .with_change_feed_start(ChangeFeedStartFrom::Beginning);
    let mut plan = Box::pin(
        driver
            .plan_operation(
                operation,
                &OperationOptions::default(),
                None,
                &PlanOptions::default(),
            )
            .await?,
    );
    let result = driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await;
    if exhaustion {
        let error = match result {
            Err(error) => error,
            Ok(_) => return Err("persistent topology faults did not exhaust split retries".into()),
        };
        require(
            error.status() == status_codes::CLIENT_SPLIT_RETRIES_EXHAUSTED,
            "unexpected terminal status",
        )?;
        let diagnostics = error
            .diagnostics()
            .ok_or("exhaustion must retain diagnostics")?;
        require(
            diagnostics.effective_status() == Some(error.status()),
            "terminal diagnostic status mismatch",
        )?;
        let failed = diagnostics
            .requests()
            .iter()
            .filter(|request| *request.status() == status_codes::PARTITION_KEY_RANGE_GONE)
            .count();
        require(
            failed as u32 == rule.hit_count(),
            "injected attempts missing or duplicated",
        )?;
        require(
            rule.hit_count() == 11,
            "did not exercise the expected split retry budget",
        )?;
        println!("EXHAUSTION {}", diagnostics.to_json_string(None));
        let json: serde_json::Value = serde_json::from_str(diagnostics.to_json_string(None))?;
        require(
            json["topology_recovery"]["total_attempts"] == 11,
            "missing topology transitions",
        )?;
    } else {
        let response = result?.ok_or("change feed must produce a page")?;
        let diagnostics = response.diagnostics();
        require(rule.hit_count() == 1, "finite rule must fire once")?;
        require(
            diagnostics
                .requests()
                .iter()
                .filter(|request| *request.status() == status_codes::PARTITION_KEY_RANGE_GONE)
                .count()
                == 1,
            "recovery must retain exactly one injected attempt",
        )?;
        println!("RECOVERY {}", diagnostics.to_json_string(None));
    }
    rule.disable();
    let response = driver
        .execute_plan(&mut plan, Some(container), OperationOptions::default())
        .await?
        .ok_or("change feed must continue after faults stop")?;
    require(
        response
            .diagnostics()
            .requests()
            .iter()
            .all(|request| *request.status() != status_codes::PARTITION_KEY_RANGE_GONE),
        "later page repeated old failures",
    )?;
    require(
        !response
            .diagnostics()
            .to_json_string(None)
            .contains("topology_recovery"),
        "later page repeated old topology history",
    )?;
    Ok(())
}

#[tokio::test]
#[cfg(feature = "__internal_in_memory_emulator")]
async fn in_memory_split_diagnostics_repro() -> Result<(), Box<dyn Error>> {
    use azure_core::http::Url;
    use azure_data_cosmos_driver::{
        in_memory_emulator::{InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion},
        models::AccountReference,
        options::DriverOptions,
    };
    for exhaustion in [true, false] {
        let rule = split_rule(if exhaustion { None } else { Some(1) });
        let endpoint = Url::parse("https://eastus.emulator.local")?;
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(
            vec![VirtualRegion::new("East US", endpoint.clone())],
        )?));
        emulator.store().create_database("diagnostics-repro");
        emulator.store().create_container(
            "diagnostics-repro",
            "items",
            serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))?,
        );
        let runtime = emulator
            .runtime_builder_with_fault_rules(vec![rule.clone()])
            .build()
            .await?;
        let driver = runtime
            .create_driver(
                DriverOptions::builder(AccountReference::with_account_key(
                    endpoint,
                    "ZW11bGF0b3Ita2V5",
                ))
                .build(),
            )
            .await?;
        let container = driver
            .resolve_container_by_name("diagnostics-repro", "items", OperationOptions::default())
            .await?;
        run_repro(&driver, container, &rule, exhaustion).await?;
    }
    Ok(())
}
