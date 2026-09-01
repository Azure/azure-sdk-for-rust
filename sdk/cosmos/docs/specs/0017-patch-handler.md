# PATCH Handler Specification

This document describes the contract for `OperationType::Patch` in
`azure_data_cosmos_driver`.

## SDK exposure and bounded duplicate suppression

The core driver always includes PATCH. Consuming SDKs decide whether and how
to expose it as preview using conventions appropriate to each language. The
Rust SDK, `azure_data_cosmos`, gates its public PATCH API behind the
**`preview_patch`** Cargo feature, which is off by default.

For instruction lists that are not safe to reapply, the handler persists a
tracking entry in the item as part of the same ETag-guarded Replace as the
mutation. A verification Read that observes the same tracking ID returns
success without applying the instructions again. This closes the
commit-succeeded/response-lost duplicate-application hole within the bounded
protocol described in [Tracking protocol](#tracking-protocol).

## Overview

PATCH has two execution paths. Server-side PATCH sends `PatchInstructions` to
Cosmos DB as one request. Client-side PATCH runs the tracked
**Read-Modify-Write (RMW) loop** in `driver::pipeline::patch_handler`. The
`PatchStrategy` option selects a path through `resolve_patch_strategy` before
transport planning.

The server path uses the normal operation pipeline, standard-gateway content
type `application/json_patch+json`, and Gateway 2.0 RNTBD operation ID
`0x0002`. The client path is intercepted by `CosmosDriver::execute_operation`
before transport selection and dispatches ordinary Read and Replace helpers.

## Inputs

| Field                                         | Source                                              | Notes                                                             |
| --------------------------------------------- | --------------------------------------------------- | ----------------------------------------------------------------- |
| `CosmosOperation` with `OperationType::Patch` | `CosmosOperation::patch_item(ItemReference)`        | Required.                                                         |
| Body                                          | `with_body(serde_json::to_vec(&PatchInstructions))` | Required. The handler re-parses it as `PatchInstructions`.        |
| Partition key                                 | `with_partition_key(...)`                           | Required. Used to issue the internal Read.                        |
| `patch_strategy`                              | `OperationOptions.patch_strategy`                   | Optional. Layered; defaults to `PatchStrategy::Auto`.             |
| `patch_max_attempts`                          | `with_patch_max_attempts(NonZeroU8)`                | Optional. Defaults to `DEFAULT_PATCH_MAX_ATTEMPTS` (currently 5). |
| `patch_tracking_id`                           | `with_patch_tracking_id(PatchTrackingId)`           | Optional. Generated once per invocation for unsafe instructions.  |
| `patch_tracking_capacity`                     | `with_patch_tracking_capacity(NonZeroU16)`          | Optional. Defaults to 1024 entries; oldest is evicted when full.  |
| `patch_tracking_retention_seconds`            | `with_patch_tracking_retention_seconds(NonZeroU32)` | Optional. Defaults to 300 seconds; whole-second granularity.      |

## Strategy resolution

Cosmos DB accepts at most 10 instructions in one server-side PATCH. The
client-side RMW path has no corresponding instruction-count limit.

| Requested strategy | Retry-safe list, at most 10 | Unsafe list, at most 10 | More than 10 instructions  |
| ------------------ | --------------------------- | ----------------------- | -------------------------- |
| `Auto`             | Server-side                 | Client-side             | Client-side                |
| `ClientSide`       | Client-side                 | Client-side             | Client-side                |
| `ServerSide`       | Server-side                 | Server-side             | Server-side, service `400` |

`Auto` is the default. It chooses server-side only when the complete list is
safe to resend and fits the service limit. More than 10 instructions therefore
switch automatically to RMW under `Auto`; explicit `ServerSide` is never
silently rewritten and surfaces the server rejection. Explicit unsafe
`ServerSide` PATCH disables ambiguous-outcome retries and surfaces the original
error rather than risking duplicate application. Client-side unsafe PATCH uses
the B2 marker protocol.

Tracking ID, maximum-attempt, capacity, and retention settings apply only when
strategy resolution selects client-side RMW. They do not influence strategy
resolution and are ignored whenever `Auto` or explicit `ServerSide` selects the
server path; the service does not persist the client's marker and the request
receives no marker-backed duplicate suppression.

## Client-side RMW algorithm

```text
1. Pre-flight validation:
   - reject ops whose path overlaps any partition-key path (we cannot
     move a document between physical partitions). For MoveOp this
     covers BOTH the source (`from`) and the destination (`path`).
  - reject every op that overlaps the reserved tracking path.
   - reject empty op lists.

2. Classify the instruction list. A list is retry-safe when it contains only
  Replace and non-append Set operations and no operation path is a strict
  ancestor or descendant of another. On this client-side path, a caller-supplied
  ID opts any list into tracking; without one, every other list requires
  tracking. Resolve one stable tracking ID and the capacity before entering the
  loop.
  If the reserved tracking path overlaps a container partition-key path,
  reject a tracked PATCH before dispatching any sub-operation.

3. Clone the caller's operation options for the Read and override its
  consistency strategy with LatestCommitted. Copy the caller's explicit
  session token onto the Read so reader fallback can preserve an external
  session; writer routing removes it before transport because LatestCommitted
  is not session-effective.

   Transport selection MUST preserve `LatestCommitted`: Gateway V1 sends
   `x-ms-cosmos-read-consistency-strategy: LatestCommitted`; Gateway 2.0 sends
   RNTBD token `0x00FE = 0x03` and the proxy passes that strategy through to
   its backend read; Direct mode uses the same RNTBD token. The Rust driver
   currently implements Gateway V1 and Gateway 2.0, not Direct connectivity.

loop up to max_attempts times:
   4. read = execute_operation(Read, read_options) with:
       - ReadConsistencyStrategy::LatestCommitted forced in read_options;
       - preferred-write-endpoint routing;
       - hedging suppressed, including environment-enabled hedging; and
       - the caller's session token retained for possible reader fallback but
         stripped before transport when a preferred writer is selected.
       If every preferred write endpoint is unavailable or excluded, use
     normal read routing with the account-default consistency and effective
     session token. Record a routing_fallback request event with detail
     patch_verification_read_write_endpoint_unavailable_or_excluded.
    The operation pipeline also records fallback directly on the internal
    `CosmosResponse`; the PATCH handler uses that explicit flag for marker safety.
    The diagnostic event is observability-only and is not a correctness input.
       The driver pipeline returns Err(ErrorKind::HttpResponse { .. })
       for any non-2xx Read response; the patch handler propagates that
       error verbatim (with its raw_response and diagnostics intact).
       if read.headers().etag is None: return Other("no ETag, cannot RMW").
      Evaluate any caller Precondition against read.headers().etag:
      - IfMatch succeeds only when the values match (or the value is `*`).
      - IfNoneMatch is rejected before dispatch because PATCH is a write.
      Return 412 before Replace when the condition is not met. Repeat this
      evaluation after every Read when a 412 race restarts the loop.
   5. value = serde_json::from_slice(read.body())
    For a tracked PATCH:
     - if the tracking ID is already present, return the Read as success
      without applying the instructions;
    - if routing fell back to a reader and the ID is absent, allow insertion
     only for a driver-generated ID before this invocation dispatches its
     first Replace; otherwise fail with 503 because absence is inconclusive;
    - otherwise validate the reserved array, prune entries whose retention
     window elapsed, evict the first entry if the capacity remains full, and
     append {trackingId, attemptedAt, retentionSeconds}.
       apply_patch_ops(&mut value, &spec.operations)
       merged_bytes = serde_json::to_vec(&value)
   6. replace = execute_operation(Replace(merged_bytes,
                                           Precondition::IfMatch(etag)),
                                   caller_options) with the
       Read RESPONSE's session token overriding any caller-supplied
       value (this is the SE-004 TOCTOU mitigation; see below).
       match replace result:
          Ok(_)                                            -> succeed, see step 7
         Err(HttpResponse{ status: PreconditionFailed })  -> remember and continue the loop
           Err(_)                                           -> for a tracked PATCH where any Replace
                                                               attempt may have been sent, perform one
                                                               verification Read; return success if it
                                                               finds the marker, return 503 on fallback
                                                               absence, propagate malformed state, and
                                                               otherwise preserve the original error
        7. return CosmosResponse::new(merged_bytes,
                                  replace.headers(),
                                  replace.status(),
                                  aggregated_diagnostics)
       where aggregated_diagnostics is the concatenation, in dispatch
       order, of every successful sub-op's per-request diagnostics —
       see "Response Synthesis" below.

if loop exhausted: for a tracked PATCH, perform one verification Read and
return success only if it finds the marker; otherwise return
ErrorKind::HttpResponse{ status: PreconditionFailed, .. } with the last 412
chained as the source.
```

The caller's end-to-end timeout is captured once before the RMW loop. Every
Read, Replace, retry, and terminal verification uses that same absolute
deadline; internal sub-operations never receive a fresh timeout budget.
If the deadline expires after a Replace may have committed but before its
verification Read completes, the handler does not continue past the deadline
or reapply the mutation. It returns the ambiguous timeout/error stamped with
the effective tracking ID. An application retry must reuse that ID; finding
the committed marker then returns success without issuing another Replace.

## Server-side execution

The server path sends the serialized instruction envelope directly through the
normal point-operation pipeline. Cosmos DB applies the instructions atomically
and returns the post-image when content response on write is enabled. Strategy
resolution enables that response by default to preserve the existing
`patch_item` contract; an explicit disabled setting suppresses the response
body for both server-side and client-side execution.

The service rejects an empty list, invalid operations, partition-key changes,
and lists containing more than 10 instructions. `patch_max_attempts` and the
tracking ID/capacity/retention settings apply only to client-side RMW. No
`_azsdkPatchTracking` property is written by server-side PATCH.

Retry-safe server PATCH can use normal ambiguous-outcome retries. For an unsafe
list selected explicitly with `ServerSide`, both retry layers stop after an
ambiguous result. Statuses that prove the operation was rejected before
execution retain their normal retry policy.

### Intentional implementation and test boundaries

- **Native FFI strategy and tracking identity:** Native callers select a
  per-operation strategy through `cosmos_operation_options_t.patch_strategy`.
  `UNSET` inherits the driver's layered configuration; `AUTO`, `CLIENT_SIDE`,
  and `SERVER_SIDE` map directly to the corresponding `PatchStrategy` values.
  Native submission may pre-resolve a tracking ID so cancellation can expose a
  stable identity, but normal success and error completions replace that value
  with the effective ID from the driver response or error. A provisional ID on
  cancellation does not imply that server-side PATCH persisted a marker, and
  tracking settings must not influence strategy selection.
- **Retry status coverage:** Retry tests intentionally do not duplicate the
  complete status-code matrix for server-side PATCH. PATCH-specific tests
  verify that resolved retry safety, request-sent state, and ambiguous outcomes
  reach the shared retry gate. The stored-procedure tests exercise the
  exhaustive status matrix, including safe exceptions such as 503, because
  both operation types use `CosmosOperation::allows_ambiguous_outcome_retry`
  and `is_unsafe_retry_after_possible_execution`. Add PATCH-specific status
  cases only if PATCH gains behavior outside that shared predicate.

## Tracking protocol

The reserved `_azsdkPatchTracking` property is an array of objects with a UUID
`trackingId`, non-negative Unix timestamp `attemptedAt`, and positive integer
`retentionSeconds`. It is visible user JSON and counts toward item size,
request units, and indexing. Existing marker state is validated and never
silently overwritten. Every entry must contain all three fields.

The effective tracking ID, including a driver-generated ID, is exposed on
successful responses and errors and captured as `patch_tracking_id` in the
operation diagnostics. Callers can persist that value and reuse it for an
application or process retry of the same logical PATCH.
Caller-supplied IDs should be random and unpredictable as well as unique to the
logical operation and item. Cooperating writers are trusted not to forge marker
entries. On the client-side path, supplying an ID opts any instruction list
into marker-based duplicate suppression, including a list the driver classifies
as retry-safe. Tracking settings are ignored on the server-side path. Without a
supplied ID, retry-safe client-side instruction lists do not create markers or
return an ID.

Each entry is protected from pruning for its configured positive number of
whole seconds (300 seconds by default). Persisting the window on each marker
prevents a later PATCH with a shorter setting from pruning longer-lived
evidence early. A matching ID is honored for as long as it remains present,
even after that interval; expiration only makes an entry eligible for pruning
by a later tracked PATCH. The default capacity is 1024 entries per item. When
the capacity is full after time-based pruning, PATCH removes the first entry
and appends the new marker. Duplicate suppression is therefore bounded by the
earlier of the entry's retention window and FIFO eviction under capacity
pressure. Cooperating writers must preserve marker array order.
Pruning uses only the item's service-generated `_ts`; the HTTP `Date` header is
reserved for authentication and is not a PATCH protocol clock. Marker
insertion requires a non-negative integer `_ts`. Because `_ts` has second-level
precision, pruning uses a strict cutoff: an entry is eligible only when
`attemptedAt < _ts - retention`. This guarantees the complete retention window
has elapsed even when the marker committed near the end of its timestamp
second.

Each marker persists its own `attemptedAt`. A newly committed marker initially
carries the `_ts` of the image read before its Replace; the next successful
tracked PATCH promotes only that newest marker to the document `_ts` that now
contains it. Older marker timestamps are never refreshed. If a later write or
multi-master conflict resolution advances `_ts` before that promotion, the
newest marker may be retained conservatively for longer, but it cannot be
pruned early.

The generated ID uses cryptographically secure operating-system entropy and
protects all internal retries in one invocation. A consuming SDK may accept a
caller-supplied ID to extend duplicate suppression across application retries
and process restarts. The caller must persist and reuse the ID only for the
same logical operation and item. Reusing an ID for a different operation
causes that operation to be treated as already committed.

All writers that replace a participating item must preserve the reserved
property and its unknown entry fields. A writer that removes, rewrites, or
fails to round-trip it breaks the guarantee. A malformed reserved property
causes PATCH to fail with 400. Tracked PATCH is rejected when the reserved path
overlaps a container partition-key path.

### Read/Replace consistency and the SE-004 TOCTOU mitigation

The RMW loop crosses two service round-trips (Read → Replace). Without
care, the Read could be served by one replica's view of the data and the
Replace could commit against a stale view on a *different* replica —
silently undoing recent writes the original caller would otherwise have
read. To close that window:

1. The caller-provided `OperationOptions` (end-to-end latency budget,
  throughput control, etc.) are threaded through to the internal Read
  **and** to the internal Replace. The Read's consistency strategy is
  deliberately overridden with `LatestCommitted`; other options are
  preserved.
2. `LatestCommitted` deliberately bypasses the session lane on preferred
  writer routing. The internal Read carries the caller's explicit session
  token so a reader fallback can preserve a session established by another
  client or process, but it never carries a token from a prior RMW attempt.
  On writer routing, freshness comes from the write-region quorum read rather
  than session-token resolution. The pipeline removes `x-ms-session-token`
  after all custom-header layers are resolved, so a custom header cannot
  reintroduce it on that route.
3. The Replace's session token is **overridden** with the session token
   returned on the Read's response — see `build_replace_sub_op`. This
   pins the Replace to the same replica view we just read from. Any
   further client-supplied session token on the outer PATCH is
   intentionally discarded for the Replace; the Read's response token
   is by definition fresher.
4. After a 412, the next attempt repeats the write-region
  `LatestCommitted` Read with the original caller token available only for
  fallback. A session token from the failed Replace is retained on the
  surfaced error if retries exhaust, but is not attached to the next Read.

### Verification-read routing and consistency

PATCH verification reads first use the partition's persisted PPAF writer when
one exists, then the account's write endpoint list. On multi-write accounts,
retries skip endpoints already abandoned by this operation and continue through
that write list; failed attempts are keyed by the stable regional gateway URL,
so a Gateway 2.0 metadata refresh cannot make the same writer look new. The set
is operation-local and does not mark a region unhealthy for unrelated work.
Persisted PPAF endpoints are resolved to the current account endpoint before
both reads and writes use them, so a Gateway 2.0 URL refresh applies across
operations and between the Read and Replace. An unavailable or excluded
override is skipped before considering account writers.

On a multi-write account, a write endpoint's regional quorum may still lag a
write accepted in another region. Preferred-writer routing strips the caller's
session token, so that Read can observe an older image or a plain 404 during
the replication window. The subsequent Replace carries the Read response's
token and ETag; if it routes to a region that has not reached that view it can
return 404/1002 and session-retry, while a newer image produces 412 and
restarts the RMW loop. These extra attempts are an accepted trade-off: regional
quorum narrows the contention window but cannot eliminate concurrent
multi-write conflicts.

A PPAF `current_endpoint` is the next candidate not known to have failed, not
proof that a successful write landed there. A verification Read may therefore
probe a stale candidate. The following Replace can discover that the candidate
is not writable and fail over, or reject a stale ETag with 412 and restart the
RMW loop. This is likewise an accepted availability/contention trade-off; the
candidate avoids always returning to a known-stale account writer after
partition-level failover.

The read also forces `ReadConsistencyStrategy::LatestCommitted` and suppresses
both pre-attempt and retry-upgrade hedging. A hedge winner from another region
would reintroduce the stale-observation window even when the primary targets the
write region. Partition-level read circuit-breaker overrides likewise do not
replace active write-region routing. Retry evaluation keeps PPCB classification
active, while the operation pipeline removes both `MarkPartitionUnavailable`
and `MarkEndpointUnavailable` effects before they reach shared routing state.
Failures advance through the operation-local failed-URL set instead of changing
routing for unrelated operations.

When all preferred write endpoints are unavailable or excluded, the operation
falls back to normal read routing with account-default consistency and the
effective session token. This prevents a lagging reader from supplying a stale
image to the Replace: 404/1002 session-retries instead, and an already-present
stale item cannot satisfy the token. A request that ultimately uses a reader
carries a `routing_fallback` diagnostic event; last-resort selection of a writer
does not. The fallback preserves read-your-writes but cannot provide the same
marker-observation guarantee as a write-region quorum read, so the marker
protocol must account for that when attributing an ambiguous Replace.
For a tracked PATCH, finding its ID on such a fallback remains positive proof
of a commit. Before the first Replace, absence is also conclusive for a fresh
driver-generated ID because that ID cannot have committed yet. The handler may
therefore insert that marker and proceed. For caller-supplied IDs, or after any
Replace dispatch, absence is inconclusive, so the handler returns 503 rather
than risk applying the mutation again.

## Response Synthesis

Unless `content_response_on_write` is explicitly disabled, the post-image of a
PATCH must reflect what the server *now* contains. The handler builds the
returned `CosmosResponse` from:

- **Body**: the locally-merged JSON it just sent in the successful Replace (the
  Replace's response body is *not* required to be present), or `NoPayload` when
  content response on write is disabled.
- **Headers / status**: those of the successful Replace, with request charge
  replaced by the total charge of the logical PATCH.
- **Diagnostics**: an *aggregated* `DiagnosticsContext` synthesized via
  `DiagnosticsContext::aggregate_sub_operations`, concatenating in
  dispatch order the per-request `RequestDiagnostics` of every
  successful sub-op the loop issued (every Read plus the final
  Replace). One PATCH operation therefore surfaces as one
  `DiagnosticsContext` with N `RequestDiagnostics` entries, instead of
  the prior single-Replace-only view. Operation-level fields
  (`activity_id`, options, `cpu_monitor`, `machine_id`, status) are
  inherited from the final Replace's context; total `duration` is the
  sum of all sources' durations (sub-ops are sequential).

Every successful logical PATCH reports the aggregate request charge for all
Read, Replace, and verification sub-operations. When a verification Read finds
the operation's tracking ID, that Read's body,
headers, and status are returned as the committed post-image, except that the
request-charge header is replaced with the exact total charge from the
aggregated operation diagnostics. Its diagnostics are aggregated with all
prior sub-operations from the same invocation.

## OpenTelemetry operation names

Both strategies report the caller-facing operation as `patch_item` on the root
span, operation diagnostics, and operation metric. A server-side PATCH has one
request attempt and inherits `patch_item` on its child request span.

Client-side RMW names each network helper explicitly:

- `patch_read_item` for the initial Read, every retry Read, and a terminal or
  application-level verification Read;
- `patch_replace_item` for each conditional Replace attempt.

Tracking-marker inspection, insertion, pruning, and recognition happen locally
within those helpers and do not create synthetic network spans. A caller retry
whose marker is already present therefore reports one `patch_read_item` child
under the `patch_item` root and no Replace child. This naming keeps standalone
`read_item`/`replace_item` telemetry distinct while preserving the logical PATCH
identity.

### System-property reconciliation on the synthesized body

The locally-merged body the handler synthesizes is the Read body with
the patch ops applied — but the Read body's `_etag` is the **Read's**
value, not the post-image's. Without reconciliation a caller that
deserializes the response body and reads `_etag` from it would see a
stale value that no longer matches the Replace's response header,
breaking optimistic-concurrency round-tripping.

The handler therefore reconciles the body's system properties with the
Replace response before returning:

1. If the inner Replace returned a non-empty response body
   (`content_response_on_write` enabled on the caller's options), it is
   surfaced verbatim — the service's post-image is the source of truth.
2. Otherwise, the locally-merged body's `_etag` is overwritten with
   `replace_headers.etag` (the value the Replace just minted) before
   the body is handed to `from_local_body_and_driver_headers`.
3. Other system properties (`_rid`, `_self`, `_attachments`) are stable
   across edits to the same item, so the Read's values remain correct.
4. `_ts` is not exposed on the Replace's response headers; when the
   Replace body is absent, the Read's `_ts` is left intact. It may lag
   the true post-image by the Read→Replace round-trip but never goes
   backwards. Callers that need an exact `_ts` should enable
   `content_response_on_write`.

`from_local_body_and_driver_headers` is the single helper that builds this
synthesized response. Before the logical PATCH returns, an explicit
`content_response_on_write = Disabled` replaces that synthesized payload with
`NoPayload` while retaining headers, status, diagnostics, and routing metadata.
The helper is `pub(crate)` and lives in
`driver::pipeline::from_local_body` (`src/driver/pipeline/from_local_body.rs`).

## Patch Operations

Supported (`PatchOperation` variants — all use RFC 6901 JSON Pointers):

When a pointer token addresses an array, it follows RFC 6902 index syntax and
cannot contain leading zeros except for the index `0`. The same token, such as
`01`, remains valid when its parent is an object because it is then a property
name rather than an array index.

| Variant     | JSON `op`   | Semantics                                                                  |
| ----------- | ----------- | -------------------------------------------------------------------------- |
| `Add`       | `"add"`     | Insert into object or array (`/-` appends).                                |
| `Set`       | `"set"`     | Same as `Add`; conventional for "create or overwrite" leaf assignment.     |
| `Replace`   | `"replace"` | Overwrite an existing leaf; fails if the leaf is missing.                  |
| `Remove`    | `"remove"`  | Delete a leaf; fails if the leaf is missing or the path is the root.       |
| `Increment` | `"incr"`    | Numeric add; preserves i64 fidelity, promotes i64→f64 on float operand.    |
| `MoveOp`    | `"move"`    | Move a subtree from `from` to `path`; refuses to move into own descendant. |

The `op` tags are the Cosmos DB wire contract
([REST reference](https://learn.microsoft.com/rest/api/cosmos-db/patch-a-document)),
not the lowercased Rust variant names — `Increment` serializes as `"incr"`.
The tags are pinned by `every_op_tag_matches_the_wire_contract` in
`src/models/patch.rs`. This matters wherever `PatchInstructions` reaches the
service directly, which today is the distributed-transaction patch operation.

`PatchOperation` is a public `Deserialize` type, so `Increment` additionally
accepts the legacy `"increment"` tag on **input** (a Serde alias) to keep patch
documents persisted by earlier versions of this crate parsing. Output is always
`"incr"` — re-serializing a legacy document upgrades the tag.

`CosmosNumber` is a Rust-only enum (`Int(i64)`, `Float(f64)`) that serializes
as a JSON number without precision loss.

## Errors

- Sub-operation errors preserve the failing error's status, sub-status, raw
  response, and source. The attached diagnostics are re-stamped with the
  virtual PATCH operation's `patch_item` name before the error is surfaced.
- A non-412 failure after earlier sub-operations carries an aggregated
  `DiagnosticsContext` containing those prior contexts plus the failing
  sub-operation's context, in dispatch order. When the first sub-operation
  fails, its context is retained with only the operation name rewritten. For
  a tracked PATCH where any terminal Replace attempt may have been sent, one
  verification-only Read returns success when it finds the marker. Authoritative
  absence preserves the original error, fallback absence returns 503, malformed
  state returns its validation error, and a failed verification Read preserves
  the original error while contributing its diagnostics. Finalized 400, 401,
  403, and 413 rejection responses skip verification because they prove the
  Replace did not commit.
- The handler never retries beyond `max_attempts`. A 412 causes another
  verification Read; observing this operation's marker then proves its prior
  Replace committed and returns success. A missing marker is treated as a
  genuine race and the instructions are reapplied to the new image.
- A missing marker on degraded reader routing returns 503 when the ID was
  caller-supplied or any Replace was already dispatched because absence is
  inconclusive. A fresh driver-generated ID may proceed from its first
  fallback Read. Malformed tracking state returns 400. A full marker list
  evicts its first entry before appending the new marker.
- When 412 retries exhaust `max_attempts`, the final error carries an
  aggregated `DiagnosticsContext` containing every accumulated Read and
  failed Replace context plus the final verification Read when its marker is
  absent. If that Read finds the marker, the handler returns success instead.
  Thus both successful and failed PATCH operations follow the "one PATCH
  operation = one `DiagnosticsContext`" contract.

## Why two paths?

- Server-side PATCH costs one request and preserves path-level conflict
  resolution, but accepts no more than 10 instructions and cannot safely retry
  an unsafe list after an ambiguous outcome.
- Client-side RMW supports longer lists and uses the tracking protocol for
  bounded duplicate suppression, at the cost of at least one Read plus one
  Replace and document-level ETag contention.
- `Auto` selects the server path only when it retains the same retry safety and
  falls back to RMW where the service limit or instruction semantics require it.

## Invariants

- The client-side patch handler is the only operation handler allowed to
  deserialize a data-plane item body. The server path forwards the serialized
  instruction envelope without inspecting the stored item.
- `OperationType::Patch` is *not* idempotent and is *not* read-only.
- Client-side PATCH is dispatched before the standard pipeline. Its internal
  Read and Replace operations re-enter normally. Server-side PATCH enters the
  standard retry/routing/throttling pipeline directly.
- The handler owns the `If-Match` precondition on the internal Replace.
  A caller-set ETag `Precondition` is evaluated against every authoritative
  Read before the handler applies operations; the internal Replace still uses
  the corresponding Read ETag as its lost-update guard. The PATCH wire format
  (`PatchInstructions`) has no SQL `condition` field,
  so a SQL filter predicate (peer SDKs' `FilterPredicate`) cannot be
  attached to a PATCH request in this preview.
- 412 stays non-retryable in the global retry-evaluation policy. PATCH's
  RMW retry is internal and never depends on the global policy.
- An unresolved server PATCH fails closed for ambiguous-outcome retry.
  Strategy resolution marks only a retry-safe server instruction list as
  eligible; an explicit unsafe `ServerSide` request remains at-most-once.
- Every internal Read prefers the PPAF partition writer or account write
  endpoints, forces `LatestCommitted`, and suppresses hedging. It carries the
  caller's explicit session token for possible fallback, but preferred-writer
  routing strips session-token headers before transport. Normal read routing
  with account-default/session consistency retains the token only when no
  preferred write endpoint is usable, and a fallback that ultimately uses a
  reader is recorded in request diagnostics.
- Tracked PATCH instructions are applied at most once within the tracking
  protocol's retention, capacity, routing, and cooperating-writer contract.
  The marker is committed atomically with the mutation, so a later Read can
  distinguish the operation's own committed Replace from a concurrent writer.
  Generated IDs cover one invocation; cross-process retries require callers to
  persist and reuse the same ID. The Rust SDK continues to gate PATCH behind
  `preview_patch`; other consuming SDKs choose their own exposure policy.
