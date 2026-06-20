# Design: distributing the native Cosmos driver to language SDKs

> **Status:** Draft for review · **Author:** Cosmos Rust SDK team · **Type:** Design discussion (decisions are recorded separately as ADRs — see [`adr/`](adr/0000-index.md))
>
> This is the *discussion* doc: context, alternatives, and the "why". The **decisions** live as short, immutable records in [`adr/`](adr/0000-index.md). If the two disagree, the ADR wins.

## 1. Purpose

The crate `azure_data_cosmos_driver_native` compiles to a native library — `azurecosmosdriver.{dll,so,dylib}` (and a static `.a`) plus a cbindgen-generated C header — wrapping the schema-agnostic driver core ([`NATIVE_WRAPPER_SPEC.md`](../NATIVE_WRAPPER_SPEC.md)). It is the single reuse point for **every non-Rust language SDK**. This doc defines how that compiled artifact reaches those SDKs:

- **.NET** (`Microsoft.Azure.Cosmos`) — first consumer, via NuGet.
- **Go** (`azcosmos`) — near-term, via cgo (the Foreign Function Interface — FFI — mechanism Go uses to link C libraries). **Not** NuGet.
- **Java** — future, likely a JAR with bundled natives. Consumption model **not finalized; not urgent.** Called out so the design stays multi-language.

Non-goal: the C-ABI surface itself — that is owned by `NATIVE_WRAPPER_SPEC.md`. This doc moves the *compiled bytes*; it does not define the functions.

## 2. Core principle: separate provenance from distribution

An earlier draft fused two roles into one consumer-facing "bundle". They are different concerns and must be decoupled:

- **Provenance (build-once):** there must be exactly **one** build + binary-signing of the machine code per release, or .NET, Go, and Java silently drift onto different driver builds. This lives **internal to CI** and is never consumer-facing.
- **Distribution:** how a consumer physically pulls the bytes. This should be **each language's existing, idiomatic feed** — not a new neutral feed, and not a cross-language bundle.

The whole design follows from keeping these apart: **one internal build artifact → fan-out to per-language packages on per-language feeds.**

## 3. Goals and constraints

| # | Goal |
|---|------|
| G1 | **Build-once provenance:** one signed set of platform binaries per release is the source of truth; no language rebuilds the driver. |
| G2 | **Idiomatic distribution:** each language consumes the driver as a normal dependency on its own feed (NuGet / Go module / Maven) — a Go consumer never sees a DLL or JAR. |
| G3 | A managed/native **version mismatch fails fast and clearly**, never as memory corruption. |
| G4 | Existing pure-managed / pure-Go consumers are **never force-migrated**; native is opt-in until proven. |
| G5 | **No new consumer-facing feed infrastructure** — reuse each SDK's mature feed, ACL, signing, and governance. |

Hard constraint: the binary is built in `azure-sdk-for-rust`, but each language SDK lives in its own repo (`azure-cosmos-dotnet-v3`, `azure-sdk-for-go`, `azure-sdk-for-java`). Distribution crosses that repo boundary.

## 4. Decisions at a glance

Recorded as ADRs; this is the index with one-line rationale. See [`adr/`](adr/0000-index.md) for the authoritative form.

