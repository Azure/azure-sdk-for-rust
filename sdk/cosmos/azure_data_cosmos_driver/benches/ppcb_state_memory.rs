//! Memory micro-benchmark for the PPCB / PPAF partition state introduced
//! in PR #4156.
//!
//! Mirrors `PartitionEndpointState` byte-for-byte (the real types are
//! `pub(crate)` and not reachable from `benches/`). Memory layout depends
//! only on field types and order, not on visibility — so retained-heap
//! deltas reported here equal what the production driver allocates per
//! `LocationStateStore::partitions` snapshot.
//!
//! Scenarios (per (N partitions, M regions) cell):
//!
//!   - **empty**:     fresh `PartitionEndpointState`, no failovers active.
//!   - **failover**:  only `failover_overrides` populated (PPAF active).
//!   - **breaker**:   only `circuit_breaker_overrides` populated (PPCB active).
//!   - **both**:      both maps populated (worst case).
//!   - **direct**:    `Vec<PartitionFailoverEntry>` of size N — isolates
//!                    per-entry cost from container overhead.
//!
//! Grid: N ∈ {100, 1k, 10k, 50k} × M ∈ {2, 4, 6}.
//!
//! Run:
//!   cargo bench --bench ppcb_state_memory
//! or
//!   cargo run --release --bench ppcb_state_memory

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    sync::Arc,
    time::{Duration, Instant},
};

use compact_str::CompactString;
use crossbeam_epoch::{self as epoch, Atomic, Owned};
use smallvec::SmallVec;
use url::Url;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// =========================================================================
// Mirrored types — must match azure_data_cosmos_driver::driver::routing
// =========================================================================
//
// If PR #4156 evolves any of these field shapes, update here too. Layout
// equivalence is what matters for memory accounting.

/// Mirrors `options::Region { normalized: Cow<'static, str> }`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Region {
    normalized: Cow<'static, str>,
}

/// Mirrors `transport::EndpointKey(Arc<str>)`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct EndpointKey(Arc<str>);

/// Mirrors `routing::endpoint::CosmosEndpointData`.
#[derive(Debug, PartialEq, Eq, Hash)]
struct CosmosEndpointData {
    region: Option<Region>,
    gateway_url: Url,
    gateway20_url: Option<Url>,
    endpoint_key: EndpointKey,
}

/// Mirrors `routing::endpoint::CosmosEndpoint(Arc<CosmosEndpointData>)`.
#[derive(Clone, Debug)]
struct CosmosEndpoint(Arc<CosmosEndpointData>);

impl PartialEq for CosmosEndpoint {
    fn eq(&self, other: &Self) -> bool { *self.0 == *other.0 }
}
impl Eq for CosmosEndpoint {}
impl std::hash::Hash for CosmosEndpoint {
    fn hash<H: std::hash::Hasher>(&self, s: &mut H) { self.0.hash(s); }
}

/// Mirrors `routing::partition_key_range_id::PartitionKeyRangeId(String)`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PartitionKeyRangeId(String);

/// Mirrors `routing::partition_endpoint_state::HealthStatus`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum HealthStatus { Unhealthy, ProbeCandidate }

/// Mirrors `routing::partition_endpoint_state::PartitionFailoverEntry`.
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct PartitionFailoverEntry {
    current_endpoint: CosmosEndpoint,
    first_failed_endpoint: CosmosEndpoint,
    failed_endpoints: HashSet<CosmosEndpoint>,
    read_failure_count: i32,
    write_failure_count: i32,
    first_failure_time: Instant,
    last_failure_time: Instant,
    health_status: HealthStatus,
}

/// Mirrors `routing::partition_endpoint_state::PartitionFailoverConfig`.
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct PartitionFailoverConfig {
    read_failure_threshold: i32,
    write_failure_threshold: i32,
    counter_reset_window: Duration,
    partition_unavailability_duration: Duration,
    failback_sweep_interval: Duration,
    circuit_breaker_option_enabled: bool,
}

impl Default for PartitionFailoverConfig {
    fn default() -> Self {
        Self {
            read_failure_threshold: 10,
            write_failure_threshold: 5,
            counter_reset_window: Duration::from_secs(300),
            partition_unavailability_duration: Duration::from_secs(5),
            failback_sweep_interval: Duration::from_secs(300),
            circuit_breaker_option_enabled: false,
        }
    }
}

