# Session Consistency — End-to-End Contract

**Status:** Draft / Iterating
**Date:** 2026-08-31
**Crates:** `azure_data_cosmos_driver` (token cache, resolution, capture, retry),
`azure_data_cosmos` (public options, response surface, cross-client helpers)

This spec describes the **whole session-consistency contract** as it is
implemented today: what the service guarantees, what the driver owns, how a
session token gets onto a request and back off a response, and how session
behavior interacts with routing, retries, feeds, transactions, and diagnostics.

It is deliberately a *high-level, coherent* view. Detailed mechanics that already
have their own specs are linked, not duplicated:

- Retry classification and budgets — [0006 Error Codes and Retries](0006-error-codes-and-retries.md)
- Partition-key-range resolution — [0007 Partition Key Range Cache](0007-partition-key-range-cache.md)
- Region failover and partition-level failover — [0008 Partition-Level Failover](0008-partition-level-failover.md)
- Cross-region hedging — [0009 Cross-Region Hedging](0009-cross-region-hedging.md)
- Hub-region processing-only header — [0010 Hub-Region Processing Header](0010-hub-region-processing-header.md)
- Feed/query pagination — [0012 Feed Operations and Dataflow](0012-feed-operations-and-dataflow.md)
- PATCH read-modify-write — [0017 Patch Handler](0017-patch-handler.md)
- Diagnostics shape — [0018 Diagnostics Contract](0018-diagnostics-contract.md)
- Distributed transactions and strict per-operation merge rationale —
  [0022 Distributed Transactions](0022-distributed-transactions.md#8-session-token-handling)
  and [0022 §10.7](0022-distributed-transactions.md#107-strict-per-operation-session-token-merge)

---

## Table of Contents

1. [Scope: service contract vs SDK implementation](#1-scope-service-contract-vs-sdk-implementation)
2. [Token representation and scope](#2-token-representation-and-scope)
3. [Ownership: SDK, driver, and caller](#3-ownership-sdk-driver-and-caller)
4. [Request path — token resolution](#4-request-path--token-resolution)
5. [Response path — capture and merge](#5-response-path--capture-and-merge)
6. [Region and partition routing interactions](#6-region-and-partition-routing-interactions)
7. [Retry behavior — `ReadSessionNotAvailable` and friends](#7-retry-behavior--readsessionnotavailable-and-friends)
8. [Feeds, queries, and pagination](#8-feeds-queries-and-pagination)
9. [Distributed transactions](#9-distributed-transactions)
10. [PATCH and read-modify-write](#10-patch-and-read-modify-write)
11. [Diagnostics exposure](#11-diagnostics-exposure)
12. [Concurrency and state boundaries](#12-concurrency-and-state-boundaries)
13. [Known limitations and open questions](#13-known-limitations-and-open-questions)
14. [Validation](#14-validation)

---

## 1. Scope: service contract vs SDK implementation

### 1.1 What the service provides

Session consistency is a **server-enforced** guarantee mediated by an opaque
token that the client echoes back:

- Every data-plane response from a container partition carries
  `x-ms-session-token` (RNTBD response token `0x003E`) describing the replica's
  progress for the partition key range that served the request.
- A request may carry `x-ms-session-token` (RNTBD request token `0x0005`). The
  backend serves the read from a replica that is **at least as caught up** as the
  token; if no local replica qualifies, it fails the request with
  `404 NotFound` / substatus `1002 ReadSessionNotAvailable`.
- A token referencing a partition key range that no longer exists (completed
  split/merge, and not an ancestor of the routed partition) is answered with
  `410 Gone` / `1002`, telling the client to refresh routing state.
- The guarantee is *per session*, not per account: monotonic reads, monotonic
  writes, and read-your-writes hold for the sequence of operations that share a
  token chain. Session tokens are only meaningful within one container.

The service does **not** hold session state for a client. Continuity is entirely
the client's job: capture the token, keep it, send it back.

The in-memory emulator models this contract closely — token parsing, version and
global-LSN comparison, ancestor tolerance, `1002` echoing of the *requested*
LSN rather than the partition watermark, and forced-unavailability injection —
see `azure_data_cosmos_driver/src/in_memory_emulator/session.rs` and
`.../in_memory_emulator/operations.rs`.

### 1.2 What the SDK adds

Everything above the wire is client policy and lives in the driver:

- A per-client session token cache (`SessionContainer`) keyed by collection RID
  and partition key range.
- Consistency gating: tokens are only attached and captured when session
  consistency is *effective* for the operation.
- Version-aware merge, including false-progress protection across topology
  changes.
- `1002`-driven region advance, with a budget and single-master hub-region
  behavior.
- Aggregation of tokens across query/change-feed pages.
- Public helpers so applications can move a token between processes or clients.

---

## 2. Token representation and scope

### 2.1 Wire shape

A session token header is a comma-separated list of **segments**, one per
partition key range:

```text
<pkRangeId>:<value>[,<pkRangeId>:<value>]*
```

The value is one of:

- **V2 vector** — `<version>#<globalLSN>[#<regionId>=<regionLSN>]*`. `version` is
  the partition configuration number and is *signed* on the wire (`-1` for a
  range with no assigned topology version). A region LSN of `-1` is the
  "no progress yet" sentinel.
- **V1 simple** — a bare integer LSN, accepted for older account configurations.

Parsing tries V2 first and falls back to V1
(`azure_data_cosmos_driver/src/models/vector_session_token.rs`,
`SessionTokenValue::parse`). The parsed form round-trips verbatim, including
`-1` sentinels, because merged tokens are sent back to the service.

### 2.2 Types

| Type | Crate / module | Role |
| --- | --- | --- |
| `SessionToken` | `azure_data_cosmos_driver::models` (re-exported by `azure_data_cosmos::options`) | Public, opaque `Cow<'static, str>` newtype for a full header value. Exposes `merge()`. |
| `SessionTokenSegment` | `azure_data_cosmos_driver::models` | Public parsed `<pkRangeId>:<value>` segment with `global_lsn()`, `is_as_recent_as()`, `merge_value()`. |
| `SessionTokenValue` | driver-internal | `Simple(u64)` or `Vector(VectorSessionToken)`. |
| `VectorSessionToken` | driver-internal | `version`, `global_lsn`, `region_progress: HashMap<u64, Option<u64>>`. |

The public surface is deliberately string-shaped and opaque: callers can carry,
compare-by-merge, and re-supply tokens without depending on the wire grammar.

### 2.3 Merge and recency semantics

`VectorSessionToken::merge` is the single definition of "combine two views of the
same range":

- **Same version** — take `max(globalLSN)` and per-region `max(regionLSN)`; a
  region seen only in the incoming token is added (including a `None`
  no-progress entry, which still counts as a change).
- **Different version** — the **higher version wins outright** for `globalLSN`
  and for the region set; per-region values take the max only where both sides
  have the region. Regions present only on the lower-version token are dropped.

The different-version rule is *false-progress protection*: after a split, merge,
or failover changes topology, `max(globalLSN)` across versions would synthesize
an LSN that never existed in the new topology. This matches Java's
`isSessionTokenFalseProgressMergeEnabled = true` default.

`is_as_recent_as` uses the same ordering: higher version wins; otherwise compare
`globalLSN` and every region the other side knows about (`Option<u64>` orders
`None` below any `Some`, so a no-progress region is naturally "behind").

Composite tokens are merged segment-wise by partition key range ID
(`SessionToken::merge`); segments with distinct IDs are preserved side by side.

---

## 3. Ownership: SDK, driver, and caller

```text
azure_data_cosmos (SDK)                 azure_data_cosmos_driver
──────────────────────                  ────────────────────────
options.session_token  ─────────────▶   CosmosOperation::with_session_token
OperationOptions
  .session_capturing_disabled ──────▶   pipeline consistency gate
  .read_consistency_strategy  ──────▶   ReadConsistencyStrategy::is_session_effective
  .max_session_retry_count    ──────▶   OperationRetryState session budget

ResponseHeaders::session_token ◀─────   CosmosResponseHeaders.session_token
ContainerClient::get_latest_session_token  (pure, uses SessionTokenSegment)

                                        CosmosDriver
                                          └── SessionManager
                                                └── SessionContainer  (the cache)
```

- **The driver owns automatic session state.** `CosmosDriver` holds exactly one
  `SessionManager`, which wraps the `SessionContainer` cache
  (`driver/routing/session_manager.rs`, `driver/routing/session_container.rs`).
  Nothing above the driver mutates it.
- **The SDK owns the public surface.** Per-operation `session_token` fields on
  `ItemReadOptions`, `ItemWriteOptions`, `PatchItemOptions`, `FeedOptions`,
  `QueryOptions`, `ChangeFeedOptions`, batch and DTX options; the
  `ResponseHeaders::session_token()` accessor; and
  `ContainerClient::get_latest_session_token`, a *pure* helper that merges
  caller-held `(FeedRange, SessionToken)` pairs for a target range and handles
  split/merge relationships (`azure_data_cosmos/src/session_helpers.rs`).
- **The caller owns cross-process continuity.** The driver cache is per client
  instance and in-memory only; moving a session between clients means reading a
  token off a response and passing it back in on the next request.

Configuration resolves through the standard layered option model (operation →
account → runtime → environment, see
[0002 Hierarchical Configuration Model](0002-hierarchical-configuration-model.md)).
`AZURE_COSMOS_READ_CONSISTENCY_STRATEGY` and
`AZURE_COSMOS_MAX_SESSION_RETRY_COUNT` are the session-relevant environment
knobs.

### 3.1 When session consistency is "effective"

The pipeline computes, per attempt:

```text
session_effective = !session_capturing_disabled
                 && read_consistency_strategy.is_session_effective(account_default)
```

`is_session_effective` is true when the strategy is `Session`, or when the
strategy is `Default` and the account default consistency level is `Session`.
`Eventual`, `LatestCommitted`, and `GlobalStrong` deliberately leave the session
lane, so they neither send nor capture tokens.

`session_capturing_disabled` is a single switch that turns off *both* halves —
no automatic attach, no automatic capture. Explicit per-operation tokens still
flow (see §4).

---

## 4. Request path — token resolution

Resolution happens in the operation pipeline just before the transport request is
built (`driver/pipeline/operation_pipeline.rs`, "STAGE 3"), and only when session
consistency is effective for that attempt.

`SessionManager::resolve_session_token` applies a strict precedence:

1. **Caller-supplied token wins verbatim.** A token set through options is used
   as-is with no merge against the cache. This is what makes an externally
   carried session authoritative.
2. **Partition-scoped token**, when the attempt has a resolved partition key
   range ID *and* the attempt is on the Gateway V2 / thin-client path: emit only
   `<pkRangeId>:<value>` for that range. The RNTBD backend rejects a multi-range
   composite token on a partition-scoped request
   (`"Session token specified is invalid."`), so scoping is required for
   correctness there. If that range has no cached token, the request goes out
   without one.
3. **Composite collection token** otherwise — every cached range for the
   container, sorted by range ID for deterministic output. Classic gateway
   accepts the composite and maps parent to child across splits, so sending the
   full token there stays read-your-writes safe.

Operations with no container reference (account- and database-level metadata)
never resolve a token.

The resolved value is stamped as `x-ms-session-token` on HTTP requests and as
RNTBD token `0x0005` on the thin-client path.

---

## 5. Response path — capture and merge

Capture runs immediately after the transport attempt returns and **before**
retry classification (`operation_pipeline.rs`, "STAGE 4b"). Ordering matters:
`409` and `412` classify as terminal `Abort`, and the abort disposition does not
carry headers, so capturing later would silently drop those tokens.

Capture rules:

- **Gated on session-effectiveness** for the attempt.
- **Skipped for master/metadata reads.** `is_reading_from_master` mirrors Java's
  `ReplicatedResourceClientUtils`: `DatabaseAccount`, `Database`, `Offer`, and
  `PartitionKeyRange` always target master; `DocumentCollection` targets master
  only for `ReadFeed`/`Query`/`SqlQuery`, so container CRUD still captures.
- **Captured on statuses that prove the server advanced the token**: any success,
  `409 Conflict`, `412 Precondition Failed`, and `404` whose substatus is *not*
  `1002`. A `404/1002` is never captured — it is the trigger for session retry.
- **Requires a resolved container reference**; the cache is keyed by collection.

`SessionContainer::set_session_token` splits the header into segments, parses
each value, and merges it into `collection_rid → (pk_range_id → value)` using the
version-aware merge from §2.3. Malformed segments are skipped silently on the
normal path; the strict variant (`set_session_token_checked`) is used only by
distributed transactions (§9).

A secondary `dbs/{db}/colls/{coll}` → RID index supports name-based lookup and
**container-recreation detection**: if the name now maps to a different RID, the
old RID's tokens are dropped, because a recreated container's LSNs are unrelated.

---

## 6. Region and partition routing interactions

Session state is *global to the client* but tokens are *per partition key range*,
and requests are *per region*. Three interactions follow:

- **Partition scoping.** The attempt's partition key range ID comes first from
  PK-range pre-resolution (see [0007](0007-partition-key-range-cache.md)) and
  otherwise from the first response's `x-ms-documentdb-partitionkeyrangeid`
  header. It drives both Gateway V2 token scoping (§4) and partition-level
  routing overrides.
- **Region progress inside the token.** V2 tokens carry per-region LSNs, so a
  token gathered in one region is meaningful in another. Failing over to a
  region that has not yet replicated those LSNs is exactly the `1002` case in §7.
- **Transport mode.** Classic gateway receives the composite token; Gateway V2 /
  RNTBD receives a single-range token. This is the only place transport mode
  changes session semantics on the request path.

Session tokens are **never cleared** by routing decisions. Failover, hedging, and
session retries all preserve the cache; discarding tokens to "make the read
succeed" would silently downgrade the customer's chosen guarantee.

Cross-region hedging races preserve the same token on both arms, and a `1002`
observed on either arm propagates into the surrounding retry state (see
[0009](0009-cross-region-hedging.md)).

---

## 7. Retry behavior — `ReadSessionNotAvailable` and friends

`404/1002` means the token is ahead of every replica the region could serve from.
The classification lives in `driver/pipeline/retry_evaluation.rs`
(`try_handle_read_session_not_available`), and the full table of statuses and
budgets is in [0006](0006-error-codes-and-retries.md#4041002--read-session-not-available).

At a contract level:

- A `1002` produces `OperationAction::SessionRetry`, which advances to the next
  endpoint **without** consuming the region-failover budget.
- **Budget**: `max_session_retry_count` if set, otherwise 2 on single-write
  accounts and `preferred_endpoints.len()` on multi-write accounts.
- **Single-master routing**: after the first session retry, reads are routed
  through the *preferred write* endpoint list, i.e. toward the hub region that by
  definition has the writes the token describes.
- **Single-master exhaustion**: two session retries on a single-master account is
  treated as a genuine miss, and the `1002` is surfaced rather than retried
  further.
- **Hub-region latch**: the first `1002` on a single-master data-plane operation
  latches `x-ms-cosmos-hub-region-processing-only` for the remainder of the
  operation, including propagation across hedge siblings. See
  [0010](0010-hub-region-processing-header.md) for the trigger conditions and
  wire contract.
- **Tokens are preserved** across every session retry.

Related failures that are *not* session-specific but interact with session state:

- `410/1002` (range referenced by the token is gone) is routing staleness; the
  PK-range cache refresh path owns it ([0007](0007-partition-key-range-cache.md)).
- `410` split/merge and `403/3` write-forbidden drive topology and failover
  handling ([0008](0008-partition-level-failover.md)); the next capture naturally
  carries the higher-version token, and §2.3's version rule prevents the old and
  new topologies from being averaged into a false token.

---

## 8. Feeds, queries, and pagination

Feed and query pages ride the same pipeline, so each backend page independently
resolves and captures a token. What is specific to pagination is **aggregation**:

- One emitted page may consume several backend pages (cross-partition fan-out,
  ORDER BY merge, selective filters returning empty-but-continuing pages).
  `PageAggregator` (`driver/dataflow/query_response.rs`) merges the session token
  of every absorbed response via `SessionToken::merge`, so the emitted page's
  header describes *all* ranges that contributed to it.
- The streaming ordered merge seeds each new aggregator with the running token
  and reads it back afterward, so the token survives page boundaries — including
  a partially filled page that ends in an error, since the consumed backend pages
  still advanced session state.
- `FeedPage::headers().session_token()` is how the SDK surfaces the aggregated
  value; `FeedOptions`/`QueryOptions`/`ChangeFeedOptions` carry a caller-supplied
  token onto the *initial* operation of the feed.
- Continuation tokens are independent of session tokens. A continuation may pin a
  region (which suppresses hedging), but it does not carry session state; a
  resumed feed relies on the client cache or a caller-supplied token.

See [0012](0012-feed-operations-and-dataflow.md) for pagination mechanics.

---

## 9. Distributed transactions

DTX is preview-gated (`preview_dtx`) and is the one path with *per-operation*
tokens rather than one token per request. In outline:

- Before serialization, each sub-operation lacking an explicit token is stamped
  from the cache, preferring the exact range, then parent ranges of a freshly
  split child, then the compound collection token.
- On terminal success the driver merges each successful sub-operation's returned
  `<pkRangeId>:<value>` into the shared cache.
- Under session consistency a malformed token on a **committed 2xx** response is
  a hard error rather than a silent skip, so read-your-writes cannot be quietly
  weakened; outside session consistency the bookkeeping is best-effort.

The strict behavior is intentional: a corrupt or unroutable token breaks the
Session token chain, so returning apparent success would conceal loss of
read-your-writes. The coordinator's commit remains authoritative—the merge
error does not roll it back or reclassify it—but the caller must reconcile the
committed-but-not-safely-tokenized outcome. Under weaker consistency modes,
best-effort handling avoids failing an otherwise successful operation for
bookkeeping that is not required to preserve the selected guarantee.

Full details: [0022 §8](0022-distributed-transactions.md#8-session-token-handling)
and [0022 §10.7](0022-distributed-transactions.md#107-strict-per-operation-session-token-merge).

Note that the parent-range walk exists **only** on the DTX resolution path today;
see §13.

---

## 10. PATCH and read-modify-write

The driver implements PATCH as a client-side read-modify-write loop, which makes
session state part of its correctness argument
([0017](0017-patch-handler.md)):

- The internal `Read` runs against the write region as a `LatestCommitted` read
  with no session token, so it observes the newest committed state. If write
  routing is unavailable and the read degrades to a reader region, the caller's
  explicit token (if any) is forwarded so an externally carried session is still
  honored even when the local cache is empty.
- The ETag-guarded `Replace` forwards **the session token returned by that Read**,
  overriding any caller-supplied value, so the write commits against the same
  replica view that was read. This is what closes the read-to-write TOCTOU window.
- Retries of the RMW loop repeat both steps, so each attempt re-establishes the
  pairing.

The same "forward the read's token to the dependent write" shape is the general
recommendation for any client-side RMW built on top of the SDK.

---

## 11. Diagnostics exposure

Session behavior is observable without reaching into internals:

- **Per-request records** in `DiagnosticsContext` carry the response
  `session_token` alongside `request_charge` and `activity_id`, so a captured
  chain can be reconstructed from a single operation's diagnostics
  ([0018](0018-diagnostics-contract.md)).
- **Execution context** annotates each attempt as `Initial`, `Retry`, or
  `RegionFailover`; a session retry classifies as `Retry` and takes precedence
  over failover classification when both counters are non-zero.
- **Tracing**: the per-attempt `routing decision made` debug record shows which
  region each attempt used, and the terminal `operation aborted` record includes
  `session_retries` and `pk_range_id`.
- **Response surface**: `ResponseHeaders::session_token()` on every item, batch,
  and feed response is the supported way for applications to snapshot a session.

The cache itself derives `SafeDebug`, so tokens are not leaked through debug
formatting of driver state.

---

## 12. Concurrency and state boundaries

- **One cache per client.** `SessionManager` is owned by `CosmosDriver`; the SDK
  holds an `Arc<CosmosDriver>` shared by all `DatabaseClient`/`ContainerClient`
  handles created from one `CosmosClient`. Two clients built separately do not
  share session state even against the same account.
- **Synchronous locking.** `SessionContainer` uses a `std::sync::RwLock`, never
  held across an `.await`. Resolve takes a read lock; capture takes a write lock
  for the duration of a small map update. Poisoning is recovered
  (`unwrap_or_else(|e| e.into_inner())`) because a partially updated token map is
  still usable and losing session state to a panic elsewhere would be worse.
- **Read-then-write races are benign.** Two concurrent operations may resolve the
  same token and capture different advances; the merge is commutative and
  monotone within a topology version, so the cache converges to the maximum.
- **No persistence, no eviction.** State is process-local and lives for the life
  of the client, except for the RID-mismatch purge on container recreation.
  Memory is bounded by (containers touched × ranges per container).
- **Boundary with routing state.** Session state is separate from the location
  cache, endpoint-unavailability state, and PK-range cache. Session retries read
  routing state but never mutate endpoint health, and no routing decision mutates
  the session cache.

---

## 13. Known limitations and open questions

1. **No parent-range walk on the non-DTX resolve path.** After a split, a request
   scoped to a fresh child range finds no cached token and is sent without one,
   until the first capture for that child. The DTX path already walks parents
   through the PK-range cache; unifying the two is tracked by
   `TODO(partition-key-range-parents)` in `session_manager.rs`.
2. **Container recreation is only detected by name → RID mismatch.** A session
   retry does not itself re-validate RIDs; deterministic RID comparison on the
   retry path is noted as future work in the pipeline.
3. **Malformed segments are dropped silently outside DTX.** A corrupt token from
   a response weakens the guarantee without surfacing an error. Only DTX
   currently fails closed.
4. **A caller-supplied token is never merged with the cache.** It replaces it for
   that request. Callers combining sessions must merge explicitly via
   `SessionToken::merge` or `ContainerClient::get_latest_session_token`.
5. **`session_capturing_disabled` is coarse** — it disables capture *and*
   resolution together; there is no capture-only mode.
6. **No cross-client or cross-process sharing** is built in; applications that
   need it must ferry tokens themselves.
7. **Consistency-level diagnostics attribute is not yet populated** on the
   automatic operation path ([0018](0018-diagnostics-contract.md)).
8. **V1 tokens carry no region progress**, so recency comparison for accounts
   still emitting V1 degrades to a single global LSN.

---

## 14. Validation

- **Unit** — token parsing, `Display` round-trip (including `-1` sentinels),
  same-version and cross-version merges, false-progress protection, and recency
  ordering in `models/vector_session_token.rs` and
  `models/session_token_segment.rs`; cache set/resolve, composite building,
  per-range scoping, and RID-mismatch purge in `driver/routing/session_container.rs`;
  precedence and master-resource gating in `driver/routing/session_manager.rs`;
  capture-status selection and `1002` classification in
  `driver/pipeline/operation_pipeline.rs` and `driver/pipeline/retry_evaluation.rs`.
- **End-to-end (in-memory emulator)** —
  `azure_data_cosmos/tests/in_memory_emulator_tests/session_token.rs` observes the
  outgoing `x-ms-session-token` header to prove capture-then-resolve, cache
  advance across writes, caller-token precedence, and the negative controls
  (Eventual consistency, capturing disabled, empty cache).
- **Cross-backend** — dual-backend tests compare response session tokens between
  the in-memory emulator and a real account, which is what keeps the emulator's
  modeled contract (§1.1) honest.
