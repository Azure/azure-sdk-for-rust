# ADR-002 — Model each region as a distinct localhost port

**Status:** Accepted
**Date:** 2026-07-14

## Context

The driver reads account topology and routes subsequent requests to each region's
`databaseAccountEndpoint`, which must be an independently reachable network endpoint. The store
already resolves a request's region by `(scheme, host, port)`.

## Decision

Bind one gateway listener per region on a distinct `127.0.0.1:{port}`. A single shared
`EmulatorStore` backs every listener; the region is resolved per request from the `Host` header.
A single-region account is simply one port. Gateway 2.0 adds an optional second RNTBD port per
region.

## Consequences

Multi-region topologies map cleanly onto the driver's existing endpoint routing with no new
region-resolution mechanism. The store models all regions internally, so one process and one store
serve every port. Region gateway URLs in the config are plain `http://127.0.0.1:{port}` values.

## Alternatives

- A single port differentiated by `Host` header (e.g. `eastus.localhost`) was rejected: it relies
  on client-side DNS/hosts trickery and does not match how the driver dials distinct endpoints.
- One process per region was rejected: it would fragment the shared store and complicate
  cross-region replication and failover simulation.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`
