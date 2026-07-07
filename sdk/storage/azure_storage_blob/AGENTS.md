# AGENTS.md – Blob Storage crate guidance

This file provides AI-agent guidance for work under `sdk/storage/azure_storage_blob/*`.
It supplements repository-root guidance and `sdk/storage/AGENTS.md` with Blob Storage crate-specific expectations.

## Scope

- Applies to the `azure_storage_blob` crate.
- Use this file together with the repository root `AGENTS.md` and `sdk/storage/AGENTS.md`.
- When instructions conflict, follow the more specific guidance in this directory for blob-specific code.

## Crate focus

- This crate implements the Azure Blob Storage client library.
- Keep behavior and API shape aligned with Blob Storage concepts such as containers, blobs, snapshots, versions, leases, tags, block operations, and listing semantics.
- Prefer consistency with adjacent blob operations over introducing a new pattern for a single API.

## API design conventions

- Keep public APIs ergonomic and predictable for common blob workflows.
- Follow existing naming and module patterns for container-level and blob-level operations.
- Prefer builder/options patterns for optional request parameters.
- Keep request and response model names aligned with service terminology and neighboring blob operations.
- Preserve pagination behavior and continuation token handling conventions used elsewhere in the crate.

## Request construction and protocol details

- Ensure optional blob request parameters are threaded consistently into headers and query parameters.
- Be careful with conditional request semantics (`If-Match`, `If-None-Match`, modified-since, lease conditions, etc.).
- Treat versioning, snapshots, ranges, content headers, metadata, and tags as protocol-sensitive areas that need targeted tests.
- Keep URL/path handling and resource naming consistent with existing blob request builders.
- Avoid duplicating request-building logic when shared helpers or existing operation patterns already exist.

## Data model and compatibility

- Prefer additive changes to public request/response types.
- Avoid breaking changes to public structs, enums, or method signatures unless explicitly intended.
- Keep deserialization resilient to optional or newly added service fields where existing crate patterns allow.
- Maintain backward-compatible behavior for existing defaults and unset optional fields.

## Authentication and authorization considerations

- Respect existing credential flows supported by the storage libraries.
- For SAS- or signature-sensitive changes, verify that new headers/query parameters are included where required by existing signing/auth behavior.
- Do not introduce blob-specific auth behavior that diverges from shared storage crate expectations without clear justification.

## Testing expectations

- Add unit tests for request construction, option propagation, and serialization/deserialization behavior.
- Add or update recorded/live tests for service behaviors that cannot be validated through isolated unit tests.
- Favor narrow regression tests for blob-specific semantics like ranges, metadata, tags, conditions, leases, copy flows, and listing behavior.
- Reuse existing blob test helpers, fixtures, and crate patterns before adding new infrastructure.

## Common blob-specific pitfalls

- Inconsistent handling of container vs blob scopes.
- Missing propagation of optional headers or query parameters.
- Incorrect continuation token behavior for listing operations.
- Breaking response or option shapes for convenience refactors.
- Incomplete coverage for lease, conditional, range, snapshot, or version-related behavior.
- Subtle behavior drift between similar operations that should remain consistent.

## Suggested file and code reading order

When making a blob-specific change:

1. Start from the neighboring operation/module that most closely matches the target behavior.
2. Reuse existing option, request-building, and response-model patterns where possible.
3. Check tests for adjacent operations before inventing a new testing style.
4. If shared logic is needed, prefer factoring into existing internal helpers over copying code.

## Agent task playbooks

### 1) Add a new optional blob request option

1. Extend the relevant options/builder type.
2. Document the option clearly and keep the default behavior unchanged.
3. Thread the option into header/query construction in one place.
4. Add unit tests for the serialized request shape.
5. Add service-level coverage if the behavior is protocol-sensitive.

### 2) Add or adjust blob listing behavior

1. Match existing listing APIs and terminology.
2. Preserve continuation token semantics.
3. Verify option propagation for both initial and continuation requests.
4. Add regression coverage for any changed filtering/detail flags.

### 3) Update blob response parsing

1. Keep public model changes additive whenever possible.
2. Validate behavior with representative payloads, including partial/optional fields.
3. Add regression tests for service-specific edge cases.

### 4) Add a new blob/container operation

1. Follow neighboring module naming and layout.
2. Keep request/options/response types aligned with crate conventions.
3. Add concise docs and examples when the surrounding crate pattern expects them.
4. Add unit and integration-style tests consistent with similar operations.

## Before handoff

From repo root, run or recommend the checks most relevant to the crate:

- `cargo fmt`
- `cargo clippy -p azure_storage_blob --all-targets --all-features`
- `cargo test -p azure_storage_blob`

## When in doubt

- Follow existing `azure_storage_blob` patterns first.
- Prefer consistency with neighboring blob APIs over introducing a one-off abstraction.
- If a change seems broadly applicable across storage crates, reflect that in `sdk/storage/AGENTS.md` rather than only here.
