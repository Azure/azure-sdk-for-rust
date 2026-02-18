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
      If true, auto-fix any errors found.
argument-hints:
  scope:
    - azure_data_cosmos
    - azure_data_cosmos_driver
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
   - If `changed-only` is `true` (the default), restrict scanning to `.rs` files that differ between the current local branch and `main`. Use `git diff --name-only main -- <target path>` (and include per-crate `tests/` directories) to obtain the list. Only `.rs` files in the result set are scanned; all other files are skipped.
   - If `changed-only` is `false`, scan **all** `.rs` files under the target path(s).
   - In both modes, **skip** files in `generated/` subdirectories — these are produced by external tools and must never be modified.

3. Validate using the Pre-Completion Validation Checklist in `sdk/cosmos/AGENTS.md`:
   - Formatting checks
   - Build succeeds for affected crates
   - Clippy lints pass for affected crates, with warnings treated as errors (`-D warnings`) to match CI behavior.
     Run clippy with `RUSTFLAGS=-D warnings` set:
     - Bash: `RUSTFLAGS='-D warnings' cargo clippy -p <crate> --all-features --all-targets`
     - PowerShell: `$env:RUSTFLAGS='-D warnings'; cargo clippy -p <crate> --all-features --all-targets; $env:RUSTFLAGS=$null`
   - **Re-run formatting** after any auto-fix: if `auto-fix` is true and clippy or other tools modified files,
     re-run `cargo fmt` to ensure the auto-fixed code is properly formatted (e.g., `cargo clippy --fix` can
     leave trailing blank lines when removing unused imports).
   - Documentation builds successfully where applicable
   - **Spell check (cspell)**: CI runs cspell on all changed files using the config at `.vscode/cspell.json`
     with the Cosmos-specific dictionary at `sdk/cosmos/.dict.txt`. Run locally with:
     `npx cspell lint --config .vscode/cspell.json --no-must-find-files <target path>/**`
     If `auto-fix` is true and unknown words are legitimate (e.g., API type names, technical terms),
     add them to `sdk/cosmos/.dict.txt`.
   - Unit and emulator tests relevant to the touched modules and crates
   - **CI-gated test compilation**: Some test files are conditionally compiled via `cfg` flags that CI sets but local builds omit.
     These tests will silently pass `cargo test` locally even if they contain build errors.
     Run the following checks to catch regressions before CI does:
     - `RUSTFLAGS='--cfg test_category="emulator"' cargo check -p azure_data_cosmos --features fault_injection,key_auth --tests`
     - `RUSTFLAGS='--cfg test_category="multi_write"' cargo check -p azure_data_cosmos --features fault_injection,key_auth --tests`
     On Windows (PowerShell), set the env var first: `$env:RUSTFLAGS='--cfg test_category="emulator"'` then run the `cargo check` command, and clear it afterwards with `$env:RUSTFLAGS=$null`.
     These commands only **compile** the test targets — they do not run the emulator or multi-write tests (those require a live Cosmos DB emulator or multi-region account).
     If `scope` targets a specific crate other than `azure_data_cosmos`, skip these checks.

4. Report results:
   - Summarize failures concisely
   - Include exact file paths and commands to reproduce
   - Do NOT auto-fix unless `auto-fix` argument is `true`

## Notes

- Never run repo-wide checks outside `sdk/cosmos`
- Avoid long-running integration tests unless explicitly requested
