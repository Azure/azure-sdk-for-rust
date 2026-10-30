// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Continuation token type for resumable Cosmos DB feed operations.
//!
//! A [`ContinuationToken`] is an opaque, durable representation of where a
//! feed operation left off. Tokens are produced by the SDK from a live
//! [`OperationPlan`](crate::OperationPlan) and consumed by
//! [`CosmosDriver::plan_operation`](crate::driver::CosmosDriver::plan_operation)
//! to build an equivalent pipeline that resumes at the same position.
//!
//! # Token format
//!
//! SDK-issued tokens start with a version prefix `c<N>.` followed by a
//! base64url-no-pad encoded JSON document. New query and change-feed checkpoints
//! use `c2.`. Supported legacy `c1.` checkpoints remain readable, but SDKs that
//! only support `c1.` reject `c2.` checkpoints.
//! Unsupported versions are rejected with a clear error.
//!
//! Tokens without a `c<N>.` prefix are treated as opaque server-issued
//! continuation strings and are only valid for trivial operations
//! (single-partition or non-query operations) where the SDK can pass them
//! through unmodified.

use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::{
    driver::dataflow::PipelineNodeState,
    models::{CosmosOperation, OperationType},
};

const SDK_V1_PREFIX: &str = "c1.";
const SDK_V2_PREFIX: &str = "c2.";

/// Opaque continuation token for resuming a paginated Cosmos DB operation.
///
/// Construct one from a string returned by an earlier query (either the
/// SDK's `to_continuation_token()` output, or — for trivial operations — a
/// raw server-side continuation string).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuationToken(String);

impl ContinuationToken {
    /// Wraps an opaque continuation string.
    ///
    /// No validation is performed here; the string is validated when it is
    /// passed to
    /// [`CosmosDriver::plan_operation`](crate::driver::CosmosDriver::plan_operation).
    pub fn from_string(token: String) -> Self {
        Self(token)
    }

    /// Returns the underlying string form of this token.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Encodes query or change-feed state as `c2.`.
    pub(crate) fn encode(
        operation: &CosmosOperation,
        root_state: &PipelineNodeState,
    ) -> crate::error::Result<Self> {
        let token_operation = TokenOperation::for_operation(operation)?;
        let container = operation.container().ok_or_else(|| {
            crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::new(azure_core::http::StatusCode::BadRequest)).with_message("client-side continuation tokens require a query or change feed operation targeting a container").build()
        })?;
        // Change feed continuations are mode-specific: resuming a feed in a
        // different mode (LatestVersion vs AllVersionsAndDeletes) is not
        // supported, so record the mode and validate it on resume. Only the
        // full-fidelity marker is stored; LatestVersion (and query) tokens carry
        // no marker and are treated as LatestVersion, keeping their payload
        // unchanged and backward-compatible.
        let change_feed_full_fidelity = operation
            .is_change_feed()
            .then(|| operation.request_headers().full_fidelity_feed)
            .filter(|&full_fidelity| full_fidelity);
        let state = TokenState {
            operation: token_operation,
            rid: container.rid().to_string(),
            root: root_state.clone(),
            change_feed_full_fidelity,
            change_feed_start: if operation.is_change_feed() {
                match root_state {
                    PipelineNodeState::UnorderedMerge {
                        start_from: Some(_),
                        ..
                    } => None,
                    _ => operation.change_feed_start().cloned().or_else(|| {
                        let headers = operation.request_headers();
                        (!headers.full_fidelity_feed
                            && headers.precondition.is_none()
                            && headers.if_modified_since.is_none())
                        .then_some(crate::models::ChangeFeedStartFrom::Beginning)
                    }),
                }
            } else {
                None
            },
            next_change_feed_range: None,
        };

