# sdk-reviewer distillation plan

Goal: regenerate low-token custom-agent instructions from Azure Rust guidelines with priority on correctness and compactness.

## Inputs (required)

1. `https://azure.github.io/azure-sdk/rust_introduction.html` (highest priority)
2. `https://azure.github.io/azure-sdk/rust_implementation.html`
3. Same-site linked guidance only when needed to resolve gaps/conflicts:
   - `general_introduction.html`
   - `general_design.html`
   - `general_implementation.html`
   - `general_documentation.html`
   - `policies_support.html`

Stop and report if any required URL is inaccessible.

## Distillation process

1. Extract normative statements and map modal verbs:
   - DO => MUST
   - DO NOT => MUST NOT
   - YOU SHOULD / SHOULD NOT => SHOULD / SHOULD NOT
   - YOU MAY => MAY
2. De-duplicate semantically equivalent rules; keep the strictest form.
3. Keep Rust-intro rules over other sources when overlap exists.
4. Convert long prose/examples into compact reviewer checks.
5. Keep only merge-signal rules: API compatibility, security/PII, dependency/runtime constraints, module/export layout, model/error/paging/LRO contracts, packaging metadata, generated boundaries.
6. Remove low-value narrative/history/background.
7. Normalize output into compact sections:
   - MUST
   - MUST NOT
   - SHOULD
   - MAY
   - Severity rubric
   - Output format
8. Keep wording short, imperative, non-redundant, machine-oriented.
9. Validate no contradictions against current repo conventions/workflows.

## Maintenance checklist

- Re-fetch sources each refresh (do not rely on stale summaries).
- Update rules if upstream guideline wording changed.
- Preserve priority ordering (rust_introduction first).
- Verify agent still references allowed repos/docs in workflow.
- Recompile affected agentic workflow lock files after changes.
