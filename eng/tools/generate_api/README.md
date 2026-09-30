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

To keep `api.*` files in one directory but write review state under a different working directory:

```sh
cargo run --manifest-path eng/tools/Cargo.toml -p generate_api -- \
  --manifest-path sdk/core/azure_core/Cargo.toml \
  --output-dir target/generate_api/azure_core \
  --working-dir target/generate_api/work/azure_core \
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
- `--working-dir <dir>`: optional directory for Markdown review state; defaults to the resolved
  `--output-dir` and is valid only with `--format markdown --review`

### Outputs

- default `markdown` output writes `api.md`
- `--format markdown --review` also writes `api.metadata.yml`, `api.md.map`, and
  `api.documentation.patch`
- `--format markdown --review` writes `state/version.txt` and
  `state/package-relative-path.txt` under the working directory
- `--format apiview` writes `apiview.json`
- `--check` succeeds when output files are absent or match after normalizing line endings; a mismatch
  writes an error to stderr and exits with code `1`

Both formats include available Cargo metadata (`description`, `edition`, and `rust-version`) and
the crate's default and docs.rs feature list (or all features when not configured). Markdown also
starts with the crate name. Multiline descriptions render with the `Description` label on its own
line before the description text. Children of the `default` feature are also listed. `api.md` never
contains
documentation comments.
`api.metadata.yml` is written next to `api.md` and records the normalized SHA-256 of `api.md`, the
crate version, the `generate_api` version, and the rustc version used for generation.
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

For Markdown review output, the `state` directory under the resolved working directory contains the
crate version in `version.txt` and the repository-relative crate directory in
`package-relative-path.txt`. If `--working-dir` is omitted, it defaults to `--output-dir`; if
`--output-dir` is also omitted, both directories default to the crate directory.

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

Rust API Review Hub uses repo-local PowerShell scripts rather than invoking `generate_api` inline
from YAML:

1. `eng/pipelines/templates/jobs/apireview-hub-job-rust.yml` resolves the requested Rust version
  through `eng/scripts/Resolve-ApiReviewHubRustToolchain.ps1`, which maps aliases and metadata
  values such as `1.97.0-nightly` back to the repository-managed channels from
  `eng/scripts/Language-Settings.ps1`.
2. The same job calls `eng/scripts/Build-ApiReviewHubTool.ps1`, which builds
  `eng/tools/generate_api` from the checked-out source repository, copies the built executable
  into the API Review Hub tooling directory, and publishes that staged executable path for later
  steps.
3. `eng/pipelines/templates/steps/create-apireview-hub-artifacts-rust.yml` then calls
  `eng/scripts/Export-API.ps1` with `-WorkspacePath`, `-OutputDir`, `-WorkingDir`, `-Review`, and
  the staged `-ToolPath` for each requested package/ref bundle. Review state is written under
  `<working>/state`.
4. In that mode, `Export-API.ps1` runs the staged `generate_api` executable directly with
  `--root <ApiReviewSourceDir>`. If the executable is unavailable, it falls back to
  `cargo +<resolved-toolchain> run --manifest-path <ApiReviewSourceDir>/eng/tools/Cargo.toml -p generate_api ...`.

The ARH scripts no longer copy a tooling workspace. They build once from the checked-out source
repo, then invoke the staged binary with `--root` so `generate_api` still reads and writes through
that repo's `target/doc` layout. The executable itself still uses the toolchain pinned by
`eng/tools/rust-toolchain.toml` when it invokes `cargo rustdoc` and records the resulting rustc
release in `api.metadata.yml`.

## Toolchain

The tool reads `eng/tools/rust-toolchain.toml` and invokes:

```sh
cargo +nightly-2026-04-14 rustdoc -Z unstable-options --output-format json
```
