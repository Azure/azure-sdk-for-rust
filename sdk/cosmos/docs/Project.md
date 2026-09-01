# Azure Cosmos DB SDK for Rust — Project Overview

## Purpose

This project delivers a supported, idiomatic Rust client for Azure Cosmos DB for
NoSQL, built on top of a reusable execution engine that other language SDKs can
adopt. Two goals drive nearly every design choice:

1. **A first-class Rust experience.** `azure_data_cosmos` follows the
   [Azure SDK Design Guidelines for Rust][guidelines]: typed models, `serde`
   serialization, builder-shaped options, and `azure_core` conventions for
   credentials and async runtimes.
2. **One shared implementation of the hard parts.** Routing, region failover,
   retries, session consistency, connection management, and diagnostics live
   once in `azure_data_cosmos_driver` so they do not have to be reimplemented —
   and re-debugged — per language.

The Rust SDK is a thin, typed layer over that engine. It owns serialization and
the public API shape; the driver owns everything about talking to the service.
The crate split itself is a finalized decision
([adrs/0001-sdk-driver-native-layering.md](adrs/0001-sdk-driver-native-layering.md)).

## Audiences

- **Application developers** who want a supported Cosmos DB client for Rust.
  They use `azure_data_cosmos` only.
- **SDK and driver engineers** working in this repository, who need to know
  which layer owns a behavior before changing it.
- **Other-language SDK teams** (.NET, Java, Go, Python, native C/C++) who
  consume the driver through its C ABI wrapper.
- **Coding agents**, which should read this file and
  [Architecture.md](Architecture.md) for orientation, then the numbered
  [specs](specs/) and [ADRs](adrs/) for detail, and
  [`sdk/cosmos/AGENTS.md`](../AGENTS.md) for the rules that govern code changes.

## Product and support boundaries

Not everything in `sdk/cosmos` is a shipping Azure product, and the differences
matter for versioning and support expectations.

| Scope | Crates | Support and compatibility |
| --- | --- | --- |
| Supported SDK | `azure_data_cosmos` | Full Microsoft support. Strict semantic versioning with multi-year backward compatibility per major version. |
| Public engine | `azure_data_cosmos_driver`, `azure_data_cosmos_macros` | Public APIs with community/GitHub support only. Semantic versioning, but major versions can move faster than the SDK's. |
| Cross-language interop | `azure_data_cosmos_driver_native` | Internal, unpublished. Stability is defined by the C ABI contract, not by crates.io semver. |
| Engineering tools | `azure_data_cosmos_emulator`, `azure_data_cosmos_observability_harness`, `azure_data_cosmos_perf`, `azure_data_cosmos_benchmarks` | Unpublished developer tools. No service compatibility, durability, performance, or support guarantees. |

The emulator in particular is a test double for SDK development, **not** the
Azure Cosmos DB Emulator product, and should never be described as one.

## Crate roles

| Crate | Role |
| --- | --- |
| `azure_data_cosmos` | The supported Rust SDK: clients, typed models, query and feed surfaces, options, diagnostics emission, and feature-gated basic database/container CRUD. |
| `azure_data_cosmos_driver` | Schema-agnostic execution engine: operation model, routing, caches, retries, hedging, transports, query dataflow, diagnostics collection. |
| `azure_data_cosmos_driver_native` | C ABI (`cdylib`/`staticlib`) wrapper exposing the driver to non-Rust SDKs through a completion-queue-style async model. |
| `azure_data_cosmos_macros` | Procedural macros generating the layered configuration boilerplate the SDK and driver options rely on. |
| `azure_data_cosmos_emulator` | Out-of-process host that serves the driver's in-memory emulator over real ports so any client can drive it. |
| `azure_data_cosmos_observability_harness` | Soak/load tool that validates the end-to-end diagnostics and OpenTelemetry experience. |
| `azure_data_cosmos_perf` | CLI for performance and scale runs against real accounts. |
| `azure_data_cosmos_benchmarks` | Micro-benchmarks that isolate driver overhead with an in-memory transport. |

## Design principles

- **Schema-agnostic data plane.** The driver never interprets item payloads; it
  moves opaque bytes. Serialization belongs to each language SDK. See
  [adrs/0002-schema-agnostic-driver-boundary.md](adrs/0002-schema-agnostic-driver-boundary.md),
  and [Architecture.md](Architecture.md) for the one narrow exception (PATCH).
- **No general model sharing across crate boundaries.** Conversions are
  explicit so the driver can evolve without forcing an SDK major version.
  Narrow, stable protocol types may be shared when the boundary ADR explicitly
  permits it.
- **Make feature contracts visible.** Cargo feature names distinguish backend
  selection, capability enablement, preview APIs, unsupported/no-SLA APIs, and
  internal-only surfaces. See
  [adrs/0007-cargo-feature-categories.md](adrs/0007-cargo-feature-categories.md).
- **Data-oriented pipelines.** Execution state is decomposed into focused,
  mostly immutable components transformed by narrow stage functions instead of a
  mutable god-object context. This keeps stages unit-testable in isolation.
- **Layered, resolvable configuration.** Options resolve from operation →
  account → runtime → environment, generated consistently by macros rather than
  duplicated per option struct. See
  [adrs/0008-layered-operation-configuration.md](adrs/0008-layered-operation-configuration.md)
  and
  [adrs/0009-environment-variables-are-options.md](adrs/0009-environment-variables-are-options.md).
- **Diagnostics are always collected; emission is a choice.** The driver
  materializes one canonical diagnostics record per operation; the SDK decides
  what becomes metrics, spans, or logs. See
  [adrs/0014-diagnostics-collection-and-emission.md](adrs/0014-diagnostics-collection-and-emission.md).
- **Keep transport internal.** The driver owns HTTP clients, pooling, and
  Gateway V2 framing so availability and protocol behavior cannot be bypassed
  by an injected pipeline. Build-time backends remain selectable. See
  [adrs/0006-internal-http-transport.md](adrs/0006-internal-http-transport.md).
- **Route through discovered regions.** The default account endpoint discovers
  topology; steady-state requests use regional endpoints. See
  [adrs/0012-regional-endpoint-routing.md](adrs/0012-regional-endpoint-routing.md).
- **Use one machine-readable error taxonomy.** HTTP status and Cosmos
  sub-status classify service, client, transport, and FFI failures consistently.
  See
  [adrs/0011-status-substatus-error-taxonomy.md](adrs/0011-status-substatus-error-taxonomy.md).
- **Keep basic resource CRUD nearby.** Database and container CRUD remain
  available behind the auth-agnostic `control_plane` capability; full
  management belongs to Azure Resource Manager. See
  [adrs/0013-basic-control-plane-operations.md](adrs/0013-basic-control-plane-operations.md).
- **Deterministic testing.** A memory-backed emulator, fault injection, and
  recorded/live test tiers make failover, retry, and outage behavior reproducible
  in CI rather than only observable in production.

## Navigating the documentation

- [docs/README.md](README.md) — layout of this directory and where new documents
  belong.
- [Architecture.md](Architecture.md) — how the crates, pipelines, and shared
  state fit together.
- [specs/](specs/) — numbered, mutable feature specifications (the detailed
  designs).
- [adrs/](adrs/) — numbered, finalized architecture decisions, immutable once
  accepted.
- [reports/](reports/) — historical investigations and measurement results.
- Crate `README.md`, `CHANGELOG.md`, and `CONTRIBUTING.md` files stay with their
  crates; [`sdk/cosmos/AGENTS.md`](../AGENTS.md) carries the coding rules.

[guidelines]: https://azure.github.io/azure-sdk/rust_introduction.html
