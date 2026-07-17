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
#[cfg(feature = "preview_dtx")]
pub use distributed_transaction::{
    DistributedReadTransaction, DistributedTransactionOperationOptions,
    DistributedTransactionOperationResult, DistributedTransactionPatchOperationOptions,
    DistributedTransactionResponse, DistributedWriteTransaction,
};
pub use throughput_poller::ThroughputPoller;

// =========================================================================
// Internal modules
// =========================================================================

mod container_client;
mod cosmos_client;
mod cosmos_client_builder;
mod database_client;
#[cfg(feature = "preview_dtx")]
pub(crate) mod distributed_transaction;
pub(crate) mod offers_client;
mod throughput_poller;

// =========================================================================
// Crate-internal types
// =========================================================================

use std::sync::Arc;

use azure_data_cosmos_driver::CosmosDriver;

use crate::options::BinaryEncodingOptions;

/// Shared infrastructure threaded from [`CosmosClient`](super::CosmosClient)
/// through [`DatabaseClient`](super::DatabaseClient) to
/// [`ContainerClient`](super::ContainerClient).
///
/// Bundling these fields avoids passing them individually through every
/// constructor in the client hierarchy.
#[derive(Clone, Debug)]
pub(crate) struct ClientContext {
    pub(crate) driver: Arc<CosmosDriver>,
    pub(crate) binary_encoding: BinaryEncodingOptions,
}

/// The environment variable that enables Cosmos binary JSON encoding when no
/// explicit client option is supplied.
pub(crate) const BINARY_ENCODING_ENV_VAR: &str = "AZURE_COSMOS_BINARY_ENCODING_ENABLED";

/// Resolves the client's [`BinaryEncodingOptions`] from an explicit client
/// option, falling back to the environment when the option is unset.
///
/// Resolution happens **once** at client construction. An explicit client
/// option (see
/// [`CosmosClientBuilder::with_binary_encoding_options`](crate::CosmosClientBuilder::with_binary_encoding_options))
/// takes precedence; when the caller leaves it unset (`None`), enablement falls
/// back to the [`BINARY_ENCODING_ENV_VAR`] environment variable (truthy values
/// `1` / `true` / `yes` / `on`, case-insensitive, trimmed). The resolved
/// options are the single source of truth for both encoding item bodies and
/// advertising binary-response negotiation. Binary encoding is in preview.
pub(crate) fn resolve_binary_encoding(
    explicit: Option<BinaryEncodingOptions>,
) -> BinaryEncodingOptions {
    resolve_binary_encoding_with(explicit, |name| std::env::var(name).ok())
}

/// Resolves [`BinaryEncodingOptions`] using an injected environment reader.
///
/// The `get_env` closure mirrors the driver's config tooling so callers (and
/// tests) can supply environment values without touching the real process
/// environment via `std::env::set_var` (which is `unsafe` in recent `std`).
fn resolve_binary_encoding_with(
    explicit: Option<BinaryEncodingOptions>,
    get_env: impl Fn(&str) -> Option<String>,
) -> BinaryEncodingOptions {
    explicit.unwrap_or_else(|| {
        let enabled = get_env(BINARY_ENCODING_ENV_VAR)
            .as_deref()
            .map(flag_value_is_truthy)
            .unwrap_or(false);
        BinaryEncodingOptions::new().with_enabled(enabled)
    })
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

    #[test]
    fn explicit_options_win_over_environment() {
        // An explicit disabled option must beat a truthy environment value, and
        // vice versa — the injected env reader is never consulted when the
        // caller supplied options.
        let disabled = resolve_binary_encoding_with(
            Some(BinaryEncodingOptions::new().with_enabled(false)),
            |_| Some("true".to_owned()),
        );
        assert!(!disabled.enabled);

        let enabled = resolve_binary_encoding_with(
            Some(BinaryEncodingOptions::new().with_enabled(true)),
            |_| Some("false".to_owned()),
        );
        assert!(enabled.enabled);
    }

    #[test]
    fn environment_enables_when_option_unset() {
        // With no explicit option, a truthy injected env value enables binary
        // encoding — exercised without touching the real process environment.
        let resolved = resolve_binary_encoding_with(None, |name| {
            assert_eq!(name, BINARY_ENCODING_ENV_VAR);
            Some("on".to_owned())
        });
        assert!(resolved.enabled);
    }

    #[test]
    fn environment_disabled_by_default_when_unset() {
        // No explicit option and no env value ⇒ disabled.
        let resolved = resolve_binary_encoding_with(None, |_| None);
        assert!(!resolved.enabled);

        // A non-truthy env value also resolves to disabled.
        let resolved = resolve_binary_encoding_with(None, |_| Some("nope".to_owned()));
        assert!(!resolved.enabled);
    }
}
