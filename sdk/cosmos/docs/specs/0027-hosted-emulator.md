# Hosted Cosmos DB Emulator

**Status:** Draft
**Date:** 2026-08-31
**Crates:** `azure_data_cosmos_emulator`, `azure_data_cosmos_driver`
**Feature gate:** `__internal_in_memory_emulator`

## Purpose and charter

The hosted emulator makes the driver's in-memory Cosmos DB emulator usable by
other language SDKs and any other network client. It runs the same store behind
real network listeners instead of injecting it as an in-process
`azure_core::http::HttpClient`.

Its primary purpose is deterministic testing of behavior driven by account and
partition topology:

- regional routing, endpoint outages, and recovery;
- write-region changes and topology refresh;
- replication lag and per-partition failover;
- physical partition split and merge; and
- Gateway V1 versus Gateway 2.0 transport selection.

These conditions are difficult to create repeatedly in a live account and
often cannot be triggered at the exact point required by a test. Existing
Cosmos DB emulators remain useful for general compatibility, but they do not
provide the fine-grained runtime topology controls required by these SDK
routing and resiliency tests. They also introduce platform, container,
startup-time, and certificate dependencies that make large deterministic test
matrices more expensive and less predictable.

The hosted emulator is an open-source, memory-backed SDK engineering and test
tool. It optimizes for topology control, deterministic behavior, fast startup,
and useful wire fidelity. It is not the Azure Cosmos DB Emulator product or a
complete Cosmos DB service implementation.

### Design principles

- **Reuse the in-process core.** The emulator store and operation handlers stay
  in `azure_data_cosmos_driver`; the host is a thin network and configuration
  shell.
- **Exercise real client paths.** The host speaks the supported Cosmos gateway
  contracts so clients use their normal discovery, routing, retry, session, and
  transport paths.
- **Remain deterministic and self-contained.** The memory-backed store requires
  no Azure account, Docker container, or external network access.
- **Keep emulator controls explicit.** Operations that have no Cosmos gateway
  equivalent use a separate management REST API.
- **Fail unsupported behavior explicitly.** The emulator does not silently
  approximate unsupported operations or protocol semantics in ways that could
  invalidate a test.

### Non-goals

- The host is not a supported customer product. Public availability does not
  imply service compatibility, durability, performance, availability, or
  support guarantees.
- It does not persist data or provide production data safety.
- It implements only the data-plane operations needed to provision fixtures
  and observe SDK behavior under simulated topology conditions.
- Query support is limited to the subset needed by SDK scenarios. Full Cosmos
  SQL semantics, broad query compatibility, and other complex data-plane
  features are non-goals.
- Initial Gateway 2.0 interoperability targets the Rust SDK and generic clients
  that support cleartext HTTP/2. Stock peer-SDK compatibility requires
  independent validation and, where necessary, HTTPS/H2 support.
- New features are added to enable concrete SDK test scenarios, not to pursue
  general Cosmos DB feature parity.

Live accounts remain the fidelity reference, and existing emulators remain
useful compatibility targets. Neither replaces the hosted emulator's
deterministic topology-control role.

### Charter alternatives

- Live Cosmos DB accounts were rejected as the primary test mechanism because
  they add cost, provisioning time, and environmental variability, and cannot
  deterministically expose every topology transition or failure at a chosen
  point in a test.
- Existing Cosmos DB emulators were rejected as the sole mechanism because
  they do not expose the runtime controls needed for region, replication,
  split, merge, and failover scenarios, and they carry additional platform and
  deployment dependencies.
- SDK-local mocks were rejected as the shared solution because they bypass real
  network and wire behavior, duplicate semantics in every language, and cannot
  validate Gateway 2.0 framing or cross-SDK compatibility.
- Expanding this project into a full customer-facing Cosmos DB emulator was
  rejected because it would shift effort away from deterministic SDK topology
  testing and create an impractical service-compatibility and support
  commitment.

## Ownership and crate boundary

The implementation is split between two crates:

