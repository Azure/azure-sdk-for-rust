// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! A trivial leaf node that immediately reports `Drained`.
//!
//! Used when reconstructing a pipeline from a continuation token whose
//! [`PipelineNodeState::Drained`](super::PipelineNodeState) snapshot indicates
//! the operation already completed. Allows the SDK iterator to behave
//! uniformly without the planner having to special-case the "already done"
//! state.

use async_trait::async_trait;

use super::{PageResult, PipelineContext, PipelineNode, PipelineNodeState};

pub(crate) struct DrainedLeaf;

#[async_trait]
impl PipelineNode for DrainedLeaf {
    async fn next_page(
        &mut self,
        _context: &mut PipelineContext<'_>,
    ) -> azure_core::Result<PageResult> {
        Ok(PageResult::Drained)
    }

    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>> {
        Vec::new()
    }

    fn snapshot_state(&self) -> PipelineNodeState {
        PipelineNodeState::Drained
    }
}
