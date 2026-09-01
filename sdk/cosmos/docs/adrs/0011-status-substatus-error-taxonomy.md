# ADR-0011 — Use one status and sub-status error taxonomy

**Status:** Accepted
**Date:** 2026-08-31

## Context

Cosmos failures cross the driver, the typed Rust SDK, and the native FFI
boundary. Sub-status values are meaningful only with their HTTP status, while
client, transport, serialization, and FFI failures still need stable,
machine-readable classification even when no service response exists.

## Alternatives considered

- **Independent error-kind enums per layer.** Rejected because every SDK would
  need lossy mappings among driver, SDK, and FFI vocabularies, and the mappings
  would drift as conditions are added.
- **HTTP status alone.** Rejected because one HTTP status covers distinct
  retry, routing, session, and client conditions; it cannot represent the
  service's existing sub-status contract or synthetic failures precisely.

## Decision

Use one `CosmosStatus`: an HTTP status plus an optional `SubStatusCode`, across
the driver, SDK, and native FFI. Service sub-statuses pass through unchanged.
Synthetic client and FFI conditions use reserved sub-status ranges paired with
an appropriate HTTP status rather than defining a parallel public error-kind
taxonomy.

The native ABI packs the pair into 32 bits as
`(http_status << 16) | sub_status`; zero is success and a zero low half means
no sub-status. Messages, response headers, diagnostics, and other rich detail
remain separately accessible through the error payload.

## Consequences and exceptions

Callers classify failures consistently by the status pair, including failures
that never reached the wire. Adding a condition requires allocating and
documenting a sub-status, but not coordinating new enums across layers.
Private mapping helpers are allowed inside an implementation; they must not
become a second exported taxonomy.

## Authoritative references

- [Error codes and retries specification](../specs/0006-error-codes-and-retries.md)
- [Native wrapper error model](../specs/0019-native-wrapper.md#35-error-model)
- [Driver `CosmosStatus` and `SubStatusCode`](../../azure_data_cosmos_driver/src/error/cosmos_status.rs)
- [Native packed status and rich error](../../azure_data_cosmos_driver_native/src/error.rs)
- PRs [#3583](https://github.com/Azure/azure-sdk-for-rust/pull/3583),
  [#4442](https://github.com/Azure/azure-sdk-for-rust/pull/4442), and
  [#4820](https://github.com/Azure/azure-sdk-for-rust/pull/4820)
- Issue [#4696](https://github.com/Azure/azure-sdk-for-rust/issues/4696)
