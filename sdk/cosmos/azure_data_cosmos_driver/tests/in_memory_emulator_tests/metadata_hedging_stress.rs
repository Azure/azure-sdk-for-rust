// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fault-injection **stress harness** for cross-region metadata hedging.
//!
//! This is the Rust counterpart of the .NET metadata-hedging fault-injection
//! harness (`azure-cosmos-dotnet-v3` PR #5999). It answers the same question
//! that PR's stress runs answered:
//!
//! > When the hub region's *metadata* path browns out, how much tail latency
//! > does cross-region metadata hedging actually remove, and what does it cost
//! > in extra cross-region requests?
//!
//! ## Why this is a harness and not a test
//!
//! Every scenario runs the **same workload twice** — once with hedging ON and
//! once with hedging OFF — against an injected multi-second primary-region
//! metadata delay, then reports the latency distributions side by side. The
//! injected delays are real wall-clock sleeps (`azure_core::sleep`), so a full
//! run takes minutes. That makes it useless as a CI gate and valuable as a
//! measurement tool, so it is **skipped by default** twice over:
//!
//! * `#[ignore]`, so a bare `cargo test` never picks it up, and
//! * an explicit `RUN_HEDGE_STRESS=1` env guard, so even
//!   `cargo test -- --ignored` is a no-op unless you opted in.
//!
//! Run it with:
//!
//! ```text
//! RUN_HEDGE_STRESS=1 cargo test -p azure_data_cosmos_driver \
//!     --features fault_injection,__internal_in_memory_emulator \
//!     --test in_memory_emulator metadata_hedging_stress -- --ignored --nocapture
//! ```
//!
//! ## Differences from the .NET harness
//!
//! | | .NET (#5999) | Rust (this file) |
//! |---|---|---|
//! | Backend | live 5-region account | deterministic in-memory emulator (2 regions) |
//! | Fault source | fault-injection `ResponseDelay` / 503 on Gateway metadata | `FaultOperationType::MetadataReadContainer` / `MetadataPartitionKeyRanges` rules |
//! | A/B switch | `AZURE_COSMOS_METADATA_HEDGING_ENABLED` | driver-level [`AvailabilityStrategy`] (Rust has no metadata-specific kill switch) |
//! | Baseline noise | real network jitter | none — deltas are pure hedging effect |
//!
//! Because the emulator has no network jitter, the absolute numbers here are
//! *cleaner* than the .NET run (the "no fault" arms are effectively 0 ms) but
//! the **shape** of the result — how much of an injected hub-region metadata
//! stall hedging removes, and the secondary-region request amplification it
//! costs — is directly comparable.
//!
//! ## How hedge activity is observed
//!
//! Container metadata reads can be issued as a singleton operation, so their
//! `CosmosResponse` carries [`HedgeDiagnostics`] directly. Partition-key-range
//! ReadFeeds are driven internally by the cache and surface no diagnostics to
//! the caller, so the harness installs **zero-effect counter rules** on the
//! alternate region: a fault rule with probability 1.0 and no delay / error /
//! custom response increments its hit count and then forwards the request
//! untouched, which makes it an exact per-region request counter.
//!
//! ## Scenarios
//!
//! 1. **Cold start** — fresh runtime + driver per iteration; times the first
//!    container metadata read plus the first PK-range ReadFeed.
//! 2. **Metadata refresh, low contention** — sequential singleton container
//!    reads against a warm driver.
//! 3. **PK-range cold chain, low contention** — fresh driver per iteration on a
//!    shared runtime, so the container cache is warm but the PK-range cache is
//!    cold and therefore hedge-eligible.
//! 4. **Saturating storm** — N concurrent container metadata reads.
//! 5. **Budgeted storm** — the same storm with the client's concurrent metadata
//!    hedge ceiling set below N, showing the amplification guardrail binding.
//! 6. **Mixed end-to-end** — ~70% warm point reads, ~30% metadata reads.
//! 7. **Fast-fail brownout** — primary metadata returns 503 immediately (no
//!    delay), i.e. the case hedging is *not* supposed to help, used to confirm
//!    hedging does not regress an already-fast failover.
//!
//! ## A Rust-specific caveat worth knowing before reading the numbers
//!
//! Only the **cold** (continuation-less) PK-range page is hedge-eligible. Once
//! a page has been served, its continuation is pinned to the region that served
//! it (`RegionPin`), so a *warm* forced refresh deliberately does not hedge —
//! replaying a change-feed continuation against a different region is not safe.
//! Scenario 3 therefore exercises a cold PK-range chain rather than a warm
//! forced refresh.
//!
//! That pin cuts both ways, and the second edge is the interesting one: when
//! the cold first page hedges and the alternate region wins it, the pin drags
//! the *rest* of the chain to that healthy region too. A multi-page chain that
//! would have paid the primary's stall once per page pays it zero times.

