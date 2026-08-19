<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->
<!-- cSpell:ignore Authenticode codesign dylib staticlib rustls mingw musl SPDX -->

# How the Go native driver is built and verified

This document explains the release design for the static Cosmos native driver
consumed by Go. It describes what is built, why the static library cannot be
signed directly, and which release files establish trust.

The pipeline definition is `pipeline/native-driver.yml`. The operational
commands are documented in `pipeline/README.md`.

## Scope

This pull request produces `libazurecosmosdriver.a` for:

- Windows AMD64
- Linux AMD64 and ARM64 using glibc
- Linux AMD64 and ARM64 using musl
- macOS ARM64

The Go SDK links this static library into the customer's final executable.

This pull request does not distribute DLLs, macOS dynamic libraries, or Linux
shared objects. Dynamic-library distribution and signing belong to a future
release path.

## What is published

Each target build produces an intermediate Azure DevOps pipeline artifact:

```text
<target-id>/
├── libazurecosmosdriver.a
├── <platform-dynamic-library>
├── azurecosmosdriver.h
└── rust-driver-native-interface-metadata.json
```

The dynamic library is retained only as intermediate pipeline output for
inspection. It is not copied into the combined Go-module artifact or the
downstream Go pull request. Only `libazurecosmosdriver.a` is delivered through
the Go release path. Dynamic libraries can be delivered by a future release
pipeline designed for their signing and packaging requirements.

The metadata records:

- the Rust target;
- the source repository commit;
- the native-interface and driver versions;
- the Rust and Cargo tool versions;
- the operating-system libraries required by the Go linker; and
- the SHA256 checksums of the built libraries and C header.

Before generating output, `New-GoModules.ps1` verifies that every selected
artifact matches its matrix identity and recorded file hashes, that all targets
come from the same source commit and package versions, and that every target
contains the same C header. After those checks pass, it creates the directory
layout expected by `Azure/azure-cosmos-driver`:

```text
azure-cosmos-driver/
├── _manifest/
│   └── spdx_2.2/
├── windows/amd64/
├── linux/amd64/
├── linux/arm64/
├── linux/amd64-musl/
├── linux/arm64-musl/
├── darwin/arm64/
├── provenance.json
└── SHA256SUMS
```

