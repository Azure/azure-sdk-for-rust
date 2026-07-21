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

use crate::diagnostics::{CosmosOperationContext, DiagnosticsContext, DiagnosticsHandlerChain};
use crate::models::CosmosResponse;

/// Shared infrastructure threaded from [`CosmosClient`](super::CosmosClient)
/// through [`DatabaseClient`](super::DatabaseClient) to
/// [`ContainerClient`](super::ContainerClient).
///
/// Bundling these fields avoids passing them individually through every
/// constructor in the client hierarchy.
#[derive(Clone, Debug)]
pub(crate) struct ClientContext {
    pub(crate) driver: Arc<CosmosDriver>,
    /// Diagnostics emission handlers invoked once per operation at completion.
    ///
    /// Empty by default, in which case the completion path is a zero-overhead
    /// no-op.
    pub(crate) diagnostics_handlers: DiagnosticsHandlerChain,
}

impl ClientContext {
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
    /// only materialized when at least one handler is registered, preserving the
    /// zero-overhead no-op when the chain is empty.
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
