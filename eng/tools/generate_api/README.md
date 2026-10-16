# generate_api

`generate_api` is a CLI for generating public API artifacts for Rust crates in this repository.

## Usage

Run from the repository root:

```sh
cargo run --manifest-path eng/tools/Cargo.toml -p generate_api -- \
  --manifest-path sdk/core/azure_core/Cargo.toml
```

By default, output is written to the crate directory. The command above writes
`sdk/core/azure_core/api.md`.

To write Markdown API review artifacts to a different directory:

```sh
cargo run --manifest-path eng/tools/Cargo.toml -p generate_api -- \
  --manifest-path sdk/core/azure_core/Cargo.toml \
  --output-dir target/generate_api/azure_core \
  --review
```

To verify the default output without writing it:

```sh
cargo run --manifest-path eng/tools/Cargo.toml -p generate_api -- \
  --manifest-path sdk/core/azure_core/Cargo.toml \
  --check
```

To invoke the tool from outside the repository, pass the repository root explicitly:

```sh
cargo run --manifest-path /path/to/azure-sdk-for-rust/eng/tools/Cargo.toml -p generate_api -- \
  --root /path/to/azure-sdk-for-rust \
  --manifest-path /path/to/azure-sdk-for-rust/sdk/core/azure_core/Cargo.toml \
  --output /tmp/generate_api/azure_core
```

### Arguments

- `--manifest-path <path>`: path to the target crate's `Cargo.toml`
- `--root <path>`: optional repository root; defaults to the current directory
- `--format <markdown|apiview>`: optional output format to generate; defaults to `markdown`
- `--review`: emit Markdown review sidecars; valid only with `--format markdown`
- `--check`: compare generated content with existing output files without writing them
- `--output-dir <dir>`: directory where generated files are written; defaults to the crate directory

### Outputs

- default `markdown` output writes `api.md`
- `--format markdown --review` also writes `api.metadata.yml`, `api.md.map`, and
  `api.documentation.patch`
- `--format apiview` writes `apiview.json`
- `--check` succeeds when output files are absent or match after normalizing line endings; a mismatch
  writes an error to stderr and exits with code `1`

Both formats include available Cargo metadata (`description`, `edition`, and `rust-version`) and
the crate's default and docs.rs feature list (or all features when not configured). Markdown also
starts with the crate name. Multiline descriptions render with the `Description` label on its own
line before the description text. Children of the `default` feature are also listed. `api.md` never
contains
documentation comments.
`api.metadata.yml` is written next to `api.md` and records `apiHash` as the normalized SHA-256 of
`api.md`, the crate version, the repository-relative package path, the `generate_api` version, and
the rustc version used for generation.
`api.documentation.patch` is a unified diff that adds them back, so it can be applied to toggle
documentation comments on. Each comment hunk uses all attributes and the first declaration line as
context:

```sh
patch -p1 < api.documentation.patch
```

`api.md.map` is an ECMA-426 source map that maps declaration lines in the fenced Rust API block
back to the corresponding item or member declarations. Entries in `sources` are always relative
to the repository root. The generated `sourceRoot` is the relative path from the `--output`
directory to the repository root when the output directory is inside the repository. `sourceRoot`
is omitted when the output directory is outside the repository.

Source-map consumers differ in how they resolve `sourceRoot`. If the application opens the source
map with the repository root as its base, remove `sourceRoot`. If the application cannot resolve
the generated relative `sourceRoot`, replace it with the absolute path to the repository root.
Alternatively, keep the generated relative `sourceRoot` and the source map at the same relative
location within an unchanged repository directory structure; the source paths will then continue
to resolve relative to `api.md.map` or a custom map path. Keep the `sources` entries unchanged in
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

For local testing, `Pack-Crates.ps1 -APIReview` temporarily switches to Markdown review generation
and writes the review artifacts into each crate root directory. Pipelines do not set `-APIReview`
today.

### API Review Hub

Rust API Review Hub uses two job wrappers:

1. `eng/pipelines/templates/jobs/apireview-hub-build-tools-rust.yml` uses the normal
   `use-rust.yml` setup in `eng/tools` to install its specified toolchain, builds
   `generate_api` in release mode, and copies only the executable (`generate_api.exe` on
   Windows) into `$(BinOutputDirectory)` for the shared pipeline to publish.
2. `eng/pipelines/templates/jobs/apireview-hub-generate-artifact-rust.yml` uses the normal
   nightly Rust setup and existing Cargo helpers from the base or target checkout to
   locate the requested package, then invokes the prebuilt executable from `$(BinDirectory)`.
   No scripts, helpers, toolchain manifests, or build-time dependencies are staged.
3. The generation job passes `--root $(Build.SourcesDirectory)` because the executable is outside
   the package checkout, and writes the complete review bundle to `$(ArtifactOutputDirectory)`.

The executable embeds its toolchain channel at build time and runs Cargo itself to build
the package and generate rustdoc JSON. The base and target branches are expected to share
the normal nightly setup from `main`. Generation invokes `cargo rustdoc` with the embedded
channel and records the resulting rustc release in `api.metadata.yml`. Cargo build output
stays in the source workspace; only the review bundle goes to `$(ArtifactOutputDirectory)`.

## Toolchain

The tool reads `eng/tools/rust-toolchain.toml` and invokes:

```sh
cargo +nightly-2026-04-14 rustdoc -Z unstable-options --output-format json
```
