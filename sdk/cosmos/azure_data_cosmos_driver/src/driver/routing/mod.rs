// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Routing state and systems for operation-level endpoint selection.

mod account_endpoint_state;
mod endpoint;
mod location_effects;
mod location_state_store;
pub(crate) mod partition_endpoint_state;
pub(crate) mod partition_key_range_id;
mod routing_systems;
pub(crate) mod session_container;
pub(crate) mod session_manager;

pub(crate) use account_endpoint_state::AccountEndpointState;
pub(crate) use endpoint::{CosmosEndpoint, LocationIndex, UnavailableReason};
pub(crate) use location_effects::{LocationEffect, UnavailablePartition};
#[cfg(feature = "tokio")]
pub(crate) use location_state_store::EndpointProbeFn;
pub(crate) use location_state_store::{LocationSnapshot, LocationStateStore};
#[allow(unused_imports)] // Spec-defined system function; kept for future steps.
pub(crate) use routing_systems::{
    advance_hub_region_discovery, build_account_endpoint_state, cache_hub_region,
    can_circuit_breaker_trigger_failover, expire_partition_overrides, expire_unavailable_endpoints,
    is_eligible_for_ppaf, is_eligible_for_ppcb, mark_endpoint_unavailable,
    mark_partition_unavailable, record_hedge_alternate_win, record_hedge_primary_win,
    remove_probe_succeeded_entry,
};
