---
name: export-api
description: Generate API.md artifacts for changed Rust crates after code generation or source edits.
---

# Export API artifacts

Run this after regenerating sources or changing Rust code that can affect a crate's public API.

## Default workflow

1. Collect changed non-deleted files from Git. By default, use both staged and unstaged local changes:

   ```bash
   { git diff --staged --name-only --diff-filter=d; git diff --name-only --diff-filter=d; git ls-files --others --exclude-standard -- 'sdk'; } | sort -u
   ```

2. If the user wants branch-based detection instead, diff against the merge base with `main`:

   ```bash
   base="$(git merge-base main HEAD)"
   git diff --name-only --diff-filter=d "$base"...HEAD | sort -u
   ```

   This branch-based mode only considers tracked branch changes; use the default local-change mode
   when untracked files should also trigger API regeneration.

3. Assume service crates live under `sdk/<service-directory>/<crate-name>`. For each changed file under `sdk/`,
   derive the candidate crate directory from the first three path segments and keep it only when
   `<crate-directory>/Cargo.toml` exists.

4. For each affected crate, run the explicit generator command from the repository root:

   ```bash
   cargo run --manifest-path eng/tools/generate_api/Cargo.toml -- \
     --manifest-path <crate>/Cargo.toml \
     --output <crate>/api \
     --no-docs \
     --no-map
   ```

## Optional outputs

- To include `API.comments.patch`, omit `--no-docs`.
- To include `API.md.map`, omit `--no-map`.

## No changed crates

If no changed files map to a crate directory under `sdk/<service-directory>/<crate-name>`, stop and
report that no crate API artifacts need regeneration.

## Validation

After generation, check the resulting `api/API.md` and `api/API.metadata.yml` files in `git diff`
and report which crates were updated.
