<!-- cspell:ignore THINCLIENT thinclient Mgmt cutover directconnectivity footgun cooldown ALPN -->
# Gateway 2.0 Design Spec for Rust Driver & SDK

**Status**: Draft / Iterating
**Date**: 2026-04-20
**Authors**: (team)

---

## Table of Contents

1. [Overview](#1-overview)
2. [Motivation](#2-motivation)
3. [Current Rust State](#3-current-rust-state)
4. [Rust Implementation Plan](#4-rust-implementation-plan)
5. [Open Questions](#5-open-questions)

### Related Specs

- [`TRANSPORT_PIPELINE_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/TRANSPORT_PIPELINE_SPEC.md) — sharded HTTP/2 transport, timeout regime, hedging, `(HttpClient, host:port)` shard key. Gateway 2.0 reuses the sharded transport defined there verbatim; this spec does **not** introduce a new timeout or hedging policy.
- [`PARTITION_KEY_RANGE_CACHE_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/PARTITION_KEY_RANGE_CACHE_SPEC.md) — PKRange cache semantics and `EffectivePartitionKey` usage; cited by Phase 2 for EPK computation and by Phase 4 for 410 handling.
- [`PARTITION_LEVEL_FAILOVER_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/PARTITION_LEVEL_FAILOVER_SPEC.md) — per-partition region override semantics; cited by Phase 4 for PLF precedence over Gateway 2.0 routing.

---

## 1. Overview

Gateway 2.0 (formerly "thin client") is a server-side proxy that allows SDK clients to route data-plane operations through a lightweight proxy endpoint instead of directly to backend replicas. It uses RNTBD binary protocol over HTTP/2, with the proxy handling partition routing, replica selection, and load balancing.

**Naming**: Use "Gateway 2.0" consistently in all Rust code, docs, and comments. Avoid "thin client" except when referencing Java/.NET code or existing constants (`THINCLIENT_*`).

---

## 2. Motivation

### Why Gateway 2.0?

Traditional Cosmos DB offers two connection modes:

- **Gateway mode**: Simple HTTP/REST proxy — easy to use, but adds an extra network hop through a shared stateless gateway. No latency SLA guarantees because the gateway is a shared, best-effort proxy.
- **Direct mode**: SDK connects directly to backend replicas via TCP/RNTBD — provides latency SLA guarantees but requires the SDK to manage replica discovery, connection pooling, and partition routing itself. This adds significant complexity to the SDK and requires direct network access to backend nodes.

**Gateway 2.0 bridges this gap.** It is a gateway mode with SLA latency guarantees — combining the operational simplicity of gateway mode (single endpoint, no direct backend connectivity required, firewall-friendly) with the performance characteristics of direct mode (RNTBD binary protocol, server-side partition routing, replica-aware load balancing).

### Key Benefits

- **SLA latency guarantees** — Unlike traditional gateway, Gateway 2.0 provides contractual latency commitments comparable to direct mode
- **Simplified networking** — Clients connect to a single regional proxy endpoint over HTTPS; no need to open firewall rules to individual backend replicas
- **Reduced SDK complexity** — The proxy handles replica discovery, connection management, and partition-level routing; the SDK only needs RNTBD serialization and endpoint selection
- **HTTP/2 multiplexing** — Multiple concurrent operations share a single TCP connection, reducing connection overhead vs. direct mode's per-replica TCP connections
- **Transparent failover** — The proxy handles replica failover within a partition; the SDK handles regional failover across proxy endpoints

### Design Philosophy

Gateway 2.0 moves partition-level routing intelligence from the SDK into the server-side proxy while keeping regional routing in the SDK. This gives the best of both worlds:

**SDK Responsibility:**

- Regional endpoint selection
- RNTBD serialization
- EPK header injection

**Gateway 2.0 Proxy (Server-Side):**

- Partition routing
- Replica selection
- Load balancing

### Connection Mode Comparison

| Aspect | Gateway | Gateway 2.0 | Direct |
| --- | --- | --- | --- |
| Latency SLA | No | **Yes** | Yes |
| Simple Network | Yes | Yes | No |
| Protocol | HTTP/REST | RNTBD/HTTP2 | RNTBD/TCP |
| Replica Mgmt | Gateway/Proxy | Proxy | SDK |
| Partition Route | Gateway/Proxy | Proxy | SDK |
| Regional Route | SDK | SDK | SDK |
| SDK Complexity | Medium | Medium | High |
| Firewall Rules | 1 endpoint | 1 endpoint | N replicas |

---

## 3. Current Rust State

The Rust driver (`azure_data_cosmos_driver`) already has significant gateway 2.0 scaffolding.

### 3.1 Already Implemented — endpoint & transport

- **`CosmosEndpoint`** — `gateway20_url: Option<Url>` field, `regional_with_gateway20()`, `uses_gateway20()`, `selected_url()` methods
- **`TransportMode::Gateway20`** enum variant in pipeline components
- **`RoutingDecision`** — carries `transport_mode` that distinguishes gateway vs gateway 2.0
- **`ConnectionPoolOptions`** — `is_gateway20_allowed: bool` config (see §3.4 for gating model)
- **`CosmosTransport`** — `dataplane_gateway20_transport: OnceLock<AdaptiveTransport>`, lazy init with `AdaptiveTransport::gateway20()`
- **`AdaptiveTransport::ShardedGateway20`** variant — HTTP/2 only with prior knowledge (no HTTP/1.x fallback; the proxy does not accept HTTP/1.x — see Open Question Q1, resolved)
- **`HttpClientConfig::dataplane_gateway20()`** — HTTP/2-only config; HTTP/2 negotiation failure surfaces as a transport error (handled by the existing retry policies) rather than downgrading
- **`TransportKind::Gateway20`** in diagnostics

### 3.2 Already Implemented — account metadata & routing

- **`LocationStateStore`** — `gateway20_enabled` flag, passes through to endpoint construction
- **`AccountProperties::has_thin_client_endpoints()`** (`account_metadata_cache.rs:191`) — detection helper
- **`AccountProperties::thin_client_writable_regions()` / `thin_client_readable_regions()`** (`account_metadata_cache.rs:197,205`) — region accessors
- **`parse_thin_client_locations()`** — parser for `thinClient(Readable|Writable)Locations`
- **`build_account_endpoint_state()`** (`routing_systems.rs`) — resolves gateway 2.0 URLs from account properties
- **Existing tests** in `routing_systems.rs:218–289` already exercise GW20 endpoint construction with both readable and writable thin-client locations
- **`resolve_endpoint()`** in operation pipeline — selects gateway 2.0 URL when `prefer_gateway20` is true (see §3.4)

### 3.3 Already Implemented — EPK & constants

- **`EffectivePartitionKey::compute()` / `::compute_range()`** — in `azure_data_cosmos_driver::models::effective_partition_key` (MultiHash-aware, hierarchical-PK correct). This is the canonical path and is what Gateway 2.0 header injection MUST call. Both functions return `azure_core::Result` (per PR #4087 review: MultiHash-requires-V2 and component-count checks are runtime errors, not `debug_assert`s, so a user gets `Err` rather than a panic on malformed input).
- **Constants** (in `azure_data_cosmos::constants`): `THINCLIENT_PROXY_OPERATION_TYPE` → `x-ms-thinclient-proxy-operation-type`, `THINCLIENT_PROXY_RESOURCE_TYPE` → `x-ms-thinclient-proxy-resource-type`. Phase 2 reuses these verbatim. The existing `START_EPK` (= `x-ms-start-epk`) / `END_EPK` (= `x-ms-end-epk`) constants are **not** used on Gateway 2.0 requests; Phase 2 introduces new `THINCLIENT_RANGE_MIN` (= `x-ms-thinclient-range-min`) / `THINCLIENT_RANGE_MAX` (= `x-ms-thinclient-range-max`) constants per Q3 resolution. See §Phase 2 "Header naming" for mapping.
- **Perf crate** — `gateway20_allowed` config wiring

### 3.4 Gating model (single source of truth)

Two independent guards exist today (`is_gateway20_allowed` is checked in both routing and `get_dataplane_transport`). Per PR #3942 review, **routing is the single source of truth**; the transport-layer guard is intentional defense-in-depth and is technically dead code given current callers.

Invariants this spec locks in:

- `prefer_gateway20` is computed **once per request** during `resolve_endpoint` from:
  `connection_pool().is_gateway20_allowed() && account.has_thin_client_endpoints()`
- After `resolve_endpoint`, downstream stages MUST trust `RoutingDecision.transport_mode` and not re-derive eligibility.
- `ConnectionPoolOptions.is_gateway20_allowed` and its env var `AZURE_COSMOS_CONNECTION_POOL_IS_GATEWAY20_ALLOWED` are an **unsupported, undocumented kill switch** reserved for emergency fallback. They are NOT exposed on `CosmosClientOptions` and may be removed without notice.

### 3.5 Known broken / do-not-use

- **`azure_data_cosmos::hash::get_hashed_partition_key_string()`** (called from `container_connection.rs:87`) — a legacy SDK-side function that is **a known-broken stub for MultiHash (hierarchical-PK) containers**. PR #4087's description explicitly calls it out as awaiting the SDK-to-driver cutover. **Do NOT** wire Phase 2 header injection to this function; use `EffectivePartitionKey::compute()` / `::compute_range()` (§3.3).

### 3.6 Not Yet Implemented (Gaps)

1. **RNTBD serialization/deserialization** — No binary protocol encoding/decoding exists
2. **Gateway 2.0 header injection** — Thin client proxy headers and EPK range headers are not applied to requests on the Gateway 2.0 path
3. **Supported operation filtering** — No `IsOperationSupportedByThinClient()` equivalent
4. **`x-ms-cosmos-use-thinclient` header** on account metadata requests (to trigger thin-client endpoint advertisement)
5. **SDK-to-driver cutover for EPK** — SDK call sites (`feed_range_from_partition_key`, `container_connection.rs:87`) still call the broken SDK hash; they must route through the driver's `EffectivePartitionKey::compute()`
6. **Session token handling** — Gateway 2.0 may handle session tokens differently (partition-key-range-id prefix)
7. **Rollout/cutover policy clarification** — Document the intended enablement and cutover behavior (see Phase 4); there is intentionally **no** Gateway 2.0-specific failure-driven fallback to the standard gateway
8. **Integration/E2E tests** — No gateway 2.0 test coverage beyond the routing-systems unit tests
9. **Fault injection** — No gateway 2.0 fault injection scenarios
10. **Constants cross-crate visibility** — `THINCLIENT_PROXY_*` and `START_EPK` / `END_EPK` currently live in `azure_data_cosmos::constants` but Phase 2 injects headers from the driver crate. Options (to decide in Phase 2): (a) move constants to `azure_data_cosmos_driver::constants` and re-export from SDK, (b) re-export SDK constants through a driver-side `pub use`, or (c) duplicate. Recommend (a).

---

## 4. Rust Implementation Plan

### 4.1 Current Request Flow (Gateway 1.0)

1. `CosmosClient::create_item(T)` calls `ContainerClient`
2. `container_connection.rs` serializes `T` to `&[u8]`, computes EPK (via the broken SDK hash today — see §3.5), resolves PKRange
3. `CosmosDriver::execute_operation()` enters the Operation Pipeline (7-stage loop)
4. `resolve_endpoint()` selects a gateway endpoint
5. Transport Pipeline applies cosmos headers, signs request
6. HTTP/REST request sent to Cosmos Gateway (shared proxy, no SLA)

### 4.2 Target Request Flow (Gateway 2.0)

1. `CosmosClient::create_item(T)` calls `ContainerClient`
2. `container_connection.rs` serializes `T` to `&[u8]`; EPK computation is deferred to the driver (via `EffectivePartitionKey::compute()` / `::compute_range()`), which then resolves PKRange
3. `CosmosDriver::execute_operation()` enters the Operation Pipeline (7-stage loop)
4. `resolve_endpoint()` prefers gateway 2.0 endpoint (if `prefer_gateway20` per §3.4)
5. Transport Pipeline checks `is_operation_supported_by_gateway20()`:
   - **YES**: Inject gateway 2.0 headers + RNTBD serialize → HTTP/2 POST to Gateway 2.0 Proxy (SLA)
   - **NO**: Standard HTTP/REST request to Cosmos Gateway (eligibility fallback — per-request, deterministic)

> **Naming**: The function is `is_operation_supported_by_gateway20()` throughout. Older drafts used `is_supported_by_gw20()` — do not reintroduce the abbreviation.

---

### Phase 1: RNTBD Protocol (Driver Layer)

**Crate**: `azure_data_cosmos_driver`
**New module**: `src/driver/transport/rntbd/`

The RNTBD ("Real Name To Be Determined" — a placeholder name that stuck) protocol is the wire format used by Cosmos DB for efficient binary communication. Gateway 2.0 wraps RNTBD-encoded payloads inside HTTP/2 POST requests to the proxy.

#### What Will Be Done

- **`rntbd/mod.rs`** — Module root, public types
- **`rntbd/request.rs`** — Request serialization: operation headers, resource metadata, partition key info → binary payload
- **`rntbd/response.rs`** — Response deserialization: 24-byte frame header → metadata section → optional body payload
- **`rntbd/tokens.rs`** — RNTBD token types (type IDs, lengths, value encodings) used in metadata sections
- **`rntbd/status.rs`** — RNTBD status code mapping to `CosmosStatus`

#### Versioning

Thin client RNTBD has no version negotiation on the wire. The proxy advertises a single supported frame format per endpoint and rejects mismatched frames at the HTTP layer (the HTTP/2 request fails rather than triggering an RNTBD version-mismatch error). Direct-mode RNTBD has version negotiation (`CURRENT_PROTOCOL_VERSION = 0x00000001`); **do not** apply that pattern here.

#### RNTBD Request Wire Format

The frame layout is derived from Java `com.azure.cosmos.implementation.directconnectivity.rntbd.RntbdRequestFrame.encode(...)`, which writes:

```java
out.writeIntLE(totalLength);
out.writeShortLE(resourceType.id());
out.writeShortLE(operationType.id());
RntbdUUID.encode(activityId, out);  // two longs
```

| Offset | Size | Field | Encoding | Notes |
| --- | --- | --- | --- | --- |
| 0 | 4 | Total message length | uint32 LE | **Inclusive** of the 4 length bytes themselves (matches Java `writeIntLE` semantics). |
| 4 | 2 | Resource type | uint16 LE | `writeShortLE(resourceType.id())` — narrower than direct-mode RNTBD's uint32 because thin-client IDs fit in 16 bits. |
| 6 | 2 | Operation type | uint16 LE | `writeShortLE(operationType.id())` — same rationale. |
| 8 | 16 | Activity ID | UUID, two uint64 LE | Java writes `(mostSignificantBits, leastSignificantBits)` as two little-endian `long`s — **this is not RFC 4122 byte order**. Worked example for UUID `0a1b2c3d-4e5f-6789-abcd-ef0123456789`: `mostSignificantBits = 0x0a1b2c3d_4e5f_6789` → LE bytes `89 67 5f 4e 3d 2c 1b 0a`; `leastSignificantBits = 0xabcd_ef01_2345_6789` → LE bytes `89 67 45 23 01 ef cd ab`. The on-the-wire 16-byte sequence is the MSB bytes followed by the LSB bytes. |
| 24 | var | Metadata tokens | Token stream | Filtered by `thinClientProxyExcludedSet` (see §Phase 2 header naming). |
| 24+N | 4 | Payload length | uint32 LE | **Only present when the operation type implies a payload** (writes, patch, query body, stored-proc args, batch). Absence is signaled by operation-type convention, not a flag bit. Parsers must consult the operation-type → has-payload table derived from Java's `RntbdRequestArgs`. |
| 28+N | var | Payload body | Raw bytes | JSON or Cosmos binary, per resource type. |

#### RNTBD Response Wire Format

| Offset | Size | Field | Encoding | Notes |
| --- | --- | --- | --- | --- |
| 0 | 4 | Total message length | uint32 LE | Inclusive of the 4 length bytes (same convention as request). |
| 4 | 4 | Status code | uint32 LE | Maps to HTTP status + `CosmosStatus`. |
| 8 | 16 | Activity ID | UUID, two uint64 LE | Same MSB-LE / LSB-LE pairing as request. |
| 24 | var | Metadata tokens | Token stream | Request charge, session token, continuation, etc. |
| 24+N | var | Body payload | Raw bytes | Optional; presence determined by total-length arithmetic (`total_length - header_and_tokens_len > 0`). |

#### Files Changed

```
NEW   src/driver/transport/rntbd/mod.rs        — Module root + public types
NEW   src/driver/transport/rntbd/request.rs     — serialize_request() → Vec<u8>
NEW   src/driver/transport/rntbd/response.rs    — deserialize_response(&[u8]) → RntbdResponse
NEW   src/driver/transport/rntbd/tokens.rs      — Token type definitions + encoding
NEW   src/driver/transport/rntbd/status.rs      — RNTBD ↔ CosmosStatus mapping
EDIT  src/driver/transport/mod.rs               — Add `pub(crate) mod rntbd;`
```

---

### Phase 2: Gateway 2.0 Request Pipeline (Driver Layer)

**Crate**: `azure_data_cosmos_driver`

This phase wires RNTBD serialization into the existing transport pipeline and adds gateway 2.0-specific header injection.

#### What Will Be Done

- **Operation filtering** — `is_operation_supported_by_gateway20(resource_type, operation_type) → bool`. Following Java (`ThinClientStoreModel`), only `ResourceType::Document` operations are eligible. The .NET position (`IsOperationSupportedByThinClient` additionally allows `StoredProcedure::ExecuteJavaScript`) is **intentionally not adopted**.
- **EPK computation** — Call `EffectivePartitionKey::compute()` (point) or `::compute_range()` (feed/cross-partition) from the driver layer. Do **not** call `azure_data_cosmos::hash::get_hashed_partition_key_string` (§3.5). SDK call sites that currently use it must route through the driver's implementation as part of this phase.
- **EPK error propagation** — If EPK computation returns `Err` (MultiHash-requires-V2, component-count mismatch, etc.), surface as `CosmosStatus::BadRequest` to the caller. **Do not** fall back to standard gateway — the same inputs would be equally broken there.
- **Header injection** — When `transport_mode == Gateway20`, inject the thin-client headers listed below.
- **Request body wrapping** — Serialize the entire request (headers + body) into RNTBD binary format and POST as the HTTP/2 body.
- **Response unwrapping** — Deserialize the RNTBD response body back into `CosmosResponseHeaders` + raw document bytes.
- **Eligibility fallback** — Operation ineligible for Gateway 2.0 → route through standard gateway for this single request (per-request, deterministic). See §Phase 4 for the distinct failure-driven fallback.
- **Constants placement** — Resolve the cross-crate constants question from §3.6-10 (recommend: move `THINCLIENT_PROXY_*` and `START_EPK` / `END_EPK` to a driver-side module, re-export from SDK).

#### Supported Operations

Only `ResourceType::Document` is eligible for gateway 2.0 (following Java's approach):

| Operation | Supported | Notes |
| --- | --- | --- |
| Create | Yes | |
| Read | Yes | |
| Replace | Yes | |
| Upsert | Yes | |
| Delete | Yes | |
| Patch | Yes | |
| Query | Yes | |
| QueryPlan | Yes | |
| ReadFeed | Yes | LatestVersion change feed only; excludes AllVersionsAndDeletes |
| Batch | Yes | Transactional same-PK batch (single resource, single request). |
| Bulk | Yes | SDK-side fan-out of independent CRUD ops; each fan-out leg is a separate eligible Document op. Distinct from Batch. |
| StoredProcedure Execute | **No** | Following Java; Rust does **not** follow .NET's `ExecuteJavaScript` allowance. |
| All other resource types | **No** | Metadata operations use standard gateway |

#### Header naming (proxy headers, in HTTP/2 request headers — not RNTBD tokens)

These are wire-level HTTP/2 request headers on the outer POST to the proxy. They are **not** inside the RNTBD metadata token stream.

| Header (wire) | Rust constant (crate) | Semantics | When emitted |
| --- | --- | --- | --- |
| `x-ms-thinclient-proxy-operation-type` | `THINCLIENT_PROXY_OPERATION_TYPE` (SDK today; move to driver per §3.6-10) | Numeric operation type | Every Gateway 2.0 request |
| `x-ms-thinclient-proxy-resource-type` | `THINCLIENT_PROXY_RESOURCE_TYPE` (SDK today; move) | Numeric resource type | Every Gateway 2.0 request |
| `x-ms-effective-partition-key` | **NEW** — `EFFECTIVE_PARTITION_KEY` (driver) | Canonical EPK hex | Point ops only |
| `x-ms-thinclient-range-min` | **NEW** — `THINCLIENT_RANGE_MIN` (driver) | Lower bound of EPK range | Feed / cross-partition ops only |
| `x-ms-thinclient-range-max` | **NEW** — `THINCLIENT_RANGE_MAX` (driver) | Upper bound of EPK range | Feed / cross-partition ops only |
| `x-ms-cosmos-use-thinclient` | **NEW** (driver) | Instructs account-metadata response to advertise thin-client endpoints | Account metadata fetches only |

Per Q3 resolution, the Gateway 2.0 proxy requires the Java header names `x-ms-thinclient-range-min` / `x-ms-thinclient-range-max` (it does **not** accept `x-ms-start-epk` / `x-ms-end-epk`). Phase 2 introduces the new constants above; the existing `START_EPK` / `END_EPK` constants are not emitted on the Gateway 2.0 path.

#### Range header wire format

EPK range headers (`x-ms-thinclient-range-min` / `-max`) carry the canonical, un-padded hex produced by `EffectivePartitionKey::compute_range()`. **Do not** zero-pad to N×32 on the wire. Local comparisons use `EffectivePartitionKey`'s `Ord` / `cmp` impl, which correctly handles the mixed-length boundaries returned by the backend; the `epk_cmp_*` tests in `container_routing_map.rs` (around L625–665) pin this behavior. The comparator is consumed via `binary_search_by(|r| r.min_inclusive.cmp(&epk_val))` (≈L282 of the same file). `@analogrelay`'s earlier zero-padding proposal in PR #4087 (commit `25233c903`) was **not** adopted; stay consistent with the length-aware convention.

> **`Range` semantics footgun** (from PR #4087): `compute_range` returns a Rust `std::ops::Range<EffectivePartitionKey>` where `start == end` denotes a **point operation**. Standard `Range` iteration treats that as empty, so code that uses `.contains()` or iterates the range directly will misbehave. Always treat `start == end` as the point case explicitly.

#### Gateway 2.0 Header Injection Flow

When `transport_mode == Gateway20`:

1. Set `x-ms-thinclient-proxy-operation-type` (numeric operation type)
2. Set `x-ms-thinclient-proxy-resource-type` (numeric resource type)
3. Point operation? Set `x-ms-effective-partition-key` (EPK hash from `EffectivePartitionKey::compute()`)
   Feed operation? Set `x-ms-thinclient-range-min` and `x-ms-thinclient-range-max` (from `EffectivePartitionKey::compute_range()`)
4. Serialize headers + body into RNTBD binary format (Phase 1)
5. POST RNTBD body to gateway 2.0 endpoint via HTTP/2

When `transport_mode != Gateway20`: Standard HTTP/REST request (existing flow, unchanged).

#### Files Changed

```
NEW   src/driver/transport/gateway20.rs          — inject_gateway20_headers(), RNTBD wrap/unwrap
EDIT  src/driver/transport/transport_pipeline.rs  — Branch on TransportMode in execute_transport_pipeline()
EDIT  src/driver/transport/cosmos_headers.rs      — Add gateway 2.0 header application
EDIT  src/driver/transport/mod.rs                 — Add is_operation_supported_by_gateway20()
EDIT  src/driver/pipeline/components.rs           — Add EPK fields to TransportRequest if needed
EDIT  src/driver/constants.rs (or NEW)            — Relocate THINCLIENT_PROXY_* constants per §3.6-10
EDIT  sdk/cosmos/azure_data_cosmos/src/...        — Replace SDK-side get_hashed_partition_key_string callers with driver's EffectivePartitionKey::compute()
```

---

### Phase 3: Endpoint Discovery — verification & one new header

**Crate**: `azure_data_cosmos_driver`

> Most of Phase 3 is **audit / verification** against scaffolding already in place (§3.1, §3.2). Only the `x-ms-cosmos-use-thinclient` request header is net-new code. Noted here because the dependency graph lists Phase 3 as a prerequisite for Phase 2; in practice the verification items can happen in parallel with Phase 1 and the one real code change can ride with Phase 2 if convenient.

#### What Will Be Done

- **Verify** account metadata cache parses `thinClientReadableLocations` / `thinClientWritableLocations` into `CosmosEndpoint::gateway20_url` (existing, per §3.2)
- **Confirm** `build_account_endpoint_state()` constructs `CosmosEndpoint::regional_with_gateway20()` correctly in multi-region accounts (existing tests at `routing_systems.rs:218–289` already cover this)
- **Verify** `AccountProperties::has_thin_client_endpoints()` is used as the gating signal per §3.4
- **Add** `x-ms-cosmos-use-thinclient` request header on account metadata fetches (new code)
- **Test** endpoint discovery with live account that has gateway 2.0 enabled (handled by Phase 6 live pipeline)

#### Region pairing (lock in the §PR #3942 decision)

Thin-client read locations pair **only** with read regions; thin-client write locations pair **only** with write regions. A write region that advertises no thin-client URL falls back to standard gateway **for writes** (this was deliberate in PR #3942: session retries that reroute reads to write endpoints would otherwise cross the read/write thin-client split). This is a correctness invariant — do not "fix" it by cross-pairing.

#### Endpoint Discovery Flow (Existing)

Account metadata response includes:

- `writableLocations` — standard gateway URLs
- `readableLocations` — standard gateway URLs
- `thinClientWritableLocations` — gateway 2.0 URLs (when available)
- `thinClientReadableLocations` — gateway 2.0 URLs (when available)

`build_account_endpoint_state()` matches regions across these lists and constructs `CosmosEndpoint::regional_with_gateway20(region, gw_url, gw20_url)`. The resulting `AccountEndpointState` contains endpoints with `gateway20_url: Some(...)` when gateway 2.0 is available for that region.

#### Files Changed

```
EDIT  src/driver/cache/account_metadata_cache.rs   — Verify thin client endpoint parsing (audit only)
EDIT  src/driver/transport/cosmos_headers.rs       — Add x-ms-cosmos-use-thinclient header (NEW)
TEST  src/driver/routing/routing_systems.rs        — Add tests for read/write pairing edge cases
```

---

### Phase 4: Retry & Error Handling (Driver Layer)

**Crate**: `azure_data_cosmos_driver`

Retry policies are identical between Gateway 2.0 and standard gateway modes in both Java and .NET — only endpoint selection and request encoding differ. The existing retry pipeline should work as-is for most cases.

#### What Will Be Done

- **Timeout policy** — Gateway 2.0 requests use the timeout regime defined in `TRANSPORT_PIPELINE_SPEC.md` (single timeout, not bifurcated). Do not introduce Gateway-2.0-specific timeouts in this work; any Gateway 2.0–specific timeout tuning will be addressed in a follow-up.
- **Gateway 2.0 eligibility fallback** — see "Fallback taxonomy" below.
- **Partition-Level Failover interaction** — when PLF (see `PARTITION_LEVEL_FAILOVER_SPEC.md`) selects a region, the per-request decision is: if that region's `CosmosEndpoint` exposes a `gateway20_url`, the request uses Gateway 2.0; if it does not, the request falls back to standard gateway for that partition until PLF releases its override. PLF chooses the region; Gateway 2.0 is preferred whenever it is available in that region.

#### Fallback taxonomy

Gateway 2.0 has a single fallback mechanism:

| Name | Scope | Trigger | Duration | Unwind |
| --- | --- | --- | --- | --- |
| **Eligibility fallback** | Per-request | Operation is not eligible for Gateway 2.0 (fails `is_operation_supported_by_gateway20()`) | Single request only | N/A — recomputed every request |

There is intentionally **no** Gateway 2.0–specific failure-fallback mechanism (no per-partition consecutive-failure counter, no sticky standard-gateway state, no cooldown). Java's thin client takes the same posture: `ThinClientStoreModel extends RxGatewayStoreModel`, model selection is per-request and stateless via `useThinClientStoreModel()`, and the existing `ClientRetryPolicy` / `WebExceptionRetryPolicy` chain already handles transport errors, 502/503/504, and regional unavailability uniformly across both transport modes. Rust follows the same approach: when a Gateway 2.0 request fails, the existing retry policies retry it (which may re-select Gateway 2.0 or land on standard gateway through normal regional-failover behavior); no new state machine is introduced.

#### Files Changed

```
EDIT  src/driver/pipeline/operation_pipeline.rs   — Gateway 2.0 retry classification + PLF precedence
EDIT  src/driver/pipeline/components.rs           — Gateway 2.0 retry surface integration
```

---

### Phase 5: SDK Integration

**Crate**: `azure_data_cosmos`

Gateway 2.0 is **not exposed as a customer-facing configuration**. The SDK automatically uses gateway 2.0 when the account metadata advertises thin client endpoints. This matches the design philosophy of both Java and .NET SDKs.

#### What Will Be Done

- **Auto-detection** — When account metadata includes `thinClientReadableLocations` / `thinClientWritableLocations`, the driver automatically prefers gateway 2.0 for eligible operations (per §3.4). No user opt-in required.
- **Internal kill switch** — `ConnectionPoolOptions.is_gateway20_allowed` and its env var (§3.4) remain internal. They are NOT exposed in `CosmosClientOptions` and are unsupported/undocumented.
- **Diagnostics** — `CosmosDiagnostics` should report when a request used gateway 2.0 vs standard gateway (already partially done via `TransportKind::Gateway20`).
- **User agent** — Update SDK user agent string to indicate gateway 2.0 capability.
- **EPK cutover** — Replace SDK-side callers of `get_hashed_partition_key_string` with calls into the driver's `EffectivePartitionKey::compute()` / `::compute_range()` (this is the cutover PR #4087 flagged). Gateway 2.0 header injection depends on this being correct for hierarchical-PK containers.

#### Auto-Detection Flow

When account metadata includes `thinClientReadableLocations`, gateway 2.0 is enabled automatically (internal). `CosmosEndpoint` gets `gateway20_url` and `resolve_endpoint()` prefers Gateway 2.0 (per §3.4's single-source-of-truth rule). No user configuration needed — transparent to the caller.

#### Files Changed

```
EDIT  src/driver_bridge.rs                        — Ensure internal config passes through
EDIT  src/handler/container_connection.rs         — Route EPK through driver's EffectivePartitionKey::compute()
EDIT  src/partition_key.rs                        — Update feed_range_from_partition_key call site
EDIT  src/constants.rs                            — Relocate / re-export header constants per §3.6-10
```

---

### Phase 6: Testing

Testing covers all layers from unit to E2E, matching or exceeding Java/.NET test coverage.

#### Live Tests Pipeline

A **new dedicated CI pipeline** is required for gateway 2.0 live tests. Gateway 2.0 requires a Cosmos DB account with thin client endpoints enabled, which is separate from the standard emulator and live test infrastructure.

**Trigger:** PR changes to `sdk/cosmos/**` + manual dispatch

**Provision:**

- Use a **dedicated, pre-provisioned Cosmos DB account** with gateway 2.0 / thin client endpoints enabled (hardcoded for this pipeline, reused across runs)
- Account credentials stored in pipeline secrets (e.g., `AZURE_COSMOS_GW20_ENDPOINT`, `AZURE_COSMOS_GW20_KEY`)
- Multi-region configuration (at least 2 regions)
- Verify `thinClientReadableLocations` in account metadata at pipeline start

**Test Matrix:**

- Single-region gateway 2.0
- Multi-region gateway 2.0 with failover
- Gateway 2.0 + standard gateway eligibility fallback (per-request only; normal retries still apply)

**Test Suites:**

- Point CRUD (create, read, replace, upsert, patch, delete)
- Query (single-partition, cross-partition)
- Batch operations
- Change feed (LatestVersion)
- Retry scenarios (408, 410, 503)
- Diagnostics validation (`TransportKind::Gateway20`)

**Artifacts:** Test results (JUnit XML), diagnostics logs, perf metrics (RU, latency)

#### Pipeline Files

| Action | File | Purpose |
| --- | --- | --- |
| NEW | `sdk/cosmos/ci-gateway20.yml` | Gateway 2.0 live tests pipeline definition (uses pre-provisioned account) |
| EDIT | `sdk/cosmos/live-platform-matrix.json` | Add gateway 2.0 test matrix entry |

#### Test Coverage Matrix

| Test Category | Unit | Integration | E2E | Scenarios |
| --- | --- | --- | --- | --- |
| RNTBD serialization | Yes | | | Round-trip, edge cases, malformed input |
| EPK computation | Yes | | | Single/hierarchical PK, hash versions 1 and 2, error cases (MultiHash V1, wrong component count) |
| Operation filtering | Yes | | | All ResourceType × OperationType combos; asserts StoredProc Execute is rejected |
| Header injection | Yes | | | Point vs feed EPK headers, proxy type headers, range-header un-padded form |
| Gateway 2.0 transport | Yes | Yes | | Correct HTTP/2 config, sharded pool selection |
| Read/write pairing | Yes | | | Write region without thin-client falls back for writes only |
| Point CRUD | | | Yes | Create, read, replace, upsert, patch, delete |
| Query | | | Yes | SQL query, cross-partition |
| Batch | | | Yes | Transactional batch ops |
| Bulk | | | Yes | Fan-out CRUD, distinct from Batch |
| Change feed | | | Yes | LatestVersion, incremental |
| Retry: 408 timeout | | Yes | | Cross-region for reads, local-only for writes |
| Retry: 503 | | Yes | | Regional failover via existing retry policies |
| Retry: 410 Gone | | Yes | | PKRange refresh (sub-status specific); NameCacheStale → collection cache |
| Eligibility fallback | | Yes | | StoredProc Execute → standard gateway |
| PLF precedence | | Yes | | Region without gw20_url + PLF override → standard gateway path |
| Multi-region failover | | Yes | Yes | Preferred regions, failover |
| Fault injection | | Yes | | Timeout, 503, network error |
| Perf benchmarks | | | Yes | Already wired in perf crate |
| Diagnostics validation | Yes | Yes | | TransportKind::Gateway20 in diagnostics output |

#### Files Changed

| Action | File | Purpose |
| --- | --- | --- |
| NEW | `tests/gateway20_rntbd_tests.rs` | RNTBD unit tests (driver) |
| NEW | `tests/gateway20_pipeline_tests.rs` | Header injection + operation filtering (driver) |
| NEW | `tests/emulator_tests/gateway20_e2e.rs` | E2E tests (SDK, requires emulator) |
| EDIT | `tests/emulator_tests/cosmos_fault_injection.rs` | Add gateway 2.0 fault scenarios |
| EDIT | `azure_data_cosmos_perf/src/runner.rs` | Perf config already wired |

---

### Phase Dependency Graph

- **Phase 1** (RNTBD Protocol) and the verification parts of **Phase 3** (Endpoint Discovery) can proceed in parallel
- **Phase 2** (Request Pipeline) depends on Phase 1, and folds in Phase 3's one new header (`x-ms-cosmos-use-thinclient`)
- **Phase 4** (Retry/Errors) and **Phase 5** (SDK Integration) depend on Phase 2
- **Phase 6** (Testing) depends on all previous phases

---

## 5. Open Questions

- **Q1 — HTTP/2 prior knowledge vs ALPN**: _Resolved_. Gateway 2.0 always uses HTTP/2; the proxy does not accept HTTP/1.x. Rust uses HTTP/2 with prior knowledge on the Gateway 2.0 transport (no ALPN fallback to HTTP/1.x). The broader ALPN default in `TRANSPORT_PIPELINE_SPEC.md` does **not** apply to Gateway 2.0; if HTTP/2 negotiation fails, the request fails and the existing retry policies handle it.
- **Q2 — Live test account provisioning**: Cosmos DB account configuration flags required to enable gateway 2.0 / thin client endpoints are not part of the standard Bicep templates. _Resolution_: hardcode a dedicated, pre-provisioned thin client account for the gateway 2.0 live tests pipeline and reuse it across runs (rather than provisioning per-run via Bicep). Account name and credentials stored in pipeline secrets (`AZURE_COSMOS_GW20_ENDPOINT`, `AZURE_COSMOS_GW20_KEY`); pipeline reads endpoint from environment variables.
- **Q3 — EPK range header names**: _Resolved_. The Gateway 2.0 proxy requires the Java header names `x-ms-thinclient-range-min` / `x-ms-thinclient-range-max`. Phase 2 introduces new constants (`THINCLIENT_RANGE_MIN`, `THINCLIENT_RANGE_MAX`) on the Gateway 2.0 path; the existing `START_EPK` / `END_EPK` (`x-ms-start-epk` / `x-ms-end-epk`) constants remain for any non-Gateway-2.0 callers but are **not** emitted on Gateway 2.0 requests.
