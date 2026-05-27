// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`Pipeline`] (driver-internal) and [`OperationPlan`] (driver-public).

use std::sync::Arc;

use crate::models::{ContinuationToken, CosmosOperation, CosmosResponse};

use super::context::PipelineContext;
use super::node::{PageResult, PipelineNode};
use super::snapshot::PipelineNodeState;

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
        // Validate that the root isn't a node type that can be affected by topology changes, since the pipeline has no parent to handle them if they occur.
        debug_assert!(
            !root.topology_can_change(),
            "pipeline root cannot be a node type that can be affected by topology changes that require splitting or merging"
        );
        Self { root }
    }

    /// Returns a reference to the root node.
    #[cfg(test)]
    pub(crate) fn root(&self) -> &dyn PipelineNode {
        &*self.root
    }

    /// Consumes the pipeline and returns the root node.
    #[cfg(test)]
    pub(crate) fn into_root(self) -> Box<dyn PipelineNode> {
        self.root
    }

    /// Emits the next page from the root node.
    ///
    /// Returns `Ok(Some(response))` for a page, `Ok(None)` when drained.
    pub(crate) async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<Option<CosmosResponse>> {
        match self.root.next_page(context).await? {
            PageResult::Page { response, .. } => Ok(Some(response)),
            PageResult::Drained => Ok(None),
            // Defensive: today the root is always a `Request`, `SequentialDrain`,
            // or `DrainedLeaf`, none of which can bubble `SplitRequired` up past
            // their parent. If a future node type ever does, surfacing it as an
            // explicit error is preferable to silently dropping the page.
            PageResult::SplitRequired { .. } => Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT)
                .with_message(
                    "root node cannot request a split; splits must be handled by a parent node",
                )
                .build()),
        }
    }

    /// Snapshots the pipeline's current state for continuation-token serialization.
    pub(crate) fn snapshot_state(&self) -> PipelineNodeState {
        self.root.snapshot_state()
    }
}

/// A plan for executing a Cosmos DB operation.
///
/// Produced by [`CosmosDriver::plan_operation`](crate::driver::CosmosDriver::plan_operation).
pub struct OperationPlan {
    pub(crate) pipeline: Pipeline,
    operation: Arc<CosmosOperation>,
}

impl OperationPlan {
    /// Creates an operation plan wrapping the given pipeline.
    pub(crate) fn new(pipeline: Pipeline, operation: Arc<CosmosOperation>) -> Self {
        Self {
            pipeline,
            operation,
        }
    }

    /// Snapshots this plan into a [`ContinuationToken`] suitable for cross-process
    /// resumption.
    ///
    /// Snapshotting walks the pipeline tree and serializes a minimal record of
    /// each node's progress. The result can be passed back to
    /// [`CosmosDriver::plan_operation`](crate::driver::CosmosDriver::plan_operation)
    /// (with the same operation) to resume where this plan left off.
    pub fn to_continuation_token(&self) -> crate::error::Result<ContinuationToken> {
        ContinuationToken::encode_v1(&self.operation, &self.pipeline.snapshot_state())
    }
}
