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
//! base64url-no-pad encoded JSON document. The current version is `c1.`.
//! Tokens with a `c<N>.` prefix where `N > 1` are returned by newer SDKs and
//! are rejected with a clear error.
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

/// Current SDK token version prefix.
const SDK_V1_PREFIX: &str = "c1.";

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

    /// Encodes a [`PipelineNodeState`] as a `c1.`-prefixed token.
    pub(crate) fn encode_v1(
        operation: &CosmosOperation,
        root_state: &PipelineNodeState,
    ) -> crate::error::Result<Self> {
        let token_operation = TokenOperation::for_operation(operation)?;
        let container = operation.container().ok_or_else(|| {
            crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::new(azure_core::http::StatusCode::BadRequest)).with_message("client-side continuation tokens require a query or change feed operation targeting a container").build()
        })?;
        let state = TokenState {
            operation: token_operation,
            rid: container.rid().to_string(),
            root: root_state.clone(),
        };

        let json = serde_json::to_vec(&state).map_err(|e| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("failed to serialize continuation token state")
                .with_source(e)
                .build()
        })?;
        let body = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json);
        let mut out = String::with_capacity(SDK_V1_PREFIX.len() + body.len());
        out.push_str(SDK_V1_PREFIX);
        out.push_str(&body);
        Ok(Self(out))
    }

    /// Resolves this token into a planner-ready form.
    pub(crate) fn resolve(&self) -> crate::error::Result<ResolvedToken> {
        if let Some(rest) = self.0.strip_prefix(SDK_V1_PREFIX) {
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
            let state: TokenState = serde_json::from_slice(&json).map_err(|e| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("continuation token has invalid JSON payload")
                    .with_source(e)
                    .build()
            })?;
            return Ok(ResolvedToken::ClientV1(state));
        }

        if let Some(version) = parse_client_version_prefix(&self.0) {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "continuation token uses unsupported version 'c{version}.'; \
                     this SDK only understands 'c1.' tokens — upgrade to a newer SDK"
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
                    crate::error::CosmosStatus::CLIENT_CONTINUATION_TOKEN_NON_QUERY_OPERATION,
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
}

impl TokenState {
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
    /// A client-issued v1 token containing a snapshot of pipeline state.
    ClientV1(TokenState),

    /// An opaque server continuation string. Only valid for trivial operations.
    ServerOpaque(String),
}

