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
- `--output <dir>`: directory where generated files are written

### Outputs

- default `markdown` output writes `API.md` and `API.comments.patch`
- `--no-docs` writes only `API.md`
- `--format apiview` writes `apiview.json`
- `--format apiview --no-docs` writes `apiview.json` without doc comment tokens

`API.md` never contains documentation comments. `API.comments.patch` is a unified diff that adds
them back, so it can be applied to toggle documentation comments on:

```sh
patch -p1 API.md < API.comments.patch
```

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
