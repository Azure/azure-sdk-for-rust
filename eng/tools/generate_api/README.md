# generate_api

`generate_api` is a CLI for generating public API artifacts for Rust crates in this repository.

## Usage

Run from the repository root:

```sh
cargo run --manifest-path eng/tools/Cargo.toml -p generate_api -- \
  --manifest-path sdk/core/azure_core/Cargo.toml \
  --format review \
  --output /tmp/generate_api
```

### Arguments

- `--manifest-path <path>`: path to the target crate's `Cargo.toml`
- `--format <review|apiview>`: output format to generate
- `--output <dir>`: directory where generated files are written

### Outputs

- `--format review` writes `API.md`
- `--format apiview` writes `apiview.json`

## Toolchain

The tool reads `eng/tools/rust-toolchain.toml` and invokes:

```sh
cargo +nightly-2025-05-09 rustdoc -Z unstable-options --output-format json
```

`rustc-dev` is included in that toolchain so the implementation can continue moving toward a more direct compiler/HIR-backed pipeline.

## Current state

- The CLI is implemented and validates its APIView output shape.
- A shared intermediate model is used by both output formats.
- The current extraction path adapts rustdoc JSON into the shared model.
- The implementation is intentionally structured so extraction can later move closer to direct librustdoc/HIR usage without rewriting both renderers.
