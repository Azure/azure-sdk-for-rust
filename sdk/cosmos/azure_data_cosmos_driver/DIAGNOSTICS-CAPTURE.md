# Deferred, Threshold-Gated Diagnostics Capture (prototype)

> **Status: prototype / for discussion.** This document describes an opt-in, parallel diagnostics
> subsystem under `azure_data_cosmos_driver::diagnostics::capture`. It is independent of the
> shipping [`DiagnosticsContext`] and is **off by default**. The intended next step — feeding or
> extending `DiagnosticsContext` rather than building a separate model — is the open question
> tracked in [§ Open question / deferred](#open-question--deferred).

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
- **G4** — Binary compaction is opt-in (a compact `AZD1` wire format + version header), with a
  shared decode path.
- **G5** — Compact SDK/driver version + User-Agent provenance recorded as a tiny interned
  preamble, rehydrated only when built.
- **G6** — The default summary is aggregatable and close to today's request-style diagnostics.

## 2. Design exploration & benchmarks

Four capture strategies were prototyped and benchmarked on identical scenarios (single success,
retry-then-success, error, and fan-out with N children):

| Design | What it produces | One-line trade |
|---|---|---|
| **Eager JSON** | A nested JSON object, built on every call | Most familiar, best OpenTelemetry fit, but the most expensive and the largest output. |
| **Binary Span Tree** | A small binary blob (decode to JSON with a tool) | Full call tree cheaply; dropped for free on success; smallest full-fidelity format. |
| **Tiered Hybrid** | A tiny summary by default; full binary on error/on demand | Looks like today's request diagnostics; cheap happy path; never loses detail. |
| **Deferred Gated Capture** ⭐ | Nothing on a fast success; a summary (+opt-in binary) only when slow or errored | Append-only capture, then *decide at the end* whether to build. Cheapest happy path. |

**Headline (release):** on a fast success the Deferred Gated design pays only the append +
return-to-pool cost and builds nothing — roughly an order of magnitude cheaper than building a
summary every time, and ~100x cheaper than the eager baseline. Output collapses cleanly: a wide
fan-out keeps a flat summary while the eager JSON balloons.

This PR implements the **Deferred Gated Capture** design. Measured numbers from this crate:

| Path | Cost (criterion, release) |
|---|---|
| Fast success, gate drops it | **~140 ns** (append + return-to-pool; allocation-free after warm-up) |
| Slow/errored, build summary | **~1.6 µs** (parse + reduce, only past the gate) |
| Slow/errored, build summary + `AZD1` | **~54 µs** (+ binary encode, off the hot path) |

Output size:

| Case | Dropped | Summary | Detailed (`AZD1`) |
|---|---|---|---|
| Retry 429→200 | **0 B** | 480 B | 506 B |
| Fan-out ×25 | **0 B** | **391 B (flat)** | 545 B |

## 3. Comparison with .NET `CosmosDiagnostics`

.NET V3 ships `CosmosDiagnostics`: a request-focused object with a top-level `Summary` histogram
over a nested handler → transport → `StoreResult` tree. Conceptually:

- Its **`Summary` block** is the aggregatable roll-up this design emits as the default summary —
  the migration-friendly path.
- Its **full nested tree** is the information this design ships as a compact, opt-in, on-error
  binary blob instead of building eagerly every time.
- Its once-per-diagnostics **Client Configuration / User-Agent** maps to the interned
  version/User-Agent preamble (recorded once, not per request).

The captured field set is seeded from the signals a support investigation branches on first:
final/per-attempt status, error classification (incl. Cosmos sub-status), service request id
(activity id), request charge (RU), retry/throttle counts, total elapsed, and — for fan-out — a
child count with per-child plan-node / feed-range detail in the binary tier.

## 4. The chosen design — Deferred Gated Capture

```text
HOT PATH (per attempt, write-preferred, lock-free, no Mutex):
  start   -> rent Vec<u8> from a bounded pool; write a 1-byte preamble id
  attempt -> append TLV: status, service-request-id, RU, sub-status, request_sent, timing
  child   -> append TLV: plan-node id, feed-range, timing      (fan-out)
  end     -> GATE: build? (Always | elapsed > latency_threshold | (error && capture_on_error))
               no  -> clear + return buffer to pool             (~free)
               yes -> parse log -> Summary (always)
                                -> AZD1 binary blob              (opt-in)
```

Components (`azure_data_cosmos_driver::diagnostics::capture`):

- **`LogPool`** — a shared, **bounded** pool of reusable buffers. Rent/return happen only at
  operation boundaries; oversized buffers (wide fan-out) are dropped rather than pinned.
- **`DiagnosticsRecorder`** — an operation-layer-owned, **lock-free `&mut`** append recorder.
  It is **cancellation-safe** (its `Drop` returns the buffer if `finish` never runs) and
  **panic-safe** (a partially written buffer is cleared, never poisoning a reused buffer). Elapsed
  time uses `std::time::Instant`. Fan-out children are captured as plain `ChildRecord` values by
  concurrent tasks and merged at the operation layer on join, so per-task capture stays lock-free.
- **`DiagnosticsPolicy` / `Mode`** — the gate: `Off` (default; truly zero), `Threshold`
  (build on error or over a latency threshold), `Always`. `finish()` drops cheaply or builds.
- **`Summary`** — the always-built, aggregatable roll-up (status histogram, top error incl.
  sub-status, RU as `f64`, retry/throttle counts, elapsed, final service-request-id, child count,
  client provenance).
- **`AZD1` wire format** (`capture::wire`) — a compact, deterministic binary: magic + version byte
  + flags + a flat node list (varint TLV), optional DEFLATE above a size threshold. The decoder
  **rejects unknown versions** and malformed input (checked conversions, bounded pre-allocation).
- **Interned preamble** — process-global SDK/driver version + User-Agent, recorded as a single
  byte on the hot path and rehydrated only when the gate builds.

## 5. Scope of this change

- **Opt-in, default `Off`** via `DriverOptions::with_capture_diagnostics_policy(...)`. With the
  default, no recorder is constructed and there is **no behavior change**.
- **Re-homed under `diagnostics::capture`** so it coexists with the existing `DiagnosticsContext`
  and friends; nothing in the shipping diagnostics module is removed or altered.
- **Additive driver wiring** at the operation executor seam (`execute_operation_direct`): when the
  policy is enabled, an operation-level capture is recorded and attached to the response via
  `CosmosResponse::capture_diagnostics()`. On a terminal error the summary is emitted via
  `tracing`.
- **No changes to `azure_core` / `typespec_client_core`.**
- Tests, a criterion benchmark (`benches/diagnostics_capture.rs`), and a graceful live test are
  included.

## 6. Open question / deferred

This subsystem currently builds a **separate** `Summary` / `Rendered`, parallel to the shipping
[`DiagnosticsContext`]. That is deliberate for a prototype, but it is **not** the intended end
state. The open design question is how to converge the two:

- **Feed/extend `DiagnosticsContext`.** The preferred direction is for the gated capture to drive
  (or be driven by) `DiagnosticsContext` so there is a single diagnostics model, with the gate and
  the cheap append-only hot path becoming an internal acquisition strategy rather than a second
  surface. The per-attempt detail captured today at the operation level would move down to the
  pipeline's existing per-request diagnostics.
- **Threshold configuration.** The gate's latency threshold is currently a plain `Duration`. It
  should ultimately read the driver's diagnostics configuration (verbosity / per-operation-kind
  thresholds) rather than a standalone value.
- **Wire-format & attribute governance.** The `AZD1` format, its version-evolution policy, the
  attribute keys (e.g. service-request-id), and ownership of a shared decode tool are cross-cutting
  and would need ratification before the format becomes a compatibility surface.

Until those are settled, this PR keeps the capture subsystem **parallel and opt-in**.

## 7. How to enable

```rust,ignore
use azure_data_cosmos_driver::diagnostics::capture::DiagnosticsPolicy;
use azure_data_cosmos_driver::options::DriverOptions;
use std::time::Duration;

// Build on error or when an operation exceeds 5 ms; summary only (binary off by default).
let driver_options = DriverOptions::builder(account)
    .with_capture_diagnostics_policy(DiagnosticsPolicy::threshold(Duration::from_millis(5)))
    .build();

// ... run an operation ...
if let Some(rendered) = response.capture_diagnostics() {
    if let Some(summary) = rendered.summary() {
        println!("{}", summary.to_json_pretty());
    }
}
```

[`DiagnosticsContext`]: crate::diagnostics::DiagnosticsContext
