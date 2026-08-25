# ADR-001 — Choose the patch execution path from operation retry safety

**Status:** Accepted
**Date:** 2026-08-21

## Context

Azure Cosmos DB accepts a single-document `PATCH` natively. The Rust driver did
not use it: `OperationType::Patch` was short-circuited into a client-side
Read-Modify-Write loop that read the item, applied the operations locally, and
wrote it back under an ETag precondition.

That loop is correct but costs two round trips instead of one, and — more
significantly — it resolves conflicts at the **document** level. On a
multi-write-region account, two writers patching different properties of the
same item lose one of the two updates, whereas the service resolves a native
patch at the **path** level and keeps both.

Sending every patch to the service instead is not automatically safe. A request
that fails in a way that leaves the outcome unknown may be resent by the retry
layers, and some patch operations change the document differently the second
time they are applied: `Increment` doubles its delta, an array `Add` inserts a
second element, `Remove` and `Move` fail because their target is already gone.
The client-side loop is immune to this — it re-reads and re-merges — so it is
the safe fallback. The service also caps a single-document patch at 10
operations.

## Decision

Support both paths and select between them from a property of the operation
list rather than from a global switch.

`PatchInstructions::is_retry_safe` classifies a list as safe when re-applying
it to an already-patched item leaves the document and the response status
unchanged. `PatchStrategy` exposes the choice to callers as `Auto` (default),
`ClientSide`, and `ServerSide`, resolved through the same layered
runtime → account → operation path as `ReadConsistencyStrategy`.

`Auto` runs server-side when the list is retry-safe and within the operation
limit, and falls back to the client-side loop otherwise. `ServerSide` is
honored as written: an over-long list surfaces the service's `400`, and an
unsafe list is sent with `CosmosOperation::allows_ambiguous_outcome_retry`
returning `false`, which stops both the cross-region and same-endpoint retry
layers from resending it.

The two paths are held to an equivalence contract — same resulting document,
same status and sub-status on success and failure — enforced by tests that also
assert which path actually ran.

## Consequences

The common case gets one round trip and path-level conflict resolution without
the caller asking for it, and the unsafe case keeps its previous behavior
rather than silently gaining a duplicate-application window.

The cost is a second execution path to keep honest. The two evaluators are
independent implementations, so equivalence is a property that has to be tested
rather than one that holds by construction; the equivalence suite exists for
exactly that reason and must be extended whenever an operation type or error
shape is added.

An explicit `ServerSide` request with unsafe operations trades a silent
duplicate for a visible error: the driver stops retrying and surfaces the
underlying failure. Callers who want retries for such a list should use `Auto`
and accept the extra round trip.

## Alternatives

**Always server-side.** Rejected: it would hand every `Increment` and array
append a duplicate-application window on any ambiguous transport failure, which
is a silent data-correctness change for existing callers.

**Always client-side (status quo).** Rejected: it permanently forgoes
path-level conflict resolution on multi-write accounts and doubles the round
trips for the majority of patches, which are `Set`/`Replace` only.

**A global client-level switch with no per-operation analysis.** Rejected:
safety is a property of the operation list, not of the client, so a global
switch forces callers to choose between correctness for their worst patch and
latency for their best one.

**Tiered idempotency classification** (for example, treating `Remove` as
"safe enough" because its retry error is benign). Rejected in favor of a binary
classification: a retry that changes the observed status code is a behavioral
difference, and the equivalence contract is easier to defend when the rule is
"same document and same status" with no exceptions.

**A runtime fallback from server-side to the loop.** Out of scope here. The
selection is static: it is made from the operation list before the request is
sent, and a server-side patch that the endpoint rejects surfaces that error
rather than re-running through the loop. A fallback would have to recognize
"this deployment does not accept `PATCH`" — an unsupported content type on the
standard gateway, an unknown opcode on Gateway 2.0 — separately from a genuine
`400` about the operation list, and cache the negative per endpoint so it costs
one request rather than one per patch. That is a distinguishable-error problem,
not a retry problem, and getting it wrong converts a real validation failure
into a silent double round trip. Callers that need to run against an endpoint
without native `PATCH` should pin `PatchStrategy::ClientSide`.

## Scope and known limitations

- **Equivalence is asserted against the in-memory emulator**, whose patch
  handler evaluates the operation list with the same `apply_patch_ops` the
  client-side path uses. The suite therefore pins that the two *driver* paths
  agree; it does not verify that either agrees with the real service's
  evaluator on error shape or sub-status. That parity is unverified and needs a
  live-service tier.
- **No runtime fallback**, as above.

## References

- PATCH handler spec: ../PATCH_HANDLER_SPEC.md
- Retry classification: ../ErrorCodesAndRetries.md
- Cosmos DB partial document update:
  <https://learn.microsoft.com/azure/cosmos-db/partial-document-update>
- REST contract:
  <https://learn.microsoft.com/rest/api/cosmos-db/patch-a-document>
