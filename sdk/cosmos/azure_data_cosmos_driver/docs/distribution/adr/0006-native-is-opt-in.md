# ADR 0006 — Native transport is opt-in until GA, then default-with-fallback

**Status:** Accepted (proposed for review)

## Context
`Microsoft.Azure.Cosmos` (pure managed) and the Go SDK run today on any platform with no native dependency. Introducing a platform-specific native library must not silently break that portability or force a migration on a version bump.

## Decision
- Native transport is **opt-in** per SDK (a client option / build flag / env switch) until it is GA on every supported platform.
- Only after GA may a given SDK make native the **default**, and only with a **fallback** to the managed/pure path on an unsupported platform or a load failure.
- No consumer is ever force-migrated to native by a routine version bump.

## Consequences
- The "runs anywhere" guarantee is preserved throughout the rollout.
- The native path can mature behind a flag with real dogfooding before it is the default.
- Each SDK needs a clean fallback path, not just an error, once native is the default.

## Alternatives considered
- Native-on by default from day one — rejected: breaks portability before the platform matrix is proven.
- Separate "native-enabled" package flavor — kept as a possible packaging option, not mandated here.
