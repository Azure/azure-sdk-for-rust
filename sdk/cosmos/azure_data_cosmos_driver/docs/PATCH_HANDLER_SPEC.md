# PATCH Handler Specification

This document describes the contract for `OperationType::Patch` in
`azure_data_cosmos_driver`.

## Overview

A patch executes one of two ways:

- **Server-side** — the operation list is sent to the service as a single
  `PATCH` request (`Content-Type: application/json_patch+json`, RNTBD opcode
  `0x0002`). One round trip, and on a multi-write-region account the service
  resolves concurrent patches at the **path** level, so two writers touching
  different properties of the same item both survive.
- **Client-side** — a **Read-Modify-Write (RMW) loop** run entirely by the
  driver: read the item, apply the operations locally, issue an ETag-guarded
  Replace, restart on `412`. Two round trips minimum, and conflict resolution
  is document-level last-writer-wins, so a concurrent write to an unrelated
  property is lost.

`CosmosDriver::execute_operation` picks between them via
`resolve_patch_strategy` (`src/driver/pipeline/patch_strategy.rs`) before any
of the normal pipeline stages run. The client-side handler lives in
`driver::pipeline::patch_handler` (`src/driver/pipeline/patch_handler.rs`).

## Strategy selection

`PatchStrategy` (`src/options/patch_strategy.rs`) is resolved through the same
layered runtime → account → operation path as `ReadConsistencyStrategy`, and
can also be set per request via `PatchItemOptions::strategy`.

| Requested | Retry-safe ops, ≤ 10 | Unsafe ops | More than 10 ops |
| --- | --- | --- | --- |
| `Auto` (default) | server-side | client-side | client-side |
| `ClientSide` | client-side | client-side | client-side |
| `ServerSide` | server-side | server-side, retries disabled | server-side (service returns `400`) |

`Auto` prefers the service because one round trip beats two and path-level
conflict resolution beats losing a concurrent write. It steps back only where
the server-side path cannot deliver the same result: an operation list that
could be double-applied on resend, or one the service would reject outright.

`ServerSide` is never rewritten. An over-long list surfaces the service's own
`400` rather than silently changing execution mode, and an unsafe list is sent
with ambiguous-outcome retries disabled (see below) rather than quietly
switching to the loop.

A body that cannot be parsed as a non-empty `PatchInstructions` always routes
to the client-side handler, which owns the canonical validation errors.

## Retry safety of an operation list

`PatchInstructions::is_retry_safe` asks whether re-applying the whole list to
an item it has already been applied to leaves both the item and the response
status unchanged. It decides whether a server-side patch may be resent after a
failure that left the outcome unknown.

| Operation | Retry-safe | Why |
| --- | --- | --- |
| `Set` | yes | Create-or-replace at an exact path; the second application is a no-op. |
| `Replace` | yes | Strict replace, deterministic. |
| `Add` on an object member | yes | Add-or-replace on a key. |
| `Add` on an array position (`-` or a numeric last token) | no | Append/insert shifts the remaining elements, so a resend inserts a second element. |
| `Remove` | no | The path is gone on the second pass, so the resend errors; on an array index it deletes a different element. |
| `Increment` | no | Double-applies the delta. |
| `Move` | no | `from` no longer exists on the second pass. |

When a server-side patch is dispatched with an unsafe list,
`CosmosOperation::allows_ambiguous_outcome_retry` returns `false`, which stops
both retry layers — cross-region failover and the transport pipeline's
same-endpoint shard retry — from resending it. The operation surfaces the
underlying error instead. This is the same mechanism stored procedure
execution uses; see `ErrorCodesAndRetries.md`.

An ETag precondition would make even an unsafe list safe to send, because the
resend fails with `412` instead of applying twice. Preconditions are not yet
plumbed through patch, so that relaxation does not apply.

## Equivalence contract

The two paths are an execution-cost trade-off, not a semantic one. For the same
input they must produce:

- the same resulting document,
- the same status and Cosmos sub-status, on success **and** on failure,
- an `etag` on success, and an advanced session token.

`tests/in_memory_emulator_tests/patch_equivalence.rs` asserts this per operation
type and per error shape, and separately asserts which path actually ran (one
data-plane request for server-side, two for client-side) so the comparison
cannot pass vacuously.

**What the suite does not cover.** The in-memory emulator evaluates a
server-side patch with the same `apply_patch_ops` the client-side path uses, so
the suite pins that the two *driver* paths agree — not that either agrees with
the real service's evaluator. Sub-status and message parity against the service
is unverified and needs a live-service test tier.

Divergences that are inherent to the two designs, and are asserted rather than
hidden:

- **Request charge** — the client-side loop pays for a read plus a replace.
- **Conflict resolution** — path-level server-side, document-level client-side.
- **Response body** — the client-side path always has the post-image to hand;
  the server-side path honors `content_response_on_write`, which
  `CosmosDriver::execute_operation` defaults to `Enabled` for a patch when the
  caller left it unset, so the body does not depend on which path ran.
- **Operation limit** — server-side patch is capped at
  `MAX_SERVER_SIDE_PATCH_OPERATIONS` (10) per document.
