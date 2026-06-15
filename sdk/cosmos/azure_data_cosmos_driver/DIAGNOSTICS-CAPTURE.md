# Deferred, Threshold-Gated Diagnostics Capture — the driver's diagnostics engine

> **Status: prototype / for discussion.** This document describes the diagnostics **capture**
> module (`azure_data_cosmos_driver::diagnostics::capture`), which is the driver's diagnostics
> **engine**: a cheap, append-only, lock-free hot-path recorder plus an operation-end gate
> (`Off` / `Threshold` / `Always`). The capture module **owns** the canonical diagnostics model —
> [`DiagnosticsContext`] and its builder are homed here — and the driver collects diagnostics by
> feeding that builder. The gate governs whether the resulting context is surfaced. This is an
> **ownership flip inside the driver crate**: the rich diagnostics model is re-homed under capture
> ownership with **no data loss** and **no change to the public SDK boundary**.

## 1. Problem & goals

Request-focused diagnostics across SDKs tend to share three shortcomings:

1. **Eager cost.** Diagnostics are materialized on every call, including the overwhelming
   majority of fast successes that are never read.
2. **Fan-out blindness.** Query/routing operations spawn many parallel sub-requests (per
   partition / feed range) that a request-centric model struggles to represent.
3. **One verbosity for everyone.** No cheap-by-default / full-on-demand split.

We want **outcome-aware** diagnostics: full fidelity when useful (errors / slow ops / explicit
request) with the option to drop cheaply on a fast success — without losing the rich data or
breaking the public contract.

Goals:

- **G1** — Hot path is write-preferred, compact, append-only, minimal-allocation (pooled).
- **G2** — Collection/materialization is governed by the gate, off the hot path.
- **G3** — A configurable gate evaluated at operation end decides whether to surface diagnostics:
  latency threshold + on-error, plus Off/Always modes.
- **G4** — One canonical diagnostics model ([`DiagnosticsContext`]), **owned by the capture
  module**, fed by the driver, with every rich field preserved (events, transport-shard /
  failed-shard diagnostics, fault-injection evaluations, true wall-clock timing, hedging).
- **G5** — The public SDK boundary (`diagnostics::DiagnosticsContext`, consumed by
  `azure_data_cosmos`) is **unchanged**: this is additive / non-breaking.

## 2. Default behavior

The capture gate defaults to [`Mode::Always`] — diagnostics are produced **out-of-the-box**,
matching the driver's historical always-on behavior. Callers opt into the cheaper modes explicitly
via [`DriverOptions::with_capture_diagnostics_policy`](crate::options::DriverOptions):

- **`Always`** (default) — always surface the canonical context.
- **`Threshold`** — surface on error or when the operation exceeds a latency threshold; drop a
  fast success cheaply.
- **`Off`** — never surface via the capture accessor (the rich `response.diagnostics()` boundary
  is unaffected).

## 3. Design exploration & benchmarks

Four capture strategies were prototyped and benchmarked on identical scenarios (single success,
retry-then-success, error, and fan-out with N children):

| Design | What it produces | One-line trade |
|---|---|---|
| **Eager JSON** | A nested JSON object, built on every call | Most familiar, best OpenTelemetry fit, but the most expensive and the largest output. |
| **Binary Span Tree** | A small binary blob (decode to JSON with a tool) | Full call tree cheaply; dropped for free on success; smallest full-fidelity format. |
| **Tiered Hybrid** | A tiny summary by default; full binary on error/on demand | Looks like today's request diagnostics; cheap happy path; never loses detail. |
| **Deferred Gated Capture** ⭐ | Build/keep only when slow or errored (or `Always`) | Append-only capture, then *decide at the end* whether to surface. Cheapest opt-in happy path. |

