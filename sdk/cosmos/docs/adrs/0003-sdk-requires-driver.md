# ADR-0003 — Require driver execution with no fallback

**Status:** Accepted
**Date:** 2026-08-31

## Context

The SDK once had its own gateway execution path while the driver was introduced
as a second engine. Keeping both paths would make correctness depend on which
engine happened to run and could conceal driver failures.

## Alternatives considered

- **Make the driver optional and fall back at runtime.** Rejected because
  absence, initialization failure, or execution failure could silently select a
  path with different retry, option, response, and diagnostics behavior.
- **Retain SDK-owned execution for selected operations.** Rejected because two
  permanent engines require duplicated fixes and indefinite parity testing.

## Decision

The driver is mandatory infrastructure for the Rust SDK. Every SDK network
operation delegates execution to a required `CosmosDriver`. If driver creation
or execution fails, the operation fails; the SDK does not retry through a
legacy gateway pipeline or any other execution fallback.

## Consequences and exceptions

Production and tests always exercise the same engine, and driver defects remain
visible. SDK operations must translate their public inputs and outputs at the
driver boundary, and adding an SDK operation requires a corresponding driver
execution path.

## Authoritative references

- [Architecture SDK layer](../Architecture.md#layers)
- [SDK-to-driver cutover specification](../specs/0004-sdk-to-driver-cutover.md)
- [`ClientContext` required driver field](../../azure_data_cosmos/src/clients/mod.rs)
- Merged PRs `Azure/azure-sdk-for-rust#4005` and
  `Azure/azure-sdk-for-rust#4053`
