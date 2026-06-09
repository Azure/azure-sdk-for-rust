# Hedging Detection API — Spec

**Status:** Implemented in this PR (spec + implementation landed together), targeting `main`.
**Target branch:** `main`
**Tracking issue:** [Azure/azure-sdk-for-rust#4410](https://github.com/Azure/azure-sdk-for-rust/issues/4410)
**Cross-SDK contract:** The cross-SDK Hedging Detection API (a per-operation diagnostics surface shared in shape across the Cosmos DB SDKs). This Rust spec adopts that contract; the type/method names below are the contract's canonical names rendered in idiomatic Rust.
**Sequencing:** Lands AFTER both the hedging *design spec* ([PR #4330](https://github.com/Azure/azure-sdk-for-rust/pull/4330)) and the hedging *implementation* ([PR #4432](https://github.com/Azure/azure-sdk-for-rust/pull/4432)), both already merged into `main`.

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

This document specifies the Rust SDK's adoption of that contract on `azure_data_cosmos::DiagnosticsContext` (re-export of `azure_data_cosmos_driver::diagnostics::DiagnosticsContext`).

This spec is **sequenced** with respect to PR #4330 ("Cosmos: Adds Cross-Region Hedging Design Spec to Driver Crate") and the hedging implementation (#4432), both merged into `main`:

| Phase                                                | Artifact                                          | Status                  |
|------------------------------------------------------|---------------------------------------------------|-------------------------|
| Hedging Design Spec                                  | `sdk/cosmos/azure_data_cosmos_driver/docs/HEDGING_SPEC.md` (PR #4330) | **Merged** into `main` |
| Hedging implementation (orchestrator + dispatch)     | Emits `ExecutionContext::Hedging`, populates `HedgeDiagnostics` (PR #4432) | **Merged** into `main` |
| **Hedging Detection API spec (this document)**       | `sdk/cosmos/azure_data_cosmos/docs/HEDGING_DETECTION_API_SPEC.md`     | **This PR**             |
| Hedging Detection API implementation (this spec → code) | Adds the three accessors + the public `RequestedRegion` / `RequestedRegionReason` types specified below | **This PR** (landed with the spec) |

This PR adds the spec Markdown file **and** the Hedging Detection API implementation it describes (the three accessors, the public `RequestedRegion` / `RequestedRegionReason` types, and the `Retry → OperationRetry` rename). The spec and its implementation were small enough to land together rather than in the separate follow-up PR originally envisioned.

The Hedging Detection API is **additive** on `DiagnosticsContext` and **complementary** to (not a replacement for) PR #4330's `DiagnosticsContext::hedge_diagnostics: Option<HedgeDiagnostics>` field. The two surfaces are explicitly reconciled in §6 below.

---

## 2. Verified existing surface inventory

Citations are to the `main` branch at the merge of PR #4330 — commit [`5f5d8c49d02b579a2afd2297857b919900ff1dad`](https://github.com/Azure/azure-sdk-for-rust/commit/5f5d8c49d02b579a2afd2297857b919900ff1dad) ("Cosmos: Adds Cross-Region Hedging Design Spec to Driver Crate (#4330)"). The `diagnostics_context.rs` line numbers below are SHA-pinned via [this permalink](https://github.com/Azure/azure-sdk-for-rust/blob/5f5d8c49d02b579a2afd2297857b919900ff1dad/sdk/cosmos/azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs).

| Item                               | Location                                                                                                  | Visibility                                            |
|------------------------------------|-----------------------------------------------------------------------------------------------------------|-------------------------------------------------------|
| `DiagnosticsContext`               | `sdk/cosmos/azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs`                              | `pub` in driver; re-exported as `azure_data_cosmos::DiagnosticsContext` (`sdk/cosmos/azure_data_cosmos/src/models/mod.rs:37`) |
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

All three accessors are added to `DiagnosticsContext` in `azure_data_cosmos_driver` and exposed on `DiagnosticsContext` via the existing re-export (`sdk/cosmos/azure_data_cosmos/src/models/mod.rs:37`). The reason-carrying return of `requested_regions()` (§3.1) requires two **new types** — `RequestedRegion` and `RequestedRegionReason`. Because the accessors are inherent methods on the driver's `DiagnosticsContext` and `azure_data_cosmos` depends on `azure_data_cosmos_driver` (never the reverse), these two types are **defined in the driver and re-exported from `azure_data_cosmos`**, exactly like `DiagnosticsContext`/`DiagnosticsContext` itself (see §3.5 and §5).

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
/// To deduplicate, callers can collect into a set, for example:
/// `let unique: Vec<&Region> = ctx.responded_regions().into_iter().collect::<std::collections::BTreeSet<_>>().into_iter().collect();`
pub fn responded_regions(&self) -> Vec<&Region> { ... }
```

- **Shape:** region references only — **no reason**. This follows the cross-SDK contract's `responded_regions()`, which returns region identifiers only. No new type is required.
- **Source:** `self.requests.iter()`.
- **Filter:** include only entries that (a) have `Some(region)` AND (b) actually produced a service response. Note that `completed_at.is_some()` alone is **not** sufficient: the driver sets `completed_at` from three sites — `complete()` (a real response arrived), `timeout()` (client-side end-to-end timeout), and `fail_transport()` (transport-level failure) (`diagnostics_context.rs:505,531,552`). The correct predicate excludes the latter two: `region.is_some() && completed_at.is_some() && !timed_out() && error.is_none()` (a non-2xx HTTP status such as 404/429 still counts — it is a response from the region; `error` is only set on transport failures). The implementation may instead expose a small `pub(crate)` helper (e.g., `responded_with_service_reply()`) on `RequestDiagnostics` to centralize this predicate.
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

**Definition (as shipped against `main`'s landed hedging implementation, [#4432](https://github.com/Azure/azure-sdk-for-rust/pull/4432)):**

```rust
pub fn hedging_started(&self) -> bool {
    self
        .hedge_diagnostics
        .as_ref()
        .map(|hd| hd.alternate_region().is_some())
        .unwrap_or(false)
    || self
        .requests
        .iter()
        .any(|r| matches!(r.execution_context(), ExecutionContext::Hedging))
}
```

`HedgeDiagnostics::alternate_region()` is `Some` exactly when the orchestrator dispatched an alternate hedge leg (i.e., fan-out happened); it is `None` for the primary-wins-under-threshold and deadline-exceeded-pre-threshold terminal states. This is `main`'s equivalent of the cross-SDK contract's "≥ 2 requests launched" predicate (`main`'s `HedgeDiagnostics` classifies the race via `terminal_state` / `alternate_region` rather than a `total_requests_launched` counter).

**Why the disjunction (and not either predicate alone) — SE-023:**

- **Why NOT `hedge_diagnostics.is_some()` alone.** `HedgeDiagnostics` is `Some` whenever a hedging strategy is *active for this operation*, including the **primary-wins-under-threshold** case — only the primary leg ran (`alternate_region() == None`), threshold delay never elapsed, no fan-out. The cross-SDK contract requires `hedging_started() == true` iff the SDK **actually dispatched to a hedge region**, i.e., `alternate_region().is_some()` — not "strategy was active". Returning `true` for the primary-only case would contradict the cross-SDK contract.
- **Why NOT the `ExecutionContext::Hedging` predicate alone.** A future internal refactor to `HedgeDiagnostics` (e.g., a change of semantics around `alternate_region` / `terminal_state`) could silently cause the two predicates to drift out of sync. The disjunction is the **safe** definition: it returns `true` whenever *either* signal indicates fan-out occurred, and the invariant test in §8 enforces that the two signals stay equivalent.
- **Behavior on `main`.** `main` now populates `hedge_diagnostics` and emits `ExecutionContext::Hedging` from the hedging orchestrator ([#4432](https://github.com/Azure/azure-sdk-for-rust/pull/4432)). When no read is hedged (hedging disabled, or a non-read/non-eligible operation), `hedge_diagnostics` is `None` and no request is tagged `Hedging`, so `hedging_started()` returns `false` — the correct answer: no fan-out has happened.

### 3.4 Pre-region-selection failures

For all three accessors, a `RequestDiagnostics` whose `region` is `None` (pre-region-selection failure) is **skipped**. This matches `regions_contacted()` precedent and the cross-SDK contract. `requested_regions()` therefore returns an empty `Vec` when an operation failed before any region was selected, matching the contract's "empty when there is no dispatch history" rule.

### 3.5 New public types — `RequestedRegion` and `RequestedRegionReason`

These realize the cross-SDK contract's `RequestedRegion` value type and `RequestedRegionReason` enumeration (§1), in idiomatic Rust. Because the accessors that return them are inherent methods on the driver's `DiagnosticsContext`, both types are **defined in `azure_data_cosmos_driver` and re-exported from `azure_data_cosmos`** (the crate dependency runs SDK → driver only; the driver cannot reference SDK types). This matches the existing treatment of `DiagnosticsContext` (re-exported as `DiagnosticsContext`) — see §5.

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

The accessors live on `DiagnosticsContext` in the driver crate. `DiagnosticsContext` is already re-exported as `azure_data_cosmos::DiagnosticsContext` (`sdk/cosmos/azure_data_cosmos/src/models/mod.rs:37`). The remaining question is what reason-carrying shape `requested_regions()` returns and where the per-region-reason types are defined.

**Crate-layering constraint.** `azure_data_cosmos` depends on `azure_data_cosmos_driver`; the driver has **no** back-dependency on the SDK. An inherent method on the driver's `DiagnosticsContext` therefore cannot return an SDK-defined type. Any reason-carrying return type for `requested_regions()` must be **driver-defined** (and then re-exported), unless the accessors themselves are moved out of the driver (e.g., an SDK-side extension trait or wrapper).

The cross-SDK contract requires per-region reason (a `RequestedRegion { region, reason }` pair). Within the layering constraint, the realistic options are:

| | **Option A — re-export driver `ExecutionContext` directly** | **Option B — driver-defined `RequestedRegion` + `RequestedRegionReason`, re-exported by the SDK (RECOMMENDED)** | **Option C — SDK-owned types via an SDK-side extension trait** |
|---|---|---|---|
| Where the accessors live | Inherent on driver `DiagnosticsContext` | Inherent on driver `DiagnosticsContext` | SDK extension trait on `DiagnosticsContext` |
| Per-region reason type | Raw driver `ExecutionContext` (re-exported) | Clean driver enum `RequestedRegionReason` (re-exported) | SDK-owned `RequestedRegionReason` |
| Public-name stability | Tied to the internal `ExecutionContext` taxonomy | Stable contract names, decoupled from `ExecutionContext` | Stable contract names |
| Cross-SDK conformance | Partial (enum shape/values differ from the contract) | **Full** (names match the contract) | Full |
| `From<ExecutionContext>` mapping | n/a | Driver-internal (both types in driver) | Must be SDK-internal; the driver enum must already be public |
| Cost / friction | Low | Low (mirrors existing `DiagnosticsContext` re-export) | Higher (extension-trait ergonomics; discoverability) |

**Recommendation: Option B.** Define `RequestedRegion` and `RequestedRegionReason` in the driver alongside `ExecutionContext`, keep the accessors inherent on `DiagnosticsContext`, and re-export both types from `azure_data_cosmos` — the same pattern already used for `DiagnosticsContext`. The driver-internal `ExecutionContext` is projected to `RequestedRegionReason` via the `From` impl in §3.5, so the public surface carries the stable contract names while `ExecutionContext` stays driver-private and free to evolve. This is **not** strict "No Model Sharing" — it shares a *purpose-built, stable* public model from the driver, consistent with the precedent that the entire diagnostics model (`DiagnosticsContext`) is already driver-owned and re-exported. If the team wants strict SDK ownership of the public model, Option C moves the accessors into an SDK-side extension trait at the cost of exposing the driver `ExecutionContext` (or a driver projection) for the trait to map from. Open Question (i) in §10 invites the team to confirm Option B vs. C and the `Unknown`-omission / reserved-variant choices.

---

## 6. Reconciliation with PR #4330's `HEDGING_SPEC.md`

PR #4330 is **merged** into `main`. The authoritative `HedgeDiagnostics` definition is in `sdk/cosmos/azure_data_cosmos_driver/docs/HEDGING_SPEC.md` ([SHA-pinned permalink](https://github.com/Azure/azure-sdk-for-rust/blob/5f5d8c49d02b579a2afd2297857b919900ff1dad/sdk/cosmos/azure_data_cosmos_driver/docs/HEDGING_SPEC.md)) and reads:

```rust
pub struct DiagnosticsContext {
    // ... existing fields ...
    hedge_diagnostics: Option<HedgeDiagnostics>, // accessor: hedge_diagnostics(&self) -> Option<&HedgeDiagnostics>
}

// `main`'s landed shape (#4432). Fields are private; values are read through
// accessors. The race outcome is classified by `terminal_state`, not a
// `total_requests_launched` counter.
pub struct HedgeDiagnostics {
    strategy_config: HedgingStrategyConfig,   // -> strategy_config() -> HedgingStrategyConfig
    primary_region: Region,                   // -> primary_region() -> &Region (always populated)
    alternate_region: Option<Region>,         // -> alternate_region() -> Option<&Region> (Some iff fan-out)
    response_region: Option<Region>,          // -> response_region() -> Option<&Region> (Some iff a leg won)
    terminal_state: HedgeTerminalState,        // -> terminal_state() -> HedgeTerminalState (authoritative outcome)
}
```

> **Reconciled with `main`.** `main`'s `HedgeDiagnostics` ([#4432](https://github.com/Azure/azure-sdk-for-rust/pull/4432)) does **not** carry `total_requests_launched`, `was_hedge`, or a `regions_contacted` list; it exposes `primary_region` / `alternate_region` / `response_region` (the latter two `Option`) plus an authoritative `terminal_state`. "Fan-out happened" is therefore `alternate_region().is_some()`, and "the alternate won" is `matches!(terminal_state(), HedgeTerminalState::AlternateWon)`. The §3.3 `hedging_started()` definition uses `alternate_region().is_some()` accordingly.

The two surfaces — the Rust-native `HedgeDiagnostics` field and the cross-SDK Hedging Detection API accessors — **coexist** on the same `DiagnosticsContext`. They serve different audiences (Rust-native rich detail vs. cross-SDK uniform shape) and are computed from overlapping but distinct internal sources.

### 6.1 Reconciliation table

| Surface | Cross-SDK accessor (this spec)                              | Rust-native `HedgeDiagnostics` (#4432)                                                                                              | Relationship on `main` |
|---|---|---|---|
| "Did fan-out happen?" | `hedging_started() -> bool`                                 | `hedge_diagnostics().map(\|hd\| hd.alternate_region().is_some()).unwrap_or(false)`                                          | **Equivalent**; equivalence is asserted by the invariant test in §8 (SE-023). |
| "Was a hedging strategy active?" | *not exposed* (intentionally; see §3.3 doc-comment) | `hedge_diagnostics().is_some()` (with `strategy_config` always present when so)             | `hedge_diagnostics().is_some()` is a **superset** of `hedging_started()` — true for primary-wins-under-threshold. |
| Regions tried | `requested_regions() -> Vec<RequestedRegion>` (dispatch order, duplicates, includes non-hedge attempts, **carries reason**) | `hedge_diagnostics().primary_region()` + `alternate_region()` (hedge-specific) | Different scope: `requested_regions()` always reflects every dispatched attempt (initial + retries + transport-retries + region-failovers + hedge fan-out); `HedgeDiagnostics` reflects only the hedge primary/alternate. Both are useful. |
| Regions that responded | `responded_regions() -> Vec<&Region>` (completion order, duplicates) | `hedge_diagnostics().response_region(): Option<&Region>` (single winner, `None` for terminal-error states) | Different shape: `responded_regions()` is a complete list (including late losers); `response_region()` is the single winner. `responded_regions().first()` is the winner only if responses arrived strictly in completion order — see §10 Open Question (iii). |
| Reason per region | `RequestedRegion.reason: RequestedRegionReason` (per cross-SDK contract) | *not exposed* (`HedgeDiagnostics` does not break out per-region reason) | Cross-SDK contract requires per-region reason; **now satisfied** by §3.5 (was previously deferred). |

### 6.2 Invariant (live on `main`)

```rust
// SE-023 invariant — asserted by the test in §8.
for ctx in /* every DiagnosticsContext produced by every test */ {
    let from_hedge_diag = ctx
        .hedge_diagnostics()
        .map(|hd| hd.alternate_region().is_some())
        .unwrap_or(false);
    let from_requests = ctx
        .requests()
        .iter()
        .any(|r| matches!(r.execution_context(), ExecutionContext::Hedging));
    assert_eq!(from_hedge_diag, from_requests);
}
```

This invariant is the load-bearing reason `hedging_started()` is defined as a disjunction (§3.3): if a future change ever inverts one of the two signals, the disjunction continues to return the correct user-visible answer **and** the invariant test fires in CI, surfacing the inversion immediately.

---

## 7. Sequencing

1. **PR #4330 — DONE.** The Cross-Region Hedging *design spec* merged into `main`.
2. **Hedging implementation — DONE ([#4432](https://github.com/Azure/azure-sdk-for-rust/pull/4432)).** `main` now has the orchestrator that populates `HedgeDiagnostics` **and** dispatches `RequestDiagnostics` with `ExecutionContext::Hedging`, plus the public `AvailabilityStrategy` / `HedgingStrategy` / `HedgeThreshold` surface. Both signals used by `hedging_started()` are therefore live.
3. **This PR.** Adds `HEDGING_DETECTION_API_SPEC.md` **and** the Hedging Detection API implementation it describes: the three accessors, the public `RequestedRegion` / `RequestedRegionReason` types (§3.5), the `Retry → OperationRetry` rename (§4), the re-export decision (§5), driver-level unit tests for the accessors and the `From<ExecutionContext>` mapping (§8.1 non-emulator cases plus §8.4 type-level checks), and the CHANGELOG entries (§9). Because hedging already landed (Step 2), `hedging_started()` is wired to `hedge_diagnostics` from the start.

The hedging-active emulator/fault-injection tests (§8.3) and the SE-023 invariant assertion across live multi-region runs (§8.2) are valuable follow-ups but are not required for this PR, which relies on driver-level unit coverage.

---

## 8. Test plan delta

The Hedging Detection API implementation PR (Step 4 in §7) MUST add:

### 8.1 Non-hedging operation tests (well-defined behavior)

In `sdk/cosmos/azure_data_cosmos/tests/emulator_tests/cosmos_items.rs` and `cosmos_query.rs`, add cases asserting:

- For a successful single-region read: `requested_regions().len() == 1` with the single entry's `reason == RequestedRegionReason::Initial`; `responded_regions().len() == 1`; `hedging_started() == false`.
- For an operation that retried (transient transport error) within a single region: `requested_regions()` contains the same region two or more times (duplicates allowed), with the later entries tagged `OperationRetry` (or `TransportRetry`); `hedging_started() == false`.
- For an operation that failed before region selection (e.g., misconfigured client): `requested_regions().is_empty()`; `responded_regions().is_empty()`; `hedging_started() == false`.

These cases prove the accessors are **well-defined even when hedging is disabled / not yet implemented**, and that the `ExecutionContext → RequestedRegionReason` mapping (§3.5) is exercised on real dispatch history.

### 8.2 SE-023 invariant test (recommended follow-up, live on `main`)

For every `DiagnosticsContext` produced by every emulator / fault-injection / live-multi-region test, assert the two `hedging_started()` signals agree:

```rust
let from_hedge_diag = ctx
    .hedge_diagnostics()
    .map(|hd| hd.alternate_region().is_some())
    .unwrap_or(false);
let from_requests = ctx
    .requests()
    .iter()
    .any(|r| matches!(r.execution_context(), ExecutionContext::Hedging));
assert_eq!(
    from_hedge_diag, from_requests,
    "SE-023: hedge_diagnostics and ExecutionContext::Hedging disagree on fan-out"
);
assert_eq!(ctx.hedging_started(), from_hedge_diag || from_requests);
```

### 8.3 Hedging-active tests (recommended follow-up)

Using the fault-injection harness described in `HEDGING_SPEC.md` (e.g., `hedging_read_primary_fast`, alternate-wins, both-fail cases):

- **Primary wins under threshold** (`hedging_read_primary_fast`): `hedging_started() == false`; `requested_regions()` has exactly one entry tagged `Initial`; no entry tagged `Hedging`.
- **Alternate hedge wins:** `hedging_started() == true`; `requested_regions()` contains at least one `Hedging`-tagged entry; `responded_regions()` contains the winning region; the winning region equals `hedge_diagnostics().response_region()`.
- **Late loser response:** `responded_regions().len()` may exceed the number of distinct regions; the test asserts duplicates are preserved (not deduped).

### 8.4 Type-level tests for the public enum

- `RequestedRegionReason` round-trips through the `From<ExecutionContext>` mapping for every `ExecutionContext` variant (a `match` with no wildcard in the test forces the mapping to stay total as variants are added).
- A doc-test (or unit test) demonstrates the mandatory wildcard arm when matching `RequestedRegionReason` (it is `#[non_exhaustive]`).

---

## 9. CHANGELOG entries (landed in this PR)

The entries below are added to both crates' `CHANGELOG.md` files in this PR, under their respective unreleased versions.

### 9.1 `sdk/cosmos/azure_data_cosmos/CHANGELOG.md` (under the next unreleased version, currently `0.35.0`)

`### Features Added`

- Added the cross-SDK Hedging Detection API on `DiagnosticsContext`: `hedging_started() -> bool`, `requested_regions() -> Vec<RequestedRegion>` (dispatch order, duplicates allowed, each entry tagged with a `RequestedRegionReason`), and `responded_regions() -> Vec<&Region>` (arrival order, duplicates allowed). Re-exports the `RequestedRegion` struct and `RequestedRegionReason` enum (defined in `azure_data_cosmos_driver`) that realize the cross-SDK contract's per-region-reason surface. `RequestedRegionReason` is `#[non_exhaustive]`; `TransportRetry` and `CircuitBreakerProbe` are reserved and not populated by the initial implementation.

### 9.2 `sdk/cosmos/azure_data_cosmos_driver/CHANGELOG.md` (under the next unreleased version, currently `0.4.0`)

`### Features Added`

- Added `DiagnosticsContext::requested_regions()`, `responded_regions()`, and `hedging_started()` backing the SDK's cross-SDK Hedging Detection API, along with the public `RequestedRegion` struct and `RequestedRegionReason` enum (re-exported by `azure_data_cosmos`).

`### Breaking Changes`

- Renamed `ExecutionContext::Retry` to `ExecutionContext::OperationRetry` to align with the cross-SDK reason taxonomy (`OperationRetry` distinguishes operation-level retries from `TransportRetry`). The serialized diagnostics string changes from `"retry"` to `"operation_retry"`; the old variant is retained as `#[deprecated]` for one release. Telemetry parsers that match on the string `"retry"` must be updated. (No `#[serde(rename = "retry")]` is applied; see `HEDGING_DETECTION_API_SPEC.md` §4.)

---

## 10. Open questions

- **(i) Public enum shape & ownership — RESOLVED toward the cross-SDK contract, pending placement confirmation.** The contract exposes a `RequestedRegion { region, reason }` pair with a `RequestedRegionReason`. This spec adopts the same shape (§3.5). Because the accessors are inherent on the driver's `DiagnosticsContext` and the driver cannot depend on the SDK, §5 recommends **Option B** (types defined in the driver, re-exported by `azure_data_cosmos`). Sub-questions for team confirmation: (a) Option B vs. **Option C** (SDK-side extension trait owning the public types); (b) omitting the `Unknown` sentinel in Rust (justified in §3.5); (c) keeping `TransportRetry` / `CircuitBreakerProbe` as reserved variants.
- **(ii) Driver rename vs. boundary-only mapping.** §4 renames the driver-private `ExecutionContext::Retry → OperationRetry` (gate Q5=B accepts the resulting serialized-string break). An alternative is to leave `ExecutionContext::Retry` untouched and only name the *public* `RequestedRegionReason::OperationRetry`, mapping at the boundary. The driver rename is recommended for internal consistency, but the team may prefer the boundary-only approach to avoid the diagnostics-string break entirely.
- **(iii) Completion-order index for `responded_regions()`.** §3.2 proposes a driver-private `Vec<usize>` completion-order index (option (c)). Confirm vs. the cheaper accessor-time stable sort by `completed_at` (option (b)). For the typically-small request counts per operation, (b) may be simpler with negligible cost.
- **(iv) `Deserialize` on `ExecutionContext`.** Decide whether the implementation PR adds a `Deserialize` derive (and the paired `#[serde(alias = "retry")]`) to support round-tripping persisted diagnostics, or leaves `ExecutionContext` `Serialize`-only (§4.1). No consumer requires `Deserialize` today.
- **(v) Reason for transport retries and circuit-breaker probes.** Whether the driver should tag transport-retry and PPCB-probe `RequestDiagnostics` with a resolved region (and thus surface `TransportRetry` / `CircuitBreakerProbe` in `requested_regions()`), matching the reserved enum values. The cross-SDK contract permits leaving these unpopulated in the first version.

---

## 11. Rust ↔ cross-SDK contract mapping (quick reference)

| Concept | Rust (this spec) | Cross-SDK contract |
|---|---|---|
| Did fan-out happen? | `DiagnosticsContext::hedging_started() -> bool` | `hedging_started()` |
| Regions dispatched to (with reason) | `requested_regions() -> Vec<RequestedRegion>` | `requested_regions()` |
| Regions that responded (identifiers only) | `responded_regions() -> Vec<&Region>` | `responded_regions()` |
| Per-region pair | `RequestedRegion { region: Region, reason: RequestedRegionReason }` | `RequestedRegion { region, reason }` |
| Reason taxonomy | `RequestedRegionReason { Initial, OperationRetry, TransportRetry, Hedging, RegionFailover, CircuitBreakerProbe }` (no `Unknown`) | `Initial`, `OperationRetry`, `TransportRetry`, `Hedging`, `RegionFailover`, `CircuitBreakerProbe` (plus an `Unknown` default sentinel where the host language needs one) |
| Non-exhaustiveness | `#[non_exhaustive]` (wildcard arm required) | non-exhaustive enumeration (callers must handle unknown values) |
