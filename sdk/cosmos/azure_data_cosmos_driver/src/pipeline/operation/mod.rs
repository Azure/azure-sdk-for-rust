// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Executes logical operations through shared attempt stages and explicit recovery.

use std::ops::ControlFlow;

use crate::{
    diagnostics::DiagnosticsContextBuilder,
    models::{CosmosOperation, CosmosResponse},
    options::ResolvedThroughputControl,
    pipeline::{
        operation::{
            config::{OperationPipelineConfig, OperationPipelineServices},
            policy::Policy,
            recovery::state::OperationRetryState,
            routing::{
                partition_key_range_id::PartitionKeyRangeId, LocationSnapshot, RoutingDecision,
            },
            types::OperationOverrides,
        },
        transport::types::{TransportOutcome, TransportResult},
    },
};

pub(crate) mod config;
pub(crate) mod deadline;
pub(crate) mod diagnostics;
pub(crate) mod dispatch;
pub(crate) mod hedging;
pub(crate) mod patch;
pub(crate) mod policy;
pub(crate) mod recovery;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod routing;
pub(crate) mod session;
pub(crate) mod types;

/// Fixed inputs shared by attempts and recovery without borrowing the mutable operation.
pub(crate) struct OperationContext<'a> {
    pub config: &'a OperationPipelineConfig<'a>,
    pub services: &'a OperationPipelineServices<'a>,
    pub policy: &'a Policy,
    pub overrides: &'a OperationOverrides,
}

/// Read-only inputs captured before dispatching a sequential attempt or hedge race.
///
/// Legs share policy and services, but cannot mutate operation retry progress.
pub(crate) struct AttemptContext<'a> {
    pub config: &'a OperationPipelineConfig<'a>,
    pub services: &'a OperationPipelineServices<'a>,
    pub policy: &'a Policy,
    pub operation: &'a CosmosOperation,
    pub overrides: &'a OperationOverrides,
    pub account_name: Option<&'a str>,
    pub throughput_control: Option<ResolvedThroughputControl>,
    pub can_use_multiple_write_locations: bool,
    pub hub_region_processing_only_initial: bool,
    pub partition_key_range_id: Option<PartitionKeyRangeId>,
}

impl<'a> AttemptContext<'a> {
    /// Freezes attempt inputs without sharing mutable retry progress with hedge legs.
    pub(crate) fn new(
        operation: &'a CosmosOperation,
        context: &'a OperationContext<'a>,
        captured: &'a LocationSnapshot,
        retry_state: &OperationRetryState,
    ) -> Self {
        Self {
            config: context.config,
            services: context.services,
            policy: context.policy,
            operation,
            overrides: context.overrides,
            account_name: captured.account_name.as_deref(),
            throughput_control: retry_state.throughput_control,
            can_use_multiple_write_locations: retry_state.can_use_multiple_write_locations,
            hub_region_processing_only_initial: retry_state.hub_region_processing_only,
            partition_key_range_id: retry_state.partition_key_range_id.clone(),
        }
    }
}

/// Outcome and immutable wire facts from one transport invocation.
pub(crate) struct AttemptOutcome {
    pub result: crate::error::Result<TransportResult>,
    pub sent_hub_header: bool,
}

/// Resolves sessions, prepares the request, and dispatches one attempt without publishing feedback.
///
/// Sequential requests and both hedge legs use this path. Differences in headers,
/// consistency, and diagnostic identity are explicit attempt inputs. Only the
/// transport pipeline retries within an attempt.
pub(crate) async fn perform_single_attempt(
    ctx: &AttemptContext<'_>,
    routing: &RoutingDecision,
    options: request::AttemptOptions<'_>,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> AttemptOutcome {
    let session = session::resolve(
        ctx.services.session_manager,
        ctx.operation,
        ctx.partition_key_range_id.as_ref(),
        ctx.overrides,
        options.automatic_session_resolution,
        ctx.policy.lazy_topology,
    );

    let prepared = match request::prepare_attempt(ctx, routing, &options) {
        Ok(prepared) => prepared,
        Err(error) => {
            return AttemptOutcome {
                result: Err(error),
                sent_hub_header: false,
            };
        }
    };

    let result = dispatch::send(
        prepared.request,
        session,
        ctx,
        options.gateway_v2_allowed,
        diagnostics,
    )
    .await;

    AttemptOutcome {
        result,
        sent_hub_header: prepared.sent_hub_header,
    }
}

/// Coordinates capture, routing, hedge admission, dispatch, and recovery.
///
/// A hedge diversion returns only a terminal result or updated diagnostics for
/// sequential fallback. Shared response feedback is published at explicit stage
/// checkpoints, never from an individual racing leg.
pub(crate) async fn execute(
    operation: &mut CosmosOperation,
    overrides: OperationOverrides,
    config: &OperationPipelineConfig<'_>,
    services: &OperationPipelineServices<'_>,
    diagnostics: DiagnosticsContextBuilder,
) -> crate::error::Result<CosmosResponse> {
    let policy::Resolved {
        policy,
        mut retry_state,
        mut diagnostics,
    } = policy::resolve(operation, &overrides, config, services, diagnostics)?;
    let context = OperationContext {
        config,
        services,
        policy: &policy,
        overrides: &overrides,
    };

    loop {
        diagnostics = deadline::enforce_deadline_or_timeout(
            &services.clock,
            policy.deadline,
            config.options,
            diagnostics,
        )?;

        let captured = services.location_state_store.capture();
        let route = routing::route(
            operation,
            &overrides,
            &retry_state,
            &captured,
            config,
            services,
            &policy,
        );
        let attempt = AttemptContext::new(operation, &context, &captured, &retry_state);

        if let Some(plan) = hedging::should_hedge(&attempt, &retry_state, &captured, &route.routing)
        {
            return match hedging::hedge_operation(
                &attempt,
                &captured,
                &route.routing,
                plan,
                &mut retry_state,
                diagnostics,
            )
            .await
            {
                ControlFlow::Break(result) => result,
                ControlFlow::Continue(returned) => {
                    diagnostics = returned;
                    continue;
                }
            };
        }

        let options = request::sequential_options(&attempt, &route, &retry_state, &captured);
        let result =
            self::perform_single_attempt(&attempt, &route.routing, options, &mut diagnostics)
                .await
                .result?;

        routing::capture_pkrange(&result, &mut retry_state);
        session::capture(&attempt, &result);

        if matches!(result.outcome, TransportOutcome::Success { .. }) {
            routing::complete_attempt(
                operation,
                &mut retry_state,
                services.location_state_store,
                &route.routing.endpoint,
            )
            .await;
            return response::build_cosmos_response(
                Box::new(result),
                diagnostics,
                route.routing.routing_fallback.is_some(),
            );
        }

        match recovery::recover(
            operation,
            &context,
            &captured,
            &route.routing,
            result,
            &mut retry_state,
            diagnostics,
        )
        .await
        {
            ControlFlow::Break(result) => return result,
            ControlFlow::Continue(returned) => diagnostics = returned,
        }
    }
}

#[cfg(test)]
mod tests;
