// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Resolves request session tokens and captures tokens from eligible responses.

use crate::{
    models::{CosmosOperation, OperationType, SessionToken, SubStatusCode},
    options::ReadConsistencyStrategy,
    pipeline::{
        operation::{
            routing::partition_key_range_id::PartitionKeyRangeId, session,
            session::manager::SessionManager, types::OperationOverrides,
        },
        transport::types::TransportOutcome,
    },
};

/// Seeds physical partition identity from explicit or internally resolved targeting.
pub(crate) fn partition_key_range_id_from_overrides(
    overrides: &OperationOverrides,
) -> Option<PartitionKeyRangeId> {
    overrides
        .effective_partition_key_range_id()
        .map(|id| PartitionKeyRangeId::from(id.to_owned()))
}

/// Session representations resolved once before dispatch chooses the gateway version.
#[derive(Default)]
pub(crate) struct ResolvedSession {
    /// Gateway v1 preserves composite tokens when narrowing would lose session progress.
    pub gateway: Option<SessionToken>,
    /// Gateway v2 uses only the target partition's token when its identity is known.
    pub gateway_v2: Option<SessionToken>,
    /// The request must use Gateway v1 to preserve a token that cannot be narrowed safely.
    pub requires_gateway: bool,
}

/// Resolves session representations and gateway constraints without changing routing or the cache.
///
/// Caller-supplied tokens take precedence. A composite token must be narrowed to
/// one physical partition for Gateway v2; otherwise, the caller must use Gateway v1.
/// Cached tokens are read together. Lazy logical requests retain Gateway v1 when
/// their physical partition is unknown and a cached token needs to be preserved.
pub(crate) fn resolve(
    session_manager: &SessionManager,
    operation: &CosmosOperation,
    partition_key_range_id: Option<&PartitionKeyRangeId>,
    overrides: &OperationOverrides,
    automatic_resolution_active: bool,
    lazy_topology: bool,
) -> ResolvedSession {
    if let Some(token) = operation.request_headers().session_token.as_ref() {
        // A composite token contains entries for several partitions. Only narrow it
        // when the routing map establishes which entries belong to this request.
        let scoped = if token.as_str().contains(',') {
            SessionManager::scope_explicit_token(
                token,
                partition_key_range_id.map(PartitionKeyRangeId::as_str),
                overrides.session_token_routing_map.as_deref(),
            )
        } else {
            Some(token.clone())
        };
        return ResolvedSession {
            requires_gateway: scoped.is_none(),
            gateway: Some(token.clone()),
            gateway_v2: scoped,
        };
    }

    // Cached tokens are optional, but disabling them must not discard a caller's token.
    if !automatic_resolution_active {
        return ResolvedSession::default();
    }

    let mut resolved = session_manager.resolve_for_attempt(
        operation,
        partition_key_range_id.map(PartitionKeyRangeId::as_str),
        overrides,
    );
    // Lazy logical requests keep even a single cached entry intact until the physical partition is known.
    resolved.requires_gateway = lazy_topology
        && overrides.logical_partition_key_target
        && partition_key_range_id.is_none()
        && resolved.gateway.is_some();
    resolved
}

/// Determines whether a session token should be captured from this response.
///
/// Follows Java/.NET patterns: capture on success (2xx), and on error responses
/// where the server still advanced the session token:
/// - 409 Conflict
/// - 412 Precondition Failed
/// - 404 when substatus is NOT ReadSessionNotAvailable (1002)
///
/// Does NOT capture on 404/1002 (that's the trigger for session retry/clear).
pub(crate) fn should_capture_session_token_from_status(
    substatus: Option<&SubStatusCode>,
    outcome: &TransportOutcome,
) -> bool {
    match outcome {
        TransportOutcome::Success { .. } => true,
        TransportOutcome::HttpError { status, .. } => {
            let code = status.status_code();
            if code == azure_core::http::StatusCode::Conflict
                || code == azure_core::http::StatusCode::PreconditionFailed
            {
                return true;
            }
            if code == azure_core::http::StatusCode::NotFound {
                // Capture on 404 unless substatus is ReadSessionNotAvailable (1002)
                return substatus
                    != Some(&crate::error::status_codes::substatus::READ_SESSION_NOT_AVAILABLE);
            }
            false
        }
        _ => false,
    }
}

// ── Pipeline stage helpers ────────────────────────────────────────────
//
// These pure (or narrowly-scoped) helpers carry the per-stage logic for
// `operation::execute` so the main loop body stays readable and
// each stage can be unit-tested in isolation.

/// Suppresses read-specific consistency strategy on writes.
pub(crate) fn read_consistency_strategy_for_operation(
    operation: &CosmosOperation,
    read_consistency_strategy: ReadConsistencyStrategy,
) -> ReadConsistencyStrategy {
    if operation.is_read_only() {
        read_consistency_strategy
    } else {
        ReadConsistencyStrategy::Default
    }
}

/// Allows automatic session progress for reads, batches, and multi-write operations.
pub(crate) fn operation_allows_automatic_session_token_resolution(
    operation: &CosmosOperation,
    multiple_write_locations_enabled: bool,
) -> bool {
    operation.is_read_only()
        || operation.operation_type() == OperationType::Batch
        || multiple_write_locations_enabled
}

pub(crate) mod container;
pub(crate) mod manager;

/// Publishes eligible session progress from the winning response only.
pub(crate) fn capture(
    ctx: &crate::pipeline::operation::AttemptContext<'_>,
    result: &crate::pipeline::transport::types::TransportResult,
) {
    if !ctx.policy.session_token_capture_active {
        return;
    }
    if let Some(cosmos_headers) = result.cosmos_headers() {
        if session::should_capture_session_token_from_status(
            cosmos_headers.substatus.as_ref(),
            &result.outcome,
        ) {
            ctx.services
                .session_manager
                .capture_session_token(ctx.operation, cosmos_headers);
        }
    }
}
