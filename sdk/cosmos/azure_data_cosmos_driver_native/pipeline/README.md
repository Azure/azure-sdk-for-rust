<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->

# Native driver cross-build pipeline (M2)

This folder productizes the manual `windows/amd64` bootstrap
(`Azure/azure-cosmos-driver` PR #5) into a reproducible cross-build and
supply-chain-evidence pipeline for `libazurecosmosdriver.a`.

> **Status: SKELETON.** These scripts run locally for validation. The pipeline
> (`native-driver.yml`) is **unwired** — it is not referenced by any `pr.yml` /
> `ci.yml` and cannot run until an owner wires it in. No remote pushes, PRs, or
> pipeline runs without explicit sign-off.

## Files

| File | Deliverable | Purpose |
| ---- | ----------- | ------- |
| `build-matrix.json` | — | Single source of truth: the "Big 5 + musl" target rows, Go layout, and the musl build tag. Versions come from Cargo metadata. |
| `Build-NativeMatrix.ps1` | D1 | Per-triple: capture syslibs (`--print native-static-libs`), build native libraries with `cargo-auditable`, and emit `rust-driver-native-interface-metadata.json`. |
| `New-GoModules.ps1` | D2 | Emit the `azure-cosmos-driver/<goos>/<goarch>` Go modules (`go.mod` + `link_*.go` + `native/`), splicing captured syslibs into cgo `LDFLAGS`. |
| `Invoke-LocalSupplyChain.ps1` | rehearsal | Build, test-sign, generate and validate SBOM/provenance evidence, validate Go/cgo, and prepare a local-only `azure-cosmos-driver` PR commit. |
| `native-driver.yml` | D1+D2+D3 | 1ES-style pipeline skeleton: build matrix -> security-evidence gate -> generate Go modules -> publish artifacts + `SHA256SUMS`. **Unwired and fail-closed.** |
| `../docs/NATIVE_SUPPLY_CHAIN.md` | D3 | Signing, SBOM, provenance, and `cargo-auditable` design across the static-`.a` and dynamic-lib consumption paths. |

## Local usage

```powershell
# Full local Windows/amd64 rehearsal. This never pushes or opens a remote PR.
./Invoke-LocalSupplyChain.ps1

# Build only one target (cross targets need their matching linker).
./Build-NativeMatrix.ps1 -TargetId windows-amd64

# Dry run (capture syslibs + write metadata only, no native libraries).
./Build-NativeMatrix.ps1 -SkipBuild

# Generate the Go modules from already-built artifacts.
./New-GoModules.ps1
```

Outputs land under `pipeline/artifacts/<target-id>/` (built libs + manifests)
and `pipeline/generated/azure-cosmos-driver/` (Go module layout). Both are
git-ignored — see the crate `.gitignore`.

Each local rehearsal writes to an isolated timestamped directory:

```text
pipeline/artifacts/local-rehearsal/<timestamp>/
├── native/<target-id>/{sbom,_manifest,signing,audit}/
├── azure-cosmos-driver-output/  # generated module tree
├── azure-cosmos-driver-pr/      # local clone, branch, and commit
└── LOCAL_PR_PREVIEW.md          # exact local PR summary
```

All locally generated evidence is marked non-production. The SPDX SBOM proves
the Microsoft tool invocation and file inventory; the CycloneDX SBOM proves the
Cargo dependency view. Metadata records both `HEAD` and a SHA256 snapshot of all
tracked and non-ignored source files so a dirty rehearsal is not attributed only
to a clean commit. Windows signing uses a disposable self-signed certificate.
Trusted releases use existing signed 1ES BSI build evidence. Dynamic-library
signing is deferred to the separate non-Go release path.

`native-driver.yml` deliberately throws at its security-evidence stage until
the Cargo-aware CycloneDX generation and signed 1ES BSI evidence are verified.
This prevents the skeleton from publishing an artifact based only on local or
placeholder trust evidence.

The orchestrated local rehearsal currently supports `windows-amd64`. The
`windows-arm64` row fails closed until a validated GNU toolchain or MSVC
fallback is ratified; the unwired pipeline preserves that matrix row but marks
it disabled by default. Linux module publication must provide both glibc and
musl rows; selecting either row in `New-GoModules.ps1` automatically selects
and requires its sibling.

## The musl / glibc decision (read before changing the layout)

`linux/amd64` and `linux/arm64` each map to **two** matrix rows (glibc + musl)
but a single Go module path. They are disambiguated by a consumer-set build tag:

- **glibc (default):** `go build`
- **musl (Alpine/AKS/Dapr):** `go build -tags cosmos_musl`

The link files and `native/{glibc,musl}/` subdirs are generated accordingly.
Each module also contains `azurecosmosdriver.h` at its root for cgo
preprocessing, while the release copy remains beside the archive under
`native/` or `native/<libc>/`.
Go validation passes `-tags cosmos_musl` for a selected musl row.
Forgetting the tag on a musl host is a **loud link error**, not silent breakage.
The alternative (distinct `linux_musl/amd64` module path) and the rationale are
in `../docs/NATIVE_SUPPLY_CHAIN.md`. Don't change the scheme here without
updating that doc and the `azure-sdk-for-go` consumer.

## Open risks (ratify, don't silently decide)

- **windows/arm64 gnu toolchain** immaturity — investigate an msvc-built `.a`
  fallback.
- **musl/glibc tag footgun** — the loud-error trade-off above.
- **static-`.a` signing gap** — Authenticode/codesign apply to the dynamic lib,
  not the `.a`; static-path trust is SBOM + provenance + published SHA256.
- **driver is a path dependency** (`azure_data_cosmos_driver` 0.7.0), not a
  published/workspace crate — provenance pins the source commit, not crates.io.
