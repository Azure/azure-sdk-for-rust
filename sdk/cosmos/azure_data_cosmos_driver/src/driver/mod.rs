// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB driver runtime and driver singleton management.
//!
//! This module provides the core driver infrastructure for managing connections
//! to Azure Cosmos DB accounts:
//!
//! - [`CosmosDriverRuntime`] - The global runtime environment shared across drivers
//! - [`CosmosDriverRuntimeBuilder`] - Builder for creating runtime instances
//! - [`CosmosDriver`] - A driver instance for a specific Cosmos DB account

pub(crate) mod cache;
mod cosmos_driver;
mod runtime;
pub(crate) mod transport;

pub use cosmos_driver::CosmosDriver;
pub use runtime::{CosmosDriverRuntime, CosmosDriverRuntimeBuilder};
