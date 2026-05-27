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
pub(crate) mod dataflow;
pub(crate) mod jitter;
pub(crate) mod pipeline;
pub(crate) mod routing;
mod runtime;
pub(crate) mod transport;

pub use cosmos_driver::CosmosDriver;
pub use dataflow::OperationPlan;
pub use runtime::{CosmosDriverRuntime, CosmosDriverRuntimeBuilder};

/// Walks an error's `.source()` chain and joins all distinct messages into a
/// single colon-separated string. Duplicate consecutive messages (common when
/// error wrappers repeat the inner message) are collapsed.
///
/// Accepts any `std::error::Error` so callers can pass any error type
/// (typed `crate::error::CosmosError`, transport-layer errors, etc.) without
/// conversion.
pub(crate) fn error_chain_summary(error: &(dyn std::error::Error + 'static)) -> String {
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
    use crate::error::CosmosError;
    use crate::models::CosmosStatus;
    use std::error::Error as StdError;
    use std::sync::Arc;

    #[test]
    fn returns_top_level_display_when_no_source() {
        // No source chain → the summary is exactly the error's own
        // `Display` string (`[Kind] status: message`).
        let error = CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message("top-level failure")
            .build();
        assert_eq!(error_chain_summary(&error), "400: top-level failure");
    }

    #[test]
    fn joins_chain_with_colon_separator() {
        // Outer transport error wrapping a stdlib `io::Error` as source.
        // The summary is the outer `Display` joined with each subsequent
        // source's `Display` by `": "`.
        let inner_io = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "socket reset");
        let error = CosmosError::builder()
            .with_status(crate::error::CosmosStatus::TRANSPORT_GENERATED_503)
            .with_status(CosmosStatus::TRANSPORT_IO_FAILED)
            .with_message("outer transport failure")
            .with_source(inner_io)
            .build();
        assert_eq!(
            error_chain_summary(&error),
            "503/20011: outer transport failure: socket reset"
        );
    }

    #[test]
    fn collapses_consecutive_duplicate_messages() {
        // Two equivalent client errors render to byte-identical `Display`
        // strings — the dedup collapses them so the summary is the single
        // `Display` string, not duplicated.
        let inner: Arc<dyn StdError + Send + Sync + 'static> = Arc::new(
            CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("duplicate")
                .build(),
        );
        let outer = CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message("duplicate")
            .with_arc_source(Arc::clone(&inner))
            .build();
        assert_eq!(error_chain_summary(&outer), "400: duplicate");
    }
}