| Owner | Responsibilities |
| --- | --- |
| `azure_data_cosmos_driver` | In-memory store, request dispatch, EPK routing, account topology, replication, split/merge behavior, Gateway V1 operation handlers, and the RNTBD codec plus a high-level hosted-emulator adapter. |
| `azure_data_cosmos_emulator` | CLI, host-owned JSON DTOs and validation, listener binding, HTTP bridging, Gateway 2.0 connectivity probe, management REST routing, startup seeding orchestration, ready-record publication, transport security, and authentication. |

`azure_data_cosmos_emulator` is a separate `publish = false` binary crate. The
driver exposes only the additional surface the host needs behind the existing
`__internal_in_memory_emulator` feature. The host enables that feature through
its dependency declaration; host users do not enable it manually. The
`__internal_` prefix marks the surface as outside the driver's SemVer contract.

The host remains thin even though it owns network-facing policy. It translates
host DTOs and wire requests into high-level driver operations rather than
reimplementing store, routing, or protocol rules.

### Crate-boundary alternatives

- Moving the emulator implementation into the host crate was rejected because
  it depends heavily on driver internals and would either duplicate them or
  force a large unstable surface to become public.
- Extracting the emulator into a separate library crate was rejected for the
  same reason: it would expose store, dispatch, EPK-routing, and codec internals
  across a new crate boundary.
- Hosting inside the driver crate was rejected because CLI, listener,
  serialization, certificate, and identity dependencies do not belong in the
  reusable execution engine.
- Adding a second host-specific feature was rejected because the existing
  internal emulator feature already owns this non-SemVer surface and another
  feature would add no useful granularity.

## Process and endpoint topology

One process hosts one shared `InMemoryEmulatorHttpClient` and
`EmulatorStore`. Every data-plane and management listener operates on that
shared state.

### One endpoint per region

Each virtual region has a distinct loopback standard-gateway endpoint. The
driver reads `databaseAccountEndpoint` values from account discovery and must
be able to connect to each advertised regional URL independently. The store
already resolves a request's region by `(scheme, host, port)`, so separate
listeners map directly onto the existing routing model without introducing a
second region-selection mechanism.

A single-region account has one standard gateway endpoint. A region may also
have a second, optional Gateway 2.0 endpoint. A dedicated management endpoint
is separate from all Cosmos wire-protocol endpoints.

The standard gateway and management ports default to `0`, asking the operating
system to assign available ports. Gateway 2.0 is enabled only when
`gateway20Port` is present; its value may also be `0`. Explicit nonzero ports
remain available for interactive scenarios. Every account response advertises
the actual bound URLs, never the requested port values.

This default avoids port collisions when multiple hosts run concurrently on a
shared CI agent.

### Ready record

After every listener is bound and startup provisioning is complete, the host
writes exactly one machine-readable JSON `ready` record to stdout. Diagnostic
logs go to stderr so automation can parse stdout without filtering logs.

```json
{
  "event": "ready",
  "managementEndpoint": "http://127.0.0.1:49150/",
  "accountEndpoint": "http://127.0.0.1:49151/",
  "regions": [
    {
      "name": "East US",
      "gatewayEndpoint": "http://127.0.0.1:49151/",
      "gateway20Endpoint": "http://127.0.0.1:49152/"
    }
  ]
}
```

The record contains the resolved management endpoint, hub account endpoint,
and every region's standard gateway and optional Gateway 2.0 endpoint. The
management `GET /account` response returns the same resolved topology.
Consumers use these complete URLs and never reconstruct endpoints from
configured ports.

Returning complete URLs leaves room for future host-based routing or
non-loopback bindings without changing the discovery contract.

### Endpoint-topology alternatives

- A single port differentiated by `Host` names such as `eastus.localhost` was
  not selected because it requires client-side DNS or hosts-file setup. The
  ready contract does not prevent that implementation in the future.
- One process per region was rejected because it would fragment shared state
  and complicate replication and failover simulation.
- Fixed ports were rejected as the default because parallel tests cannot
  reliably coordinate port ownership.

## Data-plane hosting

### Gateway V1

Each standard gateway listener rebuilds an `azure_core::http::Request` from the
incoming method, host, path, query, headers, and body. It delegates the request
to `InMemoryEmulatorHttpClient::execute_request` and converts the returned
`AsyncRawResponse` into the network response.

The listener serves all operations implemented by the in-process emulator,
including point operations and gateway-native metadata or lifecycle operations
such as database, container, offer, partition-key-range, and account requests.

