# Cosmos driver benchmarks

## ppcb_state_memory

Memory micro-benchmark for the per-partition failover (PPAF) and circuit
breaker (PPCB) state introduced in PR #4156.

Uses [`dhat-rs`](https://docs.rs/dhat) (Rust analog of Python `tracemalloc`)
to measure retained heap bytes around a build closure. Mirrors the production
`PartitionEndpointState` + `PartitionFailoverEntry` byte-for-byte (the real
types are `pub(crate)` and not reachable from `benches/`).

### Run

```
cargo bench --bench ppcb_state_memory
```

### Scenarios

For each (N partitions, M regions) cell the bench reports:

| Scenario | What it measures |
|---|---|
| `empty` | Fresh state, both maps empty |
| `failover-only (PPAF)` | `failover_overrides` populated only |
| `breaker-only (PPCB)` | `circuit_breaker_overrides` populated only |
| `both maps active` | Worst case — both populated |
| `Vec<Entry> (direct)` | Per-entry baseline; isolates container overhead |
| `v2 …`              | Proposed shape (see below) for the same cell |

Grid: `N ∈ {100, 1k, 10k, 50k} × M ∈ {2, 4, 6}`.

### Variant V2 — proposed shape

V2 applies four allocation-shape changes on top of the current PR #4156 design:

1. **Single merged map** — `HashMap<Key, PartitionOverride>` where
   `PartitionOverride { failover: Option<Entry>, breaker: Option<Entry> }`,
   replacing the two parallel `failover_overrides` + `circuit_breaker_overrides`
   maps. Removes duplicate keys + duplicate `HashMap` slots when both PPAF
   and PPCB act on the same partition.
2. **`SmallVec<[CosmosEndpoint; 6]>`** in place of `HashSet<CosmosEndpoint>`
   for `failed_endpoints`. Worst case is M-1 elements (≤ 5 for typical M=6),
   so the inline buffer eliminates one heap allocation per entry; lookup is
   still effectively O(1) at this size.
3. **Hoisted `PartitionFailoverConfig`** out of the CAS-swapped state. Config
   is immutable after construction, so it doesn't belong in the snapshot.
4. **`CompactString` keys** in place of `String`. PKR IDs are short
   ASCII (`"0".."49999"`, ≤ 5 chars) — `CompactString` stores ≤ 24 chars
   inline, so each key avoids a heap allocation.

### Sample results (Mac M3, dhat-rs)

Worst case (50 k partitions × 6 regions × both maps active):

| Variant | Retained MiB | Bytes/entry | Heap blocks |
|---|--:|--:|--:|
| Baseline (current PR #4156) | 25.21 | 529 | 200 004 |
| V2 (proposed)               | 17.56 | 368 |       3 |

**Reduction: 7.65 MiB (30.3%) per driver, 1.12 GiB across 150 drivers.**

The block-count delta (200 004 → 3) shows the fragmentation impact: V2
performs almost all per-entry storage inline within the `HashMap` arena
plus the SmallVec/CompactString inline buffers, instead of N separate
keys + N separate `HashSet` allocations.

### Reading the report

- `Retained MiB` — bytes retained at end of the build closure (mirrors
  `tracemalloc.get_traced_memory()` "current" reading).
- `Bytes / entry` — `Retained / N`.
- `Heap blocks` — number of distinct allocations live at the end (proxy
  for fragmentation / allocator pressure).
- `Peak delta MiB` — `dhat::HeapStats::max_bytes` delta (captures
  CAS-swap-time peak when the old + new snapshots co-exist briefly).

### Caveats

- Endpoint `Arc`s are built **outside** the measured region — in production
  they live in `AccountEndpointState` and are shared across the partition
  map, so per-entry cost is a single `Arc` pointer per slot (8 B), not the
  full `CosmosEndpointData`.
- The 5-min counter-reset window means real long-running workloads converge
  to a smaller live set than the worst-case (100 % failed) scenario.
- Numbers will differ slightly across allocators (macOS vs glibc malloc).
  The relative `V2 vs baseline` improvement is allocator-stable.
