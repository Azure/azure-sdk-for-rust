# ADR-007 — Merge per-operation session tokens strictly

**Status:** Accepted
**Date:** 2026-07-06

## Context

A DTX spans partitions, so a single commit-level token is insufficient; each operation
returns its own `{pkRangeId}:{lsn}` token. A corrupt token must not silently weaken
read-your-own-writes.

## Decision

Merge tokens per operation keyed by `partitionKeyRangeId`; support signed region
LSNs; reject malformed tokens **strictly** under Session consistency and best-effort
otherwise; run the merge only on terminal success.

## Consequences

Subsequent Session reads on any touched container see committed state; a committed
transaction is never failed due to token bookkeeping.

## Alternatives

Best-effort token handling everywhere was rejected because it hides corruption under
the strongest consistency where correctness matters most.

## References

- Distributed Transactions spec: ../spec.md
- [.NET malformed DTX session-token handling](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5958)
