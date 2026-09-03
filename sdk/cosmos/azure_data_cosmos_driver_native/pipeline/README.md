<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->

# Go native driver build pipeline

This folder contains the scripts and Azure DevOps pipeline definition that build
the Cosmos Rust native driver for Go. The output is a static Rust library named
`libazurecosmosdriver.a`, together with the C header and release evidence needed
to review where the library came from.

The production pipeline extends the repository's official 1ES wrapper but is
not connected to a CI or release definition yet. After an owner registers it in
the internal Azure SDK project, a successful manual main-branch build can open a
draft pull request in `Azure/azure-cosmos-driver`.

## What this pull request supports

The active build targets are:

- Windows AMD64
- Linux AMD64 using glibc
- Linux ARM64 using glibc
- Linux AMD64 using musl
- Linux ARM64 using musl
- macOS ARM64

Windows ARM64 and Intel macOS are outside the supported matrix. Dynamic libraries
for .NET, Java, and Python are also outside this pull request.

The generated Windows cgo linker file statically links the MinGW pthread runtime.
This prevents the final Go application from requiring a separately distributed
`libwinpthread-1.dll`.

## Files

| File | Purpose |
| ---- | ------- |
| `build-matrix.json` | Lists supported Rust targets and their Go module paths. |
| `New-NativeJobMatrix.ps1` | Converts the canonical target list into the standard Azure Pipelines matrix-generator format. |
| `Build-NativeMatrix.ps1` | Builds each static library, records required system libraries, and writes release metadata. |
| `Test-NativeLink.ps1` | Cross-links a minimal Go/cgo program against each target archive before publication. |
| `New-GoModules.ps1` | Creates the `Azure/azure-cosmos-driver` directory layout, Go module files, cgo linker files, headers, and static libraries. |
| `Prepare-GoDriverPullRequest.ps1` | Verifies the artifact, synchronizes the downstream generated roots, validates the Go modules, and stages the changes. |
| `tests/New-GoModules.Tests.ps1` | Verifies that Go module generation rejects mixed, mislabeled, or modified target artifacts. |
| `tests/Prepare-GoDriverPullRequest.Tests.ps1` | Verifies that downstream synchronization removes retired generated files without modifying hand-maintained files. |
| `tests/Test-NativeLink.Tests.ps1` | Verifies target metadata checks and Go link-smoke command wiring. |
| `Invoke-LocalSupplyChain.ps1` | Runs a local end-to-end integration test without publishing anything. |
| `native-driver.yml` | Defines the official 1ES build, Go module artifact, and downstream draft pull request. |
| `native-driver-build-job.yml` | Runs one generated target row with the appropriate pool, image, Rust setup, and linker. |
| `../docs/NATIVE_SUPPLY_CHAIN.md` | Explains how the artifacts are built and verified. |

## Production flow

```text
Pinned azure-sdk-for-rust commit
    |
    v
Generate jobs from build-matrix.json using the shared matrix infrastructure
    |
    v
Build one static library for each supported target
    |
    v
Publish each target through the official 1ES template with its standard SBOM
    |
    v
Generate and test the Go modules
    |
    v
Publish the azure-cosmos-driver-modules pipeline artifact
    |
    v
Verify the downloaded artifact and generated paths
    |
    v
Open a draft pull request in Azure/azure-cosmos-driver
    |
    v
GitHub code-owner review and approval
```

The official 1ES template is the governed build and provenance boundary. The
pipeline uses the repository's standard `1ES.PublishPipelineArtifact@1` wrapper
with SBOM generation enabled rather than implementing a second, pipeline-local
signature verifier.

The native-driver pipeline is not part of the automatic pull-request pipeline.
Authorized reviewers can run its registered pipeline definition against a pull
request with an `/azp run` comment.

The publication stage runs only after a successful manual build of
`refs/heads/main`. It mints a short-lived Azure SDK Automation GitHub App token,
clones the downstream repository, verifies `SHA256SUMS`, excludes the 1ES
`_manifest` evidence directory from payload validation, copies the complete
manifest into the downstream repository, rejects files outside the managed
roots, runs Go validation for each module definition and the Linux AMD64 module,
and opens a draft pull request. The target repository's branch rules require
review and code-owner approval before merge.

## Local integration test

Run the complete local test on Windows AMD64:

```powershell
./Invoke-LocalSupplyChain.ps1
```

The script:

1. Builds the native libraries.
2. Applies a disposable self-signed certificate to the Windows DLL.
3. Generates and validates a local SPDX inventory.
4. Writes SHA256 checksums.
5. Generates and tests the Go module.
6. Clones `Azure/azure-cosmos-driver`.
7. Creates a local branch, commit, and pull-request preview.

It never pushes the branch or opens a remote pull request. Local signatures and
SPDX files demonstrate the mechanics only; they are not Microsoft release
evidence.

The generated files are placed under:

```text
pipeline/artifacts/local-rehearsal/<timestamp>/
├── native/<target-id>/{_manifest,signing,audit,validation}/
├── azure-cosmos-driver-output/
├── azure-cosmos-driver-pr/{_manifest,windows,linux,darwin}/
└── LOCAL_PR_PREVIEW.md
```

`pipeline/artifacts/` and `pipeline/generated/` are ignored by Git. Local test
results are not committed to the repository.

Individual steps can also be run separately:

```powershell
# Build one target.
./Build-NativeMatrix.ps1 `
    -TargetId windows-amd64 `
    -CCompiler gcc

# Inspect metadata without producing native libraries.
./Build-NativeMatrix.ps1 -SkipBuild

# Generate Go modules from previously built artifacts.
./New-GoModules.ps1
```

## Linux glibc and musl

The glibc and musl builds use separate Go module paths:

- glibc: `linux/amd64` and `linux/arm64`
- musl: `linux/amd64-musl` and `linux/arm64-musl`

Each module stores its library under `native/`. The consuming Go package selects
the appropriate driver module, so users do not need a custom musl build tag.

## Work still required before release

- Register `native-driver.yml` in the internal Azure SDK project so
  `1es-redirect.yml` selects the official 1ES template.
- Confirm with the central security owners that the official 1ES template is the
  approved trust boundary for these static libraries.
- Provision every cross-compiler named by the build matrix. In particular, the
  managed Ubuntu image does not include the ARM64 musl compiler required by
  `linux-arm64-musl`; this must be resolved before registering the pipeline.
- Confirm that the Azure SDK Automation GitHub App installation includes the
  private `Azure/azure-cosmos-driver` repository and that this pipeline may use
  the `AzureSDKEngKeyVault Secrets` service connection.
