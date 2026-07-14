# ADR-004 — Expose control-plane actions via a separate management REST API

**Status:** Accepted
**Date:** 2026-07-14

## Context

Some control-plane actions are part of the Cosmos gateway contract (database/container/offer/item
CRUD, PK-ranges, account read) and are already served on the region gateway ports. Others —
partition split/merge, region offline/online, runtime write-region failover, per-partition
failover toggle, replication pause/resume — have **no** gateway equivalent. They require an
emulator-specific control surface when the emulator runs out of process.

## Decision

Serve the emulator-only control-plane actions through a dedicated **management REST API** on its
own port, distinct from the Cosmos wire protocol. Gateway-native lifecycle operations are **not**
duplicated there; callers use the standard Cosmos endpoints (or startup config seeding) for those.

## Consequences

Other SDKs and operators drive emulator-specific behavior (split, merge, failover, offline)
over HTTP without an in-process handle. The Cosmos data plane stays a pure wire-protocol surface,
and the management API never collides with Cosmos paths because it lives on a separate port.

## Alternatives

- Overloading the Cosmos data-plane ports with a reserved path prefix (e.g. `/_emulator/...`) was
  rejected as more collision-prone and less discoverable than a dedicated port.
- Duplicating database/container CRUD in the management API was rejected: those are already
  expressible through the gateway contract.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`
