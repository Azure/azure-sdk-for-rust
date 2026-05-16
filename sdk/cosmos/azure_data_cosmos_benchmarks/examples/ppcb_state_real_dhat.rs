// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end heap-allocation profile of the **PPCB** routing state against a
//! **real** Azure Cosmos DB account.
//!
//! Sibling of [`ppcb_state_dhat`](./ppcb_state_dhat.rs) — the latter measures
//! the steady-state shape of a synthesized [`PartitionEndpointState`]; this
//! example measures the same shape **layered on top of a real `CosmosDriver`**
//! constructed against a live account. The delta between the two modes below
//! is the marginal heap cost of PPCB-tripped state on top of a real driver's
//! resting footprint.
//!
//! ## Why two modes
//!
//! - `baseline`  — Construct a real `CosmosDriverRuntime` + `CosmosDriver`
//!   against the supplied account, let initialization complete (metadata
//!   probe, account discovery, default-endpoint resolution), then drop. The
//!   heap captured is **everything the driver allocates just to exist**:
//!   account metadata cache, transport, location state store, regional
//!   endpoints, session manager, PK-range cache, background-task plumbing.
//! - `injected` — Same as `baseline`, plus a synthetic injection step that
//!   uses [`LocationStateStore::apply_partition`] to insert `N` PPCB entries
//!   into the **real driver's** partition state — same CAS path the operation
//!   pipeline would take. Each entry carries `K` failed `CosmosEndpoint`s
//!   that originate from the real account's discovered regions, so the
//!   `Arc<EndpointKey>` payloads are the real ones.
//!
//! The injection bypasses [`mark_partition_unavailable`] only because that
//! function is `pub(crate)` — the heap shape it produces is identical, and
//! we exercise the same `apply_partition` CAS loop the real pipeline uses.
//!
//! ## What this is *not*
//!
//! This is **not** a test that splits a container into N physical partitions
//! and tears them down through real PPCB failover. Provisioning N partitions
//! requires either O(N) RU/s of throughput or O(N × 50 GB) of storage, both
//! of which dominate the per-entry heap cost we're trying to measure. The
//! steady-state heap shape we capture is what matters; the driver doesn't
//! know (or care) whether the PK range IDs in `circuit_breaker_overrides`
//! came from real `/pkranges` responses or from a deterministic generator.
//!
//! ## Required env vars
//!
//! - `COSMOS_CONNECTION_STRING` — e.g.
//!   `"AccountEndpoint=https://acct.documents.azure.com:443/;AccountKey=..."`.
//! - Optional `PPCB_NUM_PARTITIONS` — default `115`. The injection step
//!   inserts this many synthetic PK range IDs.
//! - Optional `PPCB_NUM_FAILED_REGIONS` — default `2`.
//! - Optional `PPCB_MODE` — `baseline` (default) or `injected`. The first
//!   CLI arg overrides the env var.
//!
//! ## Run
//!
//! ```text
//! $env:COSMOS_CONNECTION_STRING = "AccountEndpoint=...;AccountKey=...;"
//! cargo run -p azure_data_cosmos_benchmarks `
//!     --release --features dhat-heap --example ppcb_state_real_dhat -- baseline
//!
//! cargo run -p azure_data_cosmos_benchmarks `
//!     --release --features dhat-heap --example ppcb_state_real_dhat -- injected
//! ```
//!
//! Output files are named per-mode (`dhat-heap-real-baseline-N{N}.json` /
//! `dhat-heap-real-injected-N{N}.json`) and are written next to the current
//! working directory. Open with the DHAT viewer
//! (<https://nnethercote.github.io/dh_view/dh_view.html>).

use std::env;
use std::mem::size_of;
use std::str::FromStr;
use std::time::{Duration, Instant};

use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
use azure_data_cosmos_driver::models::{AccountReference, ConnectionString};
use azure_data_cosmos_driver::testing::{
    CosmosEndpoint, FailedEndpoints, HealthStatus, PartitionEndpointState, PartitionFailoverConfig,
    PartitionFailoverEntry, PartitionKeyRangeId,
};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

/// Default number of synthetic PK range IDs to inject in `injected` mode.
/// Sized to roughly match the supplied test account (115 partitions).
const DEFAULT_NUM_PARTITIONS: usize = 115;

/// Default number of failed endpoints to record per partition entry.
const DEFAULT_NUM_FAILED_REGIONS: usize = 2;

/// Environment variable holding the Cosmos DB connection string.
const CONNECTION_STRING_ENV: &str = "COSMOS_CONNECTION_STRING";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Baseline,
    Injected,
}

impl FromStr for Mode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "baseline" | "off" => Ok(Self::Baseline),
            "injected" | "ppcb" | "on" => Ok(Self::Injected),
            other => Err(format!(
                "unknown PPCB_MODE '{other}'; expected 'baseline' or 'injected'"
            )),
        }
    }
}

