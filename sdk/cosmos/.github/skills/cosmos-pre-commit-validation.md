---
name: cosmos-pre-commit-validation
description: >
  Run pre-commit checks for a specific set of crates. Use this when validating changes under sdk/cosmos before committing or during code review.
disable-model-invocation: true
arguments:
  scope:
    type: string
    required: false
    default: all
    description: >
      Crate to run Cosmos SDK pre-commit checks against. `all` means all crates under sdk/cosmos.
  changed-only:
    type: boolean
    required: false
    default: true
    description: >
      If true, only validate files changed in the current git working tree.
  auto-fix:
    type: boolean
    required: false
    default: true
    description: >
      If true, auto-fix any error found.
argument-hints:
  scope:
    - azure_data_cosmos
    - azure_data_cosmos_native

  changed-only:
    - true
    - false

  auto-fix:
    - true
    - false
---
# Cosmos SDK Pre-Commit Checks

## When to use this skill

Use this skill when:

- Reviewing or validating changes in the Cosmos SDK
- Running pre-commit checks locally before pushing
- Performing focused code review on `sdk/cosmos/**`

## Behavior

Follow these steps strictly:

1. Determine the target path:
   - If the `scope` argument is specified and is not equal (case-insensitive) to `all` or `*`, set the target path to `sdk/cosmos/<scope>` (for example, if `scope` is `azure_data_cosmos`, use `sdk/cosmos/azure_data_cosmos` as the target path).
   - Otherwise, set the target path to `sdk/cosmos`.

2. Determine file scope:
   - Always include `sdk/cosmos/tests` in the validation scope (if it exists)
   - If `changed-only` is true, restrict checks to files under the target path and `sdk/cosmos/tests` (for example, using `git diff --name-only -- <target path> sdk/cosmos/tests` or by filtering `git diff --name-only` results to those paths)
   - Otherwise, scan the entire target path and `sdk/cosmos/tests`

3. Validate using the Pre-Completion Validation Checklist in `sdk/cosmos/AGENTS.md`:
   - Formatting checks
   - Build succeeds for affected crates
   - Clippy lints pass for affected crates
   - Documentation builds successfully where applicable
   - Unit and emulator tests relevant to the touched modules and crates

4. Report results:
   - Summarize failures concisely
   - Include exact file paths and commands to reproduce
   - Do NOT auto-fix unless `auto-fix` argument is `true`

## Notes

- Never run repo-wide checks outside `sdk/cosmos`
- Avoid long-running integration tests unless explicitly requested
