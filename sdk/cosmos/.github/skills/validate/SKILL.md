---
name: validate
description: Validate Cosmos SDK changes before considering a task complete.
---

# Validate Cosmos SDK changes

Run these checks from the repository root before completing a task under
`sdk/cosmos`.

## Choose the scope

- Prefer the smallest useful command first: use `-p <package>` when a change is
  isolated to one package.
- Repeat each package command for every affected package. A successful check of
  one package does not cover other affected packages.
- Use `--workspace` when changes span packages, alter shared workspace or Cargo
  configuration, or make the complete affected package set uncertain.

## Required checks

For each affected package, run:

```bash
cargo check -p <package> --all-features --all-targets
cargo clippy -p <package> --all-features --all-targets
cargo test -p <package> --all-features
```

For workspace-scoped changes, run:

```bash
cargo check --workspace --all-features --all-targets
cargo clippy --workspace --all-features --all-targets
cargo test --workspace --all-features
```

Format the entire workspace:

```bash
cargo fmt --all
```

Invoke the project `check-spelling` workflow, including new untracked files.

On Linux or macOS with Bash:

```bash
{
    git diff --staged --name-only --diff-filter=d
    git diff --name-only --diff-filter=d
    git ls-files --others --exclude-standard
} | sort -u | ./eng/common/spelling/Invoke-Cspell.ps1
```

On Windows with PowerShell:

```powershell
$files = @(
    git diff --staged --name-only --diff-filter=d
    git diff --name-only --diff-filter=d
    git ls-files --others --exclude-standard
) | Sort-Object -Unique

$files | ./eng/common/spelling/Invoke-Cspell.ps1
```

Fix failures and rerun the failed command. Do not suppress failures, narrow the
scope to avoid them, or report completion while a required check is failing. If
a failure is unrelated or cannot be fixed within the task, report the command
and failure clearly and treat the task as blocked.

## Documentation-only changes

For changes limited to documentation or other non-Rust prose, package-specific
checks, Clippy, and tests are not required. Still run `cargo fmt --all`, the
project `lint-markdown` skill for changed Markdown files, and the spelling
command above.

## Behavior changes

Changes to actual behavior, excluding renames, refactors, and documentation-only
changes, must also follow
`sdk/cosmos/.github/skills/emulator-tests/SKILL.md`.
