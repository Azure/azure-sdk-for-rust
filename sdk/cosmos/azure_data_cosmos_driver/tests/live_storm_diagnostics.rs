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
    /// Distinct regions contacted across the batch, in first-seen order. Used to
    /// derive the account's actual region(s) rather than hardcoding one, so the
    /// fault rules target a region the driver really uses.
    regions_seen: Vec<Region>,
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
            for region in diag.regions_contacted() {
                if !stats.regions_seen.contains(&region) {
                    stats.regions_seen.push(region);
                }
            }
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

    // Best-effort: a small retained cap makes a storm's bounded output observable. The driver
    // resolves this cap per operation from the environment (`DiagnosticsOptions::default()` reads
    // `AZURE_COSMOS_DIAGNOSTICS_MAX_REQUESTS`), and there is no per-driver builder override for it
    // today, so the environment is the only channel. `set_var` is process-global (and unsound under
    // concurrent env access), but this is safe here: the `live_storm` binary contains exactly one
    // `#[tokio::test]`, so nothing else in the process reads or writes the environment concurrently
    // — the write is effectively serialized. (If a per-driver `max_request_diagnostics` builder
    // knob is added later, prefer it over this env write.)
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

    // Derive the target region from what the baseline probe actually contacted, rather than
    // hardcoding one: a hardcoded region that doesn't match this account's write region would make
    // the error rules never fire, so no storm is induced and the test passes green without ever
    // exercising the compaction it exists to validate. If the baseline reached the service but
    // reported no region, skip gracefully (the rules would target nothing).
    let Some(target_region) = baseline.regions_seen.first().cloned() else {
        eprintln!(
            "live_storm env-gated: baseline reached the service but reported no region; skipping storm"
        );
        return;
    };
    eprintln!(
        "live_storm: injecting faults against region {:?} (derived from baseline probe)",
        target_region.as_str()
    );

    // Storm batch: per-request latency (trips the gate for a large fraction) plus 429/503/410
    // faults scoped to the region the baseline probe actually used (to induce retries/failover).
    let rules = vec![
        latency_rule("storm-latency", STORM_LATENCY),
        error_rule(
            "storm-429",
            FaultInjectionErrorType::TooManyRequests,
            target_region.clone(),
            32,
        ),
        error_rule(
            "storm-503",
            FaultInjectionErrorType::ServiceUnavailable,
            target_region.clone(),
            32,
        ),
        error_rule(
            "storm-410",
            FaultInjectionErrorType::PartitionIsGone,
            target_region,
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
        // Fault injection must be active. The latency rule is region-agnostic and trips the gate on
        // every reached op, so a zero here means the injected faults never took effect at all.
        assert!(
            storm.gate_fired > 0,
            "storm reached the service but the diagnostics gate never fired — injected faults were not applied"
        );
        // The storm must actually induce retries. We only reach this point after deriving
        // `target_region` from a region the baseline probe really contacted, so the region-scoped
        // 429/503/410 rules apply to this operation and must produce strictly more attempts than the
        // fault-free baseline. If this fails, the region-scoped faults silently did not match (e.g. a
        // region-name normalization mismatch between `regions_contacted()` and the endpoint URL the
        // matcher tests), and the test would otherwise pass green WITHOUT exercising the retry-storm
        // compaction it exists to validate — the exact silent no-op this guard prevents.
        assert!(
            storm.max_request_count > baseline.max_request_count,
            "storm did not induce retries (storm max_requests={}, baseline max_requests={}); the region-scoped faults did not take effect, so compaction was never exercised",
            storm.max_request_count,
            baseline.max_request_count
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
