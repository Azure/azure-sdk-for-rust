# ADR 0003 — .NET consumes via per-RID NuGet NativeAssets + meta-package

**Status:** Accepted (proposed for review)

## Context
.NET consumers expect NuGet. Native libraries are platform-specific, so a single package carrying every platform forces every consumer to download all of them. Mature native-backed SDKs (SkiaSharp, Microsoft.Data.SqlClient.SNI) solve this with a split package layout.

## Decision
- Ship per-RID **`Microsoft.Azure.Cosmos.NativeAssets.<rid>`** packages, each carrying one platform's dynamic lib under `runtimes/<rid>/native/`.
- Front them with a thin **meta-package** whose `runtime.json` resolves the consumer's RID to the right per-RID package.
- The **native-transport major** of `Microsoft.Azure.Cosmos` takes a **direct dependency** on the meta-package (ADR 0007); the per-RID native then restores transitively like any NuGet dependency. There is no opt-in toggle — taking that major *is* taking native. (A `<PackageReference>` always restores transitively, so an "opt-in dependency" would be a contradiction.)
- Publish to **nuget.org** and the **internal azure-sdk NuGet feed** (per ADR 0002).

## Consequences
- A consumer downloads only its platform's binary.
- More package IDs to version and keep in lockstep (the meta `runtime.json` pins per-RID versions).
- Reuses Azure's already-reserved `Microsoft.Azure.*` prefix — no new org-level construct.

## Alternatives considered
- Single "fat" NativeAssets package — kept only as an **interim** (Phase 1–2) for speed, not GA.
- Embed every platform's native directly in the `Microsoft.Azure.Cosmos` package — rejected: forces every consumer to download all platforms' binaries (the per-RID split exists precisely to avoid that). Note this is *not* about sparing "pure-managed users" — in the native major (ADR 0007) there are none — it is about not shipping six platforms to every consumer.
