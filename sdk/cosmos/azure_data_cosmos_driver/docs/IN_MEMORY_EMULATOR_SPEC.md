# In-Memory Emulator Specification for `azure_data_cosmos_driver`

**Status**: Draft
**Date**: 2026-03-16 (last updated 2026-03-25)
**Authors**: (team)

---

## Table of Contents

1. [Goals & Motivation](#1-goals--motivation)
2. [Feature Gating](#2-feature-gating)
3. [Architectural Overview](#3-architectural-overview)
4. [Virtual Account Configuration](#4-virtual-account-configuration)
5. [In-Memory Store Design](#5-in-memory-store-design)
6. [Request Interception & Dispatch](#6-request-interception--dispatch)
7. [Point Operation Emulation](#7-point-operation-emulation)
8. [Response Generation](#8-response-generation)
9. [Session Token & LSN Tracking](#9-session-token--lsn-tracking)
10. [ETag & Optimistic Concurrency](#10-etag--optimistic-concurrency)
11. [Request Unit (RU) Charging Model](#11-request-unit-ru-charging-model)
12. [Multi-Region Emulation](#12-multi-region-emulation)
13. [Error Emulation](#13-error-emulation)
14. [Supported Control-Plane Operations](#14-supported-control-plane-operations)
15. [Unsupported Operations](#15-unsupported-operations)
16. [Validation Test Framework](#16-validation-test-framework)
17. [Allowlisted Header Variance](#17-allowlisted-header-variance)
18. [Module & File Layout](#18-module--file-layout)
19. [Usage API & Public API Surface](#19-usage-api--public-api-surface)
20. [Open Questions (Resolved)](#20-open-questions-resolved)
21. [Throughput Throttling](#21-throughput-throttling)
22. [V2 Vector-Clock Session Tokens](#22-v2-vector-clock-session-tokens)
23. [Partition Split & Merge](#23-partition-split--merge)

---

## 1. Goals & Motivation

### Why

Integration testing of the `azure_data_cosmos_driver` pipeline currently requires a live Cosmos DB
account or the Azure Cosmos DB Emulator (Docker/local). Both have drawbacks:

- **Live account**: Cost, latency, network dependency, flaky in CI.
- **Docker emulator**: Heavy, slow startup, limited multi-region support, no session-token
  control for edge-case testing.

An **in-memory emulator** that intercepts requests at the `HttpClient` transport boundary provides:

1. **Zero external dependencies**: Tests run without network, accounts, or Docker.
2. **Deterministic behavior**: No eventual consistency surprises, exact control over timing.
3. **Edge-case testing**: Force 404/1002 (ReadSessionNotAvailable), simulate replication lag,
   inject region-specific delays — all impossible with the real emulator.
4. **CI-friendly**: Fast, parallel-safe, no provisioning scripts.
5. **Validation mode**: The same test cases can run against a live account to verify the
   emulator matches real behavior, catching drift early.

### Non-Goals (This Phase)

- Query execution (SQL queries return a hard-coded error).
- Batch / Bulk / Patch operations (return hard-coded errors).
- Gateway 2.0 transport mode (skip for now — will come later).
- Change feed.
- Stored procedures / triggers / UDFs.
- Cross-partition feed reads (`ReadFeed`).

---

## 2. Feature Gating

All in-memory emulator code is gated behind the `in_memory_emulator` feature flag, following
the same pattern as `fault_injection` in `azure_data_cosmos`.

### Cargo.toml

```toml
[features]
in_memory_emulator = ["dep:tokio"]   # enables module, implies tokio for async replication/split/merge
```

### Module Declaration (lib.rs)

```rust
#[cfg(feature = "in_memory_emulator")]
pub mod in_memory_emulator;
```

When the feature is not enabled, the entire module tree is excluded from compilation.
Default builds are unaffected.

---

## 3. Architectural Overview

### Injection Point

The in-memory emulator implements `azure_core::http::HttpClient`, replacing the real HTTP
transport at the bottom of the stack. This means:

- The full 7-stage operation pipeline (endpoint resolution, session routing, retry, failover,
  diagnostics) executes normally.
- The transport pipeline (header application, request signing, deadline enforcement, 429 retry)
  executes normally.
- Only the final HTTP I/O is replaced by the in-memory store.

### Layering with Fault Injection

When both fault injection and the in-memory emulator are active:

```text
AdaptiveTransport
  └─ Arc<dyn HttpClient>
       └─ FaultInjectingHttpClient        (optional, evaluates fault rules first)
            └─ InMemoryEmulatorHttpClient  (intercepts all requests, never hits network)
```

Fault injection can inject errors *before* the emulator processes the request, enabling tests
that verify retry/failover behavior against the in-memory store.

### High-Level Request Flow

```text
1. CosmosDriver::execute_operation(operation, options)
2. Operation pipeline: resolve endpoint, build TransportRequest
3. Transport pipeline: apply headers, sign request, enforce deadline
4. AdaptiveTransport::send(request)
5. InMemoryEmulatorHttpClient::execute_request(request)
   ├─ Parse URL path → determine resource type + operation type
   ├─ Parse headers → extract partition key, session token, preconditions
   ├─ Route to target region's store (based on endpoint URL)
   ├─ Execute point operation against in-memory store
   ├─ Advance LSN (writes only), compute RU charge
   ├─ Build synthetic HTTP response with all required Cosmos headers
   └─ Return AsyncRawResponse
6. Transport pipeline: map response → TransportResult
7. Operation pipeline: evaluate result, complete or retry
```

---

## 4. Virtual Account Configuration

`VirtualAccountConfig` configures the emulated Cosmos DB account:

- **Regions**: Ordered list of `VirtualRegion` (name + gateway URL + region_id). First is hub.
- **Write mode**: Single-write (one designated write region) or multi-write (all regions).
- **Consistency**: Default consistency level (Session, Strong, Eventual, etc.).
- **Replication**: Configurable delay range `[min_lag, max_lag]` (default 20–50ms random).
- **RU model**: Configurable RU charging rates per size bucket.
- **Throttling**: Optional per-partition throughput enforcement (429/3200 when exceeded).

### Replication Config

```rust
ReplicationConfig::default()                   // 20–50ms random lag
ReplicationConfig::immediate()                 // Zero lag (shared store behavior)
ReplicationConfig::fixed(Duration::from_ms(100)) // Deterministic 100ms
ReplicationConfig::range(min, max)             // Custom range
```

#### Per-Direction Overrides

Replication delay can be overridden per source → target region pair. The global
config acts as the default; per-direction overrides take precedence.

```rust
let config = VirtualAccountConfig::new(regions)
    .with_replication_config(ReplicationConfig::default())
    // East US → West Europe: slow link
    .with_replication_override(
        "East US", "West Europe",
        ReplicationConfig::range(Duration::from_millis(200), Duration::from_millis(500)),
    )
    // West US → East US: instant
    .with_replication_override(
        "West US", "East US",
        ReplicationConfig::immediate(),
    );
```

### Account Properties Response

The emulator serves `GET /` requests with synthesized account properties from config:

- `readableLocations` → all regions
- `writableLocations` → all regions (MWR) or just write region (SWR)
- `enableMultipleWriteLocations` → from config
- `defaultConsistencyLevel` → from config

---

## 5. In-Memory Store Design

### Storage Hierarchy

```text
EmulatorStore
├── config: VirtualAccountConfig
├── rid_generator: RidGenerator                         # Monotonic counter per resource level
└── regions: HashMap<RegionName, RegionStore>
    └── RegionStore
        ├── databases: RwLock<HashMap<String, DatabaseMetadata>>
        └── containers: RwLock<HashMap<(String, String), ContainerState>>
            └── ContainerState
                ├── metadata: ContainerMetadata         # id, _rid, PK definition, throughput, etc.
                ├── next_partition_id: AtomicU32         # Counter for split/merge child IDs
                └── physical_partitions: Vec<PhysicalPartition>
                    └── PhysicalPartition
                        ├── id: u32                     # Partition key range ID
                        ├── epk_range: (Epk, Epk)       # [min_inclusive, max_exclusive)
                        ├── lsn: AtomicU64              # Per-partition LSN counter
                        ├── vector_clock_version: AtomicU64  # Topology version (incremented on merge)
                        ├── documents: RwLock<BTreeMap<Epk, BTreeMap<String, StoredDocument>>>
                        ├── session_state: SessionState # Forced-unavailability flag
                        ├── parents: Vec<u32>           # Parent partition IDs (after split/merge)
                        ├── locked_until: RwLock<Option<Instant>>  # Split/merge lock
                        └── throughput_tracker: Option<ThroughputTracker>  # Per-partition RU budget
```

### Resource Metadata

**DatabaseMetadata**: Stores the database-level system properties.

| Field   | Type     | Description                                                    |
| ------- | -------- | -------------------------------------------------------------- |
| `id`    | `String` | User-assigned database name                                    |
| `_rid`  | `String` | Hierarchical RID (4 bytes, base64-encoded); encodes db_id only |
| `_ts`   | `u64`    | Last-modified timestamp (Unix epoch seconds)                   |
| `_self` | `String` | Self-link: `dbs/{_rid}/`                                       |
| `_etag` | `String` | Quoted UUID, regenerated on metadata changes                   |

**ContainerMetadata**: Stores the container-level system properties and configuration.

| Field                       | Type                     | Description                                                                                        |
| --------------------------- | ------------------------ | -------------------------------------------------------------------------------------------------- |
| `id`                        | `String`                 | User-assigned container name                                                                       |
| `_rid`                      | `String`                 | Hierarchical RID (8 bytes, base64-encoded); encodes parent db (4B) + collection (4B, high bit set) |
| `_ts`                       | `u64`                    | Last-modified timestamp (Unix epoch seconds)                                                       |
| `_self`                     | `String`                 | Self-link: `dbs/{db_rid}/colls/{coll_rid}/`                                                        |
| `_etag`                     | `String`                 | Quoted UUID                                                                                        |
| `partition_key`             | `PartitionKeyDefinition` | Paths, kind (Hash/Range), version                                                                  |
| `partition_count`           | `u32`                    | Number of physical partitions (default 4)                                                          |
| `provisioned_throughput_ru` | `Option<u32>`            | Provisioned RU/s (None = no limit; minimum 400 when set)                                           |

**PartitionKeyRangeMetadata**: Stores per-partition-key-range metadata, exposed via the
`/dbs/{db}/colls/{coll}/pkranges` feed. Each physical partition has a corresponding
`PartitionKeyRangeMetadata`.

| Field                 | Type          | Description                                                                                                             |
| --------------------- | ------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `id`                  | `String`      | Partition key range ID (stringified `u32`, e.g. `"0"`)                                                                  |
| `_rid`                | `String`      | Hierarchical RID (16 bytes, base64-encoded); encodes parent db (4B) + collection (4B) + pkrange (8B, type nibble `0x5`) |
| `_self`               | `String`      | Self-link: `dbs/{db}/colls/{coll}/pkranges/{id}/`                                                                       |
| `_etag`               | `String`      | Quoted UUID                                                                                                             |
| `_ts`                 | `u64`         | Last-modified timestamp (Unix epoch seconds)                                                                            |
| `_lsn`                | `u64`         | Current LSN of this partition                                                                                           |
| `min_inclusive`       | `Epk`         | Lower EPK bound (inclusive), e.g. `Epk::MIN`                                                                            |
| `max_exclusive`       | `Epk`         | Upper EPK bound (exclusive), e.g. `Epk::MAX`                                                                            |
| `status`              | `String`      | `"online"` (or absent during split/merge lock)                                                                          |
| `parents`             | `Vec<String>` | Parent partition IDs after split/merge (empty for initial partitions)                                                   |
| `rid_prefix`          | `u32`         | Partition-local RID prefix for document allocation                                                                      |
| `throughput_fraction` | `f64`         | `1.0 / partition_count`                                                                                                 |
| `vectorClockVersion`  | `u64`         | Topology version (0 initially; incremented on merge, preserved on split)                                                |

The PKRanges feed is served at `GET /dbs/{db}/colls/{coll}/pkranges` and returns all
ranges for the container. When used as a change feed (with `A-IM: Incremental feed` and
`If-None-Match` headers), the emulator returns only changed ranges or `304 Not Modified`
if the ETag matches (no topology change).

```json
{
  "PartitionKeyRanges": [
    {
      "id": "0",
      "minInclusive": "",
      "maxExclusive": "3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
      "_rid": "...",
      "_self": "dbs/AAA=/colls/AAAB/pkranges/0",
      "_etag": "\"00000000-0000-0000-0000-000000000000\"",
      "_ts": 1711152000,
      "_lsn": 5,
      "ridPrefix": 0,
      "throughputFraction": 0.25,
      "status": "online",
      "parents": [],
      "vectorClockVersion": 0
    }
  ],
  "_rid": "...",
  "_count": 4
}
```

**StoredDocument**: Raw JSON body with injected system properties.

| Field   | Type                | Description                                                                                                              |
| ------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `body`  | `serde_json::Value` | Full document JSON (including system properties)                                                                         |
| `id`    | `String`            | Document `id` field                                                                                                      |
| `_rid`  | `String`            | Hierarchical RID (16 bytes, base64-encoded); encodes parent db (4B) + collection (4B) + document (8B, type nibble `0x0`) |
| `_etag` | `String`            | Quoted UUID, regenerated on every write                                                                                  |
| `_ts`   | `u64`               | Last-modified timestamp                                                                                                  |
| `_self` | `String`            | Self-link: `dbs/{db}/colls/{coll}/docs/{doc_rid}/`                                                                       |
| `lsn`   | `u64`               | LSN at write time (from the writing partition)                                                                           |
| `epk`   | `Epk`               | Effective partition key (hex-encoded hash)                                                                               |

### Physical Partition Model

Each container is divided into **N physical partitions** (default 4, configurable per
container via `ContainerConfig`). Each physical partition is responsible for a contiguous
range of EPK hash values:

```text
Container (4 partitions, EPK range = "" .. "FF")
├── PhysicalPartition 0: ["",              "3FFF...FF")   ← PKRange ID 0
├── PhysicalPartition 1: ["3FFF...FF",     "7FFF...FF")   ← PKRange ID 1
├── PhysicalPartition 2: ["7FFF...FF",     "BFFF...FF")   ← PKRange ID 2
└── PhysicalPartition 3: ["BFFF...FF",     "FF")          ← PKRange ID 3
```

The hex-encoded EPK space `[Epk::MIN, Epk::MAX)` is divided into N equal-width ranges.
Range boundaries are computed by dividing the numeric hash space and converting back to hex.

### The `Epk` Newtype

`Epk` is a newtype around `String` that represents an effective partition key as an
uppercase hex-encoded hash string. It provides type safety, preventing accidental
confusion with other string-typed fields (document IDs, RIDs, etc.).

```rust
/// Effective partition key — uppercase hex-encoded hash string.
///
/// Implements `Ord` via lexicographic comparison of the underlying hex string,
/// which preserves EPK ordering for BTreeMap-based range scans.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epk(String);

impl Epk {
    /// Minimum inclusive EPK (empty string — corresponds to empty partition key).
    pub const MIN: Epk = Epk(String::new());

    /// Maximum exclusive EPK (`"FF"` — corresponds to infinity partition key).
    pub fn max() -> Epk { Epk("FF".to_string()) }

    /// Returns the underlying hex string.
    pub fn as_str(&self) -> &str { &self.0 }
}

impl AsRef<str> for Epk {
    fn as_ref(&self) -> &str { &self.0 }
}
```

All EPK hash computation functions return `Epk`. All EPK range boundaries, document EPK
fields, and BTreeMap keys use `Epk`.

### EPK Hash Computation

Cosmos DB uses a **modified Murmur hash** algorithm, and the specific variant depends on
the `PartitionKeyDefinition`'s kind and version. The emulator must include a **self-contained
implementation** of all hash variants — it cannot delegate to `azure_data_cosmos::hash`
because `azure_data_cosmos` depends on (or will depend on) `azure_data_cosmos_driver`,
which would create a cyclic dependency.

The reference implementations are:
- **Rust (partial)**: `azure_data_cosmos::hash` (`sdk/cosmos/azure_data_cosmos/src/hash.rs`)
  — covers V1, V2, and binary encoding but lacks MultiHash.
- **Java (complete)**: [`PartitionKeyInternalHelper`](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/implementation/routing/PartitionKeyInternalHelper.java)
  and related files in the `routing` package (`StringPartitionKeyComponent`,
  `NumberPartitionKeyComponent`, `MurmurHash3_128`, `MurmurHash3_32`, `Int128`, `UInt128`).

The driver's `epk` module must implement all three algorithms (V1, V2, MultiHash),
including the component serialization logic (`write_for_hashing_v1/v2`,
`write_for_binary_encoding_v1`), the MurmurHash3 32-bit and 128-bit hash functions,
and the hex-encoding output.

#### Algorithm Selection

| PK Kind     | PK Version | Algorithm                         | Output                                                                | Notes                                  |
| ----------- | ---------- | --------------------------------- | --------------------------------------------------------------------- | -------------------------------------- |
| `Hash`      | 1 (V1)     | MurmurHash3 32-bit                | Hex-encoded binary (hash-as-number prepended to truncated components) | Legacy; strings truncated to 100 bytes |
| `Hash`      | 2 (V2)     | MurmurHash3 128-bit               | 16-byte hash, top 2 bits cleared, hex-encoded                         | Default for new containers             |
| `MultiHash` | 2          | MurmurHash3 128-bit per component | Concatenated per-component 16-byte hashes                             | Hierarchical PK support                |

#### Component Type Markers

Each partition key component is serialized with a leading type marker byte:

| Type      | Marker | Notes                                    |
| --------- | ------ | ---------------------------------------- |
| Undefined | `0x00` | Component not present                    |
| Null      | `0x01` | JSON `null`                              |
| False     | `0x02` | Boolean `false`                          |
| True      | `0x03` | Boolean `true`                           |
| Number    | `0x05` | Followed by IEEE 754 `f64` little-endian |
| String    | `0x08` | Followed by UTF-8 bytes + suffix byte    |
| Infinity  | `0xFF` | Used for range boundaries                |

#### Component Serialization for Hashing

**`write_for_hashing_v2`** (used by V2 and MultiHash):
- Bool/Null/Undefined/Infinity: type marker only.
- Number: marker `0x05` + 8 bytes (f64 LE).
- String: marker `0x08` + full UTF-8 bytes (no truncation) + suffix `0xFF`.

**`write_for_hashing_v1`** (used by V1):
- Same as V2 except: string suffix is `0x00`, strings truncated to 100 bytes.

#### Component Serialization for Binary Encoding (V1)

Used to build the final EPK binary for V1 after the hash-as-number prefix:
- Number: marker `0x05` + variable-length ordering-preserving encoding of `f64`
  (sign-magnitude to unsigned conversion, 7-bit continuation encoding).
- String: marker `0x08` + each byte `+1` (wrapping), up to 100 bytes if short
  (with `0x00` terminator) or 101 bytes if long (no terminator).
- Bool/Null/Undefined/Infinity: type marker only.

#### V2 Hash (Most Common)

1. Each PK component is serialized via `write_for_hashing_v2`.
2. Concatenated bytes are hashed with MurmurHash3 128-bit (x64 variant, seed 0).
3. Result bytes are reversed (little-endian to big-endian).
4. Top 2 bits of the first byte are cleared (`& 0x3F`) — ensures EPK < `"FF"`.
5. Final EPK is the uppercase hex-encoded 16-byte hash.

#### V1 Hash (Legacy)

1. Each PK component is serialized via `write_for_hashing_v1`.
2. Concatenated bytes are hashed with MurmurHash3 32-bit (seed 0).
3. The 32-bit hash is cast to `f64` (intentional precision loss mirroring other SDKs).
4. Binary encoding: `[hash_as_number] + [truncated_components]` using V1 binary rules.
5. Final EPK is the uppercase hex-encoded binary.

#### MultiHash (Hierarchical Partition Keys)

1. Each PK component is independently serialized via `write_for_hashing_v2`.
2. Each component's bytes are independently hashed with MurmurHash3 128-bit (seed 0).
3. Each 16-byte hash is reversed and top 2 bits cleared (same as V2).
4. Per-component 16-byte hashes are concatenated in order.
5. Final EPK is the uppercase hex-encoded concatenation.

For hierarchical PK with N components, the EPK is `N * 32` hex characters long.

#### EPK Range Boundaries

EPK ranges use `Epk` comparison (lexicographic on the underlying uppercase hex string):
- **Min inclusive**: `Epk::MIN` — empty string, corresponds to empty partition key
- **Max exclusive**: `Epk::MAX` — `"FF"`, corresponds to infinity partition key

The emulator includes its own full EPK hash implementation (`epk` module) to avoid
a cyclic dependency on `azure_data_cosmos`. The emulator selects the algorithm based on
the container's `PartitionKeyDefinition` kind and version.

**Routing flow**:

1. Extract partition key value from request (`x-ms-documentdb-partitionkey` header or
   document body path).
2. Look up the container's `PartitionKeyDefinition` to determine kind + version.
3. Compute EPK string using the appropriate algorithm (V1, V2, or MultiHash).
4. Find the physical partition whose `[min_inclusive, max_exclusive)` range contains the EPK.
5. All operations on this `(partition_key, id)` pair route to that physical partition.

### Document Storage Within a Physical Partition

Documents within a physical partition are organized as a two-level map:

```text
BTreeMap<Epk, BTreeMap<String, StoredDocument>>
│         │              │
│         │              └─ item id → StoredDocument
│         └─ Epk (logical partition)
└─ Sorted by Epk (Ord) for future range scan support (ReadFeed, ChangeFeed, Query)
```

Using `BTreeMap` (rather than `HashMap`) preserves EPK ordering, which is necessary for
future non-point operations like ReadFeed and ChangeFeed that iterate over EPK ranges.
`Epk` implements `Ord` via lexicographic comparison of the underlying uppercase hex string.

### Hierarchical RID Encoding

Resource IDs (`_rid`) follow the hierarchical binary encoding used by Cosmos DB, based on
the [Java SDK `ResourceId` implementation](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/implementation/ResourceId.java).
Each child resource encodes its parent's RID, enabling parent identification from any
child's RID.

#### Binary Layout

| Resource          | Total Bytes | Layout                                                       |
| ----------------- | ----------- | ------------------------------------------------------------ |
| Database          | 4           | `[db_id: u32 BE]`                                            |
| Collection        | 8           | `[db_id: u32 BE] [coll_id: u32 BE]` (high bit set on byte 4) |
| Document          | 16          | `[db_id: u32 BE] [coll_id: u32 BE] [doc_id: u64 BE]`         |
| PartitionKeyRange | 16          | `[db_id: u32 BE] [coll_id: u32 BE] [pkr_id: u64 BE]`         |

- **`db_id`**: Monotonically increasing `u32` counter per emulator instance.
- **`coll_id`**: Monotonically increasing `u32` counter. High bit of the first byte is set
  to `1` to distinguish collections from users (which share the same position but have the
  high bit unset).
- **`doc_id`**: Monotonically increasing `u64` counter per collection. The top nibble of the
  last byte encodes the child resource type (`0x0` = Document, `0x5` = PartitionKeyRange,
  `0x8` = StoredProcedure, etc.).
- All integers are big-endian.

#### Serialization

The binary RID is base64-encoded with `/` replaced by `-` to produce the string form.
The encoded string length is always a multiple of 4 (padded with `=`).

#### Example

```text
Database "mydb":           db_id = 1
  → binary:  [00 00 00 01]
  → base64:  "AAAAAQ=="

Collection "mycoll":       coll_id = 1 (with high bit: 0x80000001)
  → binary:  [00 00 00 01] [80 00 00 01]
  → base64:  "AAAAAYAAAAEa"  (illustrative)

Document "doc1":           doc_id = 1 (type nibble 0x0)
  → binary:  [00 00 00 01] [80 00 00 01] [00 00 00 00 00 00 00 10]
  → base64:  "AAAAAYAAAAEaAAAAAAAAAAEQ"  (illustrative)
```

The `RidGenerator` maintains atomic counters for each level, ensuring uniqueness and
preserving the parent-child encoding relationship.

### Cross-Region Replication

When a write succeeds in a region:

1. Document stored locally in the target physical partition. Partition LSN advanced.
2. Delay sampled from the applicable `ReplicationConfig` (per-direction override checked
   first, then global default; default 20–50ms random).
3. After delay, document copied to the corresponding physical partition in each other
   region's store. The target physical partition is identified by the same EPK hash —
   all regions have identical partition layouts for a given container.
4. Target partition's LSN set to `max(current_lsn, replicated_doc.lsn)` — **not**
   independently incremented. The replicated document carries the source region's LSN.

Only writes advance LSN. Reads never change LSN.

### Concurrency Model

Each physical partition is independently serialized:

- **`RwLock`** on the document `BTreeMap` — multiple concurrent reads, exclusive writes.
- **`AtomicU64`** for the LSN counter — lock-free increment on writes.
- Operations on different physical partitions execute concurrently without contention.
- Database/container metadata maps use separate `RwLock`s.

---

## 6. Request Interception & Dispatch

### URL Path Parsing

| Pattern                            | Resource Type                      |
| ---------------------------------- | ---------------------------------- |
| `/`                                | Account                            |
| `/dbs`                             | DatabaseFeed (create)              |
| `/dbs/{db}`                        | Database (read/delete)             |
| `/dbs/{db}/colls`                  | ContainerFeed (create)             |
| `/dbs/{db}/colls/{coll}`           | Container (read/delete)            |
| `/dbs/{db}/colls/{coll}/pkranges`  | PartitionKeyRangeFeed (read feed)  |
| `/dbs/{db}/colls/{coll}/docs`      | DocumentFeed (create/upsert/query) |
| `/dbs/{db}/colls/{coll}/docs/{id}` | Document (read/replace/delete)     |

### Operation Resolution

| HTTP Method | Path                        | Headers                           | Operation       |
| ----------- | --------------------------- | --------------------------------- | --------------- |
| `GET`       | `/`                         | —                                 | ReadAccount     |
| `POST`      | `/dbs`                      | —                                 | CreateDatabase  |
| `GET`       | `/dbs/{db}`                 | —                                 | ReadDatabase    |
| `DELETE`    | `/dbs/{db}`                 | —                                 | DeleteDatabase  |
| `POST`      | `/dbs/{db}/colls`           | —                                 | CreateContainer |
| `GET`       | `/dbs/{db}/colls/{coll}`    | —                                 | ReadContainer   |
| `DELETE`    | `/dbs/{db}/colls/{coll}`    | —                                 | DeleteContainer |
| `GET`       | `.../colls/{coll}/pkranges` | —                                 | ReadPKRanges    |
| `POST`      | `.../docs`                  | —                                 | Create          |
| `POST`      | `.../docs`                  | `x-ms-documentdb-is-upsert: True` | Upsert          |
| `POST`      | `.../docs`                  | `x-ms-documentdb-query` present   | Query (→ 501)   |
| `GET`       | `.../docs/{id}`             | —                                 | Read            |
| `PUT`       | `.../docs/{id}`             | —                                 | Replace         |
| `DELETE`    | `.../docs/{id}`             | —                                 | Delete          |

### Region Routing

Request URL host matched against `VirtualRegion.gateway_url` to determine target region.

---

## 7. Point Operation Emulation

### Content-Response-on-Write

The `x-ms-cosmos-populate-content-response-on-write` header controls whether write
responses (Create, Replace, Upsert) include the document body:

- **Header absent or `false`**: Write responses return an empty body with headers only
  (status code, ETag, session token, RU charge, etc.).
- **Header `true`**: Write responses include the full document body with system properties.

The emulator respects this header on all write operations. Delete always returns an empty
body regardless of this header.

### Create (POST → 201 Created)

1. Extract `id` from JSON body, partition key from header.
2. Compute EPK hash, route to physical partition.
3. Conflict check → 409 if `(pk, id)` exists in the logical partition.
4. Generate `_rid`, `_etag`, `_ts`, `_self` system properties.
5. Advance partition LSN, store document, trigger replication.
6. Return body per content-response-on-write header.

### Read (GET → 200 OK)

1. Compute EPK hash, route to physical partition.
2. Session consistency check: if request session token LSN > partition LSN → 404/1002.
3. Lookup by `(epk, id)` in the physical partition → 404 if not found.
4. Return stored document body with ETag.

### Replace (PUT → 200 OK)

1. Compute EPK hash, route to physical partition.
2. Lookup existing → 404 if not found.
3. `If-Match` precondition → 412 on ETag mismatch.
4. Generate new ETag, advance LSN, replace document, trigger replication.
5. Return body per content-response-on-write header.

### Upsert (POST+is-upsert → 201 or 200)

1. Compute EPK hash, route to physical partition.
2. Check if `(epk, id)` exists.
3. Not exists → create (201). Exists → replace (200).
4. Upsert RU: create charge if 201, replace charge (1.5×) if 200.
5. Return body per content-response-on-write header.

### Delete (DELETE → 204 No Content)

1. Compute EPK hash, route to physical partition.
2. Lookup existing → 404 if not found.
3. `If-Match` precondition → 412 on ETag mismatch.
4. Remove document, advance LSN, replicate tombstone.
5. Always returns empty body.

---

## 8. Response Generation

### Required Response Headers

| Header                | Source                                                               |
| --------------------- | -------------------------------------------------------------------- |
| `x-ms-activity-id`    | Echo from request or generate UUID                                   |
| `x-ms-request-charge` | Computed per RU model                                                |
| `x-ms-session-token`  | V2 format: `{pkrangeId}:{version}#{globalLSN}#{regionId}={localLSN}` |
| `etag`                | Stored/generated ETag                                                |
| `content-type`        | `application/json`                                                   |
| `date`                | Current UTC (RFC 1123)                                               |
| `x-ms-version`        | `2020-07-15`                                                         |
| `x-ms-substatus`      | Only on specific errors (1002, 3, 1007, 3200, etc.)                  |
| `x-ms-item-count`     | `1` for point reads                                                  |
| `x-ms-retry-after-ms` | Only on 429/3200 throttle responses (milliseconds to wait)           |

### Error Response Body

```json
{"code": "NotFound", "message": "Entity with the specified id does not exist in the system."}
```

### System Properties Injected into Document Body

`_rid`, `_self`, `_etag`, `_attachments`, `_ts` — injected on every write response.

---

## 9. Session Token & LSN Tracking

### LSN Model

- Each **physical partition** in each region has an `AtomicU64` LSN counter.
- Each **physical partition** also has an `AtomicU64` **vector clock version** (starts at 0).
- **Only writes advance LSN** (increment by 1). Reads never change LSN.
- Replication applies source LSN via `fetch_max` — target does not independently increment.
- A container with N physical partitions has N independent LSN counters per region.

### Session Token Format (V2)

The emulator emits V2 vector-clock session tokens:

```text
{pkrange_id}:{version}#{globalLSN}#{regionId}={localLSN}
```

Where:
- `pkrange_id` — physical partition's partition key range ID (0, 1, 2, …)
- `version` — vector clock version (topology version; incremented on merge)
- `globalLSN` — the partition's current LSN at the time of the operation
- `regionId` — numeric ID of the region that performed the operation
- `localLSN` — same as `globalLSN` (per-region tracking; always matches `globalLSN` for
  the writing region)

Example: `0:0#5#0=5` means partition 0, version 0, LSN 5, region 0 at LSN 5.

Composite session tokens (multiple partition key ranges in one token, separated by `,`)
are supported for response headers: `0:0#5#0=5,1:0#3#0=3`.

### V1 Backward Compatibility

The emulator accepts V1 tokens (`{pkrangeId}:-1#{lsn}`) on incoming requests for backward
compatibility. V1 tokens are parsed as version=0, globalLSN=lsn, no region progress.

### Session Consistency Enforcement

When account consistency is `Session` and request includes `x-ms-session-token`:

1. Parse the token to extract partition key range ID, version, and globalLSN.
2. Identify the target physical partition (by EPK hash of the partition key).
3. **Version check**: if the token's version > partition's current version → 404/1002.
   Higher version means a topology change (merge) that hasn't been observed yet.
4. **LSN check**: if same version and token's globalLSN > partition's current LSN →
   404/1002 (ReadSessionNotAvailable).
5. Otherwise proceed normally.

### Vector Clock Version

- **Split**: does NOT change vector_clock_version. Child partitions inherit the parent's
  version.
- **Merge**: increments vector_clock_version by 1 on the child partition
  (`max(parent_versions) + 1`).
- **Region add/remove**: would increment version (not yet implemented).

### Forced Session Unavailability

```rust
store.force_session_not_available("East US", r#"["pk1"]"#);
```

One-shot: next read to the physical partition containing the given partition key returns
404/1002 regardless of LSN, then resets.

---

## 10. ETag & Optimistic Concurrency

- New quoted UUID ETag generated on every write.
- **If-Match**: Compared on Replace/Delete → 412 on mismatch.
- **If-None-Match**: Reserved for future use (create-if-not-exists pattern).
- ETag returned in both `etag` header and `_etag` body property.

---

## 11. Request Unit (RU) Charging Model

### Size Buckets (Doubling)

| Bucket | Size Range | Read RU | Create RU | Replace/Delete RU |
| ------ | ---------- | ------- | --------- | ----------------- |
| 1      | 0–1 KB     | 1.0     | 5.8       | 8.7 (1.5×5.8)     |
| 2      | 1–2 KB     | 2.0     | 11.6      | 17.4              |
| 3      | 2–4 KB     | 4.0     | 23.2      | 34.8              |
| ...    | doubles    | ...     | ...       | ...               |

### Indexing Charge

Writes add `0.3 RU × (number of top-level JSON properties)`.

### Upsert

- 201 (created): create base charge.
- 200 (replaced): 1.5× create base charge.

### Configurability

`RuChargingModel` struct with customizable base rates:

```rust
RuChargingModel {
    read_base_ru: 1.0,
    create_base_ru: 5.8,
    write_multiplier: 1.5,
    indexing_ru_per_property: 0.3,
}
```

### Throughput Enforcement

See [Section 21 — Throughput Throttling](#21-throughput-throttling) for per-partition
RU/s enforcement with 429/3200 responses.

---

## 12. Multi-Region Emulation

### Write Region Enforcement

- **Single-write**: Only the designated write region accepts writes. Others return 403/3
  (WriteForbidden), triggering the driver's failover logic.
- **Multi-write**: All regions accept writes. Last-writer-wins by `_ts`.

### Replication

- Default: random 20–50ms delay per replicated write.
- `ReplicationConfig::immediate()`: synchronous, zero lag.
- `ReplicationConfig::fixed(lag)`: deterministic for testable assertions.
- Per-direction overrides via `with_override(source, target, config)` — checked
  before the global default for each `(source → target)` replication.
- Target region LSN = `max(current, source_doc.lsn)`.

---

## 13. Error Emulation

| Scenario                   | Status | Sub-Status |
| -------------------------- | ------ | ---------- |
| Item not found             | 404    | —          |
| Session not available      | 404    | 1002       |
| Conflict on create         | 409    | —          |
| Precondition failed (ETag) | 412    | —          |
| Write forbidden (SWR)      | 403    | 3          |
| Bad request                | 400    | —          |
| Unsupported operation      | 501    | —          |
| Throughput exceeded        | 429    | 3200       |
| Split/merge in progress    | 410    | 1007       |

### Programmable Hooks

```rust
// One-shot: next read to the partition containing this PK returns 404/1002, then resets.
store.force_session_not_available(region, partition_key);

// Pauses all replication TO the target region. Writes to other regions continue to
// execute and accumulate, but are not delivered to the paused region.
store.pause_replication(target_region);

// Delivers all accumulated writes to the target region in LSN order, then resumes
// normal replication flow.
store.resume_replication(target_region);

// Splits a physical partition into two children. During min_lock_duration (plus doc
// redistribution time), operations on the partition return 410/1007.
store.split_partition(db_id, coll_id, partition_id, min_lock_duration);

// Merges two adjacent physical partitions into one child. During min_lock_duration
// (plus doc merge time), operations on both partitions return 410/1007.
store.merge_partitions(db_id, coll_id, partition_id_a, partition_id_b, min_lock_duration);
```

#### Pause/Resume Semantics

- **`pause_replication(target_region)`**: Sets a flag on the target `RegionStore`. The
  replication task checks this flag before delivering each write. While paused, writes
  are enqueued in an in-memory buffer (bounded, FIFO).
- **`resume_replication(target_region)`**: Clears the flag and drains the buffer, applying
  all accumulated writes in order. Each replicated document updates the target partition's
  LSN via `fetch_max`.
- **Use cases**: Testing session-not-available scenarios after region failover, verifying
  the driver's retry logic when reads fail due to replication lag, simulating network
  partitions between regions.

---

## 14. Supported Control-Plane Operations

The emulator fully supports database, container, and partition key range operations via HTTP:

| Operation           | Method + Path                         | Notes                                                                                     |
| ------------------- | ------------------------------------- | ----------------------------------------------------------------------------------------- |
| Create database     | `POST /dbs`                           | Body: `{"id":"..."}`. Returns 409 if exists.                                              |
| Read database       | `GET /dbs/{db}`                       | Returns 404 if not found.                                                                 |
| Delete database     | `DELETE /dbs/{db}`                    | Cascades: deletes all containers and documents.                                           |
| Create container    | `POST /dbs/{db}/colls`                | Body must include `partitionKey` definition. Returns 400 if missing.                      |
| Read container      | `GET /dbs/{db}/colls/{coll}`          | Returns partition key definition. 404 if not found.                                       |
| Delete container    | `DELETE /dbs/{db}/colls/{coll}`       | Cascades: deletes all documents in the container.                                         |
| Read PKRanges       | `GET /dbs/{db}/colls/{coll}/pkranges` | Returns all partition key ranges for the container.                                       |
| ChangeFeed PKRanges | `GET /dbs/{db}/colls/{coll}/pkranges` | With `If-None-Match` and `A-IM: Incremental feed` headers. Returns changed ranges or 304. |
| Read account        | `GET /`                               | Synthesized from `VirtualAccountConfig`.                                                  |

### Partition Key Enforcement

Containers store a `PartitionKeyDefinition` (paths, kind, version). Point operations
validate that the database and container exist before proceeding (404 if not).
Partition key values are resolved by:

1. `x-ms-documentdb-partitionkey` header (takes precedence if present).
2. Extraction from document body using the container's PK definition paths.

---

## 15. Unsupported Operations

Queries, Batch, Bulk, Patch return **501 Not Implemented** with descriptive error body.

---

## 16. Validation Test Framework

### Design

Same test logic runs in two modes:

1. **Emulator mode**: `InMemoryEmulatorHttpClient` — fast, hermetic.
2. **Live validation mode**: Real Cosmos DB account — verifies fidelity.

### Mode Selection

The test mode is selected via the `AZURE_TEST_MODE` environment variable:

| Value            | Behavior                                             |
| ---------------- | ---------------------------------------------------- |
| (unset or empty) | Emulator mode (default) — no network, no credentials |
| `emulator`       | Explicit emulator mode                               |
| `live`           | Live mode — requires `COSMOS_ENDPOINT` + credentials |

### Test Helper Functions

```rust
/// Creates an emulator store with a single region, provisions a database and container,
/// and returns a ready-to-use test context.
async fn setup_single_region() -> TestContext {
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse("https://eastus.emulator.local").unwrap()),
    ])
    .with_consistency(ConsistencyLevel::Session);

    let emulator = InMemoryEmulatorHttpClient::new(config);
    let store = emulator.store();

    // Provision test database and container via HTTP requests to the emulator
    provision_database(&emulator, "testdb").await;
    provision_container(&emulator, "testdb", "testcoll", "/pk").await;

    TestContext { emulator, store, db: "testdb", coll: "testcoll" }
}

/// Creates a multi-region emulator with configurable write mode and replication.
async fn setup_multi_region(write_mode: WriteMode) -> TestContext {
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse("https://eastus.emulator.local").unwrap()),
        VirtualRegion::new("West US", Url::parse("https://westus.emulator.local").unwrap()),
    ])
    .with_write_mode(write_mode)
    .with_replication_config(ReplicationConfig::immediate());
    // ...
}
```

### Dual-Mode Test Pattern

Each test function uses a helper that dispatches to either emulator or live mode:

```rust
#[tokio::test]
async fn create_new_item() {
    let ctx = test_context().await; // Reads AZURE_TEST_MODE, returns emulator or live context

    let body = json!({"id": "item1", "pk": "pk1", "value": 42});
    let response = ctx.create_item("testdb", "testcoll", &body).await;

    assert_eq!(response.status(), 201);
    assert!(response.headers().contains_key("etag"));
    assert!(response.headers().contains_key("x-ms-request-charge"));
    assert!(response.headers().contains_key("x-ms-session-token"));

    // Verify document body has system properties
    let doc: serde_json::Value = response.json().await;
    assert!(doc.get("_rid").is_some());
    assert!(doc.get("_etag").is_some());
    assert!(doc.get("_ts").is_some());
    assert_eq!(doc["id"], "item1");
    assert_eq!(doc["value"], 42);
}
```

### Diff Reporting (Live Validation Mode)

In live mode, the framework captures both the live response and the emulator's response
for the same request, then reports differences:

```text
[EMULATOR_DIFF] Operation=CreateItem
  Status:  MATCH(201)
  Headers:
    x-ms-request-charge:      VALUE_MISMATCH  live="5.71"  emulator="5.80"
    x-ms-activity-id:         VALUE_VARIES     (expected — both are UUIDs)
    x-ms-session-token:       VALUE_VARIES     (expected — different LSN values)
    x-ms-last-state-change-utc: MISSING_IN_EMULATOR  live="Mon, 23 Mar 2026 10:00:00 GMT"
  Body:
    _rid:    VALUE_MISMATCH  live="abc123"  emulator="AAAAAQ=="
    _etag:   VALUE_VARIES    (expected — both are quoted UUIDs)
    _ts:     VALUE_VARIES    (expected — timestamps differ)
    id:      MATCH("item1")
    value:   MATCH(42)
```

Differences are classified as:

| Classification        | Meaning                                          | Action                   |
| --------------------- | ------------------------------------------------ | ------------------------ |
| `MATCH`               | Values identical                                 | None                     |
| `VALUE_VARIES`        | Both present, values differ as expected          | None                     |
| `VALUE_MISMATCH`      | Values differ unexpectedly                       | Warning                  |
| `MISSING_IN_EMULATOR` | Header/field present in live, absent in emulator | Allowed (see Section 16) |
| `EXTRA_IN_EMULATOR`   | Present in emulator, absent in live              | Warning                  |

### Test Categories and Assertions

**Point Operations** (create, read, replace, upsert, delete):
- Status code matches expected.
- All required response headers present (see Section 8).
- ETag format is valid (quoted string).
- Session token format matches V2: `{pkrange_id}:{version}#{globalLSN}#{regionId}={localLSN}`.
- RU charge is positive and within expected range for document size.
- Document body contains all system properties on reads/writes.

**Error Cases** (404, 409, 412, 1002):
- Status code and sub-status code match.
- Error response body has `code` and `message` fields.
- RU charge is still reported (even on errors).
- Session token is still returned (even on errors).

**Multi-Region** (write forbidden, replication, failover):
- Write to non-write region returns 403/3.
- Write to write region succeeds.
- Replicated documents are readable in other regions (after configured delay).
- Session-not-available is returned when reading with a session token ahead of
  the target region's LSN.

**Control Plane** (database/container CRUD):
- Create returns 201 with system properties.
- Duplicate create returns 409.
- Read returns stored metadata including partition key definition.
- Delete cascades (containers → documents), returns 204.
- Operations on non-existent resources return 404.

### Test Cases (Phase 1)

**Point Operations**: create_new_item, read_existing_item, replace_existing_item,
upsert_new_item, upsert_existing_item, delete_existing_item,
create_without_content_response, replace_without_content_response.

**Error Cases**: read_nonexistent_404, create_duplicate_409, replace_nonexistent_404,
delete_nonexistent_404, replace_stale_etag_412, session_not_available_404_1002,
forced_session_not_available.

**Multi-Region**: write_forbidden_403_3, write_to_write_region_succeeds,
multi_write_any_region, immediate_replication_cross_region,
delayed_replication_session_not_available, account_properties_reflect_config,
pause_resume_replication.

**Control Plane**: create_database, read_database, delete_database_cascades,
create_container_with_pk, read_container, delete_container_cascades,
create_container_missing_pk_400, read_nonexistent_database_404.

**Throttling**: throttle_429_3200_when_exceeds_budget, throttle_disabled_no_429,
container_creation_min_400.

**Split & Merge**: split_creates_two_children, split_locked_returns_410_1007,
split_preserves_vector_clock_version, merge_adjacent_partitions,
merge_increments_vector_clock_version, session_token_uses_v2_format,
read_after_split_succeeds.

---

## 17. Allowlisted Header Variance

**Default rule**: All response headers must have **identical values** between the emulator
and a live Cosmos DB account. Diff tests will fail for any header value mismatch.

The following headers are explicitly allowlisted as exceptions with specific reasons:

### Value-Varies (Must Be Present, Value Inherently Differs)

| Header               | Reason                                      |
| -------------------- | ------------------------------------------- |
| `x-ms-activity-id`   | UUID generated per request — always unique  |
| `x-ms-session-token` | LSN values differ between emulator and live |
| `date`               | Timestamp of response — always differs      |

All other headers are expected to match exactly. When diff tests reveal mismatches,
the fix is to update the emulator's header computation algorithm — not to add more
exceptions here. Exceptions should only be added for headers whose values are
**inherently non-deterministic** (e.g., timestamps, UUIDs) or **structurally different**
between emulator and live (with a documented explanation).

---

## 18. Module & File Layout

```text
src/in_memory_emulator/
├── mod.rs                  # Public API exports
├── client.rs               # InMemoryEmulatorHttpClient: HttpClient
├── config.rs               # VirtualAccountConfig, VirtualRegion, ConsistencyLevel, ReplicationConfig
├── store.rs                # EmulatorStore, RegionStore, PhysicalPartition, StoredDocument
├── rid.rs                  # RidGenerator, hierarchical RID encoding/decoding
├── epk.rs                  # Self-contained EPK hash: V1, V2, MultiHash, MurmurHash3, component serialization
├── dispatch.rs             # Request parsing, URL routing, operation resolution
├── operations.rs           # Point operation implementations
├── response.rs             # EmulatorResponseBuilder, error_body, header generation
├── session.rs              # SessionState, LSN tracking, token parsing/formatting
├── ru_model.rs             # RuChargingModel, size bucket computation
└── system_properties.rs    # _rid, _self, _ts, _etag generation, JSON injection

tests/
├── in_memory_emulator.rs                     # Root test file (feature-gated)
└── in_memory_emulator_tests/
    ├── mod.rs                                # Shared test helpers
    ├── point_operations.rs                   # CRUD happy path tests
    ├── error_cases.rs                        # 404, 409, 412, 404/1002 tests
    ├── multi_region.rs                       # Write forbidden, replication, account props
    ├── control_plane.rs                      # Database/container CRUD, PK extraction
    ├── throttling.rs                         # 429/3200 throughput throttling tests
    └── split_merge.rs                        # Partition split/merge, 410/1007, V2 tokens
```

---

## 19. Usage API & Public API Surface

### Overview

The in-memory emulator is used by constructing a `VirtualAccountConfig`, creating an
`InMemoryEmulatorHttpClient`, and injecting it into the driver runtime as the HTTP transport.
Tests interact with the emulator through the standard driver API — the emulator is
transparent to the operation and transport pipelines.

### Basic Setup (Single Region)

```rust
use azure_data_cosmos_driver::in_memory_emulator::{
    InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion, ConsistencyLevel,
};
use url::Url;
use std::sync::Arc;

// 1. Configure the virtual account
let config = VirtualAccountConfig::new(vec![
    VirtualRegion::new("East US", Url::parse("https://eastus.emulator.local").unwrap()),
])
.with_consistency(ConsistencyLevel::Session);

// 2. Create the emulator HTTP client
let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));

// 3. Inject into driver runtime builder
let runtime = CosmosDriverRuntimeBuilder::new()
    .with_http_client(emulator.clone())
    .build()
    .await
    .unwrap();

// 4. Provision resources (via the emulator's store or HTTP requests)
let store = emulator.store();
store.create_database("testdb");
store.create_container("testdb", "testcoll", PartitionKeyDefinition::new(vec!["/pk"]));

// 5. Use the driver normally — all requests route to the emulator
let driver = runtime.create_driver("https://eastus.emulator.local", credential);
let response = driver.create_item("testdb", "testcoll", item_body, options).await?;
```

### Multi-Region Setup

```rust
use azure_data_cosmos_driver::in_memory_emulator::{
    InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    WriteMode, ReplicationConfig, ConsistencyLevel,
};

let config = VirtualAccountConfig::new(vec![
    VirtualRegion::new("East US", Url::parse("https://eastus.emulator.local").unwrap()),
    VirtualRegion::new("West US", Url::parse("https://westus.emulator.local").unwrap()),
    VirtualRegion::new("West Europe", Url::parse("https://westeurope.emulator.local").unwrap()),
])
.with_write_mode(WriteMode::Single) // Only first region accepts writes
.with_consistency(ConsistencyLevel::Session)
.with_replication_config(ReplicationConfig::immediate()) // Zero lag for fast tests
.with_replication_override(
    "East US", "West Europe",
    ReplicationConfig::fixed(Duration::from_millis(100)), // Slow transatlantic link
);

let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
```

### Test Hooks

The `EmulatorStore` (accessible via `emulator.store()`) exposes hooks for edge-case testing:

```rust
let store = emulator.store();

// Force next read in "East US" for partition key ["pk1"] to return 404/1002
store.force_session_not_available("East US", r#"["pk1"]"#);

// Pause all replication TO "West US" — writes accumulate but aren't delivered
store.pause_replication("West US");

// Resume replication — delivers all accumulated writes, then resumes normal flow
store.resume_replication("West US");

// Split partition 0 into two children (min 100ms lock before doc redistribution)
store.split_partition("testdb", "testcoll", 0, Duration::from_millis(100));

// Merge partitions 0 and 1 into one child (min 100ms lock before doc merge)
store.merge_partitions("testdb", "testcoll", 0, 1, Duration::from_millis(100));
```

### Custom RU Charging

```rust
use azure_data_cosmos_driver::in_memory_emulator::RuChargingModel;

let config = VirtualAccountConfig::new(regions)
    .with_ru_model(RuChargingModel {
        read_base_ru: 1.0,
        create_base_ru: 5.8,
        write_multiplier: 1.5,
        indexing_ru_per_property: 0.3,
    });
```

### Custom Physical Partition Count

```rust
// Override the default 4 partitions when creating a container
store.create_container_with_config(
    "testdb",
    "testcoll",
    PartitionKeyDefinition::new(vec!["/pk"]),
    ContainerConfig::new()
        .with_partition_count(8)
        .with_throughput(4000), // 4000 RU/s → 500 RU/s per partition
);
```

### Public API Surface

All public types are exported from `azure_data_cosmos_driver::in_memory_emulator::`.

#### Configuration Types

| Type                     | Description                                                                |
| ------------------------ | -------------------------------------------------------------------------- |
| `Epk`                    | Newtype for effective partition key (hex-encoded hash string)              |
| `VirtualAccountConfig`   | Root config: regions, write mode, consistency, replication, RU, throttling |
| `VirtualRegion`          | Region name + gateway URL + region_id                                      |
| `WriteMode`              | `Single` (one write region) or `Multi` (all regions write)                 |
| `ConsistencyLevel`       | `Session`, `Strong`, `BoundedStaleness`, `Eventual`                        |
| `ReplicationConfig`      | Replication delay: `immediate()`, `fixed(d)`, `range(min,max)`             |
| `RuChargingModel`        | Configurable RU rates per operation type and document size                 |
| `ContainerConfig`        | Per-container overrides (partition count, throughput)                      |
| `PartitionKeyDefinition` | Partition key paths, kind, version                                         |

#### Core Types

| Type                         | Description                                              |
| ---------------------------- | -------------------------------------------------------- |
| `InMemoryEmulatorHttpClient` | Implements `azure_core::http::HttpClient`; entry point   |
| `EmulatorStore`              | Shared store handle; exposes test hooks and provisioning |

#### Store Methods (Test Hooks)

| Method                                                         | Description                           |
| -------------------------------------------------------------- | ------------------------------------- |
| `create_database(db_id)`                                       | Provision a database                  |
| `create_container(db_id, coll_id, pk_def)`                     | Provision a container (default 4 PPs) |
| `create_container_with_config(db_id, coll_id, pk_def, config)` | Provision with custom config          |
| `force_session_not_available(region, pk)`                      | One-shot 404/1002 on next read        |
| `pause_replication(target_region)`                             | Stop replication to target            |
| `resume_replication(target_region)`                            | Resume + drain accumulated writes     |
| `split_partition(db, coll, partition_id, min_lock_duration)`   | Split partition into two children     |
| `merge_partitions(db, coll, id_a, id_b, min_lock_duration)`    | Merge two adjacent partitions         |

---

## 20. Open Questions (Resolved)

1. ~~**Control-plane stubs**: Should the emulator support in-memory database/container
   create/delete?~~ **Resolved** — full CRUD is implemented with PK definition enforcement.

2. ~~**Partition key range IDs**: Single range ID `0` per logical partition sufficient?~~
   **Resolved** — each container has multiple physical partitions (default 4, configurable),
   each covering an EPK hash range. Documents are stored by physical partition → EPK hash →
   item ID. This model supports future ReadFeed, ChangeFeed, and Query operations that
   iterate over EPK ranges. See [Section 5](#5-in-memory-store-design).

3. ~~**Content-response-on-write**: Respect the
   `x-ms-cosmos-populate-content-response-on-write` header?~~
   **Resolved** — yes. When the header is absent or `false`, write responses omit the
   document body. When `true`, the full body is returned. See [Section 7](#7-point-operation-emulation).

4. ~~**_rid format**: Random base64 vs hierarchical?~~
   **Resolved** — hierarchical binary encoding following the
   [Java SDK `ResourceId`](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/implementation/ResourceId.java)
   implementation. Child resources encode their parent's RID (db 4B → collection 4B with
   high bit → document 8B with type nibble). Numeric identifiers are monotonically
   increasing. See [Section 5 — Hierarchical RID Encoding](#hierarchical-rid-encoding).

5. ~~**Request auth validation**: Should the emulator validate Authorization headers?~~
   **Resolved** — no. Authorization headers are ignored. The emulator focuses on data-plane
   behavior fidelity, not auth correctness.

6. ~~**Concurrent writes**: Lock-based serialization vs last-writer-wins?~~
   **Resolved** — lock serialization per physical partition. Each physical partition has its
   own `RwLock`, allowing concurrent reads and exclusive writes within a partition. Operations
   on different physical partitions execute concurrently without contention.
   See [Section 5 — Concurrency Model](#concurrency-model).

---

## 21. Throughput Throttling

### Design

Throughput enforcement is **per-partition**, matching the Cosmos DB service model where
container throughput is evenly distributed across physical partitions. Each partition
gets `container_throughput_ru / partition_count` RU/s.

Throughput is always stored and distributed when configured. **Enforcement** (returning
429/3200 when the budget is exceeded) is controlled by a separate toggle:

```rust
let config = VirtualAccountConfig::new(regions)
    .with_throttling_enabled(true); // Without this, RU is tracked but never enforced
```

### Container Configuration

```rust
ContainerConfig::new()
    .with_partition_count(4)
    .with_throughput(4000) // 4000 RU/s total → 1000 RU/s per partition
```

Minimum provisioned throughput is 400 RU/s (panics on lower values).

### Per-Partition Tracking

Each `PhysicalPartition` has an optional `ThroughputTracker` that:

1. Maintains an `AtomicU64` accumulator for consumed RU in the current second window
2. Resets the accumulator when the timestamp crosses a second boundary
3. On each operation: computes RU charge → attempts to consume → returns 429/3200 if
   the partition's per-second budget (`container_ru / partition_count`) is exceeded

### Throttle Response

- **Status**: 429 Too Many Requests
- **Substatus**: 3200 (RUBudgetExceeded)
- **Header**: `x-ms-retry-after-ms` with retry delay in milliseconds
- Throttle checks happen **after** the RU charge is computed but **before** the
  operation modifies the store (for writes)

### Interaction with Split/Merge

When partitions are split or merged, child partitions get new `ThroughputTracker`
instances with the per-partition budget recalculated from the container's total
throughput and the new partition count.

---

## 22. V2 Vector-Clock Session Tokens

### Wire Format

The emulator emits V2 vector-clock session tokens following the real Cosmos DB format:

```text
{pkrangeId}:{version}#{globalLSN}#{regionId}={localLSN}
```

This matches the `VectorSessionToken` format used by the driver's own session token
infrastructure (`models/vector_session_token.rs`).

### Region IDs

Each `VirtualRegion` has a `region_id: u64` field. By default, region IDs are 0 (and
can be explicitly set via `with_region_id(id)`). The region ID is included in every
session token emitted by operations in that region.

### Version Semantics

The `vector_clock_version` field on `PhysicalPartition` tracks the topology version:

| Event                      | Version Change                         |
| -------------------------- | -------------------------------------- |
| Initial partition creation | 0                                      |
| Split                      | Preserved (child = parent version)     |
| Merge                      | Incremented (child = max(parents) + 1) |
| Region add/remove          | Would increment (not yet implemented)  |

### Session Consistency Check (V2-Aware)

When validating session tokens on reads:

1. **Version mismatch**: if token version > partition version → 404/1002.
   The client has seen a newer topology that this partition hasn't caught up to.
2. **Same version, LSN mismatch**: if token globalLSN > partition LSN → 404/1002.
3. **Token version < partition version**: proceed normally (the partition has moved
   forward via merge; the client's older token is still valid).

### Backward Compatibility

V1 tokens (`{pkrangeId}:-1#{lsn}`) are accepted on incoming requests. They are parsed
as version=0, globalLSN=lsn, with no region progress.

---

## 23. Partition Split & Merge

### Overview

The emulator supports programmatic simulation of partition splits and merges, enabling
tests to verify the driver's handling of topology changes (PKRange cache invalidation,
session token resolution, retry on 410/1007).

### Split

Triggered via test hook:

```rust
store.split_partition("testdb", "testcoll", partition_id, min_lock_duration);
```

#### Behavior

1. **Lock**: The parent partition is locked in all regions. During the lock period
   (at least `min_lock_duration`, plus the time needed for doc redistribution), all
   operations targeting the partition return **410/1007** (CompletingSplitOrMerge).
2. **Wait**: The emulator waits for `min_lock_duration` before proceeding.
3. **Split**: After the wait, the parent's EPK range is divided at the midpoint.
   Two child partitions are created with:
   - **IDs**: Next available from the container's `AtomicU32` counter
   - **EPK ranges**: `[parent_min, midpoint)` and `[midpoint, parent_max)`
   - **LSN**: `parent_lsn + 1` (both children start with same LSN)
   - **Vector clock version**: Same as parent (split does NOT change version)
   - **Documents**: Redistributed to children based on their EPK
   - **Parents**: `[parent_id]`
4. **Cleanup**: Parent partition is removed. Lock is cleared.

### Merge

Triggered via test hook:

```rust
store.merge_partitions("testdb", "testcoll", id_a, id_b, min_lock_duration);
```

#### Behavior

1. **Validation**: The two partitions must be adjacent (the first's `epk_max` must
   equal the second's `epk_min`).
2. **Lock**: Both parent partitions are locked in all regions. Operations return
   **410/1007** during the lock period.
3. **Wait**: The emulator waits for `min_lock_duration` before proceeding.
4. **Merge**: A single child partition is created with:
   - **ID**: Next available from the container's counter
   - **EPK range**: Union of both parents `[lower_min, upper_max)`
   - **LSN**: `1` (restarts)
   - **Vector clock version**: `max(parent_versions) + 1` (merge DOES increment version)
   - **Documents**: Merged from both parents
   - **Parents**: `[id_a, id_b]`
5. **Cleanup**: Both parent partitions are removed. Lock is cleared.

### Lock Duration Semantics

The `min_lock_duration` parameter is a **floor** — the actual lock lasts at least that
long, plus the time needed for the doc redistribution/merge work. This is implemented
by setting `locked_until` to `now + min_lock_duration + large_buffer`, then clearing
the lock once the work completes.

This allows tests to:
- Use `Duration::ZERO` for instant split/merge in fast tests
- Use `Duration::from_millis(500)` to test that the driver retries on 410/1007 during
  the lock window

### PKRanges Feed After Split/Merge

The `GET /dbs/{db}/colls/{coll}/pkranges` feed reflects the updated topology:
- Child partitions include their `parents` list (e.g., `["0"]` after split, `["0", "1"]`
  after merge)
- `vectorClockVersion` reflects the current version
- Parent partitions are no longer present

### Impact on Session Tokens

- **After split**: Session tokens for the parent partition are no longer valid. The driver
  must resolve them via the `parents` field of the child partitions. The vector clock
  version is unchanged, so version-based comparisons work correctly.
- **After merge**: The child partition has an incremented vector clock version. Session
  tokens from either parent (with lower version) are treated as older topology and
  proceed normally (version < child version → valid).