#![cfg(feature = "fault_injection")]

use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_core::http::Url;
use azure_data_cosmos_driver::diagnostics::{HedgeDiagnostics, HedgeTerminalState};
use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::in_memory_emulator::WriteMode;
use azure_data_cosmos_driver::models::{
    AccountReference, ContainerReference, CosmosOperation, DatabaseReference, ItemReference,
    PartitionKey,
};
use azure_data_cosmos_driver::options::{
    AvailabilityStrategy, DriverOptions, HedgeThreshold, HedgingOptions, HedgingStrategy,
    OperationOptions, OperationOptionsBuilder, PartitionFailoverOptions, Region,
};

use super::{create_item_request, setup_multi_region, MultiRegionTestContext};

const ACCOUNT_ENDPOINT: &str = "https://eastus.emulator.local";
const ACCOUNT_KEY: &str = "ZW11bGF0b3JrZXk=";
const DB_NAME: &str = "testdb";
const COLL_NAME: &str = "testcoll";

/// Item seeded for the mixed-workload scenario's warm point reads.
const MIXED_ITEM_ID: &str = "stress-item";
const MIXED_ITEM_PK: &str = "stress-pk";

/// Warm data-plane share of the mixed workload, out of 10. Expressed per-10
/// rather than per-100 so the ratio still holds for short runs.
const MIXED_WARM_PER_10: usize = 7;

/// Env guard. Without it the harness is a no-op even under `--ignored`.
const RUN_GUARD: &str = "RUN_HEDGE_STRESS";

// ─────────────────────────────────────────────────────────────────────────────
// Configuration
// ─────────────────────────────────────────────────────────────────────────────

/// Run parameters. Every field has an env override so a run can be shortened
/// (for a smoke check) or lengthened (for a publishable measurement) without a
/// rebuild.
struct StressConfig {
    /// Injected primary-region metadata delay. Must exceed the fixed 1.5 s
    /// metadata hedge threshold for hedging to engage at all.
    delay: Duration,
    cold_start_iterations: usize,
    refresh_iterations: usize,
    pk_range_iterations: usize,
    storm_concurrency: usize,
    /// Metadata hedge ceiling applied to the *budgeted* storm scenario. Chosen
    /// well below `storm_concurrency` so the cap visibly binds.
    storm_hedge_budget: usize,
    mixed_operations: usize,
    brownout_iterations: usize,
}

impl StressConfig {
    fn from_env() -> Self {
        Self {
            delay: Duration::from_millis(env_u64("HEDGE_STRESS_DELAY_MS", 3000)),
            cold_start_iterations: env_usize("HEDGE_STRESS_COLD_ITERS", 8),
            refresh_iterations: env_usize("HEDGE_STRESS_REFRESH_ITERS", 15),
            pk_range_iterations: env_usize("HEDGE_STRESS_PKRANGE_ITERS", 10),
            storm_concurrency: env_usize("HEDGE_STRESS_STORM_CONCURRENCY", 32),
            storm_hedge_budget: env_usize("HEDGE_STRESS_STORM_BUDGET", 8),
            mixed_operations: env_usize("HEDGE_STRESS_MIXED_OPS", 30),
            brownout_iterations: env_usize("HEDGE_STRESS_BROWNOUT_ITERS", 15),
        }
    }
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

// ─────────────────────────────────────────────────────────────────────────────
// Arms
// ─────────────────────────────────────────────────────────────────────────────

/// The A/B lever. Rust has no metadata-specific kill switch, so the harness
/// toggles the driver-level [`AvailabilityStrategy`], which metadata reads
/// inherit (they execute with `OperationOptions::default()` internally, so a
/// per-operation strategy would never reach them).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Arm {
    Off,
    On,
}

impl Arm {
    const ALL: [Arm; 2] = [Arm::Off, Arm::On];

    fn label(self) -> &'static str {
        match self {
            Arm::Off => "hedging OFF",
            Arm::On => "hedging ON",
        }
    }

    fn availability_strategy(self) -> AvailabilityStrategy {
        match self {
            Arm::Off => AvailabilityStrategy::Disabled,
            // The threshold here only governs data-plane reads; metadata reads
            // always use the fixed 1.5 s metadata threshold.
            Arm::On => AvailabilityStrategy::Hedging(HedgingStrategy::new(
                HedgeThreshold::new(Duration::from_millis(500)).expect("non-zero threshold"),
            )),
        }
    }
}

