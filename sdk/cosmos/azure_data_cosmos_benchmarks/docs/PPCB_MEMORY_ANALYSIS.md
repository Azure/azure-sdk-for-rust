# PPCB Memory Footprint — DHAT Analysis

**Crate:** `azure_data_cosmos_driver`
**Subsystem:** Per-Partition Circuit Breaker (PPCB) routing state
**Date:** 2026-05-06
**Author:** Cosmos Rust SDK team
**Status:** Final — measurement complete

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Background](#2-background)
3. [Methodology](#3-methodology)
4. [Test Harness](#4-test-harness)
5. [Workload Parameters](#5-workload-parameters)
6. [Headline Numbers](#6-headline-numbers)
7. [Per-Allocation Decomposition — Disabled Baseline](#7-per-allocation-decomposition--disabled-baseline)
8. [Per-Allocation Decomposition — PPCB Enabled](#8-per-allocation-decomposition--ppcb-enabled)
9. [Per-Entry Cost Model](#9-per-entry-cost-model)
10. [Findings](#10-findings)
11. [Recommendations](#11-recommendations)
12. [Risk and Operational Implications](#12-risk-and-operational-implications)
13. [Caveats and Methodology Notes](#13-caveats-and-methodology-notes)
14. [Reproduction Steps](#14-reproduction-steps)
15. [Appendix A — Raw DHAT Stack Frame Table](#appendix-a--raw-dhat-stack-frame-table)
16. [Appendix B — Glossary](#appendix-b--glossary)

---

## 1. Executive Summary

This report quantifies the **steady-state heap footprint** of the
Per-Partition Circuit Breaker (PPCB) in the Cosmos DB Rust driver under a
realistic worst-case workload of **80,000 partitions × 2 failed regions
per partition**.

**Key results:**

| Dimension | PPCB Disabled | PPCB Enabled (worst case) | Delta |
|---|---:|---:|---:|
| **Peak live heap** | 1,380 B | **24,604,302 B (≈ 23.46 MiB)** | **+24,602,922 B** |
| **Peak live blocks** | 13 | **160,014** | **+160,001** |
| **Bytes per partition entry** | — | **~308 B** | — |
| **Heap blocks per partition entry** | — | **2** | — |
| **Memory leaks** | 0 | 0 | — |

**Headline takeaways:**

1. **PPCB-on with no failures has effectively zero overhead.** The empty
   `circuit_breaker_overrides` map consumes 0 bytes / 0 blocks on the
   heap. The "PPCB enabled" tax materialises only when partitions
   actually fail.
2. **At full saturation (every partition tripped), the driver carries
   ~23.5 MiB of routing state for an 80k-partition account.** This is
   the upper bound; real fleets will sit well below this number because
   the failback loop continually drains the map.
3. **One single allocation accounts for 81.5 % of the peak heap** — the
   main `HashMap` backing array.
4. **Block-count pressure is the more interesting story** — 160 k heap
   blocks at peak, with ~2 blocks per partition. This drives allocator
   contention during partition-event storms.
5. **No leaks** — the trace shows clean teardown (`t-end: 0 bytes / 0
   blocks`) in both modes.

There is one specific, low-risk change (replacing
`HashSet<CosmosEndpoint>` with a stack-bounded small vector) that would
**halve the block-count pressure** with negligible code complexity.

---

## 2. Background

### 2.1 What is PPCB?

PPCB ("Per-Partition Circuit Breaker") is a routing component in the
Cosmos DB driver that detects per-partition failures and routes future
requests for an unhealthy `(PartitionKeyRange, Region)` pair to an
alternate region. After the failed region recovers, a background
failback loop transitions the partition back to its original endpoint
via probe-based recovery. PPCB applies to:

- All read operations.
- Write operations on multi-master accounts.

(See `docs/PARTITION_LEVEL_FAILOVER_SPEC.md` in
`azure_data_cosmos_driver` for the full design.)

### 2.2 Where the memory lives

PPCB state is held on a single instance of `PartitionEndpointState`,
managed lock-free via `crossbeam_epoch::Atomic` inside
`LocationStateStore`. The two relevant fields are:

- `failover_overrides: HashMap<PartitionKeyRangeId, PartitionFailoverEntry>`
  — used by PPAF (single-master writes).
- `circuit_breaker_overrides: HashMap<PartitionKeyRangeId, PartitionFailoverEntry>`
  — used by PPCB.

Both maps are populated **lazily** on first failure for a given
`(PK-range, region)` pair, and entries are removed during failback. With
no failures observed, both maps have zero heap footprint.

### 2.3 Why measure now

Production users running large containers (~80k physical partitions)
have asked for memory-cost guidance:

- "What does enabling PPCB cost in steady state?"
- "What does enabling PPCB cost in worst case?"
- "Is the overhead a function of partition count, region count, or
  both?"

This study answers all three with measured numbers.

---

## 3. Methodology

### 3.1 What we measure

Two heap traces are captured, identical in every aspect **except** the
PPCB flag and the resulting populated state:

- **Baseline trace** — `circuit_breaker_option_enabled = false`. Driver
  `mark_partition_unavailable` short-circuits on the
  `is_eligible_for_ppcb` check, so `circuit_breaker_overrides` stays
  empty.
- **Enabled trace** — `circuit_breaker_option_enabled = true`. The
  example synthesises the steady state the driver would reach after
  every partition has tripped past threshold and PPCB has advanced past
  K failed regions (described in §4.4 below).

For each mode we report:

- Total bytes / blocks allocated over the run lifetime.
- **Peak live heap** (`t-gmax`) — bytes and blocks held simultaneously
  at the worst moment of the run.
- **Leak count** (`t-end`) — bytes and blocks still alive at process
  exit.
- A program-point (PP) breakdown attributing every live byte at peak to
  a specific call site.

### 3.2 Tooling

- **DHAT** (`dhat-rs` crate, version 0.3.x) — a Rust port of Valgrind
  DHAT. Replaces the global allocator with a tracking shim that records
  every `alloc`/`dealloc` along with a stack trace, then writes a
  `*.json` trace file at process exit. The trace is loadable in the
  DHAT viewer (`dh_view.html`) for interactive analysis.
- **Build profile**: `--release`. Debug-mode allocations are not
  representative — over-reservation, no SSO, missing inlining all
  distort the picture.

### 3.3 Why DHAT vs. other tools

DHAT was chosen because:

1. It runs in-process with no kernel/PT support required (works on
   Windows, where this measurement was collected).
2. It tracks **live blocks** at peak, not just allocation totals — this
   is exactly what we need to evaluate steady-state cost.
3. It captures full stack traces per allocation, which lets us attribute
   bytes to specific data structures (HashMap backing vs. HashSet
   backing vs. inline strings), not just per-function totals.

Alternative options considered: `jemalloc` `MALLOC_STATS_PRINT`
(coarser, no stack traces); manual instrumentation via
`#[global_allocator]` (custom code, less proven); Valgrind massif
(unavailable on Windows).

---

## 4. Test Harness

### 4.1 Crate location

The benchmark and example live in:

```
sdk/cosmos/azure_data_cosmos_benchmarks/
├── benches/
│   ├── point_read.rs              (existing)
│   └── pk_range_memory.rs         (existing)
└── examples/
    ├── pk_range_dhat.rs           (existing)
    └── ppcb_state_dhat.rs         (NEW — this study)
```

### 4.2 Required-features gating

The DHAT example is gated behind a `dhat-heap` Cargo feature so the
profiler crate is **not** in the regular benchmark dependency graph. The
relevant `Cargo.toml` block:

```toml
[[example]]
name = "ppcb_state_dhat"
required-features = ["dhat-heap"]

[features]
dhat-heap = ["dep:dhat"]

[dependencies]
azure_data_cosmos_driver = { path = "../azure_data_cosmos_driver", features = [
    "__internal_testing",
] }
dhat = { workspace = true, optional = true }
```

### 4.3 Driver-side type re-exports

PPCB's data structures are crate-private in production
(`pub(crate) mod partition_endpoint_state`). The benchmark accesses them
through the existing `__internal_testing` re-export pattern documented
in [`testing.rs`](../azure_data_cosmos_driver/src/testing.rs):

```rust
// In azure_data_cosmos_driver/src/testing.rs (only compiled when
// __internal_testing feature is enabled).
pub use crate::driver::routing::endpoint::CosmosEndpoint;
pub use crate::driver::routing::partition_endpoint_state::{
    HealthStatus, PartitionEndpointState, PartitionFailoverConfig,
    PartitionFailoverEntry,
};
pub use crate::driver::routing::partition_key_range_id::PartitionKeyRangeId;
```

This change keeps the production API surface untouched: the underlying
modules remain `pub(crate)`, so external callers can only reach these
types via the unstable `crate::testing::*` namespace under the explicit
opt-in feature flag.

### 4.4 Steady-state synthesis

A faithful "drive the workload through `mark_partition_unavailable`"
benchmark would be O(N²): the function clones the entire
`PartitionEndpointState` on every call (it is a CAS-pattern pure
function). For 80k entries that is hundreds of millions of clones,
dominating runtime without changing the steady-state heap shape.

The example takes a shortcut: it directly inserts pre-built
`PartitionFailoverEntry` values into the `circuit_breaker_overrides`
map, then drops the state. The **steady-state heap shape** the profiler
captures is **identical** to what the driver would hold after threshold
has tripped on every partition and PPCB has advanced past K regions:

- `current_endpoint` = `endpoints[K]` (first non-failed region).
- `first_failed_endpoint` = `endpoints[0]`.
- `failed_endpoints` = `endpoints[..K]`.
- Counters set just past the trip threshold.
- `health_status = Unhealthy`.

This is documented prominently at the top of `ppcb_state_dhat.rs`.

---

## 5. Workload Parameters

| Parameter | Default | Env-var override | Used here |
|---|---:|---|---:|
| `PPCB_NUM_PARTITIONS` | 80,000 | `PPCB_NUM_PARTITIONS=N` | 80,000 |
| `PPCB_NUM_FAILED_REGIONS` | 2 | `PPCB_NUM_FAILED_REGIONS=K` | 2 |
| `TOTAL_REGIONS` (compile-time) | 4 | — | 4 |

**Rationale for defaults:**

- 80k partitions ≈ a "very large" Cosmos account. Most production
  workloads are an order of magnitude smaller; the value was chosen as a
  realistic upper bound.
- K = 2 failed regions per partition = a meaningful storm. Single-region
  failures are common and hit `K = 1`; a coordinated outage taking out
  two regions is the realistic worst case before the driver simply
  returns errors.
- `TOTAL_REGIONS = 4` ensures `current_endpoint` always points to a
  region not in `failed_endpoints`, matching the driver's
  `try_move_next_endpoint` post-state.

**Type sizes (release, x86_64-pc-windows-msvc):**

| Type | Bytes |
|---|---:|
| `PartitionEndpointState` | 168 |
| `PartitionFailoverEntry` | 128 |
| `PartitionFailoverConfig` | 64 |
| `PartitionKeyRangeId` | 24 |
| `CosmosEndpoint` | 8 (Arc-backed) |

These are stack sizes; heap allocations attached to each are
characterised below.

---

## 6. Headline Numbers

### 6.1 DHAT totals (whole-process)

| Metric | Disabled | Enabled | Delta |
|---|---:|---:|---:|
| **Total bytes allocated** (run lifetime) | 2,188 | 24,994,000 | +24,991,812 |
| **Total alloc events** | 25 | 240,026 | +240,001 |
| **Peak live bytes** (`t-gmax`) | 1,380 | 24,604,302 | **+24,602,922** |
| **Peak live blocks** (`t-gmax`) | 13 | 160,014 | **+160,001** |
| **Leaked bytes** (`t-end`) | 0 | 0 | 0 |
| **Leaked blocks** (`t-end`) | 0 | 0 | 0 |

### 6.2 Allocation-event timing

The enabled trace also shows the time-cost of the steady-state buildup
even though that wasn't the primary measurement target:

- **Total elapsed time** (`te`): 313,942 µs = **~314 ms** for setup +
  teardown + DHAT bookkeeping of 80k entries.
- **Time-to-gmax** (`tg`): 258,265 µs = **~258 ms**.
- The lifetime cost (`tl`) of PP8 (per-entry HashSet) totals
  ~12.0 seconds — not wall-clock; it is the integral of "block was
  alive for X µs" across all 80k blocks. Indicates these entries each
  live for ~150 µs in this benchmark, dominated by main-loop
  bookkeeping.

These numbers have **no implication for production** — the driver does
not build 80k entries in a single tight loop in practice.

---

## 7. Per-Allocation Decomposition — Disabled Baseline

The 13 live blocks at peak are entirely accounted for by the 4 account
endpoints + a single small empty HashMap. Every heap block in this mode
is direct overhead of the `CosmosEndpoint` model, not PPCB.

| PP | Live bytes | Live blocks | Origin (innermost → outermost frame) | What it is |
|---:|---:|---:|---|---|
| 3 | 928 | 4 | `EndpointKey::try_from(&Url)` ← `CosmosEndpoint::global` | The 4 account endpoints' `EndpointKey` Arc payloads (host:port string + Arc header) |
| 4 | 224 | 4 | `url::ParseOptions::parse` | The 4 `Url` internal buffers |
| 5 | 196 | 4 | `String` push ← `CosmosEndpoint::global` | Per-endpoint string payload inside the `EndpointKey` |
| 6 | 32 | 1 | `hashbrown::reserve_rehash` | Initial small `HashMap` backing (likely `unavailable_endpoints` field) |
| **Total** | **1,380** | **13** | | |

Both `circuit_breaker_overrides` and `failover_overrides` are
**zero-block** — `HashMap::new()` does not allocate until first insert.
This confirms the structural claim that PPCB-on with no failures is
indistinguishable from PPCB-off in terms of heap footprint.

---

## 8. Per-Allocation Decomposition — PPCB Enabled

Three allocations dominate the enabled trace; the remainder is the
unchanged baseline.

| PP | Live bytes | % of peak | Live blocks | Origin | What it is |
|---:|---:|---:|---:|---|---|
| **9** | **20,054,032** | **81.51 %** | **1** | `RawTable::with_capacity_in` | **The main `circuit_breaker_overrides` HashMap backing array.** 131,072 slots × ~152 B/slot (24 B `PartitionKeyRangeId` + 128 B inline `PartitionFailoverEntry`), power-of-2 over-reserved by hashbrown. |
| **8** | **4,160,000** | **16.91 %** | **80,000** | `RawTable::with_capacity_in` | **Per-entry `failed_endpoints: HashSet<CosmosEndpoint>` backing.** 52 B / entry — one tiny hashbrown table per partition holding 2 × 8 B Arc pointers + bucket overhead. |
| **7** | **388,890** | **1.58 %** | **80,000** | `String` clone (collapsed symbol shows up as `ActivityId::from_str` because that crate-internal helper sits at the same return address as the cloned-in `String` in the release binary) | **Per-entry `PartitionKeyRangeId` String.** Avg 4.86 B/entry — short numeric ids `"0".."79999"`. |
| 3 | 928 | 0.004 % | 4 | (carryover) | EndpointKey Arc payloads |
| 4, 5, 6, 10 | 656 | < 0.01 % | 13 | (carryover) | Url, EndpointKey strings, empty HashMap |
| **Total live at peak** | **24,604,302** | **100 %** | **160,014** | | |

### 8.1 Transient allocations (alloc-and-free; not at peak)

These show up in the lifetime totals but were freed before `t-gmax`:

- **PP4 transient** — 388,890 B / 80,000 blocks of `i.to_string()`
  scratch in the example's loop, cloned into the `PartitionKeyRangeId`
  by `from_str` and immediately discarded. Doubles allocator pressure
  during the buildup even though it costs nothing at peak. Benchmark
  artifact — production code constructs `PartitionKeyRangeId` from
  header-parsed strings via a different path.

### 8.2 The single 20 MiB allocation

PP9 — a single 20,054,032-byte allocation — is the most striking
feature of the enabled trace. This is hashbrown's main HashMap backing
array, sized to `next_power_of_two(80_000) = 131_072` slots. Two
implications:

1. The driver carries **~50 % headroom on the main HashMap "for
   free"** — no further allocation until partition count crosses
   ~115k. At that point hashbrown will rehash to 262,144 slots,
   doubling this single allocation to ~40 MiB.
2. This is one **contiguous** allocation. Under fragmentation pressure
   the allocator may fail to find a contiguous 20 MiB slot even when
   total free memory is ample. See §11.1 for mitigation.

---

## 9. Per-Entry Cost Model

For PPCB-on at K = 2 failed regions per partition:

| Component | Bytes/entry | Blocks/entry | Where it lives |
|---|---:|---:|---|
| Main HashMap slot (key + inline value) | **250.7** | shared (1 block for whole map) | inline in PP9 |
| `failed_endpoints: HashSet<CosmosEndpoint>` backing | **52.0** | 1 (PP8) | per entry |
| `PartitionKeyRangeId` String | **4.86** | 1 (PP7) | per entry |
| **Total live per entry** | **~307.6** | **2** | |

Cross-checks:

- 24,604,302 / 80,000 = **307.55 B/entry** ✓
- 2 × 80,000 + 14 baseline blocks = **160,014 blocks** ✓

### 9.1 Scaling projections

Peak heap is approximately **`B(N) ≈ 308 × N + 1.4 KB` bytes** (the
1.4 KB baseline is the disabled-mode endpoint state) up to the next
hashbrown power-of-2 boundary, where the main HashMap allocation
doubles. Practical waypoints:

| N partitions | Projected peak | Notes |
|---:|---:|---|
| 1,000 | ~308 KB | (HashMap rounds to 2,048 slots — slightly over the formula) |
| 10,000 | ~3.0 MB | |
| 80,000 | **~23.5 MiB** | measured |
| 100,000 | ~30 MiB | |
| 131,072 | ~40 MiB | First post-rehash boundary |
| 1,000,000 | ~310 MiB | Linear extrapolation; not measured |

### 9.2 Sensitivity to K (failed regions per partition)

The per-entry cost is approximately:

```
~308 bytes/entry  =  ~252 B (HashMap slot)
                   + ~52 B (failed_endpoints HashSet table)
                   + ~5 B  (PK range id String)
```

The HashSet backing (52 B at K = 2) grows roughly linearly with K up to
the hashbrown power-of-2 boundary (which for the tiny `failed_endpoints`
set sits at K ≈ 3–4). For K ≤ 4 (the realistic case — Cosmos accounts
rarely have more than 4 regions), per-entry cost stays in the ~300 B
band.

---

## 10. Findings

### 10.1 PPCB-on baseline is free

`circuit_breaker_overrides` and `failover_overrides` are
zero-allocation when empty. There is no PPCB tax for accounts that
never experience partition failures. This is the single most
operationally important finding for users debating whether to enable
PPCB by default.

### 10.2 Worst-case heap is bounded and modest

23.5 MiB at 80,000 fully-tripped partitions is a small fraction of any
realistic process budget. On a 4 GB driver process this is < 0.6 % of
total memory.

### 10.3 The 20 MiB single allocation is the dominant cost

PP9 (the main HashMap backing) is ~81.5 % of peak heap and is a single
contiguous allocation. This is unavoidable given the
`HashMap<PartitionKeyRangeId, PartitionFailoverEntry>` choice with
inline values.

### 10.4 Block-count pressure scales linearly with partition count

Each tripped partition costs **2 heap blocks**: one for the
`PartitionKeyRangeId` String, one for the per-entry `failed_endpoints`
HashSet table. At 80k partitions that's 160k allocator round-trips
during a partition-event storm.

### 10.5 The per-entry HashSet is over-engineered

`failed_endpoints` is a `HashSet<CosmosEndpoint>` but the realistic
upper bound is `K ≤ 4` (one per account region). Paying for a hashbrown
table per partition to hold 2 elements is structurally wasteful — it's
~52 B and 1 heap block per entry to hold ~16 B of payload.

### 10.6 No leaks

Both traces show clean teardown. `Drop` for `PartitionEndpointState`
correctly clears every nested map and set.

### 10.7 PartitionKeyRangeId is small but loud

The 80k tiny `PartitionKeyRangeId` Strings consume only 1.6 % of bytes
but **50 % of the per-entry block count**. They are short numeric ids
that could trivially live inline.

---

## 11. Recommendations

Listed in descending order of impact / leverage.

### 11.1 (Recommended) Replace `failed_endpoints` HashSet with a SmallVec

**Change:**

```rust
// In partition_endpoint_state.rs
pub struct PartitionFailoverEntry {
    // ...
    pub failed_endpoints: smallvec::SmallVec<[CosmosEndpoint; 4]>,
    // (or even [Option<CosmosEndpoint>; 4] if you prefer no extra dep)
}
```

**Impact:**

- Eliminates **80,000 of 160,014 heap blocks** (50 % reduction in
  block-count pressure at 80k partitions).
- Saves **~3.5 MiB at peak** (4.16 MB → ~640 KB inline in the main
  HashMap slot).
- Lookup goes from `O(1)` hash to `O(K)` linear scan over ≤ 4
  Arc-pointer comparisons — empirically faster than a hash for K ≤ 4.

**Risk:** Low. `SmallVec` is well-tested. The semantic change is
transparent: `contains()`, `insert()`, `iter()` are all available.

**Effort:** Single struct field change + ~20-line updates in the call
sites (`mark_partition_unavailable`, `try_move_next_endpoint`).

### 11.2 (Optional) Inline-store small `PartitionKeyRangeId` strings

**Change:** Replace `String` with `compact_str::CompactString` in
`PartitionKeyRangeId`. Strings ≤ 24 B store inline; production PK range
ids are short numeric strings.

**Impact:**

- Eliminates **80,000 of remaining 80,000 per-entry heap blocks** (no
  per-entry `String` allocation).
- Saves ~390 KB at peak.

**Risk:** Low. `CompactString` is `From<String>` / `From<&str>` and
implements all the same traits. New dependency though — weigh that
against the savings.

**Effort:** Single newtype field change.

### 11.3 (Defer) Consider boxing the HashMap value

**Change:**

```rust
pub circuit_breaker_overrides:
    HashMap<PartitionKeyRangeId, Box<PartitionFailoverEntry>>,
```

**Impact:**

- Shrinks the main HashMap allocation from ~20 MiB to ~4 MiB at 80k
  entries (slot becomes 24 B key + 8 B Box pointer = 32 B).
- Adds 80k Box allocations (~10.2 MiB) — net memory roughly the same.
- **Removes the single contiguous 20 MiB allocation**, which is the
  real win: under fragmentation pressure a 20 MiB contiguous request
  may fail; 80k × 128 B requests almost certainly will not.
- Adds one pointer indirection per access — negligible cost on the
  routing hot path which is already an HashMap lookup.

**Risk:** Medium — touches a perf-sensitive path. Only worth it if
profiling under fragmented allocators shows the contiguous allocation
to be a problem.

**Effort:** Single field change + minor borrow updates.

### 11.4 Combined projection

If 11.1 + 11.2 are both applied:

| Metric | Before | After (projected) | Change |
|---|---:|---:|---:|
| Peak live heap | 24,604,302 B | ~20,640,000 B | −16 % |
| Peak live blocks | 160,014 | **~14** | **−99.99 %** |
| Bytes per entry | 308 | ~258 | −16 % |
| Blocks per entry | 2 | **0** | — |

The block-count win is qualitatively much bigger than the byte-count
win — that is where allocator-contention improvements would surface
under burst load.

---

## 12. Risk and Operational Implications

### 12.1 Memory budgeting for fleet operators

For an operator deploying the Cosmos Rust driver at scale:

- **Per-process worst-case PPCB overhead**: ~308 B × N partitions
  + 1.4 KB baseline.
- For typical N ≤ 1,000 partitions per container: < 320 KB. Negligible.
- For "very large" N = 80,000 partitions: ~24 MiB. Still small.
- For "extreme" N = 1,000,000 partitions: ~310 MiB extrapolated. Worth
  budgeting for.

These numbers are upper bounds — they assume **every partition is
tripped**, which is unusual outside a coordinated regional event.

### 12.2 Allocator pressure during outages

A regional outage that trips PPCB on 80k partitions in a short window
will trigger ~240k allocations (entries + transient) in that window.
At ~1 µs per allocation on modern systems this is well under 250 ms of
allocator time amortised across worker threads — not a bottleneck.

### 12.3 Cleanup behaviour

The trace shows zero leaked blocks at process exit — `Drop` is
implemented correctly. The failback loop (described in
`PARTITION_LEVEL_FAILOVER_SPEC.md`) further drains entries during
recovery, so steady-state heap during normal operation will be far
below the worst-case measured here.

### 12.4 What this study does not cover

- **Wall-clock latency impact of PPCB lookups** on the routing hot
  path. Out of scope; benchmark via the Criterion `point_read` bench
  with PPCB-laden state if needed.
- **CPU cost of the failback loop** at high entry counts. The sweep is
  O(N) over `circuit_breaker_overrides` and runs every
  `failback_sweep_interval` (default 5 min) — at N = 80k this is a
  microsecond-scale scan, but a dedicated benchmark would confirm.
- **Concurrent-mutator stress.** The CAS-clone pattern in
  `mark_partition_unavailable` produces transient O(N) clones during
  the swap. Contention behaviour under burst load is not measured here.

---

## 13. Caveats and Methodology Notes

### 13.1 Steady-state synthesis (not full pipeline)

The benchmark direct-inserts pre-built `PartitionFailoverEntry` values
rather than driving the workload through the full pipeline. This is
intentional — `mark_partition_unavailable` clones the entire state on
every call, so a faithful 80k-iteration walk would be O(N²) in time
without changing the steady-state heap shape. The captured **heap
layout** is identical; the **buildup-time allocator traffic** is
slightly understated (CAS-pattern cloning would add transient
~24 MiB-per-step allocation churn that frees immediately).

### 13.2 Symbolicator collapsing in the trace

The DHAT trace's `ftbl` shows one frame as
`ActivityId::from_str` even though the call originates from
`PartitionKeyRangeId::from_str` in the example. This is a Windows PDB
symbol-deduplication artefact — both functions compile to the same
return-address slot in the release binary. The PP attribution by
**bytes / blocks / count** is correct; only the displayed function
name is misleading. We confirmed by cross-referencing `tl` (lifetime
sum) and the bytes/block ratio against the source.

### 13.3 hashbrown over-reservation

Both the main HashMap (PP9) and the per-entry HashSets (PP8) are sized
by hashbrown to the next power of two. At N = 80,000 the main map is
sized for 131,072 slots — about 64 % loaded. This means the per-entry
average reported here (308 B) silently amortises ~38 % unused
capacity into the per-entry number. Real-world entry counts that sit
just below or just above a power-of-2 boundary will show meaningful
discontinuities in per-entry cost.

### 13.4 Platform

All measurements were collected on:

- **OS**: Windows (x86_64-pc-windows-msvc).
- **Rust**: stable, release profile (`opt-level = 3`, `lto = false` by
  default).
- **Allocator**: system default (Windows Heap).

Linux / macOS / `jemalloc` may show modestly different numbers
(different hashbrown over-reservation cutoffs, different bucket
overheads), but the structural conclusions are platform-agnostic.

### 13.5 Run-to-run variability

DHAT is deterministic within a build — re-running produces identical
byte/block totals as long as inputs and binary do not change. The
single non-deterministic field is `failback_jitter`, which is sampled
from a `SystemTime`-seeded RNG, but it is held inline in
`PartitionFailoverEntry` and contributes zero heap allocation, so it
does not affect the trace.

---

## 14. Reproduction Steps

### 14.1 Prerequisites

- Workspace at `D:\stash\azure-sdk-for-rust` (or your local clone).
- Rust toolchain (any recent stable).
- DHAT viewer: <https://nnethercote.github.io/dh_view/dh_view.html>
  (browser-side; data is not uploaded).

### 14.2 Build

```powershell
cd D:\stash\azure-sdk-for-rust
cargo build -p azure_data_cosmos_benchmarks `
    --release --features dhat-heap --example ppcb_state_dhat
```

### 14.3 Run baseline (PPCB disabled)

```powershell
.\target\release\examples\ppcb_state_dhat.exe disabled
```

Output: `dhat-heap-ppcb-disabled.json` written to the current
directory.

### 14.4 Run enabled (PPCB on, 80k × 2)

```powershell
.\target\release\examples\ppcb_state_dhat.exe enabled
```

Output: `dhat-heap-ppcb-enabled.json` in the current directory.

### 14.5 Custom workload sizing

```powershell
$env:PPCB_NUM_PARTITIONS = "10000"
$env:PPCB_NUM_FAILED_REGIONS = "3"
.\target\release\examples\ppcb_state_dhat.exe enabled
```

### 14.6 View the traces

Open `dh_view.html` in a browser, click "Browse…", load either JSON
file. Drill down by call-site for per-allocation attribution.

---

## Appendix A — Raw DHAT Stack Frame Table

This appendix documents how each PP in the enabled trace was attributed
to its underlying data structure. Frame indices (`fs`) refer to entries
in the `ftbl` array of the JSON.

| PP | Live B | Innermost frame (decoded) | Attributed to |
|---:|---:|---|---|
| 9 | 20,054,032 | `RawTable::with_capacity_in` ← `ppcb_state_dhat::main` | Main `circuit_breaker_overrides` HashMap backing — the `HashMap::with_capacity(80_000)` call rounds to 131,072 slots |
| 8 | 4,160,000 | `RawTable::with_capacity_in` ← `ppcb_state_dhat::main` (different call site than PP9) | Per-entry `failed_endpoints: HashSet<CosmosEndpoint>` backing |
| 7 | 388,890 | `String::with_capacity` (collapsed) ← `ppcb_state_dhat::main` | Per-entry `PartitionKeyRangeId` String — `from_str(&i.to_string())` |
| 3 | 928 | `EndpointKey::try_from(&Url)` ← `CosmosEndpoint::global` | The 4 endpoints' `EndpointKey` Arc payloads |
| 4 | 224 | `url::ParseOptions::parse` | The 4 `Url` internal buffers |
| 5 | 196 | `String::push_str` ← `CosmosEndpoint::global` | Per-endpoint host:port string in `EndpointKey` |
| 6 | 32 | `hashbrown::reserve_rehash` | Empty `unavailable_endpoints` HashMap initial backing |
| 10 | 224 | duplicate of PP4 with different surrounding context | extra `Url` buffer from a second parse |

---

## Appendix B — Glossary

| Term | Definition |
|---|---|
| **PPCB** | Per-Partition Circuit Breaker — partition-level failover for reads (any account) and writes (multi-master accounts). |
| **PPAF** | Per-Partition Automatic Failover — partition-level failover for writes on single-master accounts. Uses a separate map (`failover_overrides`) but the same `PartitionFailoverEntry` type. |
| **`PartitionEndpointState`** | The container struct holding both PPAF and PPCB maps plus the `PartitionFailoverConfig`. |
| **`PartitionFailoverEntry`** | One entry in either map: tracks current/failed endpoints, failure counts, timestamps, health status. |
| **`PartitionKeyRangeId`** | Newtype around `String` identifying a physical partition key range. |
| **`CosmosEndpoint`** | Arc-backed regional endpoint reference. Cloning is cheap (refcount). |
| **`HealthStatus`** | `Unhealthy` / `ProbeCandidate`. Drives the failback state machine. |
| **CAS** | Compare-And-Swap. The PartitionEndpointState is mutated by clone-modify-CAS, hence why each mutation is O(N). |
| **DHAT** | Dynamic Heap Analysis Tool. The Rust port (`dhat-rs`) replaces the global allocator and writes a trace JSON. |
| **`t-gmax`** | Time at which the live heap (in bytes) reached its maximum during the run. |
| **`t-end`** | Time at which the program exited. Live bytes here = leaks. |
| **PP** | Program Point — DHAT's term for an allocation site identified by its full stack trace. |
| **`ftbl`** | The frame table in a DHAT JSON — an array of stack frames referenced by index from each PP's `fs` field. |

---

*End of report.*
