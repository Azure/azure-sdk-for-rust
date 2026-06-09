# Hedging Detection API — Spec

**Status:** Draft (spec-only; no production code change in this PR)
**Target branch:** `main` (the `release/azure_data_cosmos-previews` preview branch has since merged into `main`)
**Tracking issue:** [Azure/azure-sdk-for-rust#4410](https://github.com/Azure/azure-sdk-for-rust/issues/4410)
**Cross-SDK contract:** The cross-SDK Hedging Detection API (a per-operation diagnostics surface shared in shape across the Cosmos DB SDKs). This Rust spec adopts that contract; the type/method names below are the contract's canonical names rendered in idiomatic Rust.
**Sequencing:** Lands AFTER [PR #4330](https://github.com/Azure/azure-sdk-for-rust/pull/4330) (now **merged**, via `release/azure_data_cosmos-previews`, into `main`) and BEFORE any hedging-implementation PR.

---

## 1. Introduction — cross-reference to PR #4330 and the cross-SDK surface

The Azure Cosmos DB SDKs are converging on a cross-SDK Hedging Detection API exposed on each SDK's per-operation diagnostics surface. The contract defines the following members (names canonicalized; each SDK renders them idiomatically):

| Contract member | Semantics | Shape |
|---|---|---|
| `hedging_started()` | `true` iff a hedge arm was actually dispatched | boolean |
| `requested_regions()` | regions the SDK dispatched to | dispatch order, duplicates allowed, includes initial attempt; each entry tagged with a region-reason |
| `responded_regions()` | regions that responded | arrival order, duplicates allowed; **region identifiers only**, no reason |
| `RequestedRegion` | a dispatched region paired with its reason | `{ region, reason }` value type |
| `RequestedRegionReason` | why the SDK chose a region for a dispatch | enumeration: `Initial`, `OperationRetry`, `TransportRetry`, `Hedging`, `RegionFailover`, `CircuitBreakerProbe`; non-exhaustive |

This document specifies the Rust SDK's adoption of that contract on `azure_data_cosmos::CosmosDiagnosticsContext` (re-export of `azure_data_cosmos_driver::diagnostics::DiagnosticsContext`).

This spec is **sequenced** with respect to PR #4330 ("Cosmos: Adds Cross-Region Hedging Design Spec to Driver Crate"), which has merged (via `release/azure_data_cosmos-previews`) into `main`:

| Phase                                                | Artifact                                          | Status                  |
|------------------------------------------------------|---------------------------------------------------|-------------------------|
| Hedging Design Spec                                  | `sdk/cosmos/azure_data_cosmos_driver/docs/HEDGING_SPEC.md` (PR #4330) | **Merged** into `main` |
| **Hedging Detection API spec (this document)**       | `sdk/cosmos/azure_data_cosmos/docs/HEDGING_DETECTION_API_SPEC.md`     | **This PR, draft**      |
| Hedging implementation (orchestrator + dispatch)     | Separate future PR — emits `ExecutionContext::Hedging`, populates `HedgeDiagnostics` | Future                  |
| Hedging Detection API implementation (this spec → code) | Separate future PR — adds the three accessors + the public `RequestedRegion` / `RequestedRegionReason` types specified below | Future, after the above |

This PR adds **exactly one Markdown file** and changes no production code, matching PR #4330's doc-only pattern.

The Hedging Detection API is **additive** on `DiagnosticsContext` and **complementary** to (not a replacement for) PR #4330's `DiagnosticsContext::hedge_diagnostics: Option<HedgeDiagnostics>` field. The two surfaces are explicitly reconciled in §6 below.

---

## 2. Verified existing surface inventory

Citations are to the `main` branch at the merge of PR #4330 (commit `5f5d8c49d`, "Cosmos: Adds Cross-Region Hedging Design Spec to Driver Crate (#4330)").

| Item                               | Location                                                                                                  | Visibility                                            |
|------------------------------------|-----------------------------------------------------------------------------------------------------------|-------------------------------------------------------|
| `DiagnosticsContext`               | `sdk/cosmos/azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs`                              | `pub` in driver; re-exported as `azure_data_cosmos::CosmosDiagnosticsContext` (`sdk/cosmos/azure_data_cosmos/src/models/mod.rs:37`) |
| `RequestDiagnostics`               | `sdk/cosmos/azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs`                              | `pub` in driver; **NOT** re-exported from `azure_data_cosmos` |
| `ExecutionContext`                 | `sdk/cosmos/azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs:35`                           | `pub` in driver; **NOT** re-exported                  |
| `ExecutionContext` derives         | `#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]`, `#[serde(rename_all = "snake_case")]`, `#[non_exhaustive]` | `Serialize` only — **no `Deserialize`** is derived today (relevant to §4) |
| `ExecutionContext` variants today  | `Initial`, `Retry`, `TransportRetry`, `Hedging`, `RegionFailover`, `CircuitBreakerProbe` | Declaration order as listed; `Hedging` and `CircuitBreakerProbe` are reserved (see below) |
| `ExecutionContext::as_str`         | `diagnostics_context.rs:56` — hand-written match returning `"initial"`, `"retry"`, `"transport_retry"`, `"hedging"`, `"region_failover"`, `"circuit_breaker_probe"` | The canonical string/wire form is produced by **both** `as_str()` and the serde derive |
| `DiagnosticsContext::regions_contacted()` | `diagnostics_context.rs:1580`                                                                       | `pub fn`; returns `Vec<Region>` **sorted and deduplicated** (distinct regions, not dispatch order) |
| `RequestDiagnostics::region()`     | `diagnostics_context.rs` — backing field `region: Option<Region>`                                         | `pub fn region(&self) -> Option<&Region>`             |
| `RequestDiagnostics::completed_at` | `diagnostics_context.rs:417` — `completed_at: Option<Instant>`                                            | `pub(crate)`; set when a response arrives              |
| `RequestDiagnostics::execution_context()` | `diagnostics_context.rs`                                                                           | `pub fn execution_context(&self) -> ExecutionContext` |
| Dispatch sites that set `ExecutionContext` | `sdk/cosmos/azure_data_cosmos_driver/src/driver/pipeline/operation_pipeline.rs` (e.g. lines 938–942, 1217–1348, 2772–2799); `src/driver/transport/transport_pipeline.rs` lines 190, 194 | Only `Initial`, `Retry`, `TransportRetry`, `RegionFailover` are emitted today; `Hedging` is **not** dispatched. |

**Key facts** (carried forward into §3 and §10):

- `ExecutionContext::Hedging` is a **reserved** variant; no production code path emits it today.
- `RequestDiagnostics` and `ExecutionContext` are driver-private from the SDK consumer's perspective.
- `regions_contacted()` is **distinct + sorted**, so it cannot supply the **dispatch-order, duplicates-allowed** semantics the cross-SDK contract requires for `requested_regions()`/`responded_regions()`. New accessors are therefore needed (they are not thin wrappers over `regions_contacted()`).
- `ExecutionContext` derives `Serialize` but **not** `Deserialize`. The cross-SDK design doc §10 implies a `#[serde(alias = "retry")]` back-deserialization affordance, which is only meaningful if/when `Deserialize` is added (see §4.1).

---

## 3. Proposed additive methods on `DiagnosticsContext`

All three accessors are added to `DiagnosticsContext` in `azure_data_cosmos_driver` and exposed on `CosmosDiagnosticsContext` via the existing re-export (`sdk/cosmos/azure_data_cosmos/src/models/mod.rs:37`). The reason-carrying return of `requested_regions()` (§3.1) requires two **new public types** in `azure_data_cosmos` — `RequestedRegion` and `RequestedRegionReason` — to realize the cross-SDK contract's per-region-reason shape (see §3.5 and §5).

### 3.1 `pub fn requested_regions(&self) -> Vec<RequestedRegion>`

```rust
/// Returns the regions to which this operation dispatched a request,
/// in dispatch order, each tagged with the reason the SDK chose it.
///
/// Each dispatched attempt contributes one entry. Duplicates are allowed:
/// the same region may appear more than once if it was dispatched
/// multiple times (e.g., a retry to the same region, or a hedge request
/// to a region that was also the primary). The initial attempt is
/// included and tagged `RequestedRegionReason::Initial`.
///
/// Entries with no resolved region (pre-region-selection failures) are
/// skipped.
///
/// Order matches `RequestDiagnostics` insertion order in
/// `self.requests`, which is dispatch order.
pub fn requested_regions(&self) -> Vec<RequestedRegion> { ... }
```

- **Source:** `self.requests.iter()` — each `RequestDiagnostics` carries `region: Option<Region>` and `execution_context() -> ExecutionContext`.
- **Filter:** include only entries with `Some(region)`.
- **Map:** `region.clone()` → `RequestedRegion.region`; `execution_context()` → `RequestedRegion.reason` via the `From<ExecutionContext>` mapping in §3.5.
- **Dedup:** **none**. Duplicates are allowed (cross-SDK contract).
- **Order:** source order (= dispatch order, since the `requests` collection is append-only).
- **Cost:** one `Vec<RequestedRegion>` allocation; each entry clones one `Region` (cheap) plus a `Copy` enum.

> **Shape change vs. the original draft.** The original draft returned `Vec<&Region>` (no reason). Because the cross-SDK contract exposes per-region **reason** via a `RequestedRegion { region, reason }` pair, this spec adopts the reason-carrying shape so the Rust API conforms to the contract. This resolves the previously-open question about deferring per-region reason (§10).

### 3.2 `pub fn responded_regions(&self) -> Vec<&Region>`

```rust
/// Returns the regions from which this operation received a response,
/// in arrival (completion) order.
///
/// Each completed `RequestDiagnostics` contributes one entry. Duplicates
/// are allowed: the same region may appear more than once if multiple
/// completed responses arrived from it (e.g., a late hedge response
/// after the hedge winner). `responded_regions().len() > 1` does NOT
/// imply more than one distinct region responded.
///
/// To dedupe, callers can collect into a set, for example:
/// `let unique: Vec<&Region> = ctx.responded_regions().into_iter().collect::<std::collections::BTreeSet<_>>().into_iter().collect();`
pub fn responded_regions(&self) -> Vec<&Region> { ... }
```

- **Shape:** region references only — **no reason**. This follows the cross-SDK contract's `responded_regions()`, which returns region identifiers only. No new type is required.
- **Source:** `self.requests.iter()`.
- **Filter:** include only entries with `Some(region)` AND `completed_at.is_some()` (a response actually arrived).
- **Dedup:** **none**.
- **Order:** **completion order**. The current `requests` collection is in *dispatch* order; producing completion order requires one of:
  - **(a)** the orchestrator appending in completion order (changes the existing append discipline — rejected because it would harm `requested_regions()` semantics);
  - **(b)** a stable sort by `completed_at` at accessor time (O(n log n) for a typically-small n);
  - **(c)** a separate `Vec<usize>` of indices into `requests` maintained in completion order.

  This spec **proposes (c)** — a small driver-private completion-order index — and lists this as Open Question (iii) in §10.

### 3.3 `pub fn hedging_started(&self) -> bool`

```rust
/// Returns `true` iff this operation actually dispatched at least one
/// hedge request (i.e., fan-out occurred), and `false` otherwise.
///
/// `false` does NOT mean hedging was disabled or misconfigured; it
/// means no fan-out occurred. In particular, when the primary returns
/// before the hedging threshold elapses, this returns `false` even
/// though a hedging strategy was active.
///
/// To check whether a hedging strategy was *configured*, inspect
/// `hedge_diagnostics.as_ref().map(|hd| &hd.strategy_config)`
/// (see `HedgeDiagnostics` from the Cross-Region Hedging design spec;
/// `strategy_config` is a non-optional field on `HedgeDiagnostics`,
/// so its presence is gated only by whether `hedge_diagnostics` itself
/// is `Some`).
pub fn hedging_started(&self) -> bool { ... }
```

This matches the cross-SDK contract's `hedging_started()`: `true` iff at least one hedge arm was actually dispatched; `false` for the primary-wins-under-threshold case.

**Definition (post-hedging-implementation PR):**

```rust
fn hedging_started(&self) -> bool {
    self
        .hedge_diagnostics
        .as_ref()
        .map(|hd| hd.total_requests_launched >= 2)
        .unwrap_or(false)
    || self
        .requests
        .iter()
        .any(|r| matches!(r.execution_context(), ExecutionContext::Hedging))
}
```

**Why the disjunction (and not either predicate alone) — SE-023:**

- **Why NOT `hedge_diagnostics.is_some()` alone.** PR #4330's `HedgeDiagnostics` is `Some` whenever a hedging strategy is *active for this operation*, including the **primary-wins-under-threshold** case — exactly one request launched (`total_requests_launched == 1`), threshold delay never elapsed, no fan-out. The cross-SDK contract requires `hedging_started() == true` iff the SDK **actually dispatched to a hedge region**, i.e., `total_requests_launched >= 2` — not "strategy was active". Returning `true` for `total_requests_launched == 1` would contradict the cross-SDK contract.
- **Why NOT the `ExecutionContext::Hedging` predicate alone.** A future internal refactor to `HedgeDiagnostics` (e.g., a field rename, or a change of semantics around `total_requests_launched`) could silently cause the two predicates to drift out of sync. The disjunction is the **safe** definition: it returns `true` whenever *either* signal indicates fan-out occurred, and the mandatory invariant test in §8 enforces that the two signals stay equivalent after the implementation PR lands.
- **Pre-implementation behavior.** Until the hedging implementation PR lands (the follow-up to PR #4330), no production code populates `hedge_diagnostics` and no production code emits `ExecutionContext::Hedging`. Therefore `hedging_started()` returns `false` for all operations — which is the correct answer: no fan-out has happened.

### 3.4 Pre-region-selection failures

For all three accessors, a `RequestDiagnostics` whose `region` is `None` (pre-region-selection failure) is **skipped**. This matches `regions_contacted()` precedent and the cross-SDK contract. `requested_regions()` therefore returns an empty `Vec` when an operation failed before any region was selected, matching the contract's "empty when there is no dispatch history" rule.

### 3.5 New public types — `RequestedRegion` and `RequestedRegionReason`

These realize the cross-SDK contract's `RequestedRegion` value type and `RequestedRegionReason` enumeration (§1), in idiomatic Rust. They live in `azure_data_cosmos` (the public SDK crate), not the driver — see §5.

```rust
/// A single region the SDK dispatched a request to, tagged with the
/// reason the orchestrator chose to send it.
///
/// Realizes the cross-SDK contract's `RequestedRegion`. Region equality is
/// delegated to `Region`'s own `PartialEq`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct RequestedRegion {
    /// The region the SDK dispatched to.
    pub region: Region,
    /// The reason the SDK chose this region for this dispatch attempt.
    pub reason: RequestedRegionReason,
}

/// Reason the SDK chose to dispatch a request to a particular region.
///
/// Realizes the cross-SDK contract's `RequestedRegionReason`. The enum is `#[non_exhaustive]`;
/// callers that `match` on it MUST include a wildcard arm.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RequestedRegionReason {
    /// The first dispatch of the operation.
    Initial,
    /// An operation-level retry decided by the SDK's client-retry policy
    /// (maps from the driver's `ExecutionContext::Retry`, renamed per §4).
    OperationRetry,
    /// A transport-level retry inside the per-region transport stack.
    /// Reserved; populated only once transport-retry attempts surface a
    /// resolved region in `RequestDiagnostics`.
    TransportRetry,
    /// A speculative cross-region hedge fan-out dispatch.
    /// Reserved until the hedging implementation PR lands.
    Hedging,
    /// An endpoint-failure-driven retry to a different region.
    RegionFailover,
    /// A probe dispatch to a previously circuit-broken region (PPCB).
    /// Reserved; PPCB probes do not currently tag a distinct context.
    CircuitBreakerProbe,
}

impl From<ExecutionContext> for RequestedRegionReason {
    fn from(ctx: ExecutionContext) -> Self {
        match ctx {
            ExecutionContext::Initial => RequestedRegionReason::Initial,
            // After the §4 rename this arm is `ExecutionContext::OperationRetry`.
            ExecutionContext::Retry => RequestedRegionReason::OperationRetry,
            ExecutionContext::TransportRetry => RequestedRegionReason::TransportRetry,
            ExecutionContext::Hedging => RequestedRegionReason::Hedging,
            ExecutionContext::RegionFailover => RequestedRegionReason::RegionFailover,
            ExecutionContext::CircuitBreakerProbe => RequestedRegionReason::CircuitBreakerProbe,
        }
    }
}
```

**Rust-specific choices, by design:**

- **No `Unknown` sentinel.** Some SDKs in the cross-SDK contract carry an `Unknown = 0` value because their `RequestedRegion` value type has a reachable zero/default form. In Rust, `RequestedRegion` is only ever constructed by the SDK from a real `ExecutionContext`, and the type derives no `Default`, so there is no zero-value sentinel to represent. Omitting `Unknown` keeps the Rust enum total over the reasons the SDK can actually emit. (If a future need for `Default` arises, an `Unknown` variant can be added without breaking `#[non_exhaustive]` consumers.)
- **`TransportRetry` and `CircuitBreakerProbe` are reserved** — present in the public enum for cross-SDK conformance, but not necessarily populated in the first implementation. The `From` mapping is total so they light up automatically if/when the driver tags those contexts with a resolved region.

---

## 4. The `Retry → OperationRetry` rename mechanism

The cross-SDK contract renames the legacy `Retry` reason to `OperationRetry` for clarity (it distinguishes user-visible operation retries from transport-layer retries). The Rust public `RequestedRegionReason` (§3.5) uses the contract's `OperationRetry` name. For internal consistency the driver-private `ExecutionContext::Retry` is renamed to match.

**Rust mechanics** (the cross-SDK design doc §10 proposes a `#[deprecated]` re-export, which is **not a real Rust mechanism** — Rust has no enum-variant aliases; refuted here):

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ExecutionContext {
    Initial,
    #[deprecated(since = "<version>", note = "use `OperationRetry`")]
    Retry,
    OperationRetry,
    TransportRetry,
    Hedging,
    RegionFailover,
    CircuitBreakerProbe,
}
```

Steps:

1. **Add `OperationRetry`** to `ExecutionContext` (the enum is already `#[non_exhaustive]`, so adding a variant is non-breaking at the type level).
2. **Mark `Retry`** with a `#[deprecated]` attribute whose `note` points callers to `OperationRetry`.
3. **Update the hand-written `ExecutionContext::as_str()`** (`diagnostics_context.rs:56`) so `OperationRetry => "operation_retry"`, keeping (or removing) the deprecated `Retry => "retry"` arm. **This step is mandatory and was missing from the original draft** — `as_str()`, not just the serde derive, is the canonical wire/telemetry path.
4. **Update every dispatch site** to construct `OperationRetry` instead of `Retry`:
   - `sdk/cosmos/azure_data_cosmos_driver/src/driver/pipeline/operation_pipeline.rs` (the `Retry` sites near lines 940, 1285, 2784, 2790).
   - `sdk/cosmos/azure_data_cosmos_driver/src/driver/transport/transport_pipeline.rs` line 194.
5. **(Conditional) `#[serde(alias = "retry")]`** on `OperationRetry` — see §4.1.

### 4.1 JSON wire format — break ACCEPTED at the cross-SDK gate (Q5=B)

With the enum carrying `#[serde(rename_all = "snake_case")]` **and** the hand-written `as_str()`, the serialized form derives from the Rust spelling:

- Old variant `Retry` ⇒ wire shape `"retry"`.
- New variant `OperationRetry` ⇒ wire shape `"operation_retry"`.

This is a **wire-format break**: any downstream consumer that parses `ExecutionContext` telemetry and looks for the string `"retry"` will see `"operation_retry"` instead, starting at the implementation PR.

**Deliberate decisions:**

- **`#[serde(alias = "retry")]` is conditional, not unconditional.** `ExecutionContext` currently derives **`Serialize` only — not `Deserialize`** (§2). A serde `alias` affects *deserialization*, so it has no effect today. The original draft's claim that `alias` "preserves back-deserialization" is therefore only true **if** a `Deserialize` derive is also added. The implementation PR should either (a) add `#[serde(alias = "retry")]` *together with* a `Deserialize` derive if round-tripping persisted diagnostics becomes a requirement, or (b) omit the alias entirely while no `Deserialize` exists. Until then, the practical break is purely on the **serialized** string.
- **`#[serde(rename = "retry")]` is NOT applied.** The cross-SDK gate (decision Q5=B) explicitly rejected preserving the historical wire shape. The new variant **serializes** as `"operation_retry"`. The break is intentional and surfaced in the CHANGELOG entries (§9) so downstream telemetry parsers can update.

### 4.2 Why no `#[deprecated]` re-export mechanism

Rust has no first-class "variant alias" or "deprecated variant alias that resolves to a new variant". The closest available mechanism — `#[deprecated]` on the variant itself — works for source-level deprecation only. Renaming a public enum variant in Rust requires a new variant plus dispatch-site and `as_str()` updates; serde `alias` (paired with a `Deserialize` derive) then provides JSON-deserialization continuity, and serde `rename` would (if applied) preserve serialized output shape. The gate rejected `rename`, leaving the wire-format break and (optionally) `alias`-only back-compat.

---

## 5. Public-crate re-export decision

The accessors live on `DiagnosticsContext` in the driver crate. `DiagnosticsContext` is already re-exported as `azure_data_cosmos::CosmosDiagnosticsContext` (`sdk/cosmos/azure_data_cosmos/src/models/mod.rs:37`). The remaining question is what reason-carrying shape `requested_regions()` returns and which types become public in `azure_data_cosmos`.

The cross-SDK contract resolves the open part of this decision: it **does** expose per-region reason (a `RequestedRegion { region, reason }` pair). For contract conformance the Rust SDK follows suit.

| | **Option A — re-export driver `ExecutionContext`** | **Option B — SDK-owned `RequestedRegion` + `RequestedRegionReason` (RECOMMENDED — realizes the cross-SDK contract)** |
|---|---|---|
| Public surface | Adds driver `ExecutionContext` to `azure_data_cosmos` | Adds two SDK-owned types; driver `ExecutionContext` stays private |
| Per-region reason | Exposed (raw driver enum) | Exposed (clean SDK enum) |
| Matches "No Model Sharing" (AGENTS.md) | No (re-exports a driver-internal model) | **Yes** (SDK owns its public model; maps from the driver enum at the boundary) |
| Cross-SDK conformance | Partial (different enum shape/values) | **Full** (`RequestedRegion` + `RequestedRegionReason` match the contract's names) |
| Effort to revisit | Hard once shipped | Easy: mapping lives in one `From` impl |

**Recommendation: Option B.** It honors the "No Model Sharing" principle, gives the Rust SDK a clean public enum whose variant names match the cross-SDK contract (`OperationRetry`, `Hedging`, …), and keeps the driver's `ExecutionContext` free to evolve. The driver-private `ExecutionContext` is projected to `RequestedRegionReason` via the `From` impl in §3.5. Open Question (i) in §10 invites the team to confirm the `Unknown`-omission and reserved-variant choices.

---

## 6. Reconciliation with PR #4330's `HEDGING_SPEC.md`

PR #4330 is **merged** into `main`. The authoritative `HedgeDiagnostics` definition is in `sdk/cosmos/azure_data_cosmos_driver/docs/HEDGING_SPEC.md` and reads:

```rust
pub struct DiagnosticsContext {
    // ... existing fields ...
    pub hedge_diagnostics: Option<HedgeDiagnostics>,
}

pub struct HedgeDiagnostics {
    /// The hedging strategy configuration that was active.
    pub strategy_config: HedgingStrategyConfig,   // non-optional
    /// Regions that had requests launched (up to and including the winner).
    pub regions_contacted: Vec<Region>,
    /// The target region of the winning response.
    pub response_region: Region,                  // non-optional
    /// How many hedge requests were launched (including primary): 1 or 2.
    pub total_requests_launched: usize,
    /// Whether the primary or the alternate hedge won.
    pub was_hedge: bool,
}
```

> **Correction vs. the original draft.** The original draft paraphrased `strategy_config` and `response_region` as `Option<...>`. In the merged spec both are **non-optional** (`HedgeDiagnostics` is itself wrapped in `Option` on `DiagnosticsContext`, so optionality lives at the container level). The §3.3 doc-comment is corrected accordingly (`hedge_diagnostics.as_ref().map(|hd| &hd.strategy_config)`).

The two surfaces — the Rust-native `HedgeDiagnostics` field and the cross-SDK Hedging Detection API accessors — **coexist** on the same `DiagnosticsContext`. They serve different audiences (Rust-native rich detail vs. cross-SDK uniform shape) and are computed from overlapping but distinct internal sources.

### 6.1 Reconciliation table

| Surface | Cross-SDK accessor (this spec)                              | Rust-native field (PR #4330)                                                                                              | Relationship after both PRs land |
|---|---|---|---|
| "Did fan-out happen?" | `hedging_started() -> bool`                                 | `hedge_diagnostics.map(\|hd\| hd.total_requests_launched >= 2).unwrap_or(false)`                                          | **Equivalent** after implementation PR; equivalence is asserted by the invariant test in §8 (SE-023). |
| "Was a hedging strategy active?" | *not exposed* (intentionally; see §3.3 doc-comment) | `hedge_diagnostics.is_some()` (with `strategy_config` always present when so)             | `hedge_diagnostics.is_some()` is a **superset** of `hedging_started()` — true for primary-wins-under-threshold. |
| Regions tried | `requested_regions() -> Vec<RequestedRegion>` (dispatch order, duplicates, includes non-hedge attempts, **carries reason**) | `hedge_diagnostics.regions_contacted: Vec<Region>` (hedge-specific) | Different scope: `requested_regions()` always reflects every dispatched attempt (initial + retries + transport-retries + region-failovers + hedge fan-out); `HedgeDiagnostics::regions_contacted` reflects only hedge fan-out. Both are useful. |
| Regions that responded | `responded_regions() -> Vec<&Region>` (completion order, duplicates) | `hedge_diagnostics.response_region: Region` (single winner) | Different shape: `responded_regions()` is a complete list (including late losers); `response_region` is the single winner. `responded_regions().first()` is the winner only if responses arrived strictly in completion order — see §10 Open Question (iii). |
| Reason per region | `RequestedRegion.reason: RequestedRegionReason` (per cross-SDK contract) | *not exposed* (PR #4330 does not break out per-region reason) | Cross-SDK contract requires per-region reason; **now satisfied** by §3.5 (was previously deferred). |

### 6.2 Invariant after the implementation PR lands

```rust
// SE-023 invariant — asserted by the test in §8.
for ctx in /* every DiagnosticsContext produced by every test */ {
    let from_hedge_diag = ctx
        .hedge_diagnostics
        .as_ref()
        .map(|hd| hd.total_requests_launched >= 2)
        .unwrap_or(false);
    let from_requests = ctx
        .requests
        .iter()
        .any(|r| matches!(r.execution_context(), ExecutionContext::Hedging));
    assert_eq!(from_hedge_diag, from_requests);
}
```

This invariant is the load-bearing reason `hedging_started()` is defined as a disjunction (§3.3): if a future change ever inverts one of the two signals, the disjunction continues to return the correct user-visible answer **and** the invariant test fires in CI, surfacing the inversion immediately.

---

## 7. Sequencing

1. **PR #4330 — DONE.** Merged (via `release/azure_data_cosmos-previews`) into `main`; brings `HEDGING_SPEC.md` and the design contract for `hedge_diagnostics: Option<HedgeDiagnostics>`.
2. **This spec PR merges.** Adds `HEDGING_DETECTION_API_SPEC.md`. No production code change.
3. **Hedging implementation PR** (separate, future). Implements the orchestrator that populates `HedgeDiagnostics` **and** dispatches `RequestDiagnostics` with `ExecutionContext::Hedging`. Without this PR, both signals used by `hedging_started()` remain inactive — `hedging_started()` returns `false` for every operation, which is the correct answer (no fan-out has happened).
4. **Hedging Detection API implementation PR** (separate, future). Implements the three accessors specified here, the public `RequestedRegion` / `RequestedRegionReason` types (§3.5), the `Retry → OperationRetry` rename (§4), the re-export decision (§5), the test additions (§8), and the CHANGELOG entries (§9).

Steps 3 and 4 may be combined or kept separate at the implementer's discretion; this spec does not require either ordering between them, but the invariant test in §8 only becomes meaningful once Step 3 lands.

---

## 8. Test plan delta

The Hedging Detection API implementation PR (Step 4 in §7) MUST add:

### 8.1 Non-hedging operation tests (well-defined behavior)

In `sdk/cosmos/azure_data_cosmos/tests/emulator_tests/cosmos_items.rs` and `cosmos_query.rs`, add cases asserting:

- For a successful single-region read: `requested_regions().len() == 1` with the single entry's `reason == RequestedRegionReason::Initial`; `responded_regions().len() == 1`; `hedging_started() == false`.
- For an operation that retried (transient transport error) within a single region: `requested_regions()` contains the same region two or more times (duplicates allowed), with the later entries tagged `OperationRetry` (or `TransportRetry`); `hedging_started() == false`.
- For an operation that failed before region selection (e.g., misconfigured client): `requested_regions().is_empty()`; `responded_regions().is_empty()`; `hedging_started() == false`.

These cases prove the accessors are **well-defined even when hedging is disabled / not yet implemented**, and that the `ExecutionContext → RequestedRegionReason` mapping (§3.5) is exercised on real dispatch history.

### 8.2 SE-023 mandatory invariant test (added on the hedging implementation PR)

For every `DiagnosticsContext` produced by every emulator / fault-injection / live-multi-region test, assert the two `hedging_started()` signals agree:

```rust
let from_hedge_diag = ctx
    .hedge_diagnostics
    .as_ref()
    .map(|hd| hd.total_requests_launched >= 2)
    .unwrap_or(false);
let from_requests = ctx
    .requests
    .iter()
    .any(|r| matches!(r.execution_context(), ExecutionContext::Hedging));
assert_eq!(
    from_hedge_diag, from_requests,
    "SE-023: hedge_diagnostics and ExecutionContext::Hedging disagree on fan-out"
);
assert_eq!(ctx.hedging_started(), from_hedge_diag || from_requests);
```

### 8.3 Hedging-active tests (added on / after the hedging implementation PR)

Using the fault-injection harness described in `HEDGING_SPEC.md` (e.g., `hedging_read_primary_fast`, alternate-wins, both-fail cases):

- **Primary wins under threshold** (`hedging_read_primary_fast`): `hedging_started() == false`; `requested_regions()` has exactly one entry tagged `Initial`; no entry tagged `Hedging`.
- **Alternate hedge wins:** `hedging_started() == true`; `requested_regions()` contains at least one `Hedging`-tagged entry; `responded_regions()` contains the winning region; the winning region equals `hedge_diagnostics.response_region`.
- **Late loser response:** `responded_regions().len()` may exceed the number of distinct regions; the test asserts duplicates are preserved (not deduped).

### 8.4 Type-level tests for the public enum

- `RequestedRegionReason` round-trips through the `From<ExecutionContext>` mapping for every `ExecutionContext` variant (a `match` with no wildcard in the test forces the mapping to stay total as variants are added).
- A doc-test (or unit test) demonstrates the mandatory wildcard arm when matching `RequestedRegionReason` (it is `#[non_exhaustive]`).

---

## 9. CHANGELOG entries (drafted here; landed by the implementation PR)

These entries are **drafts**. They are not added to any `CHANGELOG.md` in this spec-only PR; the Hedging Detection API implementation PR (Step 4 in §7) lands them.

### 9.1 `sdk/cosmos/azure_data_cosmos/CHANGELOG.md` (under the next unreleased version, currently `0.34.0`)

`### Features Added`

- Added the cross-SDK Hedging Detection API on `CosmosDiagnosticsContext`: `hedging_started() -> bool`, `requested_regions() -> Vec<RequestedRegion>` (dispatch order, duplicates allowed, each entry tagged with a `RequestedRegionReason`), and `responded_regions() -> Vec<&Region>` (arrival order, duplicates allowed). Adds the public `RequestedRegion` struct and `RequestedRegionReason` enum that realize the cross-SDK contract's per-region-reason surface. `RequestedRegionReason` is `#[non_exhaustive]`; `TransportRetry` and `CircuitBreakerProbe` are reserved and not populated by the initial implementation.

### 9.2 `sdk/cosmos/azure_data_cosmos_driver/CHANGELOG.md` (under the next unreleased version, currently `0.3.0`)

`### Features Added`

- Added `DiagnosticsContext::requested_regions()`, `responded_regions()`, and `hedging_started()` backing the SDK's cross-SDK Hedging Detection API.

`### Breaking Changes`

- Renamed `ExecutionContext::Retry` to `ExecutionContext::OperationRetry` to align with the cross-SDK reason taxonomy (`OperationRetry` distinguishes operation-level retries from `TransportRetry`). The serialized diagnostics string changes from `"retry"` to `"operation_retry"`; the old variant is retained as `#[deprecated]` for one release. Telemetry parsers that match on the string `"retry"` must be updated. (No `#[serde(rename = "retry")]` is applied; see `HEDGING_DETECTION_API_SPEC.md` §4.)

---

## 10. Open questions

- **(i) Public enum shape — RESOLVED toward the cross-SDK contract.** The contract exposes a `RequestedRegion { region, reason }` pair with a `RequestedRegionReason`. This spec adopts the same shape (§3.5, §5, Option B). Remaining sub-questions for team confirmation: (a) omitting the `Unknown` sentinel in Rust (justified in §3.5); (b) keeping `TransportRetry` / `CircuitBreakerProbe` as reserved variants.
- **(ii) Driver rename vs. boundary-only mapping.** §4 renames the driver-private `ExecutionContext::Retry → OperationRetry` (gate Q5=B accepts the resulting serialized-string break). An alternative is to leave `ExecutionContext::Retry` untouched and only name the *public* `RequestedRegionReason::OperationRetry`, mapping at the boundary. The driver rename is recommended for internal consistency, but the team may prefer the boundary-only approach to avoid the diagnostics-string break entirely.
- **(iii) Completion-order index for `responded_regions()`.** §3.2 proposes a driver-private `Vec<usize>` completion-order index (option (c)). Confirm vs. the cheaper accessor-time stable sort by `completed_at` (option (b)). For the typically-small request counts per operation, (b) may be simpler with negligible cost.
- **(iv) `Deserialize` on `ExecutionContext`.** Decide whether the implementation PR adds a `Deserialize` derive (and the paired `#[serde(alias = "retry")]`) to support round-tripping persisted diagnostics, or leaves `ExecutionContext` `Serialize`-only (§4.1). No consumer requires `Deserialize` today.
- **(v) Reason for transport retries and circuit-breaker probes.** Whether the driver should tag transport-retry and PPCB-probe `RequestDiagnostics` with a resolved region (and thus surface `TransportRetry` / `CircuitBreakerProbe` in `requested_regions()`), matching the reserved enum values. The cross-SDK contract permits leaving these unpopulated in the first version.

---

## 11. Rust ↔ cross-SDK contract mapping (quick reference)

| Concept | Rust (this spec) | Cross-SDK contract |
|---|---|---|
| Did fan-out happen? | `CosmosDiagnosticsContext::hedging_started() -> bool` | `hedging_started()` |
| Regions dispatched to (with reason) | `requested_regions() -> Vec<RequestedRegion>` | `requested_regions()` |
| Regions that responded (identifiers only) | `responded_regions() -> Vec<&Region>` | `responded_regions()` |
| Per-region pair | `RequestedRegion { region: Region, reason: RequestedRegionReason }` | `RequestedRegion { region, reason }` |
| Reason taxonomy | `RequestedRegionReason { Initial, OperationRetry, TransportRetry, Hedging, RegionFailover, CircuitBreakerProbe }` (no `Unknown`) | `Initial`, `OperationRetry`, `TransportRetry`, `Hedging`, `RegionFailover`, `CircuitBreakerProbe` (plus an `Unknown` default sentinel where the host language needs one) |
| Non-exhaustiveness | `#[non_exhaustive]` (wildcard arm required) | non-exhaustive enumeration (callers must handle unknown values) |
