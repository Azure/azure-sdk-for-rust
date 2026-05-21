<!-- cspell:ignore THINCLIENT thinclient Mgmt cutover directconnectivity cooldown ALPN myacct pushdown -->
# Gateway 2.0 Design Spec for Rust Driver & SDK

**Status**: Draft / Iterating
**Date**: 2026-04-20
**Authors**: (team)

---

## Table of Contents

1. [Overview](#1-overview)
2. [Motivation](#2-motivation)
3. [Gating, Configuration & Override](#3-gating-configuration--override)
4. [Retry Behavior](#4-retry-behavior)
5. [Rust Implementation Plan](#5-rust-implementation-plan)
6. [Open Questions](#6-open-questions)

### Related Specs

- [`TRANSPORT_PIPELINE_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/TRANSPORT_PIPELINE_SPEC.md) — sharded HTTP/2 transport, timeout regime, hedging, `(HttpClient, host:port)` shard key. Gateway 2.0 reuses the sharded transport defined there verbatim; this spec does **not** introduce a new timeout or hedging policy.
- [`PARTITION_KEY_RANGE_CACHE_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/PARTITION_KEY_RANGE_CACHE_SPEC.md) — PKRange cache semantics and `EffectivePartitionKey` usage; cited by Phase 2 for EPK computation and by Phase 4 for 410 handling.
- [`PARTITION_LEVEL_FAILOVER_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/PARTITION_LEVEL_FAILOVER_SPEC.md) — per-partition region override semantics; cited by Phase 4 for PLF precedence over Gateway 2.0 routing.

---

## 1. Overview

Gateway 2.0 (formerly "thin client") is a server-side proxy that evolves the existing Gateway V1 path: SDK clients still target a regional proxy endpoint instead of opening connections to backend replicas, but the proxy uses RNTBD binary message encoding over HTTP/2 in place of REST/JSON, and the proxy itself owns replica selection, connection management, and load balancing within a partition.

**RNTBD** stands for "Real Name to be Determined" — the (intentionally placeholder) code name for the proprietary message-encoding and wire-protocol format originally introduced for direct mode. Gateway 2.0 keeps the **message encoding** layer of RNTBD and moves the **wire protocol** to HTTP/2; direct mode used RNTBD over TCP.

Direct mode was never in scope for the Rust SDK, so the rest of this spec compares Gateway 2.0 to **Gateway V1**, not to direct mode.

**Naming**: Use "Gateway 2.0" consistently in all Rust code, docs, and comments. Reserve "thin client" for two narrow uses: (a) when referencing Java/.NET source symbols (`ThinClientHttpMessageHandler`, etc.), and (b) the literal wire-header names that contain `thinclient` (e.g. `x-ms-thinclient-proxy-operation-type`).

---

## 2. Motivation

### Why Gateway 2.0?

Today the Rust SDK only supports Gateway V1: a shared, stateless HTTP/REST proxy that adds a network hop and provides **no latency SLA**. (Direct mode — where an SDK opens TCP connections directly to backend replicas — was never implemented for Rust because it introduces operational cost: COGS for replica connections, plus debugging complexity for customers when network paths to backend nodes break. It is not in scope.)

**Gateway 2.0 bridges this gap.** It keeps the operational shape of Gateway V1 (one regional endpoint, no direct backend connectivity required) and adds the performance characteristics of direct mode (RNTBD binary message encoding, server-side replica selection, replica-aware load balancing) — so customers get latency SLAs without taking on the operational burden direct mode imposed.

### Key Benefits

- **SLA latency guarantees** — Unlike Gateway V1, Gateway 2.0 plans to provide contractual latency commitments comparable to direct mode
- **Reduced operational cost** — The proxy handles replica discovery, connection management, and replica-level routing within a partition; the client stays partition-aware (resolves PKRange, computes EPK) but is **never replica-aware**, avoiding the COGS and customer-side debugging burden of maintaining one connection per backend replica.
- **Transparent failover** — The proxy handles replica failover within a partition; the SDK handles regional failover across proxy endpoints

### Design Philosophy

Gateway 2.0 moves **replica-level** routing intelligence from the SDK into the server-side proxy while keeping **regional and partition-level** routing in the SDK. The SDK still resolves PKRanges, computes EPK headers, and selects the regional endpoint; what moves to the proxy is the per-request choice of which replica within a partition serves the operation, plus the connection management and load balancing that goes with it. This gives the best of both worlds:

**SDK Responsibility:**

- Regional endpoint selection
- Partition routing (PKRange resolution, EPK→PKRangeId mapping)
- RNTBD serialization
- EPK header injection
- Cross-partition query aggregation (unchanged from Gateway/Direct modes — the SDK continues to issue per-partition sub-queries and aggregate results client-side; Gateway 2.0 does not server-side aggregate)

**Gateway 2.0 Proxy (Server-Side):**

- Replica selection within a partition
- Connection management
- Load balancing

### Connection Mode Comparison

| Aspect | Gateway V1 | Gateway 2.0 | Direct (not in scope for Rust) |
| --- | --- | --- | --- |
| Latency SLA | No | **Yes** | Yes |
| Simple Network | Yes | Yes | No |
| Protocol | REST/HTTP over HTTP/2 | RNTBD message encoding over HTTP/2 | RNTBD over TCP |
| Replica Mgmt | Gateway/Proxy | Proxy | SDK |
| Partition Route | Gateway/Proxy | Proxy | SDK |
| Regional Route | SDK | SDK | SDK |
| Operational Cost (COGS + debug) | Low | Low | High |

---

## 3. Gating, Configuration & Override

Gateway 2.0 routing is decided **once per logical operation** (a point operation, a single query-iteration page, a single batch, etc.) in the driver's `resolve_endpoint` stage. The resulting `RoutingDecision.transport_mode` is then attached to the operation context and **inherited by all retries and sub-requests** of that operation — downstream pipeline stages, retry policies, and hedged sub-requests MUST trust the attached decision and not re-derive eligibility. (Re-evaluating `gateway20_suppressed` mid-operation could route a retry through a different transport than the original attempt, fragmenting diagnostics and breaking session-token affinity.)

### 3.1 The `gateway20_suppressed` formula

```text
gateway20_suppressed = options.gateway20_disabled
                    || !account.has_thin_client_endpoints()
```

When `gateway20_suppressed` is `false` (the default whenever the account advertises Gateway 2.0 endpoints and the operator has not flipped the override), the request routes through Gateway 2.0. When it is `true`, the request falls through to Gateway V1. The account-side check (`has_thin_client_endpoints()`) reads the cached account metadata. The client-side check (`gateway20_disabled`) is the only public toggle.

### 3.2 Operator override: `CosmosClientOptions::gateway20_disabled`

Default `false`. Customers and operators MAY set `gateway20_disabled = true` on `CosmosClientOptions` to force every request from the client to route through Gateway V1, even when the account advertises Gateway 2.0 endpoints and the operation would otherwise be eligible.

⚠️ **Setting this flag voids the latency-SLA story Gateway 2.0 is being built to deliver. It also impacts the ability to receive 24/7 Microsoft support for performance regressions on this client. Use only when explicitly directed by Microsoft Support during incident triage.** The flag is intentionally **not** exposed via environment variable to discourage casual / fleet-wide enablement; operators who need it must opt in per-client through code.

All settings, options, and internal flags **must use a negative-term name** (`gateway20_disabled`, `gateway20_suppressed`, etc.) so that **default values mean Gateway 2.0 is enabled**. Positive-term names (`is_gateway20_allowed`, `gateway20_allowed`, `enable_gateway20`, etc.) are not permitted anywhere — driver, SDK, perf crate, env vars, or test wiring. `gateway20_disabled` is the single supported disablement mechanism; there is no `AZURE_COSMOS_*` env var that toggles Gateway 2.0.

### 3.3 EPK Range types — driver crate is canonical

Every Gateway 2.0 EPK-range representation lives in the **driver crate** (`azure_data_cosmos_driver`):

| Type | Role |
| --- | --- |
| `azure_data_cosmos_driver::models::range::EpkRange<T>` | Generic typed EPK range (`min` / `max` / `is_min_inclusive` / `is_max_inclusive` + `contains` / `is_empty` / `check_overlapping` / `Display` `[a,b)` form) |
| `azure_data_cosmos_driver::models::partition_key_range::PartitionKeyRange` | Service model with `min_inclusive: EffectivePartitionKey` / `max_exclusive: EffectivePartitionKey` and full PKR metadata |
| `azure_data_cosmos_driver::models::effective_partition_key::EffectivePartitionKey` | Strongly-typed EPK newtype with `compute_range()` returning `std::ops::Range<EffectivePartitionKey>` |

EPK header injection MUST consume `EffectivePartitionKey::compute_range()` directly and serialize through the driver crate's existing types. It MUST NOT introduce a new EPK-range struct, and MUST NOT depend on any SDK-crate analog (`azure_data_cosmos::routing::range::Range`, `azure_data_cosmos::routing::partition_key_range::PartitionKeyRange`, `azure_data_cosmos::hash::EffectivePartitionKey`). The SDK has no Gateway-2.0 surface area whatsoever — the SDK calls the generic `CosmosDriver::execute_operation` interface and the driver decides Gateway 2.0 vs Gateway V1 internally.

---

## 4. Retry Behavior

Gateway 2.0 reuses the standard retry pipeline. Two status codes have Gateway-2.0-relevant rules; one has its own dedicated policy.

### 4.1 HTTP 449 (Retry-With) — dedicated policy, separate from 410/Gone

449 typically signals a server-side precondition failure (e.g. etag conflict on a single document being updated by many writers). Aggressive retry loops on 449 amplify pathological client patterns — for example, a numeric-ID generator where dozens or hundreds of threads patch the same document. The retry policy MUST optimize for the natural case (precondition failure that resolves with a brief wait) without amplifying abuse.

The 449 retry rules are expressed as a **new `ThrottleAction` variant** that this spec
introduces. The existing `ThrottleAction` enum in `driver/pipeline/components.rs` (today:
`Retry { delay, new_state: ThrottleRetryState } | Propagate`) gains a third variant —
`RetryWith { delay, new_state: RetryWithState }` — and `decide_throttle_action` in
`driver/transport/transport_pipeline.rs` is extended to emit that variant whenever the
resolved `(status_code, sub_status)` pair is `(449, *)`. The new `RetryWithState` struct
owns the 449-specific retry budget and is **distinct from** `ThrottleRetryState`, which
guarantees structurally that 449 retries cannot consume the 410/Gone or 429 budget (and
vice versa).

- **Few attempts** (≤ 3) with **exponential backoff** between them.
- Retry against the **same** endpoint; do not switch regions on 449.
- **Independent retry budget** — `RetryWithState` does not share counters with
  `ThrottleRetryState` or the 410/Gone retry path. A 449 followed by a 410 on the same
  logical operation gets fresh budget on the 410 side, and vice versa.
- **Gateway V1 uses the identical 449 policy.** When the server-side adds a "suppress 449" capability (under design with the server team), the client negotiates it via the `SdkSupportedCapabilities` channel and treats 449 as a non-retryable terminal error. Until that capability ships, both transports retry per the policy above.

### 4.2 HTTP 404 (Not Found) with sub-status `1002` (`READ_SESSION_NOT_AVAILABLE`)

**Always prefer a remote region for the retry** when one is available in the client's preferred-region list — the local region is suspected of carrying the stale routing, so pinning the retry to the same Gateway 2.0 endpoint that just returned 1002 reproduces the bug. **PLF takes precedence**: if PLF (per `PARTITION_LEVEL_FAILOVER_SPEC.md`) has already pinned a region for this PKRangeId, the PLF region wins over the "prefer remote" hint.

These rules apply uniformly to V1 (HTTP) and V2 (RNTBD) — the retry policy operates on the resolved `(status_code, sub_status)` pair before the transport-specific deserializer ever sees the body.

Beyond 449 and 404/1002, Gateway 2.0 follows the timeout/408 handling defined in `TRANSPORT_PIPELINE_SPEC.md` — no Gateway-2.0-specific override is introduced.

---

## 5. Rust Implementation Plan

### 5.1 Current Request Flow (Gateway V1)

1. `ContainerClient::create_item(partition_key, item, options)` calls into `ContainerClient`
2. `container_connection.rs` serializes `T` to `&[u8]`, computes EPK (via the SDK-side hash today, which does not handle MultiHash containers correctly), resolves PKRange
3. `CosmosDriver::execute_operation()` enters the Operation Pipeline (7-stage loop)
4. `resolve_endpoint()` selects a gateway endpoint
5. Transport Pipeline applies cosmos headers, signs request
6. HTTP/REST request sent to Cosmos Gateway (shared proxy, no SLA)

### 5.2 Target Request Flow (Gateway 2.0)

1. `ContainerClient::create_item(partition_key, item, options)` calls into `ContainerClient`
2. `container_connection.rs` serializes `T` to `&[u8]`; EPK computation is deferred to the driver (via `EffectivePartitionKey::compute()` / `::compute_range()`), which then resolves PKRange
3. `CosmosDriver::execute_operation()` enters the Operation Pipeline (7-stage loop)
4. `resolve_endpoint()` prefers gateway 2.0 endpoint when `!gateway20_suppressed` per §3.1
5. Transport Pipeline checks `is_operation_supported_by_gateway20()`:
   - **YES**: Inject gateway 2.0 headers + RNTBD serialize → HTTP/2 POST to Gateway 2.0 Proxy (SLA)
   - **NO**: Standard HTTP/REST request to Cosmos Gateway (eligibility fallback — per-request, deterministic)
6. Driver deserializes the RNTBD response (24-byte frame header → metadata token stream → optional body payload, per §Phase 1) into a domain `RntbdResponse`, then maps the body bytes to the typed result (`T`, `FeedResponse<T>`, etc.) before returning to the SDK. The SDK never sees the raw RNTBD bytes — that boundary stays in the driver, mirroring the EPK-pushdown decision in step 2.

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

Gateway 2.0 RNTBD has no version negotiation on the wire. The proxy advertises a single supported frame format per endpoint and rejects mismatched frames at the HTTP layer (the HTTP/2 request fails rather than triggering an RNTBD version-mismatch error). Direct-mode RNTBD has version negotiation (`CURRENT_PROTOCOL_VERSION = 0x00000001`); **do not** apply that pattern here.

#### Metadata token filtering (forward-compat contract)

The Rust deserializer **must** treat the RNTBD response metadata-token stream as forward-compatible:

- **Recognized response tokens** (mirror Java's `RntbdResponseHeader` set, finalized against Java source during implementation): request charge, session token, continuation token, activity-id echo, sub-status code, retry-after-milliseconds, LSN, partition-key-range-id, global-committed-lsn, item-lsn, transport-request-id, owner-id, and similar metadata. The exact token-ID enum is part of `rntbd/tokens.rs` (§"What Will Be Done").
- **Unknown token type IDs MUST be silently skipped** (consume `length` bytes and continue) — the deserializer must NOT panic, return an error, or fail the response, and must NOT log per-token (silent skip is the contract). The proxy is free to add new metadata tokens at any time and the driver must remain forward-compatible across proxy upgrades that ship before the corresponding Rust release. This silent-tolerance behavior is the *implementation* of the `IgnoreUnknownRntbdTokens` capability bit advertised over the `x-ms-cosmos-sdk-supportedcapabilities` header (see "SDK-supported-capabilities advertisement" below) — the proxy/backend assumes the SDK will not surface or warn on unknown tokens, so per-token logging is unnecessary noise.
- **Inverse contract on the request side**: the request serializer drops headers that appear in `thinClientProxyExcludedSet` (see §"RNTBD Request Wire Format" Notes column). That set enumerates headers the proxy does not understand on the inbound RNTBD frame; emitting them would be either ignored or rejected.

Phase 6's "RNTBD unknown-token tolerance" unit test pins this behavior: a hand-crafted response frame containing a synthetic unrecognized token ID must round-trip without error and surface every recognized token correctly.

#### SDK-supported-capabilities advertisement

The Rust SDK already wires the HTTP request header `x-ms-cosmos-sdk-supportedcapabilities` (`COSMOS_SDK_SUPPORTEDCAPABILITIES`, `azure_data_cosmos/src/constants.rs:157`) and emits it on every gateway request from `azure_data_cosmos_driver/src/driver/transport/cosmos_headers.rs:14-31`. Today the value sent over the wire is the literal string `"0"` — i.e., zero capabilities advertised.

Phase 1 must change the emitted value to the bitmask `(PartitionMerge | IgnoreUnknownRntbdTokens)`, matching the minimum capability set the .NET SDK asserts in its contract tests (`SDKSupportedCapabilities.cs`). The header value is a string-encoded decimal of the bitwise OR of the enum bits; the precise integer value should be looked up against `SDKSupportedCapabilities.cs` at implementation time and committed as a Rust constant alongside the existing `COSMOS_SDK_SUPPORTEDCAPABILITIES` header name.

The `IgnoreUnknownRntbdTokens` bit is the contract that backs the silent-skip behavior in "Metadata token filtering" above: the proxy/backend uses this advertisement to decide whether it is safe to add new RNTBD tokens without coordinating with this SDK release. Advertising the bit while *also* failing or warning on unknown tokens would be a contract violation; advertising `"0"` while silently skipping unknown tokens is "merely conservative" but causes the proxy to assume zero forward-compat tolerance — both are wrong. Phase 1 must reconcile both ends.

Phase 6 test coverage: assert the header value emitted on Gateway 2.0 (and standard Gateway) requests is the expected bitmask string, not `"0"`.

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

- **Operation filtering** — `is_operation_supported_by_gateway20(resource_type, operation_type) → bool`. Following Java (`ThinClientStoreModel`), only `ResourceType::Document` operations are eligible. All other resource types — including stored-procedure execution, which is **out of scope for Rust SDK GA** — fall through to the standard gateway via the eligibility-fallback path.
- **EPK computation** — Call `EffectivePartitionKey::compute()` (point) or `::compute_range()` (feed/cross-partition) from the driver layer. Do **not** call `azure_data_cosmos::hash::get_hashed_partition_key_string` (§3.5). SDK call sites that currently use it must route through the driver's implementation as part of this phase.
- **EPK error propagation** — If EPK computation returns `Err` (MultiHash-requires-V2, component-count mismatch, etc.), surface as `CosmosStatus::BadRequest` to the caller. **Do not** fall back to standard gateway — the same inputs would be equally broken there.
- **Header injection** — When `transport_mode == Gateway20`, inject the Gateway 2.0 headers listed below.
- **Request body wrapping** — Serialize the entire request (headers + body) into RNTBD binary format and POST as the HTTP/2 body.
- **Response unwrapping** — Deserialize the RNTBD response body back into `CosmosResponseHeaders` + raw document bytes.
- **Eligibility fallback** — Operation ineligible for Gateway 2.0 → route through standard gateway for this single request (per-request, deterministic). See §Phase 4 for the distinct failure-driven fallback.
- **Constants placement & naming** — Relocate the existing `THINCLIENT_PROXY_*` constants into `azure_data_cosmos_driver::constants` and **rename them to the `GATEWAY20_*` family** as part of Phase 2 (the wire header strings stay the same — they are server-defined and currently `x-ms-thinclient-proxy-*` — only the Rust identifier changes). Concretely: `THINCLIENT_PROXY_OPERATION_TYPE` → `GATEWAY20_OPERATION_TYPE`, `THINCLIENT_PROXY_RESOURCE_TYPE` → `GATEWAY20_RESOURCE_TYPE`, and any new EPK-range / capability constants follow the same `GATEWAY20_*` prefix. **No SDK re-export** — the SDK has no Gateway-2.0 awareness; it invokes the generic `CosmosDriver::execute_operation` interface and the driver decides Gateway 2.0 vs Gateway V1 internally.

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
| StoredProcedure Execute | **No** | Stored-procedure execution is out of scope for Rust SDK GA. Eligibility fallback routes any incoming SPROC request to the standard gateway. |
| All other resource types | **No** | Metadata operations use standard gateway |

#### Header naming (proxy headers, in HTTP/2 request headers — not RNTBD tokens)

These are wire-level HTTP/2 request headers on the outer POST to the proxy. They are **not** inside the RNTBD metadata token stream.

| Header (wire) | Rust constant (crate) | Semantics | When emitted |
| --- | --- | --- | --- |
| `x-ms-thinclient-proxy-operation-type` | `GATEWAY20_OPERATION_TYPE` (driver) | Numeric operation type | Every Gateway 2.0 request |
| `x-ms-thinclient-proxy-resource-type` | `GATEWAY20_RESOURCE_TYPE` (driver) | Numeric resource type | Every Gateway 2.0 request |
| `x-ms-effective-partition-key` | **NEW** — `EFFECTIVE_PARTITION_KEY` (driver) | Canonical EPK hex | Point ops only |
| `x-ms-documentdb-partitionkey` | existing `PARTITION_KEY` constant (SDK) | JSON-encoded partition-key value | Point ops AND single-logical-partition query ops, alongside `x-ms-effective-partition-key` |
| `x-ms-thinclient-range-min` | **NEW** — `GATEWAY20_RANGE_MIN` (driver) | Lower bound of EPK range | Feed / cross-partition ops only |
| `x-ms-thinclient-range-max` | **NEW** — `GATEWAY20_RANGE_MAX` (driver) | Upper bound of EPK range | Feed / cross-partition ops only |
| `x-ms-cosmos-use-thinclient` | **NEW** — `GATEWAY20_USE_THINCLIENT` (driver) | Instructs account-metadata response to advertise thin-client endpoints | Account metadata fetches only |

> Wire-header strings (`x-ms-thinclient-*`) are server-defined and unchanged; the Rust-side identifiers use the `GATEWAY20_*` prefix.

Per Q3 resolution, the Gateway 2.0 proxy requires the Java header names `x-ms-thinclient-range-min` / `x-ms-thinclient-range-max` (it does **not** accept `x-ms-start-epk` / `x-ms-end-epk`). Phase 2 introduces the new constants above; the existing `START_EPK` / `END_EPK` constants are not emitted on the Gateway 2.0 path.

**Tenant identification (RNTBD token, not HTTP header)**: the proxy identifies the target Cosmos account from the existing RNTBD `GlobalDatabaseAccountName` token (`0x00CE`, `String`, optional) carried inside the RNTBD metadata stream on **every** Gateway 2.0 request. No Gateway-2.0-specific HTTP headers are introduced for account or regional-account identification — the RNTBD token is the canonical carrier and matches the Java/.NET wire contract. The value is the global database account name (e.g., `myacct` from `myacct.documents.azure.com`), parsed once from the account endpoint URL at client construction.

#### Consistency header reconciliation (`ConsistencyLevel` ↔ `ReadConsistencyStrategy`)

The Cosmos SDK exposes two consistency knobs that can both target the same read operation:

- **`ConsistencyLevel`** — per-request override of the account default consistency.
- **`ReadConsistencyStrategy`** (defined in `azure_data_cosmos_driver::options::read_consistency`) — read-only strategy override (`Default`, `Eventual`, `Session`, `LatestCommitted`, `GlobalStrong`); supersedes `ConsistencyLevel` on reads.

This subsection is the Rust mirror of the cross-SDK design landed in [Java PR #48787](https://github.com/Azure/azure-sdk-for-java/pull/48787) (with .NET parity in PR #5685 and matching proxy-side changes). Wire-format and resolution semantics MUST match Java/.NET so that a single proxy-side validation suite is sufficient.

##### Wire carriers

| Transport | Wire carrier for the resolved value | Encoding |
| --- | --- | --- |
| Standard Gateway (V1, HTTP) | HTTP request header `x-ms-cosmos-read-consistency-strategy` (per Java `HttpConstants.READ_CONSISTENCY_STRATEGY`) | String, exact case-sensitive values: `"Eventual"`, `"Session"`, `"LatestCommitted"`, `"GlobalStrong"`. Header is omitted entirely when the resolved RCS is `Default`. |
| Gateway 2.0 (RNTBD) | RNTBD metadata token ID `0x00F0` | **Byte** type — `Eventual = 0x01`, `Session = 0x02`, `LatestCommitted = 0x03`, `GlobalStrong = 0x04`. The token MUST be Byte-encoded; per the Java PR an earlier String-typed prototype caused the proxy to hang. The token is omitted entirely when the resolved RCS is `Default`. |

The byte values are pinned against the proxy's C++ enum. Phase 1's RNTBD token catalog grows a row for `ReadConsistencyStrategy = 0x00F0 (Byte)` enumerating the four byte values.

##### Resolution precedence

A single resolution step runs in the driver pipeline (alongside the existing `is_session_effective` computation in `operation_pipeline.rs`) **before** transport selection. It produces exactly one resolved consistency value, which is then handed off to whichever transport (V1 HTTP or V2 RNTBD) carries it on the wire.

Sources, highest precedence first:

1. Request-level `ReadConsistencyStrategy` (read ops only)
2. Request-level `ConsistencyLevel`
3. Client-level `ReadConsistencyStrategy` (read ops only)
4. Client-level `ConsistencyLevel`
5. Account default consistency (no header / no token emitted; backend applies its default)

`ReadConsistencyStrategy::Default` at any level is a pass-through — falls through to the next source. Write operations skip steps 1 and 3 entirely (RCS is read-only); writes resolve from steps 2/4/5.

##### Dual-header rejection rule

The compute gateway rejects requests that carry both `x-ms-consistency-level` AND `x-ms-cosmos-read-consistency-strategy`. The Rust driver MUST therefore enforce mutual exclusion on both transports:

- **V1 HTTP**: when resolved RCS is non-Default, the driver sends only `x-ms-cosmos-read-consistency-strategy` and **strips** any `x-ms-consistency-level` from the outgoing header set. When resolved RCS is Default, the driver sends only `x-ms-consistency-level` (if a `ConsistencyLevel` was resolved at any level) and omits the RCS header.
- **V2 RNTBD**: same mutual exclusion applied to the RNTBD metadata stream — emit either the `ConsistencyLevel` token or the `ReadConsistencyStrategy` token (`0x00F0`), never both. The Gateway 2.0 RNTBD serializer consumes the **already-resolved** value and decides which of the two tokens to emit; it does not re-run resolution.

##### GlobalStrong client-side validation

When the resolved RCS is `GlobalStrong` and the account default consistency is **not** `Strong`, the driver MUST fail the operation **before** transport selection / serialization with a `BadRequestException`-equivalent (Rust: `azure_core::Error` with the appropriate `ErrorKind`). This avoids a wasted round-trip and matches Java's fail-fast semantics. The check uses the cached account properties already maintained by the driver; no additional metadata fetch is required.

##### Implementation pitfall (Java bug class to avoid)

Resolution MUST NOT mutate the request's header map in place. The Java fix in `RxGatewayStoreModel.applySessionToken()` switched to a header-map copy because the prior code's mutation rewrote `x-ms-consistency-level` (e.g., `LatestCommitted` was rewritten to `BoundedStaleness`); the gateway then rejected the request because `BoundedStaleness` was stricter than the Session account default. Even though the underlying conflict was real, the diagnostic was unrecoverable because the original headers had already been clobbered.

For Rust: thread the resolved consistency value through the pipeline as an explicit input to whichever transport handler runs next. Do not write back into the operation's header collection during resolution. If the operation's header collection is needed for the final serialize step, clone it first or pass the resolved value separately.

#### Range header wire format

EPK range headers (`x-ms-thinclient-range-min` / `-max`) carry the canonical, un-padded hex produced by `EffectivePartitionKey::compute_range()`. **Do not** zero-pad to N×32 on the wire. Local comparisons use `EffectivePartitionKey`'s `Ord` / `cmp` impl, which correctly handles the mixed-length boundaries returned by the backend; the `epk_cmp_*` tests in `container_routing_map.rs` (around L625–665) pin this behavior. The comparator is consumed via `binary_search_by(|r| r.min_inclusive.cmp(&epk_val))` (≈L282 of the same file). An earlier zero-padding proposal in PR #4087 (commit `25233c903`) was **not** adopted; stay consistent with the length-aware convention.

> **`Range` semantics pitfall** (from PR #4087): `compute_range` returns a Rust `std::ops::Range<EffectivePartitionKey>` where `start == end` denotes a **point operation**. Standard `Range` iteration treats that as empty, so code that uses `.contains()` or iterates the range directly will misbehave. Always treat `start == end` as the point case explicitly.

#### Gateway 2.0 Header Injection Flow

When `transport_mode == Gateway20`:

1. Set `x-ms-thinclient-proxy-operation-type` (numeric operation type)
2. Set `x-ms-thinclient-proxy-resource-type` (numeric resource type)
3. Serialize the `GlobalDatabaseAccountName` RNTBD metadata token (`0x00CE`, `String`, optional) with the account host label (e.g., `myacct`) — every request, see "Tenant identification" note above
4. Point op or single-logical-partition query op? Set `x-ms-effective-partition-key` (EPK hash from `EffectivePartitionKey::compute()`).
   Cross-partition feed / query operation? Set `x-ms-thinclient-range-min` and `x-ms-thinclient-range-max` (from `EffectivePartitionKey::compute_range()`); do **not** emit the PK header on the feed path.
5. Serialize the **already-reconciled** consistency value (per "Consistency header reconciliation" above) into the appropriate RNTBD metadata token: `ConsistencyLevel` token if RCS resolved to `Default`, OR the `ReadConsistencyStrategy` token (`0x00F0`, Byte) if RCS resolved to a non-Default value. Emit exactly one of the two — never both. The serializer consumes the resolved value as input; do not re-run resolution here.
6. Serialize headers + body into RNTBD binary format (Phase 1)
7. POST RNTBD body to gateway 2.0 endpoint via HTTP/2

When `transport_mode != Gateway20`: Standard HTTP/REST request (existing flow, unchanged).

#### Files Changed

```
NEW   src/driver/transport/gateway20.rs          — inject_gateway20_headers(), RNTBD wrap/unwrap
EDIT  src/driver/transport/transport_pipeline.rs  — Branch on TransportMode in execute_transport_pipeline()
EDIT  src/driver/transport/cosmos_headers.rs      — Add gateway 2.0 header application
EDIT  src/driver/transport/mod.rs                 — Add is_operation_supported_by_gateway20()
EDIT  src/driver/pipeline/components.rs           — Add EPK fields to TransportRequest if needed
EDIT  src/driver/constants.rs (or NEW)            — Relocate + rename THINCLIENT_PROXY_* → GATEWAY20_* constants from azure_data_cosmos to azure_data_cosmos_driver (no SDK re-export)
EDIT  sdk/cosmos/azure_data_cosmos/src/...        — Replace SDK-side get_hashed_partition_key_string callers with driver's EffectivePartitionKey::compute()
```

---

### Phase 3: Endpoint Discovery — verification & one new header

**Crate**: `azure_data_cosmos_driver`

> Most of Phase 3 is **audit / verification** against scaffolding already in place. Only the `x-ms-cosmos-use-thinclient` request header is net-new code. Noted here because the dependency graph lists Phase 3 as a prerequisite for Phase 2; in practice the verification items can happen in parallel with Phase 1 and the one real code change can ride with Phase 2 if convenient.

#### What Will Be Done

- **Verify** account metadata cache parses `thinClientReadableLocations` / `thinClientWritableLocations` into `CosmosEndpoint::gateway20_url`
- **Confirm** `build_account_endpoint_state()` constructs `CosmosEndpoint::regional_with_gateway20()` correctly in multi-region accounts (existing tests at `routing_systems.rs:218–289` already cover this)
- **Verify** `AccountProperties::has_thin_client_endpoints()` is used as the gating signal per §3.1
- **Add** `x-ms-cosmos-use-thinclient` request header on account metadata fetches (new code)
- **Test** endpoint discovery with live account that has gateway 2.0 enabled (handled by Phase 6 live pipeline)

#### Region pairing (lock in the §PR #3942 decision)

Gateway 2.0 read locations pair **only** with read regions; Gateway 2.0 write locations pair **only** with write regions. A write region that advertises no Gateway 2.0 URL falls back to standard gateway **for writes** (this was deliberate in PR #3942: session retries that reroute reads to write endpoints would otherwise cross the read/write Gateway 2.0 split). This is a correctness invariant — do not "fix" it by cross-pairing.

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

Gateway 2.0 is **on by default** when the account metadata advertises Gateway 2.0 endpoints; users opt **out** (not in) via `CosmosClientOptions::gateway20_disabled` if they need to. This matches the design philosophy of both Java and .NET SDKs and minimizes friction for the common case.

#### What Will Be Done

- **Auto-detection** — When account metadata includes `thinClientReadableLocations` / `thinClientWritableLocations`, the driver automatically prefers gateway 2.0 for eligible operations (per §3.1). No user opt-in required.
- **Operator override** — `CosmosClientOptions::gateway20_disabled` (default `false`) is a public, documented setting for forcing Gateway V1 routing per-client. **It carries an explicit warning that flipping it voids Gateway 2.0's latency SLA and impacts 24/7 Microsoft support eligibility for performance regressions.** Intentionally not exposed via env var. See §3.2 for the full normative wording. There is no positive-term internal flag; `gateway20_disabled` is the single supported disablement mechanism.
- **Diagnostics** — `CosmosDiagnostics` should report when a request used gateway 2.0 vs standard gateway (already partially done via `TransportKind::Gateway20`).
- **User agent** — Update SDK user agent string to indicate gateway 2.0 capability.
- **EPK cutover** — Replace SDK-side callers of `get_hashed_partition_key_string` with calls into the driver's `EffectivePartitionKey::compute()` / `::compute_range()` (this is the cutover PR #4087 flagged). Gateway 2.0 header injection depends on this being correct for hierarchical-PK containers.

#### Auto-Detection Flow

When account metadata includes `thinClientReadableLocations`, gateway 2.0 is enabled automatically (internal). `CosmosEndpoint` gets `gateway20_url` and `resolve_endpoint()` prefers Gateway 2.0 (per §3.1's single-source-of-truth rule). No user configuration needed — transparent to the caller.

#### Files Changed

```
EDIT  src/driver_bridge.rs                        — Ensure internal config passes through
EDIT  src/handler/container_connection.rs         — Route EPK through driver's EffectivePartitionKey::compute()
EDIT  src/partition_key.rs                        — Update feed_range_from_partition_key call site
EDIT  src/constants.rs                            — Remove THINCLIENT_PROXY_* constants (relocated + renamed to GATEWAY20_* in driver crate, no SDK re-export)
```

---

### Phase 6: Testing

Testing covers all layers from unit to E2E, matching or exceeding Java/.NET test coverage.

#### Live Tests Pipeline

A **new dedicated CI pipeline** is required for gateway 2.0 live tests. Gateway 2.0 requires a Cosmos DB account with Gateway 2.0 endpoints enabled, which is separate from the standard emulator and live test infrastructure.

**Trigger:** PR changes to `sdk/cosmos/**` + manual dispatch

**Provision:**

- Use a **dedicated, pre-provisioned Cosmos DB account** with Gateway 2.0 endpoints enabled (hardcoded for this pipeline, reused across runs)
- Account credentials stored in pipeline secrets (e.g., `AZURE_COSMOS_GW20_ENDPOINT`, `AZURE_COSMOS_GW20_KEY`)
- Multi-region configuration (at least 2 regions)
- Verify `thinClientReadableLocations` in account metadata at pipeline start

**Test Matrix:**

- Single-region gateway 2.0
- Multi-region gateway 2.0 with failover
- Gateway 2.0 + standard gateway eligibility fallback (per-request only; normal retries still apply)
- Operator override (`CosmosClientOptions::gateway20_disabled = true`) — assert all eligible Document ops route through the standard gateway

**Test Suites:**

- Point CRUD (create, read, replace, upsert, patch, delete)
- Query (single-partition, cross-partition)
- Batch operations
- Change feed (LatestVersion)
- Retry scenarios (408, 410, 449, 503, 404/1002)
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
| RNTBD unknown-token tolerance | Yes | | | Inject synthetic unknown token IDs into a response frame; deserializer must skip + log, never panic / error / drop the rest of the response |
| EPK computation | Yes | | | Single/hierarchical PK, hash versions 1 and 2, error cases (MultiHash V1, wrong component count) |
| Operation filtering | Yes | | | All ResourceType × OperationType combos; asserts StoredProc Execute is rejected |
| Header injection | Yes | | | Point vs feed EPK headers, proxy type headers, range-header un-padded form |
| HPK + Gateway 2.0: full vs partial PK | Yes | | Yes | Hierarchical container (2- and 3-component PK paths). **Full PK** (all components specified) on a point op → emits `x-ms-effective-partition-key` carrying the single EPK from `EffectivePartitionKey::compute()`. **Partial PK** (1- or 2-component prefix) on a feed / cross-partition / delete-by-PK op → emits `x-ms-thinclient-range-min` / `x-ms-thinclient-range-max` carrying the EPK range from `EffectivePartitionKey::compute_range()`. Asserted at unit level (header presence + exact wire form, range bounds for each prefix length) and E2E (round-trip against a live HPK container). |
| Account-name RNTBD token | Yes | | | `GlobalDatabaseAccountName` (`0x00CE`, `String`) present in the RNTBD metadata stream of every Gateway 2.0 request (point, feed, batch, bulk, change feed). Value matches the host label of the account endpoint URL. |
| SDK-supported-capabilities header | Yes | | | `x-ms-cosmos-sdk-supportedcapabilities` value emitted is the bitmask string for `(PartitionMerge \| IgnoreUnknownRntbdTokens)`, **not** `"0"`. Pin against the integer value sourced from .NET `SDKSupportedCapabilities.cs`. |
| Consistency reconciliation: token + header encoding | Yes | | | RNTBD token `0x00F0` Byte round-trip for all 4 strategies; HTTP header `x-ms-cosmos-read-consistency-strategy` exact wire-string mapping for all 4 strategies; `Default` emits neither carrier on either transport. |
| Consistency reconciliation: dual-header rejection | Yes | | | SDK never emits both `x-ms-consistency-level` AND `x-ms-cosmos-read-consistency-strategy` on V1; never emits both `ConsistencyLevel` and `ReadConsistencyStrategy` RNTBD tokens on V2. Verified across all 16 (CL × RCS, request-level × client-level) combinations. |
| Consistency reconciliation: 4-source precedence | Yes | | | Request-RCS > Request-CL > Client-RCS > Client-CL > account default; `Default` at any RCS layer is a pass-through. Representative subset matching Java's data-provider tests. |
| Consistency reconciliation: GlobalStrong validation | Yes | | | RCS=GlobalStrong on a non-Strong account produces a fail-fast `azure_core::Error` (no wire request emitted); on a Strong account the request proceeds normally. |
| Consistency reconciliation: header-map immutability | Yes | | | Resolution does not mutate the operation's original request headers; an `applySessionToken`-equivalent rewrite cannot clobber `x-ms-consistency-level`. |
| Consistency reconciliation: write-op behavior | Yes | | | Write op + RCS set → RCS is ignored, `ConsistencyLevel` (if any) flows through on the selected transport. |
| Gateway 2.0 transport | Yes | Yes | | Correct HTTP/2 config, sharded pool selection |
| Read/write pairing | Yes | | | Write region without Gateway 2.0 URL falls back for writes only |
| Point CRUD | | | Yes | Create, read, replace, upsert, patch, delete |
| Query | | | Yes | SQL query, cross-partition |
| Batch | | | Yes | Transactional batch ops |
| Bulk | | | Yes | Fan-out CRUD, distinct from Batch |
| Change feed | | | Yes | LatestVersion, incremental |
| Retry: 408 timeout | | Yes | | Cross-region for reads, local-only for writes |
| Retry: 449 Retry-With | | Yes | | Dedicated 449 policy (≤ 3 attempts, exponential backoff, separate budget from 410/Gone), same Gateway 2.0 endpoint, no region switch, no fallback to Gateway V1 |
| Retry: 503 | | Yes | | Regional failover via existing retry policies |
| Retry: 410 Gone | | Yes | | PKRange refresh (sub-status specific); NameCacheStale → collection cache |
| Retry: 404 / sub-status 1002 (ReadSessionNotAvailable) | | Yes | | Retry routes to a **remote-preferred** region (assert local-region retry only when no other region is available); assert PLF region wins when PLF has pinned the PKRangeId; assert that **no PKRange cache refresh** is triggered |
| Operator override (`gateway20_disabled = true`) | Yes | Yes | | All eligible Document ops (point + feed + batch + change feed) route through standard gateway; default `false` does not change behavior |
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

## 6. Open Questions

- **Q1 — HTTP/2 prior knowledge vs ALPN**: _Resolved_. Gateway 2.0 always uses HTTP/2; the proxy does not accept HTTP/1.x. Rust uses HTTP/2 with prior knowledge on the Gateway 2.0 transport (no ALPN fallback to HTTP/1.x). The broader ALPN default in `TRANSPORT_PIPELINE_SPEC.md` does **not** apply to Gateway 2.0; if HTTP/2 negotiation fails, the request fails and the existing retry policies handle it.
- **Q2 — Live test account provisioning**: Cosmos DB account configuration flags required to enable Gateway 2.0 endpoints are not part of the standard Bicep templates. _Resolution_: hardcode a dedicated, pre-provisioned Gateway 2.0 account for the gateway 2.0 live tests pipeline and reuse it across runs (rather than provisioning per-run via Bicep). Account name and credentials stored in pipeline secrets (`AZURE_COSMOS_GW20_ENDPOINT`, `AZURE_COSMOS_GW20_KEY`); pipeline reads endpoint from environment variables.
- **Q3 — EPK range header names**: _Resolved_. The Gateway 2.0 proxy requires the Java header names `x-ms-thinclient-range-min` / `x-ms-thinclient-range-max`. Phase 2 introduces new constants (`GATEWAY20_RANGE_MIN`, `GATEWAY20_RANGE_MAX`) on the Gateway 2.0 path; the existing `START_EPK` / `END_EPK` (`x-ms-start-epk` / `x-ms-end-epk`) constants remain for any non-Gateway-2.0 callers but are **not** emitted on Gateway 2.0 requests.
