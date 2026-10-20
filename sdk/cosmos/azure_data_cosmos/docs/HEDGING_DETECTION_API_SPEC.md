# Hedging Detection API — Spec

**Status:** Implemented on `main`.
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
| `DiagnosticsContext::requested_regions` | `-> Vec<RequestedRegion>` | Regions dispatched to, in **dispatch order**, duplicates allowed, each tagged with a reason. Bounded (§4.5). |
| `DiagnosticsContext::responded_regions` | `-> Vec<&Region>` | Regions that produced a **service reply**, in **completion order**, duplicates allowed. Bounded (§4.5). |
| `DiagnosticsContext::total_requested_regions` | `-> usize` | Exact dispatch count, including entries elided by the bound. |
| `DiagnosticsContext::total_responded_regions` | `-> usize` | Exact reply count, including entries elided by the bound. |
| `RequestedRegion` | `{ region, reason }` | A dispatched region paired with the reason it was chosen. |
| `RequestedRegionReason` | enum | Why the SDK dispatched to a region; `#[non_exhaustive]`. |

These build on the existing per-operation building blocks and coexist with the
Rust-native `HedgeDiagnostics` surface (see §5); the two are complementary.

---

## 2. Building blocks (already public on `main`)

| Item | Signature | Notes |
| --- | --- | --- |
| `DiagnosticsContext` | re-exported as `azure_data_cosmos::DiagnosticsContext` | The per-operation diagnostics handle. |
| `DiagnosticsContext::requests` | `-> Arc<Vec<RequestDiagnostics>>` | Retained per-attempt records in dispatch order — **not** a guaranteed-complete append-only history: under a `429`/`410` retry storm the list is bounded/compacted (see `max_request_diagnostics`, which can drop or reorder entries). A structurally-dropped hedge loser leg *is* represented for every attempt it completed (rescued through the hedge journal, §4.3), but an attempt it left in flight is absent. Cloning the `Arc` is a cheap atomic increment. |
| `DiagnosticsContext::hedge_diagnostics` | `-> Option<&HedgeDiagnostics>` | An **optional retained race outcome**, not a configuration probe. `Some` when a hedge race recorded a terminal outcome (including primary-wins-under-threshold). `None` when hedging was not selected for the operation, when a configured strategy found the operation ineligible, **and** on the both-transient→failover path, where a terminal outcome is deliberately left unset so a later successful retry does not carry a misleading `BothTransient` state. |
| `DiagnosticsContext::regions_contacted` | `-> Vec<Region>` | Distinct regions **deduplicated in first-contact (failover) order — not sorted**, captured from the full attempt list before compaction. |
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