### Gateway 2.0 and RNTBD ownership

Gateway 2.0 is enabled independently for each region by configuring
`gateway20Port`. When enabled:

1. Account discovery advertises `thinClientReadableLocations` and
   `thinClientWritableLocations` using the bound Gateway 2.0 URLs.
2. The host's Gateway 2.0 listener answers `POST /connectivity-probe` with
   `200 OK`.
3. The listener requires HTTP/2 and accepts RNTBD request frames wrapped in
   HTTP/2 POST requests.
4. The driver decodes the RNTBD frame and literal `thinclient` headers,
   reconstructs and dispatches the logical operation through the shared store,
   and encodes the result as an RNTBD response frame.

The host owns the listener, HTTP/2 policy, connectivity probe, and bridge into
the driver. The driver remains the single owner of RNTBD wire compatibility.
The inverse server operations stay beside the client codec in
`src/driver/transport/rntbd/`, and the host calls one feature-gated high-level
adapter rather than depending on public token or frame internals.

This split prevents the host from becoming a second RNTBD implementation whose
token IDs, operation mappings, or framing rules can drift from the production
driver. Unsupported RNTBD semantics fail explicitly.

Gateway V1 remains the default. A region without a Gateway 2.0 endpoint does
not advertise one, so the driver's existing discovery and probe behavior
selects the correct path without a host-specific client routing switch.

### Gateway 2.0 alternatives

- A second RNTBD codec in the host was rejected because its protocol mappings
  would drift from the client codec.
- Publishing frame and token types as a general public API was rejected because
  they are unstable transport internals.
- Always advertising Gateway 2.0 was rejected because configuration gating
  keeps Gateway V1 as the safe default and lets tests select the transport they
  intend to exercise.

## Startup configuration and seeding

The host accepts one canonical JSON configuration file through `--config`.
It declares account topology, listener hints, replication, databases,
containers, and optional seed items. The host validates the complete
configuration before provisioning any resource.

The host owns `serde` DTOs for this contract and translates them into driver
types such as `VirtualAccountConfig`, `VirtualRegion`, and `ContainerConfig`.
Driver configuration types do not gain serialization concerns; some contain
closures or shared mutable state and are intentionally not JSON DTOs.

JSON is the canonical representation. A future syntax may be added only as a
parser into the same host-owned model, not as an independently evolving
configuration contract.

### Example

```json
{
  "account": {
    "id": "emulator-account",
    "writeMode": "single",
    "consistency": "session",
    "perPartitionFailover": false,
    "throttling": false,
    "regions": [
      {
        "name": "East US",
        "gatewayPort": 0,
        "gateway20Port": 0,
        "regionId": 0
      },
      {
        "name": "West US",
        "gatewayPort": 0,
        "gateway20Port": 0,
        "regionId": 1
      }
    ],
    "replication": {
      "minDelayMs": 20,
      "maxDelayMs": 50,
      "maxBufferedReplications": 10000
    },
    "replicationOverrides": [
      {
        "source": "East US",
        "target": "West US",
        "minDelayMs": 200,
        "maxDelayMs": 500
      }
    ]
  },
  "management": {
    "port": 0
  },
  "databases": [
    {
      "id": "testdb",
      "containers": [
        {
          "id": "testcoll",
          "partitionKey": {
            "paths": [
              "/pk"
            ],
            "kind": "Hash",
            "version": 2
          },
          "partitionCount": 4,
          "throughput": 400,
          "seedItems": [
            {
              "partitionKey": [
                "pk1"
              ],
              "document": {
                "id": "1",
                "pk": "pk1",
                "value": 42
              }
            }
          ]
        }
      ]
    }
  ]
}
```

### Field contract

