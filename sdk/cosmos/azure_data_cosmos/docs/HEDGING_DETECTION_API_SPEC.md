# Hedging Detection API — Spec

**Status:** Draft (spec-only; no production code change in this PR)
**Target branch:** `release/azure_data_cosmos-previews`
**Tracking issue:** [Azure/azure-sdk-for-rust#4410](https://github.com/Azure/azure-sdk-for-rust/issues/4410)
**Cross-SDK design doc:** "Hedging Detection API" (.NET / Java / Python / Rust)
**Sequencing:** Lands AFTER [PR #4330](https://github.com/Azure/azure-sdk-for-rust/pull/4330) merges and BEFORE any hedging-implementation PR.

---

## 1. Introduction — cross-reference to PR #4330

The Azure Cosmos DB SDKs (.NET, Java, Python) are converging on a cross-SDK Hedging Detection API exposed on each SDK's per-operation diagnostics surface. This document specifies the Rust SDK's adoption of that contract on `azure_data_cosmos::CosmosDiagnosticsContext` (re-export of `azure_data_cosmos_driver::diagnostics::DiagnosticsContext`).

This spec is **sequenced** with respect to PR #4330 ("Cosmos: Adds Cross-Region Hedging Design Spec to Driver Crate"):

| Phase                                                | Artifact                                          | Status                  |
|------------------------------------------------------|---------------------------------------------------|-------------------------|
| Hedging Design Spec                                  | `sdk/cosmos/azure_data_cosmos_driver/docs/HEDGING_SPEC.md` (PR #4330) | Open, under review      |
| **Hedging Detection API spec (this document)**       | `sdk/cosmos/azure_data_cosmos/docs/HEDGING_DETECTION_API_SPEC.md`     | **This PR, draft**      |
| Hedging implementation (orchestrator + dispatch)     | Separate future PR — emits `ExecutionContext::Hedging`, populates `HedgeDiagnostics` | Future                  |
| Hedging Detection API implementation (this spec → code) | Separate future PR — adds the three accessors specified below       | Future, after the above |

This PR adds **exactly one Markdown file** and changes no production code, matching PR #4330's doc-only pattern.

The Hedging Detection API is **additive** on `DiagnosticsContext` and **complementary** to (not a replacement for) PR #4330's `DiagnosticsContext::hedge_diagnostics: Option<HedgeDiagnostics>` field. The two surfaces are explicitly reconciled in §6 below.

---

## 2. Verified existing surface inventory

Citations are to `upstream/release/azure_data_cosmos-previews` at the time of this spec.

| Item                               | Location                                                                                                  | Visibility                                            |
|------------------------------------|-----------------------------------------------------------------------------------------------------------|-------------------------------------------------------|
| `DiagnosticsContext`               | `sdk/cosmos/azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs` (line ~35)                   | `pub` in driver; re-exported as `azure_data_cosmos::CosmosDiagnosticsContext` (`sdk/cosmos/azure_data_cosmos/src/models/mod.rs:38`) |
| `RequestDiagnostics`               | `sdk/cosmos/azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs`                              | `pub` in driver; **NOT** re-exported from `azure_data_cosmos` |
| `ExecutionContext`                 | `sdk/cosmos/azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs` (line ~35)                   | `pub` in driver; **NOT** re-exported                  |
| `ExecutionContext` variants today  | `Initial`, `Retry`, `TransportRetry`, `RegionFailover`, `Hedging` (reserved), `CircuitBreakerProbe` (reserved) | —                                                     |
| `DiagnosticsContext::regions_contacted()` | `sdk/cosmos/azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs`                       | `pub fn`; returns regions contacted today             |
| Dispatch sites that set `ExecutionContext` | `sdk/cosmos/azure_data_cosmos_driver/src/pipelines/operation_pipeline.rs` lines ~938, 940, 942, 1217–1348; `transport_pipeline.rs` line ~194 | Only `Initial`, `Retry`, `TransportRetry`, `RegionFailover` are emitted today; `Hedging` is **not** dispatched. |

**Key facts** (carried forward into §3 and §10):

- `ExecutionContext::Hedging` is a **reserved** variant; no production code path emits it today.
- `RequestDiagnostics` and `ExecutionContext` are driver-private from the SDK consumer's perspective.
- The cross-SDK design doc §10 implies `ExecutionContext::Hedging` is already dispatched and that `ExecutionContext` / `RequestDiagnostics` are public — neither is currently true.

---

## 3. Proposed additive methods on `DiagnosticsContext`

All three accessors are added to `DiagnosticsContext` in `azure_data_cosmos_driver` and exposed on `CosmosDiagnosticsContext` via the existing re-export (`sdk/cosmos/azure_data_cosmos/src/models/mod.rs:38`). No new public type is required by the recommended re-export option (§5, Option B).

### 3.1 `pub fn requested_regions(&self) -> Vec<&Region>`

```rust
/// Returns the regions to which this operation dispatched a request,
/// in dispatch order.
///
/// Each dispatched attempt contributes one entry. Duplicates are allowed:
/// the same region may appear more than once if it was dispatched
/// multiple times (e.g., a retry to the same region, or a hedge request
/// to a region that was also the primary).
///
/// Entries with no resolved region (pre-region-selection failures) are
/// skipped.
///
/// Order matches `RequestDiagnostics` insertion order in
/// `self.requests`, which is dispatch order.
pub fn requested_regions(&self) -> Vec<&Region> { ... }
```

- **Source:** `self.requests.iter()` — each `RequestDiagnostics` carries `region: Option<Region>`.
- **Filter:** include only entries with `Some(region)`.
- **Dedup:** **none**. Duplicates are allowed (cross-SDK contract; see internal spec §3.6).
- **Order:** source order (= dispatch order, since the `requests` collection is append-only).
- **Cost:** one `Vec<&Region>` allocation; entries are borrows into the existing `requests` collection.

### 3.2 `pub fn responded_regions(&self) -> Vec<&Region>`

```rust
/// Returns the regions from which this operation received a response,
/// in completion order.
///
/// Each completed `RequestDiagnostics` contributes one entry. Duplicates
/// are allowed: the same region may appear more than once if multiple
/// completed responses arrived from it (e.g., a late hedge response
/// after the hedge winner). `responded_regions().len() > 1` does NOT
/// imply more than one distinct region responded.
///
/// To dedupe, callers can use:
/// `let unique: Vec<&Region> = ctx.responded_regions().into_iter().collect::<std::collections::BTreeSet<_>>().into_iter().collect();`
/// or `.dedup()` (if pre-sorted), or a `HashSet`.
pub fn responded_regions(&self) -> Vec<&Region> { ... }
```

- **Source:** `self.requests.iter()`.
- **Filter:** include only entries with `Some(region)` AND `completed_at.is_some()` (a response actually arrived).
- **Dedup:** **none**.
- **Order:** **completion order**. The current `requests` collection is in *dispatch* order; producing completion order requires one of:
  - **(a)** the orchestrator appending in completion order (changes the existing append discipline — rejected because it would harm `requested_regions()` semantics);
  - **(b)** a stable sort by `completed_at` at accessor time (O(n log n) for a typically-small n);
  - **(c)** a separate `Vec<usize>` of indices into `requests` maintained in completion order.

  This spec **proposes (c)** — a small driver-private completion-order index — and lists this as Open Question (iv) in §10.

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
/// `hedge_diagnostics.as_ref().and_then(|hd| hd.strategy_config.as_ref())`
/// (see `HedgeDiagnostics` from the Cross-Region Hedging design spec).
pub fn hedging_started(&self) -> bool { ... }
```

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

- **Why NOT `hedge_diagnostics.is_some()` alone.** PR #4330's `HedgeDiagnostics` may be `Some` whenever a hedging strategy is *configured for this operation*, including the **primary-wins-under-threshold** case — exactly one request launched, threshold delay never elapsed, no fan-out. The cross-SDK contract (`internal-spec.md` §3.1; per-SDK AC2 in `public-spec-dotnet.md` / `public-spec-java.md` / `public-spec-python.md`) requires `hedging_started() == true` iff the SDK **actually dispatched to a hedge region**, i.e., `total_requests_launched >= 2` — not "strategy was active". Returning `true` for `total_requests_launched == 1` would contradict the cross-SDK contract.
- **Why NOT the `ExecutionContext::Hedging` predicate alone.** A future internal refactor to `HedgeDiagnostics` (e.g., a field rename, or a change of semantics around `total_requests_launched`) could silently cause the two predicates to drift out of sync. The disjunction is the **safe** definition: it returns `true` whenever *either* signal indicates fan-out occurred, and the mandatory invariant test in §8 enforces that the two signals stay equivalent after the implementation PR lands.
- **Pre-implementation behavior.** Until the hedging implementation PR lands (the follow-up to PR #4330), no production code populates `hedge_diagnostics` and no production code emits `ExecutionContext::Hedging`. Therefore `hedging_started()` returns `false` for all operations — which is the correct answer: no fan-out has happened.

### 3.4 Pre-region-selection failures

For all three accessors, a `RequestDiagnostics` whose `region` is `None` (pre-region-selection failure) is **skipped**. This matches `regions_contacted()` precedent and the cross-SDK contract.

---

## 4. The `Retry → OperationRetry` rename mechanism

The cross-SDK contract (.NET / Java / Python) renames the legacy `Retry` reason to `OperationRetry` for clarity (it distinguishes user-visible operation retries from transport-layer retries). Rust must align.

**Rust mechanics** (the cross-SDK design doc §10 proposes `#[deprecated]` re-export, which is **not a real Rust mechanism** — Rust has no enum-variant aliases; refuted here):

```rust
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionContext {
    Initial,
    #[deprecated(since = "<version>", note = "use `OperationRetry`")]
    Retry,
    #[serde(alias = "retry")]
    OperationRetry,
    TransportRetry,
    RegionFailover,
    Hedging,
    CircuitBreakerProbe,
}
```

Steps:

1. **Add `OperationRetry`** to `ExecutionContext` (the enum is already `#[non_exhaustive]`, so adding a variant is non-breaking at the type level).
2. **Mark `Retry`** with `#[deprecated(since = "<version>", note = "use `OperationRetry`")]`.
3. **Update every dispatch site** to construct `OperationRetry` instead of `Retry`:
   - `sdk/cosmos/azure_data_cosmos_driver/src/pipelines/operation_pipeline.rs` lines ~938, 940, 942, 1217–1348.
   - `sdk/cosmos/azure_data_cosmos_driver/src/pipelines/transport_pipeline.rs` line ~194.
4. **Apply `#[serde(alias = "retry")]`** on `OperationRetry` so older JSON payloads (emitted before the rename) still **deserialize**.

### 4.1 JSON wire format — **break ACCEPTED at the cross-SDK gate (Q5=B)**

With the enum already carrying `#[serde(rename_all = "snake_case")]`, serde derives variant names from their Rust spelling:

- Old variant `Retry` ⇒ wire shape `"retry"`.
- New variant `OperationRetry` ⇒ wire shape `"operation_retry"`.

This is a **wire-format break**: any downstream consumer that parses `ExecutionContext` JSON and looks for the string `"retry"` will see `"operation_retry"` instead, starting at the implementation PR.

**Deliberate decisions:**

- **`#[serde(alias = "retry")]`** is applied on `OperationRetry`. This preserves **back-deserialization** compatibility: older logs / payloads / persisted diagnostics that contain `"retry"` will still parse into `OperationRetry`.
- **`#[serde(rename = "retry")]` is NOT applied.** The cross-SDK gate (decision Q5=B in `human-gate-decision.json`) explicitly rejected preserving the historical wire shape. The new variant **serializes** as `"operation_retry"`. The break is intentional and surfaced in CHANGELOG entries on both crates (see §9 below) so downstream telemetry parsers can update.

### 4.2 Why no `#[deprecated]` re-export mechanism

Rust has no first-class "variant alias" or "deprecated variant alias that resolves to a new variant". The closest available mechanism — `#[deprecated]` on the variant itself — works for source-level deprecation only. Renaming a public enum variant in Rust requires a new variant plus dispatch-site updates; serde `alias` then provides JSON-deserialization continuity, and serde `rename` would (if applied) preserve serialized output shape. The gate rejected `rename`, leaving the wire-format break and `alias`-only back-compat.

---

## 5. Public-crate re-export decision

The accessors live on `DiagnosticsContext` in the driver crate. `DiagnosticsContext` is already re-exported as `azure_data_cosmos::CosmosDiagnosticsContext` (`sdk/cosmos/azure_data_cosmos/src/models/mod.rs:38`). The remaining question is whether callers ever need to *see* `ExecutionContext` directly.

| | **Option A — re-export `ExecutionContext`** | **Option B — project to primitives at accessor boundary (RECOMMENDED — gate Q6=A)** |
|---|---|---|
| Public surface | Adds `ExecutionContext` to `azure_data_cosmos` | No new public type |
| Pattern matching | Callers can `match` on per-request execution context | Callers see `bool` + `Vec<&Region>` only |
| Matches "No Model Sharing" (AGENTS.md) | No (re-exports driver-internal model) | **Yes** |
| Matches `DiagnosticsContext` precedent | Yes (PR #4376) | No (`DiagnosticsContext` is the only driver model re-exported) |
| Symmetry with .NET / Java / Python | The other SDKs expose `RequestedRegion { region, reason }` pairs | This option still allows a *new* SDK-public `RequestedRegion { region, reason: RequestedRegionReason }` wrapper, with `RequestedRegionReason` being a one-to-one mirror of `ExecutionContext` that lives in `azure_data_cosmos` (driver-private `ExecutionContext` stays private) |
| Effort to revisit | Easy: a future PR can add the re-export without breaking changes | Easy: a future PR can introduce the `RequestedRegion` wrapper without breaking changes |

**Recommendation: Option B.** Smallest public surface, honors the "No Model Sharing" principle, defers the per-region-reason exposure to a follow-up that can pick the wrapper-struct shape based on real customer feedback. The implementation PR adopts Option B as the baseline; Open Question (i) in §10 invites the team to confirm or override.

---

## 6. Reconciliation with PR #4330's `HEDGING_SPEC.md` §10

PR #4330 head SHA at the time this spec PR opens: **`3647b404e5366e1e91344ffa5a5929a7a6845c38`**.

`HedgeDiagnostics` (as proposed by PR #4330) — permalink to the exact wording in #4330's branch (SE-023 mitigation step (a)):
[https://github.com/Azure/azure-sdk-for-rust/blob/3647b404e5366e1e91344ffa5a5929a7a6845c38/sdk/cosmos/azure_data_cosmos_driver/docs/HEDGING_SPEC.md](https://github.com/Azure/azure-sdk-for-rust/blob/3647b404e5366e1e91344ffa5a5929a7a6845c38/sdk/cosmos/azure_data_cosmos_driver/docs/HEDGING_SPEC.md)

PR #4330 adds the following field to `DiagnosticsContext` (paraphrased from the linked spec; see permalink for exact wording):

```rust
pub struct DiagnosticsContext {
    // ... existing fields ...
    pub hedge_diagnostics: Option<HedgeDiagnostics>,
}

pub struct HedgeDiagnostics {
    pub strategy_config: Option<HedgingStrategyConfig>,
    pub regions_contacted: Vec<Region>,
    pub response_region: Option<Region>,
    pub total_requests_launched: usize,
    pub was_hedge: bool,
}
```

The two surfaces — the Rust-native `HedgeDiagnostics` field and the cross-SDK Hedging Detection API accessors — **coexist** on the same `DiagnosticsContext`. They serve different audiences (Rust-native rich detail vs. cross-SDK uniform shape) and they are computed from overlapping but distinct internal sources.

### 6.1 Reconciliation table

| Surface | Cross-SDK accessor (this spec)                              | Rust-native field (PR #4330)                                                                                              | Relationship after both PRs land |
|---|---|---|---|
| "Did fan-out happen?" | `hedging_started() -> bool`                                 | `hedge_diagnostics.map(\|hd\| hd.total_requests_launched >= 2).unwrap_or(false)`                                          | **Equivalent** after implementation PR; equivalence is asserted by the invariant test in §8 (SE-023). |
| "Was a hedging strategy configured?" | _not exposed_ (intentionally; see §3.3 doc-comment) | `hedge_diagnostics.is_some()` (or, more precisely, `.and_then(\|hd\| hd.strategy_config.as_ref()).is_some()`)             | `hedge_diagnostics.is_some()` is a **superset** of `hedging_started()` — true for primary-wins-under-threshold. |
| Regions tried | `requested_regions() -> Vec<&Region>` (dispatch order, duplicates allowed, includes non-hedge attempts) | `hedge_diagnostics.regions_contacted: Vec<Region>` (hedge-specific) | Different scope: `requested_regions()` always reflects every dispatched attempt (initial + retries + transport-retries + region-failovers + hedge fan-out); `HedgeDiagnostics::regions_contacted` reflects only hedge fan-out. Both are useful. |
| Regions that responded | `responded_regions() -> Vec<&Region>` (completion order, duplicates allowed) | `hedge_diagnostics.response_region: Option<Region>` (single winner) | Different shape: `responded_regions()` is a complete list (including late losers); `response_region` is the single winner. `responded_regions().first()` is the winner only if responses arrived strictly in completion order — see §10 Open Question (iii). |
| Reason per region | _not exposed in Option B_; available via `ExecutionContext` in Option A; available via a future `RequestedRegion { region, reason }` wrapper if the team wants per-region-reason semantics | _not exposed_ (PR #4330 does not break out per-region reason) | Cross-SDK contract requires per-region reason; deferred via Open Question (i). |

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

1. **PR #4330 merges.** This brings `HEDGING_SPEC.md` (and the design contract for `hedge_diagnostics: Option<HedgeDiagnostics>`) onto `release/azure_data_cosmos-previews`.
2. **This spec PR merges.** Adds `HEDGING_DETECTION_API_SPEC.md`. No production code change.
3. **Hedging implementation PR** (separate, future). Implements the orchestrator that populates `HedgeDiagnostics` **and** dispatches `RequestDiagnostics` with `ExecutionContext::Hedging`. Without this PR, both signals used by `hedging_started()` remain inactive — `hedging_started()` returns `false` for every operation, which is the correct answer (no fan-out has happened).
4. **Hedging Detection API implementation PR** (separate, future). Implements the three accessors specified here, plus the `Retry → OperationRetry` rename (§4), the re-export decision (§5), the test additions (§8), and the CHANGELOG entries (§9).

Steps 3 and 4 may be combined or kept separate at the implementer's discretion; this spec does not require either ordering between them, but the invariant test in §8 only becomes meaningful once Step 3 lands.

---

## 8. Test plan delta

The Hedging Detection API implementation PR (Step 4 in §7) MUST add:

### 8.1 Non-hedging operation tests (well-defined behavior)

In `sdk/cosmos/azure_data_cosmos/tests/emulator_tests/cosmos_items.rs` and `cosmos_query.rs`, add cases asserting:

- For a successful single-region read: `requested_regions().len() == 1`, `responded_regions().len() == 1`, `hedging_started() == false`.
- For an operation that retried (transient transport error) within a single region: `requested_regions()` contains the same region two or more times (duplicates allowed); `hedging_started() == false`.
- For an operation that failed before region selection (e.g., misconfigured client): `requested_regions().is_empty()`; `hedging_started() == false`.

These cases prove the accessors are **well-defined even when hedging is disabled / not yet implemented**.

### 8.2 SE-023 mandatory invariant test (added on the hedging implementation PR)

For every `DiagnosticsContext` produced by every emulator / fault-injection / live-multi-region test:

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
    "SE-023: hedging_started disjunction predicates are out of sync; \
     hedge_diagnostics={:?} requests={:?}",
    ctx.hedge_diagnostics, ctx.requests,
);
```

This test guards against silent inversion of the `hedging_started()` disjunction if `HedgeDiagnostics` is later refactored (field rename, `total_requests_launched` semantics change, etc.). It is the load-bearing CI guarantee behind §3.3's disjunction design.

### 8.3 Wire-format alias test

A serde round-trip test confirms:

- `serde_json::from_str::<ExecutionContext>("\"retry\"")` deserializes to `ExecutionContext::OperationRetry` (via `#[serde(alias = "retry")]`).
- `serde_json::to_string(&ExecutionContext::OperationRetry)` produces `"\"operation_retry\""` (the new wire shape; CHANGELOG-documented break).

---

## 9. CHANGELOG entries (drafted)

Both crate CHANGELOGs receive entries on the hedging-implementation PR. The wire-format break on `ExecutionContext` MUST appear in both.

### 9.1 `sdk/cosmos/azure_data_cosmos/CHANGELOG.md`

```markdown
### Features Added

- `CosmosDiagnosticsContext` now exposes the cross-SDK Hedging Detection API:
  `hedging_started() -> bool`, `requested_regions() -> Vec<&Region>`,
  `responded_regions() -> Vec<&Region>`. See
  `sdk/cosmos/azure_data_cosmos/docs/HEDGING_DETECTION_API_SPEC.md`.

### Breaking Changes

- **Wire-format change on `ExecutionContext` JSON.** The reason previously
  serialized as `"retry"` now serializes as `"operation_retry"`
  (renamed `Retry` → `OperationRetry` to align with the cross-SDK
  Hedging Detection API). Back-deserialization of the legacy `"retry"`
  string is preserved via `#[serde(alias = "retry")]`. Downstream
  telemetry parsers that match on the literal `"retry"` MUST be updated
  to accept `"operation_retry"` as well.
- `ExecutionContext::Retry` is now `#[deprecated]`; use `OperationRetry`.
```

### 9.2 `sdk/cosmos/azure_data_cosmos_driver/CHANGELOG.md`

```markdown
### Features Added

- `DiagnosticsContext` gains three accessors for cross-SDK Hedging
  Detection: `hedging_started`, `requested_regions`, `responded_regions`.
  Spec: `sdk/cosmos/azure_data_cosmos/docs/HEDGING_DETECTION_API_SPEC.md`.
- `ExecutionContext::OperationRetry` is added (preferred over the
  deprecated `Retry`).

### Breaking Changes

- **Wire-format change on `ExecutionContext` JSON.** `Retry` is renamed
  to `OperationRetry`. With `#[serde(rename_all = "snake_case")]`, the
  serialized form changes from `"retry"` to `"operation_retry"`.
  `#[serde(alias = "retry")]` on `OperationRetry` preserves
  back-deserialization of older diagnostics payloads. Downstream
  consumers parsing the JSON form MUST be updated. The break was
  accepted at the cross-SDK design gate (Hedging Detection API,
  decision Q5=B) in preference to a permanent `#[serde(rename]` that
  would have indefinitely shadowed the Rust spelling.

### Other Changes

- `ExecutionContext::Retry` is marked `#[deprecated]`. The
  `ExecutionContext` enum is already `#[non_exhaustive]`, so adding
  `OperationRetry` is non-breaking at the type level.
```

A cspell allowlist update (`OperationRetry`, `RequestedRegion`, `responded_regions`) accompanies the implementation PR.

---

## 10. Open questions

1. **Re-export decision — Option A vs Option B.** Spec recommends **Option B** (project to primitives at the accessor boundary; `ExecutionContext` stays driver-private); the cross-SDK gate decision Q6=A endorses Option B. Confirm or override at PR review.
2. **`requested_regions()` and HTTP/2 sharded transport.** If a single logical request is sharded across N HTTP/2 sub-attempts to the same region, does `requested_regions()` contribute N entries or 1? This spec proposes 1 (aggregate at the `RequestDiagnostics` level, not the transport level), but the answer depends on whether the orchestrator emits one `RequestDiagnostics` per logical request or per transport attempt.
3. **`responded_regions()` order vs `HedgeDiagnostics::response_region`.** §6.1 notes that `responded_regions().first()` is the hedge winner only if responses are appended strictly in completion order. The proposed implementation (separate completion-order index, §3.2 option (c)) makes this true by construction; an alternative implementation (sort-on-read) would also work. Pick one in the implementation PR.
4. **Aggregation semantics under PATCH.** PATCH read-modify-write may internally submit two operations (read + write). Should the resulting `DiagnosticsContext` aggregate both phases under `requested_regions()` / `responded_regions()`, or expose them separately? Out of scope for this spec; called out for the implementation PR.

**Resolved at the cross-SDK gate (no longer open):**

- ~~_JSON wire shape of `Retry` / `OperationRetry`._~~ **Resolved (Q5=B):** wire-format break accepted; new variant serializes as `"operation_retry"`; `#[serde(alias = "retry")]` preserves back-deserialization; CHANGELOG entries on both crates are explicit.
- ~~_Sibling spec file vs. amendment to PR #4330's `HEDGING_SPEC.md` §10._~~ **Resolved (Q4=A):** sibling spec file (this document).
- ~~_Rust scope — full implementation or spec-only._~~ **Resolved (Q6=A):** spec-only; sequenced after PR #4330 merges and before the hedging implementation PR.

---

## 11. References

- Cross-SDK design doc: "Hedging Detection API" (.NET / Java / Python / Rust).
- PR #4330: [Cosmos: Adds Cross-Region Hedging Design Spec to Driver Crate](https://github.com/Azure/azure-sdk-for-rust/pull/4330) (head SHA `3647b404e5366e1e91344ffa5a5929a7a6845c38` at the time this spec PR opens).
- PR #4376 precedent for re-exporting `DiagnosticsContext` as `CosmosDiagnosticsContext`.
- `sdk/cosmos/azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs` (existing `DiagnosticsContext`, `RequestDiagnostics`, `ExecutionContext`).
- `sdk/cosmos/azure_data_cosmos_driver/src/pipelines/operation_pipeline.rs` and `transport_pipeline.rs` (dispatch sites).
- `sdk/cosmos/azure_data_cosmos/src/models/mod.rs` (public re-export surface).
