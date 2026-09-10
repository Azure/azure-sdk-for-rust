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
#[cfg(feature = "control_plane")]
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
#[cfg(feature = "control_plane")]
pub(crate) mod offers_client;
#[cfg(feature = "control_plane")]
mod throughput_poller;

// =========================================================================
// Crate-internal types
// =========================================================================

use std::sync::Arc;

use azure_data_cosmos_driver::CosmosDriver;

use crate::diagnostics::{
    ClientLifetimeToken, CosmosClientInfo, CosmosOperationContext, DiagnosticsContext,
    DiagnosticsHandlerChain,
};
use crate::models::CosmosResponse;
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
    /// Diagnostics emission handlers invoked once per operation at completion.
    ///
    /// Empty by default, in which case the completion path does nothing beyond
    /// checking whether a handler is present.
    pub(crate) diagnostics_handlers: DiagnosticsHandlerChain,
    /// Lifetime tokens handed back by the handlers when this client was built.
    ///
    /// Never read; held solely so the tokens' `Drop` runs when the last client
    /// derived from this context goes away. Because the context is cloned down
    /// into every `DatabaseClient`/`ContainerClient`, the shared `Arc` keeps the
    /// tokens alive for as long as *any* of those clients is reachable, which is
    /// the lifetime handlers are meant to observe.
    _client_tokens: Arc<[ClientLifetimeToken]>,
}

impl ClientContext {
    /// Builds the shared context for a newly constructed
    /// [`CosmosClient`](super::CosmosClient), notifying every registered handler
    /// that a client came online and taking ownership of the lifetime tokens
    /// they hand back.
    pub(crate) fn new(
        driver: Arc<CosmosDriver>,
        binary_encoding: BinaryEncodingOptions,
        diagnostics_handlers: DiagnosticsHandlerChain,
        client_info: &CosmosClientInfo,
    ) -> Self {
        let client_tokens = diagnostics_handlers.dispatch_client_created(client_info);
        Self {
            driver,
            binary_encoding,
            diagnostics_handlers,
            _client_tokens: client_tokens,
        }
    }

    /// Converts a completed driver response into the SDK
    /// [`CosmosResponse`](crate::models::CosmosResponse) and invokes the
    /// diagnostics handler chain for the operation.
    ///
    /// This is the per-operation completion seam for the singleton
    /// (non-paginated) data- and control-plane operations: the handler chain
    /// observes the operation's finalized [`DiagnosticsContext`] exactly once.
    ///
    /// `make_op_context` supplies the SDK-side operation identity
    /// ([`CosmosOperationContext`]) — operation name, database, container — that
    /// the driver context does not carry. It is a closure so the identity is
    /// only materialized when at least one handler is registered, so the
    /// completion path does no extra work when the chain is empty.
    pub(crate) fn complete_operation(
        &self,
        driver_response: azure_data_cosmos_driver::models::CosmosResponse,
        make_op_context: impl FnOnce() -> CosmosOperationContext,
    ) -> CosmosResponse {
        let response = crate::driver_bridge::driver_response_to_cosmos_response(driver_response);
        // Guard the diagnostics `Arc` clone behind the emptiness check so the
        // default (no-handler) path clones nothing.
        if !self.diagnostics_handlers.is_empty() {
            let diagnostics = response.diagnostics();
            self.dispatch_diagnostics(&diagnostics, make_op_context);
        }
        response
    }

