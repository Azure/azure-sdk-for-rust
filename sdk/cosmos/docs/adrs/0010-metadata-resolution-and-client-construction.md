# ADR-0010 — Resolve stable metadata during client construction

**Status:** Accepted
**Date:** 2026-08-31

## Context

Account topology and container identity are required by most operations, while
partition-key-range maps are needed only by partition-aware paths and can
change after splits. Construction timing determines whether failures appear at
a clear client boundary or unpredictably during the first operation.

## Alternatives considered

- **Resolve all metadata lazily.** Rejected because invalid endpoints, account
  discovery failures, missing containers, and addressing errors would surface
  on an unrelated first operation.
- **Load partition-key ranges eagerly with container metadata.** Rejected
  because it adds construction I/O for operations that do not need ranges and
  does not eliminate later refresh after splits.
- **Make construction infallible and refresh in the background.** Rejected
  because it can return clients that are not ready to route an operation.

## Decision

`CosmosClientBuilder::build` is async and fallible and eagerly resolves account
metadata while initializing the per-account driver. Likewise,
`DatabaseClient::container_client` is async and fallible and eagerly resolves
the container resource ID and partition-key definition.

Partition-key-range data remains lazy and is fetched on the first operation
that needs it, with cache and refresh semantics appropriate to changing ranges.

## Consequences and exceptions

Successfully constructed clients have the stable metadata needed for immediate
use, and metadata failures occur at explicit construction boundaries. Client
creation performs network I/O. Partition-aware first use may still incur a
range lookup, and invalidation or topology changes may require later refreshes.

## Authoritative references

- [Partition-key-range cache specification](../specs/0007-partition-key-range-cache.md)
- [`CosmosClientBuilder::build`](../../azure_data_cosmos/src/clients/cosmos_client_builder.rs)
- [`DatabaseClient::container_client`](../../azure_data_cosmos/src/clients/database_client.rs)
- [Container metadata resolution](../../azure_data_cosmos/src/clients/container_client.rs)
- [Driver account initialization](../../azure_data_cosmos_driver/src/driver/cosmos_driver.rs)
- [Lazy partition-key-range cache](../../azure_data_cosmos_driver/src/driver/cache/partition_key_range_cache.rs)
- Merged PRs `Azure/azure-sdk-for-rust#3799`,
  `Azure/azure-sdk-for-rust#3864`, `Azure/azure-sdk-for-rust#3553`, and
  `Azure/azure-sdk-for-rust#4007`
