# Deferred, Threshold-Gated Diagnostics Capture (prototype)

> **Status: prototype / for discussion.** This document describes an opt-in diagnostics
> acquisition front-end under `azure_data_cosmos_driver::diagnostics::capture`. It is a cheap,
> append-only, lock-free hot-path recorder with an operation-end gate that — when it decides
> diagnostics are worth keeping — materializes the **canonical**
> [`DiagnosticsContext`]: the same diagnostics type the rest of the driver produces and returns.
> There is one diagnostics model, not a parallel one. Capture is **off by default**.

## 1. Problem & goals

Request-focused diagnostics across SDKs tend to share three shortcomings:

1. **Eager cost.** Diagnostics are materialized on every call, including the overwhelming
   majority of fast successes that are never read.
2. **Fan-out blindness.** Query/routing operations spawn many parallel sub-requests (per
   partition / feed range) that a request-centric model struggles to represent.
3. **One verbosity for everyone.** No cheap-by-default / full-on-demand split.

We want **outcome-aware** diagnostics: pay ~nothing on a fast success, full fidelity exactly when
it's useful (errors / slow ops / explicit request).

Goals:

- **G1** — Hot path is write-preferred, compact, append-only, minimal-allocation (pooled). The
  cost paid on a fast success is ~free.
- **G2** — Collection/materialization is deferred off the hot path.
- **G3** — A configurable gate evaluated at operation end decides whether to build at all:
  latency threshold + on-error, plus Off/Always modes. If not wanted, drop cheaply.
- **G4** — When the gate fires, the captured log materializes the **canonical**
  [`DiagnosticsContext`] (one model), including hedging structure for multi-region operations.
- **G5** — Opt-in and default-`Off`: an unconfigured client pays nothing and behaves unchanged.

## 2. Design exploration & benchmarks

Four capture strategies were prototyped and benchmarked on identical scenarios (single success,
retry-then-success, error, and fan-out with N children):

| Design | What it produces | One-line trade |
|---|---|---|
| **Eager JSON** | A nested JSON object, built on every call | Most familiar, best OpenTelemetry fit, but the most expensive and the largest output. |
| **Binary Span Tree** | A small binary blob (decode to JSON with a tool) | Full call tree cheaply; dropped for free on success; smallest full-fidelity format. |
| **Tiered Hybrid** | A tiny summary by default; full binary on error/on demand | Looks like today's request diagnostics; cheap happy path; never loses detail. |
| **Deferred Gated Capture** ⭐ | Nothing on a fast success; build only when slow or errored | Append-only capture, then *decide at the end* whether to build. Cheapest happy path. |

**Headline:** on a fast success the Deferred Gated design pays only the append + return-to-pool
cost and builds nothing — roughly an order of magnitude cheaper than building a diagnostics object
every time. Output collapses cleanly: a dropped fast success produces **0 B**.

