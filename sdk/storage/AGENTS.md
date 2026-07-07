# AGENTS.md – Storage crates guidance

This file provides AI-agent guidance for work under `sdk/storage/*`.
It complements the repository root guidance and should be read together with root-level instructions.

## Scope

- Applies to all storage crates under `sdk/storage/*`.
- Crate-specific details should live in each crate folder's own `AGENTS.md` (for example, `sdk/storage/azure_storage_blob/AGENTS.md`).

## Storage family map

- `azure_storage` – shared storage primitives and common concepts.
- `azure_storage_blob` – Blob Storage client library.
- Other storage crates (queue/file/share/etc.) should follow the same design and testing patterns where applicable.

## Architecture and API conventions

- Keep public API style consistent across storage crates.
  - Prefer clear builder-based option patterns for operations with optional parameters.
  - Keep method naming aligned with service operations and existing crate naming.
  - Preserve pagination ergonomics and predictable return types.
- Prefer composition and reusable helpers in internal modules over duplicating request-building code.
- Keep transport/pipeline usage aligned with `azure_core` patterns used in the repo.

## Error handling

- Use `azure_core::Result<T>` for fallible public operations.
- Map service/HTTP failures consistently to `azure_core::Error` and existing error categorization patterns.
- Preserve retry behavior expectations; avoid introducing ad-hoc retry logic in operation methods.

## Authentication and request setup

- Respect existing credential flows used by storage crates (AAD, SAS, connection string, shared key where supported).
- Ensure new options are correctly represented in headers/query strings and signed/authenticated where required.
- For cross-crate auth behavior changes, keep semantics consistent across storage crates.

## Generated vs hand-written code

- Do **not** manually edit generated code paths/files when generation is the source of truth.
- If a change must be generated, update the source inputs and regenerate according to repo guidance.
- Keep hand-written customization in the intended extension points.

## Testing strategy

- Add/maintain unit tests for isolated behavior and serialization/parsing logic.
- Add/maintain recorded/live tests for service-integration behavior where appropriate.
- Reuse existing test helpers and fixtures before adding new ones.
- Keep tests deterministic and minimize time/network sensitivity.

## Quality bar before handoff

From repo root (or scoped to affected crates as appropriate):

- `cargo fmt`
- `cargo clippy -p <crate> --all-targets --all-features`
- `cargo test -p <crate>`

When touching multiple crates, run checks for each affected crate.

## Common pitfalls

- Inconsistent option naming across similar operations/crates.
- Breaking changes to public API shape for convenience-only refactors.
- Missing tests for new optional headers/query parameters.
- Diverging behavior between similar storage operations in different crates.

## Agent task playbooks

### 1) Add a new optional request header/parameter

1. Locate the operation options/builder type.
2. Add the new field with clear docs and sensible default (`None` when optional).
3. Thread it into request construction (header/query) in one place.
4. Add unit tests for request construction and a service-level test when required.
5. Verify no behavioral change when option is unset.

### 2) Add a new listing/pagination option

1. Extend options type and docs.
2. Apply to request query construction.
3. Validate continuation token behavior remains unchanged.
4. Add tests for first and continuation requests where possible.

### 3) Adjust response model/deserialization

1. Keep backward compatibility for public fields/types unless intentionally versioned.
2. Prefer additive changes.
3. Add regression tests for missing/extra/unknown fields as needed.

### 4) Update retryable error classification

1. Align with existing `azure_core`/pipeline strategy.
2. Avoid operation-specific special casing unless unavoidable and documented.
3. Add tests for representative status/error cases.

### 5) Add a new operation end-to-end

1. Follow existing operation module layout and naming.
2. Define request/options/response types consistent with neighboring operations.
3. Add docs with minimal, runnable-style usage examples when practical.
4. Add unit tests and recorded/live tests where the crate pattern expects them.

## When in doubt

- Follow adjacent storage crate patterns first.
- Prefer consistency with existing public APIs over introducing a new style.
- Document assumptions in code comments or PR notes when behavior is service-specific.
