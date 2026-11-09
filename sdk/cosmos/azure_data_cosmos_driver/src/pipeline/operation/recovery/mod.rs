// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Coordinates ordered error recovery and returns pipeline continuation decisions.

use std::ops::ControlFlow;

use crate::{
    diagnostics::{DiagnosticsContextBuilder, RequestSentStatus},
    models::{CosmosOperation, CosmosResponse, CosmosResponseHeaders, CosmosStatus},
    pipeline::{
        operation::{
            diagnostics, hedging,
            recovery::state::{OperationAction, OperationRetryState},
            response, routing,
            routing::{CosmosEndpoint, LocationEffects, LocationSnapshot, RoutingDecision},
            AttemptContext, OperationContext,
        },
        transport::types::{TransportOutcome, TransportResult},
    },
};

/// Recovers a non-success transport outcome, applying feedback before advancing retry progress.
///
/// Container recreation precedes status classification. Hedge upgrades are dispatched
/// only after advancing state and selecting fresh routes, without a new deadline gate.
/// DTX errors may complete with a response body for transaction-level decoding.
pub(crate) async fn recover(
    operation: &mut CosmosOperation,
    context: &OperationContext<'_>,
    captured: &LocationSnapshot,
    route: &RoutingDecision,
    result: TransportResult,
    retry_state: &mut OperationRetryState,
    diagnostics: DiagnosticsContextBuilder,
) -> ControlFlow<crate::error::Result<CosmosResponse>, DiagnosticsContextBuilder> {
    if let Some(restart) = recreation::recover(operation, context, &result, retry_state).await {
        return match restart {
            recreation::Restart::InPlace => ControlFlow::Continue(diagnostics),
            recreation::Restart::RebuildPlan => {
                ControlFlow::Break(Err(response::recreation_error(&result, diagnostics)))
            }
        };
    }

    let (action, effects) = self::classify_result(operation, &route.endpoint, result, retry_state);
    let (action, permit) =
        hedging::upgrade_retry(action, operation, context, captured, route, retry_state);
    routing::apply_location_effects(
        operation,
        effects,
        retry_state,
        context.services.location_state_store,
        context.config.options,
    )
    .await;

    match state::advance(
        action,
        operation,
        retry_state,
        &route.endpoint,
        context.services,
        context.policy.deadline,
    )
    .await
    {
        Continuation::Complete(result) => {
            routing::complete_attempt(
                operation,
                retry_state,
                context.services.location_state_store,
                &route.endpoint,
            )
            .await;
            ControlFlow::Break(response::build_cosmos_response(
                result,
                diagnostics,
                route.routing_fallback.is_some(),
            ))
        }
        Continuation::Abort(error) => {
            routing::abort_attempt(
                retry_state,
                context.services.location_state_store,
                &error.status(),
            )
            .await;
            diagnostics::record_abort(operation, retry_state, context.config, &error);
            ControlFlow::Break(Err(response::fail(error, diagnostics)))
        }
        Continuation::Retry => ControlFlow::Continue(diagnostics),
        Continuation::Hedge {
            threshold,
            strategy_config,
        } => {
            let captured = LocationSnapshot {
                // Retry upgrades retain the account identity captured before dispatch.
                account_name: captured.account_name.clone(),
                ..context.services.location_state_store.capture()
            };
            let Some((primary, secondary)) = hedging::retarget(
                operation,
                retry_state,
                &captured,
                context.config,
                context.services,
                context.policy,
            ) else {
                return ControlFlow::Continue(diagnostics);
            };
            let attempt = AttemptContext::new(operation, context, &captured, retry_state);
            let plan = hedging::eligibility::HedgeUpgrade {
                secondary_routing: secondary,
                threshold,
                strategy_config,
            };
            // Only retry-upgraded races pay for boxing; the ordinary operation future stays small.
            Box::pin(hedging::hedge_operation(
                &attempt,
                &captured,
                &primary,
                (
                    plan,
                    permit.expect("a hedge action owns an admission permit"),
                ),
                retry_state,
                diagnostics,
            ))
            .await
        }
    }
}

