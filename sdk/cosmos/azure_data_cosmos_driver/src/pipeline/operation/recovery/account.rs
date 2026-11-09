// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for account outcomes.

use crate::{
    models::{CosmosOperation, CosmosStatus},
    pipeline::operation::{
        recovery::state::{OperationAction, OperationRetryState},
        routing,
        routing::{CosmosEndpoint, LocationEffect, LocationEffects, UnavailableReason},
    },
};

/// Handles 403/1008 DatabaseAccountNotFound for all operation types.
///
/// The region no longer owns the account; refresh topology and fail over with
/// exponential backoff bounded by the 5s backend-failover budget, with the
/// first retry immediate.
pub(crate) fn recover(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
) -> Option<(OperationAction, LocationEffects)> {
    if !status.is_database_account_not_found() {
        return None;
    }

    if !retry_state.can_retry_backend_failover() {
        return None;
    }
    let delay = retry_state.backend_failover_delay();
    let new_state = retry_state.clone().advance_backend_failover(delay);
    let delay = Some(delay);

    let mut effects = vec![
        LocationEffect::RefreshAccountProperties,
        LocationEffect::MarkPartitionUnavailable(routing::make_partition_unavailable(
            operation,
            endpoint,
            retry_state,
            operation.is_read_only(),
        )),
    ];
    if !routing::is_ppcb_managed(operation, retry_state) {
        // PPCB-managed 1008 is per-partition; do not block the whole endpoint for 60s.
        effects.push(LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint.clone(),
            reason: UnavailableReason::DatabaseAccountNotFound,
        });
    }
    Some((
        OperationAction::FailoverRetry { new_state, delay },
        effects.into(),
    ))
}
