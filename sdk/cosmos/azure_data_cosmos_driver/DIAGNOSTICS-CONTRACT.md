<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->

# The driver↔SDK diagnostics contract & its OpenTelemetry mapping

> **Status: contract for the observability layer landing in this PR.** This document defines the
> **contract** between the `azure_data_cosmos_driver` and its consumers — the Rust
> `azure_data_cosmos` SDK today, and the next-major Go/Python SDKs over FFI — plus the
> **OpenTelemetry customer experience**. The emission layer it describes (the handler chain and
> the built-in metrics/tracing/log handlers) is **implemented in this same PR**, on top of the
> already-shipping `DiagnosticsContext` foundation; every addition is **additive, non-breaking,
> and off-by-default**. This doc file itself adds no code.
>
> **Emission model.** The **driver produces and materializes** one canonical, always-collected
> `DiagnosticsContext`; the **SDK decides what telemetry to emit** from it, through a chain of
> pluggable [`DiagnosticsHandler`](#4-the-emission-model--the-diagnosticshandler-chain)s with
> **tail-based (late-bound) sampling** and **cross-operation rate limiting**. Rule of thumb:
> *the driver produces and materializes; the SDK emits.*
>
> **Priority.** The contract, the OpenTelemetry metrics/traces mapping, and the emission model
> come **first**. A separately prototyped append-only *capture engine* is a **deferred
> optimization** ([WS7], salvaged from PR #4619) — it stays OFF by default and is an
> implementation detail *under* this contract, not part of it.

## 0. What exists today vs. what this PR adds

To keep the design honest, references below are tagged by where each piece lands, rather than
implying everything already exists:

- **[main]** — already on `upstream/main` (`sdk/cosmos/azure_data_cosmos_driver`): the
  always-collected [`DiagnosticsContext`][ctx] and its `requests()` / `duration()` / `status()` /
  `total_request_charge()` / `regions_contacted()` / `machine_id()` /
  `to_json_string(verbosity)` surface (plus the crate-private `is_completed()`),
  [`RequestDiagnostics`][ctx], the `DiagnosticsOptions` / `DiagnosticsVerbosity` config, and the
  non-optional `CosmosResponse::diagnostics() -> Arc<DiagnosticsContext>`. **This is the
  foundation this contract builds on, and it already exists** — so there is no separate
  foundation workstream (the earlier "WS0" was dropped and its stand-alone PR #4785 closed as
  redundant).
- **[WS2]** — the SDK-side [`DiagnosticsHandler`](#4-the-emission-model--the-diagnosticshandler-chain)
  trait + chain (the emission extension point).
- **[WS3] / [WS4] / [WS5]** — the three built-in handlers: OTel metrics, tail-sampled backdated
  tracing, and the rate-limited sampling-log handler.
- **[WS6]** — the retry-storm **bounded-size** compaction (salvaged from the closed PR #4683),
  built over the [main] context.
- **[WS8]** — SDK **op-scope wiring**: the SDK supplies each operation's identity (operation
  name, database, container) to the handler chain via a `CosmosOperationContext` on the pipeline
  `Context`, so the emitted metrics/spans carry `db.operation.name` / `db.namespace` /
  `db.collection.name` in production.
- **[WS4/WS5]** — the small driver additions the emission layer needs that upstream lacked:
  [`DiagnosticsThresholds`][thresholds] and the public `is_failure()` / `is_completed()` /
  `is_threshold_violated()` predicates and the `operation_name()` accessor.
- **[WS4c]** — the upstream span-backdating trait additions to `typespec_client_core` +
  `azure_core_opentelemetry`, shipped as the **separate** core-crate PR #4784 (not in this PR).
- **[WS7]** — the deferred, OFF-by-default append-only **capture engine** (salvaged from the
  closed PR #4619): the `event.rs` `Span`/`Attr` model, the encode path, and the per-op /
  per-attempt wall-clock capture optimization. Not in this PR.
- **[design]** — a surface this contract sketches that is **not yet implemented** (e.g. the
  SDK-facing `DiagnosticsLevel { Minimal, Standard, Full }`, which maps onto the [main]
  `DiagnosticsVerbosity`).

> **[WS2]–[WS8] all ship together in one combined observability PR (#4789)** — this contract doc
> is folded into it. They are additive and off-by-default (the OTel handlers are behind the
> `otel_metrics` / `otel_tracing` features) and are built entirely on the existing [main]
> foundation.
>
> ⚠ **Correction to the earlier draft of this doc (PR #4684).** An earlier draft assumed the
> `DiagnosticsContext` foundation was **not yet on `main`** and would land in a blocking-root
> workstream first. That is wrong: the full foundation — `DiagnosticsContext`,
> `CosmosResponse::diagnostics()`, `RequestDiagnostics`, `DiagnosticsVerbosity`, and
> `DiagnosticsOptions` — is **already on `upstream/main`**. The one diagnostics type that is
> **not** upstream is [`DiagnosticsThresholds`][thresholds]; this PR adds it alongside the
> emission handlers. The tags above reflect that reality.

## 1. Requirements this contract satisfies

Each requirement below traces to a section here (and to the workstream that implements it).

| # | Requirement | Where addressed in this doc |
| -- | ----------- | --------------------------- |
| R1 | Long-running (10–12 h) benchmark observability: **quiet at steady state, rich on error**, enough to root-cause a 5–10 min error window | The whole design; realized by §4 (chain) + §5 (tail-sampling + rate-limit) + §10.2 (always-on metrics) |
| R2 | Emit **OTel metrics** (top priority) — stable semconv first | §10.2 (operation-level metrics) |
| R3 | Emit **OTel tracing** with rich, Java-like features | §10.4 (span tree) |
| R4 | **Tail-based / late-bound sampling**: decide span emission *after* an op completes, by latency/outcome | §5.2 (the emit/skip decision) |
| R5 | **Rate-limit under error storms**: cap emissions so 10k errors/sec don't peg CPU | §5.3 (cross-operation rate limiting) + §8 (bounded per-artifact size) |
| R6 | **Driver → context only** (≤ debug level); **SDK decides emission** from the context | §4 (handler chain) + §11 (SDK↔driver split) |
| R7 | Metrics power **client-side Grafana dashboards** (account/region SLA) | §10.2 (metric instruments + dimensions) |
| R8 | Metrics carry **dimensions** (operation, status, consistency, region…) → per-combination series | §10.2 + §10.3 (attribute tiers) |
| R9 | Don't lock/log a line for **every fast op** — steady state must be near-zero cost | §5 defaults (tail-sampling skips fast successes) + §10.2 (low-cardinality always-on) |

## 2. Why a contract first

The driver already produces rich per-operation diagnostics and hands them back through a
cheap handle. Two things were underspecified, and the SDK requirements must drive the driver
design (not the other way around):

1. **One implicit shape.** Materialization is JSON-only and implicit
   ([`to_json_string`][ctx] [main]). Different consumers want different shapes: metrics want a
   structured object, tracing wants spans, logs want a string. JSON is the single costliest
   step (bench: struct build ~3.7 µs → +detailed JSON ~6.6 µs), so forcing it on everyone is
   wasteful — and lossy/expensive across an FFI boundary.
2. **No agreed emission decision.** *Who* decides whether an operation is even worth a span or
   a log line, and *where*? The driver must not emit eagerly (R6); the decision belongs to the
   SDK, computed **after** the operation completes (R4).

So we fix the **contract**, the **OpenTelemetry mapping**, and the **emission model** first;
the hot-path capture engine [WS7] is a deferrable optimization.

## 3. The contract in one picture

```text
azure_data_cosmos_driver                          azure_data_cosmos (SDK)
────────────────────────                          ───────────────────────
operation completes
        │
        ▼
   Arc<DiagnosticsContext>   ── cheap handle ──▶   DiagnosticsHandler chain   [WS2]
   [main] ALWAYS collected,       (atomic incr)         ├─ CosmosMetricsHandler  [WS3]  always-on, low-cardinality
   ALWAYS returned; the                                ├─ CosmosTracingHandler  [WS4]  tail-sampled, backdated spans
   driver emits only                                   └─ SamplingLogHandler    [WS5]  rate-limited log lines
   debug-level `tracing`                                     ▲ each handler consumes the COMPLETED context
   itself — never eager                                      │ and maps it to its own OTel/semconv view
   OTel spans/metrics.
```

Four invariants:

- **P1 — The handle is cheap and unconditional.** In Rust it *is*
  `CosmosResponse::diagnostics() -> Arc<DiagnosticsContext>` [main]. Cloning it is an atomic
  increment. It is **always** returned; `diagnostics()` is **non-optional**.
- **P2 — The driver produces & materializes; the SDK emits.** The driver never starts spans at
  operation-start and never talks to an OTel exporter on the default path. Emission is an SDK
  concern, realized by the handler chain (§4). This is R6.
- **P3 — Emission is decided from a *completed* context.** Handlers run once, at operation
  completion, so the SDK can decide *after the fact* — by latency/outcome — whether an op is
  worth a span or a log line (tail-based sampling, §5.2). This is R4.
- **P4 — The level/threshold governs EXPOSURE and DEPTH, never COLLECTION.** A diagnostics
  *level* bounds the high-cardinality **transport-level** detail a materialization includes; it
  never removes operation-level diagnostics and never disables collection.

There is **no parallel diagnostics model**: the driver owns one canonical
[`DiagnosticsContext`][ctx] [main] and every representation is a view over it.

## 4. The emission model — the `DiagnosticsHandler` chain

**The SDK owns an ordered chain of `DiagnosticsHandler`s; each consumes the completed context
and decides, independently, what (if anything) to emit.** This is the extension point that
realizes R6 (driver produces, SDK decides) and hosts the sampling (R4) and rate-limiting (R5)
policies.

```rust
// azure_data_cosmos (SDK) — [WS2]
pub trait DiagnosticsHandler: Send + Sync {
    /// Called once, at operation completion, with the completed driver context and the
    /// active trace `Context`. Handlers decide independently whether/what to emit.
    fn handle(&self, ctx: &DiagnosticsContext, cx: &Context);
}

// Built-in handlers registered by the app:
struct CosmosMetricsHandler { /* [WS3] always-on, low-cardinality      */ }
struct CosmosTracingHandler { /* [WS4] tail-sampled, backdated spans   */ }
struct SamplingLogHandler   { /* [WS5] rate-limited log lines          */ }
```

Properties:

- **Ordered, `Arc<dyn DiagnosticsHandler>`, no-op when empty.** Zero registered handlers ⇒ zero
  emission overhead (the driver still collects the context, but nothing is emitted). Ordering is
  deterministic.
- **Cosmos-local now, promotable later (D1).** The chain is built **inside `azure_data_cosmos`**
  as an **additive, swappable** surface — deliberately *not* over-fitted to the Java
  `CosmosDiagnosticsHandler` shape. If other Azure SDKs want the same extension point, the
  abstraction can be **promoted into `azure_core` later** without breaking Cosmos callers.
- **SDK maps to its own semconv view.** Each handler translates the driver's
  `DiagnosticsContext` into its **own** OTel/semantic-convention representation (§10); the
  driver's internal model never leaks into the emitted telemetry.

This replaces the earlier "materializer-only" framing: materialization (§6) is still how a
handler *reads* the context, but the **decision + emission** now has an explicit home.

## 5. Three orthogonal controls — do not conflate them

The earlier draft partly merged these. They are **independent** knobs:

### 5.1 `DiagnosticsLevel` — static **DEPTH**

`DiagnosticsLevel { Minimal, Standard, Full }` [design] is a **static config** that bounds *how
much transport-level detail a materialization includes* (§7). It answers **"how deep?"** — not
"whether to emit" and not "how many". It never disables collection (P4).

### 5.2 Tail-based sampling — dynamic **WHETHER** (R4)

A **per-operation, late-bound decision**, computed from the *completed* context, that answers
**"emit a span/log for this op at all?"**:

```rust
fn should_emit_span(ctx: &DiagnosticsContext, thresholds: &DiagnosticsThresholds) -> bool {
    ctx.is_completed()
        && (ctx.is_failure() || ctx.is_threshold_violated(thresholds) /* || level == Full */)
}
```

- **Default (D4):** a fast success (e.g. a 5 ms point read) emits **no span**; a slow op
  (≥ threshold) or a failure **does**. This is exactly R4/R9 — no per-fast-op cost.
- **Thresholds use [`DiagnosticsThresholds`][thresholds] [WS4/WS5]** (added by this PR) and are
  **configurable via the standard Rust options chain**; start from Java-like defaults, tunable before GA.
- This is the piece the earlier draft under-specified. It is distinct from `DiagnosticsLevel`
  (§5.1): the level sets *depth of what a span contains*; tail-sampling sets *whether a span
  exists*.

### 5.3 Cross-operation rate limiting — bounded **HOW MANY** (R5)

A **cross-operation cap** on emission frequency during a storm, so 10k errors/sec don't all get
logged/traced and peg the CPU:

- A reusable **token-bucket / count-per-interval** limiter (default e.g. **≤ N lines/min**),
  used by `SamplingLogHandler` [WS5] (and optionally by the tracing handler under storms).
- **Always allow a bounded number of failures** through, then emit **one** `"suppressed N until
  reset"` line per window — so a storm is visible but bounded.
- **On by default (D5)** — production-ready by default.
- This is **independent** of §8's per-artifact size bound: §8 caps the size of a *single*
  diagnostics artifact; §5.3 caps the *number of artifacts* emitted per interval.

## 6. The three representations (views over one model)

| Materializer | Consumer intent | Backed by |
| --- | --- | --- |
| **Structured object** | metrics | [`DiagnosticsContext::requests()`][ctx] [main], reduced into an operation roll-up |
| **OTel span tree** | traces | reconstructed from [`DiagnosticsContext`][ctx] + per-attempt [`RequestDiagnostics`][ctx] [main]; a `Span`/`Attr` in-memory form is the OTel-aligned shape (kept in [WS7]) |
| **String** | logs | [`DiagnosticsContext::to_json_string(verbosity)`][ctx] [main] |

Materialization is **explicit, lazy, and per-representation**: each is paid only when a handler
asks for it, so the expensive JSON step is never paid on a metrics-only or span-only path.

## 7. Where the level / threshold applies

**Gating bounds high-cardinality TRANSPORT-level telemetry — it never eliminates
diagnostics.**

| Tier | Examples | Cardinality | Gating |
| --- | --- | --- | --- |
| **Operation-level** | operation name, final status, request/retry/throttled counts, total RU, total duration, regions contacted | low | **Always on.** Never gated away. |
| **Transport-level** | per-replica / per-partition (partition key range, feed range), endpoint, direct-mode channel, transport kind/security/http-version, per-attempt RU/latency | high | **Gated by `DiagnosticsLevel` / threshold.** Included on error / slow / high level; summarized or elided on a fast-success low level. |

`DiagnosticsLevel { Minimal, Standard, Full }` [design] maps onto the internal
`DiagnosticsVerbosity` [main]:

- `Minimal` — operation-level only.
- `Standard` — + region-grouped/deduplicated transport summary (`Verbosity::Summary`).
- `Full` — + every per-attempt transport record (`Verbosity::Detailed`).

> **Collection is not gated.** Operation-level metrics are *computed by iterating the
> per-attempt records* ([`requests()`][ctx] [main]), so "cheap op-level only" is not achievable
> by dropping transport collection. The driver **always collects** the full per-attempt records
> and the level gates only *materialization + exposure* (P4).

## 8. Bounded-size guarantee (retry storms)

**Every materialized representation has an upper bound on size that is independent of attempt
count** [WS6], so a `410`/`429` retry storm or a large fan-out query cannot produce an
unbounded object, span tree, or string.

- **Mechanism.** `DiagnosticsVerbosity::Summary` [main] groups requests by region, keeps first +
  last per region in full, and deduplicates the middle by `(endpoint, status, sub_status,
  execution_context)` with count + min/max/P50.
- **Configurable caps [WS6] (salvaged from PR #4683).** The per-attempt record list is bounded
  by a `DiagnosticsOptions::max_request_diagnostics` knob (default **512**, min 16), keyed on
  `(region, endpoint, status, sub_status, execution_context)`. Consecutive near-identical
  retries collapse to **first + last of each run + a count**; an order-robust global key-bucket
  fallback holds the bound under a region ping-pong (`A→B→A→B`); the per-run rollup is itself
  bounded so a high-cardinality `410` fan-out cannot grow the artifact by topology.
- **Per-representation, independent caps.** The 512 structured-object record cap does **not**
  bound the span tree. A span emitter does not emit one span per retained record; following
  §10.4 it collapses each run to a single span carrying a repeat `count`, so the span count
  tracks the number of *runs* and applies its own **128-span** target on top. So 512 retained
  records map to ≤ 128 spans without the two caps needing to agree.
- **Truncation is marked, never silent** — the structured object carries explicit compaction
  metadata (true vs retained count, per-run rollup, and a truncation marker when a cap is hit).

This is the **per-artifact size bound**; it is distinct from the **cross-operation rate limit**
(§5.3), which caps how *many* artifacts are emitted per interval. The append-only capture
engine that realizes this cheaply is the deferred optimization [WS7].

## 9. The FFI boundary (Go / Python next-major)

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
   level)` entry point where `representation ∈ {Metrics, Spans, Log}`.
3. **No forced JSON.** JSON is one representation requested explicitly; a Go metrics exporter
   never triggers it.
4. **Explicit lifetime (opaque handle + `free`).** The handle is freed by an explicit
   `cosmos_diag_free`; refcounting stays on the Rust side. A scoped materialize-then-drop
   callback may wrap it later.
5. **Bounded output.** Every materialization honors the bounded-size guarantee (§8), so an FFI
   buffer can be sized predictably even under a retry storm (read the configured cap, not a
   hardcoded value).

The exact wire encoding of each representation (packed struct vs flatbuffer vs span callback)
is a follow-up implementation detail; the contract fixes only "opaque handle + explicit
per-representation materialize + bounded output".

## 10. OpenTelemetry mapping

This mapping targets the **OpenTelemetry semantic conventions for Azure Cosmos DB**
(`open-telemetry/semantic-conventions`, `docs/db/cosmosdb.md`). The names below are the **real**
semconv names.

> ⚠ **Naming correction (was wrong in PR #4684).** The earlier draft used non-existent
> `db.cosmosdb.*` names (`db.cosmosdb.operation.duration`, `db.cosmosdb.status_code`,
> `db.cosmosdb.sub_status_code`, `db.cosmosdb.request_charge`, `db.operation`). The correct
> names are `db.client.operation.duration`, `db.response.status_code`, `db.operation.name`,
> with Cosmos-specifics under the `azure.cosmosdb.*` namespace. This section uses the correct
> names throughout.

### 10.1 System, span name, span kind

- `db.system.name = "azure.cosmosdb"` (set at span creation).
- **Span name** follows the DB span-name convention (e.g. `"read_item orders"` — operation +
  target).
- **Span kind** = `CLIENT`. Span status follows the OTel *Recording Errors* guidance.

### 10.2 Operation-level metrics (always-on, low cardinality)

Source: an operation roll-up over [`DiagnosticsContext::requests()`][ctx] [main], emitted by
`CosmosMetricsHandler` [WS3]. **Emit the *stable* instruments first**, with only low-cardinality
attributes. This powers client-side Grafana dashboards (R7) with per-combination series (R8).

| Instrument | Stability | Kind | Unit | Description |
| --- | --- | --- | --- | --- |
| `db.client.operation.duration` | stable | histogram | `s` | End-to-end operation duration — **the primary metric**. |
| `db.client.response.returned_rows` | development | histogram | `{row}` | Rows returned in the result set. |
| `azure.cosmosdb.client.operation.request_charge` | development | histogram | `{request_unit}` | Request units (RU) consumed by the operation. |
| `azure.cosmosdb.client.active_instance.count` | development | up-down counter | `{instance}` | Number of active Cosmos client instances. *(Deferred — not emitted yet; pending real client-lifecycle wiring, see [#4789](https://github.com/Azure/azure-sdk-for-rust/pull/4789).)* |

**Always-on metric attributes (low cardinality, D7):** `db.operation.name`,
`db.response.status_code`, `db.collection.name`, `db.namespace`, `error.type`, `server.address`,
`db.system.name`. **High-cardinality attributes are OFF by default** (opt-in) — see §10.3.

> **Metrics transport gap (D2).** Neither `typespec_client_core` nor `azure_core_opentelemetry`
> currently exposes a `Meter`/`Histogram`/`Counter` abstraction. `CosmosMetricsHandler` [WS3]
> therefore emits via the **raw `opentelemetry` metrics API behind an off-by-default feature**
> (Cosmos owns a `Meter`) now, and we drive a first-class `Meter` abstraction into `azure_core`
> as a follow-up, then migrate. **Develop Cosmos-local; move to `azure_core` if there is shared
> interest.**

### 10.3 Attributes — stable vs. development tiers

Cosmos spans (and, where low-cardinality, metrics) use these semconv attributes. The **stable**
tier is safe as always-on metric dimensions; the **development** tier is span-first / opt-in as
a metric dimension to control time-series cardinality (D7).

**Stable** (safe as always-on metric dimensions)

| Attribute | Notes |
| --- | --- |
| `db.operation.name` | Canonical snake_case op name — use verbatim (see §10.3.1). |
| `db.collection.name` | Cosmos container name. |
| `db.namespace` | Database name. |
| `db.response.status_code` | Cosmos status code (string). 4xx/5xx are errors. |
| `error.type` | Present iff the operation failed; matches status or exception type. |
| `server.address` / `server.port` | Host / port (port only when not default 443). |
| `db.operation.batch.size` | Number of ops in a batch. |
| `db.query.text` | Query text (parameterized; sanitized per DB span rules). |
| `db.stored_procedure.name` | Stored-procedure name, when applicable. |
| `user_agent.original` | SDK-generated user-agent string. |

**Development** (span-first; opt-in as metric dimensions)

| Attribute | Notes |
| --- | --- |
| `azure.cosmosdb.connection.mode` | `gateway` or `direct`. |
| `azure.cosmosdb.consistency.level` | `Eventual` / `Session` / `Strong` / `BoundedStaleness` / `ConsistentPrefix`. |
| `azure.cosmosdb.operation.contacted_regions` | Ordered list of contacted regions (cross-region call if > 1). |
| `azure.cosmosdb.operation.request_charge` | RU consumed (double). |
| `azure.cosmosdb.response.sub_status_code` | Cosmos sub-status code (int). |
| `db.response.returned_rows` | Row count in the result set (int). |
| `azure.cosmosdb.request.body.size` | Request payload size in bytes. |
| `azure.client.id` | Stable per-client instance id (see D10). |
| `azure.resource_provider.namespace` | `Microsoft.DocumentDB`. |

> **Client instance id (D10).** `azure.client.id` and the active-instance metric use a stable
> per-client id. Prefer `vmId`; when VM metadata is unreachable, fall back to a **static GUID**
> so two requests can be attributed to the same `CosmosClient`/driver instance. **Check whether
> [`DiagnosticsContext`][ctx] already carries this before adding it.**

#### 10.3.1 Canonical `db.operation.name` values

Use the semconv well-known values verbatim — e.g. `read_item`, `create_item`, `upsert_item`,
`replace_item`, `patch_item`, `delete_item`, `query_items`, `read_all_items`, `read_many_items`,
`execute_batch`, `execute_bulk`, `query_change_feed`, `read_container`, `create_container`, … .
If none applies, use a language-agnostic snake_case method name.

### 10.4 Traces (span tree) — reconstructed & backdated

Emitted by `CosmosTracingHandler` [WS4], **only when tail-sampling (§5.2) says so**.

| `DiagnosticsContext` element | OTel span |
| --- | --- |
| operation (root) | root span, kind `CLIENT`, name `<operation> <target>`; window backdated over [`duration()`][ctx] [main] |
| each retained `RequestDiagnostics` (attempt / hedge leg) | child span, kind `CLIENT` |
| request-event timeline | timed span **events** on the attempt span |
| hedging | a hedge span with terminal state + regions |
| aggregated multi-run op | a **single synthetic operation root** spanning the first run's start to the last run's end, each run's attempts as children |

- **Backdating (Gap A / D3).** A *completed* `DiagnosticsContext` is reconstructed into a
  **backdated** span tree — because the current `typespec_client_core` `Tracer`/`Span` traits
  build spans at "now" and have **no** hook to set explicit start/end timestamps. Two-track
  plan: **(a) upstream** — add `start_span_at`/`end_at` (or a `SpanBuilder`-style API) to the
  `typespec_client_core` traits + `azure_core_opentelemetry` impl [WS4c]; **(b) Cosmos-local
  fallback** — the raw `opentelemetry` `SpanBuilder` (`with_start_time` / `end_with_timestamp`),
  proven by the salvaged PR #4685 spike. **Ship (b) now behind one internal seam; migrate to (a)
  when it lands.**
- **Build from the bounded list.** The emitter builds the tree from the retained attempt list
  ([`requests()`][ctx], bounded by §8), collapsing each run to one span carrying a repeat
  `count` — never one span per pre-compaction attempt (which could reach the record cap).

### 10.5 `azure_core` attribute alignment & the two upstream gaps

Where a Cosmos span overlaps `azure_core`-emitted HTTP spans, reuse the `azure_core`
span-attribute names so the spans correlate:

| Diagnostics field | `azure_core` attribute name |
| --- | --- |
| operation activity id | `az.client_request_id` |
| per-attempt service request id | `az.service_request.id` ⚠ **dot**, not underscore |
| HTTP status | `http.response.status_code` |
| retry index | `http.request.resend_count` |
| endpoint | `server.address` / `url.full` |
| namespace | `azure.resource_provider.namespace` (`Microsoft.DocumentDB`) |
| error | `error.type` |

> ⚠ **Two `azure_core` gotchas (D2/D3).** (1) The constant is `az.service_request.id` (a **dot**
> before `id`), while `az.client_request_id` uses an underscore. (2) These constants are
> **module-private** in `azure_core`
> ([`http/policies/instrumentation/mod.rs`][azcore]) and cannot be imported. Until `azure_core`
> exposes them, centralize identical string literals in one Cosmos-local module and file an
> `azure_core` issue to expose public constants + fix the naming inconsistency. **Cosmos-specific
> attributes use the real `azure.cosmosdb.*` namespace** (not the earlier draft's `db.cosmosdb.*`).

## 11. The OTel-aligned event model & the SDK↔driver split

- **Keep an OTel-aligned in-memory span model.** `Span { kind, parent: Option<SpanId>,
  start/end: TimeOffset }` + typed `Attr`s is an OpenTelemetry span tree in all but the emitter,
  and is worth retaining as the canonical in-memory shape — a data shape, not a hot-path
  commitment. It rides with the deferred capture engine [WS7]; the default path does not require
  it.
- **SDK-vs-driver split (D8): default SDK-side emission.** The **driver produces and
  materializes** the `DiagnosticsContext` (and may offer an *opt-in* exporter); the **SDK
  emits** the public operation span + operation metrics via the handler chain (§4). Default
  emission is SDK-side so there is exactly **one public span per operation** (no double-count).
  Rule of thumb: *the driver produces and materializes; the SDK emits.*

## 12. Resolved decisions

These map to the plan's decisions D1–D10.

| # | Decision | Resolution |
| --- | --- | --- |
| **D1** | Emission model | **`DiagnosticsHandler` chain** in the SDK (not a materializer-only API). Built Cosmos-local, additive/swappable, promotable to `azure_core` later. (§4) |
| **D2** | Metrics transport (Gap B) | No `Meter` abstraction exists in `azure_core`/`typespec_client_core`. Emit via the **raw `opentelemetry` metrics API behind an off-by-default feature** now; drive an `azure_core` `Meter` follow-up, then migrate. (§10.2) |
| **D3** | Span backdating (Gap A) | Two-track: **(a) upstream** trait additions to `typespec_client_core` + `azure_core_opentelemetry` [WS4c]; **(b) Cosmos-local** raw-`opentelemetry` `SpanBuilder` now. Ship (b) behind one seam; migrate to (a). (§10.4) |
| **D4** | Tail-sampling policy | Default: emit a span only when `is_completed` and (`is_failure` or `is_threshold_violated`). Fast successes emit no span; thresholds configurable via the standard options chain, Java-like defaults. (§5.2) |
| **D5** | Rate limiting | Token-bucket / count-per-interval limiter used by `SamplingLogHandler`; always allow a bounded number of failures + one "suppressed N" line per window. **On by default.** (§5.3) |
| **D6** | Semconv naming | Real names only: `db.client.operation.duration`; `db.operation.name`, `db.response.status_code`, `azure.cosmosdb.*`, `db.system.name="azure.cosmosdb"`. Centralize attribute literals Cosmos-local. (§10) |
| **D7** | Metric cardinality | Operation-scope tags always-on; high-cardinality tags (consistency level, per-request region, partition-key-range, endpoint, replica) off by default, opt-in. (§10.2/§10.3) |
| **D8** | SDK vs driver emission | Default **SDK-side** emission → exactly one public span per op; driver may expose an opt-in exporter. (§11) |
| **D9** | Bounded size vs capture engine | Land the **bounded-size guarantee** [WS6] as a customer-facing property; keep the append-only capture engine [WS7] as an OFF-by-default internal optimization. (§8) |
| **D10** | Client instance id | Stable per-client id feeds the active-instance metric + `azure.client.id`. Prefer `vmId`, fall back to a static GUID; check whether `DiagnosticsContext` already carries it. (§10.3) |

## 13. Scope & guardrails

- **Additive / non-breaking.** The public boundary is [`diagnostics::DiagnosticsContext`][ctx]
  [main], consumed by `azure_data_cosmos`. `CosmosResponse::diagnostics()` stays non-optional. No
  SemVer break. **This contract doc itself adds no code or public API; it is folded into the
  combined observability PR (#4789), which carries the implementation and its CHANGELOG entries.**
- **Off-by-default / feature-gated.** New telemetry (the metrics feature, the capture engine)
  is OFF by default (behind a feature or an unset exporter), so merging the implementation
  workstreams cannot change existing behavior.
- **Diagnostics are always collected** — there is no full-disable mode; the level governs
  exposure/depth, not collection (P4).
- **No `azure_core` / `typespec_client_core` change** is *required* by this contract; the
  upstream `Meter` (D2) and span-backdating (D3) additions are proposals pursued in parallel.

[thresholds]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/options/diagnostics_thresholds.rs
[ctx]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/src/models/cosmos_response.rs
[azcore]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/src/http/policies/instrumentation/mod.rs
