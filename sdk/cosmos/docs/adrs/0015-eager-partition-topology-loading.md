# ADR-0015 — Load partition topology during container resolution

**Status:** Accepted
**Date:** 2026-09-17

## Context

Partition topology is required by queries, change feed, partition-level
failover, session management paths, and other routing decisions. The need can
appear after a client is created because account capabilities such as
per-partition automatic failover can change dynamically.

Loading the complete partition-key-range map only when an operation first needs
it makes that operation absorb an unpredictable metadata latency spike. It can
also move topology failures away from the explicit, fallible container
construction boundary.

ADR-0010 selected lazy partition-key-range loading to avoid construction I/O for
applications that did not need topology. Experience since that decision shows
that topology is important to most applications and can be required
unexpectedly.

## Decision

The partition topology cache is always available. Its load timing is controlled
by `PartitionFailoverOptions.partition_topology_cache_mode`:

- `Eager` is the default and strongly recommended mode.
  `CosmosDriver::resolve_container_by_name`,
  `CosmosDriver::resolve_container_by_rid`, and therefore
  `DatabaseClient::container_client`, load the complete partition-key-range map.
  A failed or invalid topology load fails container resolution.
- `Lazy` preserves loading on first topology-dependent use. It is a temporary
  compatibility escape hatch for applications that cannot yet tolerate the
  additional container-resolution I/O or memory cost.

`AZURE_COSMOS_PARTITION_TOPOLOGY_CACHE_MODE` supplies the environment fallback.
An explicit builder value wins over the environment value, and an unset or
invalid value falls back to `Eager`.

This decision supersedes ADR-0010 only for partition-key-range load timing.
Account and container metadata remain eagerly resolved as ADR-0010 specifies.

## Consequences

Container resolution normally performs an additional metadata change-feed
sequence and retains a routing map for each resolved container. In return,
topology-dependent operations avoid a cold-cache latency spike and topology
failures surface at the explicit container-resolution boundary.

The cache retains single-pending-I/O semantics, so concurrent or repeated
resolution of one container does not duplicate a cold topology load. Partition
splits and container recreation still invalidate or refresh cached topology;
eager loading does not eliminate later refreshes.

Applications can choose `Lazy` during migration, but must accept that the first
operation needing topology can incur a full-cache load and fail at that later
point.

## Authoritative references

- Partition-key-range cache specification
  (`specs/0007-partition-key-range-cache.md`)
- Partition-level failover specification
  (`specs/0008-partition-level-failover.md`)
- Metadata resolution decision
  (`adrs/0010-metadata-resolution-and-client-construction.md`)
- Issue `Azure/azure-sdk-for-rust#5279`
