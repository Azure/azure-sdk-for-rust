# ADR-008 — Model outages and failover as dynamic account topology

**Status:** Proposed
**Date:** 2026-07-14

## Context

The hosted emulator needs to reproduce region outages and write-region changes so SDK routing,
topology refresh, and retry behavior can be tested without a live account. Replication
pause/resume models lag to a target region, but it does not make that region unreachable or remove
it from account discovery. The configured region list and initial write region are otherwise
static.

## Decision

Represent availability and write ownership as runtime account-topology state shared by account
discovery and request dispatch:

- An offline-region set determines which regions appear in readable and writable locations.
  Requests sent directly to an offline region fail with `503 Service Unavailable`.
- Single-write accounts maintain a current write-region selection. Changing it updates writable
  locations and the write-region guard used by data-plane operations.
- State changes are visible through subsequent account reads; clients observe them through their
  normal metadata refresh path rather than an emulator-specific client hook.

The management REST API mutates this state, while the in-process emulator can use the same store
operations directly.

## Consequences

SDK failover behavior is driven by the same account topology contract used with the service.
Replication lag and endpoint outage remain distinct failure models. Existing static behavior is
preserved until the dynamic state is changed.

### Known races

Topology mutation and the data plane are guarded by separate locks, so a mutation is not atomic
from a concurrent reader's perspective. These windows are narrow, only reachable when a topology
change races live traffic, and closing them needs deliberate cross-lock coordination:

- **Seeding vs. publication (`add_region`).** `seeded_from` deep-copies the source region before
  the new region is published. A write committing after its source partition was copied but before
  publication is absent from the copy and is never replicated, so `SeedingPolicy::Immediate` can
  advertise permanently stale data.
- **Publication vs. version bump (`add_region`).** The topology is published before vector-clock
  versions are advanced, so an account read in that window can route a session request against
  partitions still exposing the pre-change version — the old token is compared rather than
  superseded. `remove_region` avoids this by bumping first, which is safe there; `add_region`
  cannot, because the new region's own partitions must be included in the bump.
- **Split interactions.** A region added mid-split can inherit a stale partition layout;
  `catch_up_from` can clobber writes acknowledged by a delayed-seeding region under multi-write;
  and an in-flight split can revert `advance_vector_clock_versions`.

## Alternatives

- Treating replication pause as an outage was rejected because requests can still reach the
  region and account discovery still advertises it.
- Rebuilding the emulator for every topology change was rejected because clients could not test
  runtime refresh and recovery behavior.
- Adding SDK-specific failover switches was rejected because they would bypass normal account
  discovery and routing.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/AGENTS.md`
- The in-process implementation of this contract already exists in
  `azure_data_cosmos_driver`'s in-memory emulator (`EmulatorStore::add_region` /
  `remove_region` / `begin_region_removal` / `set_write_mode` /
  `set_write_region`, spec section "Dynamic Account Topology"). It answers the
  questions this ADR poses, validated against live accounts: a removed region's
  endpoint returns `403/1008` while the account read still advertises it for
  minutes; region IDs are never reused, and a re-added region keeps its original
  ID; every membership change bumps the session-token version, which is what
  supersedes a client's older token. Note the account read carries **no**
  `_etag` at all, so a client cannot use one to detect topology changes. The
  hosted emulator should match that observable behavior.
