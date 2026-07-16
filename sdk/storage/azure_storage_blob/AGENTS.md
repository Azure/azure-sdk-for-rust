# AGENTS.md – Blob Storage crate guidance

This file provides blob-specific guidance for work under `sdk/storage/azure_storage_blob/*`.

Use this together with:
- repository root `AGENTS.md`
- `sdk/storage/AGENTS.md`

If guidance conflicts, follow the more specific instructions in this directory.

## Scope

- Applies only to the `azure_storage_blob` crate.
- Do not repeat storage-wide guidance here; keep shared conventions in `sdk/storage/AGENTS.md`.

## Blob-specific focus areas

- Keep behavior aligned with Blob Storage concepts: containers, blobs, snapshots, versions, leases, tags, ranges, block operations, and listing semantics.
- Prefer consistency with neighboring blob/container operations over introducing one-off patterns.

## Protocol-sensitive areas

Be especially careful with:

- conditional request semantics (lease conditions)
- snapshot and version handling
- range requests and content headers
- metadata and tags
- continuation tokens for listing operations
- container-scope vs blob-scope behavior

## Blob-specific expectations

- Start from the closest existing blob or container operation and mirror its API, request construction, and tests.
- Reuse shared helpers and existing request-building patterns before introducing new abstractions.
- Keep public model changes additive unless a breaking change is explicitly intended.

## Blob-specific pitfalls

- Mixing up container-level and blob-level semantics
- Missing propagation of optional headers or query parameters
- Incorrect continuation token handling in listing flows
- Inconsistent behavior across similar blob operations
- Insufficient coverage for lease, conditional, range, snapshot, version, metadata, or tag behavior

## Suggested workflow for changes

1. Find the most similar existing blob/container operation.
2. Reuse its options, request construction, and response patterns where possible.
3. Add focused regression tests for the blob-specific protocol behavior being changed.
4. If guidance is broadly applicable to all storage crates, move it to `sdk/storage/AGENTS.md` instead of duplicating it here.
