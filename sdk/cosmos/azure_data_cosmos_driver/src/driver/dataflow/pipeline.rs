// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`Pipeline`] (driver-internal) and [`OperationPlan`] (driver-public).

use std::{sync::Arc, time::Instant};

use crate::{
    models::{ContinuationToken, CosmosOperation, CosmosResponse},
    options::PlanOptions,
};

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
            // `Distinct`, or `DrainedLeaf`, none of which can bubble
            // `SplitRequired` up past their parent. If a future node type ever
            // does, surfacing it as an explicit error is preferable to silently
            // dropping the page.
            PageResult::SplitRequired { .. } => Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_ROOT_NODE_CANNOT_REQUEST_SPLIT)
                .with_message(
                    "root node cannot request a split; splits must be handled by a parent node",
                )
                .build()),
        }
    }

    /// Snapshots the pipeline's current state for continuation-token serialization.
    pub(crate) fn snapshot_state(&self) -> crate::error::Result<PipelineNodeState> {
        self.root.snapshot_state()
    }

    /// Returns the number of leaf request nodes the root fans out to.
    ///
    /// Used by the planner to enforce a maximum fan-out on fresh plans.
    pub(crate) fn fan_out_width(&self) -> usize {
        self.root.fan_out_width()
    }
}

/// A plan for executing a Cosmos DB operation.
///
/// Produced by [`CosmosDriver::plan_operation()`](crate::driver::CosmosDriver::plan_operation).
/// Pass it to [`CosmosDriver::execute_plan()`](crate::driver::CosmosDriver::execute_plan)
/// to retrieve pages, then call [`to_continuation_token()`](Self::to_continuation_token)
/// to resume later.
pub struct OperationPlan {
    pub(crate) pipeline: Pipeline,
    pub(crate) operation: Arc<CosmosOperation>,
    pub(crate) plan_options: PlanOptions,
    pub(crate) is_resumed: bool,
    pub(crate) has_progressed: bool,
    pub(crate) container_recreation_recovery_attempted: bool,
    /// Policy-derived deadline shared by planning and the first page only.
    ///
    /// Consumed by the first `execute_plan` call so later pages receive a fresh
    /// per-call budget instead of inheriting an expired planning timestamp.
    initial_execution_deadline: Option<Instant>,
    /// Set when a page advanced every node's resume position but could not be
    /// handed to the caller, so the plan's progress and what the caller
    /// received have diverged. Once set, no continuation token can be minted:
    /// any token would resume *past* the lost page.
    ///
    /// Same pattern as `SkipTake`'s `poisoned` flag, at the boundary layer
    /// rather than inside a node.
    continuation_poisoned: bool,
}

impl OperationPlan {
    /// Creates an operation plan wrapping the given pipeline.
    pub(crate) fn new(
        pipeline: Pipeline,
        operation: Arc<CosmosOperation>,
        plan_options: PlanOptions,
        is_resumed: bool,
    ) -> Self {
        Self {
            pipeline,
            operation,
            plan_options,
            is_resumed,
            has_progressed: false,
            container_recreation_recovery_attempted: false,
            initial_execution_deadline: None,
            continuation_poisoned: false,
        }
    }

    /// Moves a policy-derived planning deadline out of the retained operation.
    pub(crate) fn set_initial_execution_deadline(&mut self, deadline: Instant) {
        self.initial_execution_deadline = Some(deadline);
        self.operation = Arc::new(self.operation.as_ref().clone().with_absolute_deadline(None));
    }

    /// Takes the deadline shared by planning and the first page.
    pub(crate) fn take_initial_execution_deadline(&mut self) -> Option<Instant> {
        self.initial_execution_deadline.take()
    }

    /// Clears deadlines retained by an internal replan after the current
    /// `execute_plan` call has already adopted that budget.
    pub(crate) fn clear_execution_deadlines(&mut self) {
        self.initial_execution_deadline = None;
        self.operation = Arc::new(self.operation.as_ref().clone().with_absolute_deadline(None));
    }

    /// Records that a page advanced the pipeline but never reached the caller.
    pub(crate) fn poison_continuation(&mut self) {
        self.continuation_poisoned = true;
    }

    /// Whether a page advanced this plan without reaching the caller.
    ///
    /// Read by
    /// [`CosmosDriver::execute_plan`](crate::driver::CosmosDriver::execute_plan)
    /// to refuse the *next* page as well. Closing only the token exit would
    /// leave the quieter one open: a caller that keeps pulling pages instead of
    /// minting a token would receive the page *after* the lost one and never
    /// learn a page went missing.
    pub(crate) fn continuation_poisoned(&self) -> bool {
        self.continuation_poisoned
    }

    /// The error a poisoned plan returns from every subsequent call.
    ///
    /// Deliberately carries the status of the failure that caused it — the
    /// poison is not an independent fault, it is that fault made durable. Same
    /// reasoning as `SkipTake::poisoned_error`, at the boundary layer.
    pub(crate) fn poisoned_error() -> crate::error::CosmosError {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message(
                "this plan is unusable: a page advanced it but could not be returned to the \
                 caller, so its progress and what the caller received no longer agree. Continuing \
                 would deliver the page *after* the lost one and no continuation token can be \
                 minted; re-run the query from the last token that was captured successfully",
            )
            .build()
    }

    /// Returns a [`ContinuationToken`] for resuming this plan later.
    ///
    /// Pass the token and the same operation to
    /// [`CosmosDriver::plan_operation()`](crate::driver::CosmosDriver::plan_operation).
    ///
    /// # Errors
    ///
    /// Returns an error for operations other than container queries or change
    /// feeds, if the plan's state cannot be serialized, or if an earlier
    /// failed page left its resume position unusable. If a page advanced the
    /// plan but could not be returned, this returns
    /// [`CLIENT_CONTINUATION_TOKEN_AFTER_TRANSCODE_FAILURE`](crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_AFTER_TRANSCODE_FAILURE).
    /// Re-run the operation from the last token captured successfully.
    pub fn to_continuation_token(&self) -> crate::error::Result<ContinuationToken> {
        if self.continuation_poisoned {
            return Err(crate::error::CosmosError::builder()
                .with_status(
                    crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_AFTER_TRANSCODE_FAILURE,
                )
                .with_message(
                    "a page advanced this plan but could not be returned to the caller, so no \
                     continuation token can be minted: resuming from here would skip that page. \
                     Re-run the query from the last token that was captured successfully",
                )
                .build());
        }
        ContinuationToken::encode_v1(&self.operation, &self.pipeline.snapshot_state()?)
    }

    /// Whether the driver must transcode each page's body back to text before
    /// returning it.
    ///
    /// Fixed when the plan was built, alongside the request header and the
    /// `emit_binary` flag baked into the pipeline nodes, so every page of a
    /// plan agrees on the emitted encoding.
    pub(crate) fn transcodes_response_to_text(&self) -> bool {
        self.operation.transcodes_response_to_text()
    }
}