The pipeline-owned `_manifest` root contains the complete evidence directory
produced while publishing the combined artifact. Each module contains a
`go.mod`, generated cgo linker files, the C header, and the matching static
library. The root also carries a consolidated `provenance.json` binding the
release identity (see [provenance.json](#provenancejson)). The Windows linker
file also statically links the MinGW pthread runtime so the final Go application
does not require a separate `libwinpthread-1.dll`.

## Why the static library is not code-signed

Windows Authenticode and Apple code signing apply to executable files and shared
libraries. A static `.a` file is an archive of object files used during linking;
it is not loaded or executed by itself.

Signing a DLL or macOS dynamic library built from the same Rust crate would not
authenticate the `.a`. They are different files with different bytes.

The Go release therefore uses this chain:

1. The official 1ES template builds the `.a` from a recorded repository commit.
2. The target job links a minimal Go/cgo program against the `.a`.
3. 1ES publishes its standard SBOM and governed build provenance.
4. The build writes a SHA256 checksum for the exact `.a` bytes.
5. The downstream preparation step verifies the artifact metadata and checksums.
6. The Go customer signs the final executable that contains the Rust library.

No unsigned Microsoft shared library is loaded at runtime in this model. The
`.a` becomes part of the customer's Go executable.

## Release evidence

### Signed SPDX inventory

The shared `publish-1es-artifact.yml` template invokes
`1ES.PublishPipelineArtifact@1`. For internal builds, that task generates an
SPDX software inventory containing the published files and their hashes. The
1ES release output also includes a signed form of that inventory.

The SPDX inventory answers:

- Which files were published?
- What was the SHA256 value of each file?
- Which software components were included in the release?
- Has the inventory changed since 1ES produced it?

The shared template disables this automatic generation for public and
pull-request validation builds. Downstream publication is therefore restricted
to successful non-pull-request builds of `main` in the internal project.

### Official 1ES build provenance

The pipeline extends the repository's `1es-redirect.yml` entry point. In the
internal Azure SDK project, that entry point selects the official 1ES pipeline
template and connects the release to the governed pipeline run, repository, and
source commit.

The official template is the trust boundary for this release. The native-driver
pipeline does not duplicate the platform's evidence validation with a custom
signature-verification step.

### SHA256SUMS

The trusted build writes `SHA256SUMS` from the final static libraries. A
downstream consumer recomputes each checksum and rejects any mismatch.

A checksum detects changed bytes. Its trust comes from being produced and
published by the same governed official 1ES build.

### provenance.json

`New-GoModules.ps1` writes a single consolidated `provenance.json` at the
generated artifact root. It is the human-readable release-identity manifest that
binds the published static libraries back to their exact source:

- `source_commit` — the repository commit every target was built from;
- `rust_driver_crate` / `rust_driver_version` — the path-pinned driver crate and
  its version (the driver is not published to crates.io, so the commit is the
  authoritative pin);
- `native_interface_crate` / `native_interface_version` — the wrapper crate and
  the `AZURECOSMOSDRIVER_H_VERSION` header contract; and
- `targets[]` — one entry per built row with its `id`, `triple`, `module_path`,
  and the SHA256 of the static library and C header.

`New-GoModules.ps1` cross-validates that every selected target agrees on the
identity fields before emitting the file, so a mismatched or tampered target
fails the build rather than producing an inconsistent manifest. Because
`provenance.json` sits inside the same SBOM-enabled scan root as the libraries,
1ES includes it in the signed SPDX inventory — no separate signing step is
required. The downstream `Prepare-GoDriverPullRequest.ps1` step treats it as a
managed root file and republishes it alongside `SHA256SUMS`.

### Embedded dependency information

`Build-NativeMatrix.ps1` uses `cargo-auditable` so supported compiled binaries
contain their Rust dependency information. The accompanying
`rust-audit-info` tool can read that information directly from dynamic
libraries.

`rust-audit-info` does not read the static `.a` reliably because an archive is
not an executable image. For the Go path, the signed SPDX inventory and
`SHA256SUMS` are the release checks for the `.a`. Dependency information can
also be inspected after the library has been linked into the final Go
executable.

## Build settings

The release build uses the size settings measured in
[#4748](https://github.com/Azure/azure-sdk-for-rust/issues/4748):

```text
opt-level = "z"
lto = "fat"
codegen-units = 1
strip = "symbols"
```

These settings reduced the shipped native footprint by approximately 48-54%
without an observed runtime regression.

The build does not use `panic = "abort"`. The native interface catches Rust
panics at the language boundary, which requires normal panic unwinding.

## Production pipeline

The intended production sequence is:

```text
Recorded source commit
    |
    v
Build and link-smoke each target, then publish through 1ES
    |
    v
Generate the Go modules
    |
    v
Publish the combined Go-module pipeline artifact and SHA256SUMS
    |
    v
Verify the downloaded artifact, checksums, and changed paths
    |
    v
Open a draft pull request in Azure/azure-cosmos-driver
    |
    v
Receive GitHub code-owner review and approval
```

The checked-in pipeline extends the official 1ES wrapper and uses the standard
managed pool definitions for Linux, Windows, and Apple Silicon macOS. It remains
unregistered, so an owner must create its internal Azure DevOps definition
before it can run.

The publication stage runs only for a successful non-pull-request build of
`refs/heads/main`. It uses the existing Azure SDK Automation GitHub App to clone
the downstream repository and open a draft pull request.
`Prepare-GoDriverPullRequest.ps1` verifies every checksum, requires the standard
SPDX manifest, rejects unexpected payload paths, and replaces the pipeline-owned
`_manifest`, `windows`, `linux`, and `darwin` roots with the exact generated
artifact. It validates the resulting paths and hashes and rejects changes
elsewhere in the repository. This stages retired generated files as deletions
while preserving hand-maintained repository files. The target repository then
requires one approval and code-owner approval before merge.

## Local integration test

`Invoke-LocalSupplyChain.ps1` exercises the mechanics on a developer machine. It:

1. builds the native libraries;
2. applies a disposable test signature to the Windows DLL;
3. generates and validates a local SPDX inventory;
4. writes SHA256 checksums;
5. generates and tests the Go module; and
6. creates a local branch, commit, and pull-request preview.

The script never pushes the branch or opens a remote pull request. Its generated
files are marked non-production and stored in Git-ignored directories. The local
SPDX file and test certificate do not represent Microsoft release trust.

The local test exists to catch layout, linking, and script errors before an
internal pipeline run. It does not replace the governed official 1ES build.

## Linux library selection

Linux glibc and musl use distinct Go module paths. The unmarked
`linux/<arch>` modules contain glibc, while `linux/<arch>-musl` contains musl.
Each module stores one archive under `native/`. The consuming Go package imports
the correct driver module, avoiding a custom build tag and the risk of selecting
the wrong libc archive.
