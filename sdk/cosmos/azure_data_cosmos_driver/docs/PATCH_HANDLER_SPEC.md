# PATCH Handler Specification

This document describes the contract for `OperationType::Patch` in
`azure_data_cosmos_driver`.

## Overview

`Patch` is a *virtual* operation type: the Cosmos DB REST endpoint does not
accept arbitrary JSON-Patch payloads, so the driver synthesizes the result
of a PATCH by running a **Read-Modify-Write (RMW) loop** entirely
client-side.

The handler lives in
[`driver::pipeline::patch_handler`](../src/driver/pipeline/patch_handler.rs)
and is dispatched from `CosmosDriver::execute_operation` before any of the
normal pipeline stages run.

## Inputs

| Field                                       | Source                                       | Notes                                                            |
| ------------------------------------------- | -------------------------------------------- | ---------------------------------------------------------------- |
| `CosmosOperation` with `OperationType::Patch` | `CosmosOperation::patch_item(ItemReference)` | Required.                                                        |
| Body                                        | `with_body(serde_json::to_vec(&PatchSpec))`  | Required. The handler re-parses it as `PatchSpec`.               |
| Partition key                               | `with_partition_key(...)`                    | Required. Used to issue the internal Read.                       |
| `patch_max_attempts`                        | `with_patch_max_attempts(NonZeroU8)`         | Optional. Defaults to `DEFAULT_PATCH_MAX_ATTEMPTS` (currently 5). |

## Algorithm

```text
loop up to max_attempts times:
    1. Pre-flight validation:
       - reject ops whose path overlaps any partition-key path (we cannot
         move a document between physical partitions).
       - reject ops on a Hash/Range/MultiHash partition definition where
         the spec is empty.
    2. read = execute_operation(Read, default options)
       if read.status() is not 2xx: return read error verbatim.
       if read.headers().etag is None: return Other("no ETag, cannot RMW").
    3. value = serde_json::from_slice(read.body())
       apply_patch_ops(&mut value, &spec.operations)
       merged_bytes = serde_json::to_vec(&value)
    4. replace = execute_operation(Replace(merged_bytes,
                                           Precondition::IfMatch(etag)),
                                   caller_options)
       match replace.status():
         412 PreconditionFailed -> remember and continue the loop
         2xx                    -> succeed, see step 5
         other                  -> return error verbatim
    5. return CosmosResponse::new(merged_bytes,
                                  replace.headers(),
                                  replace.status(),
                                  replace.diagnostics())

if loop exhausted: return the 412 captured in the last attempt.
```

## Response Synthesis

Because the post-image of a PATCH must reflect what the server *now*
contains, the handler builds the returned `CosmosResponse` from:

- **Body**: the locally-merged JSON it just sent in the successful Replace
  (the Replace's response body is *not* required to be present).
- **Headers / status / diagnostics**: those of the successful Replace.

`from_local_body_and_driver_headers` is the single helper that builds this
synthesized response. It is `pub(crate)` and lives in
[`driver::pipeline::from_local_body`](../src/driver/pipeline/from_local_body.rs).

## Patch Operations

Supported (`PatchOp` variants — all use RFC 6901 JSON Pointers):

| Variant     | JSON `op`     | Semantics                                                                  |
| ----------- | ------------- | -------------------------------------------------------------------------- |
| `Add`       | `"add"`       | Insert into object or array (`/-` appends).                                |
| `Set`       | `"set"`       | Same as `Add`; conventional for "create or overwrite" leaf assignment.     |
| `Replace`   | `"replace"`   | Overwrite an existing leaf; fails if the leaf is missing.                  |
| `Remove`    | `"remove"`    | Delete a leaf; fails if the leaf is missing or the path is the root.      |
| `Increment` | `"increment"` | Numeric add; preserves i64 fidelity, promotes i64→f64 on float operand.    |
| `MoveOp`    | `"move"`      | Move a subtree from `from` to `path`; refuses to move into own descendant. |

`IncrValue` is a Rust-only enum (`Int(i64)`, `Float(f64)`) that serializes
as a JSON number without precision loss.

## Errors

- All sub-operation errors are surfaced verbatim — including the
  `DiagnosticsContext` and request-tracking info from the internal Read or
  Replace.
- The handler never retries beyond `max_attempts` and never converts a 412
  into success; the final outcome is whichever of "internal sub-op error",
  "successful PATCH", or "exhausted RMW attempts (412)" terminated the
  loop.

## Why Driver-Side?

- The Cosmos DB REST data plane does not natively accept the
  rich `PatchOp` set we expose; alternate "operations" wire formats vary
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
- 412 stays non-retryable in the global retry-evaluation policy. PATCH's
  RMW retry is internal and never depends on the global policy.
