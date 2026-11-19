// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! #5483: a real hedge winner's range-local state must survive replanning in another region.

use super::{
    alternate_region, excluding, hedge_threshold, hedging_operation_options, page_document_ids,
    primary_read_delay, primary_region, request_trail,
};
use crate::framework::{
    resolve_driver_container_ready, resolve_test_env, DriverTestClient, DriverTestRunContext,
    ResourceThroughput,
};
use azure_data_cosmos_driver::{
    diagnostics::{DiagnosticsContext, HedgeTerminalState},
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRule,
        FaultInjectionRuleBuilder, FaultOperationType,
    },
    models::{
        ContainerReference, CosmosOperation, DatabaseReference, FeedRange, MaxItemCountHint,
        PartitionKey,
    },
    options::{OperationOptions, PlanOptions, QueryPlanMode, Region},
    CosmosDriver,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use futures::FutureExt;
use serde_json::{json, Value};
use std::{error::Error, num::NonZeroU32, panic::AssertUnwindSafe, sync::Arc, time::Duration};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const APPROVAL: &str = "AZURE_COSMOS_HEDGE_MULTI_RANGE_RU";
const THROUGHPUT: u64 = 11_000;
const ROWS: usize = 12;

async fn create_fixture(
    context: &DriverTestRunContext,
    database: &DatabaseReference,
    driver: &CosmosDriver,
) -> TestResult<ContainerReference> {
    let name = context.unique_container_name();
    let db = database.name().ok_or("expected name-addressed database")?;
    let resource =
        json!({"id": name, "partitionKey": {"paths": ["/pk"], "kind": "Hash", "version": 2}});
    if let Some(manager) = resolve_test_env()?
        .ok_or("missing live account")?
        .resource_manager
    {
        manager
            .create_or_update_container(
                db,
                &name,
                &resource,
                Some(ResourceThroughput::Manual(THROUGHPUT)),
            )
            .await?;
    } else {
        let create = CosmosOperation::create_container(database.clone());
        let mut headers = create.request_headers().clone();
        headers.offer_throughput = Some(THROUGHPUT);
        driver
            .execute_singleton_operation(
                create
                    .with_request_headers(headers)
                    .with_body(serde_json::to_vec(&resource)?),
                OperationOptions::default(),
            )
            .await?;
    }
    let container = resolve_driver_container_ready(driver, db, &name).await?;
    let ranges = driver
        .resolve_all_partition_key_ranges(&container, true)
        .await?
        .ok_or("missing physical range map")?;
    if ranges.len() != 2 {
        return Err(format!(
            "expected two physical ranges at {THROUGHPUT} RU/s, got {}",
            ranges.len()
        )
        .into());
    }

    // Choose keys by actual ownership before writing anything. Many logical keys
    // in the same range would not exercise the ordered merge or sequential drain.
    let mut keys = [None, None];
    for candidate in 0..128 {
        let key = format!("hedge-pk-{candidate}");
        let owners = driver
            .resolve_partition_key_ranges_for_key(&container, &PartitionKey::from(&key), false)
            .await?
            .ok_or("missing key owner")?;
        let [owner] = owners.as_slice() else {
            return Err("expected one owner per full key".into());
        };
        let index = ranges
            .iter()
            .position(|r| r.id == owner.id)
            .ok_or("range map changed during setup")?;
        keys[index].get_or_insert(key);
        if keys.iter().all(Option::is_some) {
            break;
        }
    }
    let [Some(left), Some(right)] = keys else {
        return Err("could not find keys on both ranges".into());
    };
    for i in 0..ROWS {
        let pk = if i % 2 == 0 { &left } else { &right };
        let id = format!("row-{i:02}");
        let body = serde_json::to_vec(&json!({"id": id, "pk": pk, "rank": i}))?;
        context
            .create_seed_item_with_pk(&container, pk, &body)
            .await?;
        context
            .wait_for_item_in_regions(&container, &id, pk, &[primary_region(), alternate_region()])
            .await?;
    }
    Ok(container)
}

fn require_alternate_win(diag: &DiagnosticsContext, alternate: &Region) -> TestResult {
    let replies = diag.responded_regions();
    if replies.is_empty()
        || replies.iter().any(|r| *r != alternate)
        || !diag.hedge_diagnostics().is_some_and(|h| {
            h.terminal_state() == HedgeTerminalState::AlternateWon
                && h.response_region() == Some(alternate)
        })
    {
        return Err(format!(
            "expected alternate hedge winner for every reply: {}",
            request_trail(diag)
        )
        .into());
    }
    Ok(())
}

