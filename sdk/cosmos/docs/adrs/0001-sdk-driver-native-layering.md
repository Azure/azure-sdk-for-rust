# ADR-0001 — Separate SDK, driver, and native layers

**Status:** Accepted
**Date:** 2026-08-31

## Context

The Rust SDK needs a long-lived, idiomatic public API, while Cosmos protocol,
routing, retry, and diagnostics code must evolve faster and be reusable by
other language SDKs. A C ABI also has different compatibility and support
requirements from either Rust crate.

## Alternatives considered

- **One crate for SDK, execution engine, and FFI.** Rejected because it couples
  the supported SDK's compatibility promise to faster-moving engine and ABI
  work.
- **A native wrapper over the typed Rust SDK.** Rejected because it would carry
  Rust models and serialization across the ABI, forcing host SDKs to parse data
  already interpreted by Rust. The earlier wrapper at that boundary was
  removed.
- **A separate native execution engine.** Rejected because routing, retry,
  failover, and diagnostics would diverge between Rust and other languages.

## Decision

Use three layers with separate boundaries:

- `azure_data_cosmos` is the fully supported Rust SDK. It owns the idiomatic
  API and preserves compatibility for years within a major version.
- `azure_data_cosmos_driver` is the public execution engine. It has
  community/GitHub support and semantic versioning, but may advance major
  versions faster than the SDK.
- `azure_data_cosmos_driver_native` is an unpublished `cdylib`/`staticlib`
  exposing a C ABI over the driver. Its compatibility is governed by the ABI
  contract, not crates.io versioning.

The Rust SDK calls the driver directly; the native layer wraps the driver,
never the typed SDK.

## Consequences and exceptions

The SDK absorbs driver changes through adapters, while non-Rust SDKs reuse the
same engine. This adds conversion code and three release boundaries. Any
cross-layer type sharing is limited by
[ADR-0002](0002-schema-agnostic-driver-boundary.md); it does not collapse the
support or versioning boundaries above.

## Authoritative references

- [Project support boundaries](../Project.md#product-and-support-boundaries)
- [Architecture layers](../Architecture.md#layers)
- [`azure_data_cosmos_driver_native` package contract](../../azure_data_cosmos_driver_native/Cargo.toml)
- Merged PRs `Azure/azure-sdk-for-rust#3772`,
  `Azure/azure-sdk-for-rust#4103`, and `Azure/azure-sdk-for-rust#4515`
- [Native wrapper specification](../specs/0019-native-wrapper.md)