impl Mode {
    fn dhat_filename(self, num_partitions: usize) -> String {
        match self {
            Self::Baseline => format!("dhat-heap-real-baseline-N{num_partitions}.json"),
            Self::Injected => format!("dhat-heap-real-injected-N{num_partitions}.json"),
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
    if let Some(arg) = env::args().nth(1) {
        return arg.parse().expect("invalid mode argument");
    }
    match env::var("PPCB_MODE") {
        Ok(v) => v.parse().expect("invalid PPCB_MODE"),
        Err(_) => Mode::Baseline,
    }
}

fn read_account() -> AccountReference {
    let conn_str_raw = env::var(CONNECTION_STRING_ENV).unwrap_or_else(|_| {
        panic!("{CONNECTION_STRING_ENV} must be set to the Cosmos account connection string")
    });
    let conn: ConnectionString = conn_str_raw
        .parse()
        .expect("connection string must parse (AccountEndpoint=...;AccountKey=...)");
    let endpoint = conn
        .account_endpoint()
        .parse()
        .expect("AccountEndpoint must be a valid URL");
    let key = conn.account_key().secret().to_string();
    AccountReference::with_master_key(endpoint, key)
}

fn report_sizes() {
    eprintln!("--- ppcb_state_real_dhat: size_of (bytes) ---");
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
    eprintln!("---------------------------------------------");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = read_mode();
    let num_partitions = read_usize_env("PPCB_NUM_PARTITIONS", DEFAULT_NUM_PARTITIONS);
    let num_failed_regions = read_usize_env("PPCB_NUM_FAILED_REGIONS", DEFAULT_NUM_FAILED_REGIONS);

    let account = read_account();

    eprintln!(
        "ppcb_state_real_dhat: mode={:?}, endpoint={}, partitions={}, failed_regions_per_partition={}",
        mode,
        account.endpoint(),
        num_partitions,
        num_failed_regions,
    );
    report_sizes();

    // Profiler attached *after* arg parsing / env reads so those allocations
    // are excluded from the trace. Everything after this line is captured.
    let profiler = dhat::Profiler::builder()
        .file_name(mode.dhat_filename(num_partitions))
        .build();

    // Build a real runtime + driver against the supplied account. This
    // performs HTTP/2 probe, account metadata bootstrap, default-endpoint
    // resolution, and starts the background failback loop.
    let runtime = CosmosDriverRuntime::builder().build().await?;
    let driver = runtime.get_or_create_driver(account, None).await?;
    eprintln!("driver initialized against live account");

    if matches!(mode, Mode::Injected) {
        inject_synthetic_ppcb_state(&driver, num_partitions, num_failed_regions);
        eprintln!("injected {num_partitions} synthetic PPCB entries");
    }

    // Drop driver + runtime so any in-flight `Arc`s settle before the
    // profiler snapshots `t-end`. `t-gmax` (peak) is what we actually
    // care about and is captured continuously regardless.
    drop(driver);
    drop(runtime);

    drop(profiler);
    eprintln!("wrote DHAT trace: {}", mode.dhat_filename(num_partitions));
    Ok(())
}

/// Direct-inserts `num_partitions` synthetic PPCB entries into the real
/// driver's [`PartitionEndpointState`].
///
/// Uses the real `apply_partition` CAS path (same one `mark_partition_unavailable`
/// uses), so the version-counter bump, `Arc::clone` churn, and crossbeam-epoch
/// deferred-drop machinery are all exercised.
///
/// Each entry holds `num_failed_regions` failed endpoints drawn from the
/// driver's discovered account regions. If the account does not advertise
/// enough regions, we synthesize stand-in endpoints — they consume the same
/// `~8 B` slot inside `failed_endpoints`, so the resulting heap shape is
/// indistinguishable from the production case.
fn inject_synthetic_ppcb_state(
    driver: &azure_data_cosmos_driver::driver::CosmosDriver,
    num_partitions: usize,
    num_failed_regions: usize,
) {
    let store = driver.location_state_store();
    let account_snapshot = store.account_snapshot();

    // Prefer the real account's discovered read endpoints; fall back to
    // synthetic ones if the account only advertises one (e.g., single-region
    // INT account). The synthesized endpoints reuse `CosmosEndpoint::global`
    // so they exercise the same `EndpointKey` Arc machinery as the real
    // ones — heap shape parity is preserved.
    let mut endpoints: Vec<CosmosEndpoint> = account_snapshot
        .preferred_read_endpoints
        .iter()
        .cloned()
        .collect();
    let needed = num_failed_regions + 1;
    while endpoints.len() < needed {
        let i = endpoints.len();
        let url = url::Url::parse(&format!(
            "https://synthetic-region{i:02}.documents.azure.com:443/"
        ))
        .expect("static URL parses");
        endpoints.push(CosmosEndpoint::global(url));
    }

    let now = Instant::now();
    let failed_slice = &endpoints[..num_failed_regions];
    let current_endpoint = endpoints[num_failed_regions].clone();
    let first_failed_endpoint = failed_slice[0].clone();

    // Single CAS swap inserts all entries. The pipeline normally does one
    // CAS per call to `mark_partition_unavailable`, which is O(N^2) clones
    // for N partitions. The steady-state heap shape after either path is
    // identical, so we use the cheap form.
    store.apply_partition(|current: &PartitionEndpointState| {
        let mut next = current.clone();
        // Force PPCB to be visible to anyone inspecting flags. The driver
        // would set this when the account property sync sees it; we set
        // it directly because we're never going to round-trip through
        // account-property sync in this benchmark.
        next.per_partition_circuit_breaker_enabled = true;
        for i in 0..num_partitions {
            let pk = format!("pk-{i}");
            let pk_range_id = PartitionKeyRangeId::from(pk);

            let failed_endpoints: FailedEndpoints = failed_slice.iter().cloned().collect();

            let entry = PartitionFailoverEntry {
                current_endpoint: current_endpoint.clone(),
                first_failed_endpoint: first_failed_endpoint.clone(),
                failed_endpoints,
                read_failure_count: 11,
                write_failure_count: 0,
                first_failure_time: now,
                last_failure_time: now,
                health_status: HealthStatus::Unhealthy,
                failback_jitter: Duration::from_millis(0),
            };
            next.circuit_breaker_overrides.insert(pk_range_id, entry);
        }
        next
    });
}
