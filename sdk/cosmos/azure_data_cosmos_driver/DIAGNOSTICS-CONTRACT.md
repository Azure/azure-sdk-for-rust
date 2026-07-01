<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->

# The driver↔SDK diagnostics contract & its OpenTelemetry mapping

> **Status: design (contract-first).** This document defines the **contract** between the
> `azure_data_cosmos_driver` and its consumers — the Rust `azure_data_cosmos` SDK today, and
> the next-major Go/Python SDKs over FFI — plus the **OpenTelemetry customer experience**. It
> is a **design document; it changes no code and adds no public API.** Where it sketches a
> future surface, that surface is **additive and non-breaking**.
>
> **Priority.** The contract and the customer OTel experience come **first**. A separately
> prototyped append-only *capture engine* (PR #4619, "[Prototype] Diagnostics capture
> engine") is a **deferred optimization** — it stays OFF by default and is an implementation
> detail *under* this contract, not part of it. This doc references that prototype
> **conceptually**; it does not depend on it.

## 0. What exists today vs. what this contract proposes

To keep the design honest, references below are tagged:

- **[main]** — exists in the crate today (`sdk/cosmos/azure_data_cosmos_driver`):
  [`CosmosResponse::diagnostics()`][diag] → `Arc<DiagnosticsContext>` (non-optional);
  [`DiagnosticsContext`][ctx] with [`requests()`][ctxsrc], [`duration()`][ctxsrc],
  [`status()`][ctxsrc], [`to_json_string(verbosity)`][ctxsrc]; [`RequestDiagnostics`][ctxsrc];
  [`DiagnosticsVerbosity`][opts] and [`DiagnosticsOptions::max_summary_size_bytes`][opts].
- **[contract]** — proposed here (additive): the named `DiagnosticsHandle` materializers
  (`as_metrics` / `as_spans` / `as_log`), the `DiagnosticsLevel` enum, the FFI surface, and
  the bounded-size caps.
- **[#4619]** — lives in the deferred capture-engine prototype only, not on `main`: the
  `diagnostics::capture` module (the `event.rs` `Span`/`Attr` model, the `Off`/`Threshold`/
  `Always` gate), `DiagnosticsSummary`, `DiagnosticsEncoding`/`encode()`, and the per-op /
  per-attempt wall-clock `start_time`/`end_time` fields.

## 1. Why a contract first

The driver already produces rich per-operation diagnostics and hands them back through a
cheap handle. Two things were underspecified, and the SDK requirements must drive the driver
design (not the other way around):

1. **One implicit shape.** Materialization is JSON-only and implicit
   ([`to_json_string`][ctxsrc]). Different consumers want different shapes: metrics want a
   structured object, tracing wants spans, logs want a string. JSON is the single costliest
   step (bench: struct build ~3.7 µs → +detailed JSON ~6.6 µs), so forcing it on everyone is
   wasteful — and lossy/expensive across an FFI boundary.
2. **No agreed boundary for non-Rust consumers.** The next-major Go/Python SDKs consume the
   driver over FFI. A fixed serialized format at that boundary is the wrong default.

So we fix the **contract** and the **OpenTelemetry mapping** first; the hot-path capture
engine [#4619] is a deferrable optimization.

## 2. The contract in one picture

```text
driver operation completes
        │
        ▼
   DiagnosticsHandle          ← cheap: today's Arc<DiagnosticsContext> [main] (an atomic incr).
        │                        ALWAYS available. Diagnostics are ALWAYS collected.
        │  materialize on demand — pay only for the shape you ask for:
        ├──▶ as_metrics(level) → structured object   (metrics)   [contract]
        ├──▶ as_spans()        → OTel span tree        (traces)    [contract]
        └──▶ as_log(...)       → String                (logs)      [main: to_json_string]
```

Three invariants:

- **P1 — The handle is cheap and unconditional.** In Rust it *is*
  [`CosmosResponse::diagnostics()`][diag] → `Arc<DiagnosticsContext>` [main]. Cloning it is an
  atomic increment. It is **always** returned; `diagnostics()` is **non-optional**.
- **P2 — Materialization is explicit, lazy, per-representation.** No single fixed serialized
  format. Each materializer is paid only when called and cached, so the expensive JSON step is
  never paid on a metrics-only or span-only path.
- **P3 — The level/threshold governs EXPOSURE and DEPTH, never COLLECTION.** A diagnostics
  *level* bounds the high-cardinality **transport-level** detail a materialization includes;
  it never removes operation-level diagnostics and never disables collection.

There is **no parallel diagnostics model**: the driver owns one canonical
[`DiagnosticsContext`][ctx] [main] and every representation is a view over it.

## 3. The three representations (views over one model)

| Materializer | Consumer intent | Backed by |
| --- | --- | --- |
| **Structured object** | metrics | [`DiagnosticsContext::requests()`][ctxsrc] [main], reduced into an operation roll-up (the `DiagnosticsSummary` reduction is added by [#4619]) |
| **OTel span tree** | traces | reconstructed from [`DiagnosticsContext`][ctx] + per-attempt [`RequestDiagnostics`][ctxsrc] [main]; the [#4619] `event.rs` `Span`/`Attr`/`SpanKind`/`TimeOffset` model is the OTel-aligned in-memory form |
| **String** | logs | [`DiagnosticsContext::to_json_string(verbosity)`][ctxsrc] [main] (a compact/base64 `encode()` is added by [#4619]) |

## 4. The FFI boundary (Go / Python next-major)

**How diagnostics flow over FFI: an opaque handle + explicit materialize calls. Never force
full JSON at the boundary.**

```text
Rust driver                         FFI (C ABI)                    Go / Python SDK
───────────                         ───────────                    ───────────────
Arc<DiagnosticsContext>  ── boxed →  DiagnosticsHandle (opaque ptr) hold cheaply; free later
                                     cosmos_diag_materialize(
                                       handle, representation,       caller picks the shape
                                       level, out_buf)
                                       ├ Metrics → packed struct / flatbuffer
                                       ├ Spans   → span-tree buffer (or per-span callback)
                                       └ Log     → UTF-8 bytes (JSON / compact / encoded)
                                     cosmos_diag_free(handle)        explicit lifetime
```

Contract rules for the boundary:

1. **Opaque handle.** The FFI surfaces an opaque pointer wrapping the `Arc`. Crossing the
   boundary is a pointer move + refcount, **not** a serialization.
2. **Explicit materialize, consumer-chosen shape.** One `materialize(handle, representation,
   level)` entry point where `representation ∈ {Metrics, Spans, Log}` and `level` selects
   transport-detail depth.
3. **No forced JSON.** JSON is one representation requested explicitly; a Go metrics exporter
   never triggers it.
4. **Explicit lifetime (resolved: opaque handle + `free`).** The handle is freed by an
   explicit `cosmos_diag_free`; refcounting stays on the Rust side. A scoped
   materialize-then-drop callback may be added later as a convenience wrapper.
5. **Bounded output.** Every materialization honors the bounded-size guarantee ([§6](#6-bounded-size-guarantee-retry-storms)),
   so an FFI buffer can be sized predictably even under a retry storm.

The exact wire encoding of each representation (packed struct vs flatbuffer vs span callback)
is a follow-up implementation detail; the contract fixes only "opaque handle + explicit
per-representation materialize + bounded output".

## 5. Where levels / thresholds apply

**Gating bounds high-cardinality TRANSPORT-level telemetry — it never eliminates
diagnostics.**

| Tier | Examples | Cardinality | Gating |
| --- | --- | --- | --- |
| **Operation-level** | operation name, final status, request/retry/throttled counts, total RU, total duration, regions contacted | low | **Always on.** Never gated away. |
| **Transport-level** | per-replica / per-partition (partition key range, feed range), endpoint, direct-mode channel, transport kind/security/http-version, transport shard, per-attempt RU/latency | high | **Gated by `DiagnosticsLevel` / threshold.** Included on error / slow / high level; summarized or elided on a fast-success low level. |

The knob is a dedicated **`DiagnosticsLevel { Minimal, Standard, Full }`** [contract] (resolved:
a new enum rather than overloading [`DiagnosticsVerbosity`][opts] [main], which is
string-render specific and has no `Minimal`). `DiagnosticsLevel` maps onto
`DiagnosticsVerbosity` internally:

- `Minimal` — operation-level only.
- `Standard` — + region-grouped/deduplicated transport summary (`Verbosity::Summary`).
- `Full` — + every per-attempt transport record (`Verbosity::Detailed`).

> **Collection is not gated.** Operation-level metrics are *computed by iterating the
> per-attempt records* ([`requests()`][ctxsrc] [main]; the `DiagnosticsSummary` reduction is
> [#4619]), so "cheap op-level only" is not achievable by dropping transport collection. The
> driver therefore **always collects the full per-attempt records** (the ~3.7 µs struct build)
> and the level gates only *materialization + exposure*. See [§9](#9-resolved-decisions) Q1.

## 6. Bounded-size guarantee (retry storms)

**Every materialized representation has an upper bound on size that is independent of attempt
count**, so a `410`/`429` retry storm or a large fan-out query cannot produce an unbounded
object, span tree, or string.

- **Mechanism (primitive already present [main]).** [`DiagnosticsVerbosity::Summary`][opts]
  groups requests by region, keeps first + last per region in full, deduplicates the middle by
  `(endpoint, status, sub_status, execution_context)` with count + min/max/P50, bounded by
  [`DiagnosticsOptions::max_summary_size_bytes`][opts] (default 8 KB, min 4 KB).
- **Contract [contract] (resolved: configurable per-representation caps with documented
  defaults).**
  - max attempts rendered in the object (default 64), max spans in the tree (default 128),
    max bytes in the string (default 8 KB) — each overridable via `DiagnosticsOptions`.
  - Compaction is lossy only in the *middle* of a run; the head/tail extremes and the
    aggregates (counts, histogram, min/max/P50) are always exact.
  - Truncation is marked, never silent.

This document defines the guarantee; the append-only compaction engine that realizes it
cheaply is the deferred optimization [#4619].

## 7. OpenTelemetry mapping

### 7.1 Operation-level metrics (always-on, low cardinality)

Source: an operation roll-up over [`DiagnosticsContext::requests()`][ctxsrc] [main] (the
`DiagnosticsSummary` reduction [#4619]). Emit as OTel metrics with only low-cardinality
attributes (`db.operation`, `db.cosmosdb.status_code`, op-granularity `region`):

| Instrument | Kind | Source | Attributes |
| --- | --- | --- | --- |
| `db.cosmosdb.operation.duration` | histogram | total operation duration | `db.operation`, `db.cosmosdb.status_code` |
| `db.cosmosdb.operation.requests` | counter | request count | `db.operation` |
| `db.cosmosdb.operation.retries` | counter | retry count | `db.operation` |
| `db.cosmosdb.operation.throttled` | counter | throttled (429) count | `db.operation` |
| `db.cosmosdb.request_charge` | histogram (RU) | total request charge | `db.operation`, `db.cosmosdb.status_code` |
| status distribution | counter | `(status, sub_status)` histogram | `db.cosmosdb.status_code`, `db.cosmosdb.sub_status_code` |

### 7.2 Transport-level → traces, never metric dimensions

Source: per-attempt [`RequestDiagnostics`][ctxsrc] [main] (endpoint, region, status, RU,
transport kind/security/http-version) and — in the [#4619] event model — partition key range
/ feed range via `AttrKey::PartitionKeyRangeId` / `AttrKey::FeedRange`. These are **unbounded**
in practice; putting them on metric attributes explodes time-series cardinality, so they go on
**span attributes** (and per-attempt RU/latency may be span exemplars), gated by
`DiagnosticsLevel`.

### 7.3 Traces (span tree)

| `DiagnosticsContext` element | OTel span |
| --- | --- |
| operation (root) | root span, kind `Client`, name `Cosmos <operation>`; window from the operation's wall-clock start over its [`duration()`][ctxsrc] [main] (absolute `start_time`/`end_time` fields are added by [#4619]; on `main` the emitter captures them) |
| each `RequestDiagnostics` (attempt / hedge leg) | child span, kind `Client` |
| request-event timeline | timed span **events** on the attempt span |
| hedging | a hedge span with terminal state + regions |
| **aggregated multi-run op** (`aggregate_sub_operations`) | **a single synthetic operation root** spanning the first run's start to the last run's end, with each run's attempts as children (resolved: [§9](#9-resolved-decisions) Q9) |

### 7.4 Attribute alignment with `azure_core`

Reuse the `azure_core` span-attribute names so Cosmos spans correlate with `azure_core`-
emitted spans:

| Diagnostics field | Attribute name |
| --- | --- |
| operation activity id | `az.client_request_id` |
| per-attempt service request id | `az.service_request.id` ⚠ **dot**, not underscore |
| status | `http.response.status_code` |
| retry index | `http.request.resend_count` |
| endpoint | `server.address` / `url.full` |
| namespace | `az.namespace` (`Microsoft.DocumentDB`) |
| error | `error.type` |

> ⚠ **Two `azure_core` gotchas (resolved: [§9](#9-resolved-decisions) Q5).** (1) The constant
> is `az.service_request.id` (a **dot** before `id`), while `az.client_request_id` uses an
> underscore. (2) These constants are **module-private** in `azure_core`
> ([`sdk/core/azure_core/src/http/policies/instrumentation/mod.rs`][azcore]) and cannot be
> imported. Until `azure_core` exposes them, the mapping centralizes identical string literals
> in one Cosmos-local module and an `azure_core` issue tracks exposing public constants +
> fixing the naming inconsistency. Cosmos-specific attributes (RU, sub-status, partition key
> range) use a documented `db.cosmosdb.*` namespace with underscores.

## 8. The OTel-aligned event model & the SDK↔driver split

- **The [#4619] `event.rs` model is the OTel-aligned representation — keep it there.**
  `Span { kind: SpanKind::{Operation,Attempt,Hedge}, parent: Option<SpanId>, start/end:
  TimeOffset }` + typed `Attr`s is an OpenTelemetry span tree in all but the emitter. It is
  worth retaining as the canonical in-memory shape **even though the append-log perf
  optimization is deferred and the capture engine stays OFF** — it is a data shape, not a
  hot-path commitment.
- **Retroactive spans are feasible.** A *completed* `DiagnosticsContext` can be reconstructed
  into a **backdated** OTel span tree using the raw `opentelemetry` `SpanBuilder`
  (`with_start_time`/`with_end_time`), because the `azure_core::tracing` abstraction builds
  spans at "now" and has no backdating hook. A **throwaway `otel_spans_spike`** feasibility
  test proves this on its own branch (kept separate so this contract doc stands alone).
- **SDK-vs-driver split (resolved: hybrid, default SDK-side emission).** The **driver produces
  and materializes** the `DiagnosticsContext` (and offers an opt-in exporter); the **SDK (or
  the opt-in driver exporter) emits** the public operation span + operation metrics. Default
  emission is SDK-side so there is exactly **one public span per operation** (avoids
  double-counting). Rule of thumb: *the driver produces and materializes; the SDK emits.*

## 9. Resolved decisions

| # | Decision | Resolution |
| --- | --- | --- |
| **Q1** | Collection vs a disable mode | **Always collect** full per-attempt records; a `DiagnosticsLevel`/gate governs materialization + exposure only. If/when the [#4619] gate lands, its `Mode::Off` must not become a *collection* switch (and removing an `Off` variant would be a public break — the prototype `Mode` is not `#[non_exhaustive]`, so deprecate rather than remove). A counters-only cheap tier is a possible later optimization. |
| **Q2** | FFI handle lifetime | **Opaque handle + explicit `free`** as the primitive; scoped-callback convenience may wrap it later. |
| **Q3** | Transport-tier gating knob | **New `DiagnosticsLevel { Minimal, Standard, Full }`**, mapping onto [`DiagnosticsVerbosity`][opts] internally. |
| **Q4** | Bounded-size caps | **Configurable per-representation caps** with documented defaults (8 KB string / 128 spans / 64 attempt records); first+last-per-region + exact aggregates always retained. |
| **Q5** | `azure_core` constants | Constants are private + `az.service_request.id` uses a dot: **centralize identical literals in a Cosmos-local module now**, file an `azure_core` issue to expose public constants + fix naming, then switch. |
| **Q6** | SDK-vs-driver emission | **Hybrid, default SDK-side emission**; driver offers an opt-in exporter. |
| **Q9** | Aggregated span-tree shape | **Single synthetic operation root** with each run's attempts as children. |

## 10. Scope & guardrails

- **Additive / non-breaking.** The public boundary is exactly [`diagnostics::DiagnosticsContext`][ctx],
  consumed by `azure_data_cosmos`. `CosmosResponse::diagnostics()` stays non-optional. No SemVer
  break. **This PR is documentation only — no code changes.**
- **The capture engine [#4619] stays a deferred optimization** (OFF by default); this contract
  does not depend on it and does not modify it.
- **Diagnostics are always collected** — there is no full-disable mode; the level governs
  exposure, not collection.
- **No `azure_core` / `typespec_client_core` change** is made by this contract; any new public
  constant there is a proposal (Q5).

[diag]: ./src/models/cosmos_response.rs
[ctx]: ./src/diagnostics/mod.rs
[ctxsrc]: ./src/diagnostics/diagnostics_context.rs
[opts]: ./src/options/diagnostics_options.rs
[azcore]: ../../core/azure_core/src/http/policies/instrumentation/mod.rs
