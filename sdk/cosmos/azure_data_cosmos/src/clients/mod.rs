// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Clients used to communicate with Azure Cosmos DB

// =========================================================================
// Public API
// =========================================================================

pub use container_client::ContainerClient;
pub use cosmos_client::CosmosClient;
pub use cosmos_client_builder::CosmosClientBuilder;
pub use database_client::DatabaseClient;
pub use throughput_poller::ThroughputPoller;

// =========================================================================
// Internal modules
// =========================================================================

mod container_client;
mod cosmos_client;
mod cosmos_client_builder;
mod database_client;
pub(crate) mod offers_client;
mod throughput_poller;

// =========================================================================
// Crate-internal types
// =========================================================================

use std::sync::Arc;

use azure_data_cosmos_driver::CosmosDriver;

/// Shared infrastructure threaded from [`CosmosClient`](super::CosmosClient)
/// through [`DatabaseClient`](super::DatabaseClient) to
/// [`ContainerClient`](super::ContainerClient).
///
/// Bundling these fields avoids passing them individually through every
/// constructor in the client hierarchy.
#[derive(Clone, Debug)]
pub(crate) struct ClientContext {
    pub(crate) driver: Arc<CosmosDriver>,
    pub(crate) binary_encoding: BinaryEncoding,
}

/// Whether the client uses Cosmos binary JSON on the wire.
///
/// Governs two things together so they cannot drift apart: encoding item write
/// bodies as binary, and advertising that the client accepts binary responses
/// via the `x-ms-cosmos-supported-serialization-formats` negotiation header.
///
/// Resolved **once** at client construction from the
/// `AZURE_COSMOS_BINARY_ENCODING_ENABLED` environment variable and disabled
/// unless it is set to a truthy value (`1` / `true` / `yes` / `on`,
/// case-insensitive, trimmed). This is the single source of truth for both
/// directions. Binary encoding is in preview, so enablement is env-only for
/// now; a public client/driver builder option (defaulting from the same
/// variable) layers on when it ships.
#[derive(Clone, Copy, Debug)]
pub(crate) struct BinaryEncoding {
    enabled: bool,
}

impl BinaryEncoding {
    /// Resolves enablement from `AZURE_COSMOS_BINARY_ENCODING_ENABLED`.
    pub(crate) fn from_env() -> Self {
        Self {
            enabled: env_flag_enabled("AZURE_COSMOS_BINARY_ENCODING_ENABLED"),
        }
    }

    /// Returns whether binary encoding (and response negotiation) is enabled.
    pub(crate) fn enabled(self) -> bool {
        self.enabled
    }

    /// Test-only constructor for a fixed enablement state, so tests don't depend
    /// on (or race on) the process environment.
    #[cfg(test)]
    pub(crate) fn for_test(enabled: bool) -> Self {
        Self { enabled }
    }
}

/// Returns `true` if the named environment variable is set to a truthy value
/// (`1` / `true` / `yes` / `on`, case-insensitive and trimmed).
fn env_flag_enabled(name: &str) -> bool {
    std::env::var(name)
        .map(|v| flag_value_is_truthy(&v))
        .unwrap_or(false)
}

/// Returns `true` if `value` is one of the accepted truthy spellings
/// (`1` / `true` / `yes` / `on`), case-insensitive and trimmed.
fn flag_value_is_truthy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truthy_flag_values_are_accepted() {
        for v in ["1", "true", "TRUE", "Yes", "on", " On ", "\ttrue\n"] {
            assert!(flag_value_is_truthy(v), "{v:?} should be truthy");
        }
    }

    #[test]
    fn non_truthy_flag_values_are_rejected() {
        for v in ["", "0", "false", "no", "off", "2", "enabled", "y"] {
            assert!(!flag_value_is_truthy(v), "{v:?} should not be truthy");
        }
    }
}
