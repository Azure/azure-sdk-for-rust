// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pipeline node snapshot state used to serialize / deserialize continuation
//! tokens.
//!
//! Each variant captures the information required to reconstruct an
//! equivalent pipeline on resume. [`SequentialDrain`] uses a sparse encoding:
//! a single `left_most_undrained_epk` cursor (anything strictly below is
//! implicitly drained) plus a list of [`RangedToken`] entries for ranges that
//! carry a live server continuation. Ranges at or above the cursor with no
//! matching entry are implicitly fresh-start (`Request { None }`).
//!
//! Size is O(S) where S is the number of partitions with a live continuation
//! token at snapshot time, not O(P) where P is the total partition count.

use serde::{Deserialize, Serialize};

use crate::models::ChangeFeedStartFrom;

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

    /// A sequential drain over EPK-ordered children, encoded sparsely.
    ///
    /// `left_most_undrained_epk` is the inclusive lower EPK bound of the
    /// first range that still has remaining work. Anything strictly below
    /// this cursor has already been drained and must not be re-queried on
    /// resume. An empty string means the cursor is at the start of the
    /// query range (no partitions have been drained yet).
    ///
    /// `active_tokens` carries one entry per range that has an in-flight
    /// server continuation. Ranges at or above the cursor with no matching
    /// entry are implicitly fresh-start (`Request { None }`). The list is
    /// sorted ascending by `min_epk` and non-overlapping; the planner
    /// validates this on resume and rejects malformed tokens.
    SequentialDrain {
        left_most_undrained_epk: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        active_tokens: Vec<RangedToken>,
    },

    /// An unordered merge over all partition children.
    ///
    /// Unlike `SequentialDrain`, every child is kept alive even after
    /// returning 304 (no changes). All children's server continuations are
    /// stored so the feed can be resumed from any checkpoint.
    ///
    /// `active_tokens` carries one entry per child partition that has a
    /// server continuation. Children with no entry are fresh-start on
    /// resume.
    ///
    /// `start_from` records the feed's original start position so partitions
    /// that were never polled before the checkpoint (and thus have no entry in
    /// `active_tokens`) re-apply it on resume instead of reading from the
    /// beginning. `None` means no start position was recorded (e.g. the node
    /// was not built from a change feed operation).
    UnorderedMerge {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        active_tokens: Vec<RangedToken>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        start_from: Option<ChangeFeedStartFrom>,
    },
}

/// One entry in a [`PipelineNodeState::SequentialDrain`] `active_tokens`
/// list.
///
/// `min_epk` and `max_epk` are the EPK bounds of the range that owns the
/// server continuation at the time the snapshot was taken.
/// `server_continuation` is the opaque page token returned by the server.
/// Ranges with no live token are not represented in `active_tokens` —
/// they're implicitly fresh-start.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct RangedToken {
    pub(crate) min_epk: String,
    pub(crate) max_epk: String,
    pub(crate) server_continuation: String,
}

/// A child's snapshot state as visible to its parent.
///
/// Parents (e.g., [`SequentialDrain`](super::drain::SequentialDrain))
/// assemble their own state from each child's contribution without needing
/// to pattern-match on the full [`PipelineNodeState`] enum. Variants the
/// parent doesn't support are surfaced as an error in
/// [`PipelineNodeState::into_child_contribution`] rather than reaching the
/// parent at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ChildSnapshotContribution {
    /// The child has produced all of its pages and contributes no state.
    Drained,

    /// The child still has work to do, optionally with an in-flight server
    /// continuation. `None` means the child hasn't issued its first request
    /// yet (fresh-start on resume).
    Pending { server_continuation: Option<String> },
}

