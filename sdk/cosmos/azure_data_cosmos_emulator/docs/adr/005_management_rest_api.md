# ADR-005 — Expose control-plane actions via a separate management REST API

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

Topology-changing actions such as partition split and merge are represented as long-running
operation resources. Creating one returns `202 Accepted` and an operation ID. The operation exposes
these phases:

1. `Preparing`: the existing partition topology remains available.
2. `Swapping`: source partition requests return `410/1007`; replacement partitions are not yet
   visible.
3. `Succeeded`: source partitions are gone and replacement partitions are visible.
4. `Failed`: a terminal error is available on the operation resource.

Each operation selects one progression mode:

- `automatic` (default): enter `Swapping` automatically, remain there for `lockDurationMs`, then
  complete. The duration defaults to `0`, which provides no guaranteed observable `Swapping`
  window.
- `manual`: remain in each non-terminal phase until `POST /operations/{operationId}/advance` moves
  the operation forward by one phase.

`lockDurationMs` must be non-negative and is accepted only for `automatic` operations. Supplying it
for `manual` progression returns `400 Bad Request`. Advancing an automatic or terminal operation
returns `409 Conflict`. The operation state machine, not a generic lock API, owns partition locking
so tests cannot create a lock state unrelated to an actual topology transition.

## Consequences

Other SDKs and operators drive emulator-specific behavior (split, merge, failover, offline)
over HTTP without an in-process handle. The Cosmos data plane stays a pure wire-protocol surface,
and the management API never collides with Cosmos paths because it lives on a separate port.
Manual advancement gives tests deterministic assertions before, during, and after a topology
change without timing races.

## Alternatives

- Overloading the Cosmos data-plane ports with a reserved path prefix (e.g. `/_emulator/...`) was
  rejected as more collision-prone and less discoverable than a dedicated port.
- Duplicating database/container CRUD in the management API was rejected: those are already
  expressible through the gateway contract.
- Exposing generic partition lock/unlock endpoints was rejected because they could create states
  that do not correspond to a real topology operation.
- Relying only on lock duration was rejected because timing-based tests are inherently racy.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`
