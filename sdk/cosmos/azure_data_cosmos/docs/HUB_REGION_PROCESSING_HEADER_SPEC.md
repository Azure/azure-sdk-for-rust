# Hub-Region Processing-Only Header Spec for `azure_data_cosmos`

<!-- cspell:words PPAD misrouting dashboarding -->

**Status:** Draft / Iterating — **rebased to `release/azure_data_cosmos-previews`**
**Confidence:** Medium. The original `ClientRetryPolicy` call graph (single-master read budget, `before_send_request`/`should_retry_on_session_not_available` flow, `GlobalEndpointManager` → `LocationCache` mutex layering) was traced against `main` and remains accurate for that branch, but **on `release/azure_data_cosmos-previews` (this branch) the SDK retry layer is no longer reached by user-facing data-plane operations** — every read/query/write call on `ContainerClient`/`DatabaseClient`/`CosmosClient`/`OffersClient` dispatches through `azure_data_cosmos_driver::CosmosDriver::execute_operation` directly, bypassing `CosmosPipeline → BackOffRetryHandler → ClientRetryPolicy`. The implementation site for this feature on previews is therefore the **driver**, not the SDK. §4 (SDK code-change targets) is retained as a parity reference for the .NET SDK's behavioral shape; the actual implementation lands in the driver per [ALT-6](#alt-6--implement-the-latch--header-as-a-driver-side-policy), which is now the **primary path** rather than a forward target. The backend's case-sensitivity for the `x-ms-cosmos-hub-region-processing-only` header value is **assumed** to match .NET's `bool.TrueString` (`"True"`) and is not verified against a backend contract reference. Driver-crate file citations in this spec are relative to `sdk/cosmos/azure_data_cosmos_driver/src/`.
**Date:** 2026-04-29
**Authors:** (team)
**Crate (implementation home on previews):** `azure_data_cosmos_driver` — see [§1.5](#15-two-crate-landscape-sdk-vs-driver) and [ALT-6](#alt-6--implement-the-latch--header-as-a-driver-side-policy).
**Crate (parity reference, .NET shape):** `azure_data_cosmos` — see §4. The SDK-resident pseudocode in §4 documents the .NET-equivalent semantics; on previews it is reference material and is **not** the place implementers should land code.
**Tracks:** [#4303](https://github.com/Azure/azure-sdk-for-rust/issues/4303)
**Mirrors:** [Azure/azure-cosmos-dotnet-v3#5447](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5447)

---

## Table of Contents

1. [Goals & Motivation](#1-goals--motivation)
   - [1.5 Two-crate landscape (SDK vs Driver)](#15-two-crate-landscape-sdk-vs-driver)
2. [Architectural Overview](#2-architectural-overview)
3. [Behavior Specification](#3-behavior-specification)
4. [Code Changes](#4-code-changes)
5. [Side Effects & Risk](#5-side-effects--risk)
6. [Alternatives Considered](#6-alternatives-considered)
7. [Testing Strategy](#7-testing-strategy)
8. [Open Questions & Future Work](#8-open-questions--future-work)

---

## 1. Goals & Motivation

### Problem Statement

On a **single-master** Cosmos DB account using **Session** consistency, a region failover or
failback opens a catch-up window during which a satellite read region may not yet have applied
the latest writes. A read against the lagging region returns `404` with sub-status
`1002` (`READ_SESSION_NOT_AVAILABLE`).

Today the Rust `ClientRetryPolicy`'s session-not-available path for read requests on a
single-master account allows **one retry total** and routes that retry via write locations
(`retry_request_on_preferred_locations: false`) rather than rotating preferred read regions.
Concretely, in `should_retry_on_session_not_available` the counter is incremented at entry,
and the `!can_use_multiple_write_locations` branch returns `DoNotRetry` once
`session_token_retry_count > 1`. That single client-side reroute cannot ask the backend to
*guarantee* it serves the request from the hub (write) region, nor can it latch that
requirement across whatever retries do follow within the same operation, so the current
behavior offers no parity with the .NET fix described below.

The .NET SDK ([azure-cosmos-dotnet-v3#5447](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5447))
shipped a fix: when a 404/1002 fires inside a single-master operation, the SDK attaches a new
request header — `x-ms-cosmos-hub-region-processing-only: True` — to the retry and every
subsequent retry within the same operation. The backend honors the header by serving the
request from the hub (write) region, which is guaranteed to be current. .NET also retries
across all read endpoints (`sessionTokenRetryCount > ReadEndpoints.Count`) which gives the
header more attempts to ride on; Rust's existing single-retry budget gives the header exactly
one ride. This spec brings header-emission parity within Rust's current budget; extending the
budget for full retry-count parity is captured under [Future Work](#future-work). Issue
[#4303](https://github.com/Azure/azure-sdk-for-rust/issues/4303) tracks the parity work.

### Primary Target

Single-master Session-consistency reads during a region failover or failback catch-up window.
Specifically: any `read_item` / `query` / point-read on a single-master account where a
satellite region briefly lags behind the hub. The same gate applies to all
read-shaped customer operations on single-master accounts that flow through the
driver's status-code retry hook — including (but not limited to) `read_item`,
point-reads, `query` (single-partition and cross-partition), `read_many_*` (when added),
and change-feed reads — because they share the same 404/1002 → `SessionRetry` evaluation
in `evaluate_transport_result`. **`read_many_*` and change-feed are not implemented in
the driver today**; when they land, the latch fires automatically because the trigger
is centralized in `evaluate_transport_result`'s 404/1002 arm — no per-operation wiring
is required. Writes are out of scope (writes do not encounter 404/1002 the same way and
.NET also does not emit the header on write retries).

On `release/azure_data_cosmos-previews`, every such call dispatches through
`CosmosDriver::execute_operation` (`driver/cosmos_driver.rs`); the latch and header emission
must therefore land in the driver's operation pipeline. See §1.5 and [ALT-6](#alt-6--implement-the-latch--header-as-a-driver-side-policy).

#### 1.6 Gateway-mode interaction (GW v1 vs GW v2)

The gateway version affects how quickly a 404/1002 propagates back to the client
retry layer, which in turn affects how quickly the latch (set in
`evaluate_transport_result`'s 404/1002 arm) fires:

- **GW v2** — service iterates over replicas in the local region exactly once and
  returns failure to the SDK promptly. The cross-region retry (and therefore the latched
  hub-region-processing-only header) fires faster, which is the desired behavior for
  failover/failback latency.
- **GW v1** — service performs extensive intra-region retries before returning
  failure. Until Gateway offers an opt-in to `RemoteRegionPreferred` handling on GW v1,
  customers on GW v1 will see longer time-to-first-cross-region-retry and therefore a
  larger window during which the header is not yet emitted. This is a known service-side
  gap and is **not** addressable in this client-side change; it is called out here so
  that the SDK's observed failover-latency telemetry is interpretable. Latency-sensitive
  workloads on GW v1 will benefit most when the Gateway side ships
  `RemoteRegionPreferred`.

This client-side spec is correct for both GW v1 and GW v2 — the trigger boundary,
gate, and header value are unchanged. What changes is *when* the trigger fires
relative to the original request.

### Goals

1. **Header-emission parity with .NET, within Rust's existing retry budget.** Mirror .NET's
   gate (single-master only) and trigger (any 404/1002 on the path) such that whatever retries
   the existing Rust budget does permit carry the header. We do **not** extend the read-retry
   budget in this work item; see [Future Work](#future-work) and [ALT-5](#alt-5--extend-rusts-read-retry-budget-to-match-nets-readendpointscount-bound).
2. **Surgical, additive change.** No public-API change, no error-type change, no new options
   surfaced to callers. The feature is fully automatic.
3. **Forward-compatible on the wire.** Older backend builds that do not yet honor the header
   ignore it; behavior degrades gracefully to today's baseline.
4. **Contained blast radius.** All logic lives inside `ClientRetryPolicy`; existing retry
   budgets, delays, and status-code routing are unchanged.

### Non-Goals (This Spec)

- The 403/3 (`NotWriteRegion`) **skip-set rotation** flow described in the issue text. The
  .NET PR did not implement it; mirroring .NET is the explicit constraint of this work item.
  See [§6 Alternatives Considered](#6-alternatives-considered) for why this is intentional.
  **Update:** this was a non-goal for the header-latch work item only; it has since been
  implemented in the driver as the anticipated follow-up (the §4.2 latch is the hook point)
  — see [azure-sdk-for-rust#4555](https://github.com/Azure/azure-sdk-for-rust/pull/4555).
- A user-facing toggle to disable the header. The feature mirrors .NET's "always on for
  single-master" contract.
- Backend rollout coordination. The header is forward-compatible; older backends ignore it.
- Multi-master account behavior. Multi-master accounts never emit the header.
- **Per-partition automatic failover (PPAD) integration.** When PPAD is enabled, the
  effective "hub region" is a *per-partition* concept rather than an account-wide one
  (each partition has its own current write region, which can shift independently). This
  spec emits a single account-scoped header value because (a) the .NET PR
  ([Azure/azure-cosmos-dotnet-v3#5447](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5447))
  also emits it account-scoped — partition-scoped routing happens server-side via
  PartitionKeyRange-level state — and (b) the driver's `AccountMetadataCache` exposes
  account-level write-region topology, not partition-scoped routing. **Validation:** the
  implementation PR's tests must include at least one PPAD-enabled single-master account
  scenario to confirm the backend correctly routes the latched retry to the
  per-partition hub region (i.e. the server is the source of truth for partition→hub
  mapping, and emitting the account-scoped header is sufficient). If post-rollout
  telemetry shows misrouting on PPAD accounts, file a follow-up to plumb partition-scoped
  hub-region resolution through `PartitionKeyRangeCache` and emit a partition-aware
  variant of the header.
  **Update:** per-partition hub-region caching has since been implemented in the driver —
  see [azure-sdk-for-rust#4555](https://github.com/Azure/azure-sdk-for-rust/pull/4555).

### 1.5 Two-crate landscape (SDK vs Driver) — previews architecture

On `release/azure_data_cosmos-previews` the data plane has migrated into the driver. The two
crates are no longer independent: `sdk/cosmos/azure_data_cosmos/Cargo.toml:20` declares
`azure_data_cosmos_driver = { workspace = true, default-features = false }`, and every SDK
client (`ContainerClient`, `DatabaseClient`, `CosmosClient`, `OffersClient`) holds
`pub(crate) driver: Arc<CosmosDriver>` (`src/clients/mod.rs:33`) and routes data-plane calls
to `self.context.driver.execute_operation(...)`. The legacy `CosmosPipeline →
BackOffRetryHandler → ClientRetryPolicy` chain is still compiled but is reached only by two
internal call sites (`handler/container_connection.rs:155` for per-container metadata,
`routing/partition_key_range_cache.rs:304` for the PKR cache); no customer read or write
goes through it.

| Concern                          | `azure_data_cosmos` (SDK) | `azure_data_cosmos_driver` (Driver) | Status on previews |
|----------------------------------|---------------------------|--------------------------------------|--------------------|
| Public client API surface        | ✅ Owns                   | —                                    | Stable             |
| Customer data-plane dispatch     | Forwards to driver via `context.driver.execute_operation(...)` | ✅ `CosmosDriver::execute_operation` is the entry point | Implemented |
| Outbound default headers         | —                         | ✅ `cosmos_headers::apply_cosmos_headers()` (free function called at `driver/cosmos_driver.rs:301`) | Implemented |
| Request signing                  | —                         | ✅ `request_signing` module (`driver/transport/request_signing.rs`) | Implemented |
| Status-code retry (404/1002, 429, 503, timeout, …) | Legacy `ClientRetryPolicy` retained for metadata/PKR-cache calls only | ✅ `evaluate_transport_result` (`driver/pipeline/retry_evaluation.rs:31`); 404/1002 → `SessionRetry` already at `:70` (`status.is_read_session_not_available() && retry_state.can_retry_session()`) | Driver-owned for customer ops |
| Transport-failure retry           | —                         | ✅ `CosmosDriver::execute_operation` (idempotent / `definitely_not_sent`) | Implemented |
| Account topology / region tracking | Legacy `GlobalEndpointManager` / `LocationCache` retained for the residual SDK-pipeline call sites | ✅ `account_metadata_cache::AccountProperties::write_region()` (`driver/cache/account_metadata_cache.rs:175`); driver tracks per-account regions via `AccountMetadataCache` | Driver-owned for customer ops |
| SDK consumes driver               | ✅ Yes — `Cargo.toml:20`, `clients/mod.rs:33` | —                                    | Wired               |

> **Removed in the previews refactor:** `CosmosHeadersPolicy`, `AuthorizationPolicy`, and
> `TrackedTransportPolicy` no longer exist as `Policy` types. The `Vec<Arc<dyn Policy>>`
> pipeline-chain abstraction is gone for the data-plane path; header application and request
> signing are now direct function calls inside `execute_operation`. Two replacement-comment
> citations: `driver/transport/cosmos_headers.rs:6` (*"This replaces `CosmosHeadersPolicy`
> from the old policy-chain pipeline."*) and `driver/transport/request_signing.rs:6` (*"This
> replaces `AuthorizationPolicy` from the old policy-chain pipeline."*).

**Implication for this work item.** All three prerequisites that previously gated the
implementation in the SDK (status-code retry hook, single-master detection, per-operation
state) now exist — or have direct equivalents — in the driver:

1. **Status-code retry hook:** `evaluate_transport_result(&op, &endpoint, result, &state)`
   is the canonical decision function and already classifies 404/1002 as `SessionRetry`
   (`driver/pipeline/retry_evaluation.rs:12, 70`). The latch-set point is here.
2. **Account-level single-master detection:** `AccountProperties::write_region() ->
   Option<Region>` exists at `driver/cache/account_metadata_cache.rs:175`; a multi-write
   accessor must be added or surfaced (e.g. counting `write_regions()` or calling an
   equivalent of `can_use_multiple_write_locations`). This is a small additive change in
   the driver's account-metadata cache, not a redesign.
3. **Per-operation latch state:** the `RetryState` already threaded into
   `evaluate_transport_result` (`retry_evaluation.rs:31`) is the natural carrier — extend
   it with a `pub(crate) hub_region_processing_only: bool` field, set in the 404/1002
   classification arm, and read by the pipeline's pre-send stage. No `azure_core::http::Context`
   plumbing is needed.

Day 1 therefore lands in the **driver**. The SDK-resident description in §3 and §4 is
preserved as a parity reference (it captures the .NET behavioral shape and the trigger
boundary `session_token_retry_count == 1`); the equivalent driver landing sites are
enumerated in [ALT-6](#alt-6--implement-the-latch--header-as-a-driver-side-policy).

---

## 2. Architectural Overview

> **Branch note:** the .NET-equivalent retry shape lives in two different places depending
> on branch. On `main` the relevant hooks are on `azure_data_cosmos::ClientRetryPolicy`. On
> `release/azure_data_cosmos-previews` (this branch) customer ops route through
> `CosmosDriver::execute_operation` and the equivalent hooks live in the driver's operation
> pipeline + `evaluate_transport_result`. The SDK-side description below is preserved for
> .NET parity reference; see [§2.1 Driver-resident architecture (previews)](#21-driver-resident-architecture-previews)
> for the actual implementation surface on this branch.

The Rust pipeline is shaped almost identically to .NET. The two relevant hooks already exist on
`ClientRetryPolicy`:

| .NET (`ClientRetryPolicy.cs`)         | Rust (`client_retry_policy.rs`)                      | Role |
|---------------------------------------|------------------------------------------------------|------|
| `OnBeforeSendRequest`                 | `before_send_request` (line ~150)                    | Sole pre-flight mutation hook for outbound headers / endpoint routing. |
| `ShouldRetryInternalAsync` — 404/1002 | `should_retry_on_session_not_available` (line ~267)  | Decides whether to retry a session-not-available failure and how many regions remain. |

The change introduces a single boolean field on `ClientRetryPolicy` — the *latch* — that is set
inside `should_retry_on_session_not_available` and consumed inside `before_send_request`. The
retry handler (`BackOffRetryHandler`) re-enters `before_send_request` for every attempt, which
is the property that lets the latched flag persist onto subsequent retries automatically.

```mermaid
flowchart TD
    subgraph Loop["BackOffRetryHandler::send (loop, per attempt)"]
        BSR["ClientRetryPolicy::before_send_request<br/><i>runs on EVERY attempt</i><br/>if self.add_hub_region_processing_only_header<br/>&nbsp;&nbsp;&nbsp;&nbsp;{ request.headers.insert(...) }"]
        SEND[("HTTP send → response")]
        SR["ClientRetryPolicy::should_retry_on_*<br/>on 404/1002 + single-master + count==1:<br/>&nbsp;&nbsp;&nbsp;&nbsp;self.add_hub_region_..._header = true<br/><i>← latches the flag</i>"]
        BSR --> SEND --> SR
        SR -. "Retry → next iteration" .-> BSR
    end
```

`ClientRetryPolicy` is constructed once per top-level operation by
`retry_handler::retry_policy_for_request`. The latch's lifetime is therefore naturally bounded
to a single user-visible operation — see [§5 Side Effects & Risk](#5-side-effects--risk) for the
verification step required in implementation.

### Forward target — driver-resident equivalent

> Section heading retained for diff stability. **On `release/azure_data_cosmos-previews`
> this *is* the present-day target, not a forward target.** The "forward" framing applied
> to `main`; on previews the driver already owns the data-plane retry/header surface.

#### 2.1 Driver-resident architecture (previews)

The driver's data-plane shape on previews is:

`CosmosDriver::execute_operation` (`driver/cosmos_driver.rs:958`) → operation pipeline
(`driver/pipeline/operation_pipeline.rs`) → per-attempt transport request build
(`build_transport_request`, `:442`) and pre-send mutation → HTTP send →
`evaluate_transport_result` (`driver/pipeline/retry_evaluation.rs:31`) → either
`Action::Return(...)` or a retry that re-enters the pipeline. Header mutation and request
signing are direct calls — `cosmos_headers::apply_cosmos_headers(&mut request, user_agent)`
at `driver/cosmos_driver.rs:301` and the `request_signing` module — there is no
`Vec<Arc<dyn Policy>>` chain on this path.

The hub-region-processing-only feature maps onto these stages as:

| Role | Site on previews |
|------|------------------|
| Per-attempt outbound mutation (emission) | The pre-send / `build_transport_request` stage of the operation pipeline (`driver/pipeline/operation_pipeline.rs:442`). When the per-operation latch is set, insert `request_header_names::HUB_REGION_PROCESSING_ONLY: "True"` here. |
| Status-code retry decision (trigger) | `evaluate_transport_result` (`driver/pipeline/retry_evaluation.rs:31`). The 404/1002 arm at `:70` already classifies `is_read_session_not_available()` as `SessionRetry`; the latch is set here, gated on single-master + first-1002. |
| Per-operation latch state | A new `pub(crate) hub_region_processing_only: bool` field on `RetryState` (the per-operation state already passed to `evaluate_transport_result`). |
| Single-master gate | `AccountProperties::write_region()` (`driver/cache/account_metadata_cache.rs:175`) plus a small additive accessor (`fn account_supports_multi_write(&self) -> bool`) sourced from `write_account_region` / region-count semantics already present in `AccountMetadataCache`. |
| Header constant | `request_header_names::HUB_REGION_PROCESSING_ONLY` in `driver/src/models/cosmos_headers.rs:17` (joins the existing `request_header_names` module: `PREFER`, `IS_UPSERT`, `BATCH_*`, `SESSION_TOKEN`, `PRIORITY_LEVEL`, `THROUGHPUT_BUCKET`, etc.). |

```mermaid
flowchart TD
    EO["CosmosDriver::execute_operation<br/>(driver/cosmos_driver.rs:958)"]
    subgraph Loop["Operation pipeline (per-attempt loop)"]
        PRE["Pre-send: build_transport_request<br/>operation_pipeline.rs:442<br/><i>runs on EVERY attempt</i><br/>if retry_state.hub_region_processing_only<br/>&nbsp;&nbsp;&nbsp;&nbsp;{ headers.insert(HUB_REGION_PROCESSING_ONLY, \"True\"); }"]
        SEND[("HTTP send → response")]
        EVAL["evaluate_transport_result<br/>retry_evaluation.rs:31<br/>404/1002 + single-master + first-1002:<br/>&nbsp;&nbsp;&nbsp;&nbsp;retry_state.hub_region_processing_only = true<br/><i>← latches the flag</i>"]
        PRE --> SEND --> EVAL
        EVAL -. "SessionRetry → next attempt" .-> PRE
    end
    EO --> PRE
```

The .NET concept-mapping table from §2 still applies; the columns shift right one crate:

| .NET (`ClientRetryPolicy.cs`)         | Driver (previews)                                                       | Role |
|---------------------------------------|-------------------------------------------------------------------------|------|
| `OnBeforeSendRequest`                 | `build_transport_request` pre-send stage in `operation_pipeline.rs:442` | Per-attempt outbound mutation. |
| `ShouldRetryInternalAsync` — 404/1002 | `evaluate_transport_result` (`retry_evaluation.rs:31`); 404/1002 arm at `:70` | Decide whether to retry; latch is set here. |
| `addHubRegionProcessingOnlyHeader` field | `RetryState::hub_region_processing_only` (new field)                 | Per-operation boolean latch; same volatile-bool semantics as .NET. |

#### 2.2 SDK-resident description (parity reference, `main`)

The remainder of this section captures the SDK-resident shape that applied to `main`.
Preserved for .NET parity readers and for the residual SDK-pipeline call sites
(`container_connection.rs:155`, `partition_key_range_cache.rs:304`); not the implementation
target on previews.

---

## 3. Behavior Specification

### 3.1 Trigger

The latch flips to `true` when **all** of the following hold inside a single
`ClientRetryPolicy` instance:

1. `should_retry_on_session_not_available` is invoked (i.e., the response was `404` with
   sub-status `1002 / READ_SESSION_NOT_AVAILABLE`).
2. `enable_endpoint_discovery` is `true`. (When discovery is disabled, the policy returns
   `DoNotRetry` and the latch is not touched.)
3. The **account** is single-master. Source the gate from
   `LocationCache::can_use_multiple_write_locations()` via the `GlobalEndpointManager`'s
   location cache — *not* from `ClientRetryPolicy.can_use_multiple_write_locations` and *not*
   from `GlobalEndpointManager::can_use_multiple_write_locations(request)`. Both of the
   latter are computed per request and return `false` for **all** read operations regardless
   of account topology, so they cannot serve as a single-master gate.
4. `self.session_token_retry_count == 1` after the increment at the top of
   `should_retry_on_session_not_available` — i.e., the call that is about to return `Retry`
   for the *first* permitted retry. Latching here, rather than at .NET's "second-or-later"
   point, is required because the existing single-master budget returns `DoNotRetry` once
   `session_token_retry_count > 1`; if we waited for the second 1002 the latch would flip on
   the same call that ends the operation and the header would never reach the wire.
   Consequence: this is a **deliberate parity gap** with .NET, which would have allowed the
   first retry to go without the header. It is the cost of staying inside Rust's current
   retry budget; full retry-count parity is captured in
   [ALT-5](#alt-5--extend-rusts-read-retry-budget-to-match-nets-readendpointscount-bound) and
   [Future Work](#future-work).

Once latched, the flag is **never reset** within the policy. .NET uses a single `volatile bool`
field with the same semantics. In Rust, plain `bool` is correct: `ClientRetryPolicy` is mutated
through `&mut self` for the duration of an operation (no concurrent access), so no atomicity
is required. This assumption must be preserved by future refactors and is recorded as part of
the SE-003 acceptance gate in [§7.4](#74-acceptance-gates-implementation-pr).

### 3.2 Outbound effect

On every subsequent attempt within the same operation, `before_send_request` checks the latch.
When set, it adds:

```text
x-ms-cosmos-hub-region-processing-only: True
```

to the outgoing `CosmosRequest`. The header value is the literal `"True"` (capitalized) — see
[§4.2(b)](#42-sdkcosmosazure_data_cosmossrcretry_policiesclient_retry_policyrs) for the wire-format rationale.
The header rides along on retries triggered by **any** status code that does still fall within
the policy's budget — connection failures (which are tracked under a separate budget,
`MAX_RETRY_COUNT_ON_CONNECTION_FAILURE`, rather than the `session_token_retry_count` budget),
endpoint-failure failovers, and any future status codes that produce additional attempts —
until the operation terminates.

### 3.3 What stays unchanged

- Retry budgets, delays, and status-code routing in `should_retry_on_*`.
- Client-side region selection in `RetryContext`. The header is a *backend* routing hint;
  Rust continues to drive its own region rotation through the existing failover ladder.
- Session-token propagation, partition-key handling, consistency-level handling.
- The existing inline `request.headers.insert(constants::ALLOW_TENTATIVE_WRITES, "true")` on
  `before_send_request` — that line stays exactly where it is and is the structural precedent
  the new header follows. The new header's *value* uses `"True"` to match .NET wire format
  (see [§4.2(b)](#42-sdkcosmosazure_data_cosmossrcretry_policiesclient_retry_policyrs)); the
  divergence from `ALLOW_TENTATIVE_WRITES`'s `"true"` is intentional.

### 3.4 Worked example

Single-master account, Session consistency, preferred regions `[East US 2, Central US]`,
write region `East US 2`.

| Attempt | Targeted region | Server response | Header on **this** attempt | Notes |
|---|---|---|---|---|
| 1 | East US 2 (write region, per `retry_request_on_preferred_locations: false`) | `404` / `1002` | _absent_ | First 404/1002. Counter goes 0→1. The call latches `add_hub_region_processing_only_header = true` *and* returns `Retry`. The retry it returns will carry the header. |
| 2 | East US 2 (same write region; client-side region rotation is unaffected by the header) | `200` ✓ | **`True`** | `before_send_request` reads the latch and emits the header. Backend honors hub-region-only processing. Operation completes. |

If attempt 2 were to fail with another `404 / 1002`, the existing single-master budget returns
`DoNotRetry` (`session_token_retry_count > 1`) and the operation surfaces the error to the
caller. This is the deliberate parity gap noted in [§3.1](#31-trigger): the header reached the
wire on the one retry the current budget allows, but Rust does not retry across additional
read endpoints the way .NET does. Extending the budget would close the gap; that is captured
in [ALT-5](#alt-5--extend-rusts-read-retry-budget-to-match-nets-readendpointscount-bound) and
deferred to Future Work.

#### 3.4.1 Worked example — partition hub-region migration (PPAD interaction, post-ALT-5)

This second example exercises the interaction Fabian flagged on the spec PR: the header is the
backend's signal that the **partition's** hub region (not the account's) should be discovered
on the retry. When a partition has been migrated to a new hub (PPAD — Per-Partition
Automatic-failover / Affinity-Driven routing), the original write region returns `403 / 3`
("write forbidden — wrong region for this partition"). The latch — already set by the prior
404/1002 — keeps emitting the header on the subsequent retries so the backend can route the
attempt to whichever satellite region now owns the partition's writes.

Single-master account, Session consistency, account write region `Region A`, preferred regions
`[Region B, Region A, Region C]`, partition has been migrated so its **partition-scoped** hub
is now `Region C`.

| Attempt | Targeted region | Server response | Header on **this** attempt | Notes |
|---|---|---|---|---|
| 1 | Region B (preferred read) | `404 / 1002` | _absent_ | First 404/1002. Counter 0→1. Latch flips ON; policy returns `Retry`. |
| 2 | Region A (account hub, per `retry_request_on_preferred_locations: false`) | `403 / 3` | **`True`** | Backend rejects: this partition no longer hubs in Region A. Header was on the wire so the backend knows the client wants partition-hub routing. |
| 3 | Region B (next preferred per failover ladder) | `403 / 3` | **`True`** | Latch persists. Header keeps flowing so the partition router can redirect rather than serve stale routing. |
| 4 | Region C (partition's new hub, discovered via failover ladder) | `200` ✓ | **`True`** | Backend serves from the new partition hub. Operation completes. |

**Budget caveat — this scenario is gated on [ALT-5](#alt-5--extend-rusts-read-retry-budget-to-match-nets-readendpointscount-bound).**
Under the *current* Rust single-master read budget, attempt 3 does not happen: the second
`403/3` (or any second non-success after the 404/1002) exits with `DoNotRetry`. The trace
above is the **post-ALT-5** behavior the latch-based design is forward-compatible with —
[§3.1](#31-trigger) calls out that the latch sets once and rides every retry the budget
permits, so widening the budget under ALT-5 automatically extends header coverage to attempts
3 and 4 with no change to the trigger or emission code. Until ALT-5 lands, the parity gap
described after [§3.4](#34-worked-example) applies here as well: the header reaches the wire
on attempt 2 (the one retry today's budget allows), but Rust will not continue rotating
through Region B / Region C to discover the partition's new hub.

**PPAD scope note.** Per-partition hub-region discovery (which partition routes to which
region after a migration) is a backend concern; the header is the *only* client-side
contribution. Rust's client-side `LocationCache` / `RetryContext` does not model
per-partition affinity today — it rotates account-level preferred regions. The header
delegates the partition-scoped routing decision to the backend, so no client-side PPAD
modeling is required for this work item. Future Work for explicit per-partition routing on
the client is out of scope here.

---

## 4. Code Changes

> **Branch note.** §4 below describes the SDK-resident landing on `main`. **On
> `release/azure_data_cosmos-previews` (this branch) implementers must not edit
> `azure_data_cosmos::ClientRetryPolicy` for this feature** — that path is not reached
> by customer reads/writes on previews. The driver-resident landing — header constant in
> `driver/src/models/cosmos_headers.rs` (`request_header_names` module), trigger in
> `driver/src/driver/pipeline/retry_evaluation.rs` (`evaluate_transport_result`, 404/1002
> arm), emission in `driver/src/driver/pipeline/operation_pipeline.rs`
> (`build_transport_request`), latch on `RetryState`, single-master gate via
> `AccountMetadataCache` — is enumerated in [§2.1](#21-driver-resident-architecture-previews)
> and [ALT-6](#alt-6--implement-the-latch--header-as-a-driver-side-policy). §4 is
> retained as a parity reference for the .NET behavioral shape (trigger boundary,
> latch-once semantics, header value, gate semantics) which the driver implementation
> must replicate cell-for-cell.

> **Crate boundary note (parity reference).** All file paths in §4.1–§4.4 are under
> `sdk/cosmos/azure_data_cosmos/` (the SDK). They map onto the previews-branch
> implementation sites enumerated in §2.1.

### 4.1 `sdk/cosmos/azure_data_cosmos/src/constants.rs`

Add **one** entry to the existing `cosmos_headers!` macro invocation, adjacent to
`ALLOW_TENTATIVE_WRITES`:

```rust
cosmos_headers! {
    // ... existing entries ...
    ALLOW_TENTATIVE_WRITES => "x-ms-cosmos-allow-tentative-writes",
    SHOULD_PROCESS_ONLY_IN_HUB_REGION => "x-ms-cosmos-hub-region-processing-only",
    // ... existing entries ...
}
```

The macro auto-defines the constant via `HeaderName::from_static` *and* auto-registers it in
`COSMOS_ALLOWED_HEADERS` (used by the default logging policy). Do **not** define the constant
outside the macro and do **not** hand-edit `COSMOS_ALLOWED_HEADERS`.

### 4.2 `sdk/cosmos/azure_data_cosmos/src/retry_policies/client_retry_policy.rs`

**(a) Add the latch field.** Insert into the `ClientRetryPolicy` struct alongside the other
retry-state counters:

```rust
/// Latched flag indicating that subsequent attempts in this operation must carry
/// the `x-ms-cosmos-hub-region-processing-only` header. Set in
/// `should_retry_on_session_not_available` for single-master accounts on the
/// first 404/1002 within the operation (`session_token_retry_count == 1` after
/// the increment); never reset within the policy. Mirrors the
/// `addHubRegionProcessingOnlyHeader` field added in .NET PR #5447. Plain `bool`
/// is correct because `ClientRetryPolicy` is mutated through `&mut self` for
/// the duration of an operation; no atomicity is required.
add_hub_region_processing_only_header: bool,
```

Initialize to `false` in `ClientRetryPolicy::new`.

**(b) Apply the header.** In `before_send_request`, after the existing routing logic and
**immediately adjacent** to the existing `ALLOW_TENTATIVE_WRITES` line (currently around
line 168), add:

```rust
if self.add_hub_region_processing_only_header {
    request
        .headers
        .insert(constants::SHOULD_PROCESS_ONLY_IN_HUB_REGION, "True");
}
```

The literal must be `"True"` (capitalized first letter) for parity with .NET's
`bool.TrueString`. **The backend's case-sensitivity for this header value is
explicitly not verified against a backend contract reference** (see opening
**Confidence** block); we follow the .NET wire value verbatim because that is the value
the .NET client emits and was implicitly accepted by the backend during the .NET PR
#5447 rollout. The adjacent `ALLOW_TENTATIVE_WRITES` line uses lowercase `"true"`, but
that is a different header with its own (independent) backend parser; we cannot assume
the new header's parser is case-insensitive without a contract reference. Do **not**
use `"true"` (lowercase) or `"True".to_string()`.

**(c) Set the latch.** In `should_retry_on_session_not_available`, after the
discovery-enabled early return (`if !self.enable_endpoint_discovery { return DoNotRetry }`)
and the counter increment around line 268, but **before** the budget/gating
conditionals that decide `Retry` vs `DoNotRetry`:

```rust
if !self.add_hub_region_processing_only_header
    && self.session_token_retry_count == 1
    && !self.global_endpoint_manager.account_supports_multi_write()
{
    self.add_hub_region_processing_only_header = true;
    debug!(
        "latched x-ms-cosmos-hub-region-processing-only for single-master \
         session retry"
    );
}
```

Notes on the gate:

- The single-master check goes through a new forwarding accessor on
  `GlobalEndpointManager`, **not** a borrow of the `LocationCache` itself: the
  `location_cache` field is `Mutex<LocationCache>` (`global_endpoint_manager.rs:31`),
  so any accessor that returned `&LocationCache` would not be lifetime-compatible
  with method chaining behind the lock guard. Add this method to
  `GlobalEndpointManager`:

  ```rust
  pub(crate) fn account_supports_multi_write(&self) -> bool {
      self.location_cache
          .lock()
          .unwrap()
          .can_use_multiple_write_locations()
      // (.unwrap() is consistent with existing callers at lines 350 and 430
      // that .lock().unwrap() the same mutex on previews; the corresponding
      // lines on `main` are 283 and 354.)
  }
  ```

  This wraps `LocationCache::can_use_multiple_write_locations()`
  (`location_cache.rs:381`), which is account-level (`write_endpoints().len() > 1`).
  Do **not** use `self.can_use_multiple_write_locations` on the policy — that field
  is populated from `GlobalEndpointManager::can_use_multiple_write_locations(request)`
  (`:303` on previews; `:242` on `main`) which returns `false` for all read operations
  regardless of account topology, and would cause the latch to fire for every read on a
  multi-master account too. The `Mutex<LocationCache>` field referenced above is at
  `global_endpoint_manager.rs:39` on previews (`:31` on `main`).
- The trigger uses `== 1` (exact equality), matching the boundary that pins this
  latch to Rust's existing single-master read-retry budget. See
  [§3.1](#31-trigger) for the parity-gap rationale.
- The condition uses `!self.global_endpoint_manager.account_supports_multi_write()`
  (logical negation) rather than `... == false`, which clippy's `bool_comparison`
  lint flags. §7.3 enforces `cargo clippy -- -D warnings`, so the example must
  match the lint policy.

Also, at the increment site itself (`self.session_token_retry_count += 1;`, line 268), add a
short inline comment cross-referencing this spec so a future refactor that moves the
increment doesn't silently flip the trigger boundary:

```rust
// HUB_REGION_PROCESSING_HEADER_SPEC.md §3.1: the latch trigger pins to
// session_token_retry_count == 1 *after* this increment. Moving the
// increment changes the trigger boundary.
self.session_token_retry_count += 1;
```

Extend the existing `use tracing::error;` import to `use tracing::{debug, error};`.

**(d) Doc-comment hygiene (opportunistic).** The surrounding doc comments on
`should_retry_on_session_not_available` and `should_retry_on_http_status` cite "404.1022" for
session-not-available; the actual sub-status is `1002` (`READ_SESSION_NOT_AVAILABLE`).
`PARTITION_KEY_RANGE_GONE` is sub-status `1002` of status `410`, not `404`. Correct in the
same edit.

### 4.3 `sdk/cosmos/azure_data_cosmos/CHANGELOG.md`

A `## 0.34.0 (Unreleased)` block already exists at the top of this branch with
`### Features Added`, `### Breaking Changes`, `### Bugs Fixed`, and `### Other Changes`
subsections (all currently empty). **Do not** create a new version block. Add the new
entry under the existing `### Features Added` subsection of `0.34.0 (Unreleased)`,
preserving the existing subsection ordering:

```markdown
## 0.34.0 (Unreleased)

### Features Added

- Added `x-ms-cosmos-hub-region-processing-only` request header on retries after
  `404`/`1002` for single-master accounts, opting the retry into hub-region processing
  during failover/failback windows. ([#NNNN](https://github.com/Azure/azure-sdk-for-rust/pull/NNNN))
```

The `(#NNNN)` suffix is mandatory; replace with the implementation PR number once opened.
The implementer should re-verify the block is still `(Unreleased)` immediately before opening
the PR; if it has been cut to a release in the interim, create a new
`## 0.35.0 (Unreleased)` block above it. Mirror the same entry into
`sdk/cosmos/azure_data_cosmos_driver/CHANGELOG.md` if the implementation lands in the
driver per [ALT-6](#alt-6--implement-the-latch--header-as-a-driver-side-policy).

### 4.4 No changes required

- No `Cargo.toml` change.
- No public API, options, or builder change.
- No change to `request_options.additional_headers`, `COSMOS_ALLOWED_HEADERS` (the macro
  handles it), or `cosmos_pipeline.rs`.
- No integration-test fixture change.

---

## 5. Side Effects & Risk

| ID     | Severity        | Description | Mitigation |
|--------|-----------------|-------------|------------|
| SE-001 | 🟡 Potential    | Backend rollout asymmetry — older regions ignore the header; behavior reverts to today's baseline (no recovery during failback). | Forward-compatible by design. Document graceful degradation in CHANGELOG. No code mitigation required. |
| SE-002 | 🟡 Potential    | Future backend mis-routing if header were honored on multi-master accounts. | Latch is gated by the **account-level** check `LocationCache::can_use_multiple_write_locations()`; the per-request fields are explicitly avoided. Asserted in unit test ([§7.1](#71-unit-tests) AC-4). |
| SE-003 | 🔴 Breaking — accepted via AG-1..AG-4 | Latch correctness depends on `ClientRetryPolicy` being constructed fresh per top-level operation. If the policy is ever pooled or reused across operations, the latch leaks and the header attaches to unrelated requests. | **Promoted to a hard acceptance gate** for the implementation PR — see [§7.4](#74-acceptance-gates-implementation-pr). The plain `bool` field choice ([§3.1](#31-trigger)) further depends on `&mut self`-only access being preserved by future refactors. |
| SE-004 | 🟢 Minor        | Spec deviation: the issue text describes a 403/3 "skip-set rotation" flow. The .NET PR did not implement it; we mirror .NET. | Documented in [§6](#6-alternatives-considered) (ALT-4) and [§8](#8-open-questions--future-work) (OQ-1). Existing `should_retry_on_endpoint_failure` already rotates on 403/3 and the latched header rides along. |
| SE-005 | 🟢 Minor        | Pre-existing inline doc-comment typo `404.1022 → 404.1002 (READ_SESSION_NOT_AVAILABLE)`. | Fixed opportunistically in §4.2(d). |
| SE-006 | 🟢 Resolved on previews | **Crate-boundary drift (resolved).** On `main` this work would land in `azure_data_cosmos`, with a deferred relocation to the driver. On `release/azure_data_cosmos-previews` (this branch) the data-plane refactor has already moved customer dispatch into the driver, so the implementation lands directly in `azure_data_cosmos_driver` per [§2.1](#21-driver-resident-architecture-previews) and [ALT-6](#alt-6--implement-the-latch--header-as-a-driver-side-policy). No future migration is needed. The cross-walk table in ALT-6 enumerates the driver-resident sites; AG-5's migration anchor is therefore not required on this branch (see [§7.4](#74-acceptance-gates-implementation-pr)). | Wire output is identical regardless of which crate emits the header; only the implementation site moves. |

### Cross-cutting concerns touched

- **Retry logic & policies** — direct change site; additive only.
- **Telemetry / tracing** — one new `debug!` event when the latch flips. Low cost, high
  diagnostic value during failover events.
- **Wire format** — one additive forward-compatible outbound header. No request body or
  response handling change.
- **Consistency level handling** — 404/1002 is a Session-consistency symptom; backend
  honors the header for consistency `< Strong`. We trigger purely on response code, matching
  .NET — no client-side consistency-level branching.
- **Cross-region failover & preferred locations** — touched **by association**. The header
  is a backend routing hint, not a client-side region change. Client-side region selection in
  `ClientRetryPolicy` is unchanged.

### Cross-cutting concerns NOT touched

Authentication / token refresh, connection pooling, session token propagation, partition key
handling, configuration / initialization, async execution / cancellation,
serialization / response parsing.

---

## 6. Alternatives Considered

### ALT-1 — Reuse the existing `RetryContext.route_to_hub` field

**Verdict:** Rejected.
The existing `route_to_hub` field on `RetryContext` controls *client-side* routing (via
`request_context.route_to_location_endpoint(hub_uri)`), not header emission. Conflating the
two would force every latched retry to be sent directly to the local hub URI — which is **not**
what .NET does. .NET keeps the existing client-side failover ladder and lets the *backend*
re-route via the header. Adding a separate boolean keeps the two concerns orthogonal and
preserves .NET parity.

### ALT-2 — Set the header directly in `should_retry_on_session_not_available`, no latch

**Verdict:** Rejected.
A latch is still preferable even though, under Rust's current single-master budget, only one
post-1002 retry exists today. The latch keeps the wire-emission decision in
`before_send_request` (the canonical outbound-mutation hook), separates "trigger" from
"emission" so future budget extensions ([ALT-5](#alt-5--extend-rusts-read-retry-budget-to-match-nets-readendpointscount-bound))
slot in without restructuring, and matches .NET's shape (`addHubRegionProcessingOnlyHeader`
field) at zero extra cost.

### ALT-3 — Plumb the latch through `RequestContext` (per-request) instead of `ClientRetryPolicy`

**Verdict:** Rejected.
`RequestContext` is reset per attempt; the latch state would have to be re-derived on every
retry, requiring the policy to inspect history each time. State on the policy mirrors .NET
exactly and matches Rust's lifetime model — `ClientRetryPolicy` is per-operation, giving the
latch a natural and bounded scope at no extra plumbing cost.

### ALT-4 — Implement the full issue-text spec, including the 403/3 NotWriteRegion skip-set rotation

**Verdict:** Rejected for this work item.
The user explicitly directed mirroring .NET PR #5447, which did **not** implement the
skip-set rotation. The existing `should_retry_on_endpoint_failure` already rotates on 403/3,
and the latched header rides along on the rotated retry — producing observably equivalent
behavior to the more elaborate flow described in the issue. If a future work item proves a
dedicated rotation is required, file a follow-up. See OQ-1 in [§8](#8-open-questions--future-work).

### ALT-5 — Extend Rust's read-retry budget to match .NET's `ReadEndpoints.Count` bound

**Verdict:** Rejected for this work item; captured as [Future Work](#future-work).
True observable parity with .NET PR #5447 — header rides every read endpoint until the budget
is exhausted, including the latch-on-second-1002 trigger point .NET uses — requires changing
the `else if self.session_token_retry_count > 1 { return DoNotRetry; }` branch in
`should_retry_on_session_not_available` (around line 294 of `client_retry_policy.rs`) to bound
on `global_endpoint_manager.read_endpoints().len()` and to retry across preferred read regions
rather than via write locations. That is a semantic change to the single-master read retry
contract that affects every read in the SDK, not just 404/1002. Bundling it with header
injection couples two unrelated risks. Day 1 ships header-emission parity within the existing
budget; if post-rollout telemetry shows the single retry is insufficient, file a follow-up
that adopts the .NET budget and at the same time moves this latch's trigger to
`session_token_retry_count >= 2` for full parity.

### ALT-6 — Implement the latch + header as a driver-resident change *(promoted: primary path on previews)*

**Verdict on previews:** **Adopted as the primary implementation path.** ALT-6 was
"forward target / future work" against `main`. On `release/azure_data_cosmos-previews`
it is the only path that affects customer reads, because customer ops bypass the SDK
retry layer (see [§1.5](#15-two-crate-landscape-sdk-vs-driver-previews-architecture)
and [§2.1](#21-driver-resident-architecture-previews)).

The driver crate (`azure_data_cosmos_driver`) is the layer that owns transport, headers,
status-code retry, and account-metadata caching on previews. The natural shape on
previews is **not** a new `Policy` (the `Vec<Arc<dyn Policy>>` chain has been removed
from the data-plane path) but a direct extension of:

1. **`evaluate_transport_result`** (`driver/pipeline/retry_evaluation.rs:31`) — set the
   latch in the existing 404/1002 arm at `:70` (`status.is_read_session_not_available()
   && retry_state.can_retry_session()`), gated on single-master + first-1002.
2. **`build_transport_request`** (`driver/pipeline/operation_pipeline.rs:442`) — read
   the latch from the per-operation `RetryState` and emit the header on every attempt
   when set (joins existing per-attempt header insertions like `request_header_names::
   PREFER`, `IS_UPSERT`, `SESSION_TOKEN`, `PRIORITY_LEVEL`, `THROUGHPUT_BUCKET`).

**Prerequisite status on previews** — what was previously enumerated as blocking ALT-6
on `main` has been substantially resolved by the data-plane refactor:

1. **Status-code retry in the driver — ✅ exists.** The driver's
   `evaluate_transport_result` (`retry_evaluation.rs:31`) is a pure function that
   classifies transport results and already handles 404/1002:
   `retry_evaluation.rs:12` (module-level doc comment): *"404/1002
   ReadSessionNotAvailable → SessionRetry (advances region)"*; `retry_evaluation.rs:70`:
   `if status.is_read_session_not_available() && retry_state.can_retry_session()`. The
   trigger arm exists; this work item adds latch-set logic next to it.
2. **Account-metadata fetch and topology tracking — ✅ partially exists.** The driver
   has an `AccountMetadataCache` (`driver/cache/account_metadata_cache.rs`) with a
   `pub(crate) fn write_region(&self) -> Option<Region>` accessor at
   `account_metadata_cache.rs:175`, and `AccountProperties::write_account_region()` is
   used in `cosmos_driver.rs:1009`. A multi-write accessor (e.g. `fn
   account_supports_multi_write(&self) -> bool`, sourced from a region-count check)
   needs to be added or surfaced on `AccountProperties` / `AccountMetadataCache`. This
   is a small additive change, not a redesign — the metadata is already cached.
3. **Per-operation state model — ✅ exists.** `RetryState` is already threaded into
   `evaluate_transport_result` (`retry_evaluation.rs:31`), which is the natural carrier
   for the latch. Extend it with `pub(crate) hub_region_processing_only: bool`. No
   `azure_core::http::Context` plumbing or per-operation wrapper struct is needed.

**Cross-walk: SDK parity reference (§4 above) → driver implementation (previews).**
All cells are now mechanical or small-additive; none remain "design-pending":

| SDK parity reference (§4)                                                | Driver implementation on previews                                                                                  | Status |
|--------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------|--------|
| `ClientRetryPolicy::before_send_request` emits the header                | `build_transport_request` (`operation_pipeline.rs:442`) reads the `RetryState` latch and inserts the header        | Mechanical |
| Latch field `add_hub_region_processing_only_header: bool` on `&mut self` | New field `hub_region_processing_only: bool` on `RetryState` (already passed to `evaluate_transport_result`)        | Mechanical |
| Trigger in `should_retry_on_session_not_available` (single-master + first-1002) | Trigger in the existing 404/1002 arm of `evaluate_transport_result` (`retry_evaluation.rs:70`); same gate semantics | Mechanical |
| Single-master gate via `GlobalEndpointManager::account_supports_multi_write()` | Single-master gate via a new accessor on `AccountMetadataCache` / `AccountProperties` (sourced from `write_region()` already at `:175`) | Small additive |
| Constant `SHOULD_PROCESS_ONLY_IN_HUB_REGION` in `azure_data_cosmos::constants` | Constant `HUB_REGION_PROCESSING_ONLY` in `request_header_names` (`driver/src/models/cosmos_headers.rs:17`)         | Mechanical |

The wire output is identical to the SDK-resident description. The implementation PR
should still leave a `// HUB_REGION_PROCESSING_HEADER_SPEC.md §3.1` cross-reference at
the trigger-set site so a future refactor of `evaluate_transport_result` doesn't
silently flip the trigger boundary.

---

## 7. Testing Strategy

### 7.1 Unit Tests

Add new `#[tokio::test] async fn` entries inside the existing
`#[cfg(test)] mod tests` block in `client_retry_policy.rs`, immediately following
`test_should_retry_session_not_available_single_write` (~line 1037). Use the local
`test_*` naming prefix to match the file's existing convention (the global copilot
guidance to omit `test_` does **not** apply here — every test in this file uses the prefix).

| ID    | Name (suggested)                                                | Asserts |
|-------|-----------------------------------------------------------------|---------|
| AC-1  | `test_session_not_available_no_header_before_first_retry`       | Single-master, fresh policy, before any 404/1002 → `before_send_request` does NOT emit the header. |
| AC-2  | `test_session_not_available_emits_header_on_first_retry`        | Single-master, drive one 404/1002 through `should_retry_on_session_not_available` → next `before_send_request` DOES emit `x-ms-cosmos-hub-region-processing-only: True` (capitalized, exact wire value). |
| AC-3  | `test_session_not_available_latch_pins_to_counter_eq_1`         | Boundary regression. Pre-set `session_token_retry_count = 0` and call into the latch path → latch flips ON and `before_send_request` emits the header. Pre-set `session_token_retry_count = 1` (so the increment at line 268 takes it to 2) → policy returns `DoNotRetry` and the latch does NOT flip. Protects the trigger from silent breakage if the increment is ever moved. |
| AC-4  | `test_session_not_available_no_header_for_multi_master`         | Multi-master account (mock `LocationCache::can_use_multiple_write_locations()` to return `true`), any number of 404/1002 invocations → header NEVER emitted. Pin specifically to the **account-level** gate, not to `ClientRetryPolicy.can_use_multiple_write_locations`. |
| AC-5  | `test_session_not_available_no_header_when_discovery_disabled`  | `enable_endpoint_discovery = false` → policy returns `DoNotRetry`, latch never set. |
| AC-6  | `test_hub_region_header_persists_across_connection_failure`     | After AC-2 latches the flag, drive a connection failure (the SDK's term for a transport-layer failure, tracked under `MAX_RETRY_COUNT_ON_CONNECTION_FAILURE`, separate from the `session_token_retry_count` budget) that takes the same-endpoint retry path → `before_send_request` STILL emits the header. Confirms the latch survives intervening attempts that exist under the current budget. |
| AC-7  | `test_hub_region_header_persists_across_403_3_partition_migration` | **Gated on [ALT-5](#alt-5--extend-rusts-read-retry-budget-to-match-nets-readendpointscount-bound) — add only after the read budget is widened.** Models the [§3.4.1](#341-worked-example--partition-hub-region-migration-ppad-interaction-post-alt-5) trace: drive 404/1002 (latch ON), then drive 403/3 through `should_retry_on_*` for the second attempt → next `before_send_request` STILL emits the header; drive a second 403/3 → header still emitted. Asserts the latch is sticky across partition-hub migration responses, not just session-not-available retries. Until ALT-5 lands, this test belongs in the same change that extends the budget. |

The earlier AC-3 / AC-6 / AC-7 tests covering "post-latch retry on 503" and "post-latch retry
on 410/1022" have been intentionally **dropped**: under Rust's current single-master read
budget, a second 404/1002 returns `DoNotRetry` and there is no further attempt for the header
to ride on. Re-adding those tests is part of the work captured in
[ALT-5](#alt-5--extend-rusts-read-retry-budget-to-match-nets-readendpointscount-bound) /
[Future Work](#future-work) (extending the budget).

Test mechanics follow the existing pattern: build a `ClientRetryPolicy` with a mocked
`GlobalEndpointManager`, call `policy.should_retry(...)` with a synthetic 404/1002, then call
`policy.before_send_request(&mut request)` and assert
`request.headers.get(constants::SHOULD_PROCESS_ONLY_IN_HUB_REGION)` is `Some("True")` or `None`
as appropriate. Note the value is `"True"` (capitalized), not `"true"` — see
[§4.2(b)](#42-sdkcosmosazure_data_cosmossrcretry_policiesclient_retry_policyrs).

### 7.2 Integration Tests

No new recorded integration tests are required for this work item. Existing emulator-backed
fault-injection coverage in
`sdk/cosmos/azure_data_cosmos/tests/multi_write_tests/cosmos_multi_write_fault_injection.rs`
exercises 404/1002 paths and is expected to remain green; those tests assert on retry counts /
regions, not on outbound headers, so the additive header does not flip them. Phase 3+ may
optionally add a single recorded scenario asserting the header on a captured retry request.

### 7.3 Validation Gates

Before opening the PR:

```text
cargo fmt -p azure_data_cosmos
cargo clippy -p azure_data_cosmos --all-features -- -D warnings
cargo build -p azure_data_cosmos --all-features
cargo test  -p azure_data_cosmos --all-features
```

All four must pass with no diff (fmt) and no warnings (clippy).

### 7.4 Acceptance gates (implementation PR)

These gates promote SE-003 (latch lifetime) and the threading-model assumption from §3.1 from
"side effects" to **blocking** items for the implementation PR. They are intentionally
out-of-scope for this spec PR but must all hold before code review can be requested:

1. **AG-1: Per-operation construction.** Verify in `BackOffRetryHandler::retry_policy_for_request`
   (or the equivalent constructor of `ClientRetryPolicy` used by the retry handler) that a
   *fresh* policy is constructed per top-level operation, with the source-line anchor cited in
   the PR description. If the policy is reused across operations, this design is invalid as
   written and must add an explicit reset hook at the operation boundary.
2. **AG-2: Cross-operation isolation test.** Add a unit test that constructs two policies
   sequentially, latches the first, and asserts the second policy's
   `add_hub_region_processing_only_header` is `false` on a fresh request.
3. **AG-3: Within-operation persistence test.** Confirm AC-6 (above) covers the assertion that
   the latch survives intervening retries (connection failure, endpoint failure) within the
   same operation.
4. **AG-4: Threading-model invariant.** Spot-check that no path mutates `ClientRetryPolicy`
   from anything other than `&mut self` (no `Arc<Mutex<...>>` wrapping, no shared clones) for
   the duration of an operation. The plain `bool` ([§3.1](#31-trigger)) is correct only under
   this invariant.
5. **AG-5: Driver-migration anchor.** *(Not applicable on `release/azure_data_cosmos-previews`.)*
   On `main`, the implementation lands in `azure_data_cosmos::ClientRetryPolicy` and a
   `// TODO(driver-migration):` anchor would mark the future relocation site. On
   previews, the implementation lands directly in `azure_data_cosmos_driver` per
   [§2.1](#21-driver-resident-architecture-previews) and
   [ALT-6](#alt-6--implement-the-latch--header-as-a-driver-side-policy); no migration
   anchor is required. Implementers should still leave a one-line cross-reference at
   the latch-set site (`evaluate_transport_result`'s 404/1002 arm,
   `retry_evaluation.rs:70`) of the form
   `// HUB_REGION_PROCESSING_HEADER_SPEC.md §3.1 — set latch on first 404/1002 for`
   `single-master accounts.` so a future refactor of `evaluate_transport_result` doesn't
   silently drop the trigger boundary.

---

## 8. Open Questions & Future Work

### OQ-1 — Skip-set rotation parity *(resolved for Day 1 = .NET parity; rotation since implemented in the driver — see [azure-sdk-for-rust#4555](https://github.com/Azure/azure-sdk-for-rust/pull/4555))*

Issue [#4303](https://github.com/Azure/azure-sdk-for-rust/issues/4303) gestures at a broader
region-rotation flow (skip-set rotation on 403/3 NotWriteRegion) that .NET PR
[#5447](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5447) deliberately did not
implement. **Decision (owner-confirmed):** Day 1 ships .NET parity only — the
`x-ms-cosmos-hub-region-processing-only` header and its latch. Skip-set rotation is
explicitly out of scope for this work item. Rationale:

1. The header alone resolves the 404/1002 failback storm in .NET production telemetry; that
   is the validated fix, and parity with that fix is what #4303 is asking for on Day 1.
2. Skip-set rotation is a net-new design that touches `LocationCache` / preferred-region
   state and the `should_retry_on_endpoint_failure` path. Bundling that with header
   injection couples two unrelated risks in a single PR.
3. The latch introduced in §4.2 is the exact hook any future rotation logic would attach to,
   so deferring rotation is reversible at zero cost to this design.

Rotation is captured below under **Future Work** for a possible follow-up if post-rollout
telemetry shows the header alone is insufficient.

### OQ-2 — CHANGELOG version block *(resolved: append under existing `0.34.0 (Unreleased)`)*

Verified on `release/azure_data_cosmos-previews` at spec rebase time: the CHANGELOG
already has an open `## 0.34.0 (Unreleased)` block with `### Features Added`,
`### Breaking Changes`, `### Bugs Fixed`, and `### Other Changes` subsections.
**Resolution:** add the new entry under the existing `### Features Added` subsection,
preserving the existing subsection ordering. Do not introduce a new version block.
Implementer should re-verify the block is still `(Unreleased)` immediately before
opening the PR; if the block has been cut to a release in the interim, create a new
`## 0.35.0 (Unreleased)` block above it.

### Future Work

- **Driver migration *(resolved on previews; retained for `main` parity).***
  On `main` the latch and header emission would land in
  `azure_data_cosmos::ClientRetryPolicy` and later relocate to
  `azure_data_cosmos_driver` once the driver acquired (a) status-code retry on
  `404/1002`, (b) an account-topology surface, and (c) a per-operation state
  mechanism. **On `release/azure_data_cosmos-previews` (this branch) all three
  prerequisites are met or near-met** (see
  [ALT-6 prerequisite status](#alt-6--implement-the-latch--header-as-a-driver-side-policy)),
  so the implementation lands directly in the driver and there is no follow-up
  migration. This bullet is retained only as a forward note for any subsequent merge
  back into `main`: the cross-walk table in ALT-6 documents the SDK→driver mapping
  for that direction.
- **Extend single-master read retry budget for full .NET parity ([ALT-5](#alt-5--extend-rusts-read-retry-budget-to-match-nets-readendpointscount-bound)).**
  Day 1 ships header-emission parity within Rust's existing one-retry budget. .NET's
  observable parity additionally retries across all read endpoints
  (`sessionTokenRetryCount > ReadEndpoints.Count`) and uses a "second-or-later" trigger.
  Reaching that requires changing the `else if session_token_retry_count > 1 { DoNotRetry }`
  branch in `should_retry_on_session_not_available` and rotating preferred read regions
  rather than retrying via write locations, and the latch's trigger would then move from
  `== 1` back to `>= 2`. File a follow-up if post-rollout telemetry shows the single retry
  is insufficient, or if maintainers decide retry-count parity is required up front.
- **Dedicated 403/3 skip-set rotation.** *Implemented in the driver — see [azure-sdk-for-rust#4555](https://github.com/Azure/azure-sdk-for-rust/pull/4555); original deferral note retained below for history.* Day 1 explicitly ships .NET parity only. If
  post-rollout telemetry shows the header alone is insufficient — i.e. failback storms
  persist after the latched hub-targeted retry — file a follow-up to add an explicit
  skip-set in `should_retry_on_endpoint_failure`. The latch added in §4.2 is the intended
  hook point.
- **Diagnostics surface.** Surface "hub-region-only retry" as a first-class field on the
  Cosmos diagnostics object once the broader diagnostics design (see
  [Feed Operations Spec](https://github.com/Azure/azure-sdk-for-rust/pull/4261)) lands.
- **Backend rollout dashboarding.** Coordinate with service partners on a metric for
  header-honored vs. header-ignored response distributions during failover events.