impl PipelineNodeState {
    /// Reduces a child's full snapshot state to the subset a parent needs.
    ///
    /// Returns an error for variants no parent currently supports nesting
    /// (e.g., a nested [`PipelineNodeState::SequentialDrain`]). The
    /// `parent`, `idx`, and `total` arguments are folded into the error
    /// message so callers don't have to format their own.
    pub(crate) fn into_child_contribution(
        self,
        parent: &str,
        idx: usize,
        total: usize,
    ) -> crate::error::Result<ChildSnapshotContribution> {
        match self {
            PipelineNodeState::Drained => Ok(ChildSnapshotContribution::Drained),
            PipelineNodeState::Request {
                server_continuation,
            } => Ok(ChildSnapshotContribution::Pending {
                server_continuation,
            }),
            other => Err(crate::error::CosmosError::builder()
                .with_status(
                    crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_UNEXPECTED_NESTED_SHAPE,
                )
                .with_message(format!(
                    "{parent} child {idx} of {total} produced an unsupported snapshot shape: {}",
                    match &other {
                        PipelineNodeState::Drained => "Drained",
                        PipelineNodeState::Request { .. } => "Request",
                        PipelineNodeState::SequentialDrain { .. } => "SequentialDrain",
                        PipelineNodeState::UnorderedMerge { .. } => "UnorderedMerge",
                    },
                ))
                .build()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(min: &str, max: &str, t: &str) -> RangedToken {
        RangedToken {
            min_epk: min.to_owned(),
            max_epk: max.to_owned(),
            server_continuation: t.to_owned(),
        }
    }

    #[test]
    fn drained_round_trips() {
        let json = serde_json::to_string(&PipelineNodeState::Drained).unwrap();
        assert_eq!(json, r#"{"kind":"drained"}"#);
        let parsed: PipelineNodeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, PipelineNodeState::Drained);
    }

    #[test]
    fn request_round_trips_with_and_without_token() {
        let with = PipelineNodeState::Request {
            server_continuation: Some("abc".to_owned()),
        };
        let json = serde_json::to_string(&with).unwrap();
        assert_eq!(json, r#"{"kind":"request","server_continuation":"abc"}"#);
        let parsed: PipelineNodeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, with);

        let without = PipelineNodeState::Request {
            server_continuation: None,
        };
        let json = serde_json::to_string(&without).unwrap();
        assert_eq!(json, r#"{"kind":"request"}"#);
        let parsed: PipelineNodeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, without);
    }

    #[test]
    fn sequential_drain_round_trips_cursor_only() {
        let state = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: "80".to_owned(),
            active_tokens: vec![],
        };
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(
            json, r#"{"kind":"sequential_drain","left_most_undrained_epk":"80"}"#,
            "empty active_tokens must be omitted from the wire form",
        );
        let parsed: PipelineNodeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }

    #[test]
    fn sequential_drain_round_trips_with_active_tokens() {
        let state = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: String::new(),
            active_tokens: vec![token("00", "55", "t1"), token("55", "AA", "t2")],
        };
        let json = serde_json::to_string(&state).unwrap();
        let parsed: PipelineNodeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }

    #[test]
    fn sequential_drain_deserializes_with_absent_active_tokens_field() {
        // The wire form omits `active_tokens` when empty; deserialization
        // must accept the absent field as an empty list.
        let json = r#"{"kind":"sequential_drain","left_most_undrained_epk":"40"}"#;
        let parsed: PipelineNodeState = serde_json::from_str(json).unwrap();
        assert_eq!(
            parsed,
            PipelineNodeState::SequentialDrain {
                left_most_undrained_epk: "40".to_owned(),
                active_tokens: vec![],
            },
        );
    }

    #[test]
    fn legacy_lossy_sequential_drain_shape_fails_to_deserialize() {
        // The prior `SequentialDrain` wire shape — `{current_min_epk,
        // current_max_epk, left_most}` — only carried the front
        // child's state, so tokens minted in that shape cannot
        // faithfully resume a mid-fan-out drain. Reject them on
        // deserialize so callers see a clear failure instead of a
        // partial resume; the CHANGELOG documents this as a breaking
        // change.
        let legacy = r#"{"kind":"sequential_drain","current_min_epk":"00","current_max_epk":"FF","left_most":{"kind":"request","server_continuation":"tok"}}"#;
        let result: Result<PipelineNodeState, _> = serde_json::from_str(legacy);
        assert!(
            result.is_err(),
            "legacy SequentialDrain shape must fail to deserialize under the new schema"
        );
    }

    #[test]
    fn unknown_kind_fails_to_deserialize() {
        let bogus = r#"{"kind":"something_new"}"#;
        let result: Result<PipelineNodeState, _> = serde_json::from_str(bogus);
        assert!(result.is_err());
    }

    #[test]
    fn into_child_contribution_maps_drained() {
        let contrib = PipelineNodeState::Drained
            .into_child_contribution("Parent", 0, 1)
            .expect("Drained must always reduce successfully");
        assert_eq!(contrib, ChildSnapshotContribution::Drained);
    }

    #[test]
    fn into_child_contribution_maps_request_with_token() {
        let contrib = PipelineNodeState::Request {
            server_continuation: Some("tok".to_owned()),
        }
        .into_child_contribution("Parent", 0, 1)
        .expect("Request must always reduce successfully");
        assert_eq!(
            contrib,
            ChildSnapshotContribution::Pending {
                server_continuation: Some("tok".to_owned()),
            }
        );
    }

    #[test]
    fn into_child_contribution_maps_request_without_token() {
        let contrib = PipelineNodeState::Request {
            server_continuation: None,
        }
        .into_child_contribution("Parent", 1, 4)
        .expect("Request must always reduce successfully");
        assert_eq!(
            contrib,
            ChildSnapshotContribution::Pending {
                server_continuation: None,
            }
        );
    }

    #[test]
    fn into_child_contribution_rejects_nested_sequential_drain() {
        let err = PipelineNodeState::SequentialDrain {
            left_most_undrained_epk: "80".to_owned(),
            active_tokens: vec![],
        }
        .into_child_contribution("Parent", 2, 5)
        .expect_err("nested SequentialDrain is not a supported child shape");
        let msg = format!("{err:?}");
        assert!(
            msg.contains("Parent child 2 of 5"),
            "error should carry parent/idx/total context: {msg}"
        );
        assert!(
            msg.contains("SequentialDrain"),
            "error should name the offending variant: {msg}"
        );
    }

    #[test]
    fn into_child_contribution_rejects_nested_unordered_merge() {
        let err = PipelineNodeState::UnorderedMerge {
            active_tokens: vec![],
            start_from: None,
        }
        .into_child_contribution("Parent", 0, 1)
        .expect_err("nested UnorderedMerge is not a supported child shape");
        let msg = format!("{err:?}");
        assert!(
            msg.contains("UnorderedMerge"),
            "error should name the offending variant: {msg}"
        );
    }

    #[test]
    fn unordered_merge_round_trips_empty() {
        let state = PipelineNodeState::UnorderedMerge {
            active_tokens: vec![],
            start_from: None,
        };
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(
            json, r#"{"kind":"unordered_merge"}"#,
            "empty active_tokens must be omitted from the wire form",
        );
        let parsed: PipelineNodeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }

    #[test]
    fn unordered_merge_round_trips_with_tokens() {
        let state = PipelineNodeState::UnorderedMerge {
            active_tokens: vec![token("00", "55", "t1"), token("55", "AA", "t2")],
            start_from: None,
        };
        let json = serde_json::to_string(&state).unwrap();
        let parsed: PipelineNodeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }

    #[test]
    fn unordered_merge_round_trips_with_start_from() {
        let now = PipelineNodeState::UnorderedMerge {
            active_tokens: vec![],
            start_from: Some(ChangeFeedStartFrom::Now),
        };
        let json = serde_json::to_string(&now).unwrap();
        assert_eq!(
            json, r#"{"kind":"unordered_merge","start_from":{"kind":"now"}}"#,
            "start_from must be persisted so never-polled partitions honor it on resume",
        );
        assert_eq!(
            serde_json::from_str::<PipelineNodeState>(&json).unwrap(),
            now
        );

        let point_in_time = PipelineNodeState::UnorderedMerge {
            active_tokens: vec![token("00", "FF", "t1")],
            start_from: Some(ChangeFeedStartFrom::PointInTime(time::macros::datetime!(
                2015-10-21 07:28:00 UTC
            ))),
        };
        let json = serde_json::to_string(&point_in_time).unwrap();
        assert_eq!(
            serde_json::from_str::<PipelineNodeState>(&json).unwrap(),
            point_in_time
        );
    }
}