/// Mirrors `routing::partition_endpoint_state::PartitionEndpointState`.
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct PartitionEndpointState {
    failover_overrides: HashMap<PartitionKeyRangeId, PartitionFailoverEntry>,
    circuit_breaker_overrides: HashMap<PartitionKeyRangeId, PartitionFailoverEntry>,
    per_partition_automatic_failover_enabled: bool,
    per_partition_circuit_breaker_enabled: bool,
    config: PartitionFailoverConfig,
}

impl PartitionEndpointState {
    fn new() -> Self {
        Self {
            failover_overrides: HashMap::new(),
            circuit_breaker_overrides: HashMap::new(),
            per_partition_automatic_failover_enabled: false,
            per_partition_circuit_breaker_enabled: false,
            config: PartitionFailoverConfig::default(),
        }
    }
}

// =========================================================================
// Synthetic generators
// =========================================================================

fn build_endpoints(m_regions: usize) -> Vec<CosmosEndpoint> {
    const REGION_NAMES: &[&str] = &[
        "eastus", "westus", "northeurope", "westeurope", "eastasia", "southeastasia",
        "centralus", "southcentralus", "japaneast", "australiaeast",
    ];
    (0..m_regions)
        .map(|i| {
            let name = REGION_NAMES.get(i).copied().unwrap_or("eastus");
            let url = Url::parse(&format!(
                "https://acct-{}.documents.azure.com:443/", name
            )).unwrap();
            let endpoint_key = EndpointKey(Arc::from(
                format!("acct-{}.documents.azure.com:443", name).as_str()
            ));
            CosmosEndpoint(Arc::new(CosmosEndpointData {
                region: Some(Region { normalized: Cow::Owned(name.to_string()) }),
                gateway_url: url.clone(),
                gateway20_url: None,
                endpoint_key,
            }))
        })
        .collect()
}

fn make_entry(endpoints: &[CosmosEndpoint], partition_idx: usize) -> PartitionFailoverEntry {
    let m = endpoints.len();
    let first_failed = endpoints[partition_idx % m].clone();
    let current = endpoints[(partition_idx + 1) % m].clone();

    let mut failed_endpoints = HashSet::with_capacity(m.saturating_sub(1));
    for (i, ep) in endpoints.iter().enumerate() {
        if i != (partition_idx + 1) % m {
            failed_endpoints.insert(ep.clone());
        }
    }

    let now = Instant::now();
    PartitionFailoverEntry {
        current_endpoint: current,
        first_failed_endpoint: first_failed,
        failed_endpoints,
        read_failure_count: 12,
        write_failure_count: 6,
        first_failure_time: now - Duration::from_secs(60),
        last_failure_time: now,
        health_status: HealthStatus::Unhealthy,
    }
}

fn make_pkr_id(i: usize) -> PartitionKeyRangeId {
    PartitionKeyRangeId(i.to_string())
}

fn populate_map(
    n: usize,
    endpoints: &[CosmosEndpoint],
) -> HashMap<PartitionKeyRangeId, PartitionFailoverEntry> {
    let mut m = HashMap::with_capacity(n);
    for i in 0..n {
        m.insert(make_pkr_id(i), make_entry(endpoints, i));
    }
    m
}