This PR implements the **Deferred Gated Capture** design and wires it to build the **canonical
`DiagnosticsContext`** when the gate fires. Measured numbers from this crate (criterion, dev
profile; see [§7](#7-test--bench-results)):

| Path | Cost (criterion) |
|---|---|
| Opt-in, gate drops fast success | **~0.4 µs** (start + append + gate + return-to-pool) |
| `Mode::Off` (even if a stream was appended) | **~0.37 µs** (gate drops without building) |
| Gate fires, build `DiagnosticsContext` | **~5 µs** (parse + replay onto the builder, only past the gate) |

> The real driver default (`Off`) constructs **no recorder at all**, so the default-Off cost on
> the hot path is zero — the `Mode::Off` row above is the upper bound for code that *did* record.

Output size — a dropped fast success serializes nothing (**0 B**); a built `DiagnosticsContext`
serializes to the driver's standard JSON (see the examples in [§7](#7-test--bench-results)).

## 3. Comparison with .NET `CosmosDiagnostics`

.NET V3 ships `CosmosDiagnostics`: a request-focused object with a top-level `Summary` histogram
over a nested handler → transport → `StoreResult` tree. Conceptually:

- Its **per-request tree** maps onto this driver's existing
  [`RequestDiagnostics`](crate::diagnostics::RequestDiagnostics) list inside `DiagnosticsContext` —
  which is exactly what the gate builds, so there is no second model to reconcile.
- Its **once-per-diagnostics Client Configuration / User-Agent** maps to the
  context's client-level metadata, populated only when the gate builds.
- Its **`Summary` roll-up** corresponds to `DiagnosticsContext`'s `Summary` serialization verbosity
  (`DiagnosticsVerbosity::Summary`) — a view over the one model, not a separate object.

The captured field set is seeded from the signals a support investigation branches on first:
final/per-attempt status, error classification (incl. Cosmos sub-status), service request id
(activity id), request charge (RU), retry/throttle counts, total elapsed, and — for hedged
operations — per-region legs with the winning leg and terminal state.

## 4. The chosen design — Deferred Gated Capture

```text
HOT PATH (per attempt, write-preferred, lock-free, no Mutex):
  start   -> rent Vec<u8> from a bounded pool; write the op header
  attempt -> append TLV: exec-context, region, endpoint, status, sub-status,
                         service-request-id, RU, request_sent, timing
  hedge   -> append TLV: terminal state, primary/alternate/winning region   (multi-region)
  end     -> GATE: build? (Always | elapsed > latency_threshold | (error && capture_on_error))
               no  -> clear + return buffer to pool                          (~free)
               yes -> parse log once -> replay onto DiagnosticsContextBuilder
                                     -> DiagnosticsContext  (the canonical type)
```

Components (`azure_data_cosmos_driver::diagnostics::capture`):

- **`LogPool`** (`pool.rs`) — a shared, **bounded** pool of reusable buffers. Rent/return happen
  only at operation boundaries; oversized buffers (wide fan-out) are dropped rather than pinned.
- **`DiagnosticsRecorder` / `AttemptRecord`** (`recorder.rs`) — an operation-layer-owned,
  **lock-free `&mut`** append recorder. It is **cancellation-safe** (its `Drop` returns the buffer
  if `finish` never runs) and **panic-safe** (a partially written buffer is cleared, never
  poisoning a reused buffer). Elapsed time uses `std::time::Instant`. Concurrent hedge legs /
  fan-out children are captured as plain values by their own tasks and merged at the operation
  layer on join, so per-task capture stays lock-free.
- **`DiagnosticsPolicy` / `Mode`** (`gate.rs`) — the gate: `Off` (default; truly zero),
  `Threshold` (build on error or over a latency threshold), `Always`. `finish()` drops cheaply or
  builds a `DiagnosticsContext`.
- **`build_context`** (`context.rs`) — past the gate, parses the captured log once and replays it
  onto the existing `DiagnosticsContextBuilder`: each attempt becomes a `RequestDiagnostics` with
  its `ExecutionContext` (Initial / Retry / Hedging / …), region, endpoint, status, sub-status,
  request charge and request-sent signal; a hedged operation additionally attaches a
  `HedgeDiagnostics` describing the region legs and terminal state.

There is **no parallel `Summary`/`Rendered` type and no separate binary wire format** — the gate's
output is the same `DiagnosticsContext` the driver already returns.

## 5. Scope of this change

- **Opt-in, default `Off`** via `DriverOptions::with_capture_diagnostics_policy(...)`. With the
  default, no recorder is constructed and there is **no behavior change**.
- **Re-homed under `diagnostics::capture`** so it coexists with the existing `DiagnosticsContext`
  and friends; nothing in the shipping diagnostics module is removed or altered.
- **Builds the canonical `DiagnosticsContext`.** When the policy is enabled, the gate materializes
  a `DiagnosticsContext` and attaches it to the response via
  `CosmosResponse::capture_diagnostics()`. On a terminal error the built context's JSON is emitted
  via `tracing`.
- **No changes to `azure_core` / `typespec_client_core`.**
- Tests, two example generators (`tests/diagnostics_examples.rs`), a criterion benchmark
  (`benches/diagnostics_capture.rs`), and a graceful live test are included.

## 6. Known nuances & next steps

The capture front-end and the canonical `DiagnosticsContext` are now one model — but a few items
remain open:

- **Replay timing fidelity.** Rebuilding a `DiagnosticsContext` after the fact means the builder's
  own wall-clock timer measures the (synchronous) replay, not the original request, so the
  context's top-level `total_duration_ms` reflects replay time. The authoritative per-attempt
  latency the hot path captured is surfaced via each request's `server_duration_ms`. The long-term
  direction is for capture to **gate the pipeline's own live-built context** rather than rebuild
  one, which preserves true wall-clock timing end to end.
- **Depth of pipeline wiring.** The op-level capture hooks at the operation-executor seam and sees
  the terminal response, so an op-level built context currently carries op-level attempts. The
  per-leg hedging / fan-out detail (multiple `RequestDiagnostics`, winning leg, terminal state) is
  fully modeled and exercised by driving the recorder API directly (see the hedged example and
  tests); pushing those hooks deeper into `operation_pipeline` so live multi-leg executions
  populate them automatically is follow-up work.
- **Threshold configuration.** The gate's latency threshold is a plain `Duration`. It should
  ultimately read the driver's diagnostics configuration (verbosity / per-operation-kind
  thresholds) rather than a standalone value.

## 7. Test & bench results

Regenerate the two example outputs (real `DiagnosticsContext` JSON):

```text
cargo test -p azure_data_cosmos_driver --all-features --test diagnostics_examples -- --nocapture --test-threads=1
```

Run the benchmark:

```text
cargo bench -p azure_data_cosmos_driver --bench diagnostics_capture
```

Run the full gate:

```text
cargo fmt -p azure_data_cosmos_driver --check
cargo clippy -p azure_data_cosmos_driver --all-features --all-targets -- -D warnings
cargo test -p azure_data_cosmos_driver --all-features
```

## 8. How to enable

```rust,ignore
use azure_data_cosmos_driver::diagnostics::capture::DiagnosticsPolicy;
use azure_data_cosmos_driver::options::DriverOptions;
use std::time::Duration;

// Build a DiagnosticsContext on error or when an operation exceeds 5 ms; otherwise drop cheaply.
let driver_options = DriverOptions::builder(account)
    .with_capture_diagnostics_policy(DiagnosticsPolicy::threshold(Duration::from_millis(5)))
    .build();

// ... run an operation ...
if let Some(diagnostics) = response.capture_diagnostics() {
    println!("{}", diagnostics.to_json_string(None));
}
```

[`DiagnosticsContext`]: crate::diagnostics::DiagnosticsContext
