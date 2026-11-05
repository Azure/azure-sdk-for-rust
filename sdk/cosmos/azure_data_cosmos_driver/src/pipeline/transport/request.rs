// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Prepares a signed wire request or identifies an authorization or encoding failure.

use super::{
    cosmos_headers::{self, NO_RETRY_449},
    cosmos_transport_client::HttpRequest,
    deadline,
    gateway_v2::{self, GatewayV2Headers},
    request_signing,
    types::{TransportMode, TransportOutcome, TransportRequest, TransportResult},
    TransportPipelineConfig,
};
use crate::{
    diagnostics::RequestSentStatus,
    error::{status_codes, CosmosError},
    models::{CosmosStatus, Credential},
};
use azure_core::http::headers::HeaderValue;
use std::time::Duration;

/// A wire request with its transport mode and timeout sampled before signing.
pub(super) struct PreparedRequest {
    pub http_request: HttpRequest,
    pub mode: TransportMode,
    pub timeout: Option<Duration>,
}

/// The request is ready to send, or preparation failed before any network request.
pub(super) type PreparationOutcome = Result<PreparedRequest, PreparationFailure>;

/// Identifies the preparation step that failed and retains its underlying error.
pub(super) enum PreparationFailure {
    Authorization(CosmosError),
    Encoding(CosmosError),
}

impl PreparationFailure {
    /// Returns the client status reported for the failed preparation step.
    // Inline the small enum projection.
    #[inline(always)]
    pub fn status(&self) -> CosmosStatus {
        match self {
            Self::Authorization(_) => status_codes::CLIENT_GENERATED_401,
            Self::Encoding(_) => status_codes::CLIENT_BAD_REQUEST,
        }
    }
}

impl From<PreparationFailure> for TransportResult {
    // Inline result assembly; signing and encoding retain their call frames.
    #[inline]
    fn from(failure: PreparationFailure) -> Self {
        let status = failure.status();
        let error = match failure {
            PreparationFailure::Authorization(error) | PreparationFailure::Encoding(error) => error,
        };
        Self {
            outcome: TransportOutcome::TransportError {
                status,
                error,
                request_sent: RequestSentStatus::NotSent,
            },
        }
    }
}

/// Builds the request, samples its timeout, applies headers, signs it, and encodes Gateway V2.
///
/// Failures identify the preparation step; this function never sends a request or records diagnostics.
pub(super) async fn prepare(
    request: &TransportRequest,
    config: &TransportPipelineConfig<'_>,
    credential: &Credential,
) -> PreparationOutcome {
    let mut http_request = HttpRequest {
        url: request.url.clone(),
        method: request.method,
        headers: request.headers.clone(),
        body: request.body.clone(),
        timeout: None,
        #[cfg(feature = "fault_injection")]
        evaluation_collector: None,
    };
    let timeout = deadline::remaining(request.deadline).map(deadline::request_timeout);
    tracing::trace!(
        per_request_timeout = ?timeout, "transport pipeline: computed per-request timeout");
    cosmos_headers::apply_cosmos_headers(&mut http_request, config.user_agent, config.client_id);
    if !request.transport_mode.is_gateway_v2() {
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
    if request.transport_mode.is_gateway_v2() {
        let inputs = GatewayV2Headers {
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
    Ok(PreparedRequest {
        http_request,
        mode: request.transport_mode,
        timeout,
    })
}