pub(crate) mod account;
pub(crate) mod deadline;
#[cfg(feature = "unstable_dtx")]
pub(crate) mod dtx;
pub(crate) mod error;
pub(crate) mod failover;
pub(crate) mod hedge;
pub(crate) mod recreation;
pub(crate) mod retry_with;
pub(crate) mod safety;
pub(crate) mod server;
pub(crate) mod session;
pub(crate) mod state;
pub(crate) mod transport;
pub(crate) mod write_region;

/// Control flow after the classified action has advanced operation-local progress.
#[allow(clippy::large_enum_variant)]
pub(crate) enum Continuation {
    Retry,
    Complete(Box<crate::pipeline::transport::types::TransportResult>),
    Abort(crate::error::CosmosError),
    Hedge {
        threshold: crate::options::HedgeThreshold,
        strategy_config: crate::diagnostics::hedging::HedgingStrategyConfig,
    },
}

/// Evaluates the result of a transport attempt and decides what to do next.
///
/// This is a pure function: it takes the operation, result, and retry state,
/// and returns an `OperationAction`. No side effects.
///
/// Dispatches to a per-outcome handler so each transport outcome shape is
/// classified in isolation. The HTTP-error handler in turn dispatches to a
/// chain of per-status-family helpers.
pub(crate) fn classify_result(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    result: TransportResult,
    retry_state: &OperationRetryState,
) -> (OperationAction, LocationEffects) {
    // Destructure the owned outcome to move error values out without
    // losing the error source chain.
    match result.outcome {
        outcome @ TransportOutcome::Success { .. } => (
            OperationAction::Complete(Box::new(TransportResult { outcome })),
            LocationEffects::default(),
        ),

        TransportOutcome::HttpError {
            status,
            cosmos_headers,
            body,
            request_sent,
        } => self::recover_http(
            operation,
            endpoint,
            retry_state,
            status,
            cosmos_headers,
            body,
            request_sent,
        ),

        TransportOutcome::TransportError {
            status,
            error,
            request_sent,
        } => transport::recover(
            operation,
            endpoint,
            retry_state,
            status,
            error,
            request_sent,
        ),

        TransportOutcome::DeadlineExceeded { request_sent } => deadline::recover(request_sent),
    }
}

/// Classifies an HTTP error response by walking a chain of per-status-family
/// handlers in priority order.
///
/// The order matters: the more specific Cosmos sub-status checks (403/3,
/// 404/1002, 429/3092) come before the generic status-code-family checks
/// (5xx). The first handler that recognizes the response returns
/// `Some(action, effects)`; if none match, the response is aborted with a
/// rich HTTP error.
#[allow(clippy::too_many_arguments)]
pub(crate) fn recover_http(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: CosmosStatus,
    cosmos_headers: CosmosResponseHeaders,
    body: Vec<u8>,
    request_sent: RequestSentStatus,
) -> (OperationAction, LocationEffects) {
    if let Some(result) = write_region::recover(operation, endpoint, retry_state, &status) {
        return result;
    }

    // Topology recovery precedes DTX delivery so stale ownership cannot escape
    // to the transaction coordinator as an ordinary response.
    if let Some(result) = account::recover(operation, endpoint, retry_state, &status) {
        return result;
    }

    #[cfg(feature = "unstable_dtx")]
    if operation.resource_type() == crate::models::ResourceType::DistributedTransactionBatch {
        return dtx::recover(status, cosmos_headers, body, retry_state);
    }

    if let Some(result) = session::recover(retry_state, &status, &cosmos_headers, &body) {
        return result;
    }

    if let Some(result) = failover::recover(operation, endpoint, retry_state, &status, request_sent)
    {
        return result;
    }

    if let Some(result) = retry_with::recover(retry_state, &status) {
        return result;
    }

    if let Some(result) = server::recover(operation, endpoint, retry_state, &status) {
        return result;
    }

    (
        OperationAction::Abort {
            error: error::build_service_error(&status, &cosmos_headers, &body),
        },
        LocationEffects::default(),
    )
}

#[cfg(test)]
mod tests;
