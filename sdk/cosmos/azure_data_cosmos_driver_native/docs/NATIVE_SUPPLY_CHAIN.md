<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->
<!-- cSpell:ignore Authenticode codesign dylib staticlib rustls mingw musl SPDX BSI COSE -->

# How the Go native driver is built and verified

This document explains the complete release design for the static Cosmos native
driver consumed by Go. It describes what is built, why the static library cannot
be signed directly, which release files establish trust, and what remains to be
implemented before the pipeline can run in production.

The pipeline definition is
[`pipeline/native-driver.yml`](../pipeline/native-driver.yml). The operational
commands are documented in [`pipeline/README.md`](../pipeline/README.md).

## Scope

This pull request produces `libazurecosmosdriver.a` for:

- Windows AMD64
- Linux AMD64 and ARM64 using glibc
- Linux AMD64 and ARM64 using musl
- macOS ARM64

The Go SDK links this static library into the customer's final executable.

This pull request does not distribute DLLs, macOS dynamic libraries, or Linux
shared objects. Dynamic-library distribution and signing are tracked by
[#5048](https://github.com/Azure/azure-sdk-for-rust/issues/5048). Windows ARM64
and MSVC support are tracked by
[#5047](https://github.com/Azure/azure-sdk-for-rust/issues/5047).

## What is published

Each supported target produces:

```text
<target-id>/
├── libazurecosmosdriver.a
├── azurecosmosdriver.h
└── rust-driver-native-interface-metadata.json
```

The metadata records:

- the Rust target;
- the source repository commit;
- the native-interface and driver versions;
- the Rust and Cargo tool versions;
- the operating-system libraries required by the Go linker; and
- the SHA256 checksum of the static library.

After all targets pass their release checks, `New-GoModules.ps1` creates the
directory layout expected by `Azure/azure-cosmos-driver`:

```text
azure-cosmos-driver/
├── windows/amd64/
├── linux/amd64/
│   └── native/{glibc,musl}/
├── linux/arm64/
│   └── native/{glibc,musl}/
├── darwin/arm64/
└── SHA256SUMS
```

Each module contains a `go.mod`, generated cgo linker files, the C header, and
the matching static library.

## Why the static library is not code-signed

Windows Authenticode and Apple code signing apply to executable files and shared
libraries. A static `.a` file is an archive of object files used during linking;
it is not loaded or executed by itself.

Signing a DLL or macOS dynamic library built from the same Rust crate would not
authenticate the `.a`. They are different files with different bytes.

The Go release therefore uses this chain:

1. 1ES builds the `.a` from a recorded repository commit.
2. 1ES publishes a signed SPDX inventory and a signed build record.
3. The build writes a SHA256 checksum for the exact `.a` bytes.
4. The downstream Go release verifies those files before accepting the library.
5. The Go customer signs the final executable that contains the Rust library.

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
pull-request validation builds. The final paths and signatures must therefore be
confirmed in an internal non-pull-request pipeline run.

### Signed 1ES build record

1ES also emits a signed build record, commonly represented by `bsi.json` and
`bsi.cose`. It connects the release to the governed pipeline run, repository,
and source commit.

In plain terms, the build record answers, “Did the approved Microsoft build
produce this release from the expected source?”

### SHA256SUMS

The trusted build writes `SHA256SUMS` from the final static libraries. A
downstream consumer recomputes each checksum and rejects any mismatch.

A checksum detects changed bytes. Its trust comes from being produced and
published by the same governed build whose signed record is verified.

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
Build and publish each target through 1ES
    |
    v
Locate and verify the signed SPDX inventory and signed build record
    |
    v
Generate and test all Go modules
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

The checked-in pipeline currently stops at the evidence check. That stage throws
an error deliberately because the exact internal 1ES evidence paths and
verification commands have not yet been confirmed. Consequently, the Go module
and draft pull-request stages cannot run.

The pipeline does publish per-target build artifacts before reaching that check.
Those artifacts are not releasable until the evidence check succeeds.

After the evidence check succeeds, the publication stage runs only for a
successful non-pull-request build of `refs/heads/main`. It uses the existing Azure
SDK Automation GitHub App to clone the downstream repository and open a draft
pull request. `Prepare-GoDriverPullRequest.ps1` verifies every checksum, rejects
unexpected generated paths, replaces only module paths declared in
`build-matrix.json`, validates the Go files, and rejects changes elsewhere in the
repository. The target repository then requires one approval and code-owner
approval before merge.

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
internal pipeline run. It does not replace 1ES evidence verification.

## Linux library selection

Linux glibc and musl use the same Go module path. The generated Go files select
the correct directory:

- glibc is selected by default;
- musl is selected with the `cosmos_musl` build tag.

A musl consumer runs:

```text
go build -tags cosmos_musl
```

Omitting the tag on a musl system produces a linker or runtime failure rather
than silently selecting the intended musl library.

## Remaining release work

The following work must be completed before this pipeline can release Go
artifacts:

1. Connect each target to an approved 1ES managed pool.
2. Run an internal non-pull-request build and record the actual signed SPDX and
   build-record paths.
3. Replace the deliberate evidence-check error with commands that verify both
   signed files and fail on any mismatch.
4. Confirm that `SHA256SUMS` contains every published `.a`.
5. Confirm that the Azure SDK Automation GitHub App installation includes the
   private `Azure/azure-cosmos-driver` repository.
6. Authorize this pipeline to use the `AzureSDKEngKeyVault Secrets` service
   connection that mints the short-lived GitHub App token.

The existing shared
`eng/common/pipelines/templates/steps/create-pull-request.yml` template provides
the branch push and pull-request creation mechanism. `native-driver.yml` reuses
it without storing a personal access token.

## Deferred work

- Dynamic libraries and .NET/NuGet distribution:
  [#5048](https://github.com/Azure/azure-sdk-for-rust/issues/5048)
- Windows ARM64 and MSVC:
  [#5047](https://github.com/Azure/azure-sdk-for-rust/issues/5047)
- Artifact-digest-bound build evidence:
  [#5050](https://github.com/Azure/azure-sdk-for-rust/issues/5050)
- Intel macOS: enable only when customer demand justifies the additional target.
