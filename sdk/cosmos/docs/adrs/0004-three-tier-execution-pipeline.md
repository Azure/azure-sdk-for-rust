# ADR-0004 — Use Dataflow, Operation, and Transport tiers

**Status:** Accepted
**Date:** 2026-08-31

## Context

Cosmos execution has nested scopes: feed planning spans pages and partitions,
logical operations span regions and attempts, and transport work targets one
endpoint. Retry decisions depend on Cosmos status/sub-status, request-sent
state, sessions, topology, and regional availability.

## Alternatives considered

- **Use generic `azure_core` retry policies or an extensible ordered policy
  chain.** Rejected because a linear chain cannot reliably express the nested
  Cosmos retry scopes, topology effects, and ordering invariants.
- **Use one Cosmos retry loop for every concern.** Rejected because local
  throttling/connectivity retries, cross-region failover, and feed repair have
  different state and budgets.
- **Let each SDK assemble its own policies.** Rejected because behavior would
  diverge across languages and bypass the shared driver's purpose.

## Decision

Execution flows in one direction: **Dataflow -> Operation -> Transport**.

- **Dataflow** plans and advances paged, partitioned work and continuation
  state, invoking the operation tier for each service request.
- **Operation** owns one logical request across routing choices, hedges,
  failover, sessions, and operation-level diagnostics.
- **Transport** owns an attempt against one endpoint, including signing,
  common headers, local throttling/connectivity retries, deadline enforcement,
  and attempt diagnostics.

Cosmos retry classification, budgets, and effects belong to the Cosmos driver.
`azure_core` supplies transport and runtime abstractions, not the retry
architecture or a customer-extensible execution-policy chain.

## Consequences and exceptions

Retry scopes and ownership are explicit and shared by every consuming SDK. The
separation naturally produces typed stage state and independently testable
transitions; those are implementation consequences, not a fixed component
inventory or another tier. Explicitly documented bootstrap or maintenance
flows may enter below Dataflow, but their retries remain driver-owned.

## Authoritative references

- [Architecture execution pipelines](../Architecture.md#execution-pipelines)
- [Operation and transport pipelines specification](../specs/0005-operation-and-transport-pipelines.md)
- [Feed operations and dataflow specification](../specs/0012-feed-operations-and-dataflow.md)
- [`operation_pipeline` implementation](../../azure_data_cosmos_driver/src/driver/pipeline/operation_pipeline.rs)
- [`transport_pipeline` implementation](../../azure_data_cosmos_driver/src/driver/transport/transport_pipeline.rs)
- Merged PRs `Azure/azure-sdk-for-rust#3829`,
  `Azure/azure-sdk-for-rust#3875`, and `Azure/azure-sdk-for-rust#4440`
