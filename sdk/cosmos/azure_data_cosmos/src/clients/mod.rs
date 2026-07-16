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

/// Shared infrastructure threaded from [`CosmosClient`](super::CosmosClient)
/// through [`DatabaseClient`](super::DatabaseClient) to
/// [`ContainerClient`](super::ContainerClient).
///
/// Bundling these fields avoids passing them individually through every
/// constructor in the client hierarchy.
#[derive(Clone, Debug)]
pub(crate) struct ClientContext {
    pub(crate) driver: Arc<CosmosDriver>,
}
