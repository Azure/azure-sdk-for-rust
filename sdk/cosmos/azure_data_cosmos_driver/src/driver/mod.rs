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
// cspell:ignore splitmix

pub(crate) mod cache;
mod cosmos_driver;
pub(crate) mod jitter;
pub(crate) mod pipeline;
pub(crate) mod routing;
mod runtime;
pub(crate) mod transport;

pub use cosmos_driver::CosmosDriver;
pub use runtime::{CosmosDriverRuntime, CosmosDriverRuntimeBuilder};

/// Walks an error's `.source()` chain and joins all distinct messages into a
/// single colon-separated string. Duplicate consecutive messages (common when
/// error wrappers repeat the inner message) are collapsed.
pub(crate) fn error_chain_summary(error: &azure_core::Error) -> String {
    use std::error::Error as _;
    let mut parts = vec![error.to_string()];
    let mut source = error.source();
    while let Some(cause) = source {
        let cause_str = cause.to_string();
        if parts.last() != Some(&cause_str) {
            parts.push(cause_str);
        }
        source = cause.source();
    }
    parts.join(": ")
}
