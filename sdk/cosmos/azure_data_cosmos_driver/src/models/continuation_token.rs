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

use crate::driver::dataflow::PipelineNodeState;

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
    pub(crate) fn encode_v1(state: &PipelineNodeState) -> azure_core::Result<Self> {
        let json = serde_json::to_vec(state).map_err(|e| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!("failed to serialize continuation token state: {e}"),
            )
        })?;
        let body = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json);
        let mut out = String::with_capacity(SDK_V1_PREFIX.len() + body.len());
        out.push_str(SDK_V1_PREFIX);
        out.push_str(&body);
        Ok(Self(out))
    }

    /// Resolves this token into a planner-ready form.
    pub(crate) fn resolve(&self) -> azure_core::Result<ResolvedToken> {
        if let Some(rest) = self.0.strip_prefix(SDK_V1_PREFIX) {
            let json = base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(rest)
                .map_err(|e| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        format!("continuation token has invalid base64 payload: {e}"),
                    )
                })?;
            let state: PipelineNodeState = serde_json::from_slice(&json).map_err(|e| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::DataConversion,
                    format!("continuation token has invalid JSON payload: {e}"),
                )
            })?;
            return Ok(ResolvedToken::ClientV1(state));
        }

        if let Some(version) = parse_client_version_prefix(&self.0) {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!(
                    "continuation token uses unsupported version 'c{version}.'; \
                     this SDK only understands 'c1.' tokens — upgrade to a newer SDK"
                ),
            ));
        }

        // No client-version prefix: treat as an opaque server-issued token.
        Ok(ResolvedToken::ServerOpaque(self.0.clone()))
    }
}

/// Resolved form of a [`ContinuationToken`] for use during planning.
pub(crate) enum ResolvedToken {
    /// A client-issued v1 token containing a snapshot of pipeline state.
    ClientV1(PipelineNodeState),

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
        let token = ContinuationToken::encode_v1(&PipelineNodeState::Drained).unwrap();
        assert_eq!(decode_v1_payload(&token), r#"{"kind":"drained"}"#);
    }

    #[test]
    fn encode_v1_request_state_omits_absent_server_continuation() {
        let token = ContinuationToken::encode_v1(&PipelineNodeState::Request {
            server_continuation: None,
        })
        .unwrap();
        assert_eq!(decode_v1_payload(&token), r#"{"kind":"request"}"#);
    }

    #[test]
    fn encode_v1_request_state_includes_server_continuation() {
        let token = ContinuationToken::encode_v1(&PipelineNodeState::Request {
            server_continuation: Some("server-token-1".to_string()),
        })
        .unwrap();
        assert_eq!(
            decode_v1_payload(&token),
            r#"{"kind":"request","server_continuation":"server-token-1"}"#,
        );
    }

    #[test]
    fn encode_v1_sequential_drain_state() {
        let token = ContinuationToken::encode_v1(&PipelineNodeState::SequentialDrain {
            current_min_epk: "3F".to_string(),
            left_most: Box::new(PipelineNodeState::Request {
                server_continuation: None,
            }),
        })
        .unwrap();
        assert_eq!(
            decode_v1_payload(&token),
            r#"{"kind":"sequential_drain","current_min_epk":"3F","left_most":{"kind":"request"}}"#,
        );
    }

    // ── Deserialization ─────────────────────────────────────────────────

    #[test]
    fn resolve_v1_drained_state() {
        let token = encode_v1_payload(r#"{"kind":"drained"}"#);
        match token.resolve().unwrap() {
            ResolvedToken::ClientV1(state) => assert_eq!(state, PipelineNodeState::Drained),
            other => panic!("expected ClientV1, got {other:?}"),
        }
    }

    #[test]
    fn resolve_v1_request_state_with_server_continuation() {
        let token =
            encode_v1_payload(r#"{"kind":"request","server_continuation":"opaque-srv-token"}"#);
        match token.resolve().unwrap() {
            ResolvedToken::ClientV1(state) => assert_eq!(
                state,
                PipelineNodeState::Request {
                    server_continuation: Some("opaque-srv-token".to_string()),
                }
            ),
            other => panic!("expected ClientV1, got {other:?}"),
        }
    }

    #[test]
    fn resolve_v1_request_state_without_server_continuation() {
        let token = encode_v1_payload(r#"{"kind":"request"}"#);
        match token.resolve().unwrap() {
            ResolvedToken::ClientV1(state) => assert_eq!(
                state,
                PipelineNodeState::Request {
                    server_continuation: None,
                }
            ),
            other => panic!("expected ClientV1, got {other:?}"),
        }
    }

    #[test]
    fn resolve_v1_sequential_drain_state() {
        let token = encode_v1_payload(
            r#"{"kind":"sequential_drain","current_min_epk":"3F","left_most":{"kind":"request"}}"#,
        );
        match token.resolve().unwrap() {
            ResolvedToken::ClientV1(state) => assert_eq!(
                state,
                PipelineNodeState::SequentialDrain {
                    current_min_epk: "3F".to_string(),
                    left_most: Box::new(PipelineNodeState::Request {
                        server_continuation: None,
                    }),
                }
            ),
            other => panic!("expected ClientV1, got {other:?}"),
        }
    }

    // ── Error and fallback paths ────────────────────────────────────────

    #[test]
    fn rejects_newer_sdk_token() {
        // cspell:ignore somethingnew
        let token = ContinuationToken::from_string("c2.somethingnew".to_string());
        let err = token.resolve().unwrap_err();
        assert!(matches!(
            err.kind(),
            azure_core::error::ErrorKind::DataConversion
        ));
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
        let err = token.resolve().unwrap_err();
        assert!(matches!(
            err.kind(),
            azure_core::error::ErrorKind::DataConversion
        ));
    }

    #[test]
    fn rejects_invalid_json_in_v1_token() {
        let token = encode_v1_payload(r#"{"kind":"unknown_variant"}"#);
        let err = token.resolve().unwrap_err();
        assert!(matches!(
            err.kind(),
            azure_core::error::ErrorKind::DataConversion
        ));
    }
}
