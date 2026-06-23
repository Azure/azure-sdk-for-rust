# ADR 0005 — Native lib exports an ABI version; hosts check it before use

**Status:** Accepted (proposed for review)

## Context
The native library and each language SDK ship on independent cadences. A managed/native version mismatch at the byte level (the Application Binary Interface — ABI) corrupts memory or crashes deep in a call, with no clear cause.

## Decision
- The native library exports **`cosmos_abi_version() -> u32`** — a **monotonically increasing integer ABI revision, not a packed SemVer** — and ships the same value in the hand-off's `ABI_VERSION`.
- Every language host reads it **at load, before the first real call**, and accepts only the **single ABI revision it was built against** (`Expected`): any mismatch — the native revision being **lower (too old)** *or* **higher (too new / unknown)** — fails fast with a versioned message, so a different native is never silently accepted. There is **no accepted range**: because every release is one coordinated native version fanned out to all SDKs (ADR 0010), each host targets exactly one ABI revision, which keeps the compatibility test matrix at a single point instead of a range.
- A breaking C-ABI change is a **major bump**, coordinated with SDK releases; each SDK pins the **exact** native version it targets (ADR 0010), not a range.

## Consequences
- Mismatches surface as a clear, actionable error instead of a crash.
- Both too-old and too-new natives are rejected explicitly; a host never runs against an ABI revision it does not understand.
- Adds a tiny, one-time check at load per process.
- Creates an explicit compatibility contract across all three languages.

## Alternatives considered
- Rely on package version pins only — rejected: private deployment / manual overrides bypass the package graph.
- No handshake — rejected: silent corruption is the worst failure mode here.
