<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->
<!-- cSpell:ignore syso -->

# Go native driver build pipeline

This folder builds the static Cosmos Rust native driver and the separate signed
Windows ARM64 MSVC DLL payload.

The production pipeline extends the repository's official 1ES wrapper and is
registered for manual internal runs rather than automatic CI. A successful
manual main-branch build can open a draft pull request in
`Azure/azure-cosmos-driver`.

Production jobs install the centrally pinned Microsoft Rust toolchain from
`eng/templates/ms-rust-toolchain.toml` through the internal `RustInstaller@1`
feed. The native build invokes Cargo and rustc with the explicit pinned
toolchain selector, compares its sysroot and version identities with the
compiler and Cargo binaries under the `RUST_BIN_PATH` reported by
`RustInstaller@1`, and rejects an upstream compiler, a different or unpinned
channel, and targets that `msrustup` cannot install. Each matrix job validates
its target against the centralized list and passes only that target to
`RustInstaller@1`; it does not attempt to install targets assigned to other
operating systems.

## Configured release matrix

The Microsoft Rust policy applies to five static targets and one DLL target.
Windows AMD64 (GNU) remains deferred.

| OS and architecture | Rust target | Observed `ms-prod-1.95` status |
| --- | --- | --- |
| Linux AMD64 (glibc) | `x86_64-unknown-linux-gnu` | Available: installation reached build validation |
| Linux ARM64 (glibc) | `aarch64-unknown-linux-gnu` | Available: installation reached build validation |
| Linux AMD64 (musl) | `x86_64-unknown-linux-musl` | Available: installation reached build validation |
| Linux ARM64 (musl) | `aarch64-unknown-linux-musl` | Available: installation reached build validation |
| macOS ARM64 | `aarch64-apple-darwin` | Available: installation reached build validation |
| Windows ARM64 DLL | `aarch64-pc-windows-msvc` | Build/sign pipeline configured; physical ARM64 validation required |

These observations come from internal pipeline runs with RustInstaller 1.0.92
and Microsoft Rust package `1.95.0-ms-20260618.5`. The five available targets
still require a fresh end-to-end run after the toolchain identity fixes in this
pull request.

### Deferred targets

| OS and architecture | Rust target | Reason deferred |
| --- | --- | --- |
| Windows AMD64 (GNU) | `x86_64-pc-windows-gnu` | Unavailable: the `ms-prod` feed does not publish `rust.std` for the GNU/MinGW Windows target, so `RustInstaller@1` fails before the build script runs |

Windows AMD64 GNU is the target the downstream Go cgo consumer requires (cgo
uses the GCC/MinGW toolchain on Windows and links the static `.a` produced here).
Because the `ms-prod` channel is MSVC-only today, this target is temporarily
removed from `build-matrix.json` and `eng/templates/ms-rust-toolchain.toml`
rather than left in place to fail closed on every release. The deferred target
object is preserved under `_windows_deferred_comment` in `build-matrix.json` so
it can be restored verbatim once a path is agreed. Restoring it requires one of:

1. Microsoft Rust publishing `rust.std` for `x86_64-pc-windows-gnu`.
2. An explicitly approved Windows-only upstream `rustup` exception.
3. Moving the Windows target to MSVC and absorbing the additional
   DLL/runtime-loading complexity in the downstream Go layer.

Windows ARM64 does not replace or establish support for Windows AMD64 GNU.
Intel macOS and dynamic libraries for other language bindings remain outside
the release matrix.

## Files

