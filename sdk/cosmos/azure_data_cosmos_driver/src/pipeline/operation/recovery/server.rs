// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for server outcomes.

use crate::{
    models::{CosmosOperation, CosmosStatus},
    pipeline::operation::{
        recovery,
        recovery::state::{OperationAction, OperationRetryState},
        routing,
        routing::{CosmosEndpoint, LocationEffect, LocationEffects, UnavailableReason},
    },
};

/// Handles generic 5xx server errors (and 408 RequestTimeout as a defensive
/// fallback for the rare path where it didn't get classified by the
/// retry-trigger-group helper).
///
/// Cross-region retry is attempted for both reads and writes — the assumption
/// is that an internal error in one region is unlikely to repeat in another.
pub(crate) fn recover(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
) -> Option<(OperationAction, LocationEffects)> {
    let status_code = status.status_code();
    let is_eligible_status = status_code.is_server_error()
        || status_code == azure_core::http::StatusCode::RequestTimeout;
    if !(is_eligible_status && retry_state.can_retry_failover()) {
        return None;
    }

    if recovery::safety::is_unsafe_retry_after_possible_execution(operation, status) {
        return None;
    }

    let mut effects = vec![LocationEffect::MarkPartitionUnavailable(
        routing::make_partition_unavailable(
            operation,
            endpoint,
            retry_state,
            operation.is_read_only(),
        ),
    )];
    if !routing::is_ppcb_managed(operation, retry_state) {
        effects.push(LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint.clone(),
            reason: UnavailableReason::InternalServerError,
        });
    }
    Some((
        OperationAction::FailoverRetry {
            new_state: retry_state.clone().advance_failover(),
            delay: None,
        },
        effects.into(),
    ))
}
