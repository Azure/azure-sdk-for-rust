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

## 2. Default behavior & how the gate short-circuits the build

The capture gate defaults to [`Mode::Always`] — diagnostics are produced **out-of-the-box**,
matching the driver's historical always-on behavior. Callers opt into the cheaper modes explicitly
via [`DriverOptionsBuilder::with_capture_diagnostics_policy`](crate::options::DriverOptionsBuilder)
(through [`DriverOptions::builder`](crate::options::DriverOptions::builder)). The gate
decides **before** the build, not after:

- **`Always`** (default) — the builder collects fully and the canonical context is surfaced. No
  behavior change versus the historical driver.
- **`Off`** — the `DiagnosticsContextBuilder` is created **disabled**: `start_request` returns an
  out-of-range handle without recording, so all per-request population auto-no-ops, and `complete()`
  yields a minimal context (activity id + final status, empty request list). The per-request build
  cost is genuinely skipped, not built-then-dropped. `response.diagnostics()` returns that minimal
  context (the accessor is non-optional), and `capture_diagnostics()` is `None`.
- **`Threshold`** — the builder collects fully during the operation (the slow/error verdict is only
  known at the end), and at op-end the gate decides whether to **surface** the context via
  `capture_diagnostics()`. On a fast success the standalone capture materialization
  ([`build_context`](self::context)) is short-circuited before it runs (see §7); the pipeline's own
  incrementally-built context still backs the always-on `response.diagnostics()`. See
  [§8 Known nuances](#8-known-nuances--boundaries) for the precise boundary on fast-success
  build-avoidance under `Threshold`.

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
  start   -> rent an EventLog lease (two flat Vecs: spans + attrs) from an Arc<LogPool>;
             push the operation span
  attempt -> push a typed Span (+ its Attrs) per attempt / hedge leg (just Vec::push)
  ...     -> the driver pipeline populates the capture-owned DiagnosticsContextBuilder with the
             rich per-attempt / hedging / transport data, with true wall-clock timing
  end     -> GATE: surface? (Always | elapsed > threshold | (error && capture_on_error))
               no  -> drop the recorder; its EventLog lease returns storage to the pool (~free)
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
- **`event`** (`event.rs`) — the hot-path data model: an [`EventLogStorage`] of two flat lists,
  `Span`s (`kind` + `Option<SpanId>` parent + op-relative `TimeOffset`s) and `Attr`s (typed
  `key`/`value` tagged with their owning `SpanId`). Storing either is a single `Vec::push` of a
  small value — **no byte encoding and no varints on the hot path**. `AttrValue` keeps the hot path
  allocation-light: numerics and a first-class `CosmosStatus` are `Copy`, and a string can be a
  zero-copy `&'static str`, a shared `Arc<str>`, or an owned `Box<str>`. An `EventLog` is an **RAII
  lease** bundling an `Arc<LogPool>` with the rented storage; dropping it returns the storage to the
  pool automatically.
- **`LogPool`** (`pool.rs`) — a **bounded** pool of reusable `EventLogStorage`s. Consumers hold it
  behind an `Arc<LogPool>`; `rent()` hands out an `EventLog` lease. Retention is sized to the common
  operation: only storages still at the default capacity are pooled; any that had to grow are freed.
- **`DiagnosticsRecorder` / `AttemptRecord`** (`recorder.rs`) — an operation-layer-owned,
  **lock-free `&mut`** recorder that pushes typed `Span`/`Attr` values into its rented `EventLog`
  lease. Cancellation- and panic-safe: it just owns the lease, so dropping the recorder (on success,
  error, cancellation, or panic) returns the storage to the pool — there is no explicit
  "return the buffer" step. It records the operation outcome + elapsed time the gate reads.
- **`DiagnosticsPolicy` / `Mode`** (`gate.rs`) — the gate: `Off`, `Threshold`, `Always` (default).
- **`build_context`** (`context.rs`) — walks a captured `EventLogStorage` (spans/attrs) back into a
  tree and replays it onto the capture-owned `DiagnosticsContextBuilder` to materialize a standalone
  `DiagnosticsContext`. The typed log *is* the parsed form, so there is no byte-parse step (used by
  the recorder's `finish()` for tests/examples and any caller that captures without the pipeline).
- **`encode`** (`encode.rs`) — an optional **cold-path** compact binary serialization of an
  `EventLogStorage`'s two lists (`EventLogStorage::to_compact_bytes` / `from_compact_bytes`). The
  varint/TLV machinery lives here, off the hot path: the binary form is just a compact way to store
  the two lists, paid only when bytes are actually requested.

There is **no parallel diagnostics model**: capture owns the one canonical type and gates it.

## 5a. The `summary` block (.NET-style top-level roll-up)

When a `DiagnosticsContext` is built, it carries a top-level [`DiagnosticsSummary`] —
an aggregatable roll-up modeled on the `Summary` block of .NET's `CosmosDiagnostics`. It is
**computed once at finalization** (in `complete()`), as a reduction over the already-collected
requests — **never on the hot path**. Because it lives on a built context, it exists exactly when
diagnostics are built; a dropped fast success (Off, or Threshold fast-success) produces no context
and hence no summary.

Fields (seeded from the high-value signals an investigation branches on first): `final_status`,
`request_count`, `retry_count`, `throttled_count`, `total_request_charge`, `total_duration_ms`,
`regions_contacted`, a `(status, sub-status) -> count` histogram (`status_counts`), and `top_error`.
It is serialized as the first section of the diagnostics output (like .NET puts `Summary` at the
top). Example for a retry (429 → 200):

```json
"summary": {
  "final_status": { "status": "200" },
  "request_count": 2,
  "retry_count": 1,
  "throttled_count": 1,
  "total_request_charge": 8.4,
  "total_duration_ms": 0,
  "regions_contacted": ["eastus"],
  "status_counts": [
    { "status": 200, "count": 1 },
    { "status": 429, "sub_status": 3200, "count": 1 }
  ]
}
```

## 5b. Diagnostics string encoding (client option)

How a `DiagnosticsContext` is rendered to a string is a driver client option,
[`DiagnosticsEncoding`], set via
[`DriverOptionsBuilder::with_diagnostics_encoding`](crate::options::DriverOptionsBuilder)
(via [`DriverOptions::builder`](crate::options::DriverOptions::builder)). It is honored by
[`DiagnosticsContext::encode`](crate::diagnostics::DiagnosticsContext::encode) and
[`CosmosResponse::diagnostics_string`](crate::models::CosmosResponse::diagnostics_string):

- **`Json`** (default) — pretty-printed, human-readable JSON. Default, so existing output is
  unchanged.
- **`Compact`** — minified JSON (same content, smallest text form).
- **`Encoded`** — base64 of the compact JSON, a single opaque token for size-sensitive logging;
  decode with standard base64 then parse the JSON to recover the full diagnostics.

All encodings carry the full detailed diagnostics including the top-level `summary`.

## 5c. Wall-clock timestamps (RFC 3339)

Alongside the precise elapsed-time fields (which are derived from `Instant` and are not
serializable), a built `DiagnosticsContext` carries **absolute wall-clock timestamps** so its output
can be correlated against server-side and external logs. These are captured with
[`azure_core::time::OffsetDateTime`] and serialized as RFC 3339 / ISO 8601 strings (matching the
convention the public `azure_data_cosmos` models already use):

- On the operation (`DiagnosticsContext`): `start_time` (captured when the recorder is created) and
  `end_time` (captured at `complete()`). The `summary` block carries the same operation window.
- On each attempt (`RequestDiagnostics`): `start_time` (captured when the attempt begins) and an
  optional `end_time` (captured when the attempt completes, times out, or fails its transport;
  omitted via `skip_serializing_if` while an attempt is still in flight).

The fields are additive and non-breaking — existing consumers that ignore them are unaffected, and
`Instant`-based durations (`duration_ms`, `total_duration_ms`) are unchanged. Like the summary, the
timestamps exist only when a context is built, so a dropped fast success carries none. Example:

```json
{
  "activity_id": "…",
  "start_time": "2026-06-16T14:22:06.123456Z",
  "end_time": "2026-06-16T14:22:06.158901Z",
  "summary": { "…": "…" },
  "requests": [
    {
      "start_time": "2026-06-16T14:22:06.123900Z",
      "end_time": "2026-06-16T14:22:06.140012Z",
      "…": "…"
    }
  ]
}
```

## 5d. The event-log representation (hot-path data model)

The recorder's hot path stores a typed, append-only **event log** of two flat lists
(`event.rs`), instead of a `Vec<u8>` TLV byte stream:

- `Vec<Span>` — one `Span` per timestamped scope or point event (the operation root, each request
  attempt, a hedge race). A span carries its `kind`, an `Option<SpanId>` parent, and op-relative
  `TimeOffset` start/end. A `SpanId` is a `NonZero<u32>` holding the span's index + 1, so
  `Option<SpanId>` encodes "no parent" (the root) in the niche for free, and the flat list
  reconstructs into a tree via the parent links.
- `Vec<Attr>` — one `Attr` per key/value (`Region`, `Endpoint`, `Status`, `RequestCharge`, …),
  tagged with the `SpanId` it belongs to. `AttrValue` keeps the hot path allocation-light:
  `U64`, `F64`, and a first-class `Status(CosmosStatus)` are `Copy`; a string can be a zero-copy
  `StaticStr(&'static str)`, a shared `SharedStr(Arc<str>)` that points at an existing heap
  allocation, or an owned `Str(Box<str>)`.

Appending is a single `Vec::push` of a typed value — no varints, no byte writing on the hot path.
The two `Vec` backbones (an `EventLogStorage`) are pooled and reused; an `EventLog` is an **RAII
lease** that bundles an `Arc<LogPool>` with the rented storage and returns it to the pool on drop,
so callers never manage the buffer lifecycle. Below the gate threshold the recorder is simply
dropped (storage pooled, ~free); past the gate, `context.rs` walks the spans/attrs into the
canonical `DiagnosticsContext` (the typed log *is* the parsed form — no byte-parse step). The
optional compact binary form (`EventLogStorage::to_compact_bytes` / `from_compact_bytes`, in
`encode.rs`) is a **cold-path** serializer of the same two lists, paid only when bytes are
actually requested.

## 6. Scope of this change

- **Ownership flip, driver-scoped.** The rich diagnostics model moved from
  `diagnostics/diagnostics_context.rs` to `diagnostics/capture/model.rs` (capture owns it); the
  public re-export paths (`diagnostics::DiagnosticsContext`, …) are unchanged.
- **The driver routes through capture.** At the operation-executor seam, the recorder records the
  outcome + elapsed and the gate decides whether the canonical `DiagnosticsContext` is surfaced via
  `CosmosResponse::capture_diagnostics()`.
- **`.NET`-style `summary` block** computed at finalization, and a **`DiagnosticsEncoding`** client
  option — both additive and non-breaking (default encoding `Json`).
- **No data loss.** The pipeline keeps populating the (now capture-owned) builder, so events,
  transport-shard diagnostics, fault-injection evaluations, true wall-clock timing, and hedging are
  all preserved.
- **No public break.** `response.diagnostics()` and the `azure_data_cosmos` boundary type are
  unchanged. The default is `Always`, so diagnostics are still produced out-of-the-box.
- **No `azure_core` / `typespec_client_core` changes.**

[`DiagnosticsSummary`]: crate::diagnostics::DiagnosticsSummary
[`DiagnosticsEncoding`]: crate::options::DiagnosticsEncoding

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

## 8. Known nuances & boundaries

- **`Off` short-circuits the build (resolved).** Under `Off` the builder is created disabled, so the
  per-request population and per-attempt allocations are skipped entirely — the gate decides before
  the build, not after. The only residual cost is the trivial minimal `complete()` (an empty request
  list wrapped in an `Arc`), which is unavoidable because `response.diagnostics()` is non-optional.
- **`Threshold` fast-success build-avoidance — the precise boundary.** Under `Threshold` the
  slow/error verdict is only known at op-end, *after* the pipeline has already populated the builder
  incrementally during I/O. The rich fields (`RequestEvent` timelines, transport-shard /
  fault-injection diagnostics, true per-attempt wall-clock timing) are produced incrementally and
  cannot be reconstructed after the fact; deferring them would require buffering that costs the same
  as populating. The always-on, non-optional `response.diagnostics()` needs the context regardless.
  Therefore, on a `Threshold` fast success the pipeline's incremental build still happens (it is
  cheap — `complete()` is a move + `Arc`, and JSON is lazy); only the standalone capture
  materialization is short-circuited. Fully avoiding the pipeline's incremental build on a
  `Threshold` fast success **without data loss** would require routing all rich-field collection
  through a byte append-log — investigated and **rejected** (see
  [Investigated alternative](#investigated-alternative--full-collection-through-append-log-rejected-not-a-net-win)).
  **No rich data is dropped when the gate fires.**
- **Standalone replay timing.** When the recorder builds a `DiagnosticsContext` standalone (no
  pipeline — tests/examples), the builder's clock measures replay time, so the top-level
  `total_duration_ms` reflects replay; per-attempt latency is surfaced via `server_duration_ms`. In
  the live driver path the pipeline feeds the builder with **true wall-clock timing**, so this only
  affects standalone use.
- **Threshold configuration.** The gate's latency threshold is a plain `Duration`; it should
  eventually read the driver's diagnostics configuration (verbosity / per-operation-kind
  thresholds).

### Investigated alternative — full collection-through-append-log (rejected; not a net win)

We investigated rerouting **all** rich-field collection through a lock-free byte append-log (to
realize the prototype spike's sub-µs hot path inside the integrated rich model) and materializing
the full `DiagnosticsContext` only at op-end. It was **rejected** — it is not a net win and has
hard blockers:

1. **Full fidelity ≠ sub-µs.** The spike was sub-µs because it captured a flat ~6-field subset
   (status, RU, region, sub-status, request_sent, timing). Byte-encoding the *full* model — ~22
   `RequestDiagnostics` fields + `RequestEvent` timelines + `TransportShardDiagnostics` +
   `FailedTransportShardDiagnostics` + `FaultInjectionEvaluation`s — costs **more** than the current
   struct population (it is serialization: varints + per-field string copies) and adds a
   decode/replay cost when the gate fires. You cannot have sub-µs *and* full fidelity.
2. **The builder is already the optimal shape.** `DiagnosticsContextBuilder` is already a lock-free
   append (single-owner `&mut Vec<RequestDiagnostics>`, no `Mutex`/atomics on the per-attempt path)
   with a cheap `complete()` and **lazy JSON** (`OnceLock`). The expensive step is already deferred,
   and `Off` already disables per-request population. A byte-log would add encode+decode cost and
   likely make the default `Always` path **slower**.
3. **Two fields can't be losslessly byte-encoded.** The `Instant` timestamps
   (`RequestDiagnostics::started_at`/`completed_at`, `RequestEvent::timestamp`) have no portable
   byte representation, and `FaultInjectionEvaluation` is a `#[non_exhaustive]` enum whose
   hand-rolled codec would silently lose data when a variant is added.

**Resolution:** keep the live rich path feeding the builder — full fidelity, lock-free append,
lazy JSON, `Off` as the cheap opt-out, default `Always`. What *did* change is the **recorder's own
capture representation**: it is now a typed two-list `EventLog` (`Vec<Span>` + `Vec<Attr>`, see
[§5d](#5d-the-event-log-representation-hot-path-data-model)) rather than a `Vec<u8>` TLV byte stream. That is the
idiomatic realization of the "flat, timestamped log of events" idea for the captured subset —
`Vec::push` of typed structs on the hot path, tree reconstruction on the cold path — and it
sidesteps blocker 3 entirely (nothing is byte-encoded on the hot path; the `Instant`-derived
durations and enum discriminants are stored as plain fields). Byte serialization survives only as
an **optional cold-path** form (`encode.rs`) over the captured subset, never the full model.

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
