// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live threshold-storm validation for diagnostics materialization cost + compaction.
//!
//! Deliverable 2 of the "storm-safe diagnostics" work item. Uses the
//! `fault_injection` feature to inject a latency + error (429/503/410) storm
//! against a LIVE multi-region account, then measures:
//!   * the fraction of operations for which the `Threshold` gate fires
//!     (`capture_diagnostics()` is `Some`) under a storm vs. a baseline batch,
//!   * the incremental cost of materializing detailed diagnostics JSON per
//!     operation, and
//!   * that compaction keeps the retained per-attempt count bounded below the
//!     true attempt count.
//!
//! **Env + feature gated.** Compiled only with `--features reqwest` (file-level
//! `cfg`) and `required-features = ["fault_injection"]` (Cargo), so CI/playback
//! never build or run it without both features. Reads `COSMOSDB_MULTI_REGION`
//! (a Cosmos connection string; master key) and **skips gracefully** — passing
//! without asserting — when the var is absent, does not parse, or the account
//! is unreachable. Secret values are never printed.
//!
//! The reproducible CPU/size numbers live in the in-crate deterministic
//! measurement
//! `diagnostics::capture::model::tests::storm_materialization_cost_and_size`
//! (run with `--ignored --nocapture`); this live test corroborates them against
//! real network latency and topology.

#![cfg(feature = "reqwest")]

use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_data_cosmos_driver::diagnostics::capture::DiagnosticsPolicy;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder,
};
use azure_data_cosmos_driver::models::{
    AccountReference, ConnectionString, CosmosOperation, DatabaseReference,
};
use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions, Region};
use azure_data_cosmos_driver::{
    CosmosDriver, CosmosDriverRuntime, CosmosError, CosmosResponse, DiagnosticsContext,
    DiagnosticsVerbosity,
};
use url::Url;

/// Maximum time to wait for a live call before treating the account as unreachable.
const LIVE_TIMEOUT: Duration = Duration::from_secs(20);
/// Number of probe operations per batch.
const BATCH: usize = 12;
/// The gate trips when an operation exceeds this latency; the injected latency is above it.
const GATE_THRESHOLD: Duration = Duration::from_millis(5);
/// Per-request injected latency, comfortably above `GATE_THRESHOLD`.
const STORM_LATENCY: Duration = Duration::from_millis(50);
/// Small retained cap so a storm's bounded per-attempt count is observable.
const STORM_CAP: &str = "16";

type OpResult = std::result::Result<CosmosResponse, CosmosError>;

fn account_from_env() -> Option<AccountReference> {
    // Q4: COSMOSDB_MULTI_REGION is a Cosmos connection string (AccountEndpoint=...;AccountKey=...;).
    let conn_str = std::env::var("COSMOSDB_MULTI_REGION").ok()?;
    let conn: ConnectionString = conn_str.parse().ok()?;
    let endpoint = Url::parse(conn.account_endpoint()).ok()?;
    Some(AccountReference::with_master_key(
        endpoint,
        conn.account_key().clone(),
    ))
}

/// A `DiagnosticsContext` is available on both the success and error paths.
fn diagnostics_of(result: &OpResult) -> Option<Arc<DiagnosticsContext>> {
    match result {
        Ok(resp) => Some(resp.diagnostics()),
        Err(err) => err.diagnostics(),
    }
}

fn gate_fired(result: &OpResult) -> bool {
    match result {
        Ok(resp) => resp.capture_diagnostics().is_some(),
        Err(err) => err.capture_diagnostics().is_some(),
    }
}

fn latency_rule(id: &str, delay: Duration) -> Arc<FaultInjectionRule> {
    let result = FaultInjectionResultBuilder::new()
        .with_delay(delay)
        .with_probability(1.0)
        .build();
    Arc::new(FaultInjectionRuleBuilder::new(id, result).build())
}

fn error_rule(
    id: &str,
    err: FaultInjectionErrorType,
    region: Region,
    hit_limit: u32,
) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_region(region)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(err)
        .with_probability(1.0)
        .build();
    Arc::new(
        FaultInjectionRuleBuilder::new(id, result)
            .with_condition(condition)
            .with_hit_limit(hit_limit)
            .build(),
    )
}

#[derive(Default)]
struct BatchStats {
    ops: usize,
    reached: usize,
    gate_fired: usize,
    max_request_count: usize,
    max_retained: usize,
    compacted_ops: usize,
    total_json_materialization: Duration,
    json_samples: usize,
}

impl BatchStats {
    fn avg_json_materialization(&self) -> Duration {
        if self.json_samples == 0 {
            Duration::ZERO
        } else {
            self.total_json_materialization / self.json_samples as u32
        }
    }
}

