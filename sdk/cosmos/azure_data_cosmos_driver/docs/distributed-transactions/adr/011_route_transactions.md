# ADR-011 — Route distributed transactions through account write regions

**Status:** Accepted
**Date:** 2026-07-07

## Context

Distributed transactions are coordinated by the service-side DTC. Both write
transactions and read-snapshot transactions must reach the coordinator in the
account's current write region. Caller regional preferences are appropriate for
ordinary point reads and writes, but they are not the routing contract for DTX.

During failover or when a region is taken offline, the account metadata can expose
more than one writable region even for single-master accounts. The first writable region in the
latest account topology is the steady-state target. If that region is no longer the
right coordinator region, the service returns `403 / 3 (WriteForbidden)`, which the
driver uses to refresh topology and advance to the next writable region.

## Decision

Maintain a separate account-order write endpoint snapshot for DTX routing. Route
both `CommitDistributedTransaction` and `ReadDistributedTransaction` through that
account-order write endpoint list, ignoring caller preferred-region reordering and
the PPAF write path that probes via read endpoints for normal single-master writes.

The shared operation pipeline still applies endpoint availability, excluded-region
filters, and `403 / 3` topology refresh before DTX-specific retry classification.

## Consequences

DTX steady-state routing targets the DTC coordinator in the first write region from
the latest account metadata. Under write-region failover or when a region is taken offline, DTX
retries follow the same account-topology write-region order and let `403 / 3`
drive topology refresh and advancement. Normal point-operation routing remains
unchanged: read regional preferences, normal write preferences, PPAF, and PPCB keep
their existing endpoint selection behavior.

## Alternatives

Using `preferred_write_endpoints` for DTX was rejected because that list is
reordered by caller regional preferences and can point DTX at a non-coordinator
region in steady state. Reusing the PPAF read-endpoint probing path was rejected
because DTX is coordinator-scoped and should not discover write regions by walking
read preferences.

## References

- Distributed Transactions spec: ../spec.md
