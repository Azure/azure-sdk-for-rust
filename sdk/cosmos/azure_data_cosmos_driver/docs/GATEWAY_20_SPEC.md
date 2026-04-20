# Gateway 2.0 Design Spec for Rust Driver & SDK

## 1. Overview

Gateway 2.0 (formerly "thin client") is a server-side proxy that allows SDK clients to route data-plane operations through a lightweight proxy endpoint instead of directly to backend replicas. It uses RNTBD binary protocol over HTTP/2, with the proxy handling partition routing, replica selection, and load balancing.

**Naming**: Use "Gateway 2.0" consistently in all Rust code, docs, and comments. Avoid "thin client" except when referencing Java/.NET code.

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
| Replica Mgmt | Proxy | Proxy | SDK |
| Partition Route | Proxy | Proxy | SDK |
| Regional Route | SDK | SDK | SDK |
| SDK Complexity | Low | Medium | High |
| Firewall Rules | 1 endpoint | 1 endpoint | N replicas |

---

## 3. Current Rust State

The Rust driver (`azure_data_cosmos_driver`) already has significant gateway 2.0 scaffolding:

### Already Implemented
- **`CosmosEndpoint`** — `gateway20_url: Option<Url>` field, `regional_with_gateway20()`, `uses_gateway20()`, `selected_url()` methods
- **`TransportMode::Gateway20`** enum variant in pipeline components
- **`RoutingDecision`** — carries `transport_mode` that distinguishes gateway vs gateway 2.0
- **`ConnectionPoolOptions`** — `is_gateway20_allowed: bool` config, env var `AZURE_COSMOS_CONNECTION_POOL_IS_GATEWAY20_ALLOWED`
- **`CosmosTransport`** — `dataplane_gateway20_transport: OnceLock<AdaptiveTransport>`, lazy init with `AdaptiveTransport::gateway20()`
- **`AdaptiveTransport::ShardedGateway20`** variant — always HTTP/2 with prior knowledge
- **`HttpClientConfig::dataplane_gateway20()`** — HTTP/2-only config
- **`TransportKind::Gateway20`** in diagnostics
- **`LocationStateStore`** — `gateway20_enabled` flag, passes through to endpoint construction
- **Routing systems** — `build_account_endpoint_state()` resolves gateway 2.0 URLs from account properties
- **`resolve_endpoint()`** in operation pipeline — selects gateway 2.0 URL when `prefer_gateway20` is true
- **Constants** — `THINCLIENT_PROXY_OPERATION_TYPE`, `THINCLIENT_PROXY_RESOURCE_TYPE`, `START_EPK`, `END_EPK` headers defined
- **EPK computation** — `get_hashed_partition_key_string()` already computes EPK in `container_connection.rs`
- **Perf crate** — `gateway20_allowed` config wiring

### Not Yet Implemented (Gaps)
1. **RNTBD serialization/deserialization** — No binary protocol encoding/decoding exists
2. **Gateway 2.0 header injection** — Thin client proxy headers (`x-ms-thinclient-proxy-operation-type`, `x-ms-thinclient-proxy-resource-type`, EPK range headers) are not applied to requests
3. **Supported operation filtering** — No `IsOperationSupportedByThinClient()` equivalent
4. **Gateway 2.0 endpoint discovery** — Verify account metadata parsing of `thinClientReadableLocations`/`thinClientWritableLocations`
5. **Session token handling** — Gateway 2.0 may handle session tokens differently (partition-key-range-id prefix)
6. **Gateway 2.0 specific retry logic** — Fallback from gateway 2.0 to standard gateway on specific errors
7. **Integration/E2E tests** — No gateway 2.0 test coverage
8. **Fault injection** — No gateway 2.0 fault injection scenarios

---

## 4. Rust Implementation Plan

### Current Request Flow (Gateway 1.0)

1. `CosmosClient::create_item(T)` calls `ContainerClient`
2. `container_connection.rs` serializes `T` to `&[u8]`, computes EPK, resolves PKRange
3. `CosmosDriver::execute_operation()` enters the Operation Pipeline (7-stage loop)
4. `resolve_endpoint()` selects a gateway endpoint
5. Transport Pipeline applies cosmos headers, signs request
6. HTTP/REST request sent to Cosmos Gateway (shared proxy, no SLA)

### Target Request Flow (Gateway 2.0)

1. `CosmosClient::create_item(T)` calls `ContainerClient`
2. `container_connection.rs` serializes `T` to `&[u8]`, computes EPK, resolves PKRange
3. `CosmosDriver::execute_operation()` enters the Operation Pipeline (7-stage loop)
4. `resolve_endpoint()` prefers gateway 2.0 endpoint (if available from account metadata)
5. Transport Pipeline checks `is_supported_by_gw20()?`:
   - **YES**: Inject gateway 2.0 headers + RNTBD serialize -> HTTP/2 POST to Gateway 2.0 Proxy (SLA)
   - **NO**: Standard HTTP/REST request to Cosmos Gateway (fallback)

