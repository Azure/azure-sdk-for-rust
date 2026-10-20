---
name: lint-markdown
description: Check and fix formatting and other issues in markdown files using markdownlint-cli2.
---

# Markdown linting

Check markdown files for common mistakes.

## Installation and usage

Find the nearest `package.json` in the current directory or an ancestor through the
repository root that lists `markdownlint-cli2` in `devDependencies`. Run `npm ci`
from that directory, then run `npx markdownlint-cli2 <command>` from the current
directory.

## Configuration

Configuration is in the repository root `.markdownlint-cli2.yaml`.

Use the schema matching the installed `markdownlint-cli2` version. Nest markdownlint
rules under `config` and use friendly names such as `line-length` instead of `MD013`.

## Check Markdown

Lint an explicit list of modified markdown files:

```bash
npx markdownlint-cli2 --no-globs <files...>
```

## Fix issues

Fix supported issues in an explicit list of files:

```bash
npx markdownlint-cli2 --no-globs --fix <files...>
```

## Testing

Run the check command again. Report any remaining issues.