| File | Purpose |
| ---- | ------- |
| `build-matrix.json` | Lists supported Rust targets and their Go module paths. |
| `../../../../eng/templates/ms-rust-toolchain.toml` | Pins the Microsoft Rust release channel and required Rust targets. |
| `../../../../eng/pipelines/templates/steps/use-ms-rust.yml` | Provides the opt-in internal Microsoft Rust installer path without changing ordinary Rust jobs. |
| `New-NativeJobMatrix.ps1` | Converts the canonical target list into the standard Azure Pipelines matrix-generator format. |
| `Build-NativeMatrix.ps1` | Verifies Microsoft Rust, builds each static library, and writes schema 4 release metadata. |
| `Finalize-WindowsArm64Artifact.ps1` | Verifies the signed ARM64 PE, exports, imports, timestamp, and final hashes. |
| `Test-NativeLink.ps1` | Cross-links a minimal Go/cgo program against each target archive before publication. |
| `New-GoModules.ps1` | Creates the `Azure/azure-cosmos-driver` directory layout, Go module files, cgo linker files, headers, and static libraries. |
| `Test-GoModuleConsumer.ps1` | Builds direct and vendored Go consumers that call `cosmos_version()` from a generated host module. |
| `Prepare-GoDriverPullRequest.ps1` | Verifies the artifact, synchronizes the downstream generated roots, validates the Go modules, and stages the changes. |
| `tests/New-GoModules.Tests.ps1` | Verifies that Go module generation rejects mixed, mislabeled, or modified target artifacts. |
| `tests/Prepare-GoDriverPullRequest.Tests.ps1` | Verifies that downstream synchronization removes retired generated files without modifying hand-maintained files. |
| `tests/Test-GoModuleConsumer.Tests.ps1` | Verifies direct and vendored consumer command flow and symbol-call source generation. |
| `tests/Test-NativeLink.Tests.ps1` | Verifies target metadata checks and Go link-smoke command wiring. |
| `Invoke-LocalSupplyChain.ps1` | Runs a local end-to-end integration test without publishing anything. |
| `native-driver.yml` | Defines the official 1ES build, Go module artifact, and downstream draft pull request. |
| `native-driver-build-job.yml` | Runs one generated target row with the appropriate pool, image, Rust setup, and linker. |
| `../docs/NATIVE_SUPPLY_CHAIN.md` | Explains how the artifacts are built and verified. |

## Production flow

```text
Pinned azure-sdk-for-rust commit and Microsoft Rust channel
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
Build direct and vendored Linux AMD64 consumers
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

The Windows ARM64 row branches after build: MSVC produces
`azurecosmosdriver.dll` and `azurecosmosdriver.pdb`; ESRP signs and verifies the
DLL; finalization validates PE machine `0xAA64`, imports and exports, then hashes
the signed bytes; 1ES publishes the payload with signed SPDX evidence. It skips
Go/cgo linking, module generation, and downstream source staging.

The signing task requires the repository's approved Cosmos native signing
service connection, client/tenant, Key Vault, and certificate pipeline
variables. Missing onboarding fails the job rather than producing an unsigned
artifact. Signing diagnostics remain pipeline-only and are never copied by
`Prepare-GoDriverPullRequest.ps1`, which retains the exact six-file signed
evidence allowlist.

The Windows payload is staged under
`windows-arm64-msvc/v<native-interface-version>/` and contains
`azurecosmosdriver.dll`,
`azurecosmosdriver.pdb`, `azurecosmosdriver.h`,
`rust-driver-native-interface-metadata.json`, and `SHA256SUMS`. The PDB is
published for diagnostics but is not required to load the DLL. See
`../docs/NATIVE_SUPPLY_CHAIN.md` for the distribution/security tradeoffs and
the Go-team handoff.

The official 1ES template is the governed build and provenance boundary. The
pipeline uses the repository's standard `1ES.PublishPipelineArtifact@1` wrapper
with SBOM generation enabled rather than implementing a second, pipeline-local
signature verifier.

Each target artifact includes schema 4 metadata with the selected toolchain
manager, pinned Microsoft Rust channel, exact RustInstaller package, manager and
Cargo versions, invoked and installer compiler paths, selected sysroot,
complete `rustc -Vv` output and compiler commit, target triple, and linker
command, resolved path, and version output. `New-GoModules.ps1` rejects missing
or mixed toolchain identities before writing schema 2 `provenance.json`. The
link smoke test remains toolchain-neutral so third-party builds can validate
compatible artifacts; the Microsoft Rust policy applies only at the governed
production build and publication boundaries.

The native-driver pipeline is not part of the automatic pull-request pipeline.
Authorized reviewers can run its registered pipeline definition against a pull
request with an `/azp run` comment.

The publication stage runs only after a successful manual build of
`refs/heads/main`. It mints a short-lived Azure SDK Automation GitHub App token,
clones the downstream repository, verifies `SHA256SUMS`, excludes the 1ES
`_manifest` evidence directory from payload validation, then exports only the
required signed evidence bundle: `manifest.spdx.json`,
`manifest.spdx.json.sha256`, `manifest.spdx.cose`, `manifest.cat`, `bsi.json`,
and `bsi.cose`. The downloaded pipeline artifact may contain verbose 1ES and
ESRP diagnostic logs, but the downstream staging script never copies them into
the checkout. The script rejects files outside the managed roots, runs Go
validation for each module definition and the Linux AMD64 module, and opens a
draft pull request. The target repository's branch rules require review and
code-owner approval before merge.

Each generated module has a flat native layout:

```text
<goos>/<goarch>[-musl]/
├── go.mod
├── link_<goos>_<goarch>.go
├── azurecosmosdriver.h
└── libazurecosmosdriver.a
```

The cgo linker directive uses `-L${SRCDIR} -lazurecosmosdriver`. Keeping the
archive at the module root ensures `go mod vendor` copies it into the vendored
package. The pipeline builds both a normal local-replace consumer and a
`-mod=vendor` consumer that calls `cosmos_version()`, proving the real archive
is present and its symbol resolves in both layouts. The generator emits neither
a `.syso` copy nor a duplicate `native/` directory.

## Local integration test

Run the complete local test (defaults to the `linux-amd64-glibc` target):

```powershell
$installerPackageVersion = '<exact RUST-INSTALLER Actual value>'
$installerBinPath = '<RUST_BIN_PATH reported by RustInstaller@1>'
./Invoke-LocalSupplyChain.ps1 `
    -InstallerPackageVersion $installerPackageVersion `
    -InstallerBinPath $installerBinPath
