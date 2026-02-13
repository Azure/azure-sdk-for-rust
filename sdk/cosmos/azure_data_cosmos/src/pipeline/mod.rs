// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod authorization_policy;
pub(crate) mod components;
mod retry_decisions;
mod signature_target;

use crate::constants::{self, SubStatusCode, SUB_STATUS};
use crate::cosmos_request::CosmosRequest;
use crate::models::CosmosResponse;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::CosmosClientOptions;
pub(crate) use authorization_policy::AuthorizationPolicy;
use azure_core::async_runtime::get_async_runtime;
use azure_core::error::{CheckSuccessOptions, ErrorKind};
use azure_core::http::{response::Response, Context, PipelineSendOptions, RawResponse};
use components::{OperationInfo, RetryState, RoutingState, ThrottleState};
use retry_decisions::{
    apply_data_plane_decision, apply_metadata_decision, apply_routing_for_next_region,
    apply_routing_for_write_endpoint, apply_service_unavailable_decision, apply_session_decision,
    apply_throttle_decision, decide_data_plane_retry, decide_metadata_retry, decide_throttle_retry,
    mark_endpoint_failed, RetryDecision,
};
use std::sync::Arc;
use tracing::debug;
use url::Url;

/// Success codes: 200-299 range plus 304 (Not Modified)
const SUCCESS_CODES: [u16; 101] = {
    let mut codes = [0u16; 101];
    let mut i = 0;
    while i < 100 {
        codes[i] = 200 + i as u16;
        i += 1;
    }
    codes[100] = 304;
    codes
};

/// Newtype that wraps an Azure Core pipeline to provide a Cosmos-specific pipeline
/// which configures our authorization policy and enforces that a [`ResourceType`]
/// is set on the context.
///
/// The pipeline uses a data-oriented retry loop: focused data components
/// ([`RoutingState`], [`RetryState`], [`ThrottleState`], [`OperationInfo`]) are
/// evaluated by pure decision functions in [`retry_decisions`], and the
/// orchestration loop applies state transformations between attempts.
#[derive(Debug, Clone)]
pub(crate) struct GatewayPipeline {
    pub(crate) endpoint: Url,
    pipeline: azure_core::http::Pipeline,
    global_endpoint_manager: Arc<GlobalEndpointManager>,
    options: CosmosClientOptions,
    #[cfg_attr(not(feature = "fault_injection"), allow(dead_code))]
    fault_injection_enabled: bool,
}

impl GatewayPipeline {
    pub(crate) fn new(
        endpoint: Url,
        pipeline: azure_core::http::Pipeline,
        global_endpoint_manager: Arc<GlobalEndpointManager>,
        options: CosmosClientOptions,
        fault_injection_enabled: bool,
    ) -> Self {
        GatewayPipeline {
            endpoint,
            pipeline,
            global_endpoint_manager,
            options,
            fault_injection_enabled,
        }
    }

