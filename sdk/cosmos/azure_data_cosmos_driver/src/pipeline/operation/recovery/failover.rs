// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for failover outcomes.

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

/// Classifies topology and transient failures according to request delivery safety and retry budgets.
pub(crate) fn recover(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    retry_state: &OperationRetryState,
    status: &CosmosStatus,
    request_sent: RequestSentStatus,
) -> Option<(OperationAction, LocationEffects)> {
    let is_system_resource_unavailable = status.is_throttled()
        && status.sub_status()
            == Some(crate::error::status_codes::substatus::SYSTEM_RESOURCE_UNAVAILABLE);
    let is_service_unavailable =
        status.status_code() == azure_core::http::StatusCode::ServiceUnavailable;
    // Partition Topology changes (410 with sub-status 1009) are handled by the dataflow layer, not classified as retry triggers here. Only non-topology 410s trigger retries.
    let is_gone = status.is_gone() && !status.is_partition_topology_change();
    let is_request_timeout = status.status_code() == azure_core::http::StatusCode::RequestTimeout;

    let in_trigger_group =
        is_system_resource_unavailable || is_service_unavailable || is_gone || is_request_timeout;
    if !(in_trigger_group && retry_state.can_retry_failover()) {
        return None;
    }

    if request_sent.definitely_not_sent() {
        return Some((
            OperationAction::FailoverRetry {
                new_state: retry_state.clone().advance_failover(),
                delay: None,
            },
            LocationEffects::default(),
        ));
    }

    // Declining here falls through to the caller's terminal branch, which
    // surfaces the service's own error rather than a synthesized one.
    if recovery::safety::is_unsafe_retry_after_possible_execution(operation, status) {
        return None;
    }

    let unavailable_reason = if is_request_timeout {
        UnavailableReason::RequestTimeout
    } else {
        UnavailableReason::ServiceUnavailable
    };

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
            reason: unavailable_reason,
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
