# ADR 0004 — Native lib exports an ABI version; hosts check it before use

**Status:** Accepted (proposed for review)

## Context
The native library and each language SDK ship on independent cadences. A managed/native version mismatch at the byte level (the Application Binary Interface — ABI) corrupts memory or crashes deep in a call, with no clear cause.

## Decision
- The native library exports **`cosmos_abi_version() -> u32`**, and ships the same value in the hand-off's `ABI_VERSION`.
- Every language host reads it **at load, before the first real call**, and fails fast with a versioned message when it is below the host's `MinSupported`.
- A breaking C-ABI change is a **major bump**, coordinated with SDK releases; each SDK pins a compatible range.

## Consequences
- Mismatches surface as a clear, actionable error instead of a crash.
- Adds a tiny, one-time check at load per process.
- Creates an explicit compatibility contract across all three languages.

## Alternatives considered
- Rely on package version pins only — rejected: private deployment / manual overrides bypass the package graph.
- No handshake — rejected: silent corruption is the worst failure mode here.
