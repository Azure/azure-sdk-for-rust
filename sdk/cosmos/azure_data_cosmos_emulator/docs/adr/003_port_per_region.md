# ADR-003 — Expose each region through a distinct discoverable endpoint

**Status:** Accepted
**Date:** 2026-07-14

## Context

The driver reads account topology and routes subsequent requests to each region's
`databaseAccountEndpoint`, which must be an independently reachable network endpoint. The store
already resolves a request's region by `(scheme, host, port)`. Fixed ports are convenient for
interactive use but collide when tests run concurrently on shared CI agents.

## Decision

Bind one gateway listener per region to a distinct loopback endpoint. A single shared
`EmulatorStore` backs every listener; the region is resolved per request from the full URL. A
single-region account has one standard gateway endpoint. Gateway 2.0 adds an optional second RNTBD
endpoint per region.

The standard gateway and management ports default to `0`, which asks the operating system to assign
available ports. Gateway 2.0 is enabled by including `gateway20Port`; its value may also be `0` for
OS assignment, while omission disables that optional listener. Explicit non-zero ports remain
available for interactive scenarios that need stable endpoints. Account discovery advertises the
**actual bound URLs**, never the requested port values.

After all listeners are bound, the host writes one machine-readable JSON `ready` record to stdout.
Diagnostic logs go to stderr so automation can parse stdout without log filtering. The record
contains the management endpoint, the hub account endpoint, and every region's standard gateway
and optional Gateway 2.0 endpoint. For example:

```json
{
  "event": "ready",
  "managementEndpoint": "http://127.0.0.1:49150/",
  "accountEndpoint": "http://127.0.0.1:49151/",
  "regions": [
    {
      "name": "East US",
      "gatewayEndpoint": "http://127.0.0.1:49151/",
      "gateway20Endpoint": "http://127.0.0.1:49152/"
    }
  ]
}
```

The management account endpoint returns the same resolved topology after startup. Consumers use
the complete URLs from either contract and do not reconstruct endpoints from configured ports.

## Consequences

Multi-region topologies map cleanly onto the driver's existing endpoint routing with no new
region-resolution mechanism. The store models all regions internally, so one process and one store
serve every endpoint. Concurrent test hosts avoid port collisions by default. Supplying full URLs
to clients preserves the option to use host-based routing or non-loopback bindings in the future
without changing the discovery contract.

## Alternatives

- A single port differentiated by `Host` header (e.g. `eastus.localhost`) was not selected because
  it relies on client-side DNS/hosts setup. Returning full URLs leaves that implementation option
  open for the future.
- One process per region was rejected: it would fragment the shared store and complicate
  cross-region replication and failover simulation.
- Requiring fixed ports was rejected because parallel tests cannot reliably coordinate port
  ownership on shared agents.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`