| ADR | Decision | One-line why |
|---|---|---|
| [0001](adr/0001-build-once-internal-handoff.md) | One build produces all platform binaries + header + ABI version as an **internal-only hand-off artifact**; no neutral consumer bundle/feed. | Single provenance without forcing a cross-language download. |
| [0002](adr/0002-dotnet-nuget-nativeassets.md) | .NET consumes via **per-RID NuGet NativeAssets + meta-package** on nuget.org + the internal NuGet feed. | Matches SkiaSharp / SqlClient.SNI; download only your RID. |
| [0003](adr/0003-go-cgo-prebuilt.md) | Go consumes via **cgo against a prebuilt header + lib** delivered through the azure-sdk-for-go feed (Universal Package / vendored module). | Go has no NuGet; cgo links a C header + lib at build time. |
| [0004](adr/0004-abi-version-handshake.md) | The native lib exports **`cosmos_abi_version()`**; every host checks it before first use. | Independent cadences drift; fail fast, not corrupt. |
| [0005](adr/0005-binding-owns-marshalling.md) | Each binding **owns its marshalling and buffer copy-out**; the ABI stays bytes-in/bytes-out. | Keeps the ABI schema-agnostic. |
| [0006](adr/0006-native-is-opt-in.md) | Native transport is **opt-in** until GA on all platforms, then may default with fallback. | Preserve the "runs anywhere" guarantee. |
| [0007](adr/0007-platform-matrix.md) | A defined **platform matrix**; unsupported platforms fail with an actionable error. | Bounded support surface. |
| [0008](adr/0008-build-and-signing-pipeline.md) | **One build, sign the binaries once, fan-out** to per-language publish jobs that must consume the hand-off and never rebuild. | Provenance + trust enforced by pipeline discipline. |
| [0009](adr/0009-per-language-feed-distribution.md) | Distribution is **per-language native packages into each language's existing internal + external feed**; no neutral consumer feed. | Idiomatic; reuses mature per-language infra; download-only-your-format. |

## 5. The model

```
   Rust build — ONE run, all platforms (azure-sdk-for-rust CI)
   produces .dll/.so/.dylib/.a + azurecosmosdriver.h + ABI_VERSION
   and SIGNS THE BINARIES (Authenticode / codesign + notarize)
                          │
                          ▼
   INTERNAL HAND-OFF ARTIFACT  (Azure Artifacts Universal Package
   or pipeline artifact) — provenance + SBOM anchor.
   NOT consumer-facing. NOT language-shaped.          ← ADR 0001
                          │
        ┌─────────────────┼──────────────────┐
 .NET   │           Go    │            Java   │   ← each job pulls the SAME signed binaries
 job    ▼           job   ▼            job    ▼      (never rebuilds — ADR 0008)
 NuGet NativeAssets   Go module /        JAR w/ natives
 → nuget.org +        Universal Package  → azure-sdk-for-java
   internal NuGet     → azure-sdk-for-go   Maven feed + Central
        │             feed                 │
   .NET consumers     │              Java consumers
   (only .nupkg)  Go consumers       (only .jar)
                  (only Go artifact)                  ← ADR 0009
```

Consumer distribution is **per-language feeds**; the "bundle" is demoted from the distributed product to an **internal CI hand-off** no consumer touches. This removes any need for a new neutral consumer feed and ensures a consumer downloads only its own package format.

## 6. Per-consumer link model

### 6.1 .NET — NuGet (ADR 0002)
Per-RID **NativeAssets** packages (`Microsoft.Azure.Cosmos.NativeAssets.<rid>`) each carry one platform's dynamic lib under `runtimes/<rid>/native/`, fronted by a thin **meta-package** whose `runtime.json` resolves the consumer's RID (Runtime IDentifier — the `<os>-<arch>` platform string) to the right per-RID package. The managed package takes an **opt-in** dependency on the meta-package. Published to nuget.org **and** the internal azure-sdk NuGet feed. At runtime a custom resolver (`NativeLibrary.SetDllImportResolver`) loads the lib, runs the ABI handshake, and errors clearly on unsupported RIDs; P/Invoke uses source-generated `LibraryImport`.

### 6.2 Go — cgo prebuilt (ADR 0003)
Go cannot consume NuGet. cgo links a **C header + library** at `go build` time: a `#cgo CFLAGS: -I<include>` makes `#include "azurecosmosdriver.h"` resolve to the prebuilt header (cgo auto-generates the callable `C.*` symbols — no hand-written signatures), and `#cgo <os> LDFLAGS: -L<lib/rid> -lazurecosmosdriver` links the lib (static `.a` by default for a self-contained binary). The header + lib are delivered through the **azure-sdk-for-go feed** as an Azure Artifacts Universal Package fetched at build, or a vendored "binaries" Go module with per-OS build tags (open Q3). The Go layer already owns the completion-queue receive loop, `cgo.Handle` correlation, and copy-out of response buffers. **No runtime resolver / runtime.json / RID probing** — everything resolves at `go build`.