fn account() -> AccountReference {
    AccountReference::with_master_key(
        Url::parse(ACCOUNT_ENDPOINT).expect("valid endpoint"),
        ACCOUNT_KEY,
    )
}

fn driver_options(arm: Arm) -> DriverOptions {
    driver_options_with_hedge_budget(arm, None)
}

/// Same as [`driver_options`], but optionally caps the driver-wide metadata
/// hedge budget (scenario 5).
fn driver_options_with_hedge_budget(
    arm: Arm,
    metadata_hedge_budget: Option<usize>,
) -> DriverOptions {
    let hedging = match metadata_hedge_budget {
        Some(limit) => HedgingOptions::builder()
            .with_max_concurrent_metadata_attempts(limit)
            .build(),
        None => HedgingOptions::default(),
    };

    DriverOptions::builder(account())
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
        .with_operation_options(
            OperationOptionsBuilder::new()
                .with_availability_strategy(arm.availability_strategy())
                .build(),
        )
        .with_partition_failover_options(
            PartitionFailoverOptions::builder()
                .with_circuit_breaker_enabled(false)
                .build()
                .expect("partition failover options build"),
        )
        .with_hedging_options(hedging)
        .build()
}

// ─────────────────────────────────────────────────────────────────────────────
// Fault rules
// ─────────────────────────────────────────────────────────────────────────────

/// What to do to the primary region's metadata path.
#[derive(Clone, Copy)]
enum PrimaryFault {
    /// Stall past the 1.5 s metadata hedge threshold, then succeed.
    Delay(Duration),
    /// Fail immediately with a retriable 503 (no delay).
    ServiceUnavailable,
}

/// The four rules installed for a run: two that brown out East US metadata, and
/// two zero-effect counters on West US metadata that make cross-region
/// dispatches observable even for internally-driven reads.
struct FaultSet {
    east_container: Arc<FaultInjectionRule>,
    east_pk_ranges: Arc<FaultInjectionRule>,
    west_container: Arc<FaultInjectionRule>,
    west_pk_ranges: Arc<FaultInjectionRule>,
}

impl FaultSet {
    fn new(fault: PrimaryFault) -> Self {
        Self {
            east_container: primary_rule(
                "east-container-read",
                FaultOperationType::MetadataReadContainer,
                fault,
            ),
            east_pk_ranges: primary_rule(
                "east-pk-ranges",
                FaultOperationType::MetadataPartitionKeyRanges,
                fault,
            ),
            west_container: counter_rule(
                "west-container-read",
                FaultOperationType::MetadataReadContainer,
            ),
            west_pk_ranges: counter_rule(
                "west-pk-ranges",
                FaultOperationType::MetadataPartitionKeyRanges,
            ),
        }
    }

    fn rules(&self) -> Vec<Arc<FaultInjectionRule>> {
        vec![
            Arc::clone(&self.east_container),
            Arc::clone(&self.east_pk_ranges),
            Arc::clone(&self.west_container),
            Arc::clone(&self.west_pk_ranges),
        ]
    }

    /// Metadata requests served by (or attempted against) the browned-out
    /// primary region.
    fn primary_requests(&self) -> u32 {
        self.east_container.hit_count() + self.east_pk_ranges.hit_count()
    }

    /// Metadata requests dispatched to the healthy alternate region — hedge
    /// dispatches with hedging ON, plain failover retries with it OFF.
    fn alternate_requests(&self) -> u32 {
        self.west_container.hit_count() + self.west_pk_ranges.hit_count()
    }
}

/// Region-targeted rule that browns out one metadata operation on East US.
fn primary_rule(
    id: &str,
    operation: FaultOperationType,
    fault: PrimaryFault,
) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(operation)
        .with_region(Region::EAST_US)
        .build();

    let result = match fault {
        PrimaryFault::Delay(delay) => FaultInjectionResultBuilder::new()
            .with_delay(delay)
            .with_probability(1.0)
            .build(),
        PrimaryFault::ServiceUnavailable => FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .with_probability(1.0)
            .build(),
    };

    Arc::new(
        FaultInjectionRuleBuilder::new(id, result)
            .with_condition(condition)
            .build(),
    )
}