// =========================================================================
// Improved (proposed) types — applies 4 optimizations:
//   1. Merged map: single HashMap with Option<Entry> for failover/breaker.
//   2. SmallVec<[CosmosEndpoint; 6]> instead of HashSet<CosmosEndpoint>.
//   3. Hoisted config — not part of the CAS-swapped state.
//   4. CompactString key (inline ≤24 chars; PKR IDs are ≤5 chars).
// =========================================================================

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct PartitionFailoverEntryV2 {
    current_endpoint: CosmosEndpoint,
    first_failed_endpoint: CosmosEndpoint,
    failed_endpoints: SmallVec<[CosmosEndpoint; 6]>,
    read_failure_count: i32,
    write_failure_count: i32,
    first_failure_time: Instant,
    last_failure_time: Instant,
    health_status: HealthStatus,
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
struct PartitionOverrideV2 {
    failover: Option<PartitionFailoverEntryV2>,
    breaker:  Option<PartitionFailoverEntryV2>,
}

/// Improved partition state: single map, SmallVec for failed endpoints,
/// no embedded config, CompactString keys.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
struct PartitionEndpointStateV2 {
    overrides: HashMap<CompactString, PartitionOverrideV2>,
    per_partition_automatic_failover_enabled: bool,
    per_partition_circuit_breaker_enabled: bool,
}

fn make_entry_v2(endpoints: &[CosmosEndpoint], partition_idx: usize) -> PartitionFailoverEntryV2 {
    let m = endpoints.len();
    let first_failed = endpoints[partition_idx % m].clone();
    let current = endpoints[(partition_idx + 1) % m].clone();
    let mut failed_endpoints: SmallVec<[CosmosEndpoint; 6]> = SmallVec::new();
    for (i, ep) in endpoints.iter().enumerate() {
        if i != (partition_idx + 1) % m {
            failed_endpoints.push(ep.clone());
        }
    }
    let now = Instant::now();
    PartitionFailoverEntryV2 {
        current_endpoint: current,
        first_failed_endpoint: first_failed,
        failed_endpoints,
        read_failure_count: 12,
        write_failure_count: 6,
        first_failure_time: now - Duration::from_secs(60),
        last_failure_time: now,
        health_status: HealthStatus::Unhealthy,
    }
}

fn populate_merged_v2(
    n: usize,
    endpoints: &[CosmosEndpoint],
    failover: bool,
    breaker: bool,
) -> HashMap<CompactString, PartitionOverrideV2> {
    let mut m = HashMap::with_capacity(n);
    for i in 0..n {
        let key = CompactString::new(i.to_string());
        let entry = PartitionOverrideV2 {
            failover: if failover { Some(make_entry_v2(endpoints, i)) } else { None },
            breaker:  if breaker  { Some(make_entry_v2(endpoints, i)) } else { None },
        };
        m.insert(key, entry);
    }
    m
}

// =========================================================================
// Scenarios
// =========================================================================

#[derive(Clone, Copy, Debug)]
enum Scenario { Empty, Failover, Breaker, Both, DirectVec, FailoverV2, BreakerV2, BothV2 }

impl Scenario {
    fn label(self) -> &'static str {
        match self {
            Scenario::Empty     => "empty",
            Scenario::Failover  => "failover-only (PPAF)",
            Scenario::Breaker   => "breaker-only (PPCB)",
            Scenario::Both      => "both maps active",
            Scenario::DirectVec => "Vec<Entry> (direct, no map/Arc)",
            Scenario::FailoverV2 => "v2 failover-only (PPAF)",
            Scenario::BreakerV2  => "v2 breaker-only (PPCB)",
            Scenario::BothV2     => "v2 both (merged map)",
        }
    }
}

// =========================================================================
// Measurement
// =========================================================================

#[derive(Debug, Clone, Copy)]
struct Snapshot { curr_bytes: u64, curr_blocks: u64, max_bytes: u64 }

fn snapshot() -> Snapshot {
    let s = dhat::HeapStats::get();
    Snapshot {
        curr_bytes: s.curr_bytes as u64,
        curr_blocks: s.curr_blocks as u64,
        max_bytes:  s.max_bytes  as u64,
    }
}

#[derive(Debug)]
struct Cell {
    scenario: Scenario,
    n: usize,
    m: usize,
    retained_bytes: i64,
    retained_blocks: i64,
    peak_delta_bytes: i64,
}

fn measure<F, T>(scenario: Scenario, n: usize, m: usize, build: F) -> Cell
where
    F: FnOnce() -> T,
{
    let before = snapshot();
    let v = build();
    let after = snapshot();
    std::hint::black_box(&v);
    drop(v);
    Cell {
        scenario, n, m,
        retained_bytes:  (after.curr_bytes as i64) - (before.curr_bytes as i64),
        retained_blocks: (after.curr_blocks as i64) - (before.curr_blocks as i64),
        peak_delta_bytes: (after.max_bytes as i64) - (before.max_bytes as i64),
    }
}

