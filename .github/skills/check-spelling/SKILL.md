---
name: check-spelling
description: Check and fix spelling in project source files using cSpell.
---

# Spell checking

## Usage

Get all staged and unstaged non-deleted files, then pipe them to the spell check script:

```bash
{ git diff --staged --name-only --diff-filter=d; git diff --name-only --diff-filter=d; } | sort -u | ./eng/common/spelling/Invoke-Cspell.ps1
```

## Configuration

The root configuration is `.vscode/cspell.json`, and CI runs cSpell with this config via `eng/common/spelling/Invoke-Cspell.ps1`.

Each service directory (e.g., `sdk/{service-directory}`) or crate directory (e.g., `sdk/{service-directory}/{crate-directory}`) can have a `.cspell.json` that should `import` either a parent `.cspell.json` or `.vscode/cspell.json` from the root of the repo. Per-directory configs are loaded based on the paths of files being checked.

## Fix spelling

Show a summary of the misspellings to the user. Prompt the user for which words should be replaced and which should be ignored.

If you cannot confidently determine the correct spelling, ask the user.

For each flagged word, consider one of the following:

1. **It's a real error** — fix the misspelling directly in the source file.
2. **It's a domain-specific term** (e.g., "RNTBD" in CosmosDB) unlikely to appear in other services — add it to the `ignoreWords` list in `sdk/{service}/.cspell.json`. Keep that list sorted alphabetically.
3. **It's a term that applies to multiple services** (e.g., "upsert") — add it to the `words` list in `.vscode/cspell.json`. Keep that list sorted alphabetically.

cSpell is case-insensitive, so you don't need to worry about the casing of ignored or added words.

Example `sdk/keyvault/.cspell.json`:

```json
{
  "import": [
    "../../.vscode/cspell.json"
  ],
  "ignoreWords": [
    "ciphertext",
    "purgeable"
  ]
}
```

Seldom used words can be ignored within the file they are used by adding an appropriate comment e.g.:

```js
// cspell:ignore <word>
```

## Testing

Run the same command again used to check spelling. All misspellings should be fixed.
