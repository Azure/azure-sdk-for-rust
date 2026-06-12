// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pipeline node snapshot state used to serialize / deserialize continuation
//! tokens.
//!
//! Each variant captures the information required to reconstruct an
//! equivalent pipeline on resume. [`SequentialDrain`] preserves the full
//! remaining-work ledger: every still-pending or in-progress child range and
//! its state. The planner uses this list as the authoritative set of work
//! left to do; ranges of the topology that are not represented here have
//! already been drained and must not be re-queried.

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
    /// The `children` list is the authoritative remaining-work ledger: each
    /// entry carries the original EPK range of one not-yet-drained child plus
    /// the snapshot of that child's state. The list must be sorted ascending
    /// by `min_epk` and the ranges must be non-overlapping; the planner
    /// validates this on resume and rejects malformed tokens.
    SequentialDrain { children: Vec<RangedChildState> },
}

/// One entry in a [`PipelineNodeState::SequentialDrain`] children list.
///
/// `min_epk` and `max_epk` are the EPK bounds of the child at the time the
/// snapshot was taken. `state` is the child's own `snapshot_state()` and
/// must currently be a [`PipelineNodeState::Request`] (in-progress or
/// not-yet-started) or [`PipelineNodeState::Drained`]; nested
/// `SequentialDrain` is not supported.
///
/// `Drained` per-entry state is structurally valid but is not produced by
/// the current `snapshot_state` writer (drained front children are popped
/// off the queue before the snapshot, so a sibling can never reach the
/// snapshot already drained). The variant is intentionally accepted so
/// future writers can use it as an explicit "this sub-range completed"
/// watermark without breaking the schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct RangedChildState {
    pub(crate) min_epk: String,
    pub(crate) max_epk: String,
    pub(crate) state: PipelineNodeState,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn child(min: &str, max: &str, token: Option<&str>) -> RangedChildState {
        RangedChildState {
            min_epk: min.to_owned(),
            max_epk: max.to_owned(),
            state: PipelineNodeState::Request {
                server_continuation: token.map(str::to_owned),
            },
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
    fn sequential_drain_round_trips_single_child() {
        let state = PipelineNodeState::SequentialDrain {
            children: vec![child("00", "FF", Some("tok"))],
        };
        let json = serde_json::to_string(&state).unwrap();
        let parsed: PipelineNodeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }

    #[test]
    fn sequential_drain_round_trips_multi_child_including_drained() {
        let state = PipelineNodeState::SequentialDrain {
            children: vec![
                RangedChildState {
                    min_epk: "00".into(),
                    max_epk: "55".into(),
                    state: PipelineNodeState::Drained,
                },
                child("55", "AA", Some("tok-2")),
                child("AA", "FF", None),
            ],
        };
        let json = serde_json::to_string(&state).unwrap();
        let parsed: PipelineNodeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }

    #[test]
    fn sequential_drain_round_trips_empty_children() {
        // Empty is structurally valid (the snapshot path emits Drained
        // instead, but the serde shape must accept and round-trip an
        // explicit empty list).
        let state = PipelineNodeState::SequentialDrain { children: vec![] };
        let json = serde_json::to_string(&state).unwrap();
        let parsed: PipelineNodeState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }

    #[test]
    fn legacy_lossy_sequential_drain_shape_fails_to_deserialize() {
        // 0.4.0 minted tokens with this lossy `{current_min_epk,
        // current_max_epk, left_most}` shape — it only preserved the
        // front child's state, dropping sibling state mid-fan-out. The
        // new SDK rejects them so callers see a clear failure instead of
        // silent data loss; the CHANGELOG documents this as a breaking
        // change.
        let legacy = r#"{"kind":"sequential_drain","current_min_epk":"00","current_max_epk":"FF","left_most":{"kind":"request","server_continuation":"tok"}}"#;
        let result: Result<PipelineNodeState, _> = serde_json::from_str(legacy);
        assert!(
            result.is_err(),
            "legacy 0.4.0 SequentialDrain shape must fail to deserialize under the new schema"
        );
    }

    #[test]
    fn unknown_kind_fails_to_deserialize() {
        let bogus = r#"{"kind":"something_new"}"#;
        let result: Result<PipelineNodeState, _> = serde_json::from_str(bogus);
        assert!(result.is_err());
    }
}
