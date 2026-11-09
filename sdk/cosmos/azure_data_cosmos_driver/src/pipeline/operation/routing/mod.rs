// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Routing state and systems for operation-level endpoint selection.

use crate::{
    models::{CosmosOperation, DefaultConsistencyLevel},
    options::{self, ReadConsistencyStrategy},
    pipeline::operation::{
        config::{OperationPipelineConfig, OperationPipelineServices},
        policy::Policy,
        recovery::state::OperationRetryState,
        session,
        types::OperationOverrides,
    },
};

mod account_endpoint_state;
mod endpoint;
mod feedback;
pub(crate) use feedback::{
    abort_attempt, apply_hedge_effects, apply_location_effects, complete_attempt, is_ppcb_managed,
    is_region_confirming_status, make_partition_unavailable, partition_effects_for_deferral,
    populate_hub_region_cache_for_hedge_winner, record_hedge_outcome, HedgeOutcome,
};
#[cfg(test)]
pub(crate) use feedback::{
    hub_region_cache_populate_target, is_effect_already_applied, location_effects_for_operation,
};
mod location_effects;
mod location_state_store;
pub(crate) mod partition_endpoint_state;
pub(crate) mod partition_key_range_id;
mod routing_systems;

pub(crate) use account_endpoint_state::AccountEndpointState;
pub(crate) use endpoint::{CosmosEndpoint, LocationIndex, UnavailableReason};
pub(crate) use location_effects::{
    InFlightFailures, LocationEffect, LocationEffects, UnavailablePartition,
};
#[cfg(feature = "tokio")]
pub(crate) use location_state_store::{background_refresh_interval, EndpointProbeFn};
pub(crate) use location_state_store::{LocationSnapshot, LocationStateStore};
#[allow(unused_imports)] // Spec-defined system function; kept for future steps.
pub(crate) use routing_systems::{
    advance_hub_region_discovery, build_account_endpoint_state, cache_hub_region,
    can_circuit_breaker_trigger_failover, expire_partition_overrides, expire_unavailable_endpoints,
    is_eligible_for_ppaf, is_eligible_for_ppcb, mark_endpoint_unavailable,
    mark_partition_unavailable, record_hedge_alternate_win, record_hedge_primary_win,
    remove_probe_succeeded_entry,
};

mod selection;
pub(crate) use selection::{
    current_account_endpoint, endpoint_matches_region_or_url, fallback,
    is_distributed_transaction_operation, resolve, resolve_candidate_endpoints,
    resolve_partition_overrides, select, try_partition_circuit_breaker, try_partition_failover,
    try_select_endpoint, RoutingPolicy,
};

/// Route and session inputs resolved from the current snapshot for one attempt.
pub(crate) struct AttemptRoute {
    pub routing: RoutingDecision,
    pub consistency: DefaultConsistencyLevel,
    pub read_strategy: ReadConsistencyStrategy,
    pub automatic_session_resolution: bool,
}

/// Resolves a regional endpoint and consistency policy without changing shared state.
///
/// Region-pinned requests bypass endpoint selection. Session tokens and gateway
/// versions are resolved later by each attempt, including individual hedge legs.
#[allow(clippy::too_many_arguments)]
pub(crate) fn route(
    operation: &CosmosOperation,
    overrides: &OperationOverrides,
    retry_state: &OperationRetryState,
    location: &LocationSnapshot,
    config: &OperationPipelineConfig<'_>,
    services: &OperationPipelineServices<'_>,
    policy: &Policy,
) -> AttemptRoute {
    // An explicit region pin must survive retries and hedge-leg execution.
    let routing = match overrides.pinned_endpoint() {
        Some(pinned) => self::select(pinned),
        None => self::resolve(
            &services.clock,
            operation,
            retry_state,
            location,
            policy.endpoint_unavailability_ttl,
        ),
    };
    // PATCH checks prefer the writer. A fallback reader must not retain
    // a consistency strategy that requires reaching that writer.
    let attempt_read_consistency_strategy =
        if operation.prefers_write_endpoints_for_read() && routing.routing_fallback.is_some() {
            ReadConsistencyStrategy::Default
        } else {
            policy.operation_read_consistency_strategy
        };
    let attempt_effective_consistency = options::resolve_effective_consistency(
        attempt_read_consistency_strategy,
        config.account_default_consistency,
    );
    // Automatic tokens depend on both the operation kind and effective consistency.
    let attempt_session_token_resolution_active = !policy.session_capturing_disabled
        && session::operation_allows_automatic_session_token_resolution(
            operation,
            location.account.multiple_write_locations_enabled,
        )
        && attempt_read_consistency_strategy
            .is_session_effective(config.account_default_consistency);
    AttemptRoute {
        routing,
        consistency: attempt_effective_consistency,
        read_strategy: attempt_read_consistency_strategy,
        automatic_session_resolution: attempt_session_token_resolution_active,
    }
}

/// Learns physical partition identity once, without replacing an explicit request target.
pub(crate) fn capture_pkrange(
    result: &crate::pipeline::transport::types::TransportResult,
    state: &mut OperationRetryState,
) {
    if state.partition_key_range_id.is_none() {
        state.partition_key_range_id = result
            .cosmos_headers()
            .and_then(|headers| headers.partition_key_range_id.as_deref())
            .map(|id| partition_key_range_id::PartitionKeyRangeId::from(id.to_owned()));
    }
}

/// Routing decision for the current attempt.
///
/// Wraps a `CosmosEndpoint` that carries the resolved URL and optional
/// region for diagnostics and routing purposes.
#[derive(Clone, Debug)]
pub(crate) struct RoutingDecision {
    /// The resolved endpoint for this attempt.
    pub endpoint: CosmosEndpoint,
    /// Why normal routing replaced the operation's preferred route.
    pub routing_fallback: Option<RoutingFallbackReason>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RoutingFallbackReason {
    PatchVerificationReadWriteEndpointUnavailableOrExcluded,
}

impl RoutingFallbackReason {
    /// Returns the stable diagnostic label for a routing fallback.
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::PatchVerificationReadWriteEndpointUnavailableOrExcluded => {
                "patch_verification_read_write_endpoint_unavailable_or_excluded"
            }
        }
    }
}

impl std::fmt::Display for RoutingDecision {
    /// Formats the selected endpoint with its region when known.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(region) = self.endpoint.region() {
            write!(f, "{}({})", region, self.endpoint.url())
        } else {
            write!(f, "{}", self.endpoint.url())
        }
    }
}
