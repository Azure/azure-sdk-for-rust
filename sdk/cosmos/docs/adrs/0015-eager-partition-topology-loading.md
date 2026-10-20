# ADR-0015 — Resolve stable metadata and partition topology during construction

**Status:** Accepted
**Date:** 2026-09-17

## Context

Account topology and container identity are required by most operations.
Resolving them lazily would allow invalid endpoints, account discovery
failures, missing containers, and addressing errors to surface on an unrelated
first operation instead of at an explicit construction boundary.

Partition topology is required by queries, change feed, partition-level
failover, session management paths, and other routing decisions. The need can
also appear after a client is created because account capabilities such as
per-partition automatic failover can change dynamically.

Loading the complete partition-key-range map only when an operation first needs
it makes that operation absorb an unpredictable metadata latency spike. It can
also move topology failures away from the explicit, fallible container
construction boundary.

ADR-0010 established eager account and container metadata resolution but
selected lazy partition-key-range loading to avoid construction I/O for
applications that did not need topology. Experience since that decision shows
that topology is important to most applications and can be required
unexpectedly. This ADR carries forward the stable metadata decisions and
replaces the partition-topology decision, fully superseding ADR-0010.

## Alternatives considered

- **Resolve all metadata lazily.** Rejected because construction could return
  clients that are not ready to route an operation, and stable metadata failures
  would surface unpredictably on first use.
- **Resolve account and container metadata eagerly but always load partition
  topology lazily.** Rejected because the first topology-dependent operation
  would absorb the full-cache latency and failure risk, even when failover or an
  unexpected cross-partition query introduced the requirement.
- **Always load partition topology eagerly.** Rejected as an immediate
  compatibility requirement because some existing applications need time to
  accommodate the additional up-front I/O and memory usage.
- **Refresh all metadata continuously in the background.** Rejected because it
  does not provide a clear readiness boundary and does not eliminate
  operation-time refresh after partition splits or container recreation.

## Decision

`CosmosClientBuilder::build` is async and fallible and eagerly resolves account
metadata while initializing the per-account driver.

`DatabaseClient::container_client` is async and fallible and eagerly resolves
the container resource ID and partition-key definition.

The partition topology cache is always available. Its initial load timing is
controlled by `PartitionFailoverOptions.partition_topology_cache_mode`:

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

This decision fully supersedes ADR-0010. Its account and container metadata
decisions are retained here, while its lazy partition-key-range decision is
replaced.

## Consequences

Successfully constructed clients have the stable metadata needed for immediate
use, and metadata failures occur at explicit construction boundaries. Client
and container construction perform network I/O.

In eager mode, container resolution performs an additional metadata change-feed
sequence and retains a routing map for each resolved container. In return,
topology-dependent operations avoid a cold-cache latency spike and initial
topology failures surface at the explicit container-resolution boundary.

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
- Superseded metadata resolution decision
  (`adrs/0010-metadata-resolution-and-client-construction.md`)
- Issues and merged PRs `Azure/azure-sdk-for-rust#5279`,
  `Azure/azure-sdk-for-rust#3799`, `Azure/azure-sdk-for-rust#3864`,
  `Azure/azure-sdk-for-rust#3553`, and `Azure/azure-sdk-for-rust#4007`
