// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for transport outcomes.

use crate::{
    diagnostics::RequestSentStatus,
    models::{CosmosOperation, CosmosStatus},
    pipeline::operation::{
        recovery,
        recovery::state::{OperationAction, OperationRetryState},
        routing,
        routing::{CosmosEndpoint, LocationEffect, LocationEffects, UnavailableReason},
    },
};

/// Handles transport-layer errors where no HTTP response was produced.
///
/// Not-sent marks the endpoint only; sent/unknown marks the partition only, matching .NET/Python.
pub(crate) fn recover(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: CosmosStatus,
    error: crate::error::CosmosError,
    request_sent: RequestSentStatus,
) -> (OperationAction, LocationEffects) {
    if request_sent.definitely_not_sent() {
        // Not-sent means endpoint-wide failure; do not inflate PPCB's partition counter.
        let effects = vec![LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint.clone(),
            reason: UnavailableReason::TransportError,
        }];

        if retry_state.can_retry_failover() {
            return (
                OperationAction::FailoverRetry {
                    new_state: retry_state.clone().advance_failover(),
                    delay: None,
                },
                effects.into(),
            );
        }

        return (
            OperationAction::Abort {
                error: recovery::error::build_transport_error(&status, error),
            },
            effects.into(),
        );
    }

    // Request was sent (or unknown) — the endpoint is reachable, but this
    // partition had an issue. Only mark the partition; do NOT mark the
    // endpoint since other partitions on it are unaffected.
    let effects = vec![LocationEffect::MarkPartitionUnavailable(
        routing::make_partition_unavailable(
            operation,
            endpoint,
            retry_state,
            operation.is_read_only(),
        ),
    )];

    // A stored procedure may already have run to completion on the far side of
    // this failure, so the outcome is unknowable and the retry is unsafe.
    if retry_state.can_retry_failover() && operation.allows_ambiguous_outcome_retry() {
        return (
            OperationAction::FailoverRetry {
                new_state: retry_state.clone().advance_failover(),
                delay: None,
            },
            effects.into(),
        );
    }

    // Budget exhausted, or the operation cannot tolerate an ambiguous retry.
    (
        OperationAction::Abort {
            error: recovery::error::build_transport_error(&status, error),
        },
        effects.into(),
    )
}
