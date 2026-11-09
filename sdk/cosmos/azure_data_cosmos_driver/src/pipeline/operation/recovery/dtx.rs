// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for dtx outcomes.

use crate::{
    models::{CosmosResponseHeaders, CosmosStatus},
    pipeline::{
        operation::{
            recovery,
            recovery::state::{
                OperationAction, OperationRetryState, DTX_COORDINATOR_RETRY_INTERVAL,
                DTX_INFRA_BASE_BACKOFF, DTX_INFRA_MAX_BACKOFF, DTX_INFRA_MAX_EXPONENT,
            },
            routing::LocationEffects,
        },
        transport::types::{TransportOutcome, TransportResult},
    },
};

/// Retries eligible bodyless coordinator failures or delivers the response for transaction-level decoding.
#[cfg(feature = "unstable_dtx")]
pub(crate) fn recover(
    status: CosmosStatus,
    cosmos_headers: CosmosResponseHeaders,
    body: Vec<u8>,
    retry_state: &OperationRetryState,
) -> (OperationAction, LocationEffects) {
    if body.is_empty() {
        if let Some((new_state, delay)) =
            recovery::dtx::try_dtx_bodyless_retry(&status, &cosmos_headers, retry_state)
        {
            return (
                OperationAction::DtxRetry { new_state, delay },
                LocationEffects::default(),
            );
        }
    }

    // A DTX coordinator response that is not retried here is handed to the outer
    // loop as a transport-level *delivery* success: the body reached us intact.
    // This is NOT a DTX success — the real per-operation outcome (including
    // `452`/`500`/etc.) is (re)derived by `DistributedTransactionResponse::from_body`
    // downstream. Wrapping it as `Success` here just means "deliver the body up".
    (
        OperationAction::Complete(Box::new(TransportResult {
            outcome: TransportOutcome::Success {
                status,
                cosmos_headers,
                body,
            },
        })),
        LocationEffects::default(),
    )
}

/// Proposes a coordinator or infrastructure retry with its independent budget and delay.
#[cfg(feature = "unstable_dtx")]
pub(crate) fn try_dtx_bodyless_retry(
    status: &CosmosStatus,
    cosmos_headers: &CosmosResponseHeaders,
    retry_state: &OperationRetryState,
) -> Option<(OperationRetryState, std::time::Duration)> {
    if recovery::dtx::is_dtx_bodyless_coordinator_retriable(status) {
        if !retry_state.can_retry_dtx_coordinator() {
            return None;
        }

        let delay = cosmos_headers
            .retry_after_ms
            .map(std::time::Duration::from_millis)
            .unwrap_or(DTX_COORDINATOR_RETRY_INTERVAL);
        return Some((retry_state.clone().advance_dtx_coordinator_retry(), delay));
    }

    if recovery::dtx::is_dtx_bodyless_infra_retriable(status) {
        if !retry_state.can_retry_dtx_infra() {
            return None;
        }

        let delay = recovery::dtx::dtx_infra_retry_delay(retry_state.dtx_infra_retry_count);
        return Some((retry_state.clone().advance_dtx_infra_retry(), delay));
    }

    None
}

/// Recognizes bodyless coordinator timeout and race-conflict responses.
#[cfg(feature = "unstable_dtx")]
pub(crate) fn is_dtx_bodyless_coordinator_retriable(status: &CosmosStatus) -> bool {
    status.status_code() == azure_core::http::StatusCode::RequestTimeout
        || (u16::from(status.status_code()) == 449
            && status.sub_status()
                == Some(crate::error::status_codes::substatus::DTC_COORDINATOR_RACE_CONFLICT))
}

/// Recognizes bodyless ledger, account-configuration, and dispatch failures.
#[cfg(feature = "unstable_dtx")]
pub(crate) fn is_dtx_bodyless_infra_retriable(status: &CosmosStatus) -> bool {
    status.status_code() == azure_core::http::StatusCode::InternalServerError
        && matches!(
            status.sub_status(),
            Some(crate::error::status_codes::substatus::DTC_LEDGER_FAILURE)
                | Some(crate::error::status_codes::substatus::DTC_ACCOUNT_CONFIG_FAILURE)
                | Some(crate::error::status_codes::substatus::DTC_DISPATCH_FAILURE)
        )
}

/// Computes capped exponential infrastructure backoff without consuming retry state.
#[cfg(feature = "unstable_dtx")]
pub(crate) fn dtx_infra_retry_delay(attempt: u32) -> std::time::Duration {
    let exponent = attempt.min(DTX_INFRA_MAX_EXPONENT);
    let delay = DTX_INFRA_BASE_BACKOFF.mul_f64(2_f64.powi(exponent as i32));
    delay.min(DTX_INFRA_MAX_BACKOFF)
}
