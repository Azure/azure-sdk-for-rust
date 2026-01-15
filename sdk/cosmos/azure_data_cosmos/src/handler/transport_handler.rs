// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![allow(dead_code)]

use crate::cosmos_request::CosmosRequest;
use crate::pipeline::CosmosPipeline;
use crate::routing::container_cache::ContainerCache;
use crate::routing::partition_key_range_cache::PartitionKeyRangeCache;
use azure_core::http::{Context, Response};
use std::sync::Arc;

/// Handler for managing transport-level operations with Cosmos DB.
#[derive(Debug, Clone)]
pub struct TransportHandler {
    pipeline: Arc<CosmosPipeline>,
    container_cache: Arc<ContainerCache>,
    pk_range_cache: Arc<PartitionKeyRangeCache>,
}

impl TransportHandler {
    /// Creates a new `TransportHandler` with the specified pipeline.
    ///
    /// # Arguments
    ///
    /// * `pipeline` - The Cosmos pipeline to use for sending requests.
    pub(crate) fn new(
        pipeline: Arc<CosmosPipeline>,
        container_cache: Arc<ContainerCache>,
        pk_range_cache: Arc<PartitionKeyRangeCache>,
    ) -> Self {
        Self {
            pipeline,
            container_cache,
            pk_range_cache,
        }
    }

    pub async fn send<T>(
        &self,
        cosmos_request: CosmosRequest,
        context: Context<'_>,
    ) -> azure_core::Result<Response<T>> {
        let container_prop = self
            .container_cache
            .resolve_by_id("sdk_rust_container".parse()?, None, false)
            .await?;
        let _pk_range = self
            .pk_range_cache
            .resolve_partition_key_range_by_id(&container_prop.id, "0".as_ref(), false)
            .await;
        self.pipeline.send(cosmos_request, context).await
    }
}
