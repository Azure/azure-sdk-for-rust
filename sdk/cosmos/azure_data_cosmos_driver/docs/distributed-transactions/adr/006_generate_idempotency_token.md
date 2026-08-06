# ADR-006 — Generate and reuse idempotency tokens

**Status:** Accepted
**Date:** 2026-07-06

## Context

Safe replay needs server-side dedupe. .NET auto-generates a `Guid` token, reuses it
across internal retries, and does not accept a caller-supplied token.

## Decision

Generate one `Uuid` per `DistributedTransactionRequest`, send it as
`x-ms-cosmos-idempotency-token` on every attempt, and expose it read-only on the
response. Mark DTX operation types idempotent so the retry machinery treats replays
as safe.

## Consequences

Idempotency within a single commit is automatic. The token is observability and
correlation only — there is **no** exactly-once guarantee across process restarts;
callers must reconcile after an unknown outcome.

## Alternatives

Accepting a caller token for cross-restart exactly-once is deferred as an open
question in the .NET API review.

## References

- Distributed Transactions spec: ../spec.md
- [.NET DTX API review](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5877)