/// A rule with probability 1.0 and no delay, error, or custom response. It
/// records a hit and then returns `NoEffect`, so the request is forwarded to
/// the real transport untouched — an exact, zero-impact request counter.
fn counter_rule(id: &str, operation: FaultOperationType) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(operation)
        .with_region(Region::WEST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_probability(1.0)
        .build();
    Arc::new(
        FaultInjectionRuleBuilder::new(id, result)
            .with_condition(condition)
            .build(),
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Measurement
// ─────────────────────────────────────────────────────────────────────────────

/// One arm's worth of measurements for a single scenario.
struct ArmResult {
    arm: Arm,
    samples: Vec<Duration>,
    primary_requests: u32,
    alternate_requests: u32,
    /// Hedge races that actually launched an alternate (container reads only —
    /// PK-range ReadFeeds expose no diagnostics).
    hedges_fired: u32,
    /// Hedge races the alternate won.
    hedges_won: u32,
}

impl ArmResult {
    fn percentile(&self, p: f64) -> Duration {
        if self.samples.is_empty() {
            return Duration::ZERO;
        }
        let mut sorted = self.samples.clone();
        sorted.sort_unstable();
        let rank = ((p / 100.0) * sorted.len() as f64).ceil().max(1.0) as usize;
        sorted[rank.min(sorted.len()) - 1]
    }

    fn mean(&self) -> Duration {
        if self.samples.is_empty() {
            return Duration::ZERO;
        }
        self.samples.iter().sum::<Duration>() / self.samples.len() as u32
    }
}

/// Accumulates timings and hedge attribution for one arm of one scenario.
struct Recorder {
    arm: Arm,
    samples: Vec<Duration>,
    hedges_fired: u32,
    hedges_won: u32,
}

impl Recorder {
    fn new(arm: Arm) -> Self {
        Self {
            arm,
            samples: Vec::new(),
            hedges_fired: 0,
            hedges_won: 0,
        }
    }

    fn record(&mut self, elapsed: Duration) {
        self.samples.push(elapsed);
    }

    fn record_hedge(&mut self, diagnostics: Option<&HedgeDiagnostics>) {
        let Some(diag) = diagnostics else { return };
        if diag.alternate_region().is_some() {
            self.hedges_fired += 1;
        }
        if diag.terminal_state() == HedgeTerminalState::AlternateWon {
            self.hedges_won += 1;
        }
    }

    fn finish(self, faults: &FaultSet) -> ArmResult {
        ArmResult {
            arm: self.arm,
            samples: self.samples,
            primary_requests: faults.primary_requests(),
            alternate_requests: faults.alternate_requests(),
            hedges_fired: self.hedges_fired,
            hedges_won: self.hedges_won,
        }
    }
}

/// A scenario's two arms plus the prose that explains what it measured.
struct Scenario {
    name: &'static str,
    detail: String,
    arms: Vec<ArmResult>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Workload primitives
// ─────────────────────────────────────────────────────────────────────────────

/// Builds a runtime over the shared emulator with `faults` installed.
async fn build_runtime(
    ctx: &MultiRegionTestContext,
    faults: &FaultSet,
) -> Arc<azure_data_cosmos_driver::driver::CosmosDriverRuntime> {
    ctx.emulator
        .runtime_builder_with_fault_rules(faults.rules())
        .build()
        .await
        .expect("runtime builds")
}

/// Issues the container metadata read as a singleton operation so its
/// `HedgeDiagnostics` is observable, and returns the elapsed time alongside it.
async fn timed_container_read(
    driver: &Arc<CosmosDriver>,
) -> (Duration, Option<HedgeDiagnostics>, bool) {
    let db_ref = DatabaseReference::from_name(account(), DB_NAME.to_owned());
    let operation = CosmosOperation::read_container_by_name(db_ref, COLL_NAME.to_owned());

    let started = Instant::now();
    let response = driver
        .execute_singleton_operation(operation, OperationOptions::default())
        .await;
    let elapsed = started.elapsed();

    match response {
        Ok(response) => (
            elapsed,
            response.diagnostics().hedge_diagnostics().cloned(),
            true,
        ),
        Err(_) => (elapsed, None, false),
    }
}

/// Drives the PK-range ReadFeed through the cache and returns the elapsed time.
async fn timed_pk_range_read(
    driver: &Arc<CosmosDriver>,
    container: &ContainerReference,
) -> Duration {
    let started = Instant::now();
    driver
        .resolve_all_partition_key_ranges(container, false)
        .await
        .expect("partition key range read succeeds")
        .expect("partition key range read returns topology");
    started.elapsed()
}

/// Warm data-plane point read — no fault applies to it, so it measures the
/// steady-state cost of having hedging enabled at all.
async fn timed_point_read(driver: &Arc<CosmosDriver>, container: &ContainerReference) -> Duration {
    let item_ref = ItemReference::from_name(
        container,
        PartitionKey::from(MIXED_ITEM_PK.to_owned()),
        MIXED_ITEM_ID.to_owned(),
    );
    let started = Instant::now();
    let _ = driver
        .execute_operation(
            CosmosOperation::read_item(item_ref),
            OperationOptions::default(),
        )
        .await;
    started.elapsed()
}

async fn seed_mixed_item(ctx: &MultiRegionTestContext) {
    let body = serde_json::json!({
        "id": MIXED_ITEM_ID,
        "pk": MIXED_ITEM_PK,
        "value": 1,
    });
    let pk_header = format!(r#"["{MIXED_ITEM_PK}"]"#);
    let req = create_item_request(&ctx.east_url, DB_NAME, COLL_NAME, &body, &pk_header, false);
    ctx.emulator
        .execute_request(&req)
        .await
        .expect("seed create returns a response");
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenarios
// ─────────────────────────────────────────────────────────────────────────────

/// Scenario 1 — cold start. A fresh runtime *and* driver per iteration means
/// both the container cache (runtime-scoped) and the PK-range cache
/// (driver-scoped) are empty, so both metadata reads take the browned-out path.
async fn scenario_cold_start(ctx: &MultiRegionTestContext, cfg: &StressConfig) -> Scenario {
    let mut arms = Vec::new();

    for arm in Arm::ALL {
        let faults = FaultSet::new(PrimaryFault::Delay(cfg.delay));
        let mut recorder = Recorder::new(arm);

        for _ in 0..cfg.cold_start_iterations {
            let runtime = build_runtime(ctx, &faults).await;
            let driver = runtime
                .create_driver(driver_options(arm))
                .await
                .expect("driver initializes");

            let started = Instant::now();
            let container = driver
                .resolve_container_by_name(DB_NAME, COLL_NAME, OperationOptions::default())
                .await
                .expect("container resolves");
            driver
                .resolve_all_partition_key_ranges(&container, false)
                .await
                .expect("cold partition key range read succeeds")
                .expect("cold partition key range read returns topology");
            recorder.record(started.elapsed());
        }

        arms.push(recorder.finish(&faults));
    }

    Scenario {
        name: "Cold start (container read + PK-range ReadFeed)",
        detail: format!(
            "Fresh runtime and driver per iteration, so both metadata caches are cold. \
             {} iterations per arm.",
            cfg.cold_start_iterations
        ),
        arms,
    }
}

/// Scenario 2 — sequential container metadata reads against a warm driver.
/// Singleton execution bypasses the container cache, so every iteration pays
/// the browned-out primary.
async fn scenario_refresh_low_contention(
    ctx: &MultiRegionTestContext,
    cfg: &StressConfig,
) -> Scenario {
    let mut arms = Vec::new();

    for arm in Arm::ALL {
        let faults = FaultSet::new(PrimaryFault::Delay(cfg.delay));
        let runtime = build_runtime(ctx, &faults).await;
        let driver = runtime
            .create_driver(driver_options(arm))
            .await
            .expect("driver initializes");

        let mut recorder = Recorder::new(arm);
        for _ in 0..cfg.refresh_iterations {
            let (elapsed, diagnostics, ok) = timed_container_read(&driver).await;
            assert!(
                ok,
                "container metadata read should succeed under a delay fault"
            );
            recorder.record(elapsed);
            recorder.record_hedge(diagnostics.as_ref());
        }

        arms.push(recorder.finish(&faults));
    }

    Scenario {
        name: "Metadata refresh — Collection Read, low contention",
        detail: format!(
            "Sequential singleton container metadata reads against a warm driver. \
             {} iterations per arm.",
            cfg.refresh_iterations
        ),
        arms,
    }
}

/// Scenario 3 — cold PK-range chain. A fresh driver on a *shared* runtime keeps
/// the container cache warm while emptying the driver-scoped PK-range cache, so
/// the timed read is the continuation-less first page — the only PK-range page
/// that is hedge-eligible.
async fn scenario_pk_range_cold_chain(
    ctx: &MultiRegionTestContext,
    cfg: &StressConfig,
) -> Scenario {
    let mut arms = Vec::new();

    for arm in Arm::ALL {
        let faults = FaultSet::new(PrimaryFault::Delay(cfg.delay));
        let runtime = build_runtime(ctx, &faults).await;

        let mut recorder = Recorder::new(arm);
        for _ in 0..cfg.pk_range_iterations {
            let driver = runtime
                .create_driver(driver_options(arm))
                .await
                .expect("driver initializes");
            let container = driver
                .resolve_container_by_name(DB_NAME, COLL_NAME, OperationOptions::default())
                .await
                .expect("container resolves");
            recorder.record(timed_pk_range_read(&driver, &container).await);
        }

        arms.push(recorder.finish(&faults));
    }

    Scenario {
        name: "PK-range ReadFeed — cold chain, low contention",
        detail: format!(
            "Fresh driver per iteration on a shared runtime: warm container cache, cold \
             PK-range cache. Only the continuation-less first page is hedge-eligible. \
             {} iterations per arm.",
            cfg.pk_range_iterations
        ),
        arms,
    }
}

/// Scenario 4 — saturating storm. N container metadata reads dispatched at
/// once, mirroring the .NET "refresh storm" run where every caller piles onto
/// the same browned-out metadata endpoint.
async fn scenario_saturating_storm(ctx: &MultiRegionTestContext, cfg: &StressConfig) -> Scenario {
    let mut arms = Vec::new();

    for arm in Arm::ALL {
        let faults = FaultSet::new(PrimaryFault::Delay(cfg.delay));
        let runtime = build_runtime(ctx, &faults).await;
        let driver = runtime
            .create_driver(driver_options(arm))
            .await
            .expect("driver initializes");

        let pending = (0..cfg.storm_concurrency).map(|_| timed_container_read(&driver));
        let outcomes = futures::future::join_all(pending).await;

        let mut recorder = Recorder::new(arm);
        for (elapsed, diagnostics, ok) in outcomes {
            assert!(
                ok,
                "container metadata read should succeed under a delay fault"
            );
            recorder.record(elapsed);
            recorder.record_hedge(diagnostics.as_ref());
        }

        arms.push(recorder.finish(&faults));
    }

    Scenario {
        name: "Saturating storm — concurrent Collection Reads",
        detail: format!(
            "{} container metadata reads dispatched concurrently against one driver.",
            cfg.storm_concurrency
        ),
        arms,
    }
}

/// Scenario 5 — the same storm, but with the client's concurrent-metadata-hedge
/// ceiling set well below the storm size. This is the guardrail from #4914: it
/// shows what the budget buys (bounded amplification on the alternate region)
/// and what it costs (only the admitted subset gets the latency recovery).
async fn scenario_budgeted_storm(ctx: &MultiRegionTestContext, cfg: &StressConfig) -> Scenario {
    let mut arms = Vec::new();

    for arm in Arm::ALL {
        let faults = FaultSet::new(PrimaryFault::Delay(cfg.delay));
        let runtime = build_runtime(ctx, &faults).await;
        // The OFF arm never hedges, so the ceiling is irrelevant there; applying
        // it only to the ON arm keeps the comparison to a single variable.
        let budget = (arm == Arm::On).then_some(cfg.storm_hedge_budget);
        let driver = runtime
            .create_driver(driver_options_with_hedge_budget(arm, budget))
            .await
            .expect("driver initializes");

        let pending = (0..cfg.storm_concurrency).map(|_| timed_container_read(&driver));
        let outcomes = futures::future::join_all(pending).await;

        let mut recorder = Recorder::new(arm);
        for (elapsed, diagnostics, ok) in outcomes {
            assert!(
                ok,
                "container metadata read should succeed even when refused a hedge slot"
            );
            recorder.record(elapsed);
            recorder.record_hedge(diagnostics.as_ref());
        }

        arms.push(recorder.finish(&faults));
    }

    Scenario {
        name: "Budgeted storm — concurrent Collection Reads, hedge ceiling applied",
        detail: format!(
            "Same {} concurrent container metadata reads, but the ON arm's client is \
             capped at {} simultaneous metadata hedge races. Operations refused a slot \
             skip the hedge and take the ordinary sequential path, so the alternate \
             region absorbs at most {} extra requests instead of {}.",
            cfg.storm_concurrency,
            cfg.storm_hedge_budget,
            cfg.storm_hedge_budget,
            cfg.storm_concurrency,
        ),
        arms,
    }
}

/// Scenario 6 — mixed end-to-end. Most traffic is warm data-plane reads that no
/// fault touches; the metadata minority is what hedging can act on. This is the
/// "what does the application actually see" number.
async fn scenario_mixed_workload(ctx: &MultiRegionTestContext, cfg: &StressConfig) -> Scenario {
    let mut arms = Vec::new();

    for arm in Arm::ALL {
        let faults = FaultSet::new(PrimaryFault::Delay(cfg.delay));
        let runtime = build_runtime(ctx, &faults).await;
        let driver = runtime
            .create_driver(driver_options(arm))
            .await
            .expect("driver initializes");
        let container = driver
            .resolve_container_by_name(DB_NAME, COLL_NAME, OperationOptions::default())
            .await
            .expect("container resolves");

        // Warm both caches first: the point-read leg is only meaningful as
        // "warm" traffic if it is not paying for a cold PK-range chain.
        driver
            .resolve_all_partition_key_ranges(&container, false)
            .await
            .expect("partition key range warm-up succeeds")
            .expect("partition key range warm-up returns topology");
        let _ = timed_point_read(&driver, &container).await;

        let mut recorder = Recorder::new(arm);
        for i in 0..cfg.mixed_operations {
            if i % 10 < MIXED_WARM_PER_10 {
                recorder.record(timed_point_read(&driver, &container).await);
            } else {
                let (elapsed, diagnostics, _) = timed_container_read(&driver).await;
                recorder.record(elapsed);
                recorder.record_hedge(diagnostics.as_ref());
            }
        }

        arms.push(recorder.finish(&faults));
    }

    Scenario {
        name: "Mixed end-to-end (~70% warm point reads / ~30% metadata)",
        detail: format!(
            "{} operations per arm, interleaved 7:3. Both caches are pre-warmed and no fault \
             touches the point reads, so this measures what an application sees rather than \
             the metadata path alone.",
            cfg.mixed_operations
        ),
        arms,
    }
}

/// Scenario 7 — fast-fail brownout. The primary returns 503 immediately, so
/// ordinary failover already recovers in milliseconds. Hedging should neither
/// help nor hurt here; this arm exists to prove the second claim.
async fn scenario_fast_fail_brownout(ctx: &MultiRegionTestContext, cfg: &StressConfig) -> Scenario {
    let mut arms = Vec::new();

    for arm in Arm::ALL {
        let faults = FaultSet::new(PrimaryFault::ServiceUnavailable);
        let runtime = build_runtime(ctx, &faults).await;
        let driver = runtime
            .create_driver(driver_options(arm))
            .await
            .expect("driver initializes");

        let mut recorder = Recorder::new(arm);
        for _ in 0..cfg.brownout_iterations {
            let (elapsed, diagnostics, _) = timed_container_read(&driver).await;
            recorder.record(elapsed);
            recorder.record_hedge(diagnostics.as_ref());
        }

        arms.push(recorder.finish(&faults));
    }

    Scenario {
        name: "Fast-fail brownout (primary metadata 503, no delay)",
        detail: format!(
            "Primary metadata fails immediately, so ordinary cross-region failover already \
             recovers fast. {} iterations per arm.",
            cfg.brownout_iterations
        ),
        arms,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Reporting
// ─────────────────────────────────────────────────────────────────────────────

fn ms(duration: Duration) -> String {
    let value = duration.as_secs_f64() * 1000.0;
    if value < 10.0 {
        format!("{value:.1}")
    } else {
        format!("{value:.0}")
    }
}

/// Percentage change from `base` to `arm`, rendered as a signed integer.
///
/// Sub-5 ms baselines fall back to an absolute delta: a percentage of a value
/// that small says more about scheduler jitter than about hedging.
fn delta_pct(base: Duration, arm: Duration) -> String {
    let base_ms = base.as_secs_f64() * 1000.0;
    let arm_ms = arm.as_secs_f64() * 1000.0;
    if base_ms <= 0.0 {
        return "n/a".to_owned();
    }
    if base_ms < 5.0 {
        return format!("{:+.1} ms", arm_ms - base_ms);
    }
    format!("{:+.0}%", (arm_ms - base_ms) / base_ms * 100.0)
}

fn render(scenario: &Scenario) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "\n### {}\n\n{}\n\n",
        scenario.name, scenario.detail
    ));
    out.push_str("| arm | n | p50 (ms) | p90 (ms) | p99 (ms) | max (ms) | mean (ms) |\n");
    out.push_str("|---|---:|---:|---:|---:|---:|---:|\n");

    for result in &scenario.arms {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            result.arm.label(),
            result.samples.len(),
            ms(result.percentile(50.0)),
            ms(result.percentile(90.0)),
            ms(result.percentile(99.0)),
            ms(result.percentile(100.0)),
            ms(result.mean()),
        ));
    }

    let off = scenario.arms.iter().find(|r| r.arm == Arm::Off);
    let on = scenario.arms.iter().find(|r| r.arm == Arm::On);
    if let (Some(off), Some(on)) = (off, on) {
        out.push_str(&format!(
            "| **delta (ON vs OFF)** | | **{}** | **{}** | **{}** | **{}** | **{}** |\n",
            delta_pct(off.percentile(50.0), on.percentile(50.0)),
            delta_pct(off.percentile(90.0), on.percentile(90.0)),
            delta_pct(off.percentile(99.0), on.percentile(99.0)),
            delta_pct(off.percentile(100.0), on.percentile(100.0)),
            delta_pct(off.mean(), on.mean()),
        ));
    }

    out.push_str("\n| arm | primary-region metadata reqs | alternate-region metadata reqs | hedges fired | hedges won by alternate |\n");
    out.push_str("|---|---:|---:|---:|---:|\n");
    for result in &scenario.arms {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            result.arm.label(),
            result.primary_requests,
            result.alternate_requests,
            result.hedges_fired,
            result.hedges_won,
        ));
    }

    if let (Some(off), Some(on)) = (off, on) {
        let off_total = off.primary_requests + off.alternate_requests;
        let on_total = on.primary_requests + on.alternate_requests;
        if off_total > 0 {
            out.push_str(&format!(
                "\nMetadata request amplification (ON / OFF): **{:.2}x** ({} → {} total metadata requests).\n",
                on_total as f64 / off_total as f64,
                off_total,
                on_total,
            ));
        }
    }

    out
}

