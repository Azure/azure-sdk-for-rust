# ADR 0009 — One build, sign binaries once, fan-out; jobs never rebuild

**Status:** Accepted (proposed for review)

## Context
The binary is built in `azure-sdk-for-rust`, but the language packages live in other repos and publish to other feeds. The bytes that define the ABI must be signed at their source, and per-language packaging must not be able to alter, re-sign, or independently rebuild them — otherwise the languages drift onto different driver builds (the failure ADR 0001 exists to prevent).

## Decision
- A single pipeline next to the Rust build produces the per-platform binaries (ADR 0001), **signs each binary once** (Authenticode on Windows; codesign + notarization on macOS), checksums them, and publishes the internal hand-off artifact.
- Per-language publish jobs **consume the already-signed hand-off** and emit NuGet / Go-consumable / JAR packages; they **never rebuild or re-sign the native binary** (they may sign their own package wrapper, e.g. the `.nupkg`/`.jar`, in that language's existing ESRP flow).
- Build-once is enforced by discipline: all language jobs consume one hand-off from one Rust build.

## Consequences
- The ABI-defining bytes are signed once, at the source, identically for all languages.
- Per-language jobs are simple repackagers; they can live in or near each SDK repo.
- Requires the hand-off artifact (ADR 0001) and an SBOM / component-governance owner (open Q7).

## Alternatives considered
- Each language rebuilds + signs its own copy — rejected: drift and duplicated trust roots.
- Sign only the language packages, not the native binary — rejected: leaves the actual loaded bytes unsigned.
