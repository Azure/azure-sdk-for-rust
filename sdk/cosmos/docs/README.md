# Azure Cosmos DB SDK for Rust Documentation

This directory contains durable project, architecture, design, and decision
documentation for the Azure Cosmos DB SDK for Rust.

## Start here

- [Project overview](Project.md) explains the project's purpose and boundaries.
- [Architecture overview](Architecture.md) explains the SDK, driver, execution
  pipelines, and shared state at a high level.
- [Cosmos Rust code style](CodeStyle.md) collects concise implementation
  conventions after the architecture is understood.

## Directory layout

```text
docs/
├── Project.md
├── Architecture.md
├── CodeStyle.md
├── README.md
├── AGENTS.md
├── specs/       # Numbered, mutable feature specifications
├── adrs/        # Numbered, finalized architecture decisions
└── reports/     # Unnumbered historical investigations and measurements
```

Project and architecture are the top-level design overviews. `CodeStyle.md`
contains implementation conventions. See [`specs/`](specs/) for detailed
feature designs, [`adrs/`](adrs/) for finalized decisions, and
[`reports/`](reports/) for archival technical reports.

Crate READMEs, changelogs, contributor guidance, and Markdown fragments consumed
directly by Rust source remain with their crates.

## Specifications

| Number | Specification |
| --- | --- |
| 0001 | [Configuration options](specs/0001-configuration-options.md) |
| 0002 | [Hierarchical configuration model](specs/0002-hierarchical-configuration-model.md) |
| 0003 | [Response metadata](specs/0003-response-metadata.md) |
| 0004 | [SDK-to-driver cutover](specs/0004-sdk-to-driver-cutover.md) |
| 0005 | [Operation and transport pipelines](specs/0005-operation-and-transport-pipelines.md) |
| 0006 | [Error codes and retries](specs/0006-error-codes-and-retries.md) |
| 0007 | [Partition key range cache](specs/0007-partition-key-range-cache.md) |
| 0008 | [Partition-level failover](specs/0008-partition-level-failover.md) |
| 0009 | [Cross-region hedging](specs/0009-cross-region-hedging.md) |
| 0010 | [Hub-region processing header](specs/0010-hub-region-processing-header.md) |
| 0011 | [Gateway V2](specs/0011-gateway-v2.md) |
| 0012 | [Feed operations and dataflow](specs/0012-feed-operations-and-dataflow.md) |
| 0013 | [Query engine](specs/0013-query-engine.md) |
| 0014 | [Binary encoding high-level design](specs/0014-binary-encoding-high-level-design.md) |
| 0015 | [Binary encoding](specs/0015-binary-encoding.md) |
| 0016 | [Binary encoding roundtrip fuzzer](specs/0016-binary-encoding-roundtrip-fuzzer.md) |
| 0017 | [Patch handler](specs/0017-patch-handler.md) |
| 0018 | [Diagnostics contract](specs/0018-diagnostics-contract.md) |
| 0019 | [Native wrapper](specs/0019-native-wrapper.md) |
| 0020 | [Native async invocation](specs/0020-native-async-invocation.md) |
| 0021 | [In-memory emulator](specs/0021-in-memory-emulator.md) |
| 0022 | [Distributed transactions](specs/0022-distributed-transactions.md) |
| 0023 | [Emulator transport security and authentication](specs/0023-emulator-transport-security-and-authentication.md) |
| 0024 | [Fault injection](specs/0024-fault-injection.md) |
| 0025 | [Throughput control](specs/0025-throughput-control.md) |
| 0026 | [Session consistency](specs/0026-session-consistency.md) |
| 0027 | [Hosted emulator](specs/0027-hosted-emulator.md) |

## Architecture decision records

| Number | Decision |
| --- | --- |
| 0001 | [SDK, driver, and native layering](adrs/0001-sdk-driver-native-layering.md) |
| 0002 | [Schema-agnostic driver boundary](adrs/0002-schema-agnostic-driver-boundary.md) |
| 0003 | [SDK requires the driver](adrs/0003-sdk-requires-driver.md) |
| 0004 | [Three-tier execution pipeline](adrs/0004-three-tier-execution-pipeline.md) |
| 0005 | [Flat native ABI data model](adrs/0005-flat-native-abi-data-model.md) |
| 0006 | [Internal HTTP transport](adrs/0006-internal-http-transport.md) |
| 0007 | [Cargo feature categories](adrs/0007-cargo-feature-categories.md) |
| 0008 | [Layered operation configuration](adrs/0008-layered-operation-configuration.md) |
| 0009 | [Environment variables are options](adrs/0009-environment-variables-are-options.md) |
| 0010 | [Metadata resolution and client construction](adrs/0010-metadata-resolution-and-client-construction.md) |
| 0011 | [Status/SubStatus error taxonomy](adrs/0011-status-substatus-error-taxonomy.md) |
| 0012 | [Regional endpoint routing](adrs/0012-regional-endpoint-routing.md) |
| 0013 | [Basic control-plane operations](adrs/0013-basic-control-plane-operations.md) |
| 0014 | [Diagnostics collection and emission](adrs/0014-diagnostics-collection-and-emission.md) |

## Reports

- [Binary encoding `u64::MAX` analysis](reports/binary-encoding-u64-max-analysis.md)
- [PPCB memory analysis](reports/ppcb-memory-analysis.md)
