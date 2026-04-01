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

pub mod cache;
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

#[cfg(test)]
mod tests {
    use super::error_chain_summary;

    #[test]
    fn error_chain_summary_single_error() {
        let error = azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "top-level failure",
        );
        assert_eq!(error_chain_summary(&error), "top-level failure");
    }

    #[test]
    fn error_chain_summary_with_source_chain() {
        let inner = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "socket reset");
        let error = azure_core::Error::with_error(
            azure_core::error::ErrorKind::Io,
            inner,
            "reqwest transport failed",
        );
        let summary = error_chain_summary(&error);
        assert!(summary.contains("reqwest transport failed"));
        assert!(summary.contains("socket reset"));
    }

    #[test]
    fn error_chain_summary_deduplicates_consecutive_messages() {
        // When a wrapper repeats the inner message, only one copy should appear.
        let inner = azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "connection refused",
        );
        // Wrap with the same message text.
        let outer = azure_core::Error::with_error(
            azure_core::error::ErrorKind::Connection,
            inner,
            "connection refused",
        );
        let summary = error_chain_summary(&outer);
        // "connection refused" should appear only once, not "connection refused: connection refused".
        assert_eq!(summary, "connection refused");
    }
}
