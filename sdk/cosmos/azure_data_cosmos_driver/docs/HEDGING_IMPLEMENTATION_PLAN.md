# Hedging Phase 1 — Implementation Plan

**Status:** Approved plan, ready to execute
**Spec:** [`HEDGING_SPEC.md`](./HEDGING_SPEC.md)
**Scope:** Phase 1 only (point-read hedging + PPCB feedback + hub-region shared latch)
**Branch:** `users/kundadebdatta/3357_implement_cross_regional_hedging`

---

## Current state (verified against the codebase)

- [`operation_pipeline.rs`](../src/driver/pipeline/operation_pipeline.rs) has the 7-stage operation loop with STAGE 7 dispatching `Complete` / `FailoverRetry` / `SessionRetry` / `Abort` — but **no `Hedge` arm**.
- [`components.rs`](../src/driver/pipeline/components.rs) `OperationAction` has 4 variants; the existing comment notes `Hedge` is the planned addition.
- `OperationRetryState.hub_region_processing_only` exists; comments already mention the §9.6 shared-latch extension is pending.
- [`retry_evaluation.rs`](../src/driver/pipeline/retry_evaluation.rs) `evaluate_transport_result` exists and `build_session_retry_state` already latches the per-state flag.
- [`driver_options.rs`](../src/options/driver_options.rs) has `preferred_regions`; no `availability_strategy` on either `DriverOptions` or `OperationOptions`.
- No `HedgeThreshold` / `HedgingStrategy` / `AvailabilityStrategy` / `HedgeDiagnostics` types exist.

Phase 1 is purely additive against the existing pipeline.

---

## Implementation broken into 7 parts

### Part 1 — Public config types & options plumbing *(pure data, no behavior)*

**Files (new):**

- `src/options/availability_strategy.rs` — `HedgeThreshold` (newtype, spec §4.1), `HedgingStrategy`, `AvailabilityStrategy` enum.

**Files (modified):**

- `src/options/driver_options.rs` — `availability_strategy: Option<AvailabilityStrategy>` field + `with_availability_strategy(..)` builder method.
- `src/options/operation_options.rs` — same field + builder method (spec §4.3).
- `src/options/env_parsing.rs` — read `AZURE_COSMOS_HEDGING_THRESHOLD_MS` and `AZURE_COSMOS_HEDGING_DISABLED` (spec §4.4).
- `src/options/mod.rs` — re-exports.

**Tests:** `hedge_threshold_rejects_zero`, `hedge_threshold_accepts_positive`, env-var parsing happy/sad paths, builder round-trips. No pipeline touched.

---

### Part 2 — Diagnostics types *(also pure data)*

**Files (new):**

- `src/driver/pipeline/hedging_diagnostics.rs` — `HedgeDiagnostics`, `HedgingStrategyConfig` per spec §10.1 including the constructor used in the happy path (`primary_only`).

**Files (modified):**

- `src/diagnostics/mod.rs` — add `hedge_diagnostics: Option<HedgeDiagnostics>` to `DiagnosticsContext` (spec §10.2).
- `src/driver/pipeline/mod.rs` — wire the new module.

**Tests:** struct construction + `Debug`/`Clone` round-trip. Nothing wired into the pipeline yet.

---

### Part 3 — Eligibility + final-result classification *(pure functions, fully unit-testable)*

**Files (modified):** `src/driver/pipeline/retry_evaluation.rs` (or new sibling `hedging_eligibility.rs` if it gets large).

**Adds:**

- `is_final_result(status)` per spec §7.1.
- `should_hedge(strategy, operation, account_state)` per spec §5.1 — decision-matrix rows 1–6; Phase 1 hard-codes the phase-allowed set to `{Document}` + `{Read}`.
- `resolve_availability_strategy()` — implements the spec §11.3.1 priority chain (op > client > env > driver default > none), including the §5.2 default-on activation with threshold = `min(1000ms, request_timeout / 2)`.
- `build_secondary_routing(primary, user_excluded, regions)` per spec §6.3.

**Tests (all from spec §15.1):**

- All 8 `should_hedge_*` tests
- All 6 `is_final_result_*` tests
- `alternate_region_pin_excludes_primary`, `alternate_region_pin_unions_user_excludes`

---

### Part 4 — `OperationAction::Hedge` variant + STAGE 7 dispatch + `execute_hedged()` *(the core)*

**Files (modified):**