---

### Phase 1: RNTBD Protocol (Driver Layer)

**Crate**: `azure_data_cosmos_driver`
**New module**: `src/driver/transport/rntbd/`

The RNTBD (Reliable Network Transfer Binary Data) protocol is the wire format used by Cosmos DB for efficient binary communication. Gateway 2.0 wraps RNTBD-encoded payloads inside HTTP/2 POST requests to the proxy.

#### What Will Be Done

- **`rntbd/mod.rs`** — Module root, public types
- **`rntbd/request.rs`** — Request serialization: operation headers, resource metadata, partition key info → binary payload
- **`rntbd/response.rs`** — Response deserialization: 24-byte frame header → metadata section → optional body payload
- **`rntbd/tokens.rs`** — RNTBD token types (type IDs, lengths, value encodings) used in metadata sections
- **`rntbd/status.rs`** — RNTBD status code mapping to `CosmosStatus`

#### RNTBD Request Wire Format

There is no version negotiation for thin client RNTBD. The frame format is fixed (derived from Java `RntbdRequestFrame`):

| Offset | Size | Field | Encoding |
| --- | --- | --- | --- |
| 0 | 4 | Total message length | uint32 LE (frame + metadata + payload) |
| 4 | 2 | Resource type | uint16 LE |
| 6 | 2 | Operation type | uint16 LE |
| 8 | 16 | Activity ID | UUID (two uint64 LE) |
| 24 | var | Metadata tokens | Token stream (filtered by `thinClientProxyExcludedSet`) |
| 24+N | 4 | Payload length | uint32 LE (only if payload present) |
| 28+N | var | Payload body | Raw bytes (JSON or Cosmos binary) |

Frame header is 24 bytes. The `encode(byteBuf, forThinClient=true)` flag in Java excludes certain headers that the proxy does not need.

#### RNTBD Response Wire Format

| Offset | Size | Field | Encoding |
| --- | --- | --- | --- |
| 0 | 4 | Total message length | uint32 LE |
| 4 | 4 | Status code | uint32 LE |
| 8 | 16 | Activity ID | UUID (two uint64 LE) |
| 24 | var | Metadata tokens | Token stream (request charge, session token, continuation, etc.) |
| 24+N | var | Body payload | Raw bytes (optional) |

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

- **Operation filtering** — New function `is_operation_supported_by_gateway20(resource_type, operation_type) → bool` to match .NET's `IsOperationSupportedByThinClient()`. Only data-plane document operations + stored procedure Execute are eligible.
- **Header injection** — When `TransportMode::Gateway20`, inject thin client headers before sending:
  - `x-ms-thinclient-proxy-operation-type` — numeric operation type
  - `x-ms-thinclient-proxy-resource-type` — numeric resource type
  - `x-ms-effective-partition-key` — EPK hash for point operations
  - `x-ms-thinclient-range-min` / `x-ms-thinclient-range-max` — EPK range for feed operations
- **Request body wrapping** — Serialize the entire request (headers + body) into RNTBD binary format and POST as the HTTP/2 body
- **Response unwrapping** — Deserialize the RNTBD response body back into `CosmosResponseHeaders` + raw document bytes
- **Fallback** — If operation is not supported by gateway 2.0, transparently route through standard gateway

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
| Batch | Yes | Includes bulk operations |
| StoredProcedure Execute | **No** | Following Java; only `ResourceType::Document` is eligible |
| All other resource types | **No** | Metadata operations use standard gateway |

#### Gateway 2.0 Header Injection Flow

When `transport_mode == Gateway20`:

1. Set `x-ms-thinclient-proxy-operation-type` (numeric operation type)
2. Set `x-ms-thinclient-proxy-resource-type` (numeric resource type)
3. Point operation? Set `x-ms-effective-partition-key` (EPK hash)
   Feed operation? Set `x-ms-thinclient-range-min` and `x-ms-thinclient-range-max`
4. Serialize headers + body into RNTBD binary format
5. POST RNTBD body to gateway 2.0 endpoint via HTTP/2

When `transport_mode != Gateway20`: Standard HTTP/REST request (existing flow, unchanged)

#### Files Changed

```
NEW   src/driver/transport/gateway20.rs          — inject_gateway20_headers(), RNTBD wrap/unwrap
EDIT  src/driver/transport/transport_pipeline.rs  — Branch on TransportMode in execute_transport_pipeline()
EDIT  src/driver/transport/cosmos_headers.rs      — Add gateway 2.0 header application
EDIT  src/driver/transport/mod.rs                 — Add is_operation_supported_by_gateway20()
EDIT  src/driver/pipeline/components.rs           — Add EPK fields to TransportRequest if needed
```

