// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pipeline node snapshot state used to serialize / deserialize continuation
//! tokens.
//!
//! Each variant captures only the information required to reconstruct an
//! equivalent pipeline on resume. In particular, [`SequentialDrain`] preserves
//! its left-most child plus the active child's original EPK bounds; the
//! planner reconstructs the remaining (yet-to-drain) children from the
//! operation's query ranges and the current topology.

use serde::{Deserialize, Serialize};

/// Serializable snapshot of a [`PipelineNode`](super::PipelineNode) subtree.
///
/// The shape is intentionally open to future intermediate node kinds so a
/// parent does not need to know what type its child is — every node produces
/// a `PipelineNodeState` from `snapshot_state()`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum PipelineNodeState {
    /// The node has produced all of its pages.
    Drained,

    /// A leaf request node.
    ///
    /// `server_continuation` is the opaque page token returned by the server
    /// for the next page, or `None` when no request has yet been issued.
    Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        server_continuation: Option<String>,
    },

    /// A sequential drain over EPK-ordered children.
    ///
    /// Only the left-most (currently-active) child's snapshot is preserved.
    /// `current_min_epk` / `current_max_epk` are the original bounds of the
    /// active child when the snapshot was taken. The planner uses
    /// `current_min_epk` to skip already-drained ranges, and `current_max_epk`
    /// to recover correct continuation behavior if that child's partition has
    /// merged before resume.
    SequentialDrain {
        current_min_epk: String,
        current_max_epk: String,
        left_most: Box<PipelineNodeState>,
    },
}