| Path | Type | Meaning |
| --- | --- | --- |
| `account.writeMode` | `"single"` or `"multi"` | Maps to `WriteMode`; the first region is the initial write region in single-write mode. |
| `account.consistency` | enum | `strong`, `boundedStaleness`, `session`, `consistentPrefix`, or `eventual`. |
| `account.perPartitionFailover` | boolean | Initial per-partition failover behavior; the management API may change it. |
| `account.throttling` | boolean | Enables per-partition RU/s enforcement. |
| `account.regions[].gatewayPort` | optional `u16` | Standard gateway listener; missing or `0` requests OS assignment. |
| `account.regions[].gateway20Port` | optional `u16` | Presence enables Gateway 2.0; `0` requests OS assignment. |
| `account.regions[].regionId` | optional `u64` | Auto-assigned by position when omitted. |
| `account.replication` | object | Default replication delay and buffer cap. |
| `account.replicationOverrides[]` | array | Per source-to-target replication overrides. |
| `management.port` | optional `u16` | Management listener; missing or `0` requests OS assignment. |
| `databases[].containers[].partitionKey` | object | Cosmos partition-key definition. |
| `databases[].containers[].partitionCount` | `u32` | Initial physical partition count. |
| `databases[].containers[].throughput` | `u32` | Provisioned RU/s used by throttling. |
| `databases[].containers[].seedItems[]` | array | Startup items with partition-key values and document bodies. |

Region names, effective region IDs, and explicit nonzero listener ports must be
unique. Region-name references are case-sensitive. Database IDs must be unique,
container IDs must be unique within a database, and resource IDs cannot be
empty or contain `/`, `\`, `?`, or `#`.

### Normal-write-path seeding

The host creates declared databases and containers, then seeds every item by
synthesizing one normal create-item request and sending it through
`execute_request`. It does not write store internals directly. Consequently,
startup data uses the same EPK routing, validation, RU accounting, LSN
advancement, and replication behavior as a client-issued write.

The host waits for scheduled seed replication to finish before publishing the
ready record. No client can observe a partially seeded startup topology through
the advertised endpoints.

### Configuration alternatives

- Deriving `serde` directly on driver configuration types was rejected because
  several fields are not serializable and doing so would leak host concerns
  into the driver.
- A second canonical YAML contract was rejected because independently evolving
  representations would make validation and automation ambiguous.
- Writing seed items directly into store internals was rejected because it
  would bypass the normal routing, charge, LSN, and replication semantics.

## Management REST API

The management API exposes only emulator controls that have no Cosmos gateway
equivalent. Database, container, offer, and item lifecycle are not duplicated
here; callers use the standard Cosmos endpoints or startup configuration.

All management endpoints use the resolved `managementEndpoint`, accept and
return JSON where a body is needed, and report conventional HTTP status codes.
Errors use a `{ "error": "..." }` body.

### Introspection

```text
GET /health
    -> 200 { "status": "ok" }

GET /account
    -> 200 { topology: regions, writeMode, consistency, offlineRegions,
             writeRegion, resolvedEndpoints, ... }
```

### Partition split and merge

Split supports three boundary-selection modes:

| Mode | Boundary | Store operation |
| --- | --- | --- |
| `midpoint` | Geometric midpoint of the partition's EPK range, independent of data distribution. | Existing `split_partition`. |
| `epk` | Explicit hex EPK between the partition's minimum-inclusive and maximum-exclusive boundaries. | Existing `split_partition_at_epk` plus host boundary parsing. |
| `storage` | EPK chosen to balance document count or storage across the children. | `split_partition_by_storage`. |

```text
POST /databases/{db}/containers/{coll}/partitions/{partitionId}/split
    body (optional): {
      "mode": "midpoint" | "epk" | "storage",
      "epk": "<hex EPK>",
      "progressionMode": "automatic" | "manual",
      "lockDurationMs": 500
    }
    -> 202 {
         "operationId": "op-split-123",
         "status": "Running",
         "phase": "Preparing"
       }

POST /databases/{db}/containers/{coll}/partitions/merge
    body: {
      "partitionIds": [4, 5],
      "progressionMode": "manual"
    }
    -> 202 {
         "operationId": "op-merge-456",
         "status": "Running",
         "phase": "Preparing"
       }

GET /operations/{operationId}
    -> 200 {
         "operationId": "...",
         "status": "Running" | "Succeeded" | "Failed",
         "phase": "Preparing" | "Swapping" | "Succeeded" | "Failed",
         "database": "testdb",
         "container": "testcoll",
         "parent": 0,
         "children": [4, 5],
         "mode": "storage",
         "splitEpk": "6A3C000000000000000000000000000000"
       }

POST /operations/{operationId}/advance
    -> 200 {
         "operationId": "...",
         "status": "Running",
         "phase": "Swapping"
       }
```