    /// Sends a Cosmos request through the pipeline with data-oriented retry orchestration.
    ///
    /// The loop evaluates each response through pure decision functions and applies
    /// state transformations between attempts. No mutable policy objects are used.
    pub(crate) async fn send<T>(
        &self,
        mut cosmos_request: CosmosRequest,
        context: Context<'_>,
    ) -> azure_core::Result<CosmosResponse<T>> {
        cosmos_request.client_headers(&self.options);

        #[cfg(feature = "fault_injection")]
        if self.fault_injection_enabled {
            cosmos_request.add_fault_injection_headers();
        }

        // ── Build immutable operation info ──────────────────────────────
        let can_use_multi_write = self
            .global_endpoint_manager
            .can_use_multiple_write_locations(&cosmos_request);

        let op = OperationInfo::new(
            cosmos_request.operation_type,
            cosmos_request.resource_type,
            can_use_multi_write,
            self.global_endpoint_manager.preferred_location_count(),
            cosmos_request.excluded_regions.clone(),
        );

        // Set tentative writes header based on multi-write capability
        if can_use_multi_write {
            cosmos_request
                .headers
                .insert(constants::ALLOW_TENTATIVE_WRITES, "true");
        } else {
            cosmos_request
                .headers
                .remove(constants::ALLOW_TENTATIVE_WRITES);
        }

        // ── Initialize mutable state components ─────────────────────────
        let mut routing = RoutingState::default();
        let mut retry = RetryState::default();
        let mut throttle = ThrottleState::default();

        // ── Orchestration loop ──────────────────────────────────────────
        loop {
            // Refresh location cache (non-blocking, best-effort)
            _ = self.global_endpoint_manager.refresh_location(false).await;

            // Resolve endpoint from current routing state
            let endpoint = self
                .global_endpoint_manager
                .resolve_endpoint_from_components(&routing, &op);

            routing.resolved_endpoint = Some(endpoint.clone());

            // Stamp the resolved endpoint on the request for wire conversion
            cosmos_request
                .request_context
                .route_to_location_endpoint(endpoint.clone());

            debug!(
                target: "azure_data_cosmos::pipeline",
                "Sending request - endpoint: {:?}, operation: {:?}, resource: {:?}",
                endpoint,
                cosmos_request.operation_type,
                cosmos_request.resource_type,
            );

            // ── Send the HTTP request ───────────────────────────────────
            let result = {
                let pipeline = self.pipeline.clone();
                let ctx = context.clone();
                let resource_link = cosmos_request.resource_link.clone();
                let mut raw_req = cosmos_request.clone().into_raw_request();
                let success_options = CheckSuccessOptions {
                    success_codes: &SUCCESS_CODES,
                };
                let pipeline_send_options = PipelineSendOptions {
                    skip_checks: false,
                    check_success: success_options,
                };
                let ctx_owned = ctx.with_value(resource_link).into_owned();
                pipeline
                    .send(&ctx_owned, &mut raw_req, Some(pipeline_send_options))
                    .await
            };

            // ── Evaluate response ───────────────────────────────────────
            let (status, sub_status) = extract_status_info(&result);

            let decision = if let Some(status) = status {
                // First check data-plane / metadata retry
                let dp_decision = if op.is_metadata {
                    decide_metadata_retry(status, sub_status, &retry, &op)
                } else {
                    decide_data_plane_retry(status, sub_status, &retry, &op)
                };

                // If data-plane returns Done and this is a 429, check throttle
                if dp_decision == RetryDecision::Done
                    && status == azure_core::http::StatusCode::TooManyRequests
                {
                    let retry_after_ms = extract_retry_after_ms(&result);
                    decide_throttle_retry(&throttle, retry_after_ms)
                } else {
                    dp_decision
                }
            } else {
                // Transport error (no HTTP status): no retry
                RetryDecision::Done
            };

            // ── Apply decision ──────────────────────────────────────────
            match &decision {
                RetryDecision::Done => return finalize(result, cosmos_request),

                RetryDecision::Abort => return finalize(result, cosmos_request),

                RetryDecision::RetryNextRegion { delay } => {
                    // Determine which counter to bump based on the status
                    if let Some(s) = status {
                        if s == azure_core::http::StatusCode::ServiceUnavailable
                            || (s == azure_core::http::StatusCode::InternalServerError
                                && op.is_read_only)
                            || (s == azure_core::http::StatusCode::Gone
                                && sub_status == Some(SubStatusCode::LEASE_NOT_FOUND))
                        {
                            retry = apply_service_unavailable_decision(retry);
                        } else if op.is_metadata {
                            retry = apply_metadata_decision(retry);
                        } else {
                            retry = apply_data_plane_decision(retry, &decision);
                            // Mark endpoint unavailable & force-refresh for endpoint failover
                            mark_unavailable_and_refresh(
                                &self.global_endpoint_manager,
                                &endpoint,
                                &op,
                                true,
                            )
                            .await;
                        }
                    } else {
                        retry = apply_data_plane_decision(retry, &decision);
                    }

                    routing = mark_endpoint_failed(routing, &endpoint);
                    routing = apply_routing_for_next_region(
                        routing,
                        &retry,
                        status != Some(azure_core::http::StatusCode::Forbidden),
                    );

                    get_async_runtime().sleep(*delay).await;
                }

                RetryDecision::RetryOnWriteEndpoint { delay } => {
                    retry = apply_session_decision(retry);
                    routing = mark_endpoint_failed(routing, &endpoint);
                    routing = apply_routing_for_write_endpoint(routing);

                    get_async_runtime().sleep(*delay).await;
                }

                RetryDecision::RetrySameEndpoint { delay } => {
                    throttle = apply_throttle_decision(throttle, &decision);

                    get_async_runtime().sleep(*delay).await;
                }
            }

            // Update routing state on the request_context for the next attempt
            cosmos_request.request_context.clear_route_to_location();
            cosmos_request
                .request_context
                .route_to_location_index(routing.location_index, routing.use_preferred_locations);
        }
    }
}

/// Marks the current endpoint as unavailable and optionally force-refreshes
/// the location cache.
async fn mark_unavailable_and_refresh(
    gem: &GlobalEndpointManager,
    endpoint: &Url,
    op: &OperationInfo,
    force_refresh: bool,
) {
    if op.is_read_only {
        gem.mark_endpoint_unavailable_for_read(endpoint);
    } else {
        gem.mark_endpoint_unavailable_for_write(endpoint);
    }
    _ = gem.refresh_location(force_refresh).await;
}

/// Extracts HTTP status and Cosmos sub-status from a result.
fn extract_status_info(
    result: &azure_core::Result<RawResponse>,
) -> (Option<azure_core::http::StatusCode>, Option<SubStatusCode>) {
    match result {
        Ok(resp) => {
            let sub = resp
                .headers()
                .get_as::<u32, std::num::ParseIntError>(&SUB_STATUS)
                .ok()
                .map(SubStatusCode::from);
            (Some(resp.status()), sub)
        }
        Err(err) => {
            if let ErrorKind::HttpResponse { raw_response, .. } = err.kind() {
                if let Some(resp) = raw_response.as_ref() {
                    let sub = resp
                        .headers()
                        .get_as::<u32, std::num::ParseIntError>(&SUB_STATUS)
                        .ok()
                        .map(SubStatusCode::from);
                    return (Some(resp.status()), sub);
                }
            }
            (None, None)
        }
    }
}

/// Extracts the Retry-After header value in milliseconds from a response.
fn extract_retry_after_ms(result: &azure_core::Result<RawResponse>) -> Option<i64> {
    let resp = match result {
        Ok(resp) => resp,
        Err(err) => {
            if let ErrorKind::HttpResponse { raw_response, .. } = err.kind() {
                raw_response.as_ref()?
            } else {
                return None;
            }
        }
    };

    resp.headers()
        .get_as::<i64, std::num::ParseIntError>(&constants::RETRY_AFTER_MS)
        .ok()
}

/// Converts the HTTP result into a typed `CosmosResponse`, propagating errors.
fn finalize<T>(
    result: azure_core::Result<RawResponse>,
    cosmos_request: CosmosRequest,
) -> azure_core::Result<CosmosResponse<T>> {
    let raw_response = result?;
    let typed_response: Response<T> = raw_response.into();
    Ok(CosmosResponse::new(typed_response, cosmos_request))
}
