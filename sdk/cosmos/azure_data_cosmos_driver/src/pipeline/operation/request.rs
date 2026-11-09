// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Prepares transport requests from operation and attempt inputs.

use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

use azure_core::http::headers::{HeaderName, HeaderValue};

use crate::{
    diagnostics::{ExecutionContext, PipelineKind},
    models::{
        cosmos_headers::{PATCH_CONTENT_TYPE, QUERY_CONTENT_TYPE},
        effective_partition_key::EffectivePartitionKey,
        request_header_names, ActivityId, CosmosOperation, DefaultConsistencyLevel, OperationType,
    },
    options::{OperationOptionsView, ReadConsistencyStrategy, ResolvedThroughputControl},
    pipeline::{
        operation::{
            diagnostics,
            recovery::state::OperationRetryState,
            request,
            routing::{AttemptRoute, LocationSnapshot, RoutingDecision},
            session,
            types::OperationOverrides,
            AttemptContext,
        },
        transport::{
            types::{TransportMode, TransportRequest},
            AuthorizationContext,
        },
    },
};

/// Per-attempt overrides applied by the common request path.
pub(crate) struct AttemptOptions<'a> {
    pub execution_context: ExecutionContext,
    pub effective_consistency: DefaultConsistencyLevel,
    pub read_consistency_strategy: ReadConsistencyStrategy,
    pub automatic_session_resolution: bool,
    pub hub_region_processing_only: bool,
    pub hub_region_signal: Option<&'a AtomicBool>,
    pub tentative_writes_enabled: bool,
    /// Prevents a hedge alternate from upgrading beyond its primary's gateway capability.
    /// Dispatch still checks operation, endpoint, and session requirements.
    pub gateway_v2_allowed: bool,
}

/// A prepared request and the hub-header decision used to classify its response.
pub(crate) struct PreparedAttempt {
    pub request: TransportRequest,
    pub sent_hub_header: bool,
}

/// Resolves sequential wire policy from the captured route and current retry progress.
pub(crate) fn sequential_options(
    ctx: &AttemptContext<'_>,
    route: &AttemptRoute,
    state: &OperationRetryState,
    captured: &LocationSnapshot,
) -> AttemptOptions<'static> {
    AttemptOptions {
        execution_context: diagnostics::execution_context(state),
        effective_consistency: route.consistency,
        read_consistency_strategy: session::read_consistency_strategy_for_operation(
            ctx.operation,
            route.read_strategy,
        ),
        automatic_session_resolution: route.automatic_session_resolution,
        hub_region_processing_only: state.hub_region_processing_only,
        hub_region_signal: None,
        tentative_writes_enabled: captured.account.multiple_write_locations_enabled,
        gateway_v2_allowed: true,
    }
}

/// Resolves hedge-leg overrides without inheriting the sequential hub-header latch.
///
/// Only the secondary receives the coordinator's signal. This keeps primary wire
/// behavior stable when a retry is upgraded to a race.
pub(crate) fn hedge_options<'a>(
    ctx: &AttemptContext<'_>,
    primary: &RoutingDecision,
    execution_context: ExecutionContext,
    signal: Option<&'a AtomicBool>,
) -> AttemptOptions<'a> {
    AttemptOptions {
        execution_context,
        effective_consistency: ctx.policy.effective_consistency,
        read_consistency_strategy: ctx.policy.read_consistency_strategy,
        automatic_session_resolution: ctx.policy.session_token_resolution_active,
        hub_region_processing_only: false,
        hub_region_signal: signal,
        tentative_writes_enabled: false,
        gateway_v2_allowed: primary.endpoint.uses_gateway_v2(true),
    }
}

