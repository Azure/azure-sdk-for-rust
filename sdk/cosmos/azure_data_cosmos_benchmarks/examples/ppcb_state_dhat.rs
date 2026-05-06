// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Heap-allocation profile of the **PPCB** (Per-Partition Circuit Breaker)
//! routing state, enabled vs. disabled.
//!
//! Builds a populated [`PartitionEndpointState`] representing the **steady
//! state** after `N` partitions have each experienced `K` region failures,
//! drops it, and writes a `dhat-heap.json` trace to the working directory.
//!
//! Two modes are available via the `PPCB_MODE` env var (or the first CLI arg):
//!
//! - `disabled` *(baseline)* — `circuit_breaker_option_enabled = false`. The
//!   driver's `mark_partition_unavailable` short-circuits before inserting,
//!   so `circuit_breaker_overrides` stays empty. Captures the static cost of
//!   carrying the (empty) PPCB state alongside `AccountEndpointState`.
//! - `enabled` — `circuit_breaker_option_enabled = true`. We synthesize the
//!   resulting steady state by directly inserting one
//!   [`PartitionFailoverEntry`] per PK range, each holding `K` failed
//!   endpoints in `failed_endpoints` and a non-default
//!   `current_endpoint`. This mirrors the heap shape the driver would reach
//!   after PPCB has tripped on every partition.
//!
//! Workload sizing is controlled by env vars:
//!
//! - `PPCB_NUM_PARTITIONS` — default `80000`.
//! - `PPCB_NUM_FAILED_REGIONS` — default `2`. Each entry's `failed_endpoints`
//!   set will hold this many endpoints.
//!
//! Run with:
//!
//! ```text
//! # baseline (PPCB disabled)
//! cargo run -p azure_data_cosmos_benchmarks `
//!     --release --features dhat-heap --example ppcb_state_dhat -- disabled
//!
//! # PPCB enabled, 80k partitions, 2 failed regions per partition (defaults)
//! cargo run -p azure_data_cosmos_benchmarks `
//!     --release --features dhat-heap --example ppcb_state_dhat -- enabled
//! ```
//!
//! The output `dhat-heap.json` is named per-mode (`dhat-heap-ppcb-disabled.json`
//! / `dhat-heap-ppcb-enabled.json`) so successive runs do not clobber the
//! previous trace. Open with the DHAT viewer
//! (<https://nnethercote.github.io/dh_view/dh_view.html>).
//!
//! `--release` matters: debug allocations look very different from release
//! (Vec over-reservation, no SSO, Box::new not inlined).

use std::collections::{HashMap, HashSet};
use std::env;
use std::mem::size_of;
use std::str::FromStr;
use std::time::{Duration, Instant};

use azure_data_cosmos_driver::testing::{
    CosmosEndpoint, HealthStatus, PartitionEndpointState, PartitionFailoverConfig,
    PartitionFailoverEntry, PartitionKeyRangeId,
};
use url::Url;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

/// Default number of partition key ranges to populate. Sized to match a large
/// production container (~80k physical partitions).
const DEFAULT_NUM_PARTITIONS: usize = 80_000;

/// Default number of regions marked failed per partition. Each entry's
/// `failed_endpoints` set will hold this many `CosmosEndpoint`s.
const DEFAULT_NUM_FAILED_REGIONS: usize = 2;

/// Total regions to materialize on the synthetic account. Must be strictly
/// greater than `num_failed_regions` so that `current_endpoint` can point at a
/// region that is *not* in `failed_endpoints` (matches what the driver does
/// after PPCB advances past the failed region).
const TOTAL_REGIONS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Disabled,
    Enabled,
}

impl FromStr for Mode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "disabled" | "off" | "baseline" => Ok(Self::Disabled),
            "enabled" | "on" => Ok(Self::Enabled),
            other => Err(format!(
                "unknown PPCB_MODE '{other}'; expected 'disabled' or 'enabled'"
            )),
        }
    }
}

impl Mode {
    fn dhat_filename(self) -> &'static str {
        match self {
            Self::Disabled => "dhat-heap-ppcb-disabled.json",
            Self::Enabled => "dhat-heap-ppcb-enabled.json",
        }
    }
}

fn read_usize_env(key: &str, default: usize) -> usize {
    match env::var(key) {
        Ok(v) => v
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("invalid {key}='{v}': {e}")),
        Err(_) => default,
    }
}

fn read_mode() -> Mode {
    // First CLI arg wins; otherwise PPCB_MODE; otherwise default to enabled
    // (the more interesting case if someone forgets to specify).
    if let Some(arg) = env::args().nth(1) {
        return arg.parse().expect("invalid mode argument");
    }
    match env::var("PPCB_MODE") {
        Ok(v) => v.parse().expect("invalid PPCB_MODE"),
        Err(_) => Mode::Enabled,
    }
}

fn build_endpoints(count: usize) -> Vec<CosmosEndpoint> {
    (0..count)
        .map(|i| {
            // Use a realistic-looking gateway URL so the endpoint key /
            // host parsing exercises the same code paths as production.
            let url = Url::parse(&format!(
                "https://account-region{i:02}.documents.azure.com:443/"
            ))
            .expect("static URL parses");
            CosmosEndpoint::global(url)
        })
        .collect()
}

