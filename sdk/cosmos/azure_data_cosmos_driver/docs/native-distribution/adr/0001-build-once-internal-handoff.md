# ADR 0001 — One build → internal-only hand-off artifact; no neutral consumer bundle

**Status:** Accepted (proposed for review)

## Context
The native driver must reach .NET, Go, and later Java, each with a different package format and its own feed. Two concerns are easily conflated: **provenance** (there must be exactly one build + binary-signing per release, or the languages drift onto different driver builds) and **distribution** (how a consumer pulls the bytes). A neutral consumer-facing bundle would force a Go user to download a package containing DLLs/JARs they cannot use, and would require standing up new consumer feed infrastructure.

## Decision
- For each release, **one** Rust build produces all platform binaries (cdylib + staticlib), the cbindgen C header, an `ABI_VERSION`, and checksums, and **signs the binaries**.
- These are published as an **internal-only hand-off artifact** (e.g. an Azure Artifacts Universal Package or a pipeline artifact) that is the **single source of truth for provenance**.
- This artifact is **not consumer-facing and not language-shaped.** Consumers never download it; per-language publish jobs consume it (distribution is ADR 0002; the pipeline that produces and fans it out is ADR 0009).
- The hand-off is **RID-keyed** (one subtree per `<os>-<arch>`/libc target) and carries **both link forms per RID** — the dynamic `.dll`/`.so`/`.dylib` *and* the static `.a` — plus a single **C-only header** (no C++ constructs). This layout is a hard requirement so any language, including a future Java JAR (ADR 0002), can repackage straight from the hand-off with **no redesign and no rebuild**.
- The Rust crate declares `crate-type = ["cdylib", "staticlib"]` so both link forms come from one build.

## Consequences
- Single provenance + SBOM anchor without forcing any cross-language or cross-platform download on consumers.
- No new **consumer** feed to operate; only an internal hand-off artifact.
- All per-language jobs must consume this artifact rather than rebuilding (enforced by ADR 0009).

## Alternatives considered
- Neutral consumer-facing bundle/feed — rejected: forces irrelevant bytes on consumers; new infra.
- Per-language independent builds — rejected: drift risk, duplicated signing.

> Pairs with ADR 0002: provenance is this internal hand-off (0001); distribution is per-language feeds (0002). The two are intentionally decoupled.

