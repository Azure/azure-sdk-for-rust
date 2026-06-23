# Architecture Decision Records — native driver distribution

ADRs (Architecture Decision Records) capture **what we decided** and a brief **why**, in a minimal, easy-to-reference form. They are **numbered and immutable**: once accepted, an ADR is not edited; a later ADR may **supersede** it. Detailed discussion and alternatives live in the design doc ([`../distribution-design.md`](../distribution-design.md)), not here — an ADR is the "regression test" that keeps us doing the same thing everywhere.

**Format (template):** Context (2–4 sentences) · Decision (1–3 bullets) · Consequences (2–4 bullets) · Alternatives considered (1 line each) · Status.

| # | Title | Status |
|---|-------|--------|
| [0001](0001-build-once-internal-handoff.md) | One build → internal-only hand-off artifact; no neutral consumer bundle | Accepted |
| [0002](0002-per-language-feed-distribution.md) | Distribute as per-language packages on each language's existing feeds | Accepted |
| [0003](0003-dotnet-nuget-nativeassets.md) | .NET consumes via per-RID NuGet NativeAssets + meta-package | Accepted |
| [0004](0004-go-cgo-prebuilt.md) | Go consumes via cgo against a prebuilt header + lib from the Go feed | Accepted |
| [0005](0005-abi-version-handshake.md) | Native lib exports an ABI version; hosts check it before use | Accepted |
| [0006](0006-binding-owns-marshalling.md) | Each language binding owns marshalling and buffer copy-out | Accepted |
| [0007](0007-native-is-opt-in.md) | Native transport is opt-in until GA, then default-with-fallback | Accepted |
| [0008](0008-platform-matrix.md) | A defined platform matrix; unsupported platforms error clearly | Accepted |
| [0009](0009-build-and-signing-pipeline.md) | One build, sign binaries once, fan-out; jobs never rebuild | Accepted |

> These are **proposed** for the design review. "Accepted" is provisional until the review signs off.
