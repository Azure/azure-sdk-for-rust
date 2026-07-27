// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`CosmosClientOptions`] — options for [`CosmosClient`](crate::CosmosClient) construction.

use std::sync::Arc;

use azure_data_cosmos_driver::options::{OperationOptions, UserAgentSuffix};

use crate::diagnostics::{DiagnosticsHandler, DiagnosticsHandlerChain};

/// Options used when creating a [`CosmosClient`](crate::CosmosClient).
///
/// This struct is used internally by [`CosmosClientBuilder`](crate::CosmosClientBuilder).
/// Use the builder pattern via [`CosmosClient::builder()`](crate::CosmosClient::builder())
/// to configure client options.
#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct CosmosClientOptions {
    /// Default [`OperationOptions`] applied to all requests made by this client,
    /// unless overridden by per-request options.
    pub operation: OperationOptions,
    pub(crate) user_agent_suffix: Option<UserAgentSuffix>,
    /// Diagnostics emission handlers invoked once per operation at completion.
    pub(crate) diagnostics_handlers: DiagnosticsHandlerChain,
}

impl CosmosClientOptions {
    pub fn with_user_agent_suffix(mut self, suffix: UserAgentSuffix) -> Self {
        self.user_agent_suffix = Some(suffix);
        self
    }

    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }

    /// Registers a [`DiagnosticsHandler`](crate::diagnostics::DiagnosticsHandler)
    /// invoked once per operation at completion.
    ///
    /// Handlers run in registration order. Call this multiple times to build an
    /// ordered chain.
    pub fn with_diagnostics_handler(mut self, handler: Arc<dyn DiagnosticsHandler>) -> Self {
        self.diagnostics_handlers = self.diagnostics_handlers.with_handler(handler);
        self
    }
}
