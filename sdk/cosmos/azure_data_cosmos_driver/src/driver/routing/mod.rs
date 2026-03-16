// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Routing state and systems for operation-level endpoint selection.

mod account_endpoint_state;
mod endpoint;
mod location_effects;
mod location_state_store;
pub(crate) mod partition_endpoint_state;
mod routing_systems;

pub(crate) use account_endpoint_state::AccountEndpointState;
pub(crate) use endpoint::{CosmosEndpoint, LocationIndex, UnavailableReason};
pub(crate) use location_effects::{LocationEffect, UnavailablePartition};
pub(crate) use location_state_store::{LocationSnapshot, LocationStateStore};
#[allow(unused_imports)] // Spec-defined system function; kept for future steps.
pub(crate) use routing_systems::{
    build_account_endpoint_state, can_circuit_breaker_trigger_failover, expire_partition_overrides,
    expire_unavailable_endpoints, is_eligible_for_ppaf, is_eligible_for_ppcb,
    mark_endpoint_unavailable, mark_partition_unavailable,
};
