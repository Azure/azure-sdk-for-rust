// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`PipelineNode`] trait and [`PageResult`] returned from each pull.

use async_trait::async_trait;

use crate::models::{CosmosResponse, FeedRange};

use super::{context::PipelineContext, snapshot::PipelineNodeState};

/// Result of a single `next_page` call on a pipeline node.
///
/// The `Page` variant contains a large `CosmosResponse` inline, but boxing it
/// would add a heap allocation on every page fetch — the hot path. The `SplitRequired`
/// variant is rare (only on partition splits), so the size difference is acceptable.
#[must_use = "a PageResult carries the next page, drain signal, or a split request that the caller must act on"]
#[allow(clippy::large_enum_variant)]
pub(crate) enum PageResult {
    /// A page of results was produced.
    ///
    /// `is_terminal` is `true` when this node has no more pages to emit
    /// after this one — set by leaf nodes when the server returned no
    /// continuation token, and propagated by intermediate nodes when their
    /// last child has emitted its terminal page. Parents use this to evict
    /// drained children eagerly so that snapshots of the pipeline do not
    /// include children that are already done.
    Page {
        response: CosmosResponse,
        is_terminal: bool,
    },
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
            PageResult::Page { is_terminal, .. } => {
                write!(f, "Page(terminal={is_terminal})")
            }
            PageResult::Drained => f.write_str("Drained"),
            PageResult::SplitRequired {
                replacement_nodes, ..
            } => write!(f, "SplitRequired({} nodes)", replacement_nodes.len()),
        }
    }
}

/// A dataflow node that emits pages and may own child nodes.
///
/// Each `next_page` call boxes a future via `async_trait`; the per-page
/// allocation is negligible compared to the multi-millisecond network I/O
/// of a Cosmos DB request.
#[async_trait]
pub(crate) trait PipelineNode: Send + std::any::Any {
    /// Emits the next page of results, signals drain completion, or requests a split.
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> azure_core::Result<PageResult>;

    /// Consumes this node and returns its children as a `Vec`.
    ///
    /// Used by tests to inspect the dataflow tree's shape after planning.
    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>>;

    /// Snapshots this node's state for continuation-token serialization.
    fn snapshot_state(&self) -> PipelineNodeState;

    /// Returns `true` if it's possible for this node to require a topology change (split or merge) in the future.
    ///
    /// A node where `topology_can_change()` is true cannot be the root of the pipeline because there is no parent to perform the necessary split/merge replacement if a topology change occurs.
    fn topology_can_change(&self) -> bool;

    /// Returns the EPK range this node currently targets, if known.
    ///
    /// Used by intermediate nodes (e.g. [`super::SequentialDrain`]) to record
    /// the current cursor position when snapshotting, without needing to know
    /// the concrete type of their children. Defaults to `None`.
    ///
    /// # Invariant
    ///
    /// Every node in the dataflow tree is responsible for some contiguous EPK
    /// sub-range of the container key space. Intermediate nodes that drain
    /// children in EPK order (such as [`super::SequentialDrain`]) may use the
    /// front child's `feed_range()` as their own cursor; intermediates that
    /// combine results across ranges (e.g. a future k-way merge for streaming
    /// `ORDER BY`) are responsible for snapshotting whatever cursor
    /// representation makes sense for their ordering semantics.
    fn feed_range(&self) -> Option<&FeedRange> {
        None
    }
}

#[cfg(test)]
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
