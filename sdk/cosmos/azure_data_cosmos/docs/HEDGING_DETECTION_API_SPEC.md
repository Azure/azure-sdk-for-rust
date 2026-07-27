# Hedging Detection API — Spec

**Status:** Implemented on `main`.
**Tracking issue:** [Azure/azure-sdk-for-rust#4410](https://github.com/Azure/azure-sdk-for-rust/issues/4410)
**Cross-SDK contract:** The Azure Cosmos DB SDKs are converging on a "Hedging
Detection" capability exposed on each SDK's per-operation diagnostics surface.
This document specifies how the Rust SDK satisfies that contract.

---

## 1. Summary

The Rust Cosmos SDK surfaces hedging and per-region dispatch/response history
from a single [`DiagnosticsContext`]. This capability is realized by three
inherent accessors on the driver's `DiagnosticsContext` plus two small public
value types, all re-exported from `azure_data_cosmos`:

| Member | Signature | Semantics |
| --- | --- | --- |
| `DiagnosticsContext::hedging_started` | `-> bool` | `true` iff a hedge arm was actually dispatched (fan-out happened). |
| `DiagnosticsContext::requested_regions` | `-> Vec<RequestedRegion>` | Regions dispatched to, in **dispatch order**, duplicates allowed, each tagged with a reason. |
| `DiagnosticsContext::responded_regions` | `-> Vec<&Region>` | Regions that produced a **service reply**, in **completion order**, duplicates allowed. |
| `RequestedRegion` | `{ region, reason }` | A dispatched region paired with the reason it was chosen. |
| `RequestedRegionReason` | enum | Why the SDK dispatched to a region; `#[non_exhaustive]`. |

These build on the existing per-operation building blocks and coexist with the
Rust-native `HedgeDiagnostics` surface (see §5); the two are complementary.

---

## 2. Building blocks (already public on `main`)

| Item | Signature | Notes |
| --- | --- | --- |
| `DiagnosticsContext` | re-exported as `azure_data_cosmos::DiagnosticsContext` | The per-operation diagnostics handle. |
| `DiagnosticsContext::requests` | `-> Arc<Vec<RequestDiagnostics>>` | All dispatched attempts, in **dispatch order** (append-only). Cloning the `Arc` is a cheap atomic increment. |
| `DiagnosticsContext::hedge_diagnostics` | `-> Option<&HedgeDiagnostics>` | `Some` whenever a hedging strategy was active for the operation (including primary-wins-under-threshold). |
| `DiagnosticsContext::regions_contacted` | `-> Vec<Region>` | **Sorted and deduplicated** distinct regions — not dispatch order. |
| `RequestDiagnostics::region` | `-> Option<&Region>` | `None` for pre-region-selection failures. |
| `RequestDiagnostics::execution_context` | `-> ExecutionContext` | Why this attempt was dispatched (see §3). |
| `RequestDiagnostics::completed_at` | `-> Option<Instant>` | Set by `complete()`, `timeout()`, **and** `fail_transport()` — so "completed" alone is not "responded" (see §4.3). |
| `RequestDiagnostics::timed_out` | `-> bool` | `true` for a client-side end-to-end timeout. |
| `RequestDiagnostics::error` | `-> Option<&str>` | `Some` for a transport-level failure with no service reply. |
| `HedgeDiagnostics::primary_region` | `-> &Region` | The primary leg's region (unknown-region sentinel for global-endpoint accounts). |
| `HedgeDiagnostics::alternate_region` | `-> Option<&Region>` | `Some` exactly when the orchestrator dispatched an alternate hedge leg (fan-out happened). |
| `HedgeDiagnostics::response_region` | `-> Option<&Region>` | The single winning region, when a leg produced a final response. |
| `HedgeDiagnostics::terminal_state` | `-> HedgeTerminalState` | Authoritative race outcome. |

Because these are inherent methods on the driver's `DiagnosticsContext` and the
SDK depends on the driver (never the reverse), the diagnostics model is
driver-owned and re-exported by `azure_data_cosmos`, exactly like
`DiagnosticsContext` itself.

The hedging orchestrator/dispatch is **landed** on `main`
([#4432](https://github.com/Azure/azure-sdk-for-rust/pull/4432)): it emits
`ExecutionContext::Hedging` for alternate legs and populates `HedgeDiagnostics`
(design: [`HEDGING_SPEC.md`](../../azure_data_cosmos_driver/docs/HEDGING_SPEC.md),
[PR #4330](https://github.com/Azure/azure-sdk-for-rust/pull/4330)).

---

## 3. The `Retry → OperationRetry` rename

`ExecutionContext` is the per-request "why" returned by
`RequestDiagnostics::execution_context()`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ExecutionContext {
    Initial,
    #[deprecated(since = "0.7.0", note = "use `ExecutionContext::OperationRetry`")]
    Retry,
    OperationRetry, // was: Retry
    TransportRetry,
    Hedging,
    RegionFailover,
    CircuitBreakerProbe,
}
```

`Retry` is renamed to `OperationRetry` so the operation-level retry reason is
clearly distinct from the transport-level `TransportRetry`. The hand-written
`ExecutionContext::as_str()` and every dispatch site (`operation_pipeline.rs`,
`transport_pipeline.rs`, `cosmos_driver.rs`) are updated accordingly.

**Compatibility.** The old `Retry` variant is retained for one release as a
`#[deprecated]` alias so existing source keeps compiling. The **serialized form
changes from `"retry"` to `"operation_retry"`**: telemetry parsers that match on
the literal `"retry"` must update. (`ExecutionContext` derives `Serialize` only,
not `Deserialize`, so no `#[serde(alias)]` is needed.)

---

## 4. Detection recipes → API

### 4.1 Reason mapping — `RequestedRegionReason`

`requested_regions()` tags each dispatched region with a `RequestedRegionReason`,
projected from the driver-internal `ExecutionContext` via a **total**
`From<ExecutionContext>` mapping:

| `ExecutionContext` | `RequestedRegionReason` |
| --- | --- |
| `Initial` | `Initial` |
| `Retry` (deprecated) / `OperationRetry` | `OperationRetry` |
| `TransportRetry` | `TransportRetry` |
| `Hedging` | `Hedging` |
| `RegionFailover` | `RegionFailover` |
| `CircuitBreakerProbe` | `CircuitBreakerProbe` |

The mapping is total (no wildcard arm) so it fails to compile if a new
`ExecutionContext` variant is added without a corresponding reason.

### 4.2 Did fan-out happen? — `hedging_started()`

`true` iff at least one hedge arm was actually dispatched. This is `false` — not
an error — when the primary returns before the hedging threshold elapses, even
though a hedging strategy was active. To check whether a strategy was merely
*configured*, use `ctx.hedge_diagnostics().is_some()` (a superset that includes
primary-wins-under-threshold).

The result is the disjunction of two equivalent fan-out signals —
`HedgeDiagnostics::alternate_region().is_some()` and any request tagged
`ExecutionContext::Hedging`. Either alone is sufficient; the disjunction stays
correct if a future change ever drifts one signal.

### 4.3 Regions dispatched to, with reason — `requested_regions()`

Dispatch order, duplicates preserved (a region dispatched twice appears twice),
entries with no resolved region skipped. The initial attempt is included and
tagged `RequestedRegionReason::Initial`. This is distinct from
`regions_contacted()`, which is sorted and deduplicated.

### 4.4 Regions that responded — `responded_regions()`

A region "responded" only if a service reply actually arrived. `completed_at` is
**not** a sufficient filter: the driver also sets it for client-side timeouts
(`timeout()`) and transport failures (`fail_transport()`). The internal
`RequestDiagnostics::responded_with_service_reply()` predicate excludes those two
cases:

```rust
self.region.is_some()
    && self.completed_at.is_some()
    && !self.timed_out
    && self.error.is_none()
```

A non-2xx HTTP status (404/429/503 from the service) still counts as a response.
Results are in arrival order (stable sort by `completed_at`, preserving dispatch
order among ties); duplicates are preserved. To deduplicate, collect into a
`BTreeSet`.

---

## 5. Reconciliation with `HedgeDiagnostics`

The Hedging Detection API and the Rust-native `HedgeDiagnostics`
([PR #4330](https://github.com/Azure/azure-sdk-for-rust/pull/4330) design,
[#4432](https://github.com/Azure/azure-sdk-for-rust/pull/4432) implementation)
coexist on the same `DiagnosticsContext` and serve different audiences.

| Question | Hedging Detection API | Rust-native `HedgeDiagnostics` |
| --- | --- | --- |
| Did fan-out happen? | `hedging_started()` | `alternate_region().is_some()` — equivalent |
| Was a strategy active? | *(not derived)* | `hedge_diagnostics().is_some()` — superset of fan-out |
| Regions tried | `requested_regions()` (every attempt, with reason) | `primary_region()` + `alternate_region()` (hedge legs only) |
| Regions that responded | `responded_regions()` (full list, completion order) | `response_region()` (single winner) |
| Race outcome | *(not derived)* | `terminal_state()` (authoritative) |

`main`'s `HedgeDiagnostics` classifies the race via `terminal_state` /
`alternate_region` (there is no `total_requests_launched` counter), so "fan-out
happened" is `alternate_region().is_some()` and "the alternate won" is
`matches!(terminal_state(), HedgeTerminalState::AlternateWon)`. Consult
`terminal_state()` for hedge win-rate; do not infer an alternate win from the
presence of `alternate_region()` alone (several terminal states still record an
alternate region).

---

## 6. Future work

The three accessors return owned/borrowed collections computed on demand from
the append-only attempt list, so callers allocate only when they read a derived
collection. If `ExecutionContext` becomes a prominent part of the public
detection surface it could be renamed to something friendlier (e.g.,
`RequestPurpose` / `RequestIntent`); that rename is out of scope here.

[`DiagnosticsContext`]: https://docs.rs/azure_data_cosmos/latest/azure_data_cosmos/struct.DiagnosticsContext.html
