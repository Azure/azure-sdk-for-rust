# AGENTS.md – Storage crates guidance (minimal)

Storage-specific guidance for `sdk/storage/*`.

Root `AGENTS.md` is authoritative for repo-wide rules (generated code, compatibility, security, lint/test/CI, and general coding standards). Do not repeat those rules here.

## Scope

- Applies to all crates under `sdk/storage/*`.
- Put crate-specific details in each crate’s local `AGENTS.md`.

## Storage consistency requirements

- Keep API shape consistent across storage crates:
  - Similar operations should use similar option/builder patterns.
  - Use consistent naming for equivalent concepts.
  - Keep pagination behavior/ergonomics consistent.

- Reuse shared request-construction helpers where possible; avoid copy/paste request logic.

- Preserve storage auth semantics across crates (AAD, SAS, connection string, shared key where supported), including correct header/query threading and signing behavior.

## Tests (storage-focused)

- Add unit tests for request construction and option threading.
- Add/maintain recorded or live tests for service behavior when expected by crate patterns.
- Guard pagination/continuation behavior with tests.

## Common storage pitfalls

- Option naming drift across crates.
- Behavior drift for equivalent operations in different crates.
- Missing tests for newly added headers/query params.
- Pagination regressions (continuation tokens / next-page behavior).

## Minimal playbooks

### Add optional header/query option
1. Add to options/builder with default `None`.
2. Thread into request construction once (single canonical path).
3. Add request-construction tests.
4. Confirm no behavior change when unset.

### Add pagination option
1. Extend options/docs.
2. Apply in query construction.
3. Verify continuation behavior unchanged.
4. Add first-page + continuation tests.

### Change response/deserialization
1. Prefer additive/backward-compatible changes.
2. Add regression tests for compatibility edge cases.

### Add operation
1. Follow neighboring storage operation structure and naming.
2. Keep request/options/response style aligned with adjacent operations.
3. Add unit + service-level tests per crate conventions.

## When unsure

Prefer existing patterns in adjacent storage crates and document any service-specific deviations.
