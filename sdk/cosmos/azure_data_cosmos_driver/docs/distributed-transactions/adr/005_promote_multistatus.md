# ADR-005 — Promote `207` in request order

**Status:** Accepted
**Date:** 2026-07-06

## Context

A `207 MultiStatus` must be collapsed to a single actionable status. The coordinator
may return per-operation results out of request order, so the order in which
promotion scans them determines which failure wins.

## Decision

Reorder results by `index` first, then promote to the **first** per-operation status
`>= 400` that is not `424` in **request order**. `424 FailedDependency` is neutral
and never promoted. This matches .NET's `DistributedTransactionResponse` (PR #5974),
which reorders then promotes the lowest-request-index failure.

## Consequences

For an out-of-order multi-failure response, the promoted envelope status is
deterministic in request order and agrees with the .NET SDK against the same
coordinator. `424` correctly signals "aborted due to a sibling," not a root cause.

## Alternatives

Promoting in wire order before reorder was rejected because it picks a different,
coordinator-emission-order-dependent winner than .NET and is not stable against
out-of-order responses.

## References

- Distributed Transactions spec: ../spec.md
- [.NET response reordering / fail-closed handling](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5974)
