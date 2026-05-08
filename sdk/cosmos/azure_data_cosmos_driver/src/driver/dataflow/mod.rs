// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Dataflow pipeline nodes for paged Cosmos DB operations.

mod drain;
#[cfg(test)]
pub(crate) mod mocks;
pub(crate) mod planner;
pub(crate) mod query_plan;
mod request;
mod topology;

use std::ops::Index;

use futures::future::BoxFuture;

use crate::models::{CosmosOperation, CosmosResponse, FeedRange};

pub(crate) use drain::SequentialDrain;
pub(crate) use request::{Request, RequestTarget};
pub(crate) use topology::CachedTopologyProvider;

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
    ) -> BoxFuture<'a, azure_core::Result<CosmosResponse>>;
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
    ) -> BoxFuture<'a, azure_core::Result<Vec<ResolvedRange>>>;
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
    topology_provider: &'a mut dyn TopologyProvider,
}

impl<'a> PipelineContext<'a> {
    /// Creates a new pipeline execution context.
    pub(crate) fn new(
        request_executor: &'a mut dyn RequestExecutor,
        topology_provider: &'a mut dyn TopologyProvider,
    ) -> Self {
        Self {
            request_executor,
            topology_provider,
        }
    }

    async fn execute_request(
        &mut self,
        operation: &CosmosOperation,
        target: RequestTarget,
        partition_routing_refresh: PartitionRoutingRefresh,
        continuation: Option<String>,
    ) -> azure_core::Result<CosmosResponse> {
        self.request_executor
            .execute_request(operation, target, partition_routing_refresh, continuation)
            .await
    }

    async fn resolve_ranges(
        &mut self,
        range: &FeedRange,
        refresh: PartitionRoutingRefresh,
    ) -> azure_core::Result<Vec<ResolvedRange>> {
        self.topology_provider.resolve_ranges(range, refresh).await
    }
}

/// Result of a single `next_page` call on a pipeline node.
///
/// The `Page` variant contains a large `CosmosResponse` inline, but boxing it
/// would add a heap allocation on every page fetch — the hot path. The `SplitRequired`
/// variant is rare (only on partition splits), so the size difference is acceptable.
#[must_use = "a PageResult carries the next page, drain signal, or a split request that the caller must act on"]
#[allow(clippy::large_enum_variant)]
pub(crate) enum PageResult {
    /// A page of results was produced.
    Page(CosmosResponse),
    /// This node has no more pages to emit.
    Drained,
    /// This node's EPK range has split and needs to be replaced by new child nodes.
    ///
    /// It is the parent intermediate node's responsibility to splice
    /// `replacement_nodes` into its children list (in place of the child that
    /// emitted this result) and re-attempt draining from the first replacement.
    /// If a node returns `SplitRequired` to a parent that does not handle
    /// splits (e.g. the pipeline root), the operation fails.
    SplitRequired {
        /// New child nodes covering the sub-ranges of the split partition.
        replacement_nodes: Vec<Box<dyn PipelineNode>>,
    },
}

impl std::fmt::Debug for PageResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageResult::Page(_) => f.write_str("Page(...)"),
            PageResult::Drained => f.write_str("Drained"),
            PageResult::SplitRequired {
                replacement_nodes, ..
            } => write!(f, "SplitRequired({} nodes)", replacement_nodes.len()),
        }
    }
}

/// An iterator over child pipeline nodes.
///
/// Used by [`PipelineNode::children`] to expose children for diagnostics
/// without requiring a contiguous slice, which `VecDeque`-backed nodes
/// cannot always provide.
pub(crate) enum ChildNodes<'a> {
    /// No children (leaf nodes).
    None,
    /// Children stored in a contiguous slice (e.g. a `Vec`).
    Slice(&'a [Box<dyn PipelineNode>]),
    /// Children stored in a `VecDeque`, exposed as two contiguous slices.
    Split(&'a [Box<dyn PipelineNode>], &'a [Box<dyn PipelineNode>]),
}

impl<'a> ChildNodes<'a> {
    /// Returns the total number of children.
    pub fn len(&self) -> usize {
        match self {
            ChildNodes::None => 0,
            ChildNodes::Slice(s) => s.len(),
            ChildNodes::Split(a, b) => a.len() + b.len(),
        }
    }
}

impl<'a> Index<usize> for ChildNodes<'a> {
    type Output = Box<dyn PipelineNode>;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            ChildNodes::None => panic!("index out of bounds"),
            ChildNodes::Slice(s) => &s[index],
            ChildNodes::Split(a, b) => {
                if index < a.len() {
                    &a[index]
                } else {
                    &b[index - a.len()]
                }
            }
        }
    }
}

