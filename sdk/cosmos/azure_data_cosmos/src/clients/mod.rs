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

use crate::diagnostics::{DiagnosticsContext, DiagnosticsHandlerChain};
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
    pub(crate) fn complete_operation(
        &self,
        driver_response: azure_data_cosmos_driver::models::CosmosResponse,
    ) -> CosmosResponse {
        let response = crate::driver_bridge::driver_response_to_cosmos_response(driver_response);
        let diagnostics = response.diagnostics();
        self.dispatch_diagnostics(&diagnostics);
        response
    }

    /// Invokes the registered diagnostics handlers with a completed context.
    ///
    /// Zero-overhead no-op when no handlers are registered: the trace
    /// [`Context`](azure_core::http::Context) is only constructed when at least
    /// one handler will observe it.
    pub(crate) fn dispatch_diagnostics(&self, diagnostics: &DiagnosticsContext) {
        if self.diagnostics_handlers.is_empty() {
            return;
        }
        let cx = azure_core::http::Context::new();
        self.diagnostics_handlers.dispatch(diagnostics, &cx);
    }
}