// `PipelineNodeState` lives in driver internals and is not Debug-printable
// outside; provide a tiny Debug shim so test panic messages can include it.
impl std::fmt::Debug for ResolvedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolvedToken::ClientV1(state) => write!(f, "ClientV1({state:?})"),
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
        let account = AccountReference::with_master_key(
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

    /// Decodes the base64url-no-pad payload of a `c1.`-prefixed token into
    /// its raw JSON bytes for inspection.
    fn decode_v1_payload(token: &ContinuationToken) -> String {
        let body = token
            .as_str()
            .strip_prefix(SDK_V1_PREFIX)
            .expect("token must be c1.-prefixed");
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
    fn encode_v1_drained_state() {
        let token = ContinuationToken::encode_v1(&query_op(), &PipelineNodeState::Drained).unwrap();
        assert_eq!(
            decode_v1_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"drained"}}"#,
        );
    }

    #[test]
    fn encode_v1_request_state_omits_absent_server_continuation() {
        let token = ContinuationToken::encode_v1(
            &query_op(),
            &PipelineNodeState::Request {
                server_continuation: None,
            },
        )
        .unwrap();
        assert_eq!(
            decode_v1_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"request"}}"#,
        );
    }

    #[test]
    fn encode_v1_request_state_includes_server_continuation() {
        let token = ContinuationToken::encode_v1(
            &query_op(),
            &PipelineNodeState::Request {
                server_continuation: Some("server-token-1".to_string()),
            },
        )
        .unwrap();
        assert_eq!(
            decode_v1_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"request","server_continuation":"server-token-1"}}"#,
        );
    }

    #[test]
    fn encode_v1_sequential_drain_state() {
        let token = ContinuationToken::encode_v1(
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
            decode_v1_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"sequential_drain","left_most_undrained_epk":"3F","active_tokens":[{"min_epk":"3F","max_epk":"7F","server_continuation":"srv"}]}}"#,
        );
    }

    #[test]
    fn encode_v1_sequential_drain_state_omits_empty_active_tokens() {
        // Common fast-path: every range below the cursor is drained and
        // no range above it owes a server continuation. The wire form
        // must omit `active_tokens` entirely (sparse encoding) so the
        // O(P) blow-up doesn't sneak back in.
        let token = ContinuationToken::encode_v1(
            &query_op(),
            &PipelineNodeState::SequentialDrain {
                left_most_undrained_epk: "80".to_string(),
                active_tokens: vec![],
            },
        )
        .unwrap();
        assert_eq!(
            decode_v1_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"sequential_drain","left_most_undrained_epk":"80"}}"#,
        );
    }

    #[test]
    fn encode_v1_includes_rid_regardless_of_query_body() {
        // The token only identifies the container by rid; the query text
        // itself is not hashed into the token.
        let token = ContinuationToken::encode_v1(
            &query_op().with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec()),
            &PipelineNodeState::Drained,
        )
        .unwrap();
        assert_eq!(
            decode_v1_payload(&token),
            r#"{"op":"Query","rid":"coll_rid","root":{"kind":"drained"}}"#,
        );
    }

    #[test]
    fn encode_v1_rejects_non_query_operation() {
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let read = CosmosOperation::read_item(item);
        let _err = ContinuationToken::encode_v1(&read, &PipelineNodeState::Drained).unwrap_err();
    }

    #[test]
    fn encode_v1_change_feed_operation_round_trips() {
        // A change feed iterator captures its position as an SDK-issued `c1.`
        // token; the resume request restores the server ETag via If-None-Match,
        // never `x-ms-continuation`. Guards against the regression where
        // `to_continuation_token()` on a change feed errored with 20117.
        let token = ContinuationToken::encode_v1(
            &change_feed_op(),
            &PipelineNodeState::Request {
                server_continuation: Some("\"etag-1\"".to_string()),
            },
        )
        .unwrap();
        assert_eq!(
            decode_v1_payload(&token),
            r#"{"op":"ChangeFeed","rid":"coll_rid","root":{"kind":"request","server_continuation":"\"etag-1\""}}"#,
        );

        match token.resolve().unwrap() {
            ResolvedToken::ClientV1(state) => {
                assert_eq!(state.operation, TokenOperation::ChangeFeed);
                state
                    .is_valid_for_operation(&change_feed_op())
                    .expect("change feed token resumes a change feed operation");
            }
            other => panic!("expected ClientV1 token, got {other:?}"),
        }
    }

    #[test]
    fn is_valid_for_operation_rejects_query_token_on_change_feed() {
        let state = TokenState {
            operation: TokenOperation::Query,
            rid: "coll_rid".to_string(),
            root: PipelineNodeState::Drained,
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
        };
        let err = state.is_valid_for_operation(&query_op()).unwrap_err();
        assert!(err.to_string().contains("Query"));
    }

    // ── Deserialization ─────────────────────────────────────────────────

    #[test]
    fn resolve_v1_drained_state() {
        let token =
            encode_v1_payload(r#"{"op":"Query","rid":"coll_rid","root":{"kind":"drained"}}"#);
        match token.resolve().unwrap() {
            ResolvedToken::ClientV1(state) => {
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
            ResolvedToken::ClientV1(state) => {
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
            ResolvedToken::ClientV1(state) => {
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
            ResolvedToken::ClientV1(state) => {
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
            ResolvedToken::ClientV1(state) => {
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
        };
        state.is_valid_for_operation(&query_op()).unwrap();
    }

    #[test]
    fn is_valid_for_operation_rejects_mismatched_rid() {
        let state = TokenState {
            operation: TokenOperation::Query,
            rid: "different_rid".to_string(),
            root: PipelineNodeState::Drained,
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
        };
        let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let read = CosmosOperation::read_item(item);
        let _err = state.is_valid_for_operation(&read).unwrap_err();
    }

    // ── CosmosError and fallback paths ────────────────────────────────────────

    #[test]
    fn rejects_newer_sdk_token() {
        // cspell:ignore somethingnew
        let token = ContinuationToken::from_string("c2.somethingnew".to_string());
        let err = token.resolve().unwrap_err();
        assert!(err.to_string().contains("c2."));
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