async fn check_resume(
    driver: &CosmosDriver,
    container: &ContainerReference,
    rule: &FaultInjectionRule,
    primary: &Region,
    alternate: &Region,
    ordered: bool,
) -> TestResult {
    let sql = if ordered {
        "SELECT c.id FROM c ORDER BY c.rank ASC"
    } else {
        "SELECT c.id FROM c"
    };
    // Each leaf can fetch two items. In sort order the first two emitted rows
    // come from different ranges, leaving one unconsumed row in each leaf.
    let query = CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
        .with_body(serde_json::to_vec(
            &json!({"query": sql, "parameters": []}),
        )?)
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(2).unwrap()));
    let planning = PlanOptions::default()
        .with_query_plan_mode(QueryPlanMode::GatewayOnly)
        .with_max_fan_out(2);
    let hedged = OperationOptions::default();
    let primary_only = excluding(std::slice::from_ref(alternate));
    rule.enable();
    let mut plan = driver
        .plan_operation(query.clone(), &hedged, None, &planning)
        .await?;
    let before = rule.hit_count();
    let first = driver
        .execute_plan(&mut plan, Some(container.clone()), hedged.clone())
        .await?
        .ok_or("missing first query page")?;
    if rule.hit_count() == before {
        return Err(format!("{sql}: primary query delay never fired").into());
    }
    require_alternate_win(&first.diagnostics(), alternate)?;
    let mut seen = page_document_ids(first);
    let token = plan.to_continuation_token()?;
    drop(plan);

    // Inspect the checkpoint, not just the output: a range boundary or a
    // fresh-start token could give correct rows without testing portability.
    let state: Value = serde_json::from_slice(
        &URL_SAFE_NO_PAD.decode(
            token
                .as_str()
                .strip_prefix("c2.")
                .ok_or("expected a driver v2 checkpoint")?,
        )?,
    )?;
    let root = &state["root"];
    if ordered {
        let ranges = root["ranges"]
            .as_array()
            .ok_or("missing ORDER BY range checkpoints")?;
        if seen != ["row-00", "row-01"]
            || ranges.len() != 2
            || !ranges
                .iter()
                .all(|r| r["boundary"].is_object() && r["server_continuation"].is_null())
        {
            return Err(
                "ORDER BY must resume both partially consumed pages via resumeFilter".into(),
            );
        }

        // Replay once with the race still enabled: the alternate must execute
        // the rewritten query *with* the resumeFilter, not just the fresh query.
        // Discard this independent replay; the verified drain below starts
        // from the original alternate-minted token and runs only on primary.
        let mut replay = driver
            .plan_operation(query.clone(), &hedged, Some(&token), &planning)
            .await?;
        let next = driver
            .execute_plan(&mut replay, Some(container.clone()), hedged)
            .await?
            .ok_or("missing hedged resumeFilter page")?;
        require_alternate_win(&next.diagnostics(), alternate)?;
        if page_document_ids(next) != ["row-02", "row-03"] {
            return Err("hedged resumeFilter did not return the next ordered pair".into());
        }
    } else if seen.len() != 2
        || root["active_tokens"][0]["server_continuation"]
            .as_str()
            .is_none_or(str::is_empty)
    {
        return Err("sequential drain did not capture a live backend continuation".into());
    }

    rule.disable();
    let mut resumed = driver
        .plan_operation(query, &primary_only, Some(&token), &planning)
        .await?;
    let mut primary_replied = false;
    let mut pages = 0;
    while let Some(page) = driver
        .execute_plan(&mut resumed, Some(container.clone()), primary_only.clone())
        .await?
    {
        let diag = page.diagnostics();
        let replies = diag.responded_regions();
        // A merged page served entirely from buffered rows can have no replies.
        if replies.iter().any(|r| *r != primary) {
            return Err(format!(
                "{sql}: non-primary reply on resume: {}",
                request_trail(&diag)
            )
            .into());
        }
        primary_replied |= replies.contains(&primary);
        seen.extend(page_document_ids(page));
        pages += 1;
        if pages > ROWS * 2 {
            return Err("resumed query did not terminate".into());
        }
    }
    if !ordered {
        seen.sort();
    }
    let expected: Vec<_> = (0..ROWS).map(|i| format!("row-{i:02}")).collect();
    if !primary_replied || seen != expected {
        return Err(format!(
            "{sql}: primary_replied={primary_replied}, expected {expected:?}, got {seen:?}"
        )
        .into());
    }
    Ok(())
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
async fn hedged_multi_range_continuations_resume_in_primary() -> TestResult {
    match std::env::var(APPROVAL) {
        Ok(value) if value == THROUGHPUT.to_string() => (),
        Err(std::env::VarError::NotPresent) => {
            eprintln!("Skipping: set {APPROVAL}={THROUGHPUT} to approve one temporary container");
            return Ok(());
        }
        _ => return Err(format!("{APPROVAL} must be exactly {THROUGHPUT}").into()),
    }
    let primary = primary_region();
    let alternate = alternate_region();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "slow-primary-multi-range-query",
            FaultInjectionResultBuilder::new()
                .with_delay(primary_read_delay())
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::QueryItem)
                .with_region(primary.clone())
                .build(),
        )
        .build(),
    );
    DriverTestClient::run_with_unique_db_and_hedging(
        vec![Arc::clone(&rule)],
        hedging_operation_options(hedge_threshold()),
        vec![primary.clone(), alternate.clone()],
        async |context, database| {
            // Return errors (including helper assertion failures) to the harness
            // so its best-effort database deletion runs outside the time limit.
            let scenario = async {
                let driver = context
                    .runtime()
                    .create_driver(context.driver_options()?)
                    .await?;
                let container = create_fixture(&context, &database, &driver).await?;
                for ordered in [false, true] {
                    check_resume(&driver, &container, &rule, &primary, &alternate, ordered).await?;
                }
                Ok(())
            };
            tokio::time::timeout(
                Duration::from_secs(600),
                AssertUnwindSafe(scenario).catch_unwind(),
            )
            .await?
            .map_err(|_| "multi-range scenario panicked; see assertion output")?
        },
    )
    .await
}
