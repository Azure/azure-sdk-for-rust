// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

//! Azure Cosmos DB Driver - Core Implementation Layer
//!
//! This crate provides the core transport, routing, and protocol handling for Azure Cosmos DB.
//! It is designed to be reused across multiple language SDKs (Rust, Java, .NET, Python) via
//! the C API wrapper (`azure_data_cosmos_native`).
//!
//! # Support Model
//!
//! This crate has a **public API** but receives **community/GitHub support only** (no 24x7 Microsoft Support).
//! For production Rust applications, use [`azure_data_cosmos`](https://docs.rs/azure_data_cosmos) instead,
//! which provides full Microsoft support.
//!
//! # Schema-Agnostic Design
//!
//! The driver is intentionally ignorant of document/item schemas. Data plane operations accept
//! raw bytes (`&[u8]`) and return buffered responses (`Vec<u8>`). Serialization is handled by
//! the consuming SDK in its native language.

pub mod diagnostics;
pub mod driver;
pub mod models;
pub mod options;
pub(crate) mod system;

// Re-export key types at crate root
pub use diagnostics::{DiagnosticsContext, ExecutionContext, RequestDiagnostics, RequestHandle};
pub use driver::{CosmosDriver, CosmosDriverRuntime, CosmosDriverRuntimeBuilder};
pub use models::{ActivityId, CosmosResult, CosmosStatus, RequestCharge};
pub use options::{DiagnosticsOptions, DiagnosticsVerbosity, DriverOptions};
