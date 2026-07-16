// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-operation diagnostics surfaced by the Cosmos DB SDK.
//!
//! Every fallible Cosmos operation produces a [`DiagnosticsContext`] capturing
//! request tracking, retries, regions contacted, and other observability
//! signals from the request pipeline. The context is reachable from
//! [`CosmosError`](crate::CosmosError) on failure, and from the
//! [`FeedPage`](crate::feed::FeedPage), [`ItemResponse`](crate::models::ItemResponse), and
//! similar response wrappers on success.
//!
//! The SDK also exposes an emission extension point on top of that context: a
//! [`DiagnosticsHandler`] receives each operation's completed
//! [`DiagnosticsContext`], and an ordered [`DiagnosticsHandlerChain`] invokes
//! registered handlers once per operation at completion. Register handlers via
//! [`CosmosClientBuilder::with_diagnostics_handler`](crate::CosmosClientBuilder::with_diagnostics_handler).
//! With no handlers registered the chain is a zero-overhead no-op.

// =========================================================================
// Public API
// =========================================================================

#[doc(inline)]
pub use azure_data_cosmos_driver::diagnostics::{DiagnosticsContext, TransportKind};
pub use handler::{DiagnosticsHandler, DiagnosticsHandlerChain};

// =========================================================================
// Internal modules
// =========================================================================

mod handler;
