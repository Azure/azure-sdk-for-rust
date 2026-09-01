# ADR-0002 — Keep the driver boundary schema-agnostic

**Status:** Accepted
**Date:** 2026-08-31

## Context

Cosmos DB item schemas belong to applications, and each language SDK must use
its native serialization model. The shared driver therefore cannot make Rust
types or `serde` behavior part of the cross-language execution contract.

## Alternatives considered

- **Make driver operations generic over typed items.** Rejected because typed
  serialization cannot cross the C ABI and would privilege Rust's data model.
- **Share all models through a common crate or SDK re-exports.** Rejected
  because it locks the supported SDK to the driver's faster version cadence.
- **Forbid every form of body parsing in the driver.** Rejected because the
  driver must interpret service-defined protocol envelopes, and PATCH requires
  a bounded client-side read-modify-write implementation.

## Decision

The driver accepts and returns application payloads as bytes. Consuming SDKs
own item serialization, typed public models, and conversion to and from driver
operations. The driver may detect documented encodings and parse
service-defined protocol or control envelopes without acquiring knowledge of
the application's item schema.

Protocol-defined types may cross the SDK/driver boundary only when the driver
must interpret them and their semantics are stable under the service wire
contract. Adding such a shared type is an explicit compatibility decision, not
the default.

PATCH is the narrow application-body exception. Its driver-side RMW handler may
parse and rewrite an item only to implement the protocol in
[specification 0017](../specs/0017-patch-handler.md); that permission does not
extend to ordinary operations or general driver stages.

## Consequences and exceptions

Language SDKs retain native serialization and the driver remains reusable.
Adapters and some model duplication are intentional. Shared protocol types join
the SDK's long-lived compatibility surface, and any new application-body parser
requires a separately documented architectural exception.

## Authoritative references

- [Architecture serialization boundary](../Architecture.md#the-serialization-boundary)
- [PATCH handler specification](../specs/0017-patch-handler.md)
- [SDK-to-driver cutover specification](../specs/0004-sdk-to-driver-cutover.md)
- [`patch_handler` implementation](../../azure_data_cosmos_driver/src/driver/pipeline/patch_handler.rs)
- Merged PRs `Azure/azure-sdk-for-rust#4005` and
  `Azure/azure-sdk-for-rust#4440`
