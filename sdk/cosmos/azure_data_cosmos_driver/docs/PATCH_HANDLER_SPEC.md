# PATCH Handler Specification

This document describes the contract for `OperationType::Patch` in
`azure_data_cosmos_driver`.

## Known limitation and SDK exposure

The core driver always includes PATCH. Consuming SDKs decide whether and how
to expose it as preview using conventions appropriate to each language. The
Rust SDK, `azure_data_cosmos`, gates its public PATCH API behind the
**`preview_patch`** Cargo feature, which is off by default.

The handler does not deliver exactly-once semantics under transport failures:
an interrupted patch may re-apply non-idempotent operations (`Increment`,
`Add` on an array, `Move`). See [Invariants](#invariants) for the exact
interleaving. The Rust SDK API will stay gated until that hole is closed.

## Overview

`Patch` is a *virtual* operation type: the Cosmos DB REST endpoint does not
accept arbitrary JSON-Patch payloads, so the driver synthesizes the result
of a PATCH by running a **Read-Modify-Write (RMW) loop** entirely
client-side.

The handler lives in
`driver::pipeline::patch_handler` (`src/driver/pipeline/patch_handler.rs`)
and is dispatched from `CosmosDriver::execute_operation` before any of the
normal pipeline stages run.

## Inputs

| Field                                         | Source                                              | Notes                                                             |
| --------------------------------------------- | --------------------------------------------------- | ----------------------------------------------------------------- |
| `CosmosOperation` with `OperationType::Patch` | `CosmosOperation::patch_item(ItemReference)`        | Required.                                                         |
| Body                                          | `with_body(serde_json::to_vec(&PatchInstructions))` | Required. The handler re-parses it as `PatchInstructions`.        |
| Partition key                                 | `with_partition_key(...)`                           | Required. Used to issue the internal Read.                        |
| `patch_max_attempts`                          | `with_patch_max_attempts(NonZeroU8)`                | Optional. Defaults to `DEFAULT_PATCH_MAX_ATTEMPTS` (currently 5). |

## Algorithm

```text
1. Pre-flight validation:
   - reject any caller-set Precondition on the outer PATCH operation. The
     handler owns the If-Match precondition on the internal Replace and
     captures the ETag off the matching Read; honoring a caller-set value
     would either shadow that ETag (silently breaking the RMW guarantee)
     or require resolving it against the handler's own ETag (no sensible
     merge). The SDK wrapper already drops any Precondition before
     reaching this layer; the guard fail-fasts a driver-level user that
     constructed `CosmosOperation::patch_item(..).with_precondition(..)`
     directly.
   - reject ops whose path overlaps any partition-key path (we cannot
     move a document between physical partitions). For MoveOp this
     covers BOTH the source (`from`) and the destination (`path`).
   - reject empty op lists.

2. Clone the caller's operation options for the Read and override its
  consistency strategy with `LatestCommitted`. The caller's session token is
  intentionally not copied to the Read; `LatestCommitted` is not
  session-effective.

loop up to max_attempts times:
    3. read = execute_operation(Read, read_options) with:
       - ReadConsistencyStrategy::LatestCommitted forced in read_options;
       - preferred-write-endpoint routing;
       - hedging suppressed, including environment-enabled hedging; and
       - no session token.
       If every preferred write endpoint is unavailable or excluded, use
       normal read routing and record a `routing_fallback` request event
       with detail
      `patch_verification_read_write_endpoint_unavailable_or_excluded`.
       The driver pipeline returns Err(ErrorKind::HttpResponse { .. })
       for any non-2xx Read response; the patch handler propagates that
       error verbatim (with its raw_response and diagnostics intact).
       if read.headers().etag is None: return Other("no ETag, cannot RMW").
    4. value = serde_json::from_slice(read.body())
       apply_patch_ops(&mut value, &spec.operations)
       merged_bytes = serde_json::to_vec(&value)
    5. replace = execute_operation(Replace(merged_bytes,
                                           Precondition::IfMatch(etag)),
                                   caller_options) with the
       Read RESPONSE's session token overriding any caller-supplied
       value (this is the SE-004 TOCTOU mitigation; see below).
       match replace result:
         Ok(_)                                            -> succeed, see step 6
         Err(HttpResponse{ status: PreconditionFailed })  -> remember and continue the loop
         Err(_)                                           -> return error verbatim
    6. return CosmosResponse::new(merged_bytes,
                                  replace.headers(),
                                  replace.status(),
                                  aggregated_diagnostics)
       where aggregated_diagnostics is the concatenation, in dispatch
       order, of every successful sub-op's per-request diagnostics —
       see "Response Synthesis" below.

if loop exhausted: return ErrorKind::HttpResponse{ status:
PreconditionFailed, .. } with the last 412 chained as the source.
```

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
2. `LatestCommitted` deliberately bypasses the session lane. The internal
  Read therefore does **not** carry the caller's session token or a token
  from a prior attempt. Freshness comes from the write-region quorum read,
  not session-token resolution. The pipeline removes
  `x-ms-session-token` after all custom-header layers are resolved, so a
  runtime/account/operation custom header cannot reintroduce it.
3. The Replace's session token is **overridden** with the session token
   returned on the Read's response — see `build_replace_sub_op`. This
   pins the Replace to the same replica view we just read from. Any
   further client-supplied session token on the outer PATCH is
   intentionally discarded for the Replace; the Read's response token
   is by definition fresher.
4. After a 412, the next attempt repeats the write-region
  `LatestCommitted` Read. A session token from the failed Replace is retained
  on the surfaced error if retries exhaust, but is not attached to the next
  Read.

### Verification-read routing and consistency

PATCH verification reads first use the partition's persisted PPAF writer when
one exists, then the account's write endpoint list. On multi-write accounts,
retries skip endpoints already abandoned by this operation and continue through
that write list; failed attempts are keyed by the stable regional gateway URL,
so a Gateway 2.0 metadata refresh cannot make the same writer look new. The set
is operation-local and does not mark a region unhealthy for unrelated work. A
persisted PPAF writer is resolved to the current account endpoint before use;
an unavailable or excluded override is skipped before considering account
writers. This is stricter than ordinary session-retry routing: after a Replace
commits but its response is lost, the client has no response session token
proving that commit. A nearest-region Session read may therefore miss the write
and make a future marker check incorrectly conclude that the Replace did not
land.

The read also forces `ReadConsistencyStrategy::LatestCommitted` and suppresses
both pre-attempt and retry-upgrade hedging. A hedge winner from another region
would reintroduce the stale-observation window even when the primary targets the
write region. Partition-level read circuit-breaker overrides likewise do not
replace active write-region routing. Retry evaluation disables PPCB management
for these reads as well, and the operation pipeline removes
`MarkPartitionUnavailable` effects before they reach routing state. HTTP
failures can still create endpoint state that the write-route selector consumes;
ambiguous transport failures advance through the operation-local failed-URL
set. Neither path updates a breaker the resolver deliberately ignores.

When all preferred write endpoints are unavailable or excluded, the operation
falls back to normal read routing instead of targeting a known-unhealthy
endpoint. The selected request carries a `routing_fallback` diagnostic event so
the degraded guarantee is observable. This fallback preserves availability but
cannot provide the same marker-observation guarantee as a write-region read;
the marker protocol must account for that when attributing an ambiguous
Replace. PPCB overrides stay disabled on the fallback path so routing and retry
classification remain symmetric.

## Response Synthesis

Because the post-image of a PATCH must reflect what the server *now*
contains, the handler builds the returned `CosmosResponse` from:

- **Body**: the locally-merged JSON it just sent in the successful Replace
  (the Replace's response body is *not* required to be present).
- **Headers / status**: those of the successful Replace.
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
synthesized response. It is `pub(crate)` and lives in
`driver::pipeline::from_local_body` (`src/driver/pipeline/from_local_body.rs`).

## Patch Operations

Supported (`PatchOperation` variants — all use RFC 6901 JSON Pointers):

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
  fails, its context is retained with only the operation name rewritten.
- The handler never retries beyond `max_attempts` and never converts a 412
  into success; the final outcome is whichever of "internal sub-op error",
  "successful PATCH", or "exhausted RMW attempts (412)" terminated the
  loop.
- When 412 retries exhaust `max_attempts`, the final error carries an
  aggregated `DiagnosticsContext` containing every accumulated Read and
  failed Replace context. Thus both successful and failed PATCH operations
  follow the "one PATCH operation = one `DiagnosticsContext`" contract.

## Why Driver-Side?

- The Cosmos DB REST data plane does not natively accept the
  rich `PatchOperation` set we expose; alternate "operations" wire formats vary
  by SDK and have never been consistent across languages.
- A driver-side RMW gives us a single, schema-agnostic implementation
  that benefits every language SDK once they wrap `OperationType::Patch`.
- The cost — one extra request per PATCH — is acceptable for the
  current feature scope; a future revision may switch to a server-side
  patch endpoint when one is universally available.

## Invariants

- The patch handler is the **only** code path allowed to deserialize a
  data plane response body. Every other pipeline stage continues to treat
  the body as `Vec<u8>`.
- `OperationType::Patch` is *not* idempotent and is *not* read-only.
- `OperationType::Patch` is dispatched **before** the standard retry/
  routing/throttling pipeline. The internal Read and Replace ops re-enter
  the pipeline normally, but they are never themselves `Patch`, so there
  is no recursive loop.
- The handler owns the `If-Match` precondition on the internal Replace.
  A caller-set `Precondition` on the outer PATCH `CosmosOperation` is
  rejected by the pre-flight guard before any sub-operation is dispatched.
  Likewise, the PATCH wire format (`PatchInstructions`) has no `condition` field,
  so a SQL filter predicate (peer SDKs' `FilterPredicate`) cannot be
  attached to a PATCH request in this preview.
- 412 stays non-retryable in the global retry-evaluation policy. PATCH's
  RMW retry is internal and never depends on the global policy.
- Every internal Read prefers the PPAF partition writer or account write
  endpoints, forces `LatestCommitted`, strips session-token headers, and
  suppresses hedging. Normal read routing is used only when no preferred write
  endpoint is usable, and that fallback is recorded in request diagnostics.
- **PATCH is not exactly-once under transport failures.** The internal
  Replace is `OperationType::Replace`, which the pipeline classifies as
  idempotent (`OperationType::is_idempotent`). If a transport-layer error
  fires after the inner Replace has been sent but before its response is
  received, and the server has already committed the write, the pipeline
  will cross-region retry the Replace. A retry against a replica that has
  already replicated the original commit returns 412, which the RMW loop
  treats as a normal race-lost and recovers by re-Reading and re-applying.
  Non-idempotent ops (`Increment`, `Add` on an array, `Move`) may therefore
  be applied **more than once** under this scenario. This is why the Rust SDK
  treats PATCH as preview and gates it behind `preview_patch`; other consuming
  SDKs choose their own exposure policy. Closing the hole requires the RMW loop
  to be able to *recognize its own committed write* rather than mistaking it
  for a concurrent writer — i.e. stamping each attempt with a marker the loop
  can look for on the verification read. Until then, callers needing
  exactly-once should either use idempotent ops (`Set` on a caller-computed
  value) or detect duplicate-application via a monotonic application-level
  sequence number.