/// Builds the request used by every transport attempt without resolving sessions or choosing a gateway.
///
/// Captures the hub-header decision once so later race observations cannot change
/// how this response is classified. Request preparation never publishes shared state.
pub(crate) fn prepare_attempt(
    ctx: &AttemptContext<'_>,
    route: &RoutingDecision,
    options: &AttemptOptions<'_>,
) -> crate::error::Result<PreparedAttempt> {
    let request_context = TransportRequestContext {
        routing: route,
        activity_id: ctx.config.activity_id,
        execution_context: options.execution_context,
        deadline: ctx.policy.deadline,
        effective_consistency: options.effective_consistency,
        read_consistency_strategy: options.read_consistency_strategy,
        throughput_control: ctx.throughput_control,
    };
    let mut request = self::prepare(
        ctx.operation,
        ctx.overrides,
        ctx.config.custom_headers,
        &request_context,
        ctx.policy.fault_injection_enabled,
    )?;

    let sent_hub_header = self::apply_hub_region_header(
        &mut request,
        options.hub_region_processing_only,
        options.hub_region_signal,
    );
    self::apply_tentative_writes_header(
        &mut request,
        ctx.operation,
        options.tentative_writes_enabled,
    );
    self::apply_optional_request_headers(&mut request, ctx.operation, ctx.config.options);

    Ok(PreparedAttempt {
        request,
        sent_hub_header,
    })
}

/// Parameters resolved per-attempt for building a transport request.
///
/// Groups per-attempt state that varies across retries and failovers,
/// reducing the number of arguments passed to `build_transport_request`.
pub(crate) struct TransportRequestContext<'a> {
    pub(crate) routing: &'a RoutingDecision,
    pub(crate) activity_id: &'a ActivityId,
    pub(crate) execution_context: ExecutionContext,
    pub(crate) deadline: Option<Instant>,
    pub(crate) effective_consistency: DefaultConsistencyLevel,
    /// Raw (uncollapsed) RCS to be emitted on the wire for read operations.
    /// `Default` for non-reads or when caller did not specify an RCS.
    pub(crate) read_consistency_strategy: ReadConsistencyStrategy,
    pub(crate) throughput_control: Option<ResolvedThroughputControl>,
}

/// Builds an operation request for tests without dispatch's gateway-specific adjustments.
#[cfg(test)]
pub(crate) fn build_transport_request(
    operation: &CosmosOperation,
    overrides: &OperationOverrides,
    custom_headers: Option<&std::collections::HashMap<HeaderName, HeaderValue>>,
    ctx: &TransportRequestContext<'_>,
) -> crate::error::Result<TransportRequest> {
    request::prepare(operation, overrides, custom_headers, ctx, false)
}

