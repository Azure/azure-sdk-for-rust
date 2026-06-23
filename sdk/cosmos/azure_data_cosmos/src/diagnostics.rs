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

// =========================================================================
// Public API
// =========================================================================

#[doc(inline)]
pub use azure_data_cosmos_driver::diagnostics::{DiagnosticsContext, TransportKind};