`midpoint` is the default split mode. `epk` requires the `epk` field; other
modes ignore it. Merge accepts exactly two adjacent partitions. The operation
result echoes the resolved split mode and concrete `splitEpk`, so callers can
observe the boundary selected for `midpoint` or `storage`.

Topology-changing requests create operation resources with deterministic
phases:

1. `Preparing`: source partitions remain available and replacements are hidden.
2. `Swapping`: source requests return `410/1007` and replacements remain
   hidden.
3. `Succeeded`: source partitions are removed and replacements are available.
4. `Failed`: the terminal error is retained and no further advancement is
   allowed.

`automatic` progression is the default. It enters `Swapping`, holds that phase
for `lockDurationMs`, and completes. The duration defaults to `0`, so it does
not guarantee an observable lock window, and its maximum is 60,000 ms.

`manual` progression advances exactly one phase per
`POST /operations/{operationId}/advance`, giving tests deterministic assertions
before, during, and after the topology change. Manual requests reject
`lockDurationMs` with `400 Bad Request`. Advancing an automatic or terminal
operation returns `409 Conflict`.

The operation state machine, rather than a generic lock endpoint, owns
partition locking so tests cannot create a lock state unrelated to an actual
topology transition.

### Region availability

```text
POST /regions/{region}/offline
    -> 200 { "region": "West US", "state": "offline" }

POST /regions/{region}/online
    -> 200 { "region": "West US", "state": "online" }
```

An offline region is removed from readable and writable locations and returns
`503 Service Unavailable` for direct data-plane requests. Returning it online
restores its eligibility. This outage control is distinct from the
add/remove/draining lifecycle described by the in-memory emulator specification.

### Runtime write-region failover

```text
POST /failover
    body: { "writeRegion": "West US" }
    -> 200 { "writeRegion": "West US" }
```

This changes the current write region for a single-write account. Subsequent
account reads advertise the new writable location, and clients reroute through
their normal metadata refresh path.

### Per-partition failover

```text
PUT /config/per-partition-failover
    body: { "enabled": true }
    -> 200 { "enabled": true }
```

### Replication pause and resume

```text
POST /regions/{region}/replication/pause
    -> 200 { "region": "West US", "replication": "paused" }

POST /regions/{region}/replication/resume
    -> 200 { "region": "West US", "replication": "resumed" }
```

Pausing replication to a target region buffers replicas up to the configured
cap. Exceeding that cap returns `429/3075`. This models a lagging or partially
unavailable replica without making the region unreachable.

### Management API alternatives

- A reserved path prefix on Cosmos data-plane ports was rejected because it is
  more collision-prone and less discoverable than a dedicated endpoint.
- Duplicating database or container CRUD was rejected because those operations
  already exist in the gateway contract.
- Generic partition lock and unlock endpoints were rejected because they could
  create states unrelated to a real topology operation.
- Relying only on a timed lock duration was rejected because timing-based tests
  are inherently racy; manual progression supplies deterministic phase control.

## Dynamic account topology

The store models region availability and write ownership as runtime account
topology shared by account discovery and request dispatch:

- The offline-region set controls which regions appear in readable and writable
  locations. Direct requests to an offline region return `503`.
- Single-write accounts maintain a current write-region selection used by both
  writable-location discovery and the write-region guard.
- Account reads expose state changes, so clients observe them through normal
  metadata refresh instead of an emulator-specific client hook.
- Replication pause and region outage remain separate failure models: one makes
  a reachable replica lag; the other makes an endpoint unavailable.

The management API mutates this state out of process. In-process tests use the
same store operations directly. Detailed membership, region-ID, session-token,
draining, seeding-policy, and write-mode semantics remain owned by the
[in-memory emulator specification](0021-in-memory-emulator.md).

### Known races

Topology mutation and the data plane are guarded by separate locks, so a
mutation is not atomic from a concurrent reader's perspective. These windows
are narrow, only reachable when a topology change races live traffic, and
closing them needs deliberate cross-lock coordination:

- **Seeding vs. publication (`add_region`).** `seeded_from` deep-copies the
  source region before the new region is published. A write committing after
  its source partition was copied but before publication is absent from the
  copy and is never replicated, so `SeedingPolicy::Immediate` can advertise
  permanently stale data.
- **Publication vs. version bump (`add_region`).** The topology is published
  before vector-clock versions are advanced, so an account read in that window
  can route a session request against partitions still exposing the pre-change
  version — the old token is compared rather than superseded.
  `remove_region` avoids this by bumping first, which is safe there;
  `add_region` cannot, because the new region's own partitions must be included
  in the bump.
- **Split interactions.** A region added mid-split can inherit a stale
  partition layout; `catch_up_from` can clobber writes acknowledged by a
  delayed-seeding region under multi-write; and an in-flight split can revert
  `advance_vector_clock_versions`.

### Dynamic-topology alternatives

- Treating replication pause as an outage was rejected because requests can
  still reach the region and account discovery still advertises it.
- Rebuilding the emulator for every topology change was rejected because
  clients could not test runtime refresh and recovery behavior.
- SDK-specific failover switches were rejected because they would bypass
  normal account discovery and routing.

The core also deliberately models add/remove lifecycle behavior validated
against live accounts: a draining or removed regional endpoint returns
`403/1008` while account discovery may continue to advertise it; region IDs are
never reused; re-added regions keep their original IDs; and membership changes
bump the session-token vector-clock version. The account read has no `_etag`,
so clients cannot use one to detect topology changes.

## HTTP/2, transport security, and authentication

Gateway 2.0 requires HTTP/2. For local no-auth scenarios, the host accepts
cleartext HTTP/2 prior-knowledge connections on configured loopback endpoints.
The Gateway 2.0 listener rejects HTTP/1.x. A standard gateway listener may
accept HTTP/1.1 or HTTP/2 so the driver's normal negotiation and fallback
behavior remains observable.

The driver permits `http://` Gateway 2.0 URLs only for recognized emulator
hosts. Production Gateway 2.0 endpoints remain HTTPS-only. The existing
`Http2Only` transport supplies prior knowledge, and the existing account probe
decides whether standard gateway traffic uses HTTP/2 or falls back to HTTP/1.1.

Authentication and TLS are host-boundary concerns. The supported modes and
trust requirements are defined by the
[emulator transport security and authentication specification](0023-emulator-transport-security-and-authentication.md).
The store and operation handlers receive only requests that have passed host
policy and do not depend on certificate or identity libraries.

### Transport alternatives

- Supporting only HTTP/1.1 was rejected because it cannot exercise Gateway 2.0.
- Requiring TLS for every local test was rejected because certificate setup
  would obscure tests unrelated to transport security.
- Adding a client option solely to force prior knowledge was rejected because
  the existing HTTP/2-only transport already provides that behavior.

## Validation and delivery

Hosted-emulator validation must exercise the existing emulator suites through
real network clients in both Gateway V1 and Gateway 2.0 modes. Test setup:

1. builds and starts the host with a provisioning configuration;
2. parses the single ready record from stdout;
3. waits for `GET /health` on the resolved management endpoint;
4. builds the client connection information from the reported account
   endpoint; and
5. runs the suites using the hosted-emulator test category.

Peer SDKs validate their own HTTP/2, h2c, TLS, and RNTBD interoperability before
claiming compatibility. A host implementation must not infer cross-language
compatibility solely from the Rust test suite.

## Deferred work

- A maintained YAML parser may be added if demand warrants, but it must map to
  the canonical host-owned JSON model.
- HTTP/1.1 fallback matching may be broadened if end-to-end h2c validation
  reveals an explicit incompatibility the existing matcher misses.
- Additional management controls, such as throttling toggles, forced
  session-not-available, or replication-delay overrides, may be added when a
  concrete cross-SDK test requires them.

## Related documents

- [In-memory emulator](0021-in-memory-emulator.md) -- store, operation, and
  dynamic membership semantics.
- [Emulator transport security and authentication](0023-emulator-transport-security-and-authentication.md)
  -- TLS and credential validation at the host boundary.
- [Gateway V2](0011-gateway-v2.md) -- client-side Gateway 2.0 behavior.
- [Operation and transport pipelines](0005-operation-and-transport-pipelines.md)
  -- driver execution and transport layering.