/// Prepares the regional request URL, payload, and operation headers before gateway selection.
///
/// Returns preparation errors before dispatch. The caller supplies attempt-specific
/// consistency and diagnostics identity; dispatch attaches the resolved session token
/// and gateway-specific headers. No shared state is changed.
pub(crate) fn prepare(
    operation: &CosmosOperation,
    overrides: &OperationOverrides,
    custom_headers: Option<&std::collections::HashMap<HeaderName, HeaderValue>>,
    ctx: &TransportRequestContext<'_>,
    fault_injection_enabled: bool,
) -> crate::error::Result<TransportRequest> {
    let paths = operation.compute_resource_paths();
    let url = {
        let mut base = ctx.routing.endpoint.url().clone();
        let request_path = paths.request_path();
        let normalized = if request_path.starts_with('/') {
            request_path.to_string()
        } else if request_path.is_empty() {
            String::new()
        } else {
            format!("/{}", request_path)
        };
        // Escape literal percent bytes before `Url::set_path`: it preserves valid
        // `%HH` sequences, but Cosmos resource names treat them as literal text.
        // Leave path separators and path-legal characters (`@`, `+`, `=`) intact
        // so RID routing and Gateway 2.0 path comparisons remain unchanged.
        if normalized.contains('%') {
            base.set_path(&normalized.replace('%', "%25"));
        } else {
            base.set_path(&normalized);
        }
        base
    };

    let method = operation.operation_type().http_method();
    let resource_type = operation.resource_type();
    // Move `paths` into AuthorizationContext so the signing link is a zero-copy
    // sub-slice of the path buffer — no additional string allocation needed.
    let auth_context = AuthorizationContext::from_paths(method, resource_type, paths);

    // Build headers from the operation.
    // Custom headers are inserted first so that SDK-set headers below always
    // take precedence on conflicts (matching the SDK's ItemOptions::apply_headers
    // pattern where custom headers are added before SDK headers).
    let mut headers = azure_core::http::headers::Headers::new();
    if let Some(custom) = custom_headers {
        for (name, value) in custom {
            headers.insert(name.clone(), value.clone());
        }
    }
    operation.request_headers().write_to_headers(&mut headers);
    // Add activity ID if not already set by the operation
    if operation.request_headers().activity_id.is_none() {
        headers.insert(
            HeaderName::from_static("x-ms-activity-id"),
            HeaderValue::from(ctx.activity_id.as_str().to_owned()),
        );
    }

    // Apply operation type-specific headers.
    match operation.operation_type() {
        OperationType::Upsert => {
            headers.insert(
                HeaderName::from_static(request_header_names::IS_UPSERT),
                HeaderValue::from_static("true"),
            );
        }
        OperationType::Batch => {
            headers.insert(
                HeaderName::from_static(request_header_names::IS_BATCH_REQUEST),
                HeaderValue::from_static("True"),
            );
            headers.insert(
                HeaderName::from_static(request_header_names::BATCH_ATOMIC),
                HeaderValue::from_static("True"),
            );
            headers.insert(
                HeaderName::from_static(request_header_names::BATCH_CONTINUE_ON_ERROR),
                HeaderValue::from_static("False"),
            );
        }
        OperationType::Patch => {
            headers.insert(
                azure_core::http::headers::CONTENT_TYPE,
                HeaderValue::from_static(PATCH_CONTENT_TYPE),
            );
        }
        OperationType::Query | OperationType::SqlQuery => {
            headers.insert(
                HeaderName::from_static(request_header_names::IS_QUERY),
                HeaderValue::from_static("True"),
            );
            headers.insert(
                azure_core::http::headers::CONTENT_TYPE,
                HeaderValue::from_static(QUERY_CONTENT_TYPE),
            );
            let supported_features_header =
                HeaderName::from_static(request_header_names::SUPPORTED_QUERY_FEATURES);
            if headers
                .get_optional_str(&supported_features_header)
                .is_none()
            {
                headers.insert(
                    supported_features_header,
                    HeaderValue::from_static(crate::query::SUPPORTED_QUERY_FEATURES),
                );
            }
            let query_version_header = HeaderName::from_static(request_header_names::QUERY_VERSION);
            if headers.get_optional_str(&query_version_header).is_none() {
                headers.insert(query_version_header, HeaderValue::from_static("1.0"));
            }
        }
        OperationType::QueryPlan => {
            headers.insert(
                HeaderName::from_static(request_header_names::IS_QUERY),
                HeaderValue::from_static("True"),
            );
            headers.insert(
                azure_core::http::headers::CONTENT_TYPE,
                HeaderValue::from_static(QUERY_CONTENT_TYPE),
            );
            headers.insert(
                HeaderName::from_static(request_header_names::IS_QUERY_PLAN_REQUEST),
                HeaderValue::from_static("True"),
            );
            // These two headers must always be set on QueryPlan
            // requests. The thin-client proxy reads them out of the RNTBD body
            // (mirrored from these HTTP headers in gateway_v2::request_tokens) and rejects
            // requests where they're missing entirely. Default them here when the
            // caller hasn't already set explicit values.
            let supported_features_header =
                HeaderName::from_static(request_header_names::SUPPORTED_QUERY_FEATURES);
            if headers
                .get_optional_str(&supported_features_header)
                .is_none()
            {
                headers.insert(
                    supported_features_header,
                    HeaderValue::from_static(crate::query::SUPPORTED_QUERY_FEATURES),
                );
            }
            let query_version_header = HeaderName::from_static(request_header_names::QUERY_VERSION);
            if headers.get_optional_str(&query_version_header).is_none() {
                headers.insert(query_version_header, HeaderValue::from_static("1.0"));
            }
        }
        _ => {}
    }

    // Add operation type header for fault injection rule matching
    #[cfg(feature = "fault_injection")]
    if fault_injection_enabled {
        if let Some(fault_op) =
            crate::fault_injection::FaultOperationType::from_operation_and_resource(
                &operation.operation_type(),
                &operation.resource_type(),
            )
        {
            crate::pipeline::transport::cosmos_headers::apply_fault_injection_operation_tag(
                &mut headers,
                fault_op,
            );
        }
        if let Some(region) = ctx.routing.endpoint.region() {
            headers.insert(
                crate::models::cosmos_headers::fault_injection_header_names::FAULT_INJECTION_REGION,
                HeaderValue::from(region.as_str().to_owned()),
            );
        }
    }
    #[cfg(not(feature = "fault_injection"))]
    let _ = fault_injection_enabled;

    // Apply overrides — these take precedence over operation-level headers
    // (e.g., an override partition key replaces the operation's partition key).
    // Change feed reads carry their continuation via `If-None-Match`, not
    // `x-ms-continuation`.
    overrides.apply_headers(&mut headers, operation.is_change_feed())?;

    let effective_partition_key = request::effective_partition_key_for_request(operation)?;
    let effective_partition_key = if overrides.partition_key_range_id.is_some() {
        // Range targets already preserve the logical scope in their EPK bounds.
        headers.remove(HeaderName::from_static(request_header_names::PARTITION_KEY));
        None
    } else {
        effective_partition_key
    };

    // Add throughput control headers, if any layer (direct override or
    // resolved group) produced a value.
    if let Some(throughput_control) = ctx.throughput_control {
        if let Some(priority) = throughput_control.priority_level {
            headers.insert(
                request_header_names::PRIORITY_LEVEL,
                HeaderValue::from(priority.as_str().to_owned()),
            );
        }
        if let Some(bucket) = throughput_control.throughput_bucket {
            headers.insert(
                request_header_names::THROUGHPUT_BUCKET,
                HeaderValue::from(bucket.to_string()),
            );
        }
    }

    Ok(TransportRequest {
        method,
        endpoint: ctx.routing.endpoint.clone(),
        transport_mode: TransportMode::Gateway,
        operation_type: operation.operation_type(),
        effective_partition_key,
        effective_consistency: ctx.effective_consistency,
        read_consistency_strategy: ctx.read_consistency_strategy,
        url,
        headers,
        #[cfg(feature = "unstable_dtx")]
        resource_type,
        body: operation.body().map(azure_core::Bytes::copy_from_slice),
        auth_context,
        execution_context: ctx.execution_context,
        routing_fallback: ctx.routing.routing_fallback,
        deadline: ctx.deadline.into(),
    })
}

