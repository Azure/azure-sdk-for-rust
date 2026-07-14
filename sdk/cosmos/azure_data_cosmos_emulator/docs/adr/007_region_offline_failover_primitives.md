# ADR-007 — Add runtime region-offline and write-region failover primitives

**Status:** Accepted
**Date:** 2026-07-14

## Context

The control plane must support taking a region fully offline and changing the write region at
runtime, to exercise the driver's failover behavior. The store has replication pause/resume and a
per-partition failover flag, but **no** first-class region-offline state and **no** runtime
write-region selection — the write region is fixed to the first configured region in single-write
mode.

## Decision

Introduce two new store primitives in PR2, gated behind the host feature:

- **Region offline/online:** a runtime-mutable set of offline regions. Offline regions are dropped
  from the account topology's readable/writable locations and return `503` for data-plane requests.
- **Runtime write-region failover:** a runtime-mutable write-region override (an
  `Arc<RwLock<Option<String>>>`, mirroring the existing per-partition-failover atomic pattern)
  consulted by `is_write_region` / `write_region_name` and by the writable-locations topology.

Both ship with net-new in-process tests and corresponding management REST endpoints.

## Consequences

The emulator can reproduce region outage and planned/unplanned write-region failover, so the
driver's re-routing and topology-refresh paths are testable offline. The override defaults to
`None`, preserving today's "first region is the write region" behavior when unused.

## Alternatives

- Reusing replication pause/resume to approximate an offline region was rejected: it models lag,
  not a region being unreachable, and does not remove the region from topology.
- Making these primitives part of PR1 was rejected: the existing suites do not need them, and they
  warrant dedicated tests, so they belong in their own PR.

## References

- Plan & summary: ../plan.md