fn run_cell(scenario: Scenario, n: usize, m: usize) -> Cell {
    // Build endpoints OUTSIDE the measured region — endpoint Arcs live in
    // AccountEndpointState in production, not the partition map.
    let endpoints = build_endpoints(m);
    match scenario {
        Scenario::Empty => measure(scenario, n, m, || {
            Arc::new(PartitionEndpointState::new())
        }),
        Scenario::Failover => measure(scenario, n, m, || {
            let mut s = PartitionEndpointState::new();
            s.per_partition_automatic_failover_enabled = true;
            s.failover_overrides = populate_map(n, &endpoints);
            Arc::new(Atomic::new(s))
        }),
        Scenario::Breaker => measure(scenario, n, m, || {
            let mut s = PartitionEndpointState::new();
            s.per_partition_circuit_breaker_enabled = true;
            s.circuit_breaker_overrides = populate_map(n, &endpoints);
            Arc::new(Atomic::new(s))
        }),
        Scenario::Both => measure(scenario, n, m, || {
            let mut s = PartitionEndpointState::new();
            s.per_partition_automatic_failover_enabled = true;
            s.per_partition_circuit_breaker_enabled = true;
            s.failover_overrides = populate_map(n, &endpoints);
            s.circuit_breaker_overrides = populate_map(n, &endpoints);
            Arc::new(Atomic::new(s))
        }),
        Scenario::DirectVec => measure(scenario, n, m, || {
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                v.push(make_entry(&endpoints, i));
            }
            v
        }),
        Scenario::FailoverV2 => measure(scenario, n, m, || {
            let mut s = PartitionEndpointStateV2::default();
            s.per_partition_automatic_failover_enabled = true;
            s.overrides = populate_merged_v2(n, &endpoints, true, false);
            Arc::new(Atomic::new(s))
        }),
        Scenario::BreakerV2 => measure(scenario, n, m, || {
            let mut s = PartitionEndpointStateV2::default();
            s.per_partition_circuit_breaker_enabled = true;
            s.overrides = populate_merged_v2(n, &endpoints, false, true);
            Arc::new(Atomic::new(s))
        }),
        Scenario::BothV2 => measure(scenario, n, m, || {
            let mut s = PartitionEndpointStateV2::default();
            s.per_partition_automatic_failover_enabled = true;
            s.per_partition_circuit_breaker_enabled = true;
            s.overrides = populate_merged_v2(n, &endpoints, true, true);
            Arc::new(Atomic::new(s))
        }),
    }
}

// =========================================================================
// Reporting
// =========================================================================

fn mib(b: i64) -> f64 { b as f64 / (1024.0 * 1024.0) }