/// Computes the effective partition key for an item-scoped request before the
/// transport pipeline runs, so wire layers receive a ready-to-encode EPK rather
/// than raw partition-key/definition values.
///
/// Returns `Ok(None)` when the operation has no partition key, no container
/// context, or an empty partition key. Surfaces a `BadRequest` when the caller
/// supplies more components than the container's partition-key definition
/// declares (the hash routines would otherwise silently hash the extras into a
/// broken EPK).
pub(crate) fn effective_partition_key_for_request(
    operation: &CosmosOperation,
) -> crate::error::Result<Option<EffectivePartitionKey>> {
    let (Some(partition_key), Some(container)) = (operation.partition_key(), operation.container())
    else {
        return Ok(None);
    };

    if partition_key.is_empty() {
        return Ok(None);
    }

    let partition_key_definition = container.partition_key_definition();
    if partition_key.values().len() > partition_key_definition.paths().len() {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::status_codes::CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS)
            .with_message(
                "Partition key supplies more components than the container's \
                 partition-key definition declares",
            )
            .build());
    }

    if operation.is_change_feed() && !partition_key_definition.is_complete(partition_key) {
        return Ok(None);
    }

    Ok(Some(EffectivePartitionKey::compute(
        partition_key.values(),
        partition_key_definition.kind(),
        partition_key_definition.version(),
    )))
}

