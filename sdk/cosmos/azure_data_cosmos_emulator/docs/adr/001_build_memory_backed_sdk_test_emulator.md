# ADR-001 — Build a memory-backed emulator host for Cosmos DB SDK testing

**Status:** Accepted
**Date:** 2026-07-15

## Context

Cosmos DB SDKs need deterministic tests for behavior driven by account and partition topology:
regional routing, endpoint outages and recovery, write-region changes, replication lag, physical
partition split and merge, and Gateway V1 versus Gateway 2.0 transport selection. These conditions
are difficult to create repeatedly in a live account and often cannot be triggered at the exact
point required by a test.

The existing Cosmos DB emulators are useful for general service compatibility, but they do not
provide the fine-grained, runtime topology controls required by SDK routing and resiliency tests.
They also introduce platform, container, startup-time, and certificate dependencies that make
large deterministic test matrices more expensive and less predictable.

A network-accessible host is required so Rust, Java, .NET, Python, and other SDKs can exercise their
real HTTP stacks and wire protocols against the same controllable topology model.

## Decision

Build an open-source, memory-backed Cosmos DB emulator host whose primary purpose is testing SDK
routing, topology refresh, retry, failover, and transport behavior. The host exposes supported
Cosmos DB data-plane and metadata contracts over network ports and exposes an emulator-specific
management API for deterministic topology changes.

Optimize the emulator for topology control, deterministic behavior, fast startup, and useful wire
fidelity. It is not a complete Cosmos DB service implementation and is not intended to become one.
In particular:

- It is an SDK engineering and test tool, not a supported customer product. Because it is public
  and open source, customers may use it, but it provides no service compatibility, durability,
  performance, availability, or support guarantees.
- It stores all state in memory and does not provide persistence or production data safety.
- It implements only the data-plane operations needed to provision fixtures and observe SDK
  behavior under simulated topology conditions.
- Query support is deliberately limited to the subset needed by SDK scenarios. Full SQL query
  semantics, broad query compatibility, and other complex data-plane features are non-goals.
- Unsupported operations or protocol semantics fail explicitly rather than silently approximating
  behavior that could invalidate an SDK test.

## Consequences

Topology and transport fidelity take priority over breadth of service emulation. New capabilities
are added when they enable a concrete SDK test scenario, not to pursue general Cosmos DB feature
parity.

Tests can run quickly and deterministically without Azure resources or heavyweight emulator
infrastructure. Cross-language SDKs can share the same network-visible behavior while retaining
their real client pipelines.

The project must document supported behavior and known divergences clearly so its intentionally
limited scope is not mistaken for customer-grade service emulation.

## Alternatives

- Live Cosmos DB accounts were rejected as the primary test mechanism because they add cost,
  provisioning time, environmental variability, and cannot deterministically expose every
  topology transition or failure at a chosen point in a test.
- Existing Cosmos DB emulators were rejected as the sole mechanism because they do not expose the
  runtime topology controls required for region, replication, split, merge, and failover scenarios,
  and they carry additional platform and deployment dependencies.
- SDK-local mocks were rejected as the shared solution because they bypass real network and wire
  behavior, duplicate semantics in every language, and cannot validate Gateway 2.0 framing or
  cross-SDK compatibility.
- Expanding the project into a full customer-facing Cosmos DB emulator was rejected because it
  would shift effort away from deterministic SDK topology testing and create an impractical service
  compatibility and support commitment.

## References

- Plan & summary: `sdk/cosmos/azure_data_cosmos_emulator/docs/plan.md`
