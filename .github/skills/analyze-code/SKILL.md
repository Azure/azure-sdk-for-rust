---
name: analyze-code
description: Run repository code analysis as a final check after completing prompts, targeting every changed Rust package by PackageNames.
---

# Analyze changed packages

Run this as a final check after completing any prompt that changes repository files.

1. Collect changed files from Git, including staged, unstaged, untracked, and deleted files:

   ```bash
   { git diff --staged --name-only; git diff --name-only; git ls-files --others --exclude-standard -- 'sdk'; } | sort -u
   ```

2. Assume service crates live under `sdk/<service-directory>/<crate-name>`. For each changed file under `sdk/`,
   derive the candidate crate directory from the first three path segments and keep it only when
   `<crate-directory>/Cargo.toml` exists. Include any crate names specified in the prompt, then collect and
   deduplicate all `<crate-name>` values as package names.

3. Run the analyzer once from the repository root with all changed package names:

   ```powershell
   ./eng/scripts/Analyze-Code.ps1 -PackageNames @('<crate-name-1>', '<crate-name-2>')
   ```

If neither changed files nor the prompt identify a service crate, report that no packages need analysis.
Otherwise, report the packages analyzed and any failures.