### 6.3 Java — JAR (future, not finalized)
Likely a JAR bundling per-OS dynamic libs extracted and loaded via `System.load`, with JNI or Panama (Project Panama / FFM) as the FFI, published to the azure-sdk-for-java Maven feed + Maven Central. **Deliberately unspecified** — flagged so the build-once hand-off (ADR 0001) stays general enough to repackage into a JAR later without redesign.

## 7. Platform matrix (ADR 0007)

| Target / RID | Notes |
|---|---|
| `win-x64`, `win-arm64` | Authenticode-signed |
| `linux-x64` (glibc) | low glibc floor for portability |
| `linux-musl-x64` (Alpine) | distinct target — glibc build won't load on musl |
| `linux-arm64` | cross-compiled |
| `osx-x64`, `osx-arm64` | codesigned + notarized |

Out: `wasm` (no FFI story). Open: `win-x86`, `linux-musl-arm64`, mobile.

## 8. Build, signing, and CI (ADR 0008)

One pipeline next to the Rust build produces the per-platform binaries, **signs each binary once** (Authenticode / codesign + notarize), checksums them, and publishes the **internal hand-off artifact**. Per-language publish jobs consume that artifact and emit NuGet / Go-consumable / JAR packages, signing only their **package wrapper** in that language's existing ESRP flow — they **never rebuild or re-sign the native binary**. Build-once is enforced by discipline: all language jobs consume one hand-off from one Rust build. Supply chain: SBOM / component governance for the Rust crate graph is new surface for the consuming orgs — owner TBD (Q7).

## 9. ABI and versioning (ADR 0004)

The lib exports `cosmos_abi_version() -> u32`; each host reads it at load and fails fast on `< MinSupported` with a versioned message. The native binaries carry their own SemVer; each language package pins a **compatible range**. A breaking C-ABI change = major bump + coordinated SDK releases.

## 10. Compatibility posture (ADR 0006)

Native is **opt-in** per SDK (client option / build flag) until GA on every supported platform. Only then may a given SDK make native the default, and only with a **fallback** to the managed/pure path on unsupported platforms or load failure. No consumer is force-migrated by a version bump.

## 11. Rollout

| Phase | Distribution | Consumers | Selection |
|---|---|---|---|
| 0 — dev loop | local artifact copy | us | always |
| 1 — internal | hand-off artifact; .NET fat NativeAssets on internal feed; Go via internal Universal Package | internal dogfood | opt-in |
| 2 — public prerelease | .NET `-preview` on nuget.org; Go module published; (Java TBD) | early adopters | opt-in |
| 3 — GA | per-RID NuGet + meta on nuget.org; Go module GA on azure-sdk-for-go feed | all | opt-in |
| 4 — native-default (later) | same | all | default + fallback |

## 12. Open questions

1. **Internal hand-off shape** — Azure Artifacts Universal Package vs raw pipeline artifact for the one-build → three-jobs hand-off?
2. **Packaging ownership** — central fan-out (in azure-sdk-for-rust publishing into three language feeds) vs each SDK repo pulls the hand-off and publishes itself? (Security-boundary question.)
3. **Go delivery** — Universal Package fetched at build vs a vendored "binaries" Go module?
4. **Static vs dynamic default for Go** — self-contained `.a` vs shared `.so`?
5. **Version mapping** — one native version fanned out to three feeds simultaneously, or independent per-language cadence off a pinned native version?
6. **NuGet naming** — `Microsoft.Azure.Cosmos.NativeAssets.*` (reserved prefix) confirmed; meta-package name?
7. **Supply chain** — SBOM / component-governance owner for the Rust crate graph.
8. **Java** — when does it need this, and does JNI vs Panama change any hand-off requirement (e.g. a Panama-friendly header)?