/// Adds the hub-only header when the sequential operation's owned retry state requires it.
pub(crate) fn apply_hub_region_header(
    transport_request: &mut TransportRequest,
    latched: bool,
    signal: Option<&AtomicBool>,
) -> bool {
    let emit = request::should_emit_hub_region_header(latched, signal);
    if emit {
        transport_request.headers.insert(
            HeaderName::from_static(request_header_names::HUB_REGION_PROCESSING_ONLY),
            HeaderValue::from_static("True"),
        );
    }
    emit
}

/// Reads the request's hub-only decision without changing either signal.
///
/// A hedge secondary observes a race-local signal published by its coordinator.
/// Callers must retain this decision rather than reread the signal when classifying a response.
pub(crate) fn should_emit_hub_region_header(
    per_state_latched: bool,
    shared_latch: Option<&AtomicBool>,
) -> bool {
    per_state_latched || shared_latch.is_some_and(|s| s.load(Ordering::Acquire))
}

/// Enables a race-local hub signal only for single-master data-plane operations.
pub(crate) fn should_build_shared_hub_region_latch(
    pipeline_type: PipelineKind,
    can_use_multiple_write_locations: bool,
) -> bool {
    pipeline_type.is_data_plane() && !can_use_multiple_write_locations
}

/// Emits `x-ms-cosmos-allow-tentative-writes: true` on writes when the account
/// is multi-write. Without this header, satellite write regions reject writes
/// with `403 / 3 (WriteForbidden)` because the request is treated as
/// single-master traffic to a non-primary region. The header is gated on
/// `enableMultipleWriteLocations` from the account metadata.
pub(crate) fn apply_tentative_writes_header(
    transport_request: &mut TransportRequest,
    operation: &CosmosOperation,
    multiple_write_locations_enabled: bool,
) {
    if multiple_write_locations_enabled && !operation.is_read_only() {
        transport_request.headers.insert(
            HeaderName::from_static(request_header_names::ALLOW_TENTATIVE_WRITES),
            HeaderValue::from_static("true"),
        );
    }
}

/// Applies operation-options-driven request headers that are only known
/// after the request has been built by `build_transport_request`.
///
/// Two concerns are layered here, in this order:
///
/// 1. **content-response-on-write**: For non-read operations whose
///    `content_response_on_write` is `None` (default) or `Disabled`,
///    inject `Prefer: return=minimal` so the service suppresses the
///    response body. Reads always need the body and are unaffected.
/// 2. **custom_headers**: Pass-through of caller-supplied headers, but
///    only when no SDK-set header with the same name already exists —
///    SDK headers always take precedence.
pub(crate) fn apply_optional_request_headers(
    transport_request: &mut TransportRequest,
    operation: &CosmosOperation,
    options: &OperationOptionsView<'_>,
) {
    let content_response_enabled = !operation.operation_type().is_read_only()
        && matches!(
            options.content_response_on_write(),
            Some(&crate::options::ContentResponseOnWrite::Enabled)
        );
    if !operation.operation_type().is_read_only() && !content_response_enabled {
        transport_request.headers.insert(
            request_header_names::PREFER,
            HeaderValue::from_static("return=minimal"),
        );
    }

    if let Some(custom_headers) = options.custom_headers() {
        for (name, value) in custom_headers {
            if !transport_request.headers.iter().any(|(n, _)| n == name) {
                transport_request
                    .headers
                    .insert(name.clone(), value.clone());
            }
        }
    }

    if content_response_enabled {
        transport_request
            .headers
            .remove(request_header_names::PREFER);
    }

    if operation.prefers_write_endpoints_for_read() && transport_request.routing_fallback.is_none()
    {
        // LatestCommitted is not session-effective. Enforce that invariant
        // after every custom-header layer so callers cannot reintroduce a
        // session token behind the operation pipeline's resolution logic.
        transport_request
            .headers
            .remove(request_header_names::SESSION_TOKEN);
    }
}