fn print_results(cells: &[Cell]) {
    println!();
    println!("# Rust PPCB / PPAF partition state — memory micro-benchmark");
    println!();
    println!("Tool: dhat-rs (retained-heap delta around the build closure)");
    println!("Mirrors `PartitionEndpointState` + `PartitionFailoverEntry` + `CosmosEndpoint`.");
    println!("Endpoint Arcs are built OUTSIDE the measured region (shared via Arc).");
    println!();
    println!("| Scenario | N | M | Retained MiB | Bytes/entry | Blocks | Peak delta MiB |");
    println!("|----------|--:|--:|-------------:|------------:|-------:|---------------:|");
    for c in cells {
        let bpe = if c.n == 0 { "—".to_string() }
                  else { format!("{:.0}", c.retained_bytes as f64 / c.n as f64) };
        println!(
            "| {} | {} | {} | {:.3} | {} | {} | {:.3} |",
            c.scenario.label(), c.n, c.m,
            mib(c.retained_bytes), bpe, c.retained_blocks,
            mib(c.peak_delta_bytes),
        );
    }
    println!();
    println!("## Container overhead breakdown (Both vs 2× DirectVec at same N, M)");
    println!();
    println!("| N | M | Both maps (MiB) | 2× DirectVec (MiB) | Container overhead (MiB) | Overhead % |");
    println!("|--:|--:|----------------:|-------------------:|-------------------------:|-----------:|");
    for c in cells.iter().filter(|c| matches!(c.scenario, Scenario::Both)) {
        if let Some(d) = cells.iter().find(|d|
            matches!(d.scenario, Scenario::DirectVec) && d.n == c.n && d.m == c.m
        ) {
            let direct_double = (d.retained_bytes as f64) * 2.0;
            let overhead = (c.retained_bytes as f64) - direct_double;
            let pct = if c.retained_bytes > 0 {
                (overhead / c.retained_bytes as f64) * 100.0
            } else { 0.0 };
            println!(
                "| {} | {} | {:.3} | {:.3} | {:.3} | {:.1}% |",
                c.n, c.m,
                mib(c.retained_bytes),
                direct_double / (1024.0 * 1024.0),
                overhead / (1024.0 * 1024.0),
                pct,
            );
        }
    }
    println!();
    println!("## V2 (proposed improvements) vs baseline `both maps active`");
    println!();
    println!("Optimizations in V2:");
    println!("  1. Single merged HashMap (Option<Entry> for failover/breaker)");
    println!("  2. SmallVec<[CosmosEndpoint; 6]> in place of HashSet");
    println!("  3. Config hoisted out of CAS-swapped state");
    println!("  4. CompactString keys (inline for ≤24 chars; PKR IDs are ≤5 chars)");
    println!();
    println!("| N | M | Baseline both (MiB) | V2 both (MiB) | Saved (MiB) | Reduction % |");
    println!("|--:|--:|--------------------:|--------------:|------------:|------------:|");
    for c in cells.iter().filter(|c| matches!(c.scenario, Scenario::Both)) {
        if let Some(v2) = cells.iter().find(|d|
            matches!(d.scenario, Scenario::BothV2) && d.n == c.n && d.m == c.m
        ) {
            let saved = c.retained_bytes - v2.retained_bytes;
            let pct = if c.retained_bytes > 0 {
                (saved as f64 / c.retained_bytes as f64) * 100.0
            } else { 0.0 };
            println!(
                "| {} | {} | {:.3} | {:.3} | {:.3} | {:.1}% |",
                c.n, c.m,
                mib(c.retained_bytes),
                mib(v2.retained_bytes),
                mib(saved),
                pct,
            );
        }
    }
    println!();
    println!("## 150-driver projection (worst case partition state)");
    println!();
    println!("| N | M | Baseline × 150 (GiB) | V2 × 150 (GiB) | Saved (GiB) |");
    println!("|--:|--:|---------------------:|---------------:|------------:|");
    for c in cells.iter().filter(|c| matches!(c.scenario, Scenario::Both)) {
        if let Some(v2) = cells.iter().find(|d|
            matches!(d.scenario, Scenario::BothV2) && d.n == c.n && d.m == c.m
        ) {
            let base_gib = (c.retained_bytes as f64 * 150.0) / (1024.0 * 1024.0 * 1024.0);
            let v2_gib   = (v2.retained_bytes as f64 * 150.0) / (1024.0 * 1024.0 * 1024.0);
            println!("| {} | {} | {:.3} | {:.3} | {:.3} |", c.n, c.m, base_gib, v2_gib, base_gib - v2_gib);
        }
    }
    println!();
}

fn main() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let ns = [100usize, 1_000, 10_000, 50_000];
    let ms = [2usize, 4, 6];

    let mut cells: Vec<Cell> = Vec::new();

    eprintln!("[empty]");
    cells.push(run_cell(Scenario::Empty, 0, 0));

    for &m in &ms {
        for &n in &ns {
            eprintln!("[failover] N={} M={}", n, m);
            cells.push(run_cell(Scenario::Failover, n, m));
            eprintln!("[breaker]  N={} M={}", n, m);
            cells.push(run_cell(Scenario::Breaker,  n, m));
            eprintln!("[both]     N={} M={}", n, m);
            cells.push(run_cell(Scenario::Both,     n, m));
            eprintln!("[direct]   N={} M={}", n, m);
            cells.push(run_cell(Scenario::DirectVec, n, m));
            eprintln!("[v2-failover] N={} M={}", n, m);
            cells.push(run_cell(Scenario::FailoverV2, n, m));
            eprintln!("[v2-breaker]  N={} M={}", n, m);
            cells.push(run_cell(Scenario::BreakerV2, n, m));
            eprintln!("[v2-both]     N={} M={}", n, m);
            cells.push(run_cell(Scenario::BothV2, n, m));
            // Force epoch garbage flush between cells.
            let g = epoch::pin();
            g.flush();
            drop(g);
        }
    }

    print_results(&cells);
}

#[allow(dead_code)]
fn _keep_owned_import() { let _ = Owned::new(0u8); }
