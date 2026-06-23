# Design: distributing the native Cosmos driver to language SDKs

> **Status:** Draft for review · **Author:** Cosmos Rust SDK team · **Type:** Design discussion (decisions are recorded separately as ADRs — see [`adr/`](adr/0000-index.md))
>
> This is the *discussion* doc: context, alternatives, and the "why". The **decisions** live as short, immutable records in [`adr/`](adr/0000-index.md) and are **not restated here** — this body keeps only what the ADRs don't carry (the narrative, the model diagram, rollout, and open questions). If the two disagree, the ADR wins.

## 1. Purpose

The crate `azure_data_cosmos_driver_native` compiles to a native library — `azurecosmosdriver.{dll,so,dylib}` (and a static `.a`) plus a cbindgen-generated C header — wrapping the schema-agnostic driver core (see `NATIVE_WRAPPER_SPEC.md`, introduced in #4461). It is the single reuse point for **every non-Rust language SDK**. This doc defines how that compiled artifact reaches those SDKs:

- **.NET** (`Microsoft.Azure.Cosmos`) — first consumer, via NuGet.
- **Go** (`azcosmos`) — near-term, via cgo (the Foreign Function Interface — FFI — mechanism Go uses to link C libraries). **Not** NuGet.
- **Java** — future, likely a JAR with bundled natives. Consumption model **not finalized; not urgent.** Called out so the design stays multi-language.

Non-goal: the C-ABI surface itself — that is owned by `NATIVE_WRAPPER_SPEC.md`. This doc moves the *compiled bytes*; it does not define the functions.

## 2. Core principle: separate provenance from distribution

Provenance and distribution are two different concerns, and the design keeps them decoupled:

- **Provenance (build-once):** there must be exactly **one** build + binary-signing of the machine code per release, or .NET, Go, and Java silently drift onto different driver builds. This lives **internal to CI** and is never consumer-facing.
- **Distribution:** how a consumer physically pulls the bytes. This should be **each language's existing, idiomatic feed** — not a new neutral feed, and not a cross-language bundle.

The whole design follows from keeping these apart: **one internal build artifact → fan-out to per-language packages on per-language feeds.**

## 3. Goals and constraints

| # | Goal |
|---|------|
| G1 | **Build-once provenance:** one signed set of platform binaries per release is the source of truth; no language rebuilds the driver. |
| G2 | **Idiomatic distribution:** each language consumes the driver as a normal dependency on its own feed (NuGet / Go module / Maven) — a Go consumer never sees a DLL or JAR. |
| G3 | A managed/native **version mismatch fails fast and clearly**, never as memory corruption. |
| G4 | Existing consumers are **never force-migrated by a routine (minor/patch) bump**; native arrives in a **new SDK major** (ADR 0007), adopted deliberately. |
| G5 | **No new consumer-facing feed infrastructure** — reuse each SDK's mature feed, ACL, signing, and governance. |

Hard constraint: the binary is built in `azure-sdk-for-rust`, but each language SDK lives in its own repo (`azure-cosmos-dotnet-v3`, `azure-sdk-for-go`, `azure-sdk-for-java`). Distribution crosses that repo boundary.

## 4. Decisions at a glance

Recorded as ADRs; this is the index with one-line rationale. See [`adr/`](adr/0000-index.md) for the authoritative form.

| ADR | Decision | One-line why |
|---|---|---|
| [0001](adr/0001-build-once-internal-handoff.md) | One build produces all platform binaries + header + ABI version as an **internal-only hand-off artifact**; no neutral consumer bundle/feed. | Single provenance without forcing a cross-language download. |
| [0002](adr/0002-per-language-feed-distribution.md) | Distribution is **per-language native packages into each language's existing internal + external feed**; no neutral consumer feed. | Idiomatic; reuses mature per-language infra; download-only-your-format. |
| [0003](adr/0003-dotnet-nuget-nativeassets.md) | .NET consumes via **per-RID NuGet NativeAssets + meta-package** on nuget.org + the internal NuGet feed. | Matches SkiaSharp / SqlClient.SNI; download only your RID. |
| [0004](adr/0004-go-cgo-prebuilt.md) | Go consumes via **cgo against a prebuilt header + lib** delivered through the azure-sdk-for-go feed (Universal Package / vendored module). | Go has no NuGet; cgo links a C header + lib at build time. |
| [0005](adr/0005-abi-version-handshake.md) | The native lib exports **`cosmos_abi_version()`**; every host checks it before first use. | Independent cadences drift; fail fast, not corrupt. |
| [0006](adr/0006-binding-owns-marshalling.md) | Each binding **owns its marshalling and buffer copy-out**; the ABI stays bytes-in/bytes-out. | Keeps the ABI schema-agnostic. |
| [0007](adr/0007-native-is-opt-in.md) | Native transport ships in a **new SDK major**; no parallel managed transport, no silent fallback. | A major-version cutover, not a per-call opt-in. |
| [0008](adr/0008-platform-matrix.md) | A defined **platform matrix**; unsupported platforms fail with an actionable error. | Bounded support surface. |
| [0009](adr/0009-build-and-signing-pipeline.md) | **One build, sign the binaries once, fan-out** to per-language publish jobs that must consume the hand-off and never rebuild. | Provenance + trust enforced by pipeline discipline. |
| [0010](adr/0010-native-version-fanout.md) | **One native version fanned out to all feeds simultaneously**; each SDK pins that **exact** version. | One source of truth for "which driver"; no cross-language version skew. |

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
 job    ▼           job   ▼            job    ▼      (never rebuilds — ADR 0009)
 NuGet NativeAssets   Go module /        JAR w/ natives
 → nuget.org +        Universal Package  → azure-sdk-for-java
   internal NuGet     → azure-sdk-for-go   Maven feed + Central
        │             feed                 │
   .NET consumers     │              Java consumers
   (only .nupkg)  Go consumers       (only .jar)
                  (only Go artifact)                  ← ADR 0002
```

Consumer distribution is **per-language feeds**; the build-once "bundle" is an **internal CI hand-off**, not the distributed product, so no consumer touches it. This removes any need for a new neutral consumer feed and ensures a consumer downloads only its own package format.

## 6. Per-consumer and pipeline specifics → ADRs

The per-language link model, platform matrix, build/signing pipeline, ABI handshake, and compatibility posture are all **decisions** — so they live in the ADRs rather than being restated here (restating them is exactly what lets the doc and the ADRs drift). The at-a-glance table in [§4](#4-decisions-at-a-glance) links each one; in brief:

- **.NET / Go consumption** — [ADR 0003](adr/0003-dotnet-nuget-nativeassets.md) (per-RID NuGet NativeAssets + meta-package), [ADR 0004](adr/0004-go-cgo-prebuilt.md) (cgo against a prebuilt header + lib).
- **Java** — deliberately **not finalized** (likely a JAR with bundled natives); tracked in [§8](#8-open-questions) so the build-once hand-off ([ADR 0001](adr/0001-build-once-internal-handoff.md)) stays general enough to repackage into a JAR later without redesign.
- **Platform matrix** — [ADR 0008](adr/0008-platform-matrix.md).
- **ABI handshake & versioning** — [ADR 0005](adr/0005-abi-version-handshake.md) and [ADR 0010](adr/0010-native-version-fanout.md).
- **Build, signing & CI** — [ADR 0009](adr/0009-build-and-signing-pipeline.md).
- **Compatibility posture (major-version cutover, no parallel managed transport)** — [ADR 0007](adr/0007-native-is-opt-in.md).

## 7. Rollout

| Phase | Distribution | Consumers | Adoption |
|---|---|---|---|
| 0 — dev loop | local artifact copy | us | local build |
| 1 — internal | hand-off artifact; .NET fat NativeAssets on internal feed; Go via internal Universal Package | internal dogfood | native major, prerelease |
| 2 — public prerelease | .NET `-preview` on nuget.org; Go module published; (Java TBD) | early adopters | native major, `-preview` |
| 3 — GA | per-RID NuGet + meta on nuget.org; Go module GA on azure-sdk-for-go feed | all | native major, GA (native-only) |

The native transport is the defining change of the new major; there is no separate "switch native on by default later" phase, because within that major native is the only transport (ADR 0007).

## 8. Open questions

1. **Internal hand-off shape** — Azure Artifacts Universal Package vs raw pipeline artifact for the one-build → three-jobs hand-off?
2. **Packaging ownership** — central fan-out (in azure-sdk-for-rust publishing into three language feeds) vs each SDK repo pulls the hand-off and publishes itself? (Security-boundary question.)
3. **Go delivery** — Universal Package fetched at build vs a vendored "binaries" Go module?
4. **Static vs dynamic default for Go** — self-contained `.a` vs shared `.so`?
5. **Version mapping** — *Resolved by [ADR 0010](adr/0010-native-version-fanout.md):* one native version is fanned out to all feeds simultaneously and each SDK pins that **exact** version (hosts accept only that one ABI revision, ADR 0005), rather than an independent per-language cadence off a pinned range.
6. **NuGet naming** — `Microsoft.Azure.Cosmos.NativeAssets.*` (reserved prefix) confirmed; meta-package name?
7. **Supply chain** — SBOM / component-governance owner for the Rust crate graph.
8. **Java** — when does it need this, and does JNI vs Panama change any hand-off requirement (e.g. a Panama-friendly header)?