impl<'a> IntoIterator for ChildNodes<'a> {
    type Item = &'a Box<dyn PipelineNode>;
    type IntoIter = std::iter::Chain<
        std::slice::Iter<'a, Box<dyn PipelineNode>>,
        std::slice::Iter<'a, Box<dyn PipelineNode>>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        let empty: &[Box<dyn PipelineNode>] = &[];
        match self {
            ChildNodes::None => empty.iter().chain(empty.iter()),
            ChildNodes::Slice(s) => s.iter().chain(empty.iter()),
            ChildNodes::Split(a, b) => a.iter().chain(b.iter()),
        }
    }
}

/// A dataflow node that emits pages and may own child nodes.
///
/// Each `next_page` call boxes a future via `async_trait`; the per-page
/// allocation is negligible compared to the multi-millisecond network I/O
/// of a Cosmos DB request.
#[async_trait::async_trait]
pub(crate) trait PipelineNode: Send + std::any::Any {
    /// Emits the next page of results, signals drain completion, or requests a split.
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> azure_core::Result<PageResult>;

    /// Returns the node's children for diagnostic inspection.
    fn children(&self) -> ChildNodes<'_>;

    /// Consumes this node and returns its children as a `Vec`.
    fn into_children(self) -> Vec<Box<dyn PipelineNode>>;
}

impl dyn PipelineNode {
    /// Downcasts this node to a concrete type.
    pub(crate) fn downcast_ref<T: PipelineNode>(&self) -> Option<&T> {
        (self as &dyn std::any::Any).downcast_ref::<T>()
    }

    /// Downcasts this node to a concrete type.
    pub(crate) fn downcast<T: PipelineNode>(self: Box<Self>) -> Option<Box<T>> {
        (self as Box<dyn std::any::Any>).downcast::<T>().ok()
    }
}

/// A pipeline root that owns the node tree.
pub(crate) struct Pipeline {
    root: Box<dyn PipelineNode>,
}

impl std::fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pipeline").finish_non_exhaustive()
    }
}

impl Pipeline {
    /// Creates a pipeline from an owned root node.
    pub(crate) fn new(root: Box<dyn PipelineNode>) -> Self {
        Self { root }
    }

    /// Returns a reference to the root node.
    pub(crate) fn root(&self) -> &dyn PipelineNode {
        &*self.root
    }

    /// Consumes the pipeline and returns the root node.
    pub(crate) fn into_root(self) -> Box<dyn PipelineNode> {
        self.root
    }

    /// Emits the next page from the root node.
    ///
    /// Returns `Ok(Some(response))` for a page, `Ok(None)` when drained.
    pub(crate) async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> azure_core::Result<Option<CosmosResponse>> {
        match self.root.next_page(context).await? {
            PageResult::Page(response) => Ok(Some(response)),
            PageResult::Drained => Ok(None),
            // Defensive: today the root is always a `Request`, `SequentialDrain`,
            // or `DrainedLeaf`, none of which can bubble `SplitRequired` up past
            // their parent. If a future node type ever does, surfacing it as an
            // explicit error is preferable to silently dropping the page.
            PageResult::SplitRequired { .. } => Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "root node cannot request a split; splits must be handled by a parent node",
            )),
        }
    }
}

/// An opaque plan for executing a Cosmos DB operation.
///
/// Wraps the internal dataflow [`Pipeline`] to hide its structure from callers.
/// Produced by [`CosmosDriver::plan_operation`](crate::driver::CosmosDriver::plan_operation).
pub struct OperationPlan {
    pub(crate) pipeline: Pipeline,
}

impl OperationPlan {
    /// Creates an operation plan wrapping the given pipeline.
    pub(crate) fn new(pipeline: Pipeline) -> Self {
        Self { pipeline }
    }
}

#[cfg(test)]
mod tests {
    use super::mocks::*;
    use super::*;

    #[tokio::test]
    async fn pipeline_forwards_pages_from_root() {
        let mut pipeline = Pipeline::new(Box::new(MockLeaf::with_pages(vec![Ok(
            PageResult::Page(response(b"page")),
        )])));
        let mut executor = NoopRequestExecutor;
        let mut topology = NoopTopologyProvider;
        let mut context = PipelineContext::new(&mut executor, &mut topology);

        let page = pipeline.next_page(&mut context).await.unwrap().unwrap();

        assert_eq!(page.body(), b"page");
    }
}