- `src/driver/pipeline/components.rs` — add `Hedge { secondary_routing: RoutingDecision, threshold: HedgeThreshold }` variant to `OperationAction`.
- `src/driver/pipeline/retry_evaluation.rs` — extend `evaluate_transport_result` to return `OperationAction::Hedge` when `should_hedge()` says yes and a secondary routing decision can be built (spec §6.1).
- `src/driver/pipeline/operation_pipeline.rs`:
  - Add STAGE 7 arm dispatching to `execute_hedged()`.
  - Implement `execute_hedged()` per the spec §6.4 pseudocode — `tokio::select!`-based, no `tokio::spawn`, no `CancellationToken`, structural cancellation.
  - Helpers: `classify`, `decorate`, `harvest_app_cancel_error` (with `HARVEST_WINDOW = 50ms` per spec §6.5 #7).
  - Hook `ExecutionContext::Hedging` onto the secondary `transport_request`.

**Critical invariants enforced:**

- Zero-overhead happy path (spec §6.5 #3) — secondary request is **not built** until the threshold elapses.
- Max 2 concurrent attempts (spec §6.5 #1).
- App-cancel diagnostics harvest (spec §6.5 #7).

**Tests:**

- `app_cancel_preserves_hedge_diagnostics`
- `zero_overhead_happy_path_no_allocs` (`dhat-rs`-backed)
- Local fault-injected pipeline tests for primary-wins / secondary-wins / both-transient

---

### Part 5 — Hub-region shared latch (spec §9.6)

**Files (modified):**

- `src/driver/pipeline/components.rs` — add `shared_hub_region_latch: Option<Arc<AtomicBool>>` field + `with_shared_hub_region_latch()` builder to `OperationRetryState`.
- `src/driver/pipeline/retry_evaluation.rs::build_session_retry_state` — when the 4-condition latch fires, also `Release`-store on the shared latch if present.
- `src/driver/pipeline/operation_pipeline.rs::apply_hub_region_header` — emit header when **either** per-state or shared-latch (`Acquire`) is set.
- `src/driver/pipeline/operation_pipeline.rs::execute_hedged()` — after threshold fires, build the `Arc<AtomicBool>` gated by spec §9.6.3 eligibility (data-plane ∧ single-master ∧ alternate-spawned); thread it into both primary's and secondary's retry state.

**Tests (all from spec §15.1):**

- `shared_hub_region_latch_initialized_when_eligible`
- `shared_hub_region_latch_none_on_zero_overhead_happy_path`
- `shared_hub_region_latch_none_on_multi_master_or_metadata`
- `shared_hub_region_latch_propagates_first_1002_across_hedges`
- `shared_hub_region_latch_no_1002_emits_no_header`

---

### Part 6 — PPCB hedge-win feedback (spec §9.5)

Cross-module dependency: needs the `LocationStateStore` (PPCB module) to expose `record_consecutive_hedge_win(partition, primary_region)`. Spec §9.5 calls this out as a "co-designed with PPCB owner before Phase 1 ships" item.

**Sub-plan:**

- **6a:** Add a no-op `record_consecutive_hedge_win()` stub on `LocationStateStore` so the hedging side compiles. Open a tracking issue for the PPCB-side semantics (threshold N, reset rule, state transition to `Unhealthy`).
- **6b:** Wire the call from `execute_hedged()` on every alternate-win (already in the Part 4 pseudocode).
- **6c:** When the PPCB-side lands, enable the integration tests in spec §15.2 (`hedging_alternate_wins_trip_ppcb`) and §15.3 (`hedging_ppcb_feedback_cross_region`).

**Tests (unit-only in this PR):** `record_hedge_win_increments_ppcb_counter`, `primary_win_resets_hedge_win_counter` — verified via a mock/double of `LocationStateStore`.

---

### Part 7 — Integration / fault-injection / multi-region live tests

Separately landed after Parts 1–6 are in: the full spec §15.2 fault-injection table (~12 tests) and the §15.3 live tests. Split from the implementation PRs because fault-injection harness changes tend to be noisy.

---

## Sequencing rationale

| Part | Depends on | Risk | Reviewability |
|---|---|---|---|
| 1 | — | Low (pure data) | Small |
| 2 | — | Low (pure data) | Small |
| 3 | 1 | Low (pure functions) | Medium |
| 4 | 1, 2, 3 | **High** (touches hot path) | Large |
| 5 | 4 | Medium | Medium |
| 6 | 4 + PPCB owner | Medium (cross-module) | Small |
| 7 | 1–6 | Low | Medium |

Parts 1–3 are independently mergeable and unblock Part 4. Parts 5 and 6 can land in parallel after Part 4. Part 7 is the closing integration sweep.

---

## Decisions captured

| # | Question | Decision |
|---|---|---|
| 1 | One PR or seven? | _TBD_ |
| 2 | PPCB callback (Part 6): stub now or wait? | _TBD_ |
| 3 | `dhat-rs` allocation test in Part 4 or Part 7? | _TBD_ |
| 4 | Continue on current branch or cut a fresh one? | _TBD_ |

---

## Cross-references

- Spec: [`HEDGING_SPEC.md`](./HEDGING_SPEC.md)
- Transport pipeline contract: [`TRANSPORT_PIPELINE_SPEC.md`](./TRANSPORT_PIPELINE_SPEC.md) §3.4 (evaluator) and §4.1/§4.2 (STAGE 7 dispatch)
- Hub-region header baseline: PR #4389; .NET shared-context fix: [`azure-cosmos-dotnet-v3#5815`](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5815)
