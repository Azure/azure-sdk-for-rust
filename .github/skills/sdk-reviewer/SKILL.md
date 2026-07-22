---
name: sdk-reviewer
description: Review Azure SDK for Rust changes for API, packaging, and repo conformance.
---

# SDK Reviewer

Use this skill to review Rust SDK changes with Azure SDK for Rust rules.

## Scope

- Prioritize high-signal issues only (correctness, API compatibility, packaging/release readiness, security).
- Ignore style-only nits unless they break repo policy.

## Review checklist

1. **Public API surface**
   - Verify new/changed public APIs follow Azure Rust guidance and crate patterns.
   - Flag potential breaking changes and missing docs on public APIs.

2. **Crate structure and naming**
   - Confirm crate naming conventions (`azure_*`, `azure_resourcemanager_*` for mgmt) and `sdk/<service>/<crate>` layout.
   - Confirm required service metadata exists where applicable (`ci.yml`, `tsp-location.yaml`, assets/test-resource files).

3. **Workspace and Cargo consistency**
   - Confirm new crates are added to root workspace membership.
   - Prefer workspace-inherited dependencies in `sdk/*/Cargo.toml` unless a justified local path/version exception is needed.

4. **Generated-code boundaries**
   - Do not require manual edits under `generated/`; ask for TypeSpec/spec regeneration path instead.

5. **Security and secrets**
   - Flag obvious credential exposure or insecure defaults introduced by the change.

## Tooling expectations

- Use Rust LSP (`rust-analyzer`) when available to verify symbol definitions/references and API impact.
- If LSP is unavailable, fall back to file-level analysis and explicitly state that limitation.
- When TypeSpec-generated SDK shape is relevant, compare with source under `Azure/azure-rest-api-specs` when accessible.

## Output format

Return:

- **Findings**: bullet list ordered by severity.
- **Release-readiness checks**: pass/fail bullets for API, workspace/Cargo, and metadata gates.
- **Required follow-ups**: only concrete actions needed before merge.
