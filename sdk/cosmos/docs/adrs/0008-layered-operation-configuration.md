# ADR-0008 — Resolve operation configuration in layers

**Status:** Accepted
**Date:** 2026-08-31

## Context

Cosmos behavior needs per-operation overrides, per-account policy, shared
runtime defaults, and deploy-time settings. Independent configuration paths
would duplicate fields and make precedence depend on the feature being used.

## Alternatives considered

- **Give each feature its own configuration and precedence path.** Rejected
  because options would drift and interacting features could resolve the same
  concern differently.
- **Use mutable process-global configuration.** Rejected because concurrent
  clients and in-flight operations would observe timing-dependent changes.
- **Copy fields and builders into every option type.** Rejected because the
  boilerplate invites inconsistent API shape and resolution.

## Decision

Resolve ordinary values from highest to lowest priority:
**operation -> account -> runtime -> environment**. Option-group definitions
drive generated views, builders, environment loading, and field resolution so
the hierarchy is uniform. Execution consumes resolved snapshots rather than
mutable shared option objects.

Where a real option is explicitly declared overridable, its generated
environment override layer is separate and precedes the ordinary hierarchy.

## Consequences and exceptions

One option has one resolution contract across the SDK and driver, with less
hand-written boilerplate. Some groups legitimately omit layers where their
scope cannot apply, such as connection-pool settings at operation scope.
Feature-specific mechanics may consume resolved values, but may not establish
an independent precedence chain.

## Authoritative references

- [Configuration options specification](../specs/0001-configuration-options.md)
- [Hierarchical configuration model](../specs/0002-hierarchical-configuration-model.md)
- [`CosmosOptions` macro contract](../../azure_data_cosmos_macros/src/lib.rs)
- [Generated layered views](../../azure_data_cosmos_macros/src/view.rs)
- [Generated option builders](../../azure_data_cosmos_macros/src/builder.rs)
- Merged PRs `Azure/azure-sdk-for-rust#3778`,
  `Azure/azure-sdk-for-rust#3803`, and `Azure/azure-sdk-for-rust#3744`
