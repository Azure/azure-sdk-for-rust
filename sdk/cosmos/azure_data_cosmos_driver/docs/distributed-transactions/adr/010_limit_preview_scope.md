# ADR-010 — Limit preview scope

**Status:** Accepted
**Date:** 2026-07-06

## Context

The preview targets the coordinator's currently supported surface.

## Decision

Support Gateway connectivity, same-account transactions, JSON encoding, and
commit-only semantics (no explicit abort API — abandoning a transaction means not
committing). Read-within-write is not supported.

## Consequences

Matches the .NET preview limitations (PR #5877 §12) and keeps the Rust surface
aligned. These constraints are revisited as the coordinator evolves.

## Alternatives

Broadening scope now was rejected because server support and API review are still in
progress.

## References

- Distributed Transactions spec: ../spec.md
- [.NET DTX API review](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5877)
