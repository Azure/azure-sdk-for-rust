# generate_api

`generate_api` is a CLI for generating public API artifacts for Rust crates in this repository.

## Usage

Run from the repository root:

```sh
cargo run --manifest-path eng/tools/Cargo.toml -p generate_api -- \
  --manifest-path sdk/core/azure_core/Cargo.toml \
  --output /tmp/generate_api
```

### Arguments

- `--manifest-path <path>`: path to the target crate's `Cargo.toml`
- `--format <markdown|apiview>`: optional output format to generate; defaults to `markdown`
- `--no-docs`: when generating `apiview`, omit documentation comment tokens
- `--output <dir>`: directory where generated files are written

### Outputs

- default `markdown` output writes `API.md`
- `--format apiview` writes `apiview.json`
- `--format apiview --no-docs` writes `apiview.json` without doc comment tokens

## Toolchain

The tool reads `eng/tools/rust-toolchain.toml` and invokes:

```sh
cargo +nightly-2025-05-09 rustdoc -Z unstable-options --output-format json
```

`rustc-dev` is included in that toolchain so the implementation can continue moving toward a more direct compiler/HIR-backed pipeline.

## Pipeline entrypoints

The current API review caller chain under `eng/pipelines/` is:

1. `eng/pipelines/pr.yml` or `eng/pipelines/pullrequest.yml`
2. `eng/pipelines/templates/stages/archetype-sdk-client.yml`
3. `eng/pipelines/templates/jobs/ci.yml`
4. `eng/pipelines/templates/jobs/pack.yml`
5. `eng/scripts/Pack-Crates.ps1`

`pack.yml` also runs the shared `create-apireview` step. Today `Pack-Crates.ps1` still invokes
`eng/tools/generate_api_report` to produce the API review artifact consumed by that step.

## Current state

- The CLI is implemented and validates its APIView output shape.
- A shared intermediate model is used by both output formats.
- The current extraction path adapts rustdoc JSON into the shared model.
- The implementation is intentionally structured so extraction can later move closer to direct librustdoc/HIR usage without rewriting both renderers.
