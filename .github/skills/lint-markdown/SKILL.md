---
name: lint-markdown
description: Check and fix formatting and other issues in markdown files using markdownlint-cli2.
---

# Markdown linting

Check markdown files for common mistakes.

## Installation and usage

Run `npm install --dev` from this skill directory (`.github/skills/lint-markdown/`) first. Then run commands using `npx --prefix .github/skills/lint-markdown markdownlint-cli2 <command>` from the repository root.

## Configuration

Configuration is in `.markdownlint-cli2.yaml` files. The root configuration is at the repository root.

For markdownlint rules configuration, nest it under the `config` property following the markdownlint schema at `https://raw.githubusercontent.com/DavidAnson/markdownlint/main/schema/markdownlint-config-schema.json`. Use friendly rule names (e.g., `line-length`) instead of codes (e.g., `MD013`).

## Check Markdown

Run `npx --prefix .github/skills/lint-markdown markdownlint-cli2` from the repository root to lint Markdown files according to the configuration.

## Fix issues

Run with the `--fix` flag to automatically fix supported issues:

```bash
npx --prefix .github/skills/lint-markdown markdownlint-cli2 --fix
```

## Testing

Run the same lint command again to verify all issues are fixed. There should be no errors reported.
