# generate_api

`generate_api` is a CLI for generating public API artifacts for Rust crates in this repository.

## Usage

Run from the repository root:

```sh
cargo run --manifest-path eng/tools/Cargo.toml -p generate_api -- \
  --manifest-path sdk/core/azure_core/Cargo.toml \
  --output target/generate_api/azure_core
```

### Arguments

- `--manifest-path <path>`: path to the target crate's `Cargo.toml`
- `--format <markdown|apiview>`: optional output format to generate; defaults to `markdown`
- `--no-docs`: omit documentation comments: APIView doc tokens, or the Markdown comments patch
- `--no-map`: omit the Markdown source map; accepted as a no-op for APIView output
- `--check`: compare generated content with existing output files without writing them
- `--output <dir>`: directory where generated files are written

### Outputs

- default `markdown` output writes `API.md`, `API.md.map`, and `API.comments.patch`
- `--no-docs` skips `API.comments.patch`
- `--no-map` skips `API.md.map`
- `--format apiview` writes `apiview.json`
- `--format apiview --no-docs` writes `apiview.json` without doc comment tokens
- `--check` succeeds when output files are absent or match after normalizing line endings; a mismatch
  writes an error to stderr and exits with code `1`

Both formats include available Cargo metadata (`description`, `edition`, and `rust-version`) and
the crate's default and docs.rs feature list (or all features when not configured). Markdown also
starts with the crate name. Multiline descriptions render with the `Description` label on its own
line before the description text. Children of the `default` feature are also listed. `API.md` never
contains
documentation comments.
`API.comments.patch` is a unified diff that adds them back, so it can be applied to toggle
documentation comments on:

```sh
patch -p1 < API.comments.patch
```

`API.md.map` is an ECMA-426 source map that maps declaration lines in the fenced Rust API block
back to the corresponding item or member declarations. Entries in `sources` are always relative
to the repository root. The generated `sourceRoot` is the relative path from the `--output`
directory to the repository root when the output directory is inside the repository. `sourceRoot`
is omitted when the output directory is outside the repository.

Source-map consumers differ in how they resolve `sourceRoot`. If the application opens the source
map with the repository root as its base, remove `sourceRoot`. If the application cannot resolve
the generated relative `sourceRoot`, replace it with the absolute path to the repository root.
Alternatively, keep the generated relative `sourceRoot` and the source map at the same relative
location within an unchanged repository directory structure; the source paths will then continue
to resolve relative to `API.md.map` or a custom map path. Keep the `sources` entries unchanged in
all cases.

## Workflow

The current API review caller chain under `eng/pipelines/` is:

1. `eng/pipelines/pr.yml` or `eng/pipelines/pullrequest.yml`
2. `eng/pipelines/templates/stages/archetype-sdk-client.yml`
3. `eng/pipelines/templates/jobs/ci.yml`
4. `eng/pipelines/templates/jobs/pack.yml`
5. `eng/scripts/Pack-Crates.ps1`

From that path:

- `Pack-Crates.ps1` runs `generate_api --format apiview` for each packed crate.
- The staged package artifact keeps the existing downstream shape by renaming that output to
  `<package>/<package>.rust.json`.
- The shared `create-apireview` pipeline step consumes that staged JSON artifact.

For local testing, `Pack-Crates.ps1 -APIReview` temporarily switches to markdown generation and
writes `API.md` into each crate root directory. Pipelines do not set `-APIReview` today.

## Toolchain

The tool reads `eng/tools/rust-toolchain.toml` and invokes:

```sh
cargo +nightly-2026-04-14 rustdoc -Z unstable-options --output-format json
```