        let json = serde_json::to_vec(&state).map_err(|e| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("failed to serialize continuation token state")
                .with_source(e)
                .build()
        })?;
        let body = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json);
        let prefix = SDK_V2_PREFIX;
        let mut out = String::with_capacity(prefix.len() + body.len());
        out.push_str(prefix);
        out.push_str(&body);
        Ok(Self(out))
    }

    /// Resolves this token into a planner-ready form.
    pub(crate) fn resolve(&self) -> crate::error::Result<ResolvedToken> {
        if let Some(rest) = self
            .0
            .strip_prefix(SDK_V1_PREFIX)
            .or_else(|| self.0.strip_prefix(SDK_V2_PREFIX))
        {
            let json = base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(rest)
                .map_err(|e| {
                    crate::error::CosmosError::builder()
                        .with_status(crate::error::CosmosStatus::new(
                            azure_core::http::StatusCode::BadRequest,
                        ))
                        .with_message(format!(
                            "continuation token has invalid base64 payload: {e}"
                        ))
                        .build()
                })?;
            let mut state: TokenState = serde_json::from_slice(&json).map_err(|e| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("continuation token has invalid JSON payload")
                    .with_source(e)
                    .build()
            })?;
            if let Some(next) = state.next_change_feed_range.take() {
                match &mut state.root {
                    PipelineNodeState::UnorderedMerge {
                        next_poll_min_epk, ..
                    } if state.operation == TokenOperation::ChangeFeed => {
                        if next_poll_min_epk.is_none() {
                            *next_poll_min_epk = Some(next);
                        }
                    }
                    _ => return Err(crate::error::CosmosError::builder()
                        .with_status(
                            crate::error::status_codes::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH,
                        )
                        .with_message(
                            "change-feed polling position requires an unordered-merge checkpoint",
                        )
                        .build()),
                }
            }
            return Ok(ResolvedToken::Client(state));
        }

        if let Some(version) = parse_client_version_prefix(&self.0) {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "continuation token uses unsupported version 'c{version}.'; \
                     this SDK understands 'c1.' and 'c2.' tokens — upgrade to a newer SDK"
                ))
                .build());
        }

        // No client-version prefix: treat as an opaque server-issued token.
        Ok(ResolvedToken::ServerOpaque(self.0.clone()))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenOperation {
    Query,
    ChangeFeed,
}

impl TokenOperation {
    /// Determines the [`TokenOperation`] that corresponds to the given
    /// operation, or returns an error if the operation does not support
    /// client-side continuation tokens.
    ///
    /// Only query and change feed (incremental `ReadFeed`) operations carry
    /// SDK-issued continuation tokens. Every other operation type either has
    /// no resumable position or relies on a server-issued opaque token.
    fn for_operation(operation: &CosmosOperation) -> crate::error::Result<Self> {
        if operation.operation_type() == OperationType::Query {
            Ok(TokenOperation::Query)
        } else if operation.is_change_feed() {
            Ok(TokenOperation::ChangeFeed)
        } else {
            Err(crate::error::CosmosError::builder()
                .with_status(
                    crate::error::status_codes::CLIENT_CONTINUATION_TOKEN_NON_QUERY_OPERATION,
                )
                .with_message(
                    "client-side continuation tokens are only supported for query and change \
                     feed operations",
                )
                .build())
        }
    }
}

/// The decoded state of a continuation token that can be used to resume an operation.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TokenState {
    /// The kind of operation this token is for (e.g. query, read feed, etc.). This is used to ensure the token is only used with a compatible operation.
    #[serde(rename = "op")]
    operation: TokenOperation,

    /// The resource id (RID) of the container the operation was targeting.
    /// Validated on resume to ensure the token is being used against the same container.
    rid: String,

    /// The root node's state at the point of snapshotting.
    root: PipelineNodeState,

    /// For change feed tokens, `true` when the feed was issued in the
    /// AllVersionsAndDeletes (full-fidelity) mode and `false` for LatestVersion.
    /// `None` for query tokens (and for legacy change feed tokens that predate
    /// mode encoding, which are treated as LatestVersion). Validated on resume:
    /// a change feed cannot switch modes across continuations.
    #[serde(rename = "cfm", default, skip_serializing_if = "Option::is_none")]
    change_feed_full_fidelity: Option<bool>,

    /// Original change-feed start, including checkpoints taken before a
    /// single-request feed has received its first ETag.
    #[serde(rename = "cfs", default, skip_serializing_if = "Option::is_none")]
    change_feed_start: Option<crate::models::ChangeFeedStartFrom>,

    /// Legacy top-level polling position, normalized into the merge snapshot on decode.
    #[serde(rename = "cfn", default, skip_serializing)]
    next_change_feed_range: Option<String>,
}

impl TokenState {
    pub(crate) fn change_feed_start(&self) -> Option<&crate::models::ChangeFeedStartFrom> {
        if self.operation != TokenOperation::ChangeFeed {
            return None;
        }
        match &self.root {
            PipelineNodeState::UnorderedMerge { start_from, .. } => start_from.as_ref(),
            _ => None,
        }
        .or(self.change_feed_start.as_ref())
    }

