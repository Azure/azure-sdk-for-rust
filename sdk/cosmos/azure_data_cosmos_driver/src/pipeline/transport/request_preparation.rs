// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Builds and signs one wire request without recording diagnostics.

use super::{
    cosmos_headers::{self, NO_RETRY_449},
    cosmos_transport_client::HttpRequest,
    execution::TransportPipelineConfig,
    gateway_v2::{self, WrapInputs},
    request_signing,
    types::{TransportMode, TransportRequest},
};
use crate::{
    error::{status_codes, CosmosError},
    models::Credential,
};
use azure_core::http::headers::HeaderValue;

pub(super) enum PreparationFailure {
    Authorization(CosmosError),
    Encoding(CosmosError),
}

/// Copies the routed request into an HTTP request with no backend timeout override.
pub(super) fn build_http_request(request: &TransportRequest) -> HttpRequest {
    HttpRequest {
        url: request.url.clone(),
        method: request.method,
        headers: request.headers.clone(),
        body: request.body.clone(),
        timeout: None,
        #[cfg(feature = "fault_injection")]
        evaluation_collector: None,
    }
}

/// Applies headers, signs the request, and wraps Gateway V2 requests for submission.
///
/// Returns the failure stage without sending the request or changing diagnostics.
pub(super) async fn prepare_request(
    mut http_request: HttpRequest,
    request: &TransportRequest,
    config: &TransportPipelineConfig<'_>,
    credential: &Credential,
) -> Result<HttpRequest, PreparationFailure> {
    cosmos_headers::apply_cosmos_headers(&mut http_request, config.user_agent, config.client_id);
    if request.transport_mode != TransportMode::GatewayV2 {
        cosmos_headers::apply_read_consistency_strategy(
            &mut http_request,
            request.read_consistency_strategy,
            request.operation_type.is_read_only(),
        );
        http_request
            .headers
            .insert(NO_RETRY_449, HeaderValue::from_static("true"));
    }
    request_signing::sign_request(&mut http_request, credential, &request.auth_context)
        .await
        .map_err(PreparationFailure::Authorization)?;
    if request.transport_mode == TransportMode::GatewayV2 {
        let inputs = WrapInputs {
            auth_context: &request.auth_context,
            operation_type: request.operation_type,
            resource_type: request.auth_context.resource_type,
            effective_partition_key: request.effective_partition_key.as_ref(),
            effective_consistency: request.effective_consistency,
            read_consistency_strategy: request.read_consistency_strategy,
            account_name: config.account_name.as_deref(),
            collection_rid: config.collection_rid.as_deref(),
        };
        http_request =
            gateway_v2::wrap_request_for_gateway_v2(http_request, &inputs).map_err(|error| {
                PreparationFailure::Encoding(
                    CosmosError::builder()
                        .with_status(status_codes::CLIENT_BAD_REQUEST)
                        .with_message(format!("Gateway 2.0 request wrap failed: {error}"))
                        .with_source(error)
                        .build(),
                )
            })?;
    }
    Ok(http_request)
}
