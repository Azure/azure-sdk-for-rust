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
  driver must transcode documented wire encodings, implement generic query
  operators over projected rows, interpret service-defined protocol envelopes,
  and perform PATCH's bounded client-side read-modify-write implementation.

## Decision

The driver accepts and returns application payloads as bytes. Consuming SDKs
own item serialization, typed public models, and conversion to and from driver
operations. The driver may detect documented encodings and parse
service-defined protocol or control envelopes without acquiring knowledge of
the application's item schema. It may also perform bounded, schema-independent
processing required by driver-owned features:

- transcode between text JSON and Cosmos binary JSON;
- parse query envelopes and projected rows for generic operators such as
  ordering, pagination, and `DISTINCT`; and
- parse and rewrite an item in the isolated PATCH read-modify-write handler.

Protocol-defined types may cross the SDK/driver boundary only when the driver
must interpret them and their semantics are stable under the service wire
contract. Adding such a shared type is an explicit compatibility decision, not
the default.

These processing stages may understand an encoding or generic JSON structure,
but they must not depend on customer field names, generated SDK models, or
application-specific schema semantics. Any new body processing must remain
bounded to a driver-owned feature and be documented in its specification.

## Consequences and exceptions

Language SDKs retain native serialization and the driver remains reusable.
Adapters and some model duplication are intentional. Shared protocol types join
the SDK's long-lived compatibility surface. New body processing requires
explicit design review to confirm that it remains schema-independent; acquiring
application-schema knowledge would require superseding this decision.

## Authoritative references

- [Architecture serialization boundary](../Architecture.md#the-serialization-boundary)
- [Binary encoding specification](../specs/0015-binary-encoding.md)
- [Query engine specification](../specs/0013-query-engine.md)
- [PATCH handler specification](../specs/0017-patch-handler.md)
- [SDK-to-driver cutover specification](../specs/0004-sdk-to-driver-cutover.md)
- [`patch_handler` implementation](../../azure_data_cosmos_driver/src/driver/pipeline/patch_handler.rs)
- Merged PRs `Azure/azure-sdk-for-rust#4005` and
  `Azure/azure-sdk-for-rust#4440`
