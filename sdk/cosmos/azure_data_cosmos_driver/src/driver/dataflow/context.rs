// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Execution context plumbed through [`PipelineNode::next_page`] calls.

use futures::future::BoxFuture;

use crate::models::{CosmosOperation, CosmosResponse, FeedRange};

use super::request::RequestTarget;

/// Request execution mode for partition routing metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PartitionRoutingRefresh {
    /// Use existing partition routing metadata.
    UseCached,
    /// Force partition routing metadata to be refreshed before executing.
    ForceRefresh,
}

/// Executes leaf request nodes through the existing operation pipeline.
pub(crate) trait RequestExecutor: Send {
    /// Executes a single request node.
    fn execute_request<'a>(
        &'a mut self,
        operation: &'a CosmosOperation,
        target: RequestTarget,
        partition_routing_refresh: PartitionRoutingRefresh,
        continuation: Option<String>,
    ) -> BoxFuture<'a, crate::error::Result<CosmosResponse>>;
}

/// Resolves EPK ranges to their current physical partition key ranges.
///
/// Used by pipeline nodes to recover from partition topology changes (splits)
/// and by the planner to resolve initial query ranges.
/// The `PartitionKeyRangeCache` implements this trait in production.
pub(crate) trait TopologyProvider: Send {
    /// Resolves the physical partitions that currently cover the given EPK range.
    ///
    /// `refresh` controls whether the topology cache is refreshed before resolving:
    /// callers use [`PartitionRoutingRefresh::ForceRefresh`] for split recovery
    /// and [`PartitionRoutingRefresh::UseCached`] for planning.
    ///
    /// Returns partition key range IDs paired with their EPK sub-ranges, ordered
    /// by EPK from smallest to largest.
    fn resolve_ranges<'a>(
        &'a mut self,
        range: &'a FeedRange,
        refresh: PartitionRoutingRefresh,
    ) -> BoxFuture<'a, crate::error::Result<Vec<ResolvedRange>>>;
}

/// A physical partition's EPK sub-range, as resolved from the current topology.
#[derive(Debug, Clone)]
pub(crate) struct ResolvedRange {
    /// The partition key range ID for this physical partition.
    pub partition_key_range_id: String,
    /// The EPK sub-range within this physical partition.
    pub range: FeedRange,
}

/// Context passed through dataflow node execution.
pub(crate) struct PipelineContext<'a> {
    request_executor: &'a mut dyn RequestExecutor,
    topology_provider: Option<&'a mut dyn TopologyProvider>,
}

impl<'a> PipelineContext<'a> {
    /// Creates a new pipeline execution context.
    ///
    /// `topology_provider` is `None` for plans that cannot need topology
    /// resolution (e.g. non-partitioned resource operations). If a node calls
    /// [`resolve_ranges`](Self::resolve_ranges) while it is `None`, an error
    /// is returned.
    pub(crate) fn new(
        request_executor: &'a mut dyn RequestExecutor,
        topology_provider: Option<&'a mut dyn TopologyProvider>,
    ) -> Self {
        Self {
            request_executor,
            topology_provider,
        }
    }

    pub(crate) async fn execute_request(
        &mut self,
        operation: &CosmosOperation,
        target: RequestTarget,
        partition_routing_refresh: PartitionRoutingRefresh,
        continuation: Option<String>,
    ) -> crate::error::Result<CosmosResponse> {
        self.request_executor
            .execute_request(operation, target, partition_routing_refresh, continuation)
            .await
    }

    pub(crate) async fn resolve_ranges(
        &mut self,
        range: &FeedRange,
        refresh: PartitionRoutingRefresh,
    ) -> crate::error::Result<Vec<ResolvedRange>> {
        let provider = self.topology_provider.as_deref_mut().ok_or_else(|| {
            crate::error::Error::builder(crate::error::Kind::Client).with_message("topology resolution requested for a plan that was not given a topology provider").build()
        })?;
        provider.resolve_ranges(range, refresh).await
    }
}