    /// Validates that this token state is compatible with the provided query
    pub fn is_valid_for_operation(&self, operation: &CosmosOperation) -> crate::error::Result<()> {
        let expected = TokenOperation::for_operation(operation)?;
        if self.operation != expected {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "token operation type {op:?} is not compatible with this operation; \
                     expected {expected:?}",
                    op = self.operation,
                ))
                .build());
        }
        if expected == TokenOperation::ChangeFeed {
            // Legacy tokens without an encoded mode are treated as LatestVersion.
            let token_full_fidelity = self.change_feed_full_fidelity.unwrap_or(false);
            let operation_full_fidelity = operation.request_headers().full_fidelity_feed;
            if token_full_fidelity != operation_full_fidelity {
                let mode_name = |full_fidelity| {
                    if full_fidelity {
                        "AllVersionsAndDeletes"
                    } else {
                        "LatestVersion"
                    }
                };
                return Err(crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(
                        azure_core::http::StatusCode::BadRequest,
                    ))
                    .with_message(format!(
                        "change feed continuation token was issued for {token_mode} mode but this \
                         request uses {operation_mode} mode; a change feed cannot switch modes \
                         across continuations. Start a new change feed to change mode.",
                        token_mode = mode_name(token_full_fidelity),
                        operation_mode = mode_name(operation_full_fidelity),
                    ))
                    .build());
            }
        }
        let container = operation.container().ok_or_else(|| {
            crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::new(azure_core::http::StatusCode::BadRequest)).with_message("client-side continuation tokens require a query operation targeting a container").build()
        })?;
        if self.rid != container.rid() {
            return Err(crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::new(azure_core::http::StatusCode::BadRequest)).with_message(format!(
                    "token container rid {token_rid:?} does not match the operation's container rid {op_rid:?}; \
                     this token was generated against a different container and cannot be used to resume this one",
                    token_rid = self.rid,
                    op_rid = container.rid(),
                )).build());
        }
        Ok(())
    }

    /// Extracts the root node state from this token for use in planning a resume pipeline.
    pub fn into_root_node_state(self) -> PipelineNodeState {
        self.root
    }
}

/// Resolved form of a [`ContinuationToken`] for use during planning.
pub(crate) enum ResolvedToken {
    /// A supported client-issued token containing a snapshot of pipeline state.
    Client(TokenState),

    /// An opaque server continuation string. Only valid for trivial operations.
    ServerOpaque(String),
}

// `PipelineNodeState` lives in driver internals and is not Debug-printable
// outside; provide a tiny Debug shim so test panic messages can include it.
impl std::fmt::Debug for ResolvedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolvedToken::Client(state) => write!(f, "Client({state:?})"),
            ResolvedToken::ServerOpaque(s) => write!(f, "ServerOpaque({s})"),
        }
    }
}

/// Returns `Some(N)` if `s` starts with `c<N>.` for some unsigned integer `N`,
/// otherwise `None`.
///
/// The `c<N>.` prefix is a deliberate, reserved namespace for SDK-issued
/// tokens (where `N` is the SDK's continuation-token format version).
/// Server-issued opaque continuation tokens have never been observed to start
/// with this pattern, so the SDK treats any `c<N>.` token as SDK-versioned and
/// anything else as a server opaque token. If the server format ever changes
/// to collide with `c<N>.`, this is the place to revisit.
fn parse_client_version_prefix(s: &str) -> Option<u32> {
    let after_c = s.strip_prefix('c')?;
    let dot = after_c.find('.')?;
    after_c[..dot].parse::<u32>().ok()
}

// Allow direct serde of ContinuationToken as a string (e.g. for users storing
// it in a JSON document alongside other fields).
impl Serialize for ContinuationToken {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ContinuationToken {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Self(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::dataflow::RangedToken;
    use crate::models::{
        AccountReference, ContainerProperties, ContainerReference, FeedRange, ItemReference,
        PartitionKey, PartitionKeyDefinition, SystemProperties,
    };

    use url::Url;

    fn test_container() -> ContainerReference {
        let account = AccountReference::with_account_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        );
        let partition_key: PartitionKeyDefinition =
            serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        let props = ContainerProperties {
            id: "coll".into(),
            partition_key,
            system_properties: SystemProperties::default(),
        };
        ContainerReference::new(account, "db", "db_rid", "coll", "coll_rid", &props)
    }

    /// Builds a query-items operation against `test_container()` (rid `coll_rid`).
    fn query_op() -> CosmosOperation {
        CosmosOperation::query_items(test_container(), Some(FeedRange::full()))
    }

    /// Builds a single-partition change feed operation against `test_container()`
    /// (rid `coll_rid`).
    fn change_feed_op() -> CosmosOperation {
        let def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        CosmosOperation::change_feed(
            test_container(),
            Some(FeedRange::for_partition(PartitionKey::from("pk1"), &def)),
        )
    }

    /// Builds a single-partition AllVersionsAndDeletes (full-fidelity) change
    /// feed operation against `test_container()` (rid `coll_rid`).
    fn change_feed_avad_op() -> CosmosOperation {
        let def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        CosmosOperation::change_feed_all_versions_and_deletes(
            test_container(),
            Some(FeedRange::for_partition(PartitionKey::from("pk1"), &def)),
        )
    }