---

### Phase 3: Endpoint Discovery (Driver Layer)

**Crate**: `azure_data_cosmos_driver`

This phase is largely already implemented. The remaining work is confirming the endpoint URL pattern and ensuring the account metadata cache properly resolves gateway 2.0 URLs.

#### What Will Be Done

- **Verify** account metadata cache already parses `thin_client_readable_locations` / `thin_client_writable_locations` into gateway 2.0 URLs
- **Confirm** `build_account_endpoint_state()` in routing_systems.rs correctly constructs `CosmosEndpoint::regional_with_gateway20()`
- **Test** endpoint discovery with live account that has gateway 2.0 enabled
- **Add** `x-ms-cosmos-use-thinclient` header to account metadata requests to trigger gateway 2.0 endpoint advertisement

#### Endpoint Discovery Flow (Existing)

Account metadata response includes:

- `writableLocations` — standard gateway URLs
- `readableLocations` — standard gateway URLs
- `thinClientWritableLocations` — gateway 2.0 URLs (when available)
- `thinClientReadableLocations` — gateway 2.0 URLs (when available)

`build_account_endpoint_state()` matches regions across these lists and constructs `CosmosEndpoint::regional_with_gateway20(region, gw_url, gw20_url)`. The resulting `AccountEndpointState` contains endpoints with `gateway20_url: Some(...)` when gateway 2.0 is available for that region.

#### Files Changed

```
EDIT  src/driver/cache/account_metadata_cache.rs  — Verify/fix thin client endpoint parsing
EDIT  src/driver/transport/cosmos_headers.rs       — Add x-ms-cosmos-use-thinclient header
TEST  src/driver/routing/routing_systems.rs        — Add tests for gateway 2.0 endpoint construction
```

---

### Phase 4: Retry & Error Handling (Driver Layer)

**Crate**: `azure_data_cosmos_driver`

Retry policies are identical between thin client and standard gateway modes in both Java and .NET — only endpoint selection and request encoding differ. The existing retry pipeline should work as-is for most cases.

#### What Will Be Done

- **Timeout policy** — Gateway 2.0 requests may use different timeout values than standard gateway
- **Read timeout cross-region retry** — On HTTP 408 with `GATEWAY_ENDPOINT_READ_TIMEOUT` sub-status, retry read operations in the next preferred region
- **Service unavailable (503)** — Mark endpoint unavailable for partition key range, then retry. Follow Java's conservative approach: only retry server-returned 503 or SDK-generated 503 with `SERVER_GENERATED_410` sub-status
- **Gone (410)** — Action depends on sub-status code:
  - `PARTITION_KEY_RANGE_GONE` (1002): Refresh PKRange cache, retry
  - `COMPLETING_SPLIT_OR_MERGE` (1007): Refresh PKRange cache, retry
  - `COMPLETING_PARTITION_MIGRATION` (1008): Refresh PKRange cache, retry
  - `NAME_CACHE_IS_STALE` (1000): Refresh **collection** cache (NOT PKRange), retry
  - Other sub-statuses: Retry with backoff, no cache refresh
- **Gateway 2.0 fallback** — On persistent gateway 2.0 failures (e.g., proxy down), fall back to standard gateway for the remainder of the operation

#### Retry Decision Table

| Response | Sub-Status | Action |
| --- | --- | --- |
| 200-299 | — | Success |
| 404 | — | Not Found (propagate to caller) |
| 408 Timeout | — | Read: retry cross-region; Write: retry local only |
| 410 Gone | 1002 (PKRangeGone) | Refresh PKRange cache, retry |
| 410 Gone | 1007 (SplitMerge) | Refresh PKRange cache, retry |
| 410 Gone | 1008 (PartitionMigration) | Refresh PKRange cache, retry |
| 410 Gone | 1000 (NameCacheStale) | Refresh **collection** cache, retry |
| 410 Gone | other | Retry with backoff |
| 429 Throttled | — | Existing throttle retry loop (unchanged) |
| 449 Retry With | — | Retry same region (transient conflict) |
| 503 Unavailable | server-returned | Mark endpoint unavailable, failover |
| 503 Unavailable | SDK-generated | Only retry if `SERVER_GENERATED_410` sub-status |
| Proxy unreachable | — | Fallback to `TransportMode::Gateway` (standard HTTP/REST) |

#### Files Changed

