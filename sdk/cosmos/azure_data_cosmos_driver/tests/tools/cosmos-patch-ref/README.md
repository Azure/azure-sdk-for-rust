# `cosmos-patch-ref` — .NET reference oracle for Cosmos PATCH

**Status**: deferred handoff. This directory currently holds *only* this
README. The actual `cosmos-patch-ref.csproj` is intentionally **not**
checked in yet; this document is the contract a future change should
follow.

## Why

The Rust driver's PATCH pipeline (`patch_handler.rs`) implements the
Read-Modify-Write loop locally — it evaluates each `PatchSpec` against a
fetched document and Replaces the merged image. The fidelity of that
locally-computed post-image is the single largest risk surface in the
feature.

A9 (`cosmos_patch_compare`, see `../../emulator_tests/driver_patch.rs`)
already exercises this end-to-end against an emulator, which catches
evaluator bugs that change the *observable* result. What it cannot
distinguish is a Rust evaluator bug that happens to match the *emulator's*
JSON Patch evaluator — both of those are first-party implementations, and a
shared regression would slip past A9.

`cosmos-patch-ref` closes that gap: it is a small .NET console binary that
wraps `Microsoft.Azure.Cosmos.PatchOperation` and reports what the .NET SDK
*would* compute for the same input. The harness then compares the Rust
post-image byte-for-byte (after canonicalizing key order) with the .NET
oracle.

## Contract

### Invocation

```text
cosmos-patch-ref < stdin.json > stdout.json
```

- The binary reads **a single** JSON object from stdin.
- The binary writes **a single** JSON object to stdout.
- Exit code is `0` on success and non-zero on internal failure of the
  reference itself; an *evaluator* error (e.g. JSON Patch semantic
  failure) is reported via the response body, not the exit code.

### Stdin schema

```json
{
  "initial_doc": { "id": "x", "pk": "p", "...": "..." },
  "ops": [
    { "op": "set",       "path": "/a", "value": 42 },
    { "op": "add",       "path": "/b/-", "value": "x" },
    { "op": "increment", "path": "/c", "value": 7 },
    { "op": "remove",    "path": "/d" },
    { "op": "move",      "from": "/e", "path": "/f" }
  ]
}
```

`ops` uses the exact same `PatchSpec` JSON shape that `PatchOp` serializes
to in Rust. Field names and casing are stable and tested by the Phase 3
serde round-trip suite — do not deviate.

### Stdout schema (success)

```json
{
  "outcome": "post_image",
  "post_image": { "...": "merged document" }
}
```

### Stdout schema (semantic error)

```json
{
  "outcome": "error",
  "error_kind": "missing_path" | "type_mismatch" | "array_index_oor" | "...",
  "message": "freeform diagnostic from the .NET SDK"
}
```

Comparable error taxonomy (must be a closed enum of strings) is required
so the harness can match the *kind* of failure across SDKs rather than
the exact prose.

## Harness wiring

1. **Where**: a new Rust integration test
   `tests/emulator_tests/driver_patch_dotnet_compare.rs` would shell out
   to the binary, send each fixture from
   `cosmos_patch_compare_coverage.md`, and compare results.
2. **How**: prefer `tokio::process::Command` with stdin piping; do not
   rely on filesystem temp files.
3. **Discovery**: the harness should locate the binary via an env var
   (`COSMOS_PATCH_REF_BIN`) set by `Test-Setup.ps1`. If the env var is
   unset, the comparison test should `#[ignore]` itself with a clear
   message — never fail.

## Why this is deferred

- The binary is a build target in a foreign toolchain (.NET).
  Wiring it through `eng/` and CI is a separate work item.
- A9 already provides end-to-end coverage against the emulator; the .NET
  oracle is a *strengthening* signal, not a gating one.
- The fixture schema in `driver_patch.rs::PatchCompareCase` is already
  designed to be reflowed into the stdin format above with zero changes
  to test bodies, so this deferral does not create rework.

## Acceptance checklist for the future PR

- [ ] `cosmos-patch-ref.csproj` builds locally and on CI for the same
      .NET SDK version pinned by the Cosmos team.
- [ ] `tests/tools/cosmos-patch-ref/README.md` (this file) is updated
      with concrete build / run commands.
- [ ] `Test-Setup.ps1` is **appended** (never overwritten) to publish
      `COSMOS_PATCH_REF_BIN`.
- [ ] `driver_patch_dotnet_compare.rs` exercises every row from
      `cosmos_patch_compare_coverage.md` and fails on the first
      post-image mismatch with a printable diff.
- [ ] A new entry in `.coding-harness/implementation-state.json#commits[]`
      records the .NET oracle landing; the
      `deferred_scope[].dotnet_helper` entry is removed.
