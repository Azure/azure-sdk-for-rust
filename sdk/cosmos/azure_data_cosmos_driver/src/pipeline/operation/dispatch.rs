// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Sends an operation attempt through the fixed-endpoint transport pipeline.

use azure_core::http::headers::HeaderValue;

use crate::{
    diagnostics::{DiagnosticsContextBuilder, PipelineKind},
    models::{request_header_names, CosmosOperation},
    pipeline::{
        operation::{config, session::ResolvedSession, AttemptContext},
        transport::{
            self,
            types::{TransportMode, TransportRequest, TransportResult},
            EndpointKey, TransportPipelineConfig, TransportPipelineServices,
        },
    },
};

/// Selects the transport and sends one prepared attempt with an independent throttle budget.
///
/// Transport creation errors remain distinct from transport outcomes. Shared routing and
/// session feedback belongs to the coordinator after this function returns.
pub(crate) async fn send(
    request: TransportRequest,
    session: ResolvedSession,
    ctx: &AttemptContext<'_>,
    gateway_v2_allowed: bool,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> crate::error::Result<TransportResult> {
    let (request, endpoint_key) = self::prepare_transport(
        request,
        ctx.operation,
        session,
        ctx.config.pipeline_type,
        ctx.account_name.is_some(),
        gateway_v2_allowed,
    );
    let selected = match ctx.config.pipeline_type {
        PipelineKind::DataPlane => ctx
            .services
            .transport
            .get_dataplane_transport(ctx.config.account_endpoint, request.transport_mode)?,
        PipelineKind::Metadata => ctx
            .services
            .transport
            .get_metadata_transport(ctx.config.account_endpoint)?,
    };

    tracing::trace!(method = ?request.method, url = %request.url, mode = ?request.transport_mode, "transport request prepared");
    Ok(transport::execute(
        request,
        &TransportPipelineConfig {
            allow_sent_transport_retry: ctx.operation.allows_ambiguous_outcome_retry(),
            user_agent: ctx.config.user_agent,
            client_id: ctx.config.client_id,
            pipeline_type: ctx.config.pipeline_type,
            transport_security: ctx.config.transport_security,
            endpoint_key,
            account_name: ctx.account_name.map(str::to_owned),
            collection_rid: ctx
                .operation
                .container()
                .map(|container| container.rid().to_owned()),
            throttle_policy: config::throttle_policy(ctx.config),
        },
        &TransportPipelineServices {
            transport: &selected,
            credential: ctx.services.credential,
            clock: ctx.services.clock.clone(),
        },
        diagnostics,
    )
    .await)
}

/// Chooses a gateway and applies its URL, session token, and headers as one dispatch decision.
///
/// Metadata always uses Gateway v1. The input request has the regional gateway URL
/// and no SDK-added gateway-specific headers; its encoded resource path is preserved when upgrading.
pub(crate) fn prepare_transport(
    mut request: TransportRequest,
    operation: &CosmosOperation,
    session: ResolvedSession,
    pipeline_type: PipelineKind,
    account_name_present: bool,
    gateway_v2_allowed: bool,
) -> (TransportRequest, EndpointKey) {
    let use_gateway_v2 = pipeline_type.is_data_plane()
        && gateway_v2_allowed
        && account_name_present
        && !session.requires_gateway
        && request.endpoint.uses_gateway_v2(true)
        && transport::is_operation_supported_by_gateway_v2(
            operation.resource_type(),
            operation.operation_type(),
            operation.request_headers().full_fidelity_feed,
            operation.resource_reference().is_rid_addressed(),
        );

    let token = if use_gateway_v2 {
        let mut url = request.endpoint.selected_url(true).clone();
        url.set_path(request.url.path());
        request.url = url;
        request.transport_mode = TransportMode::GatewayV2;
        session.gateway_v2
    } else {
        // The name-addressed Gateway v1 path needs the intended RID to detect container recreation.
        if let Some(container) = operation
            .container()
            .filter(|container| !container.is_by_rid())
        {
            request.headers.insert(
                request_header_names::INTENDED_COLLECTION_RID,
                HeaderValue::from(container.rid().to_owned()),
            );
        }
        session.gateway
    };

    // Resolved tokens override caller headers; absent tokens leave custom headers intact.
    if let Some(token) = token.filter(|_| {
        // A PATCH result check at the writer must remain independent of session progress.
        !operation.prefers_write_endpoints_for_read() || request.routing_fallback.is_some()
    }) {
        request.headers.insert(
            request_header_names::SESSION_TOKEN,
            HeaderValue::from(token.as_str().to_owned()),
        );
    }
    let endpoint_key = if use_gateway_v2 {
        EndpointKey::try_from(&request.url).expect("selected URL must have a valid host and port")
    } else {
        request.endpoint.endpoint_key()
    };
    (request, endpoint_key)
}
