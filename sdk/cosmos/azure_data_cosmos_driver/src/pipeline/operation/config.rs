// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Caller configuration and shared operation services.

use std::{collections::HashMap, time::Duration};

use azure_core::http::headers::{HeaderName, HeaderValue};

use crate::{
    clock::Clock,
    diagnostics::{PipelineKind, TransportSecurity},
    driver::CosmosDriver,
    models::{AccountEndpoint, ActivityId, Credential, DefaultConsistencyLevel},
    options::{OperationOptions, OperationOptionsView, ResolvedThroughputControl},
    pipeline::{
        operation::{
            hedging::budget::HedgeBudget, routing::LocationStateStore,
            session::manager::SessionManager,
        },
        transport::{
            recovery::{
                DATA_PLANE_MAX_PER_RETRY_DELAY, DATA_PLANE_MAX_THROTTLE_ATTEMPTS,
                DATA_PLANE_MAX_THROTTLE_WAIT, METADATA_MAX_PER_RETRY_DELAY,
                METADATA_MAX_THROTTLE_ATTEMPTS, METADATA_MAX_THROTTLE_WAIT,
            },
            CosmosTransport,
        },
    },
};

/// Immutable caller identity and policy inputs for one operation invocation.
pub(crate) struct OperationPipelineConfig<'a> {
    pub options: &'a OperationOptionsView<'a>,
    pub operation_options: &'a OperationOptions,
    pub custom_headers: Option<&'a HashMap<HeaderName, HeaderValue>>,
    pub account_endpoint: &'a AccountEndpoint,
    pub user_agent: &'a HeaderValue,
    pub client_id: &'a HeaderValue,
    pub activity_id: &'a ActivityId,
    pub pipeline_type: PipelineKind,
    pub transport_security: TransportSecurity,
    pub account_default_consistency: DefaultConsistencyLevel,
    pub throughput_control: Option<ResolvedThroughputControl>,
}

/// Borrowed I/O and shared-state dependencies; stages receive only the capabilities they need.
pub(crate) struct OperationPipelineServices<'a> {
    pub driver: &'a CosmosDriver,
    pub location_state_store: &'a LocationStateStore,
    pub transport: &'a CosmosTransport,
    pub credential: &'a Credential,
    pub session_manager: &'a SessionManager,
    pub hedge_budget: &'a HedgeBudget,
    pub clock: Clock,
}

/// Default throttle-retry budget for a pipeline class when the caller hasn't
/// configured `ThrottlingRetryOptions`: the maximum number of 429 retries, the
/// cumulative-wait budget, and the per-retry delay cap ("interval"). Data-plane
/// gets more retries at a longer interval (count-limited); metadata keeps the
/// patient, shorter-interval budget.
pub(crate) fn default_throttle_budget(pipeline_type: PipelineKind) -> (u32, Duration, Duration) {
    if pipeline_type.is_data_plane() {
        (
            DATA_PLANE_MAX_THROTTLE_ATTEMPTS,
            DATA_PLANE_MAX_THROTTLE_WAIT,
            DATA_PLANE_MAX_PER_RETRY_DELAY,
        )
    } else {
        (
            METADATA_MAX_THROTTLE_ATTEMPTS,
            METADATA_MAX_THROTTLE_WAIT,
            METADATA_MAX_PER_RETRY_DELAY,
        )
    }
}

/// Resolves the limits for each independent fixed-endpoint transport invocation.
pub(crate) fn throttle_policy(
    config: &OperationPipelineConfig<'_>,
) -> crate::pipeline::transport::recovery::ThrottleRetryPolicy {
    let (attempts, wait, interval) = self::default_throttle_budget(config.pipeline_type);
    let options = config.options.throttling_retry_options();
    crate::pipeline::transport::recovery::ThrottleRetryPolicy::with_limits(
        options.max_retry_count().copied().unwrap_or(attempts),
        options.max_retry_wait_time().copied().unwrap_or(wait),
        interval,
    )
}
