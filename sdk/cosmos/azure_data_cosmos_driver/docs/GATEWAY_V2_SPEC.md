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
   - 4.1 [HTTP 449 (Retry-With)](#41-http-449-retry-with--dedicated-policy-separate-from-410gone)
   - 4.2 [HTTP 404/1002 (READ_SESSION_NOT_AVAILABLE)](#42-http-404-not-found-with-sub-status-1002-read_session_not_available)
   - 4.3 [Fail-fast on Gateway 2.0 transport failures](#43-fail-fast-on-gateway-20-transport-failures)
5. [Behavior Reference](#5-behavior-reference)
6. [Open Questions](#6-open-questions)

### Related Specs

- [`TRANSPORT_PIPELINE_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/TRANSPORT_PIPELINE_SPEC.md) — sharded HTTP/2 transport, timeout regime, hedging, `(HttpClient, host:port)` shard key. Gateway 2.0 reuses the sharded transport defined there verbatim; this spec does **not** introduce a new timeout or hedging policy.
- [`PARTITION_KEY_RANGE_CACHE_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/PARTITION_KEY_RANGE_CACHE_SPEC.md) — PKRange cache semantics and `EffectivePartitionKey` usage (EPK computation in §5.4, 410 handling in §5.6).
- [`PARTITION_LEVEL_FAILOVER_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/docs/PARTITION_LEVEL_FAILOVER_SPEC.md) — per-partition region override semantics (PLF precedence over Gateway 2.0 routing in §5.6).

---

## 1. Overview

Gateway 2.0 (formerly "thin client") is a server-side proxy that evolves the existing Gateway V1 path: SDK clients still target a regional proxy endpoint instead of opening connections to backend replicas, but the proxy uses RNTBD binary message encoding over HTTP/2 in place of REST/JSON, and the proxy itself owns replica selection, connection management, and load balancing within a partition.

**RNTBD** stands for "Real Name to be Determined" — the (intentionally placeholder) code name for the proprietary message-encoding and wire-protocol format originally introduced for direct mode. Gateway 2.0 keeps the **message encoding** layer of RNTBD and moves the **wire protocol** to HTTP/2; direct mode used RNTBD over TCP.

Direct mode was never in scope for the Rust SDK, so the rest of this spec compares Gateway 2.0 to **Gateway V1**, not to direct mode.

**Naming**: Use "Gateway 2.0" consistently in all Rust code, docs, and comments. Reserve "thin client" only for the literal wire-header names that contain `thinclient` (e.g. `x-ms-thinclient-proxy-operation-type`).

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

## 3. Gating & Configuration

Gateway 2.0 routing is decided **once per logical operation** (a point operation, a single query-iteration page, a single batch, etc.) in the driver's `resolve_endpoint` stage. The resulting `RoutingDecision.transport_mode` is then attached to the operation context and **inherited by all retries and sub-requests** of that operation — downstream pipeline stages, retry policies, and hedged sub-requests MUST trust the attached decision and not re-derive eligibility. (Re-evaluating `gateway_v2_suppressed` mid-operation could route a retry through a different transport than the original attempt, fragmenting diagnostics and breaking session-token affinity.)

### 3.1 The `gateway_v2_suppressed` formula

```text
gateway_v2_suppressed = !http2_available
                    || !account.has_gateway_v2_endpoints()
```

When `gateway_v2_suppressed` is `false` — the default whenever the account advertises Gateway 2.0 endpoints, the connectivity probe has confirmed them, and HTTP/2 is available — the request routes through Gateway 2.0. When it is `true`, the request falls through to Gateway V1. Both inputs are **server- and environment-driven, not customer-configurable**: `has_gateway_v2_endpoints()` reads the cached account metadata (which only carries Gateway 2.0 endpoints after the connectivity probe succeeds for every advertised region), and `http2_available` reflects whether the transport could negotiate HTTP/2 (the one hard client-side prerequisite). There is no client API or environment variable to force Gateway 2.0 on or off.

### 3.2 Server-driven selection — no client toggle

Gateway 2.0 vs. Gateway V1 is selected entirely by the service: the account advertises thin-client (`thinClient*Locations`) endpoints, and the driver confirms each advertised region with a one-time connectivity probe before routing data-plane traffic to it. There is **no customer- or operator-facing switch** — no `CosmosClientOptions` field and no `AZURE_COSMOS_*` environment variable — to opt a client in or out. Keeping the choice abstract from customers lets the service migrate accounts between Gateway V1 and Gateway 2.0 transparently.

The only client-side prerequisite is **HTTP/2**: if the runtime cannot negotiate HTTP/2, Gateway 2.0 is suppressed for that client and all traffic stays on Gateway V1.

All internal flags **must use a negative-term name** (`gateway_v2_disabled`, `gateway_v2_suppressed`, etc.) so that **default values mean Gateway 2.0 is enabled**. Positive-term names (`is_gateway_v2_allowed`, `gateway_v2_allowed`, `enable_gateway_v2`, etc.) are not permitted anywhere — driver, SDK, perf crate, or test wiring. These flags are internal implementation details derived from HTTP/2 availability; there is no public option or `AZURE_COSMOS_*` env var that toggles Gateway 2.0.

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

### 4.3 Fail-fast on Gateway 2.0 transport failures

#### 4.3.1 Problem

Gateway 2.0 endpoints are advertised by the account on a different host (and may be advertised on a non-443 port — the test fixture in `routing_systems.rs` uses `:444`). Some enterprise networks block outbound TCP to non-443 ports. In those environments **every** Gateway 2.0 attempt from the client will fail at the transport layer (TCP refused, TCP timeout, TLS handshake failure, or HTTP/2 negotiation failure) while Gateway V1 on the *standard* gateway host:443 still works.

Today the `endpoint_is_available()` check in `operation_pipeline.rs` skips a *single* G2 endpoint after we mark it `UnavailableReason::TransportError`, but the retry then immediately picks the *next* G2 endpoint. In a blanket-firewall scenario all G2 endpoints have the same outcome, so the operation exhausts its retry budget on G2 and fails — even though G1 would have succeeded immediately. Today the customer sees a generic transport error with no clear remediation.

#### 4.3.2 Decision — fail-fast

Keep the regional-retry behavior. A single attempt against an unreachable G2 endpoint fails, the endpoint is marked `TransportError`, the retry tries the next region's G2 endpoint, and the operation eventually fails with a transport error if all G2 regions are unreachable. The error and diagnostics surface a clear, actionable hint that the Gateway 2.0 endpoints are unreachable. Because transport selection is server-driven, the durable remediation is automatic: the connectivity probe gates Gateway 2.0 off for unreachable regions on the next refresh, and/or the account stops advertising thin-client endpoints — either way the client converges to G1 with no code change.

Why fail-fast was chosen:

- **Predictable, uniform behavior.** No automatic G2 → G1 fallback keeps the regional retry stack identical across transports. Diverging would increase the support matrix and cause customer confusion.
- **No silent latency surprise.** Auto-switching transport modes mid-workload would change the latency profile in ways the customer did not ask for. Customers tuning their workload around the selected transport would see a hidden regression they cannot attribute.
- **Operational guarantees stay honest.** The guarantees published for the selected transport are tied to that transport staying selected for the duration of the workload. Auto-degrading would silently violate that contract for the affected client.
- **No hidden state.** Firewall mis-configuration is exactly the kind of infrastructure problem that should bubble up loudly so the customer can react. A circuit breaker would mask it.
- **Simplicity.** No new state, no concurrency contract, no recovery logic in the data path. Customers behind firewalls get a single deterministic verdict and an automatic, server-driven remediation: the connectivity probe gates Gateway 2.0 off for unreachable regions, and once the account stops advertising thin-client endpoints (`getDatabaseAccount` no longer returns `thinClient*Locations`) the driver drops the G2 endpoints on the next metadata refresh and routes through G1 — all with no client change.

---

## 5. Behavior Reference

### 5.1 Current Request Flow (Gateway V1)

1. `ContainerClient::create_item(partition_key, item, options)` calls into `ContainerClient`
2. `container_connection.rs` serializes `T` to `&[u8]`, computes EPK (via the SDK-side hash today, which does not handle MultiHash containers correctly), resolves PKRange
3. `CosmosDriver::execute_operation()` enters the Operation Pipeline (7-stage loop)
4. `resolve_endpoint()` selects a gateway endpoint
5. Transport Pipeline applies cosmos headers, signs request
6. HTTP/REST request sent to Cosmos Gateway (shared proxy, no SLA)

### 5.2 Gateway 2.0 Request Flow

1. `ContainerClient::create_item(partition_key, item, options)` calls into `ContainerClient`
2. `container_connection.rs` serializes `T` to `&[u8]`; EPK computation is deferred to the driver (via `EffectivePartitionKey::compute()` / `::compute_range()`), which then resolves PKRange
3. `CosmosDriver::execute_operation()` enters the Operation Pipeline (7-stage loop)
4. `resolve_endpoint()` prefers gateway 2.0 endpoint when `!gateway_v2_suppressed` per §3.1
5. Transport Pipeline checks `is_operation_supported_by_gateway_v2()`:
   - **YES**: Inject gateway 2.0 headers + RNTBD serialize → HTTP/2 POST to Gateway 2.0 Proxy (SLA)
   - **NO**: Standard HTTP/REST request to Cosmos Gateway (eligibility fallback — per-request, deterministic)
6. Driver deserializes the RNTBD response (24-byte frame header → metadata token stream → optional body payload, per §5.3) into a domain `RntbdResponse`, then maps the body bytes to the typed result (`T`, `FeedResponse<T>`, etc.) before returning to the SDK. The SDK never sees the raw RNTBD bytes — that boundary stays in the driver, mirroring the EPK-pushdown decision in step 2.

> **Naming**: The function is `is_operation_supported_by_gateway_v2()` throughout. Older drafts used `is_supported_by_gw_v2()` — do not reintroduce the abbreviation.

---

### 5.3 RNTBD Protocol

The RNTBD protocol lives in `src/driver/transport/rntbd/` (driver crate).

The RNTBD ("Real Name To Be Determined" — a placeholder name that stuck) protocol is the wire format used by Cosmos DB for efficient binary communication. Gateway 2.0 wraps RNTBD-encoded payloads inside HTTP/2 POST requests to the proxy.

#### Module layout

- **`rntbd/mod.rs`** — Module root, public types
- **`rntbd/request.rs`** — Request serialization: operation headers, resource metadata, partition key info → binary payload
- **`rntbd/response.rs`** — Response deserialization: 24-byte frame header → metadata section → optional body payload
- **`rntbd/tokens.rs`** — RNTBD token types (type IDs, lengths, value encodings) used in metadata sections
- **`rntbd/status.rs`** — RNTBD status code mapping to `CosmosStatus`

#### Versioning

Gateway 2.0 RNTBD has no version negotiation on the wire. The proxy advertises a single supported frame format per endpoint and rejects mismatched frames at the HTTP layer (the HTTP/2 request fails rather than triggering an RNTBD version-mismatch error). Direct-mode RNTBD has version negotiation (`CURRENT_PROTOCOL_VERSION = 0x00000001`); **do not** apply that pattern here.

#### Metadata token filtering (forward-compat contract)

The Rust deserializer **must** treat the RNTBD response metadata-token stream as forward-compatible:

- **Recognized response tokens**: request charge, session token, continuation token, activity-id echo, sub-status code, retry-after-milliseconds, LSN, partition-key-range-id, global-committed-lsn, item-lsn, transport-request-id, owner-id, and similar metadata. The exact token-ID enum is part of `rntbd/tokens.rs` (§module layout).
- **Unknown token type IDs MUST be silently skipped** (consume `length` bytes and continue) — the deserializer must NOT panic, return an error, or fail the response, and must NOT log per-token (silent skip is the contract). The proxy is free to add new metadata tokens at any time and the driver must remain forward-compatible across proxy upgrades that ship before the corresponding Rust release. This silent-tolerance behavior is the *implementation* of the `IgnoreUnknownRntbdTokens` capability bit advertised over the `x-ms-cosmos-sdk-supportedcapabilities` header (see "SDK-supported-capabilities advertisement" below) — the proxy/backend assumes the SDK will not surface or warn on unknown tokens, so per-token logging is unnecessary noise.
- **Inverse contract on the request side**: the request serializer drops headers that appear in `thinClientProxyExcludedSet` (see §"RNTBD Request Wire Format" Notes column). That set enumerates headers the proxy does not understand on the inbound RNTBD frame; emitting them would be either ignored or rejected.

##### Continuation-token format (request and response)

Continuation tokens are **opaque server-issued strings** in both directions; the SDK never parses, validates, or rewrites them. The wire format is a length-prefixed UTF-8 string token:

- **Request side** — `RntbdRequestToken::ContinuationToken` (ID `0x0006`, `TokenType::String`). When the inbound HTTP request carries `x-ms-continuation`, the wrap path serializes the value verbatim into the RNTBD metadata stream and **strips** the header from the outer HTTP request (the outer body is the RNTBD frame; metadata never duplicates onto outer headers). Empty values are passed through as zero-length string tokens — the wrap path does not infer intent from emptiness, matching the unwrap side.
- **Response side** — `RntbdResponseToken::ContinuationToken` (ID `0x0003`, `TokenType::String`). The unwrap path forwards the token value verbatim into the synthetic HTTP response's `x-ms-continuation` header.

`ContinuationToken` is *not* in `thinClientProxyExcludedSet`, so it traverses the same encode/decode path as standard direct-mode RNTBD. There is no Gateway-2.0-specific token format, base64 wrapper, or version prefix; pagination cursors round-trip byte-for-byte.

A unit test pins this behavior: a hand-crafted response frame containing a synthetic unrecognized token ID must round-trip without error and surface every recognized token correctly.

#### SDK-supported-capabilities advertisement

The Rust SDK already wires the HTTP request header `x-ms-cosmos-sdk-supportedcapabilities` (`COSMOS_SDK_SUPPORTEDCAPABILITIES`, `azure_data_cosmos/src/constants.rs:157`) and emits it on every gateway request from `azure_data_cosmos_driver/src/driver/transport/cosmos_headers.rs:14-31`. Today the value sent over the wire is the literal string `"0"` — i.e., zero capabilities advertised.

The driver emits `IgnoreUnknownRntbdTokens` (bit 3, decimal 8). The header value is a string-encoded decimal of the bitwise OR of the enum bits; the precise integer value should be committed as a Rust constant alongside the existing `COSMOS_SDK_SUPPORTEDCAPABILITIES` header name.

The `IgnoreUnknownRntbdTokens` bit is the contract that backs the silent-skip behavior in "Metadata token filtering" above: the proxy/backend uses this advertisement to decide whether it is safe to add new RNTBD tokens without coordinating with this SDK release. Advertising the bit while *also* failing or warning on unknown tokens would be a contract violation; advertising `"0"` while silently skipping unknown tokens is "merely conservative" but causes the proxy to assume zero forward-compat tolerance — both are wrong; the driver reconciles both ends.

##### Capability bit composition (Rust = `8`)

The bitmask the Rust driver advertises is **`8`** (`IgnoreUnknownRntbdTokens`). Pinned in `azure_data_cosmos_driver/src/driver/transport/cosmos_headers.rs:16-22` with a `const _: () = assert!(SUPPORTED_CAPABILITIES_BITS == 8);` invariant. The bits are:

| Bit | Decimal | Capability | Rust advertises | Notes |
| --- | --- | --- | --- | --- |
| 0 | 1 | `PartitionMerge` | **no** | Forward-compat with merged partition-key ranges. The Rust driver does not yet handle merged ranges in its partition-key routing, so advertising the bit without honoring the behavior could cause incorrect routing on accounts that surface merged ranges. Track in a follow-up when the driver grows merged-range support. |
| 1 | 2 | (reserved capability) | **no** | An additional proxy capability the Rust driver does not yet consume. Unilaterally advertising it without honoring the corresponding behavior could cause mis-framing or unexpected proxy behavior. Track in a follow-up if/when the driver grows the corresponding support. |
| 3 | 8 | `IgnoreUnknownRntbdTokens` | yes | Forward-compat with new RNTBD response tokens added by future proxy/backend versions; backed by the silent-skip behavior in "Metadata token filtering" above. |

Total: Rust `8`. The driver only advertises capabilities it actually implements end-to-end. Adding any further bit requires implementing the corresponding behavior first, then updating the constant in `cosmos_headers.rs` and re-pinning the header-value test.

Test coverage: assert the header value emitted on Gateway 2.0 (and standard Gateway) requests is the expected bitmask string, not `"0"`.

#### RNTBD Request Wire Format

The frame layout is:

| Offset | Size | Field | Encoding | Notes |
| --- | --- | --- | --- | --- |
| 0 | 4 | Total message length | uint32 LE | **Inclusive** of the 4 length bytes themselves (little-endian uint32). |
| 4 | 2 | Resource type | uint16 LE | Narrower than direct-mode RNTBD's uint32 because Gateway 2.0 IDs fit in 16 bits. |
| 6 | 2 | Operation type | uint16 LE | Same rationale as the resource-type field. |
| 8 | 16 | Activity ID | UUID, two uint64 LE | Written as `(mostSignificantBits, leastSignificantBits)` — two little-endian 64-bit integers — **this is not RFC 4122 byte order**. Worked example for UUID `0a1b2c3d-4e5f-6789-abcd-ef0123456789`: `mostSignificantBits = 0x0a1b2c3d_4e5f_6789` → LE bytes `89 67 5f 4e 3d 2c 1b 0a`; `leastSignificantBits = 0xabcd_ef01_2345_6789` → LE bytes `89 67 45 23 01 ef cd ab`. The on-the-wire 16-byte sequence is the MSB bytes followed by the LSB bytes. |
| 24 | var | Metadata tokens | Token stream | Filtered by `thinClientProxyExcludedSet` (see §5.4 header naming). |
| 24+N | 4 | Payload length | uint32 LE | **Only present when the operation type implies a payload** (writes, patch, query body, stored-proc args, batch). Absence is signaled by operation-type convention, not a flag bit. Parsers must consult the operation-type → has-payload table derived from the operation-type definitions. |
| 28+N | var | Payload body | Raw bytes | JSON or Cosmos binary, per resource type. |

#### RNTBD Response Wire Format

| Offset | Size | Field | Encoding | Notes |
| --- | --- | --- | --- | --- |
| 0 | 4 | Total message length | uint32 LE | Inclusive of the 4 length bytes (same convention as request). |
| 4 | 4 | Status code | uint32 LE | Maps to HTTP status + `CosmosStatus`. |
| 8 | 16 | Activity ID | UUID, two uint64 LE | Same MSB-LE / LSB-LE pairing as request. |
| 24 | var | Metadata tokens | Token stream | Request charge, session token, continuation, etc. |
| 24+N | var | Body payload | Raw bytes | Optional; presence determined by total-length arithmetic (`total_length - header_and_tokens_len > 0`). |

---

### 5.4 Gateway 2.0 Request Pipeline

The Gateway 2.0 pipeline wires RNTBD serialization into the existing transport pipeline and adds gateway 2.0-specific header injection.

#### Pipeline behavior

- **Operation filtering** — `is_operation_supported_by_gateway_v2(resource_type, operation_type) → bool`. Only `ResourceType::Document` operations (CRUD, query, batch, read-feed) are eligible. Every other resource type falls through to the standard gateway via the eligibility-fallback path.
- **EPK computation** — Call `EffectivePartitionKey::compute()` (point) or `::compute_range()` (feed/cross-partition) from the driver layer. Do **not** call `azure_data_cosmos::hash::get_hashed_partition_key_string`; SDK call sites route through the driver's implementation.
- **EPK error propagation** — If EPK computation returns `Err` (MultiHash-requires-V2, component-count mismatch, etc.), surface as `CosmosStatus::BadRequest` to the caller. **Do not** fall back to standard gateway — the same inputs would be equally broken there.
- **Header injection** — When `transport_mode == GatewayV2`, inject the Gateway 2.0 headers listed below.
- **Request body wrapping** — Serialize the entire request (headers + body) into RNTBD binary format and POST as the HTTP/2 body.
- **Response unwrapping** — Deserialize the RNTBD response body back into `CosmosResponseHeaders` + raw document bytes.
- **Eligibility fallback** — Operation ineligible for Gateway 2.0 → route through standard gateway for this single request (per-request, deterministic). See §5.6 for the distinct failure-driven fallback.
- **Constants placement & naming** — Gateway 2.0 header constants live in `azure_data_cosmos_driver::constants` under the **`GATEWAY_V2_*` family** (`GATEWAY_V2_OPERATION_TYPE`, `GATEWAY_V2_RESOURCE_TYPE`). The Rust identifiers use the `GATEWAY_V2_*` prefix; the wire header strings are server-defined and remain `x-ms-thinclient-proxy-*`. Any new EPK-range / capability constants follow the same `GATEWAY_V2_*` prefix. **No SDK re-export** — the SDK has no Gateway-2.0 awareness; it invokes the generic `CosmosDriver::execute_operation` interface and the driver decides Gateway 2.0 vs Gateway V1 internally.

#### Supported Operations

Only `ResourceType::Document` is eligible for gateway 2.0:

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
| `x-ms-thinclient-proxy-operation-type` | `GATEWAY_V2_OPERATION_TYPE` (driver) | Numeric operation type | Every Gateway 2.0 request |
| `x-ms-thinclient-proxy-resource-type` | `GATEWAY_V2_RESOURCE_TYPE` (driver) | Numeric resource type | Every Gateway 2.0 request |
| `x-ms-effective-partition-key` | **NEW** — `EFFECTIVE_PARTITION_KEY` (driver) | Canonical EPK hex | Point ops only |
| `x-ms-documentdb-partitionkey` | existing `PARTITION_KEY` constant (SDK) | JSON-encoded partition-key value | Point ops AND single-logical-partition query ops, alongside `x-ms-effective-partition-key` |
| `x-ms-thinclient-range-min` | **NEW** — `GATEWAY_V2_RANGE_MIN` (driver) | Lower bound of EPK range | Feed / cross-partition ops only |
| `x-ms-thinclient-range-max` | **NEW** — `GATEWAY_V2_RANGE_MAX` (driver) | Upper bound of EPK range | Feed / cross-partition ops only |
| `x-ms-cosmos-use-thinclient` | **NEW** — `GATEWAY_V2_USE_THINCLIENT` (driver) | Instructs account-metadata response to advertise Gateway 2.0 endpoints | Account metadata fetches only |

> Wire-header strings (`x-ms-thinclient-*`) are server-defined and unchanged; the Rust-side identifiers use the `GATEWAY_V2_*` prefix.

Per Q3 resolution, the Gateway 2.0 proxy requires the header names `x-ms-thinclient-range-min` / `x-ms-thinclient-range-max` (it does **not** accept `x-ms-start-epk` / `x-ms-end-epk`). The Gateway 2.0 path introduces the new constants above; the existing `START_EPK` / `END_EPK` constants are not emitted on it.

**Tenant identification (RNTBD token, not HTTP header)**: the proxy identifies the target Cosmos account from the existing RNTBD `GlobalDatabaseAccountName` token (`0x00CE`, `String`, optional) carried inside the RNTBD metadata stream on **every** Gateway 2.0 request. No Gateway-2.0-specific HTTP headers are introduced for account or regional-account identification — the RNTBD token is the canonical carrier and matches the proxy wire contract. The value is the global database account name (e.g., `myacct` from `myacct.documents.azure.com`), parsed once from the account endpoint URL at client construction.

#### Consistency header reconciliation (`ConsistencyLevel` ↔ `ReadConsistencyStrategy`)

The Cosmos SDK exposes two consistency knobs that can both target the same read operation:

- **`ConsistencyLevel`** — per-request override of the account default consistency.
- **`ReadConsistencyStrategy`** (defined in `azure_data_cosmos_driver::options::read_consistency`) — read-only strategy override (`Default`, `Eventual`, `Session`, `LatestCommitted`, `GlobalStrong`); supersedes `ConsistencyLevel` on reads.

Wire-format and resolution semantics MUST match the proxy-side contract so that a single proxy-side validation suite is sufficient.

##### Wire carriers

| Transport | Wire carrier for the resolved value | Encoding |
| --- | --- | --- |
| Standard Gateway (V1, HTTP) | HTTP request header `x-ms-cosmos-read-consistency-strategy` | String, exact case-sensitive values: `"Eventual"`, `"Session"`, `"LatestCommitted"`, `"GlobalStrong"`. Header is omitted entirely when the resolved RCS is `Default`. |
| Gateway 2.0 (RNTBD) | RNTBD metadata token ID `0x00FE` | **Byte** type — `Eventual = 0x01`, `Session = 0x02`, `LatestCommitted = 0x03`, `GlobalStrong = 0x04`. The token MUST be Byte-encoded; an earlier String-typed prototype caused the proxy to hang. The token is omitted entirely when the resolved RCS is `Default`. |

The byte values are pinned against the proxy's C++ enum. The RNTBD token catalog (§5.3) carries a row for `ReadConsistencyStrategy = 0x00FE (Byte)` enumerating the four byte values.

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
- **V2 RNTBD**: same mutual exclusion applied to the RNTBD metadata stream — emit either the `ConsistencyLevel` token or the `ReadConsistencyStrategy` token (`0x00FE`), never both. The Gateway 2.0 RNTBD serializer consumes the **already-resolved** value and decides which of the two tokens to emit; it does not re-run resolution.

##### GlobalStrong client-side validation

When the resolved RCS is `GlobalStrong` and the account default consistency is **not** `Strong`, the driver MUST fail the operation **before** transport selection / serialization with a `BadRequestException`-equivalent (Rust: `CosmosError` with the appropriate `ErrorKind`). This avoids a wasted round-trip. The check uses the cached account properties already maintained by the driver; no additional metadata fetch is required.

##### Implementation pitfall

Resolution MUST NOT mutate the request's header map in place; use a header-map copy instead. An in-place mutation can rewrite `x-ms-consistency-level` (e.g., `LatestCommitted` rewritten to `BoundedStaleness`); the gateway then rejects the request because `BoundedStaleness` is stricter than the Session account default. Even though the underlying conflict is real, the diagnostic becomes unrecoverable once the original headers have been clobbered.

For Rust: thread the resolved consistency value through the pipeline as an explicit input to whichever transport handler runs next. Do not write back into the operation's header collection during resolution. If the operation's header collection is needed for the final serialize step, clone it first or pass the resolved value separately.

#### Range header wire format

EPK range headers (`x-ms-thinclient-range-min` / `-max`) carry the canonical, un-padded hex produced by `EffectivePartitionKey::compute_range()`. **Do not** zero-pad to N×32 on the wire. Local comparisons use `EffectivePartitionKey`'s `Ord` / `cmp` impl, which correctly handles the mixed-length boundaries returned by the backend; the `epk_cmp_*` tests in `container_routing_map.rs` (around L625–665) pin this behavior. The comparator is consumed via `binary_search_by(|r| r.min_inclusive.cmp(&epk_val))` (≈L282 of the same file). An earlier zero-padding proposal in PR #4087 (commit `25233c903`) was **not** adopted; stay consistent with the length-aware convention.

> **`Range` semantics pitfall** (from PR #4087): `compute_range` returns a Rust `std::ops::Range<EffectivePartitionKey>` where `start == end` denotes a **point operation**. Standard `Range` iteration treats that as empty, so code that uses `.contains()` or iterates the range directly will misbehave. Always treat `start == end` as the point case explicitly.

#### Gateway 2.0 Header Injection Flow

When `transport_mode == GatewayV2`:

1. Set `x-ms-thinclient-proxy-operation-type` (numeric operation type)
2. Set `x-ms-thinclient-proxy-resource-type` (numeric resource type)
3. Serialize the `GlobalDatabaseAccountName` RNTBD metadata token (`0x00CE`, `String`, optional) with the account host label (e.g., `myacct`) — every request, see "Tenant identification" note above
4. Point op or single-logical-partition query op? Set `x-ms-effective-partition-key` (EPK hash from `EffectivePartitionKey::compute()`).
   Cross-partition feed / query operation? Set `x-ms-thinclient-range-min` and `x-ms-thinclient-range-max` (from `EffectivePartitionKey::compute_range()`); do **not** emit the PK header on the feed path.
5. Serialize the **already-reconciled** consistency value (per "Consistency header reconciliation" above) into the appropriate RNTBD metadata token: `ConsistencyLevel` token if RCS resolved to `Default`, OR the `ReadConsistencyStrategy` token (`0x00FE`, Byte) if RCS resolved to a non-Default value. Emit exactly one of the two — never both. The serializer consumes the resolved value as input; do not re-run resolution here.
6. Serialize headers + body into RNTBD binary format (§5.3)
7. POST RNTBD body to gateway 2.0 endpoint via HTTP/2

When `transport_mode != GatewayV2`: Standard HTTP/REST request (existing flow, unchanged).

---

### 5.5 Endpoint Discovery

#### Region pairing (§PR #3942)

Gateway 2.0 read locations pair **only** with read regions; Gateway 2.0 write locations pair **only** with write regions. A write region that advertises no Gateway 2.0 URL falls back to standard gateway **for writes** (this was deliberate in PR #3942: session retries that reroute reads to write endpoints would otherwise cross the read/write Gateway 2.0 split). This is a correctness invariant — do not "fix" it by cross-pairing.

#### Connectivity probe (HTTP/2 reachability gate)

After endpoint discovery resolves `thinClient{Writable,Readable}Locations` into `CosmosEndpoint::gateway_v2_url`, the driver runs a lightweight HTTP/2 **connectivity probe** against every discovered proxy endpoint before any data-plane RNTBD traffic is allowed to flow. The probe proves TCP + TLS + HTTP/2 reachability to the proxy port — without it, firewall / NSG / Private Endpoint misconfigurations surface later as opaque RNTBD timeouts that customers struggle to attribute.

**Wire contract** (matches the proxy-side `Nghttp2ProxyProtocolHandler` definition and the shared contract in `Product/Cosmos/CLUB/Docs/Photon/Router/ProxyConnectivityProbeDesign.md`, ADO PR 2107592):

| Element | Value |
| --- | --- |
| Method + Path | `POST /connectivity-probe` |
| Request body | empty |
| Response body | empty |
| Protocol | HTTP/2 required (no HTTP/1.1 fallback) |
| `200 OK` | Probe enabled, proxy ready |
| `503 Service Unavailable` | Proxy reachable but federation flag `enableConnectivityProbe` is OFF |
| Any other status / network failure / timeout | Proxy unreachable |

**Gating policy** (mirrors the proxy-side guidance):

1. **Strict** — only `200` counts as success. A `503` (feature disabled) fails the probe; the federation has not opted in to Gateway 2.0 yet, so the data plane stays on Gateway V1.
2. **All-or-nothing, gateway-wide** — when the probe fails, Gateway 2.0 is suppressed for **every operation, every region, every routing decision** (point reads, single-PK queries, full-container scans, HPK queries, plan fetches, the lot). It is **not** an op-by-op filter that downgrades only the operations Gateway V1 can't serve. The fact that V1 happens to satisfy most operations is incidental — they are silently downgraded along with the operations V1 would reject. The all-or-nothing gate stays in effect until a subsequent probe pass succeeds on every region.
3. **No SDK-side opt-out** — the probe runs whenever `thinClient*Locations` are advertised. The federation flag is the operator-facing kill switch; there is **no** customer-facing knob to disable Gateway 2.0 (the only client-side prerequisite is HTTP/2, which short-circuits the entire G2 code path, probe included, when it cannot be negotiated).

**Lifecycle and timing.** The probe must run **on bootstrap, before any data-plane request is dispatched**, so the very first operation routes against a probe-verified snapshot — never against the optimistic "GW_V2 on" snapshot that `build_account_endpoint_state` produces from `thinClient*Locations` alone. Concretely, the bootstrap sequence is:

1. Fetch `getDatabaseAccount`.
2. Run the probe against every advertised thin-client endpoint and resolve the `effective_gateway_v2_enabled` gate.
3. Build the first routing snapshot from the probe-aware gate (so the gate is correct on the *first* `sync_account_properties`, not only on the next background refresh).
4. Re-probe and rebuild every time endpoint discovery refreshes the advertisements (account-metadata refresh loop, region-failure-triggered refresh, etc.).

Probes execute in parallel across regions with a per-endpoint deadline of `DEFAULT_PROBE_TIMEOUT` (5s) — short enough to keep the gating loop responsive but long enough to absorb cold-TCP / TLS-handshake latency.

**Diagnostics.** Each probe call records `(region, role, url, outcome)` into the same diagnostics surface as data-plane requests, so a customer support case can show the exact probe verdict per region. `ProbeFailure::Network(message)` preserves the underlying transport error text verbatim (DNS failure, TLS handshake, connection refused, timeout, etc.) so operators can correlate against their firewall logs.

**Module entry point.** `driver::transport::connectivity_probe::ConnectivityProbe` (trait) + `Http2ConnectivityProbe` (production impl). The trait isolates the probe surface so test code can substitute deterministic outcomes without standing up a real proxy. `CosmosDriver::new` wires the production probe into `LocationStateStore`; bootstrap runs it via `sync_account_properties_with_probe` (in `CosmosDriver::initialize`) and every account-metadata refresh re-runs it inside `refresh_account_properties_inner`.

#### Endpoint Discovery Flow

Account metadata response includes:

- `writableLocations` — standard gateway URLs
- `readableLocations` — standard gateway URLs
- `thinClientWritableLocations` — gateway 2.0 URLs (when available)
- `thinClientReadableLocations` — gateway 2.0 URLs (when available)

`build_account_endpoint_state()` matches regions across these lists and constructs `CosmosEndpoint::regional_with_gateway_v2(region, gw_url, gw_v2_url)`. The resulting `AccountEndpointState` contains endpoints with `gateway_v2_url: Some(...)` when gateway 2.0 is available for that region.

---

### 5.6 Retry & Error Handling

Retry policies are identical between Gateway 2.0 and standard gateway modes — only endpoint selection and request encoding differ. The existing retry pipeline works as-is for most cases.

#### Retry behavior

- **Timeout policy** — Gateway 2.0 requests use the timeout regime defined in `TRANSPORT_PIPELINE_SPEC.md` (single timeout, not bifurcated). Do not introduce Gateway-2.0-specific timeouts in this work; any Gateway 2.0–specific timeout tuning will be addressed in a follow-up.
- **Gateway 2.0 eligibility fallback** — see "Fallback taxonomy" below.
- **Partition-Level Failover interaction** — when PLF (see `PARTITION_LEVEL_FAILOVER_SPEC.md`) selects a region, the per-request decision is: if that region's `CosmosEndpoint` exposes a `gateway_v2_url`, the request uses Gateway 2.0; if it does not, the request falls back to standard gateway for that partition until PLF releases its override. PLF chooses the region; Gateway 2.0 is preferred whenever it is available in that region.

#### Fallback taxonomy

Gateway 2.0 has a single fallback mechanism:

| Name | Scope | Trigger | Duration | Unwind |
| --- | --- | --- | --- | --- |
| **Eligibility fallback** | Per-request | Operation is not eligible for Gateway 2.0 (fails `is_operation_supported_by_gateway_v2()`) | Single request only | N/A — recomputed every request |

There is intentionally **no** Gateway 2.0–specific failure-fallback mechanism (no per-partition consecutive-failure counter, no sticky standard-gateway state, no cooldown). Model selection is per-request and stateless, and the existing retry policies already handle transport errors, 502/503/504, and regional unavailability uniformly across both transport modes. When a Gateway 2.0 request fails, the existing retry policies retry it (which may re-select Gateway 2.0 or land on standard gateway through normal regional-failover behavior); no new state machine is introduced.

---

### 5.7 SDK Integration

Gateway 2.0 is **on by default** when the account metadata advertises Gateway 2.0 endpoints and the connectivity probe confirms them; there is no user opt-in or opt-out. Transport selection is fully server-driven, which minimizes friction for the common case and lets the service migrate accounts transparently.

#### SDK behavior

- **Auto-detection** — When account metadata includes `thinClientReadableLocations` / `thinClientWritableLocations`, the driver automatically prefers gateway 2.0 for eligible operations (per §3.1). No user opt-in required.
- **No client toggle** — There is no `CosmosClientOptions` field and no `AZURE_COSMOS_*` env var to force Gateway V1 or Gateway 2.0 routing. The only client-side prerequisite is HTTP/2; see §3.2 for the full normative wording. Internal negative-term flags (e.g. `gateway_v2_disabled`) are implementation details derived from HTTP/2 availability, not public API.
- **Diagnostics** — `CosmosDiagnostics` should report when a request used gateway 2.0 vs standard gateway (already partially done via `TransportKind::GatewayV2`).
- **User agent** — Update SDK user agent string to indicate gateway 2.0 capability.
- **EPK cutover** — Replace SDK-side callers of `get_hashed_partition_key_string` with calls into the driver's `EffectivePartitionKey::compute()` / `::compute_range()` (this is the cutover PR #4087 flagged). Gateway 2.0 header injection depends on this being correct for hierarchical-PK containers.

#### Auto-Detection Flow

When account metadata includes `thinClientReadableLocations`, gateway 2.0 is enabled automatically (internal). `CosmosEndpoint` gets `gateway_v2_url` and `resolve_endpoint()` prefers Gateway 2.0 (per §3.1's single-source-of-truth rule). No user configuration needed — transparent to the caller.

---

### 5.8 Testing

Testing covers all layers from unit to E2E.

#### Live Tests Pipeline

A **new dedicated CI pipeline** is required for gateway 2.0 live tests. Gateway 2.0 requires a Cosmos DB account with Gateway 2.0 endpoints enabled, which is separate from the standard emulator and live test infrastructure.

**Trigger:** PR changes to `sdk/cosmos/**` + manual dispatch

**Provision:**

- Use a **dedicated, pre-provisioned Cosmos DB account** with Gateway 2.0 endpoints enabled (hardcoded for this pipeline, reused across runs)
- Account credentials stored in pipeline secrets (e.g., `AZURE_COSMOS_GW_V2_ENDPOINT`, `AZURE_COSMOS_GW_V2_KEY`)
- Multi-region configuration (at least 2 regions)
- Verify `thinClientReadableLocations` in account metadata at pipeline start

**Test Matrix:**

- Single-region gateway 2.0
- Multi-region gateway 2.0 with failover
- Gateway 2.0 + standard gateway eligibility fallback (per-request only; normal retries still apply)
- HTTP/2 unavailable — assert all eligible Document ops route through the standard gateway (the one client-side prerequisite that suppresses Gateway 2.0)

**Test Suites:**

- Point CRUD (create, read, replace, upsert, patch, delete)
- Query (single-partition, cross-partition)
- Batch operations
- Change feed (LatestVersion)
- Retry scenarios (408, 410, 449, 503, 404/1002)
- Diagnostics validation (`TransportKind::GatewayV2`)

**Artifacts:** Test results (JUnit XML), diagnostics logs, perf metrics (RU, latency)

#### Pipeline Files

| Action | File | Purpose |
| --- | --- | --- |
| EDIT | `sdk/cosmos/ci.yml` | Add a second `LiveTestMatrixConfigs` entry (`Cosmos_gateway_v2_live_test`) that points at `live-gateway_v2-matrix.json`, plus an `EnvVars` block that injects `AZURE_COSMOS_GW_V2_ENDPOINT` / `AZURE_COSMOS_GW_V2_KEY` from the `azure-sdk-tests-cosmos` service connection. |
| NEW  | `sdk/cosmos/live-gateway_v2-matrix.json` | Gateway 2.0 live test matrix (single-region + multi-region; `testCategory` = `gateway_v2` / `gateway_v2_multi_region`). The pre-provisioned account is supplied via the env vars above; the matrix's `ArmTemplateParameters` block is preserved so the deploy step still runs even though the per-run account is unused. |
| EDIT | `sdk/cosmos/live-platform-matrix.json` | Add gateway 2.0 test matrix entry |

#### Test Coverage Matrix

| Test Category | Unit | Integration | E2E | Scenarios |
| --- | --- | --- | --- | --- |
| RNTBD serialization | Yes | | | Round-trip, edge cases, malformed input |
| RNTBD unknown-token tolerance | Yes | | | Inject synthetic unknown token IDs into a response frame; deserializer must skip + log, never panic / error / drop the rest of the response |
| EPK computation | Yes | | | Single/hierarchical PK, hash versions 1 and 2, error cases (MultiHash V1, wrong component count) |
| Operation filtering | Yes | | | All ResourceType × OperationType combos; asserts only `Document` ops are eligible and every other resource type (including `StoredProcedure`) falls through to the standard gateway |
| Header injection | Yes | | | Point vs feed EPK headers, proxy type headers, range-header un-padded form |
| HPK + Gateway 2.0: full vs partial PK | Yes | | Yes | Hierarchical container (2- and 3-component PK paths). **Full PK** (all components specified) on a point op → emits `x-ms-effective-partition-key` carrying the single EPK from `EffectivePartitionKey::compute()`. **Partial PK** (1- or 2-component prefix) on a feed / cross-partition / delete-by-PK op → emits `x-ms-thinclient-range-min` / `x-ms-thinclient-range-max` carrying the EPK range from `EffectivePartitionKey::compute_range()`. Asserted at unit level (header presence + exact wire form, range bounds for each prefix length) and E2E (round-trip against a live HPK container). |
| Account-name RNTBD token | Yes | | | `GlobalDatabaseAccountName` (`0x00CE`, `String`) present in the RNTBD metadata stream of every Gateway 2.0 request (point, feed, batch, bulk, change feed). Value matches the host label of the account endpoint URL. |
| SDK-supported-capabilities header | Yes | | | `x-ms-cosmos-sdk-supportedcapabilities` value emitted is the bitmask string for `IgnoreUnknownRntbdTokens` (`"8"`), **not** `"0"`. |
| Consistency reconciliation: token + header encoding | Yes | | | RNTBD token `0x00FE` Byte round-trip for all 4 strategies; HTTP header `x-ms-cosmos-read-consistency-strategy` exact wire-string mapping for all 4 strategies; `Default` emits neither carrier on either transport. |
| Consistency reconciliation: dual-header rejection | Yes | | | SDK never emits both `x-ms-consistency-level` AND `x-ms-cosmos-read-consistency-strategy` on V1; never emits both `ConsistencyLevel` and `ReadConsistencyStrategy` RNTBD tokens on V2. Verified across all 16 (CL × RCS, request-level × client-level) combinations. |
| Consistency reconciliation: 4-source precedence | Yes | | | Request-RCS > Request-CL > Client-RCS > Client-CL > account default; `Default` at any RCS layer is a pass-through. Representative subset of the full matrix. |
| Consistency reconciliation: GlobalStrong validation | Yes | | | RCS=GlobalStrong on a non-Strong account produces a fail-fast `CosmosError` (no wire request emitted); on a Strong account the request proceeds normally. |
| Consistency reconciliation: header-map immutability | Yes | | | Resolution does not mutate the operation's original request headers; a session-token rewrite cannot clobber `x-ms-consistency-level`. |
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
| HTTP/2 unavailable | Yes | Yes | | All eligible Document ops (point + feed + batch + change feed) route through standard gateway when HTTP/2 cannot be negotiated; default (HTTP/2 available) does not change behavior |
| Eligibility fallback | | Yes | | StoredProcedure Execute (and all other non-Document resource types) → standard gateway |
| PLF precedence | | Yes | | Region without gw_v2_url + PLF override → standard gateway path |
| Multi-region failover | | Yes | Yes | Preferred regions, failover |
| Fault injection | | Yes | | Timeout, 503, network error |
| Perf benchmarks | | | Yes | Already wired in perf crate |
| Diagnostics validation | Yes | Yes | | TransportKind::GatewayV2 in diagnostics output |

---

## 6. Open Questions

- **Q1 — HTTP/2 prior knowledge vs ALPN**: _Resolved_. Gateway 2.0 always uses HTTP/2; the proxy does not accept HTTP/1.x. Rust uses HTTP/2 with prior knowledge on the Gateway 2.0 transport (no ALPN fallback to HTTP/1.x). The broader ALPN default in `TRANSPORT_PIPELINE_SPEC.md` does **not** apply to Gateway 2.0; if HTTP/2 negotiation fails, the request fails and the existing retry policies handle it.
- **Q2 — Live test account provisioning**: Cosmos DB account configuration flags required to enable Gateway 2.0 endpoints are not part of the standard Bicep templates. _Resolution_: hardcode a dedicated, pre-provisioned Gateway 2.0 account for the gateway 2.0 live tests pipeline and reuse it across runs (rather than provisioning per-run via Bicep). Account name and credentials stored in pipeline secrets (`AZURE_COSMOS_GW_V2_ENDPOINT`, `AZURE_COSMOS_GW_V2_KEY`); pipeline reads endpoint from environment variables.
- **Q3 — EPK range header names**: _Resolved_. The Gateway 2.0 proxy requires the header names `x-ms-thinclient-range-min` / `x-ms-thinclient-range-max`. The Gateway 2.0 path introduces new constants (`GATEWAY_V2_RANGE_MIN`, `GATEWAY_V2_RANGE_MAX`); the existing `START_EPK` / `END_EPK` (`x-ms-start-epk` / `x-ms-end-epk`) constants remain for any non-Gateway-2.0 callers but are **not** emitted on Gateway 2.0 requests.
- **Q4 — Connectivity-failure handling (G2 → G1)**: Decision is **fail-fast**: when all G2 endpoints fail with connectivity-class errors, the operation surfaces the standard transport error without auto-fallback. Recovery is server-driven — the connectivity probe gates Gateway 2.0 off for unreachable regions, and/or the account stops advertising thin-client endpoints — with no client-side toggle. See §4.3.
