# Hedging Detection — Diagnostics Surface Spec

**Status:** Implemented in this PR, targeting `main`.
**Target branch:** `main`
**Tracking issue:** [Azure/azure-sdk-for-rust#4410](https://github.com/Azure/azure-sdk-for-rust/issues/4410)
**Cross-SDK contract:** The Cosmos DB SDKs are converging on a "Hedging Detection" capability exposed on each SDK's per-operation diagnostics. This document specifies how the Rust SDK satisfies that capability.

---

## 1. Summary

The Rust Cosmos SDK already exposes everything needed to detect hedging and to reconstruct per-region dispatch/response history from a single `DiagnosticsContext`. Rather than ship a parallel set of computed convenience helpers (`hedging_started()`, `requested_regions()`, `responded_regions()`) and a parallel `RequestedRegion` / `RequestedRegionReason` model, this PR keeps the v1 public surface deliberately small and lets callers read the already-public building blocks directly.

This decision follows reviewer feedback on [PR #4558](https://github.com/Azure/azure-sdk-for-rust/pull/4558): convenience helpers that "dig through the requests and create new data" force allocations a caller may not want, and lock in an API shape before we know how customers consume it. Cosmos is pre-1.0, so we prefer to be conservative now and add helpers later (e.g., when we build loggable diagnostic strings) once usage patterns are clear.

The only code change this PR makes to the diagnostics surface is renaming the internal `ExecutionContext::Retry` reason to `ExecutionContext::OperationRetry` (§3), which clarifies the per-request "why" that the detection recipes in §4 rely on.

---

## 2. Building blocks (already public)

All citations are SHA-pinned to the `main` commit at the merge of the hedging design spec ([PR #4330](https://github.com/Azure/azure-sdk-for-rust/pull/4330)), commit [`5f5d8c49d02b579a2afd2297857b919900ff1dad`](https://github.com/Azure/azure-sdk-for-rust/commit/5f5d8c49d02b579a2afd2297857b919900ff1dad), file [`diagnostics_context.rs`](https://github.com/Azure/azure-sdk-for-rust/blob/5f5d8c49d02b579a2afd2297857b919900ff1dad/sdk/cosmos/azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs). Current line numbers may have drifted; the accessors and their semantics are stable.

| Item | Signature | Notes |
| --- | --- | --- |
| `DiagnosticsContext` | re-exported as `azure_data_cosmos::DiagnosticsContext` | The per-operation diagnostics handle. |
| `DiagnosticsContext::requests()` | `-> Arc<Vec<RequestDiagnostics>>` | All dispatched attempts, in **dispatch order** (append-only). Cloning the `Arc` is a cheap atomic increment. |
| `DiagnosticsContext::hedge_diagnostics()` | `-> Option<&HedgeDiagnostics>` | `Some` whenever a hedging strategy was active for the operation (including primary-wins-under-threshold). |
| `DiagnosticsContext::regions_contacted()` | `-> Vec<Region>` | **Sorted and deduplicated** distinct regions — not dispatch order. |
| `RequestDiagnostics::region()` | `-> Option<&Region>` | `None` for pre-region-selection failures. |
| `RequestDiagnostics::execution_context()` | `-> ExecutionContext` | Why this attempt was dispatched (see §3). |
| `RequestDiagnostics::completed_at()` | `-> Option<Instant>` | Set by `complete()`, `timeout()`, **and** `fail_transport()` — so "completed" alone is not "responded" (see §4.2). |
| `RequestDiagnostics::timed_out()` | `-> bool` | `true` for a client-side end-to-end timeout. |
| `HedgeDiagnostics::alternate_region()` | `-> Option<&Region>` | `Some` exactly when the orchestrator dispatched an alternate hedge leg (i.e., fan-out happened). |
| `HedgeDiagnostics::response_region()` | `-> Option<&Region>` | The single winning region, when a leg won. |
| `HedgeDiagnostics::terminal_state()` | `-> HedgeTerminalState` | Authoritative race outcome. |

Because these are inherent methods on the driver's `DiagnosticsContext` and the SDK depends on the driver (never the reverse), the diagnostics model is driver-owned and re-exported by `azure_data_cosmos`, exactly like `DiagnosticsContext` itself.

---

## 3. The `Retry → OperationRetry` rename

`ExecutionContext` is the per-request "why" returned by `RequestDiagnostics::execution_context()`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ExecutionContext {
    Initial,
    OperationRetry,   // was: Retry
    TransportRetry,
    Hedging,
    RegionFailover,
    CircuitBreakerProbe,
}
```

This PR renames `Retry` to `OperationRetry` so the operation-level retry reason is clearly distinct from the transport-level `TransportRetry`. The hand-written `ExecutionContext::as_str()` and every dispatch site (`operation_pipeline.rs`, `transport_pipeline.rs`) are updated accordingly.

**Wire-format note.** The serialized form changes from `"retry"` to `"operation_retry"`. Cosmos is pre-1.0, so the old variant is removed outright rather than kept as a `#[deprecated]` alias — there is no enum-variant alias mechanism in Rust, and aliasing a soon-to-change wire string adds churn without value. Telemetry parsers that match on the literal `"retry"` must update. (`ExecutionContext` derives `Serialize` only, not `Deserialize`, so no `#[serde(alias)]` is needed.)

---

## 4. Detection recipes

These are the exact computations a caller runs against the building blocks in §2. They are documented here (rather than shipped as methods) so callers allocate only when they actually need a derived collection.

### 4.1 Did fan-out happen? (`hedging_started`)

`true` iff at least one hedge arm was actually dispatched. This is `false` — not an error — when the primary returns before the hedging threshold elapses, even though a hedging strategy was active.

```rust
fn hedging_started(ctx: &DiagnosticsContext) -> bool {
    ctx.hedge_diagnostics()
        .map(|hd| hd.alternate_region().is_some())
        .unwrap_or(false)
        || ctx
            .requests()
            .iter()
            .any(|r| matches!(r.execution_context(), ExecutionContext::Hedging))
}
```

The two signals — `HedgeDiagnostics::alternate_region().is_some()` and any request tagged `ExecutionContext::Hedging` — are equivalent on `main` ([#4432](https://github.com/Azure/azure-sdk-for-rust/pull/4432)). Reading either is sufficient; the disjunction stays correct if a future change ever drifts one. To check whether a hedging strategy was merely *configured* (a superset that includes primary-wins-under-threshold), use `ctx.hedge_diagnostics().is_some()`.

### 4.2 Regions dispatched to, with reason (`requested_regions`)

Dispatch order, duplicates preserved (a region dispatched twice appears twice), entries with no resolved region skipped:

```rust
let dispatched: Vec<(Region, ExecutionContext)> = ctx
    .requests()
    .iter()
    .filter_map(|r| r.region().map(|reg| (reg.clone(), r.execution_context())))
    .collect();
```

`ExecutionContext` carries the reason directly, so no separate `RequestedRegionReason` enum is needed — the request's own context *is* the reason. This is distinct from `regions_contacted()`, which is sorted and deduplicated.

### 4.3 Regions that responded (`responded_regions`)

A region "responded" only if a service reply actually arrived. `completed_at` is **not** a sufficient filter: the driver also sets it for client-side timeouts (`timeout()`) and transport failures (`fail_transport()`). The correct predicate excludes those two cases; a non-2xx HTTP status (404/429) still counts as a response:

```rust
let mut responded: Vec<&RequestDiagnostics> = ctx
    .requests()
    .iter()
    .filter(|r| {
        r.region().is_some() && r.completed_at().is_some() && !r.timed_out() && r.error().is_none()
    })
    .collect();
// Arrival order, preserving dispatch order among ties.
responded.sort_by_key(|r| r.completed_at());
let responded_regions: Vec<&Region> = responded.iter().filter_map(|r| r.region()).collect();
```

Duplicates are preserved (e.g., a late hedge loser after the winner). To deduplicate, collect into a `BTreeSet`.

---

## 5. Reconciliation with `HedgeDiagnostics`

The cross-SDK detection recipes and the Rust-native `HedgeDiagnostics` ([PR #4330](https://github.com/Azure/azure-sdk-for-rust/pull/4330) design, [#4432](https://github.com/Azure/azure-sdk-for-rust/pull/4432) implementation; authoritative shape in [`HEDGING_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/5f5d8c49d02b579a2afd2297857b919900ff1dad/sdk/cosmos/azure_data_cosmos_driver/docs/HEDGING_SPEC.md)) coexist on the same `DiagnosticsContext` and serve different audiences.

| Question | Recipe (§4) | Rust-native `HedgeDiagnostics` |
| --- | --- | --- |
| Did fan-out happen? | §4.1 | `hedge_diagnostics().map(\|hd\| hd.alternate_region().is_some()).unwrap_or(false)` — equivalent |
| Was a strategy active? | *(not derived)* | `hedge_diagnostics().is_some()` — superset of fan-out |
| Regions tried | §4.2 (every attempt, with reason) | `primary_region()` + `alternate_region()` (hedge legs only) |
| Regions that responded | §4.3 (full list, completion order) | `response_region()` (single winner) |

`main`'s `HedgeDiagnostics` classifies the race via `terminal_state` / `alternate_region` (no `total_requests_launched` counter), so "fan-out happened" is `alternate_region().is_some()` and "the alternate won" is `matches!(terminal_state(), HedgeTerminalState::AlternateWon)`.

---

## 6. Future work

If usage shows callers repeatedly hand-rolling §4, we can revisit adding first-class helpers — most naturally when building concise, loggable diagnostic-string output, where the allocation is already paid for. Any such helper should land with a clear consumer rather than speculatively. Likewise, `ExecutionContext` could be renamed to something friendlier (e.g., `RequestPurpose` / `RequestIntent`) if it becomes a prominent part of the public detection surface; that rename is out of scope for this PR.
