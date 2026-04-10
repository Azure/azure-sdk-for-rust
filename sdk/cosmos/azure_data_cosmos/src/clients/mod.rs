// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Clients used to communicate with Azure Cosmos DB

mod container_client;
mod cosmos_client;
mod cosmos_client_builder;
mod database_client;
pub(crate) mod offers_client;
mod throughput_poller;

use std::sync::Arc;

use crate::{
    pipeline::GatewayPipeline,
    routing::{
        global_endpoint_manager::GlobalEndpointManager,
        global_partition_endpoint_manager::GlobalPartitionEndpointManager,
    },
};
use azure_data_cosmos_driver::CosmosDriver;

/// Shared infrastructure threaded from [`CosmosClient`](super::CosmosClient)
/// through [`DatabaseClient`](super::DatabaseClient) to
/// [`ContainerClient`](super::ContainerClient).
///
/// Bundling these fields avoids passing them individually through every
/// constructor in the client hierarchy.
#[derive(Clone, Debug)]
pub(crate) struct ClientContext {
    pub(crate) pipeline: Arc<GatewayPipeline>,
    pub(crate) driver: Arc<CosmosDriver>,
    pub(crate) global_endpoint_manager: Arc<GlobalEndpointManager>,
    pub(crate) global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
}

pub use container_client::ContainerClient;
pub use cosmos_client::CosmosClient;
pub use cosmos_client_builder::CosmosClientBuilder;
pub use database_client::DatabaseClient;
pub use throughput_poller::ThroughputPoller;