fn write_csv(scenarios: &[Scenario]) -> Option<std::path::PathBuf> {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("hedge-stress");
    std::fs::create_dir_all(&dir).ok()?;
    let path = dir.join("metadata_hedging_stress.csv");

    let mut csv = String::from(
        "scenario,arm,n,p50_ms,p90_ms,p99_ms,max_ms,mean_ms,primary_requests,alternate_requests,hedges_fired,hedges_won\n",
    );
    for scenario in scenarios {
        for result in &scenario.arms {
            csv.push_str(&format!(
                "\"{}\",{},{},{},{},{},{},{},{},{},{},{}\n",
                scenario.name,
                result.arm.label(),
                result.samples.len(),
                ms(result.percentile(50.0)),
                ms(result.percentile(90.0)),
                ms(result.percentile(99.0)),
                ms(result.percentile(100.0)),
                ms(result.mean()),
                result.primary_requests,
                result.alternate_requests,
                result.hedges_fired,
                result.hedges_won,
            ));
        }
    }

    std::fs::write(&path, csv).ok()?;
    Some(path)
}

// ─────────────────────────────────────────────────────────────────────────────
// Entry point
// ─────────────────────────────────────────────────────────────────────────────

/// Runs every scenario in both arms and prints a markdown report.
///
/// Skipped unless `RUN_HEDGE_STRESS=1`; see the module docs for the full
/// invocation.
#[ignore = "long-running fault-injection stress harness; set RUN_HEDGE_STRESS=1 to run"]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn metadata_hedging_stress_harness() {
    if std::env::var(RUN_GUARD).as_deref() != Ok("1") {
        eprintln!("skipping metadata hedging stress harness: set {RUN_GUARD}=1 to run it");
        return;
    }

    let cfg = StressConfig::from_env();
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_mixed_item(&ctx).await;

    let overall = Instant::now();
    let scenarios = vec![
        Box::pin(scenario_cold_start(&ctx, &cfg)).await,
        Box::pin(scenario_refresh_low_contention(&ctx, &cfg)).await,
        Box::pin(scenario_pk_range_cold_chain(&ctx, &cfg)).await,
        Box::pin(scenario_saturating_storm(&ctx, &cfg)).await,
        Box::pin(scenario_budgeted_storm(&ctx, &cfg)).await,
        Box::pin(scenario_mixed_workload(&ctx, &cfg)).await,
        Box::pin(scenario_fast_fail_brownout(&ctx, &cfg)).await,
    ];

    let mut report = String::new();
    report.push_str("\n## Metadata hedging — fault-injection stress harness\n\n");
    report.push_str(&format!(
        "In-memory emulator, 2 regions (East US primary, West US alternate). \
         Injected primary-region metadata delay: **{} ms** (metadata hedge threshold is a fixed \
         1.5 s). Total wall clock: {:.1} s.\n",
        cfg.delay.as_millis(),
        overall.elapsed().as_secs_f64(),
    ));

    for scenario in &scenarios {
        report.push_str(&render(scenario));
    }

    if let Some(path) = write_csv(&scenarios) {
        report.push_str(&format!("\nCSV written to `{}`.\n", path.display()));
    }

    println!("{report}");

    // Sanity floor: the harness is only meaningful if hedging actually engaged
    // somewhere. Anything weaker than this means the run measured nothing.
    let hedged = scenarios
        .iter()
        .flat_map(|s| s.arms.iter())
        .any(|r| r.arm == Arm::On && (r.hedges_fired > 0 || r.alternate_requests > 0));
    assert!(
        hedged,
        "no hedge activity observed in any ON arm — the harness measured nothing",
    );
}
