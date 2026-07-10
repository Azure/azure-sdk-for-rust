# ADR-002 — Match the .NET v3 JSON wire contract exactly

**Status:** Accepted
**Date:** 2026-07-06

## Context

A single backend DTC coordinator serves .NET, Java, and Rust clients. Divergence in
body shape, header names, or status promotion would fork server behavior per client.

## Decision

Mirror the .NET v3 contract (PR #6002) byte-for-byte: the `operations[]` body layout,
the three `x-ms-cosmos-*` headers, JSON-only encoding, and the
`condition`-inside-`resourceBody` shape for conditional patch.

## Consequences

Full interop with the shared coordinator; the .NET wire-contract doc is directly
usable as the Rust conformance oracle. No HybridRow/binary path.

## Alternatives

A Rust-native envelope with a translation layer was rejected because it adds surface
with no benefit and creates a permanent drift risk.

## References

- Distributed Transactions spec: ../spec.md
- [.NET DTX REST wire contract](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/6002)
