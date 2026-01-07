// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::pipeline::CosmosPipeline;
use std::sync::Arc;
use azure_core::http::{Context, Response};
use crate::cosmos_request::CosmosRequest;
use crate::routing::partition_key_range_cache::PartitionKeyRangeCache;

/// Handler for managing transport-level operations with Cosmos DB.
#[derive(Debug, Clone)]
pub struct TransportHandler {
    pipeline: Arc<CosmosPipeline>,
    pk_range_cache: Arc<PartitionKeyRangeCache>,
}

impl TransportHandler {
    /// Creates a new `TransportHandler` with the specified pipeline.
    ///
    /// # Arguments
    ///
    /// * `pipeline` - The Cosmos pipeline to use for sending requests.
    pub(crate) fn new(pipeline: Arc<CosmosPipeline>, pk_range_cache: Arc<PartitionKeyRangeCache>) -> Self {
        Self {
            pipeline,
            pk_range_cache
        }
    }

    pub async fn send<T>(
        &self,
        cosmos_request: CosmosRequest,
        context: Context<'_>,
    ) -> azure_core::Result<Response<T>> {
        self.pipeline.send(cosmos_request, context).await
    }
}
