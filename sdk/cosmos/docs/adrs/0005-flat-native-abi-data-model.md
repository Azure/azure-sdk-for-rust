# ADR-0005 — Use flat owned data at the native ABI

**Status:** Accepted
**Date:** 2026-08-31

## Context

An ABI built from per-field builders, getters, and response handles requires
many bindings and FFI calls for values that host languages can represent
directly. At the same time, live Rust state and values with Rust-only invariants
cannot be safely reconstructed from arbitrary host bytes.

## Alternatives considered

- **Use opaque handles for every value.** Rejected because ordinary requests
  and results would require large accessor surfaces and repeated FFI calls.
- **Use incremental native builders and per-field getters.** Rejected because
  every field expands the exported function set and every host binding.
- **Return non-owning views into temporary Rust values.** Rejected because
  asynchronous hosts need explicit, enforceable lifetimes and one clear owner.

## Decision

Values that can safely round-trip as bytes cross the ABI as flat
`#[repr(C)]` records, scalar discriminants, and pointer/length arrays. Inputs
are validated and copied before the call returns. Returned records are owned
units: Rust retains their backing allocations, pointer fields are borrowed from
that backing, and one matching free operation releases the entire unit.

Opaque handles are used only for state that cannot safely round-trip as bytes,
including live runtime, driver, queue, in-flight operation, or resolved
driver-state objects. They are not the default data-transfer mechanism.

This ADR decides the data and ownership model only. Submission, waiting,
cancellation, batching, shutdown, and other completion-queue mechanics remain
specified by [specification 0020](../specs/0020-native-async-invocation.md).

## Consequences and exceptions

Host bindings mirror a small, code-generation-friendly ABI and usually consume
a result with one read and one free. Hosts must copy borrowed fields before
freeing their owner, and ABI evolution must preserve record-layout and
discriminant compatibility. Opaque handles remain valid where byte
reconstruction would lose identity, concurrency, lifetime, or validation
invariants.

## Authoritative references

- [Native wrapper specification](../specs/0019-native-wrapper.md)
- [Native async invocation specification](../specs/0020-native-async-invocation.md)
- [`op_request` flat request model](../../azure_data_cosmos_driver_native/src/op_request.rs)
- [`CosmosCompletion` owned result model](../../azure_data_cosmos_driver_native/src/completion.rs)
- Merged PR `Azure/azure-sdk-for-rust#4515`
