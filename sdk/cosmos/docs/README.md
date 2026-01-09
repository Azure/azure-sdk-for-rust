# Overview of the Cosmos DB SDK and Native Driver

The Cosmos DB SDK is designed in a unique way, in order to be able to provide both a high-level, ergonomic Rust interface for developers, as well as a low-level native driver that can be used across FFI boundaries in other languages.

## Azure Cosmos DB Native Driver

Cosmos DB clients are complex. The client is responsible for managing connections to backend replicas, seeking quorum, handling cross-regional failover and retry, hedging requests, and much more. Implementing all of this logic in every language SDK already leads to significant duplication of effort, and makes it difficult to ensure consistency across SDKs. The Native Driver seeks to solve this problem by providing a single, shared implementation of the core Cosmos DB client logic that can be used across multiple language SDKs.

The Cosmos DB Native Driver is implemented in Rust, and exposes two FFI-compatible interfaces:

1. A low-level C-compatible interface, which can be used directly from C and C++ applications, or via FFI bindings in other languages.
2. A Rust-native interface, designed to be used by the higher-level Azure SDK for Rust Cosmos DB client.

The Native Driver can be found in the `azure_data_cosmos_driver` crate.

There is further documentation on the design and reasoning behind the Cosmos DB Native Driver in the [driver design documentation](./driver/README.md).

## Azure Cosmos DB SDK for Rust

The Azure Cosmos DB SDK for Rust is built on top of the Native Driver, and provides a high-level, ergonomic interface for developers to interact with Cosmos DB. It handles serialization and deserialization of data, provides convenient abstractions for common operations, and integrates with the rest of the Azure SDK for Rust ecosystem.

The SDK can be found in the `azure_data_cosmos` crate.

## Prototype Native SDK Wrapper

A prototype wrapper around the Rust SDK is available in the `azure_data_cosmos_native` crate. This crate exposes a C-compatible interface to the Rust SDK, allowing it to be used from other languages via FFI. This is intended as a prototype for the eventual full-featured native driver, and is being transitioned into the `azure_data_cosmos_driver` crate over time.