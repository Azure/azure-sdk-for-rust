---
name: pre-push-verify
description: Run pre-push CI verification checks locally and collect results.
---

# Pre-push verification

Run `eng/scripts/Invoke-PrePush.ps1` to execute the same analysis, testing, spell check, and link verification steps that the pull request CI pipeline performs. The script detects which packages changed relative to a target branch (default: `main`) and runs checks only for those packages.

## Usage

Run from the repository root:

```pwsh
./eng/scripts/Invoke-PrePush.ps1
```

To collect machine-readable results and per-step log files, pass `-OutputDir`:

```pwsh
./eng/scripts/Invoke-PrePush.ps1 -OutputDir ./target/prepush
```

When `-OutputDir` is set the script writes:

- `<step>.log` — merged stdout/stderr for each step
- `summary.json` — overall result and per-step status (`passed`, `failed`, or `skipped`)

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `-PackageNames` | _(detected)_ | Explicit list of package names to check instead of auto-detecting from the diff |
| `-Toolchain` | `stable` | Rust toolchain passed to analysis and test steps |
| `-TargetBranch` | `main` | Branch to diff against when detecting changed packages |
| `-OutputDir` | _(none)_ | Directory for per-step log files and `summary.json` |
| `-SkipAnalysis` | `false` | Skip `Analyze-Code.ps1` (cargo check, fmt, clippy, doc, audit) |
| `-SkipTests` | `false` | Skip `Test-Packages.ps1` |
| `-SkipSpellCheck` | `false` | Skip spell check on changed files |
| `-SkipLinkVerification` | `false` | Skip link verification on changed markdown files |

## Collecting results

After the script finishes, read `summary.json` to determine overall success and which steps failed:

```pwsh
$summary = Get-Content ./target/prepush/summary.json | ConvertFrom-Json
$summary.result        # "passed" or "failed"
$summary.steps         # array of { name, status, log } objects
```

Inspect the log file for any failed step:

```pwsh
foreach ($step in $summary.steps | Where-Object { $_.status -eq 'failed' }) {
    Write-Host "=== $($step.name) ==="
    Get-Content ./target/prepush/$($step.log)
}
```

## Steps performed

1. **setup** — generates a PR diff and package-info files that identify changed packages
2. **analysis** — runs `Analyze-Code.ps1` (cargo check, fmt, clippy, doc, audit) for changed packages
3. **tests** — runs `Test-Packages.ps1` for changed packages (skipped when none are detected)
4. **spellcheck** — runs cSpell on files changed relative to the target branch
5. **linkcheck** — verifies links in changed markdown files (skipped when no markdown files changed)

All steps run to completion even when earlier steps fail, so a single invocation reveals every issue at once.

## Interpreting output

The script prints a summary table at the end:

```text
===========================================
  Pre-push verification summary
===========================================
  [PASS] analysis
  [FAIL] tests  -> tests.log
  [SKIP] linkcheck
-------------------------------------------
  1 passed, 1 failed, 1 skipped
===========================================
```

Exit code is `0` when all executed steps pass, and `1` when any step fails.
