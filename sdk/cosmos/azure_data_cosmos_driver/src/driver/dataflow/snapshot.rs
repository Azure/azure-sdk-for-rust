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
}
