<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->
<!-- cSpell:ignore Authenticode codesign notarize notarization cyclonedx auditable staple stapled dylib cdylib staticlib rustls mingw musl syft osxcross esrp SPDX BSI COSE PURL interop NuGet -->

# Native driver supply chain: signing, SBOM, and provenance (M2)

This document is the security/supply-chain design for the native Cosmos driver
artifacts produced by [`pipeline/native-driver.yml`](../pipeline/native-driver.yml).
It covers **each** ask: per-OS code signing, the static-`.a` signing gap, SBOM
generation, provenance, and `cargo-auditable` — and maps them onto the two
distinct consumption paths.

> Companion docs: [`pipeline/README.md`](../pipeline/README.md) (how the build
> runs) and [`docs/NATIVE_WRAPPER_SPEC.md`](../../azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md)
> in the driver crate (the ABI contract).
>
> **M2 scope.** This PR ships the **Go / static-`.a`** path for windows-amd64,
> linux (glibc + musl, amd64/arm64), and darwin-arm64. The dynamic-library
> distribution path (`.dll`/`.dylib`/`.so` for .NET / Java / Python) and MSVC /
> windows-arm64 support are intentionally **deferred**: the pipeline keeps the
> dynamic build as an extensibility hook, but neither is wired or supported here.
> They are tracked by
> [Azure/azure-sdk-for-rust#5048](https://github.com/Azure/azure-sdk-for-rust/issues/5048)
> (.NET / NuGet DLL path) and
> [Azure/azure-sdk-for-rust#5047](https://github.com/Azure/azure-sdk-for-rust/issues/5047)
> (MSVC / windows-arm64).

## 0. Artifacts and the two consumption paths

The wrapper crate `azure_data_cosmos_driver_native` (`crate-type = ["cdylib",
"staticlib"]`) produces, per target:

| Artifact | Kind | Who links it | How |
| -------- | ---- | ------------ | --- |
| `libazurecosmosdriver.a` | **static** | Cosmos **Go** v2 (cgo) | Go statically links the `.a` into the consumer's own binary. |
| `azurecosmosdriver.dll` / `.dylib` / `.so` | **dynamic** | .NET / Java / Python | Loaded at runtime as a shared library. |

These paths have **different trust stories**, which is the crux of this design:

- **Static `.a` (Go):** there is *no* Microsoft-signed binary shipped to the
  runtime. The `.a` is an intermediate input that gets absorbed into the
  consumer's Go executable. Authenticode/codesign do **not** apply to a `.a`.
  → Trust rests on **SBOM + provenance + published SHA256**, and the *final*
  signed artifact is the **consumer's** Go binary (their signing responsibility).
- **Dynamic lib (.NET/Java/Python):** a real shared library is shipped and loaded
  at runtime, so it **can and should** be OS-code-signed by us.

Everything below is written against this split.

### 0.1 Source distribution — build from a pinned commit (settled)

The native interface is **built from a pinned source commit inside 1ES and
shipped as binaries**; it is *not* published as a crate to crates.io.

Downstream SDKs consume the **compiled binaries + the C ABI**, not the Rust API,
so publishing the wrapper as a crate would add an extra public package,
dependency-publishing constraints, and a semver/support commitment with no
consumer benefit. Building straight from an immutable commit also gives cleaner
provenance (§3) than a registry release would. `Cargo.toml` therefore sets
`publish = false`, and `azure_data_cosmos_driver` stays a path dependency in
this workspace.

## 1. Signing boundary

### 1.1 Go static libraries — no direct OS signature

Go **static-links the `.a`**, so:

- We **cannot** Authenticode/codesign the `.a` that Go actually consumes.
- Signing the `.dll`/`.dylib` does **not** transitively vouch for the `.a`, even
  though both are built from the same crate — they are separate output files.

**Reconciliation with Option C (static-`.a` distribution):** for the Go path,
integrity is established *without* a signature on the artifact itself:

1. The `.a` is produced only inside the **trusted build** (1ES) from a pinned
   source commit.
2. Its **SHA256** is computed inside that trusted build and published in
   `SHA256SUMS` alongside a build **provenance attestation** (§3).
3. The `.a` embeds a **`cargo-auditable`** dependency manifest (§4) and is
   accompanied by an **SBOM** (§2).
4. The consumer verifies the downloaded `.a` against the published SHA256, then
   **signs their own Go binary** — that binary is the artifact that reaches an
   end user, and it is the consumer's signature that vouches for the whole thing.

So the chain is: *our trusted build + SHA256 + SBOM + provenance* → *consumer's
verification* → *consumer's signed Go executable*. There is no point at which an
unsigned Microsoft binary is loaded at runtime; the `.a` never runs on its own.

### 1.2 Dynamic-library signing is deferred

The future .NET/Java/Python path will require a separate release design:
Authenticode for Windows DLLs, codesign/notarization for macOS dylibs, and
provenance plus checksums for Linux shared objects. That work is tracked by
Azure/azure-sdk-for-rust#5048 and is intentionally not represented by a
placeholder signing stage in this Go-only pipeline.

## 1.3 Production size profile

The native build uses the production profile measured in
Azure/azure-sdk-for-rust#4748:

```text
opt-level = "z"
lto = "fat"
codegen-units = 1
strip = "symbols"
```

The measured profile reduced the effective shipped native footprint by roughly
48-54% without an observed runtime regression. `panic = "abort"` is deliberately
not used because the FFI boundary relies on panic unwinding for correctness.

## 2. SBOM

**Two SBOMs are emitted per native-interface release, with an explicit authority
order** (Option B — settled):

- **Authoritative — 1ES SPDX (signed).** The internal 1ES
  `publish-1es-artifact.yml` template auto-injects an SPDX 2.2 manifest
  (`sbomEnabled`, default on for internal builds) with per-file hashes, plus a
  **COSE signature** (`manifest.spdx.cose`) and a build-scope attestation
  (`bsi.json`/`bsi.cose`). This is the org-authoritative evidence — Microsoft can
  sign it, and we cannot easily reproduce that signing ourselves. It is the
  document a release/audit ultimately trusts.
- **Companion — CycloneDX (`cargo-cyclonedx`).** [`cargo-cyclonedx`](https://github.com/CycloneDX/cyclonedx-rust-cargo)
  reads `Cargo.lock`, is native to the Rust build, and emits CycloneDX JSON. It
  is **Rust-dependency-accurate** (PURLs straight from the resolved graph),
  travels *with* each artifact, and is the format some downstream SCA tools
  (e.g. Dependency-Track) ingest natively. It is **not** signed — its value is
  Cargo accuracy + interop, not trust. We keep it *in addition to* SPDX, not
  instead of it. (`syft` remains a viable cross-ecosystem alternative if we later
  want one tool across Rust + Go outputs.)

The two are cross-checkable: the same crate/version set should appear in both,
and (once binaries are staged into the SBOM scan folder — see below) the same
per-file SHA256 should appear in the SPDX manifest and in `SHA256SUMS`.

**Where published:** attached to each per-target artifact in the `build` stage,
next to `rust-driver-native-interface-metadata.json`, and included in the
published `azure-cosmos-driver-modules` drop.

### Local rehearsal

`pipeline/Invoke-LocalSupplyChain.ps1` exercises the complete mechanics without
claiming production trust:

1. builds the static and dynamic libraries with `cargo-auditable`;
2. test-signs the Windows DLL with a disposable self-signed certificate;
3. generates a Cargo-aware CycloneDX 1.5 SBOM;
4. generates and validates a Microsoft SPDX 2.2 SBOM with `sbom-tool`;
5. generates `SHA256SUMS` from the final local bytes;
6. generates and tests the Go module; and
7. creates a local-only `Azure/azure-cosmos-driver` branch, commit, and PR
   preview without pushing.

The rehearsal validates formats and workflow wiring. It cannot substitute for
1ES builder identity, a signed production attestation, ESRP/Azure Trusted
Signing, or Apple notarization.

The unwired production skeleton fails closed at `security_evidence` until
per-target CycloneDX generation and the existing signed 1ES BSI evidence are
verified. Removing that gate is part of trusted-pipeline wiring, not local
rehearsal.

## 3. Provenance

**Question a consumer must be able to answer:** *who built this exact `.a`, from
what source, and how do I know it wasn't modified?*

- **Who / trusted build:** built only in **1ES** managed pools via
  `native-driver.yml` — not on a dev box. The build identity and pool are the
  root of trust.
- **From what source:** `build.rs` already bakes a provenance string into
  `BUILD_IDENTIFIER` (crate version + ADO `BUILD_SOURCEVERSION` /
  `BUILD_SOURCEBRANCH` / `BUILD_BUILDID` / `BUILD_BUILDNUMBER` + timestamp), and
  each `rust-driver-native-interface-metadata.json` records `source_commit`
  (the `git rev-parse HEAD`), the Rust driver and native-interface versions,
  and toolchain versions. The local rehearsal additionally records whether the
  source tree is clean and hashes the actual tracked plus non-ignored source
  snapshot. A dirty build therefore cannot masquerade as the recorded `HEAD`.
  - **Driver path-dep caveat:** `azure_data_cosmos_driver` 0.7.0 is a **path**
    dependency, *not* a published/workspace crate. Provenance therefore pins the
    **source commit** of this repo (which contains the driver source), not a
    crates.io release. This is an **open risk** to flag: until the driver is a
    published or workspace-versioned dependency, "which driver built this" is
    answered by the repo commit, not a registry version.
- **How to verify integrity:** the trusted build publishes a cryptographically
  secure **SHA256** per `.a` in `SHA256SUMS`, plus the existing signed **1ES BSI**
  build evidence. The consumer:
  1. downloads the `.a` + `SHA256SUMS`,
  2. recomputes SHA256 and compares,
  3. (optionally) verifies the BSI points at the expected build, repo, and commit.

Publishing the SHA256 **out of the trusted build** (not recomputed later on an
untrusted box) is what makes the hash meaningful.

### 3.1 Provenance decision — existing 1ES BSI

M2 reuses the signed build-scope evidence already emitted by 1ES. It establishes
that the governed build ran from the recorded repository commit. The per-target
metadata and `SHA256SUMS` identify the exact output bytes.

## 4. cargo-auditable

[`cargo-auditable`](https://github.com/rust-secure-code/cargo-auditable) embeds
the full dependency tree into supported compiled binaries, so an auditor can
recover it even without `Cargo.lock`.

- **Wiring:** the real artifact build in `Build-NativeMatrix.ps1` runs
  `cargo auditable build --release --target <triple> ...`. (The separate
  `--print native-static-libs` call is info-only and does not need auditable.)
  CI installs it with `cargo install cargo-auditable --locked`; the `-NoAuditable`
  switch exists only for dev boxes and **must not** be set in CI.
- **Read-back:** `rust-audit-info` successfully recovers the embedded manifest
  from the dynamic library. It rejects `libazurecosmosdriver.a` because a static
  archive is not an executable object in the format the reader accepts. For the
  static Go path, cross-check the CycloneDX SBOM against the final linked Go
  executable instead of claiming direct `.a` read-back. Divergence between the
  embedded manifest and the SBOM is a tamper signal.

## 5. Summary — controls mapped to consumption paths

| Control | Static `.a` (Go / cgo) | Dynamic lib (.NET / Java / Python) |
| ------- | ---------------------- | ---------------------------------- |
| OS code signing | **N/A** — cannot sign a `.a`; consumer signs their Go binary | **Yes** — Authenticode (Win) / codesign+notarize (macOS); Linux none |
| SBOM (SPDX authoritative + CycloneDX companion) | Yes, per artifact | Yes, per artifact |
| Provenance (1ES build + commit pin + attestation) | Yes | Yes |
| Published SHA256 (out of trusted build) | **Primary** integrity control | Secondary (backs up the signature) |
| cargo-auditable embedded manifest | Validate after linking into the Go executable; direct `.a` read-back is unsupported | Yes; verified directly from the dynamic library |
| Final runtime-signed artifact | The **consumer's** Go executable | **Our** signed dynamic lib |

## 6. Open risks (ratify — not silently decided)

1. **windows/arm64 gnu toolchain** immaturity — **deferred out of M2**. A robust
   windows-arm64 build needs MSVC + dynamic DLL output; tracked by
   [#5047](https://github.com/Azure/azure-sdk-for-rust/issues/5047). It is not
   built by this pipeline (it lives in `optional_targets` in `build-matrix.json`).
2. **musl/glibc tag footgun** — a musl host that forgets `-tags cosmos_musl`
   gets a loud link error (design in `pipeline/README.md` / `New-GoModules.ps1`).
3. **static-`.a` signing gap** — accepted and reconciled in §1.4; trust is
   SBOM + provenance + SHA256, not a signature on the `.a`.
4. **driver path-dep not published** — provenance pins a repo commit, not a
   registry version (§3).
5. **dynamic-library distribution deferred** — the `.dll`/`.dylib`/`.so` path
   (.NET / Java / Python) is out of M2 scope. The build still emits the dynamic
   libs best-effort as an extensibility hook, but distribution and per-OS code
   signing for that path are tracked by
   [#5048](https://github.com/Azure/azure-sdk-for-rust/issues/5048).