```

The local machine must already have access to the internal Microsoft Rust feed,
and the pinned channel installed through `msrustup`. Build commands select that
channel explicitly; there is no fallback to an upstream Rust installation.

The script:

1. Builds the native libraries.
2. Applies a disposable self-signed certificate when the target emits a Windows
   DLL. Windows AMD64 is deferred, so this step is skipped for the active
   targets.
3. Generates and validates a local SPDX inventory.
4. Writes SHA256 checksums.
5. Generates the Go module and builds direct and vendored consumers.
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
├── azure-cosmos-driver-pr/{_manifest,linux,darwin}/
└── LOCAL_PR_PREVIEW.md
```

`pipeline/artifacts/` and `pipeline/generated/` are ignored by Git. Local test
results are not committed to the repository.

Individual steps can also be run separately:

```powershell
$installerPackageVersion = '<exact RUST-INSTALLER Actual value>'
$installerBinPath = '<RUST_BIN_PATH reported by RustInstaller@1>'

# Build one target.
./Build-NativeMatrix.ps1 `
    -TargetId linux-amd64-glibc `
    -CCompiler gcc `
    -InstallerPackageVersion $installerPackageVersion `
    -InstallerBinPath $installerBinPath

# Inspect metadata without producing native libraries.
./Build-NativeMatrix.ps1 `
    -InstallerPackageVersion $installerPackageVersion `
    -InstallerBinPath $installerBinPath `
    -SkipBuild

# Generate Go modules from previously built artifacts.
./New-GoModules.ps1
```

## Linux glibc and musl

The glibc and musl builds use separate Go module paths:

- glibc: `linux/amd64` and `linux/arm64`
- musl: `linux/amd64-musl` and `linux/arm64-musl`

Each module stores its library and header at the module root. The consuming Go
package selects the appropriate driver module, so users do not need a custom
musl build tag.

## Work still required before release

- Run the signed pipeline with the production ESRP onboarding and verify the
  timestamp chain on the published bytes.
- Exercise the DLL on physical Windows ARM64 hardware, including lifecycle,
  completion queue, transport-error, unload, Application Verifier, and
  antivirus/enterprise policy scenarios. Cross-building on AMD64 is not a
  production-readiness substitute.

- Confirm with the central security owners that the official 1ES template is the
  approved trust boundary for these static libraries.
- Run the registered pipeline manually in the internal project and confirm that
  the five available targets complete their build and link-smoke tests after the
  toolchain identity fixes.
- Decide the Windows AMD64 GNU path before re-adding that deferred target,
  either by publishing `x86_64-pc-windows-gnu` in the pinned Microsoft Rust
  channel or by ratifying an alternate toolchain. Upstream Rust fallback is not
  permitted. See the "Deferred targets" section for the restore options.
- Confirm that the Azure SDK Automation GitHub App installation includes the
  private `Azure/azure-cosmos-driver` repository and that this pipeline may use
  the `AzureSDKEngKeyVault Secrets` service connection.