    /// Decodes the base64url-no-pad payload of an SDK-prefixed token into
    /// its raw JSON bytes for inspection.
    fn decode_payload(token: &ContinuationToken) -> String {
        let body = token
            .as_str()
            .strip_prefix(SDK_V1_PREFIX)
            .or_else(|| token.as_str().strip_prefix(SDK_V2_PREFIX))
            .expect("token must be SDK-prefixed");
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(body)
            .expect("payload must be valid base64url-no-pad");
        String::from_utf8(bytes).expect("payload must be valid UTF-8")
    }

    /// Builds a `c1.` token whose payload is the given JSON string.
    fn encode_v1_payload(json: &str) -> ContinuationToken {
        let body = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json);
        ContinuationToken::from_string(format!("{SDK_V1_PREFIX}{body}"))
    }

    // ── Serialization ───────────────────────────────────────────────────

    #[test]
    fn encode_v2_drained_state() {
        let token = ContinuationToken::encode(&query_op(), &PipelineNodeState::Drained).unwrap();
        assert_eq!(
            decode_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"drained"}}"#,
        );
    }

    #[test]
    fn encode_v2_request_state_omits_absent_server_continuation() {
        let token = ContinuationToken::encode(
            &query_op(),
            &PipelineNodeState::Request {
                server_continuation: None,
            },
        )
        .unwrap();
        assert_eq!(
            decode_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"request"}}"#,
        );
    }

    #[test]
    fn encode_v2_request_state_includes_server_continuation() {
        let token = ContinuationToken::encode(
            &query_op(),
            &PipelineNodeState::Request {
                server_continuation: Some("server-token-1".to_string()),
            },
        )
        .unwrap();
        assert_eq!(
            decode_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"request","server_continuation":"server-token-1"}}"#,
        );
    }

    #[test]
    fn encode_v2_sequential_drain_state() {
        let token = ContinuationToken::encode(
            &query_op(),
            &PipelineNodeState::SequentialDrain {
                left_most_undrained_epk: "3F".to_string(),
                active_tokens: vec![RangedToken {
                    min_epk: "3F".to_string(),
                    max_epk: "7F".to_string(),
                    server_continuation: "srv".to_string(),
                }],
            },
        )
        .unwrap();
        assert_eq!(
            decode_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"sequential_drain","left_most_undrained_epk":"3F","active_tokens":[{"min_epk":"3F","max_epk":"7F","server_continuation":"srv"}]}}"#,
        );
    }

    #[test]
    fn encode_v2_sequential_drain_state_omits_empty_active_tokens() {
        // Common fast-path: every range below the cursor is drained and
        // no range above it owes a server continuation. The wire form
        // must omit `active_tokens` entirely (sparse encoding) so the
        // O(P) blow-up doesn't sneak back in.
        let token = ContinuationToken::encode(
            &query_op(),
            &PipelineNodeState::SequentialDrain {
                left_most_undrained_epk: "80".to_string(),
                active_tokens: vec![],
            },
        )
        .unwrap();
        assert_eq!(
            decode_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"sequential_drain","left_most_undrained_epk":"80"}}"#,
        );
    }

    #[test]
    fn encode_v2_includes_rid_regardless_of_query_body() {
        // The token only identifies the container by rid; the query text
        // itself is not hashed into the token.
        let token = ContinuationToken::encode(
            &query_op().with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec()),
            &PipelineNodeState::Drained,
        )
        .unwrap();
        assert_eq!(
            decode_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"drained"}}"#,
        );
    }

    #[test]
    fn encode_v2_rejects_non_query_operation() {
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let read = CosmosOperation::read_item(item);
        let _err = ContinuationToken::encode(&read, &PipelineNodeState::Drained).unwrap_err();
    }

    #[test]
    fn encode_v2_change_feed_operation_round_trips() {
        // A change feed iterator captures its position as an SDK-issued `c2.`
        // token; the resume request restores the server ETag via If-None-Match,
        // never `x-ms-continuation`. Guards against the regression where
        // `to_continuation_token()` on a change feed errored with 20117.
        let token = ContinuationToken::encode(
            &change_feed_op(),
            &PipelineNodeState::Request {
                server_continuation: Some("\"etag-1\"".to_string()),
            },
        )
        .unwrap();
        assert_eq!(
            decode_payload(&token),
            r#"{"op":"ChangeFeed","rid":"coll_rid","root":{"kind":"request","server_continuation":"\"etag-1\""},"cfs":{"kind":"beginning"}}"#,
        );

        match token.resolve().unwrap() {
            ResolvedToken::Client(state) => {
                assert_eq!(state.operation, TokenOperation::ChangeFeed);
                state
                    .is_valid_for_operation(&change_feed_op())
                    .expect("change feed token resumes a change feed operation");
            }
            other => panic!("expected ClientV1 token, got {other:?}"),
        }
    }

    #[test]
    fn change_feed_checkpoint_keeps_start_before_first_request() {
        for start in [
            crate::models::ChangeFeedStartFrom::Beginning,
            crate::models::ChangeFeedStartFrom::Now,
            crate::models::ChangeFeedStartFrom::PointInTime(
                time::macros::datetime!(2026-08-21 12:34:56 UTC),
            ),
        ] {
            for continuation in [None, Some("\"etag\"".to_owned())] {
                let operation = change_feed_op().with_change_feed_start(start.clone());
                let root = PipelineNodeState::Request {
                    server_continuation: continuation,
                };
                let token = ContinuationToken::encode(&operation, &root).unwrap();
                assert!(token.as_str().starts_with("c2."));
                let ResolvedToken::Client(state) = token.resolve().unwrap() else {
                    panic!("SDK-issued checkpoint");
                };
                assert_eq!(state.change_feed_start(), Some(&start));
                assert_eq!(state.into_root_node_state(), root);
            }
        }
    }

    #[test]
    fn change_feed_checkpoint_records_implicit_beginning_without_guessing_custom_headers() {
        for root in [
            PipelineNodeState::Request {
                server_continuation: None,
            },
            PipelineNodeState::UnorderedMerge {
                active_tokens: vec![],
                start_from: None,
                next_poll_min_epk: None,
            },
        ] {
            for (operation, expected) in [
                (
                    change_feed_op(),
                    Some(crate::models::ChangeFeedStartFrom::Beginning),
                ),
                (
                    change_feed_op().with_precondition(crate::models::Precondition::if_none_match(
                        azure_core::http::Etag::from("*"),
                    )),
                    None,
                ),
                (
                    change_feed_op().with_if_modified_since("Fri, 21 Aug 2026 12:34:56 GMT".into()),
                    None,
                ),
            ] {
                let token = ContinuationToken::encode(&operation, &root).unwrap();
                let ResolvedToken::Client(state) = token.resolve().unwrap() else {
                    panic!("SDK-issued checkpoint");
                };
                assert_eq!(state.change_feed_start(), expected.as_ref());
            }
        }
    }

    #[test]
    fn existing_change_feed_checkpoints_remain_readable() {
        for (json, expected_start) in [
            (
                r#"{"op":"ChangeFeed","rid":"coll_rid","root":{"kind":"request","server_continuation":"etag"}}"#,
                None,
            ),
            (
                r#"{"op":"ChangeFeed","rid":"coll_rid","root":{"kind":"request"}}"#,
                None,
            ),
            (
                r#"{"op":"ChangeFeed","rid":"coll_rid","root":{"kind":"unordered_merge","active_tokens":[],"start_from":{"kind":"beginning"}}}"#,
                Some(crate::models::ChangeFeedStartFrom::Beginning),
            ),
            (
                r#"{"op":"ChangeFeed","rid":"coll_rid","root":{"kind":"unordered_merge","start_from":{"kind":"now"}}}"#,
                Some(crate::models::ChangeFeedStartFrom::Now),
            ),
        ] {
            let ResolvedToken::Client(state) = encode_v1_payload(json).resolve().unwrap() else {
                panic!("existing c1 checkpoint");
            };
            state.is_valid_for_operation(&change_feed_op()).unwrap();
            assert_eq!(state.change_feed_start(), expected_start.as_ref());
        }
    }

    #[test]
    fn fanout_checkpoint_records_restored_marker_not_resume_argument() {
        let operation =
            change_feed_op().with_change_feed_start(crate::models::ChangeFeedStartFrom::Now);
        let root = PipelineNodeState::UnorderedMerge {
            active_tokens: vec![],
            start_from: Some(crate::models::ChangeFeedStartFrom::Beginning),
            next_poll_min_epk: None,
        };
        let token = ContinuationToken::encode(&operation, &root).unwrap();
        let ResolvedToken::Client(state) = token.resolve().unwrap() else {
            panic!("SDK-issued checkpoint");
        };
        assert_eq!(
            state.change_feed_start(),
            Some(&crate::models::ChangeFeedStartFrom::Beginning)
        );
    }
    #[test]
    fn change_feed_checkpoint_polling_position_does_not_change_query_tokens() {
        let root = PipelineNodeState::UnorderedMerge {
            active_tokens: vec![],
            start_from: Some(crate::models::ChangeFeedStartFrom::Beginning),
            next_poll_min_epk: Some("80".into()),
        };
        let token = ContinuationToken::encode(&change_feed_op(), &root).unwrap();
        let ResolvedToken::Client(state) = token.resolve().unwrap() else {
            panic!("SDK-issued checkpoint");
        };
        assert!(matches!(state.into_root_node_state(),
            PipelineNodeState::UnorderedMerge { next_poll_min_epk: Some(next), .. } if next == "80"));
        let request = PipelineNodeState::Request {
            server_continuation: None,
        };
        let token = ContinuationToken::encode(&query_op(), &request).unwrap();
        assert_eq!(
            decode_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"request"}}"#
        );
    }

    #[test]
    fn change_feed_root_marker_is_authoritative_when_metadata_disagrees() {
        let token = encode_v1_payload(
            r#"{"op":"ChangeFeed","rid":"coll_rid","root":{"kind":"unordered_merge","start_from":{"kind":"beginning"}},"cfs":{"kind":"now"}}"#,
        );
        let ResolvedToken::Client(state) = token.resolve().unwrap() else {
            panic!("SDK-issued checkpoint");
        };
        assert_eq!(
            state.change_feed_start(),
            Some(&crate::models::ChangeFeedStartFrom::Beginning)
        );
    }

    #[test]
    fn change_feed_v2_has_one_start_and_snapshots_the_polling_position() {
        let operation =
            change_feed_op().with_change_feed_start(crate::models::ChangeFeedStartFrom::Beginning);
        let root = PipelineNodeState::UnorderedMerge {
            active_tokens: vec![],
            start_from: Some(crate::models::ChangeFeedStartFrom::Beginning),
            next_poll_min_epk: Some(String::new()),
        };
        let token = ContinuationToken::encode(&operation, &root).unwrap();
        assert!(token.as_str().starts_with(SDK_V2_PREFIX));
        let payload: serde_json::Value = serde_json::from_str(&decode_payload(&token)).unwrap();
        assert_eq!(
            payload,
            serde_json::json!({
                "op": "ChangeFeed", "rid": "coll_rid",
                "root": {
                    "kind": "unordered_merge",
                    "start_from": { "kind": "beginning" },
                    "next_poll_min_epk": ""
                }
            })
        );
        assert!(token.as_str().strip_prefix(SDK_V1_PREFIX).is_none());
        assert_eq!(parse_client_version_prefix(token.as_str()), Some(2));
        let ResolvedToken::Client(state) = token.resolve().unwrap() else {
            panic!("SDK-issued checkpoint");
        };
        assert_eq!(state.into_root_node_state(), root);
    }

    #[test]
    fn legacy_top_level_polling_position_is_normalized() {
        let token = encode_v1_payload(
            r#"{"op":"ChangeFeed","rid":"coll_rid","root":{"kind":"unordered_merge","start_from":{"kind":"beginning"}},"cfn":"80"}"#,
        );
        let ResolvedToken::Client(state) = token.resolve().unwrap() else {
            panic!("SDK-issued checkpoint");
        };
        assert!(matches!(state.into_root_node_state(),
            PipelineNodeState::UnorderedMerge { next_poll_min_epk: Some(next), .. } if next == "80"));
    }

    #[test]
    fn avad_header_free_checkpoint_does_not_infer_beginning() {
        let token = ContinuationToken::encode(
            &change_feed_avad_op(),
            &PipelineNodeState::Request {
                server_continuation: None,
            },
        )
        .unwrap();
        let ResolvedToken::Client(state) = token.resolve().unwrap() else {
            panic!("SDK-issued checkpoint");
        };
        assert_eq!(state.change_feed_start(), None);
    }

    #[test]
    fn legacy_query_checkpoints_reemit_as_v2_without_changing_payloads() {
        let operation = query_op();
        for json in [
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"drained"}}"#,
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"request"}}"#,
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"request","server_continuation":"server-token"}}"#,
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"sequential_drain","left_most_undrained_epk":"80","active_tokens":[{"min_epk":"80","max_epk":"FF","server_continuation":"server-token"}]}}"#,
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"skip_take","remaining_skip":2,"remaining_take":5,"child":{"kind":"request","server_continuation":"server-token"}}}"#,
        ] {
            let legacy = encode_v1_payload(json);
            let ResolvedToken::Client(legacy_state) = legacy.resolve().unwrap() else {
                panic!("legacy query checkpoint");
            };
            legacy_state.is_valid_for_operation(&operation).unwrap();
            let current = ContinuationToken::encode(&operation, &legacy_state.root).unwrap();
            assert!(current.as_str().starts_with(SDK_V2_PREFIX));
            assert_eq!(decode_payload(&current), json);
            let ResolvedToken::Client(current_state) = current.resolve().unwrap() else {
                panic!("current query checkpoint");
            };
            assert_eq!(current_state, legacy_state);
            current_state.is_valid_for_operation(&operation).unwrap();
            assert!(current_state
                .is_valid_for_operation(&change_feed_op())
                .is_err());
        }
    }

    #[test]
    fn is_valid_for_operation_rejects_query_token_on_change_feed() {
        let state = TokenState {
            operation: TokenOperation::Query,
            rid: "coll_rid".to_string(),
            root: PipelineNodeState::Drained,
            change_feed_full_fidelity: None,
            change_feed_start: None,
            next_change_feed_range: None,
        };
        let err = state.is_valid_for_operation(&change_feed_op()).unwrap_err();
        assert!(err.to_string().contains("ChangeFeed"));
    }

    #[test]
    fn is_valid_for_operation_rejects_change_feed_token_on_query() {
        let state = TokenState {
            operation: TokenOperation::ChangeFeed,
            rid: "coll_rid".to_string(),
            root: PipelineNodeState::Drained,
            change_feed_full_fidelity: None,
            change_feed_start: None,
            next_change_feed_range: None,
        };
        let err = state.is_valid_for_operation(&query_op()).unwrap_err();
        assert!(err.to_string().contains("Query"));
    }

    #[test]
    fn is_valid_for_operation_rejects_change_feed_mode_switch() {
        // A LatestVersion token (no mode marker) cannot resume an
        // AllVersionsAndDeletes feed, and vice versa.
        let latest = TokenState {
            operation: TokenOperation::ChangeFeed,
            rid: "coll_rid".to_string(),
            root: PipelineNodeState::Drained,
            change_feed_full_fidelity: None,
            change_feed_start: None,
            next_change_feed_range: None,
        };
        let err = latest
            .is_valid_for_operation(&change_feed_avad_op())
            .unwrap_err();
        assert!(err.to_string().contains("AllVersionsAndDeletes"));
        assert!(err.to_string().contains("LatestVersion"));

        let avad = TokenState {
            operation: TokenOperation::ChangeFeed,
            rid: "coll_rid".to_string(),
            root: PipelineNodeState::Drained,
            change_feed_full_fidelity: Some(true),
            change_feed_start: None,
            next_change_feed_range: None,
        };
        let err = avad.is_valid_for_operation(&change_feed_op()).unwrap_err();
        assert!(err.to_string().contains("AllVersionsAndDeletes"));
        assert!(err.to_string().contains("LatestVersion"));
    }

    #[test]
    fn is_valid_for_operation_accepts_matching_change_feed_mode() {
        let avad = TokenState {
            operation: TokenOperation::ChangeFeed,
            rid: "coll_rid".to_string(),
            root: PipelineNodeState::Drained,
            change_feed_full_fidelity: Some(true),
            change_feed_start: None,
            next_change_feed_range: None,
        };
        avad.is_valid_for_operation(&change_feed_avad_op())
            .expect("AllVersionsAndDeletes token resumes an AllVersionsAndDeletes operation");
    }

    #[test]
    fn encode_v2_records_all_versions_and_deletes_mode() {
        let token =
            ContinuationToken::encode(&change_feed_avad_op(), &PipelineNodeState::Drained).unwrap();
        assert!(decode_payload(&token).contains(r#""cfm":true"#));
    }

    #[test]
    fn encode_v2_omits_mode_marker_for_latest_version() {
        let token =
            ContinuationToken::encode(&change_feed_op(), &PipelineNodeState::Drained).unwrap();
        assert!(!decode_payload(&token).contains("cfm"));
    }

    // ── Deserialization ─────────────────────────────────────────────────

    #[test]
    fn resolve_v1_drained_state() {
        let token =
            encode_v1_payload(r#"{"op":"Query","rid":"coll_rid","root":{"kind":"drained"}}"#);
        match token.resolve().unwrap() {
            ResolvedToken::Client(state) => {
                assert_eq!(state.operation, TokenOperation::Query);
                assert_eq!(state.rid, "coll_rid");
                assert_eq!(state.root, PipelineNodeState::Drained);
            }
            other => panic!("expected ClientV1, got {other:?}"),
        }
    }

    #[test]
    fn resolve_v1_request_state_with_server_continuation() {
        let token = encode_v1_payload(
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"request","server_continuation":"opaque-srv-token"}}"#,
        );
        match token.resolve().unwrap() {
            ResolvedToken::Client(state) => {
                assert_eq!(state.operation, TokenOperation::Query);
                assert_eq!(state.rid, "coll_rid");
                assert_eq!(
                    state.root,
                    PipelineNodeState::Request {
                        server_continuation: Some("opaque-srv-token".to_string()),
                    },
                );
            }
            other => panic!("expected ClientV1, got {other:?}"),
        }
    }

    #[test]
    fn resolve_v1_request_state_without_server_continuation() {
        let token =
            encode_v1_payload(r#"{"op":"Query","rid":"coll_rid","root":{"kind":"request"}}"#);
        match token.resolve().unwrap() {
            ResolvedToken::Client(state) => {
                assert_eq!(state.operation, TokenOperation::Query);
                assert_eq!(state.rid, "coll_rid");
                assert_eq!(
                    state.root,
                    PipelineNodeState::Request {
                        server_continuation: None,
                    },
                );
            }
            other => panic!("expected ClientV1, got {other:?}"),
        }
    }

    #[test]
    fn resolve_v1_sequential_drain_state() {
        let token = encode_v1_payload(
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"sequential_drain","left_most_undrained_epk":"3F","active_tokens":[{"min_epk":"3F","max_epk":"7F","server_continuation":"srv"}]}}"#,
        );
        match token.resolve().unwrap() {
            ResolvedToken::Client(state) => {
                assert_eq!(state.operation, TokenOperation::Query);
                assert_eq!(state.rid, "coll_rid");
                assert_eq!(
                    state.root,
                    PipelineNodeState::SequentialDrain {
                        left_most_undrained_epk: "3F".to_string(),
                        active_tokens: vec![RangedToken {
                            min_epk: "3F".to_string(),
                            max_epk: "7F".to_string(),
                            server_continuation: "srv".to_string(),
                        }],
                    },
                );
            }
            other => panic!("expected ClientV1, got {other:?}"),
        }
    }

    #[test]
    fn resolve_v1_sequential_drain_state_without_active_tokens() {
        // Wire form may omit `active_tokens` entirely (common path: no
        // range above the cursor owes a server continuation). The
        // resolved state defaults to an empty `Vec`.
        let token = encode_v1_payload(
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"sequential_drain","left_most_undrained_epk":"80"}}"#,
        );
        match token.resolve().unwrap() {
            ResolvedToken::Client(state) => {
                assert_eq!(
                    state.root,
                    PipelineNodeState::SequentialDrain {
                        left_most_undrained_epk: "80".to_string(),
                        active_tokens: vec![],
                    },
                );
            }
            other => panic!("expected ClientV1, got {other:?}"),
        }
    }

    // ── Container-rid validation ────────────────────────────────────────

    #[test]
    fn is_valid_for_operation_accepts_matching_rid() {
        let state = TokenState {
            operation: TokenOperation::Query,
            rid: "coll_rid".to_string(),
            root: PipelineNodeState::Drained,
            change_feed_full_fidelity: None,
            change_feed_start: None,
            next_change_feed_range: None,
        };
        state.is_valid_for_operation(&query_op()).unwrap();
    }

    #[test]
    fn is_valid_for_operation_rejects_mismatched_rid() {
        let state = TokenState {
            operation: TokenOperation::Query,
            rid: "different_rid".to_string(),
            root: PipelineNodeState::Drained,
            change_feed_full_fidelity: None,
            change_feed_start: None,
            next_change_feed_range: None,
        };
        let err = state.is_valid_for_operation(&query_op()).unwrap_err();
        assert!(err.to_string().contains("different_rid"));
        assert!(err.to_string().contains("coll_rid"));
    }

    #[test]
    fn is_valid_for_operation_rejects_non_query_operation() {
        let state = TokenState {
            operation: TokenOperation::Query,
            rid: "coll_rid".to_string(),
            root: PipelineNodeState::Drained,
            change_feed_full_fidelity: None,
            change_feed_start: None,
            next_change_feed_range: None,
        };
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let read = CosmosOperation::read_item(item);
        let _err = state.is_valid_for_operation(&read).unwrap_err();
    }

    // ── CosmosError and fallback paths ────────────────────────────────────────

    #[test]
    fn rejects_newer_sdk_token() {
        // cspell:ignore somethingnew
        let token = ContinuationToken::from_string("c3.somethingnew".to_string());
        let err = token.resolve().unwrap_err();
        assert!(err.to_string().contains("c3."));
    }

    #[test]
    fn server_opaque_token_when_no_prefix() {
        let token = ContinuationToken::from_string("opaque-server-string".to_string());
        match token.resolve().unwrap() {
            ResolvedToken::ServerOpaque(s) => assert_eq!(s, "opaque-server-string"),
            other => panic!("expected ServerOpaque, got {other:?}"),
        }
    }

    #[test]
    fn rejects_invalid_base64_in_v1_token() {
        // cspell:ignore notvalid
        let token = ContinuationToken::from_string("c1.!!!notvalid!!!".to_string());
        let _err = token.resolve().unwrap_err();
    }

    #[test]
    fn rejects_invalid_json_in_v1_token() {
        // Missing the required `op` and `root` fields of `TokenState`.
        let token = encode_v1_payload(r#"{"kind":"drained"}"#);
        let _err = token.resolve().unwrap_err();
    }
}
