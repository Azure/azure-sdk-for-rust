// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Executes requests and transport-level retries against a fixed endpoint.
//!
//! The operation pipeline owns routing, failover, and session recovery.
//!
//! Inline small pipeline-stage helpers; retain larger I/O and recovery functions for backtraces.

use crate::{
    diagnostics::{DiagnosticsContextBuilder, PipelineKind, RequestSentStatus, TransportSecurity},
    models::Credential,
    pipeline::{
        transport::{
            adaptive_transport::AdaptiveTransport,
            recovery::{ThrottleRetryPolicy, TransportRetryProgress},
            types::{TransportRequest, TransportResult},
        },
        Clock, RecoveryAction,
    },
};
use azure_core::http::headers::HeaderValue;

pub(crate) mod adaptive_transport;
mod authorization;
pub(crate) mod connectivity_probe;
pub(crate) mod cosmos_headers;
mod cosmos_transport;
pub(crate) mod cosmos_transport_client;
mod deadline;
mod diagnostics;
mod dispatch;
#[cfg(feature = "fault_injection")]
mod fault_injection;
mod gateway_v2;
pub(crate) mod http_client_factory;
pub(crate) mod recovery;
mod request;
pub(crate) mod request_signing;
#[cfg(feature = "reqwest")]
pub(crate) mod reqwest_transport_client;
mod sent_status;
mod sharded_transport;
pub(crate) mod types;

pub(crate) use authorization::AuthorizationContext;
pub(crate) use cosmos_transport::{uses_dataplane_pipeline, CosmosTransport};
pub(crate) use gateway_v2::eligibility::is_operation_supported_by_gateway_v2;
#[cfg(feature = "fault_injection")]
pub(crate) use gateway_v2::unwrap_response_for_gateway_v2;
#[cfg(test)]
pub(crate) use gateway_v2::{wrap_request_for_gateway_v2, GatewayV2Headers};
pub(crate) use sharded_transport::EndpointKey;

/// Fixed routing, identity, and retry limits for one transport invocation.
pub(crate) struct TransportPipelineConfig<'a> {
    pub allow_sent_transport_retry: bool,
    pub user_agent: &'a HeaderValue,
    pub client_id: &'a HeaderValue,
    pub pipeline_type: PipelineKind,
    pub transport_security: TransportSecurity,
    pub endpoint_key: EndpointKey,
    pub account_name: Option<String>,
    pub collection_rid: Option<String>,
    pub throttle_policy: ThrottleRetryPolicy,
}

/// I/O dependencies shared by the preparation, dispatch, and recovery stages.
pub(crate) struct TransportPipelineServices<'a> {
    pub transport: &'a AdaptiveTransport,
    pub credential: &'a Credential,
    pub clock: Clock,
}

/// Prepares and dispatches a request, recovering failures only against the same target.
pub(crate) async fn execute(
    request: TransportRequest,
    config: &TransportPipelineConfig<'_>,
    services: &TransportPipelineServices<'_>,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    let mut progress = TransportRetryProgress::default();
    loop {
        if request.deadline.exceeded(services.clock.now()) {
            tracing::trace!("transport pipeline: deadline exceeded before attempt");
            return TransportResult::deadline_exceeded(RequestSentStatus::NotSent);
        }

        let handle = diagnostics::start_attempt(&request, config, services, &progress, diagnostics);
        let prepared = match request::prepare(
            &request,
            config,
            services.credential,
            services.clock.clone(),
        )
        .await
        {
            Ok(prepared) => prepared,
            Err(error) => match recovery::recover_prep_error(error, handle, diagnostics) {
                RecoveryAction::Retry => continue,
                RecoveryAction::Fail(result) => return result,
            },
        };
        diagnostics::start_transport(handle, diagnostics);

        #[cfg(feature = "fault_injection")]
        let (prepared, collector) =
            fault_injection::attach_collector(prepared, diagnostics.fault_injection_enabled());

        let outcome = dispatch::send(
            &prepared,
            services.transport,
            progress.excluded_shard_id.take(),
            &config.endpoint_key,
            handle,
            diagnostics,
            services.clock.clone(),
        )
        .await;
        diagnostics::complete_attempt(&outcome, handle, diagnostics);

        #[cfg(feature = "fault_injection")]
        fault_injection::record_evaluations(collector, handle, diagnostics);
        let result = match &outcome {
            Ok(result) => result,
            Err(error) => error.result(),
        };
        tracing::debug!(outcome = ?result.outcome, "transport request complete");

        match outcome {
            Ok(result) => return result,
            Err(error) => match recovery::recover_dispatch_error(
                error,
                &request,
                config,
                services.transport,
                &mut progress,
                services.clock.clone(),
            )
            .await
            {
                RecoveryAction::Retry => continue,
                RecoveryAction::Fail(result) => return result,
            },
        }
    }
}

#[cfg(test)]
mod tests;
