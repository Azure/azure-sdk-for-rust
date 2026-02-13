---
name: cosmos-pre-commit-validation
description: >
  Run pre-commit checks for specific set of crates. Use this when validating changes under sdk/cosmos before committing or during code review.
disable-model-invocation: true
arguments:
  scope:
    type: string
    required: false
    default: all
    description: >
      Crate to run Cosmos SDK pre-commit checks against. `All` means all crates under sdk/cosmos.
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
    - azure_data_cosmos_driver
    - azure_data_cosmos_native
    - sdk/cosmos/tests

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
   - If the `scope` argument is specified and is not equal (case-insensitive) to `all` or `*`, set the target path to `sdk/cosmos/<scope>` (for example, if `scope` is `helloworld`, use `sdk/cosmos/helloworld` as the target path).
   - Otherwise, set the target path to `sdk/cosmos`.

2. Determine file scope:
   - If `changed-only` is true, restrict checks to files under the target path (for example, using `git diff --name-only -- <target path>` or by filtering `git diff --name-only` results to that path)
   - Otherwise, scan the entire target path

3. Validate using Key Workflows in AGENTS.md
   - Formatting checks
   - Linting
   - Unit tests relevant to the touched modules
   - Emulator tests relevant to touched crates

4. Report results:
   - Summarize failures concisely
   - Include exact file paths and commands to reproduce
   - Do NOT auto-fix unless `auto-fix` argument is `true`

## Notes

- Never run repo-wide checks outside `sdk/cosmos`
- Avoid long-running integration tests unless explicitly requested