```
EDIT  src/driver/pipeline/operation_pipeline.rs   — Gateway 2.0 retry classification
EDIT  src/driver/pipeline/components.rs           — Add Gateway20Fallback variant if needed
EDIT  src/driver/transport/transport_pipeline.rs  — Timeout policy for gateway 2.0
NEW   src/driver/pipeline/gateway20_retry.rs      — Gateway 2.0 specific retry logic (if needed)
```

---

### Phase 5: SDK Integration

**Crate**: `azure_data_cosmos`

Gateway 2.0 is **not exposed as a customer-facing configuration**. The SDK automatically uses gateway 2.0 when the account metadata advertises thin client endpoints. This matches the design philosophy of both Java and .NET SDKs.

#### What Will Be Done

- **Auto-detection** — When account metadata includes `thinClientReadableLocations` / `thinClientWritableLocations`, the driver automatically prefers gateway 2.0 for eligible operations. No user opt-in required.
- **Internal config** — The existing `ConnectionPoolOptions.is_gateway20_allowed` remains internal-only (not exposed in `CosmosClientOptions`). It serves as a kill switch for testing or emergency fallback, not a user-facing setting.
- **Diagnostics** — `CosmosDiagnostics` should report when a request used gateway 2.0 vs standard gateway (already partially done via `TransportKind::Gateway20`)
- **User agent** — Update SDK user agent string to indicate gateway 2.0 capability
- **Container connection** — Ensure EPK is computed and available for gateway 2.0 header injection in the driver layer

#### Auto-Detection Flow

#### Auto-Detection Flow

When account metadata includes `thinClientReadableLocations`, gateway 2.0 is enabled automatically (internal). `CosmosEndpoint` gets `gateway20_url` and `resolve_endpoint()` prefers Gateway 2.0. No user configuration needed — transparent to the caller.

#### Files Changed

```
EDIT  src/driver_bridge.rs                       — Ensure internal config passes through
EDIT  src/handler/container_connection.rs         — Ensure EPK available for driver
EDIT  src/constants.rs                            — Any new header constants
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
- Gateway 2.0 + standard gateway fallback

**Test Suites:**

- Point CRUD (create, read, replace, upsert, patch, delete)
- Query (single-partition, cross-partition)
- Batch operations
- Change feed (LatestVersion)
- Retry scenarios (408, 410, 503)
- Diagnostics validation (`TransportKind::Gateway20`)

**Artifacts:** Test results (JUnit XML), diagnostics logs, perf metrics (RU, latency)

#### Pipeline Files

| NEW | `sdk/cosmos/ci-gateway20.yml` | Gateway 2.0 live tests pipeline definition (uses pre-provisioned account) |
| EDIT | `sdk/cosmos/live-platform-matrix.json` | Add gateway 2.0 test matrix entry |

#### Test Coverage Matrix

| Test Category | Unit | Integration | E2E | Scenarios |
| --- | --- | --- | --- | --- |
| RNTBD serialization | Yes | | | Round-trip, edge cases, malformed input |
| EPK computation | Yes | | | Single/hierarchical PK, hash versions 1 and 2 |
| Operation filtering | Yes | | | All ResourceType x OperationType combos |
| Header injection | Yes | | | Point vs feed EPK headers, proxy type headers |
| Gateway 2.0 transport | Yes | Yes | | Correct HTTP/2 config, sharded pool selection |
| Point CRUD | | | Yes | Create, read, replace, upsert, patch, delete |
| Query | | | Yes | SQL query, cross-partition |
| Batch | | | Yes | Transactional batch ops |
| Change feed | | | Yes | LatestVersion, incremental |
| Retry: 408 timeout | | Yes | | Cross-region for reads, local-only for writes |
| Retry: 503 | | Yes | | Regional failover |
| Retry: 410 Gone | | Yes | | PKRange refresh (sub-status specific) |
| Gateway 2.0 fallback | | Yes | | Proxy down -> standard gateway |
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

- **Phase 1** (RNTBD Protocol) and **Phase 3** (Endpoint Discovery) can proceed in parallel
- **Phase 2** (Request Pipeline) depends on Phase 1 and Phase 3
- **Phase 4** (Retry/Errors) and **Phase 5** (SDK Integration) depend on Phase 2
- **Phase 6** (Testing) depends on all previous phases

---

## 5. Open Questions

1. **HTTP/2 prior knowledge vs ALPN**: Rust already configures gateway 2.0 as HTTP/2 with prior knowledge — confirm this matches service expectations.
2. **Live test account provisioning**: Cosmos DB account configuration flags required to enable gateway 2.0 / thin client endpoints are not part of the standard Bicep templates. **Decision**: hardcode a dedicated, pre-provisioned thin client account for the gateway 2.0 live tests pipeline and reuse it across runs (rather than provisioning per-run via Bicep). The account name and credentials should be stored in pipeline secrets, with the pipeline reading the endpoint from environment variables.