fn build_populated_overrides(
    num_partitions: usize,
    num_failed_regions: usize,
    endpoints: &[CosmosEndpoint],
) -> HashMap<PartitionKeyRangeId, PartitionFailoverEntry> {
    assert!(
        num_failed_regions > 0 && num_failed_regions < endpoints.len(),
        "need at least one failed region and one healthy region remaining"
    );

    let now = Instant::now();
    // First `num_failed_regions` endpoints are the failed set; the next
    // endpoint becomes the current routing target. This matches what the
    // driver's `try_move_next_endpoint` produces after PPCB has advanced
    // past every failed region.
    let failed_slice = &endpoints[..num_failed_regions];
    let current_endpoint = endpoints[num_failed_regions].clone();
    let first_failed_endpoint = failed_slice[0].clone();

    let mut map = HashMap::with_capacity(num_partitions);
    for i in 0..num_partitions {
        let pk_range_id = PartitionKeyRangeId::from_str(&i.to_string()).expect("Infallible parse");

        let failed_endpoints: HashSet<CosmosEndpoint> = failed_slice.iter().cloned().collect();

        let entry = PartitionFailoverEntry {
            current_endpoint: current_endpoint.clone(),
            first_failed_endpoint: first_failed_endpoint.clone(),
            failed_endpoints,
            // Threshold is 10 reads (default); use a representative value
            // just past the trip point.
            read_failure_count: 11,
            write_failure_count: 0,
            first_failure_time: now,
            last_failure_time: now,
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::from_millis(0),
        };
        map.insert(pk_range_id, entry);
    }
    map
}

fn report_sizes() {
    eprintln!("--- ppcb_state_dhat: size_of (bytes) ---");
    eprintln!(
        "  PartitionEndpointState   = {}",
        size_of::<PartitionEndpointState>()
    );
    eprintln!(
        "  PartitionFailoverEntry   = {}",
        size_of::<PartitionFailoverEntry>()
    );
    eprintln!(
        "  PartitionFailoverConfig  = {}",
        size_of::<PartitionFailoverConfig>()
    );
    eprintln!(
        "  PartitionKeyRangeId      = {}",
        size_of::<PartitionKeyRangeId>()
    );
    eprintln!(
        "  CosmosEndpoint           = {}",
        size_of::<CosmosEndpoint>()
    );
    eprintln!("----------------------------------------");
}

fn main() {
    let mode = read_mode();
    let num_partitions = read_usize_env("PPCB_NUM_PARTITIONS", DEFAULT_NUM_PARTITIONS);
    let num_failed_regions = read_usize_env("PPCB_NUM_FAILED_REGIONS", DEFAULT_NUM_FAILED_REGIONS);

    eprintln!(
        "ppcb_state_dhat: mode={:?}, partitions={}, failed_regions_per_partition={}, \
         total_account_regions={}",
        mode, num_partitions, num_failed_regions, TOTAL_REGIONS,
    );
    report_sizes();

    // Profiler is installed *after* size reporting and arg parsing so those
    // allocations are excluded from the trace. Everything below this line is
    // captured.
    let profiler = dhat::Profiler::builder()
        .file_name(mode.dhat_filename())
        .build();

    let endpoints = build_endpoints(TOTAL_REGIONS);

    // Build the PPCB config matching production defaults; the only knob we
    // flip per mode is `circuit_breaker_option_enabled`. `Default::default`
    // gives the same thresholds the driver computes from env vars.
    let config = PartitionFailoverConfig {
        circuit_breaker_option_enabled: matches!(mode, Mode::Enabled),
        ..PartitionFailoverConfig::default()
    };
    let mut state = PartitionEndpointState::new(config);

    if matches!(mode, Mode::Enabled) {
        // Direct-insert simulation of the driver's steady state after PPCB
        // has tripped on every partition. We bypass `mark_partition_unavailable`
        // because it clones the entire state on every call (CAS pattern), which
        // would be O(N^2) for 80k entries; the resulting *steady-state heap*
        // is identical, and that is what we want to measure.
        state.circuit_breaker_overrides =
            build_populated_overrides(num_partitions, num_failed_regions, &endpoints);
    }
    // For Mode::Disabled we leave both maps empty — exactly what the driver
    // produces when `mark_partition_unavailable` short-circuits on the
    // `is_eligible_for_ppcb` check (see routing_systems.rs).

    eprintln!(
        "built PartitionEndpointState: cb_overrides={}, ppaf_overrides={}, \
         ppcb_enabled={}",
        state.circuit_breaker_overrides.len(),
        state.failover_overrides.len(),
        state.per_partition_circuit_breaker_enabled,
    );

    // Drop everything explicitly so the profiler captures the deallocations.
    drop(state);
    drop(endpoints);

    // Profiler dropped here, writing `<mode>.json` to the cwd.
    drop(profiler);
}
