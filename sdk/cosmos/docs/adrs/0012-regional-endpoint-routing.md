# ADR-0012 — Route steady-state work through regional endpoints

**Status:** Accepted
**Date:** 2026-08-31

## Context

The default account endpoint, such as `foo.documents.azure.com`, is routed
differently by the service from discovered regional endpoints. Treating it as
a universal fallback can send ordinary operations through the wrong routing
path. Some coordinator-scoped operations also have a stricter regional order
than caller preferences express.

## Alternatives considered

- **Fall back globally after regional exhaustion.** Rejected because the
  default endpoint is for account discovery, not a safe steady-state route for
  data-plane or non-account metadata traffic.
- **Use caller preference order for coordinator operations.** Rejected because
  a preferred region can differ from the coordinator's current write region,
  producing incorrect DTX routing during normal operation or failover.

## Decision

Use the default/global endpoint only for account-topology discovery and
bootstrap probing. All steady-state data-plane operations and non-account
metadata operations use discovered regional endpoints, including their
last-resort fallback.

Configured backup endpoints are a bootstrap exception: they are tried when the
initial global endpoint is unavailable, so the driver can discover topology.
They do not become steady-state data-plane fallbacks.

Coordinator-scoped operations, including DTX commits and read snapshots, use
writable endpoints in account-metadata order. Caller preference ordering must
not reorder that coordinator list.

## Consequences and exceptions

Regional routing remains consistent through exclusion, unavailability,
failover, and failback. Account discovery may probe the default, backup, or
already-discovered regional endpoints because its purpose is to rebuild that
regional topology. Endpoint failback may restore a region only after the
configured connectivity-probe policy accepts it.

## Authoritative references

- [Architecture: routing and endpoints](../Architecture.md#shared-state)
- [Distributed transaction coordinator routing](../specs/0022-distributed-transactions.md#31-dtc-coordinator-routing)
- [Account endpoint state](../../azure_data_cosmos_driver/src/driver/routing/account_endpoint_state.rs)
- [Operation endpoint selection](../../azure_data_cosmos_driver/src/driver/pipeline/operation_pipeline.rs)
- [Backup endpoint bootstrap API](../../azure_data_cosmos/src/clients/cosmos_client_builder.rs)
- Issue [#4487](https://github.com/Azure/azure-sdk-for-rust/issues/4487) and
  PRs [#4503](https://github.com/Azure/azure-sdk-for-rust/pull/4503),
  [#4604](https://github.com/Azure/azure-sdk-for-rust/pull/4604), and
  [#4102](https://github.com/Azure/azure-sdk-for-rust/pull/4102)