async fn run_batch(driver: &CosmosDriver, account: &AccountReference, label: &str) -> BatchStats {
    let mut stats = BatchStats::default();
    for i in 0..BATCH {
        // Probe a database that almost certainly does not exist: it still exercises the full
        // pipeline (and any injected faults/retries) and yields a real diagnostics context.
        let db = DatabaseReference::from_name(account.clone(), "diag-storm-probe-nonexistent-db");
        let operation = CosmosOperation::read_database(db);
        let outcome = tokio::time::timeout(
            LIVE_TIMEOUT,
            driver.execute_singleton_operation(operation, OperationOptions::default()),
        )
        .await;
        stats.ops += 1;
        let result = match outcome {
            Err(_elapsed) => {
                eprintln!("[{label}] op {i} timed out (account unreachable / firewall-blocked)");
                continue;
            }
            Ok(result) => result,
        };
        stats.reached += 1;
        if gate_fired(&result) {
            stats.gate_fired += 1;
        }
        if let Some(diag) = diagnostics_of(&result) {
            stats.max_request_count = stats.max_request_count.max(diag.request_count());
            stats.max_retained = stats.max_retained.max(diag.retained_request_count());
            if diag.compaction().is_some() {
                stats.compacted_ops += 1;
            }
            // Measure the incremental cost of materializing the detailed JSON (first call computes
            // and caches it). This is the steady-state overhead paid when the gate fires broadly.
            let started = Instant::now();
            let _ = diag.to_json_string(Some(DiagnosticsVerbosity::Detailed));
            stats.total_json_materialization += started.elapsed();
            stats.json_samples += 1;
        }
    }
    stats
}

#[tokio::test]
async fn live_storm_diagnostics_or_env_gated() {
    let Some(account) = account_from_env() else {
        eprintln!("live_storm env-gated: COSMOSDB_MULTI_REGION not set or unparseable; skipping");
        return;
    };

    // Best-effort: a small retained cap makes a storm's bounded output observable. Picked up when
    // the driver builds its DiagnosticsOptions from the environment.
    std::env::set_var("AZURE_COSMOS_DIAGNOSTICS_MAX_REQUESTS", STORM_CAP);

    let runtime = match CosmosDriverRuntime::builder().build().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "live_storm env-gated: could not build runtime: {}",
                e.status()
            );
            return;
        }
    };

    // Baseline batch: no faults, Threshold gate.
    let baseline_options = DriverOptions::builder(account.clone())
        .with_capture_diagnostics_policy(DiagnosticsPolicy::threshold(GATE_THRESHOLD))
        .build();
    let baseline_driver = match runtime.create_driver(baseline_options).await {
        Ok(d) => d,
        Err(e) => {
            eprintln!(
                "live_storm env-gated: could not create baseline driver: {}",
                e.status()
            );
            return;
        }
    };
    let baseline = run_batch(&baseline_driver, &account, "baseline").await;
    if baseline.reached == 0 {
        eprintln!("live_storm env-gated: account unreachable; skipping without assertions");
        return;
    }

    // Storm batch: per-request latency (trips the gate for a large fraction) plus 429/503/410
    // faults scoped to the write region (to induce retries/failover).
    let write_region = Region::new("West US 2");
    let rules = vec![
        latency_rule("storm-latency", STORM_LATENCY),
        error_rule(
            "storm-429",
            FaultInjectionErrorType::TooManyRequests,
            write_region.clone(),
            32,
        ),
        error_rule(
            "storm-503",
            FaultInjectionErrorType::ServiceUnavailable,
            write_region.clone(),
            32,
        ),
        error_rule(
            "storm-410",
            FaultInjectionErrorType::PartitionIsGone,
            write_region,
            32,
        ),
    ];
    let storm_builder = match DriverOptions::builder(account.clone())
        .with_capture_diagnostics_policy(DiagnosticsPolicy::threshold(GATE_THRESHOLD))
        .with_fault_injection_rules(rules)
    {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "live_storm env-gated: fault rule install failed: {}",
                e.status()
            );
            return;
        }
    };
    let storm_driver = match runtime.create_driver(storm_builder.build()).await {
        Ok(d) => d,
        Err(e) => {
            eprintln!(
                "live_storm env-gated: could not create storm driver: {}",
                e.status()
            );
            return;
        }
    };
    let storm = run_batch(&storm_driver, &account, "storm").await;

    eprintln!("=== live storm diagnostics ===");
    eprintln!(
        "baseline: reached={}/{} gate_fired={} max_requests={} avg_json_materialization={:?}",
        baseline.reached,
        baseline.ops,
        baseline.gate_fired,
        baseline.max_request_count,
        baseline.avg_json_materialization()
    );
    eprintln!(
        "storm   : reached={}/{} gate_fired={} max_requests={} max_retained={} compacted_ops={} avg_json_materialization={:?}",
        storm.reached,
        storm.ops,
        storm.gate_fired,
        storm.max_request_count,
        storm.max_retained,
        storm.compacted_ops,
        storm.avg_json_materialization()
    );

    // Soft invariants (only assert when the storm batch actually reached the service, so the test
    // stays green when the account is unreachable).
    if storm.reached > 0 {
        assert!(
            storm.max_request_count >= 1,
            "reached-service operations must carry diagnostics with at least one attempt"
        );
        // Whenever compaction fired, the retained per-attempt count is strictly below the true
        // total — the bounded-size guarantee holding under a live storm.
        if storm.compacted_ops > 0 {
            assert!(
                storm.max_retained < storm.max_request_count,
                "compaction must retain fewer records ({}) than the true attempt count ({})",
                storm.max_retained,
                storm.max_request_count
            );
        }
    }
}