The hedging orchestrator/dispatch is **landed** on `main`: it emits
`ExecutionContext::Hedging` for alternate legs and populates `HedgeDiagnostics`
(design: [`HEDGING_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/HEDGING_SPEC.md)).

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
    OperationRetry, // was: Retry
    TransportRetry,
    Hedging,
    RegionFailover,
    CircuitBreakerProbe,
}
```

`Retry` has been removed and replaced by `OperationRetry` so the operation-level
retry reason is clearly distinct from the transport-level `TransportRetry`. The
hand-written `ExecutionContext::as_str()` and every dispatch site
(`operation_pipeline.rs`, `transport_pipeline.rs`, `cosmos_driver.rs`) are updated
accordingly. Telemetry parsers that matched the literal `"retry"` execution context
must update; serialized output now emits `"operation_retry"` instead.

---

## 4. Detection recipes → API

### 4.1 Reason mapping — `RequestedRegionReason`

`requested_regions()` tags each dispatched region with a `RequestedRegionReason`,
projected from the driver-internal `ExecutionContext` via a **total**
`From<ExecutionContext>` mapping:

| `ExecutionContext` | `RequestedRegionReason` |
| --- | --- |
| `Initial` | `Initial` |
| `OperationRetry` | `OperationRetry` |
| `TransportRetry` | `TransportRetry` |
| `Hedging` | `Hedging` |
| `RegionFailover` | `RegionFailover` |
| `CircuitBreakerProbe` | `CircuitBreakerProbe` |

The mapping is total (no wildcard arm) so it fails to compile if a new
`ExecutionContext` variant is added without a corresponding reason.

### 4.2 Did fan-out happen? — `hedging_started()`

`true` iff at least one hedge arm was actually dispatched. This is `false` — not
an error — when the primary returns before the hedging threshold elapses, even
though a hedging strategy was active.

There is **no** accessor for "was a strategy configured?", and
`hedge_diagnostics().is_some()` is not one: it is `None` for a configured but
ineligible operation, and on the both-transient→failover path (§5). Reading it
as a configuration probe would make consumers conclude hedging was disabled when
it was not.

The result is the disjunction of two equivalent fan-out signals —
`HedgeDiagnostics::alternate_region().is_some()` and any request tagged
`ExecutionContext::Hedging`. Either alone is sufficient; the disjunction stays
correct if a future change ever drifts one signal.

### 4.3 Regions dispatched to, with reason — `requested_regions()`

Retained attempts in dispatch order, duplicates preserved (a region dispatched
twice appears twice), entries with no resolved region skipped. The initial
attempt is included and tagged `RequestedRegionReason::Initial`.

**Materialized, not derived.** All three accessors are computed once in
`DiagnosticsContextBuilder::complete()` from the **full, pre-compaction** attempt
list plus a dispatch-time hedge fan-out log, then stored as fields — the same
pattern `regions_contacted()` already used. Reading them is a field read. This
matters because the retained `requests()` list is *not* the dispatch history:

- a clean hedge race structurally drops the losing leg's sub-builder, so only the
  attempts rescued through the hedge journal survive, and any attempt the loser
  left in flight is genuinely gone (see §5 and `HedgeDiagnostics`);
- a `429`/`410` retry storm compacts `requests()` down to
  `max_request_diagnostics`, dropping whole buckets;
- `aggregate_sub_operations` keeps only **one** representative
  `HedgeDiagnostics` for a multi-round-trip operation.

**Hedge journal.** A hedge leg records into its own private sub-builder, and the
race structurally drops the loser's future — so the loser's records must be
rescued out-of-band. Every attempt a leg *completes* is mirrored, at the moment
it reaches a terminal state, into an operation-scoped hedge journal shared by the
parent and all legs; the winner's copies are discarded when its sub-builder is
merged, and `complete()` folds the remaining copies back in and stable-sorts the
union by dispatch instant. The result is exactly one record per attempt in true
global dispatch order, no matter which leg won. Consequently a dropped leg that
had already received a reply (a `429` it was backing off from, say) still
contributes its region, its status and its RU charge to `requested_regions()`,
`responded_regions()` and the charge totals.

An attempt that was still **in flight** when its leg was cancelled observed no
reply, so it is deliberately *not* recovered — reporting it would invent a
response that never arrived.

**Hedge fan-out.** Each fan-out is additionally recorded on the *parent* builder
at dispatch time, before the race runs. This is a fallback for the one case the
journal cannot cover: `select` polls the primary first, so a primary that is
already `Ready` causes the alternate to be dropped **without ever being polled**,
meaning it never reaches `start_request` and has no attempt of its own to
mirror. Such a leg is spliced in as a synthetic entry, positioned by its dispatch
instant, so both legs always appear: the primary leg tagged with the reason it
was **actually** dispatched under (`Initial` for a first attempt, or the
failover/session reason when a hedge upgraded a retry), and the alternate leg
tagged `Hedging`. A leg that did dispatch describes itself through its own
(surviving) attempts and is never double-counted. A leg that never produced a
service reply has no `responded_regions()` entry.

For an aggregated operation (e.g. `PATCH`) stitched from multiple
sub-operations, every sub-operation's fan-out is preserved: the aggregated list
is the concatenation of each sub-operation's own materialized list, in
sub-operation order.

This is distinct from `regions_contacted()`, which is *deduplicated* in
first-contact order and so answers "which distinct regions did we touch?" rather
than "what did we dispatch, in what order, and why?".

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

### 4.5 Both histories are bounded

Materializing from the *pre-compaction* attempt list is what makes a dropped
hedge leg survive, but taken alone it would make the two histories grow with
attempt count — a 410/429 retry storm would produce an O(attempts) artifact even
though the retained `requests()` list stays capped. That violates the bounded-size
guarantee in the driver's `DIAGNOSTICS-CONTRACT.md` §8, which requires every
materialized representation to have an upper bound independent of attempt count,
and it would flow straight into span attributes.

Both histories are therefore capped at `max_request_diagnostics` (default 512,
minimum 16) at finalization, and re-capped in `aggregate_sub_operations` — the
aggregate is an independent unbounded path, since a PATCH conflict loop adds a
sub-operation per retry.

The elision keeps the **head and tail** of the history and drops the repetitive
middle, mirroring the "first and last of each run" policy the contract already
applies to attempt compaction. The head preserves the initial dispatch and any
early hedge fan-out; the tail preserves where the operation finally landed.

Because the cap is applied both per sub-operation and again to the concatenated
aggregate, a long enough PATCH conflict loop is bounded **twice** — the aggregate
keeps the head and tail of a list whose own entries are already head/tail
extracts. This compounding is deliberate: it costs some middle detail that was
already elided once, and in exchange it preserves the two properties consumers
actually assert on — the operation's first dispatch and where it finally landed —
under a hard bound that holds no matter how many sub-operations run. Raising the
cap would not remove the compounding, only move the threshold at which it starts,
while making the worst-case artifact proportionally larger.

Truncation is never silent. `total_requested_regions()` / `total_responded_regions()`
report the exact pre-truncation counts, so `requested_regions().len() < total_requested_regions()`
detects an elision. On the span, the matching `*_total` attribute is emitted only
when the history was truncated, so the normal path carries no redundant integer.

---

## 5. Reconciliation with `HedgeDiagnostics`

The Hedging Detection API and the Rust-native `HedgeDiagnostics`
([PR #4330](https://github.com/Azure/azure-sdk-for-rust/pull/4330) design,
[#4432](https://github.com/Azure/azure-sdk-for-rust/pull/4432) implementation)
coexist on the same `DiagnosticsContext` and serve different audiences.

| Question | Hedging Detection API | Rust-native `HedgeDiagnostics` |
| --- | --- | --- |
| Did fan-out happen? | `hedging_started()` — from the fan-out log | `alternate_region().is_some()` — equivalent only when a terminal outcome was retained |
| Was a strategy configured? | *(not derived)* | *(not derived — see below)* |
| Regions tried | `requested_regions()` (every dispatch, with reason) | `primary_region()` + `alternate_region()` (one race's legs only) |
| Regions that responded | `responded_regions()` (full list, completion order) | `response_region()` (single winner) |
| Race outcome | *(not derived)* | `terminal_state()` (authoritative, when retained) |

Neither surface answers "was hedging configured?". `hedge_diagnostics()` is an
optional *retained race outcome*: it is `None` for a configured-but-ineligible
operation, and it is deliberately left unset when a both-transient race
continues to a successful failover, so that a later successful retry does not
carry a misleading `BothTransient` state. On that path a dispatched hedge leg
still makes `hedging_started()` `true` while `hedge_diagnostics()` is `None` —
the two are not interchangeable.

The Detection API deliberately does **not** read `hedge_diagnostics` to answer
its three questions. Aggregation keeps only one representative `HedgeDiagnostics`
for a multi-round-trip operation, so deriving from it would silently under-report
every other sub-operation's fan-out — and the both-transient path would
under-report fan-out entirely. The fan-out log is per-builder, is written at
dispatch time, and survives aggregation intact.

`main`'s `HedgeDiagnostics` classifies the race via `terminal_state` /
`alternate_region` (there is no `total_requests_launched` counter), so "fan-out
happened" is `alternate_region().is_some()` and "the alternate won" is
`matches!(terminal_state(), HedgeTerminalState::AlternateWon)`. Consult
`terminal_state()` for hedge win-rate; do not infer an alternate win from the
presence of `alternate_region()` alone (several terminal states still record an
alternate region).

### 5.1 Consequence for the observability surfaces

All three SDK emission surfaces — the sampled root span, the sampled log line,
and the opt-in `azure.cosmosdb.client.operation.hedged` counter — decide
"did this operation hedge?" from `hedging_started()`, never from
`hedge_diagnostics().is_some()`. Gating on the latter would silently undercount
every both-transient race that was subsequently resolved by a failover attempt,
which is exactly the population an operator most wants to see.

The per-outcome fields (`hedge_region`, `hedge_terminal_state`) still require
`hedge_diagnostics`, and are simply omitted when it is absent rather than
emitted as empty strings. The counter is the one exception: its
`hedge_terminal_state` dimension carries the `unresolved` sentinel instead of
being dropped, so every data point on the counter has the same attribute set
and `group by hedge_terminal_state` never fragments the time series.

---

## 6. Future work

The three accessors return owned/borrowed collections computed on demand from
the append-only attempt list, so callers allocate only when they read a derived
collection. If `ExecutionContext` becomes a prominent part of the public
detection surface it could be renamed to something friendlier (e.g.,
`RequestPurpose` / `RequestIntent`); that rename is out of scope here.

[`DiagnosticsContext`]: https://docs.rs/azure_data_cosmos/latest/azure_data_cosmos/
