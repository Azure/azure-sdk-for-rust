<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->
<!-- cSpell:ignore Authenticode codesign notarize notarization cyclonedx auditable staple stapled dylib cdylib staticlib rustls mingw musl syft osxcross esrp SLSA -->

# Native driver supply chain: signing, SBOM, and provenance (M2)

This document is the security/supply-chain design for the native Cosmos driver
artifacts produced by [`pipeline/native-driver.yml`](../pipeline/native-driver.yml).
It covers **each** ask: per-OS code signing, the static-`.a` signing gap, SBOM
generation, provenance, and `cargo-auditable` — and maps them onto the two
distinct consumption paths.

> Companion docs: [`pipeline/README.md`](../pipeline/README.md) (how the build
> runs) and [`docs/NATIVE_WRAPPER_SPEC.md`](../../azure_data_cosmos_driver/docs/NATIVE_WRAPPER_SPEC.md)
> in the driver crate (the ABI contract).

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

## 1. Per-OS code signing

### 1.1 Windows — Authenticode (dynamic `.dll`)

Authenticode-sign `azurecosmosdriver.dll` via an ESRP code-signing task
(`ESRPCodeSigning`), using an Authenticode cert profile.

> **Not** the `EsrpRelease@11` flow in
> [`archetype-rust-release.yml`](../../../../eng/pipelines/templates/stages/archetype-rust-release.yml)
> (lines ~136-153). That flow is *crate-package distribution* signing
> (`contenttype: 'Rust'`, cert `azure-sdk-esrp-release-certificate`). Per-OS
> Authenticode is a different ESRP intent and profile — a distinct task in the
> `sign` stage of `native-driver.yml`.

### 1.2 macOS — codesign + notarize (dynamic `.dylib`)

`codesign` `libazurecosmosdriver.dylib` with a Developer ID profile through ESRP,
then **notarize** and **staple**. An un-notarized dylib trips Gatekeeper on
consumer machines.

### 1.3 Linux — no artifact-signing standard

There is **no** widely-adopted per-file signing standard for a Linux `.so`/`.a`
comparable to Authenticode/codesign. Distro package signing (dpkg/rpm GPG) signs
the *package*, not the library, and we do not ship a distro package here.

**Decision: none is the expected standard for Linux.** Linux trust is carried
entirely by SBOM + provenance + published SHA256 (§2-§3), identical to the
static-path story.

### 1.4 ⚠️ The static-`.a` signing gap (call-out + reconciliation)

Authenticode and codesign sign the **dynamic** library. Go **static-links the
`.a`**, so:

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

## 2. SBOM

Generate a CycloneDX SBOM **per native-interface release**, using Cargo
metadata and `Cargo.lock`.

- **Primary tool:** [`cargo-cyclonedx`](https://github.com/CycloneDX/cyclonedx-rust-cargo)
  — reads `Cargo.lock`, native to the Rust build, emits CycloneDX JSON.
- **1ES manifest:** the internal 1ES `publish-1es-artifact.yml` template also
  auto-injects an SBOM manifest (`sbomEnabled`, default on for internal builds)
  when artifacts are published. We keep **both**: the `cargo-cyclonedx` SBOM is
  Rust-dependency-accurate and travels *with* each artifact; the 1ES manifest
  satisfies the org publishing requirement. (`syft` is a viable cross-ecosystem
  alternative if we later want one tool across Rust + Go outputs.)

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
5. emits an unsigned in-toto/SLSA provenance statement whose builder is
   explicitly `local://...` and whose source identity includes both `HEAD` and
   a SHA256 snapshot of all tracked and non-ignored files;
6. generates `SHA256SUMS` from the final local bytes;
7. generates and tests the Go module; and
8. creates a local-only `Azure/azure-cosmos-driver` branch, commit, and PR
   preview without pushing.

The rehearsal validates formats and workflow wiring. It cannot substitute for
1ES builder identity, a signed production attestation, ESRP/Azure Trusted
Signing, or Apple notarization.

The unwired production skeleton fails closed at `security_evidence` until
per-target CycloneDX generation and a signed 1ES/SLSA attestation are
configured. Removing that gate is part of trusted-pipeline wiring, not local
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
  secure **SHA256** per `.a` in `SHA256SUMS`, plus a **build provenance
  attestation** (1ES/SLSA-style). The consumer:
  1. downloads the `.a` + `SHA256SUMS`,
  2. recomputes SHA256 and compares,
  3. (optionally) verifies the attestation points at the expected repo + commit.

Publishing the SHA256 **out of the trusted build** (not recomputed later on an
untrusted box) is what makes the hash meaningful.

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
| SBOM (cargo-cyclonedx + 1ES) | Yes, per artifact | Yes, per artifact |
| Provenance (1ES build + commit pin + attestation) | Yes | Yes |
| Published SHA256 (out of trusted build) | **Primary** integrity control | Secondary (backs up the signature) |
| cargo-auditable embedded manifest | Validate after linking into the Go executable; direct `.a` read-back is unsupported | Yes; verified directly from the dynamic library |
| Final runtime-signed artifact | The **consumer's** Go executable | **Our** signed dynamic lib |

## 6. Open risks (ratify — not silently decided)

1. **windows/arm64 gnu toolchain** immaturity — may force an msvc-built `.a`
   fallback for that row.
2. **musl/glibc tag footgun** — a musl host that forgets `-tags cosmos_musl`
   gets a loud link error (design in `pipeline/README.md` / `New-GoModules.ps1`).
3. **static-`.a` signing gap** — accepted and reconciled in §1.4; trust is
   SBOM + provenance + SHA256, not a signature on the `.a`.
4. **driver path-dep not published** — provenance pins a repo commit, not a
   registry version (§3).
