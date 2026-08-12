<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->

# Go native driver build pipeline

This folder contains the scripts and Azure DevOps pipeline definition that build
the Cosmos Rust native driver for Go. The output is a static Rust library named
`libazurecosmosdriver.a`, together with the C header and release evidence needed
to review where the library came from.

The production pipeline is not connected to a CI or release definition yet. Its
security check stops the pipeline deliberately until the signed evidence
produced by 1ES can be located and verified. After that check is implemented, a
successful main-branch build can open a draft pull request in
`Azure/azure-cosmos-driver`.

## What this pull request supports

The active build targets are:

- Windows AMD64
- Linux AMD64 using glibc
- Linux ARM64 using glibc
- Linux AMD64 using musl
- Linux ARM64 using musl
- macOS ARM64

Windows ARM64 and Intel macOS remain disabled. Dynamic libraries for .NET, Java,
and Python are also outside this pull request. Those follow-up paths are tracked
by [#5047](https://github.com/Azure/azure-sdk-for-rust/issues/5047) and
[#5048](https://github.com/Azure/azure-sdk-for-rust/issues/5048).

The generated Windows cgo linker file statically links the MinGW pthread runtime.
This prevents the final Go application from requiring a separately distributed
`libwinpthread-1.dll`.

## Files

| File | Purpose |
| ---- | ------- |
| `build-matrix.json` | Lists supported Rust targets and their Go module paths. |
| `Build-NativeMatrix.ps1` | Builds each static library, records required system libraries, and writes release metadata. |
| `Test-NativeLink.ps1` | Cross-links a minimal Go/cgo program against each target archive before publication. |
| `New-GoModules.ps1` | Creates the `Azure/azure-cosmos-driver` directory layout, Go module files, cgo linker files, headers, and static libraries. |
| `Prepare-GoDriverPullRequest.ps1` | Verifies the artifact, synchronizes the downstream generated roots, validates the Go modules, and stages the changes. |
| `tests/New-GoModules.Tests.ps1` | Verifies that Go module generation rejects mixed, mislabeled, or modified target artifacts. |
| `tests/Prepare-GoDriverPullRequest.Tests.ps1` | Verifies that downstream synchronization removes retired generated files without modifying hand-maintained files. |
| `tests/Test-NativeLink.Tests.ps1` | Verifies target metadata checks and Go link-smoke command wiring. |
| `Invoke-LocalSupplyChain.ps1` | Runs a local end-to-end integration test without publishing anything. |
| `native-driver.yml` | Defines the production build, evidence check, Go module artifact, and downstream draft pull request. |
| `../docs/NATIVE_SUPPLY_CHAIN.md` | Explains how the artifacts are built and verified. |

## Production flow

```text
Pinned azure-sdk-for-rust commit
    |
    v
Build one static library for each supported target
    |
    v
Publish each target through 1ES
    |
    v
Verify the signed SPDX inventory and signed 1ES build record
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

The evidence check currently throws an error on purpose. It must remain
fail-closed until an internal non-pull-request build confirms the exact paths and
verification commands for the signed SPDX inventory and 1ES build record.

The publication stage runs only after a successful non-pull-request build of
`refs/heads/main`. It mints a short-lived Azure SDK Automation GitHub App token,
clones the downstream repository, verifies `SHA256SUMS`, rejects changes outside
the generated module paths, runs Go validation for each module definition and
the Linux AMD64 module, and opens a draft pull request. The target repository's
branch rules require review and code-owner approval before merge.

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
├── azure-cosmos-driver-pr/
└── LOCAL_PR_PREVIEW.md
```

`pipeline/artifacts/` and `pipeline/generated/` are ignored by Git. Local test
results are not committed to the repository.

Individual steps can also be run separately:

```powershell
# Build one target.
./Build-NativeMatrix.ps1 `
    -TargetId windows-amd64 `
    -CCompiler x86_64-w64-mingw32-gcc

# Inspect metadata without producing native libraries.
./Build-NativeMatrix.ps1 -SkipBuild

# Generate Go modules from previously built artifacts.
./New-GoModules.ps1
```

## Linux glibc and musl

The glibc and musl builds share the same Go module path. The generated cgo files
select the correct static library:

- glibc is the default: `go build`
- musl requires: `go build -tags cosmos_musl`

The module stores the libraries under `native/glibc/` and `native/musl/`.
Selecting either Linux row in `New-GoModules.ps1` also selects its matching libc
row so an incomplete module is not generated.

## Work still required before release

- Connect `native-driver.yml` to approved 1ES pools.
- Confirm the signed SPDX and build-record layout from an internal release run.
- Replace the deliberate evidence-check failure with real verification commands.
- Confirm that the Azure SDK Automation GitHub App installation includes the
  private `Azure/azure-cosmos-driver` repository and that this pipeline may use
  the `AzureSDKEngKeyVault Secrets` service connection.
- Decide whether Intel macOS has enough demand to enable it.