This crate implements **Deferred Gated Capture** as the diagnostics engine. Measured numbers
(criterion, dev profile; see [§7](#7-test--bench-results)):

| Path | Cost (criterion) |
|---|---|
| Gate drops a fast success (opt-in `Threshold`) | **~0.4 µs** (start + append + gate + return-to-pool) |
| `Mode::Off` (gate drops without building) | **~0.37 µs** |
| Gate fires, build standalone `DiagnosticsContext` | **~5 µs** (parse + replay onto the builder) |

## 4. Comparison with .NET `CosmosDiagnostics`

.NET V3 ships `CosmosDiagnostics`: a request-focused object with a top-level `Summary` histogram
over a nested handler → transport → `StoreResult` tree. Conceptually:

- Its **per-request tree** maps onto this driver's existing
  [`RequestDiagnostics`](crate::diagnostics::RequestDiagnostics) list inside `DiagnosticsContext` —
  the same model the driver feeds, so there is no second object to reconcile.
- Its **`Summary` roll-up** corresponds to `DiagnosticsContext`'s `Summary` serialization verbosity
  (`DiagnosticsVerbosity::Summary`) — a view over the one model.
- Its once-per-diagnostics **Client Configuration / User-Agent** maps to the context's client-level
  metadata.

## 5. The chosen design — capture owns the model, the driver feeds it, the gate governs

```text
HOT PATH (per operation, lock-free, no Mutex):
  start   -> rent Vec<u8> from a bounded pool; write the op header
  ...     -> the driver pipeline populates the capture-owned DiagnosticsContextBuilder with the
             rich per-attempt / hedging / transport data, with true wall-clock timing
  end     -> GATE: surface? (Always | elapsed > threshold | (error && capture_on_error))
               no  -> drop; return buffer to pool                      (~free)
               yes -> surface the canonical DiagnosticsContext
```

Components (`azure_data_cosmos_driver::diagnostics::capture`):

- **`model`** — the canonical diagnostics model, **re-homed here** under capture ownership:
  [`DiagnosticsContext`], `DiagnosticsContextBuilder`,
  [`RequestDiagnostics`](crate::diagnostics::RequestDiagnostics),
  [`ExecutionContext`](crate::diagnostics::ExecutionContext), the transport-shard /
  fault-injection / event types, etc. These are re-exported from `crate::diagnostics` so the public
  boundary is unchanged. The driver pipeline feeds this builder during execution, so **every rich
  field is still populated** (no regression).
- **`LogPool`** (`pool.rs`) — a shared, **bounded** pool of reusable hot-path buffers.
- **`DiagnosticsRecorder` / `AttemptRecord`** (`recorder.rs`) — an operation-layer-owned,
  **lock-free `&mut`** append recorder. Cancellation-safe (`Drop` returns the buffer if the gate
  never runs) and panic-safe. It is the cheap front-end that records the operation outcome +
  elapsed time the gate reads, and the standalone acquisition path for tests/examples.
- **`DiagnosticsPolicy` / `Mode`** (`gate.rs`) — the gate: `Off`, `Threshold`, `Always` (default).
- **`build_context`** (`context.rs`) — replays a captured log onto the capture-owned
  `DiagnosticsContextBuilder` to materialize a standalone `DiagnosticsContext` (used by the
  recorder's `finish()` for tests/examples and any caller that captures without the pipeline).

There is **no parallel diagnostics model**: capture owns the one canonical type and gates it.

## 6. Scope of this change

- **Ownership flip, driver-scoped.** The rich diagnostics model moved from
  `diagnostics/diagnostics_context.rs` to `diagnostics/capture/model.rs` (capture owns it); the
  public re-export paths (`diagnostics::DiagnosticsContext`, …) are unchanged.
- **The driver routes through capture.** At the operation-executor seam, the recorder records the
  outcome + elapsed and the gate decides whether the canonical `DiagnosticsContext` is surfaced via
  `CosmosResponse::capture_diagnostics()`.
- **No data loss.** The pipeline keeps populating the (now capture-owned) builder, so events,
  transport-shard diagnostics, fault-injection evaluations, true wall-clock timing, and hedging are
  all preserved.
- **No public break.** `response.diagnostics()` and the `azure_data_cosmos` boundary type are
  unchanged. The default is `Always`, so diagnostics are still produced out-of-the-box.
- **No `azure_core` / `typespec_client_core` changes.**

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

## 8. Known nuances & next steps

- **Standalone replay timing.** When the recorder builds a `DiagnosticsContext` standalone (no
  pipeline — tests/examples), the builder's clock measures replay time, so the top-level
  `total_duration_ms` reflects replay; per-attempt latency is surfaced via `server_duration_ms`. In
  the live driver path the pipeline feeds the builder with **true wall-clock timing**, so this only
  affects standalone use.
- **Cheap-drop and the pipeline build.** Under `Threshold`/`Off`, the gate currently drops the
  already-built context (it does not yet prevent the pipeline from building it). Gating the
  pipeline's build itself — so a fast success pays nothing — is the natural follow-up.
- **Threshold configuration.** The gate's latency threshold is a plain `Duration`; it should
  eventually read the driver's diagnostics configuration (verbosity / per-operation-kind
  thresholds).

## 9. How to configure

```rust,ignore
use azure_data_cosmos_driver::diagnostics::capture::DiagnosticsPolicy;
use azure_data_cosmos_driver::options::DriverOptions;
use std::time::Duration;

// Default is Always (diagnostics out-of-the-box). To make the hot path cheaper, surface a
// DiagnosticsContext only on error or when an operation exceeds 5 ms:
let driver_options = DriverOptions::builder(account)
    .with_capture_diagnostics_policy(DiagnosticsPolicy::threshold(Duration::from_millis(5)))
    .build();

// ... run an operation ...
if let Some(diagnostics) = response.capture_diagnostics() {
    println!("{}", diagnostics.to_json_string(None));
}
```

[`DiagnosticsContext`]: crate::diagnostics::DiagnosticsContext
[`Mode::Always`]: crate::diagnostics::capture::Mode::Always
