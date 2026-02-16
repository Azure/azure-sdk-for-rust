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
use azure_core::time::Duration;
use components::{CosmosStatus, OperationInfo, RetryState, ThrottleState};
use retry_decisions::RetryDecision;
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
        let mut retry = RetryState::default();
        let mut throttle = ThrottleState::default();

        // Safety bound: prevents infinite loops if decision logic has a bug
        const MAX_TOTAL_RETRIES: i32 = 150;
        let mut total_retries: i32 = 0;

        // ── Orchestration loop ──────────────────────────────────────────
        loop {
            if total_retries >= MAX_TOTAL_RETRIES {
                return Err(azure_core::Error::with_message(
                    ErrorKind::Other,
                    format!(
                        "Exceeded maximum total retry count ({MAX_TOTAL_RETRIES}); aborting request"
                    ),
                ));
            }
            total_retries += 1;

            // Refresh location cache (non-blocking, best-effort)
            _ = self.global_endpoint_manager.refresh_location(false).await;

            // Resolve endpoint from current routing state
            let endpoint = self
                .global_endpoint_manager
                .resolve_endpoint_from_components(&cosmos_request.routing, &op);

            cosmos_request.routing.resolved_endpoint = Some(endpoint.clone());

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

                // TODO @fabianm - Refactor this when moving to Driver - there is no need to
                // pass resource link via context to AuthorizationPolicy when we use our own pipeline
                let ctx_owned = ctx.with_value(resource_link).into_owned();
                pipeline
                    .send(&ctx_owned, &mut raw_req, Some(pipeline_send_options))
                    .await
            };

            // ── Evaluate response ───────────────────────────────────────
            let cosmos_status = extract_status_info(&result);
            let retry_after = extract_retry_after(&result);

            let decision = if let Some(ref cs) = cosmos_status {
                if op.is_metadata() {
                    RetryDecision::for_metadata(cs, &retry, &op, &throttle, retry_after)
                } else {
                    RetryDecision::for_data_plane(cs, &retry, &op, &throttle, retry_after)
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
                    // Determine which counter to bump and the new location index
                    let use_preferred;
                    if let Some(ref cs) = cosmos_status {
                        if cs.is_service_unavailable_class(op.is_read_only()) {
                            retry = retry.apply_service_unavailable();
                            use_preferred = true;
                        } else if op.is_metadata() {
                            retry = retry.apply_metadata();
                            use_preferred = true;
                        } else {
                            retry = retry.apply_data_plane(&decision);
                            use_preferred = !cs.is_write_forbidden();
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
                        retry = retry.apply_data_plane(&decision);
                        use_preferred = true;
                    }

                    cosmos_request.routing = cosmos_request.routing.mark_endpoint_failed(&endpoint);

                    // Compute the new location index based on the relevant counter
                    let new_index = if use_preferred {
                        // For service-unavailable-class or metadata: use svc unavailable count
                        // For data-plane failover with preferred: use failover count
                        if cosmos_status
                            .as_ref()
                            .is_some_and(|cs| cs.is_service_unavailable_class(op.is_read_only()))
                            || op.is_metadata()
                        {
                            retry.service_unavailable_count
                        } else {
                            retry.failover_count
                        }
                    } else {
                        retry.failover_count
                    };
                    cosmos_request.routing = cosmos_request
                        .routing
                        .apply_for_next_region(new_index, use_preferred);

                    get_async_runtime().sleep(*delay).await;
                }

                RetryDecision::RetryOnWriteEndpoint { delay } => {
                    retry = retry.apply_session();
                    cosmos_request.routing = cosmos_request.routing.mark_endpoint_failed(&endpoint);
                    cosmos_request.routing = cosmos_request.routing.apply_for_write_endpoint();

                    get_async_runtime().sleep(*delay).await;
                }

                RetryDecision::RetrySameEndpoint { delay } => {
                    throttle = throttle.apply(&decision);

                    get_async_runtime().sleep(*delay).await;
                }
            }

            // Update routing state on the request_context for the next attempt
            cosmos_request.request_context.clear_route_to_location();
            cosmos_request.request_context.route_to_location_index(
                cosmos_request.routing.location_index,
                cosmos_request.routing.use_preferred_locations,
            );
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
    if op.is_read_only() {
        gem.mark_endpoint_unavailable_for_read(endpoint);
    } else {
        gem.mark_endpoint_unavailable_for_write(endpoint);
    }
    _ = gem.refresh_location(force_refresh).await;
}

/// Extracts HTTP status and Cosmos sub-status from a result, returning a
/// [`CosmosStatus`] if an HTTP response is available.
fn extract_status_info(result: &azure_core::Result<RawResponse>) -> Option<CosmosStatus> {
    match result {
        Ok(resp) => {
            let sub = resp
                .headers()
                .get_as::<u32, std::num::ParseIntError>(&SUB_STATUS)
                .ok()
                .map(SubStatusCode::from);
            Some(CosmosStatus::new(resp.status(), sub))
        }
        Err(err) => {
            if let ErrorKind::HttpResponse { raw_response, .. } = err.kind() {
                if let Some(resp) = raw_response.as_ref() {
                    let sub = resp
                        .headers()
                        .get_as::<u32, std::num::ParseIntError>(&SUB_STATUS)
                        .ok()
                        .map(SubStatusCode::from);
                    return Some(CosmosStatus::new(resp.status(), sub));
                }
            }
            None
        }
    }
}

/// Extracts the Retry-After header value as a [`Duration`] from a response.
fn extract_retry_after(result: &azure_core::Result<RawResponse>) -> Option<Duration> {
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
        .filter(|&ms| ms > 0)
        .map(Duration::milliseconds)
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