    /// Result-aware completion seam for singleton operations.
    ///
    /// Dispatches the handler chain exactly once for **both** outcomes: on
    /// success from the bridged response's finalized [`DiagnosticsContext`], and
    /// on failure from the context the driver attaches to the returned error
    /// ([`crate::Error::diagnostics`]). The singleton call sites propagate the
    /// error with `?` *after* this seam, so without it the failure-triggered
    /// tracing and sampled logging would never run.
    ///
    /// Zero-overhead no-op when no handler is registered: neither the error's
    /// diagnostics nor the operation context is materialized.
    pub(crate) fn complete_result<E>(
        &self,
        driver_result: Result<azure_data_cosmos_driver::models::CosmosResponse, E>,
        make_op_context: impl FnOnce() -> CosmosOperationContext,
    ) -> crate::Result<CosmosResponse>
    where
        crate::CosmosError: From<E>,
    {
        match driver_result {
            Ok(driver_response) => Ok(self.complete_operation(driver_response, make_op_context)),
            Err(err) => {
                let err = crate::CosmosError::from(err);
                if !self.diagnostics_handlers.is_empty() {
                    if let Some(diagnostics) = err.diagnostics() {
                        self.dispatch_diagnostics(&diagnostics, make_op_context);
                    }
                }
                Err(err)
            }
        }
    }

    /// Invokes the registered diagnostics handlers with a completed context and
    /// the SDK-supplied operation identity.
    ///
    /// Zero-overhead no-op when no handlers are registered: neither the trace
    /// [`Context`](azure_core::http::Context) nor the
    /// [`CosmosOperationContext`] is constructed unless at least one handler
    /// will observe them.
    pub(crate) fn dispatch_diagnostics(
        &self,
        diagnostics: &DiagnosticsContext,
        make_op_context: impl FnOnce() -> CosmosOperationContext,
    ) {
        if self.diagnostics_handlers.is_empty() {
            return;
        }
        let cx = azure_core::http::Context::new().with_value(make_op_context());
        self.diagnostics_handlers.dispatch(diagnostics, &cx);
    }

    /// Result-aware failure completion seam for call sites that do not funnel
    /// through [`complete_result`](Self::complete_result) — e.g. the offers
    /// helpers, whose success path bridges the response itself.
    ///
    /// Dispatches the handler chain from the diagnostics the driver attached to
    /// the returned error ([`crate::Error::diagnostics`]), so failure-triggered
    /// tracing and sampled logging still run. Zero-overhead no-op when no handler
    /// is registered: neither the error's diagnostics nor the operation context
    /// is materialized.
    #[cfg(feature = "control_plane")]
    pub(crate) fn dispatch_error(
        &self,
        err: &crate::CosmosError,
        make_op_context: impl FnOnce() -> CosmosOperationContext,
    ) {
        if self.diagnostics_handlers.is_empty() {
            return;
        }
        if let Some(diagnostics) = err.diagnostics() {
            self.dispatch_diagnostics(&diagnostics, make_op_context);
        }
    }
}

/// The environment variable that configures Cosmos binary JSON encoding when
/// no explicit client option is supplied.
pub(crate) const BINARY_ENCODING_ENV_VAR: &str = "AZURE_COSMOS_BINARY_ENCODING_ENABLED";

/// Resolves the client's [`BinaryEncodingOptions`] from an explicit client
/// option, falling back to the environment when the option is unset.
///
/// Resolution happens **once** at client construction. An explicit client
/// option (see
/// [`CosmosClientBuilder::with_binary_encoding_options`](crate::CosmosClientBuilder::with_binary_encoding_options))
/// takes precedence; when the caller leaves it unset (`None`), enablement falls
/// back to the [`BINARY_ENCODING_ENV_VAR`] environment variable and defaults to
/// enabled when the variable is absent. Truthy values are `1` / `true` / `yes`
/// / `on` (case-insensitive, trimmed); any other value disables encoding. The
/// resolved options are the single source of truth for both encoding item
/// bodies and advertising binary-response negotiation. Binary encoding is in
/// preview.
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
            .unwrap_or(true);
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
    fn enabled_by_default_when_option_and_environment_unset() {
        // No explicit option and no env value means binary encoding is enabled.
        let resolved = resolve_binary_encoding_with(None, |_| None);
        assert!(resolved.enabled);
    }

    #[test]
    fn environment_disables_with_non_truthy_value() {
        let resolved = resolve_binary_encoding_with(None, |_| Some("nope".to_owned()));
        assert!(!resolved.enabled);
    }
}
