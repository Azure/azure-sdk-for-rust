# ADR-0014 — Separate diagnostics collection from emission

**Status:** Accepted
**Date:** 2026-08-31

## Context

Retries, hedges, regional failover, and transport attempts make a Cosmos
operation understandable only from its complete operation history. An emitter
cannot reconstruct discarded attempts after the outcome is known, while
collecting an unbounded record would make retry storms amplify memory and
serialization cost.

Tail sampling is a key diagnostics feature: whether externally visible
diagnostics should be emitted may depend on the completed operation's result,
including its status, latency, retries, regions, and threshold violations.
Those criteria cannot be evaluated until the operation completes, so the
internal evidence must already have been collected.

## Alternatives considered

- **Collect conditionally or according to verbosity.** Rejected because the
  decision to retain evidence would be made before latency, failure, retry, and
  regional behavior are known. Verbosity may control exposure, not collection.
- **Emit telemetry eagerly from the driver.** Rejected because it couples the
  shared engine to exporters and language-specific policy, prevents
  completion-time sampling, and risks duplicate public telemetry from the SDK.

## Decision

The driver always collects and returns one canonical, operation-scoped
`DiagnosticsContext`. It accounts for the full sequence of regions, retries,
hedges, and attempts. Finalization applies bounded compaction: representative
records and exact aggregate counts preserve the operation's shape while
limiting artifact size.

The SDK owns `DiagnosticsHandler`s and all default emission decisions. After
the operation completes, each language SDK evaluates its tail-sampling criteria
against the context and synthesizes the appropriate externally visible
diagnostics, such as OpenTelemetry metrics, spans, or logs. It may emit all,
some, or none of those signals.

## Consequences and exceptions

Diagnostics are available for successes and failures even when no handler is
registered. Collection has a predictable bound, with compaction and omitted
counts visible rather than silent. Feature gates, sampling, thresholds, and
verbosity may change handlers or materialized detail, but must not disable the
canonical collection path. Each language SDK can provide idiomatic telemetry
integration without changing the shared driver's evidence model. Driver debug
logging is permitted; eager public telemetry export is not the default.

## Authoritative references

- [Diagnostics contract](../specs/0018-diagnostics-contract.md)
- [Driver diagnostics context](../../azure_data_cosmos_driver/src/diagnostics/diagnostics_context.rs)
- [Bounded diagnostics compaction](../../azure_data_cosmos_driver/src/diagnostics/compaction.rs)
- [SDK diagnostics handler chain](../../azure_data_cosmos/src/diagnostics/handler.rs)
- PR [#4789](https://github.com/Azure/azure-sdk-for-rust/pull/4789)