- **Caller-set preconditions** — rejected on the client-side path, which owns
  its internal `If-Match`; the server-side path forwards them. The SDK never
  sets one, so this is reachable only from the driver crate.

## Inputs

| Field                                         | Source                                              | Notes                                                             |
| --------------------------------------------- | --------------------------------------------------- | ----------------------------------------------------------------- |
| `CosmosOperation` with `OperationType::Patch` | `CosmosOperation::patch_item(ItemReference)`        | Required.                                                         |
| Body                                          | `with_body(serde_json::to_vec(&PatchInstructions))` | Required. Re-parsed as `PatchInstructions`.                       |
| Partition key                                 | `with_partition_key(...)`                           | Required. Used to route the request.                              |
| `patch_max_attempts`                          | `with_patch_max_attempts(NonZeroU8)`                | Optional. Defaults to `DEFAULT_PATCH_MAX_ATTEMPTS` (currently 5). |

`patch_max_attempts` bounds the client-side loop only; a server-side patch is a
single request and ignores it.

## Client-side algorithm

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

2. Capture the caller's session token (if any) from the outer PATCH
   operation's request headers as the loop's *effective* session token —
   see "Session-token threading" below.

loop up to max_attempts times:
    3. read = execute_operation(Read, caller_options) with the loop's
       *effective* session token applied to the Read sub-op (caller's
       on attempt 1; carried-forward on subsequent attempts).
       The driver pipeline returns Err(ErrorKind::HttpResponse { .. })
       for any non-2xx Read response; the patch handler propagates that
       error verbatim (with its raw_response and diagnostics intact).
       if read.headers().etag is None: return Other("no ETag, cannot RMW").
       advance the *effective* session token to read.headers().session_token
       so the next attempt's Read never observes a strictly older session
       view.
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

### Session-token threading and the SE-004 TOCTOU mitigation

The RMW loop crosses two service round-trips (Read → Replace). Without
care, the Read could be served by one replica's view of the data and the
Replace could commit against a stale view on a *different* replica —
silently undoing recent writes the original caller would otherwise have
read. To close that window:

1. The caller-provided `OperationOptions` (consistency, end-to-end
   latency budget, throughput control, etc.) are threaded through to
   the internal Read **and** to the internal Replace.
2. The **caller's** session token (if any) seeds the loop's
   *effective* session token, which is applied to the first attempt's
   Read via `build_read_sub_op`, so the Read observes a session-
   consistent view of the item.
3. The Replace's session token is **overridden** with the session token
   returned on the Read's response — see `build_replace_sub_op`. This
   pins the Replace to the same replica view we just read from. Any
   further client-supplied session token on the outer PATCH is
   intentionally discarded for the Replace; the Read's response token
   is by definition fresher.
4. Across RMW attempts the loop monotonically advances the *effective*
   session token to the freshest one observed:
   - the Read's response token on every attempt;
   - the final Replace's response token on the successful attempt;
   - **the failed Replace's response token on every 412** (folded in
     via `SessionToken::merge`) — a 412 response carries a token that
     is strictly fresher than the Read we just performed (it was
     produced by a replica that already saw the conflicting writer's
     commit). Attempt N's Read therefore never regresses to a strictly
     older session view than attempt N-1 already saw, *including the
     failed-Replace's view of the post-conflict world*.

This matters most for `Session` consistency, but is correct for
`Eventual` too (a no-op there). For `Strong` / `BoundedStaleness` the
service-side replica selection already provides the guarantee, but the
handler still propagates the tokens so diagnostics surface them
end-to-end.

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

- All sub-operation errors are surfaced verbatim — including the
  `DiagnosticsContext` and request-tracking info from the internal Read or
  Replace.
- The handler never retries beyond `max_attempts` and never converts a 412
  into success; the final outcome is whichever of "internal sub-op error",
  "successful PATCH", or "exhausted RMW attempts (412)" terminated the
  loop.
- The aggregated `DiagnosticsContext` described in "Response Synthesis"
  applies to the *successful* path. On error paths the surfaced
  `DiagnosticsContext` is whatever the failing sub-op already carried —
  the handler does not synthesize an aggregated context for partial
  failures.

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
- **PATCH is not exactly-once under transport failures.** The internal
  Replace is `OperationType::Replace`, which the pipeline classifies as
  idempotent (`OperationType::is_idempotent`). If a transport-layer error
  fires after the inner Replace has been sent but before its response is
  received, and the server has already committed the write, the pipeline
  will cross-region retry the Replace. A retry against a replica that has
  already replicated the original commit returns 412, which the RMW loop
  treats as a normal race-lost and recovers by re-Reading and re-applying.
  Non-idempotent ops (`Increment`, `Add` on an array, `Move`) may therefore
  be applied **more than once** under this scenario. Lifting this caveat
  requires marking the internal Replace as non-idempotent for retry
  purposes (e.g. a per-op idempotency override on `CosmosOperation`); that
  is tracked as a follow-up because it interacts with PPAF write-retry
  semantics. Callers needing exactly-once should either use idempotent ops
  (`Set` on a caller-computed value) or detect duplicate-application via a
  monotonic application-level sequence number.
