# ADR 0009 — Distribute as per-language packages on each language's existing feeds

**Status:** Accepted (proposed for review)

## Context
Each language already has mature feed infrastructure with its own ACLs, signing (ESRP), and governance: .NET on nuget.org + an internal azure-sdk NuGet feed, Go on the azure-sdk-for-go feed, Java on the azure-sdk-for-java Maven feed + Maven Central. A neutral, cross-language consumer bundle would force consumers to download formats they cannot use (a Go user pulling DLLs/JARs) and would require new consumer feed infrastructure.

## Decision
- The native driver is distributed as **a normal dependency in each language's native package format on that language's existing internal + external feed** — not as a neutral cross-language bundle and not on any new consumer feed.
  - .NET → NuGet NativeAssets + meta-package on nuget.org + internal NuGet feed (ADR 0002).
  - Go → cgo-consumable header + lib via the azure-sdk-for-go feed (ADR 0003).
  - Java → JAR on the azure-sdk-for-java Maven feed + Maven Central (future; not finalized).
- A consumer only ever downloads its own language's package format.

## Consequences
- Reuses each SDK's mature feed, ACL, signing, and governance — **no new consumer feed to build**.
- Idiomatic: matches how the Azure SDKs already ship dependencies.
- Drift protection now depends on pipeline discipline (ADR 0008) rather than a single shared consumer artifact.

## Alternatives considered
- Single neutral consumer bundle/feed carrying all formats — rejected: forces irrelevant bytes on consumers; new infra; not idiomatic.

> Together with ADR 0001, this replaces the earlier "consumers download a canonical bundle" model: provenance is an internal hand-off (0001); distribution is per-language feeds (0009).
