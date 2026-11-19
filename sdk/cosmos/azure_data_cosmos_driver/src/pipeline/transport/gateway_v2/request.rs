// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Builds the Gateway V2 HTTP envelope around a signed request.

#[cfg(feature = "fault_injection")]
use azure_core::http::headers::HeaderName;
use azure_core::http::{
    headers::{HeaderValue, Headers, USER_AGENT},
    Method,
};

use crate::models::{OperationType, ResourceType};

use super::super::{cosmos_headers::CLIENT_ID, cosmos_transport_client::HttpRequest};
use crate::rntbd_encoding::tokens::RntbdRequestToken;

use super::request_tokens::{self, GatewayV2FrameData};
use super::{
    GatewayV2Headers, CACHE_CONTROL, GATEWAY_V2_DISCOVERY_OPT_IN, GATEWAY_V2_OPERATION_TYPE,
    GATEWAY_V2_RANGE_MAX, GATEWAY_V2_RANGE_MIN, GATEWAY_V2_RESOURCE_TYPE,
    GLOBAL_DATABASE_ACCOUNT_NAME, X_MS_ACTIVITY_ID, X_MS_DOCUMENTDB_COLLECTION_RID, X_MS_VERSION,
};

/// Serializes a signed request as an RNTBD frame inside a Gateway V2 HTTP POST envelope.
pub(crate) fn wrap_request_for_gateway_v2(
    mut request: HttpRequest,
    inputs: &GatewayV2Headers<'_>,
) -> azure_core::Result<HttpRequest> {
    let client_id = request.headers.remove(CLIENT_ID);
    let parts = request_tokens::build_request_frame(&request, inputs)?;
    // Debug-only visibility into exactly which RNTBD metadata tokens are
    // about to go on the wire. Logs token IDs (resolved to their symbolic
    // name where recognized), never values — `AuthorizationToken` carries
    // the request's HMAC signature, which must never be logged. Enable with
    // e.g. `RUST_LOG=azure_data_cosmos_driver=debug`; the field expressions
    // below are only evaluated when the level/target is actually enabled.
    tracing::debug!(
        operation = ?inputs.operation_type,
        resource_type = ?inputs.resource_type,
        tokens = ?parts.frame.metadata
            .iter()
            .map(|token| match RntbdRequestToken::try_from(token.id.value()) {
                Ok(name) => format!("{name:?}"),
                Err(()) => format!("{:#06x}", token.id.value()),
            })
            .collect::<Vec<_>>(),
        "Gateway 2.0 request RNTBD metadata tokens"
    );

    let mut frame_bytes = Vec::new();
    parts.frame.write(&mut frame_bytes)?;
    let headers = build_outer_headers(&request, inputs, &parts, client_id);
    let url = request.url.clone();
    Ok(HttpRequest {
        url,
        method: Method::Post,
        headers,
        body: Some(bytes::Bytes::from(frame_bytes)),
        timeout: request.timeout,
        #[cfg(feature = "fault_injection")]
        evaluation_collector: request.evaluation_collector.clone(),
    })
}

/// Builds the proxy's outer headers from request identity, routing inputs, and frame metadata.
fn build_outer_headers(
    request: &HttpRequest,
    inputs: &GatewayV2Headers<'_>,
    parts: &GatewayV2FrameData<'_>,
    client_id: Option<HeaderValue>,
) -> Headers {
    let mut headers = Headers::new();
    if let Some(user_agent) = request.headers.get_optional_str(&USER_AGENT) {
        headers.insert(USER_AGENT, HeaderValue::from(user_agent.to_owned()));
    }
    if let Some(client_id) = client_id {
        headers.insert(CLIENT_ID, client_id);
    }
    headers.insert(
        X_MS_ACTIVITY_ID,
        HeaderValue::from(parts.frame.activity_id.to_string()),
    );
    // Forward x-ms-version (defaults to CURRENT_VERSION = 2020-07-15)
    // and Cache-Control. The proxy requires x-ms-version to dispatch the request.
    if let Some(version) = request.headers.get_optional_str(&X_MS_VERSION) {
        headers.insert(X_MS_VERSION, HeaderValue::from(version.to_owned()));
    } else {
        headers.insert(X_MS_VERSION, HeaderValue::from("2020-07-15"));
    }
    headers.insert(CACHE_CONTROL, HeaderValue::from("no-cache"));
    headers.insert(GATEWAY_V2_DISCOVERY_OPT_IN, HeaderValue::from("true"));
    headers.insert(
        GATEWAY_V2_OPERATION_TYPE,
        HeaderValue::from(proxy_operation_type_name(inputs.operation_type)),
    );
    headers.insert(
        GATEWAY_V2_RESOURCE_TYPE,
        HeaderValue::from(proxy_resource_type_name(inputs.resource_type)),
    );
    headers.insert(
        GLOBAL_DATABASE_ACCOUNT_NAME,
        HeaderValue::from(parts.account_name.to_owned()),
    );
    if let Some(rid) = inputs.collection_rid.filter(|s| !s.is_empty()) {
        headers.insert(
            X_MS_DOCUMENTDB_COLLECTION_RID,
            HeaderValue::from(rid.to_owned()),
        );
    }
    if inputs.operation_type == OperationType::Query && !parts.emitted_point_epk {
        // For cross-partition Query (no partition-key-scoped EPK), these
        // headers are set to the literal `"true"`
        // to signal the proxy that the EPK boundaries are carried in the
        // RNTBD body as `StartEpkHash`/`EndEpkHash` tokens. For partition-
        // scoped queries (point or prefix EPK on the body), the proxy uses
        // the `EffectivePartitionKey` token directly and these flag headers
        // must NOT be sent — otherwise the proxy looks for absent
        // Start/End EPK hash tokens and rejects the request with a bare 400.
        headers.insert(GATEWAY_V2_RANGE_MIN, HeaderValue::from("true"));
        headers.insert(GATEWAY_V2_RANGE_MAX, HeaderValue::from("true"));
    }
    // The G2 wrap synthesizes a fresh HTTP header set carrying only what the
    // proxy needs (the RNTBD frame is in the body). The fault-injection
    // operation tag must be forwarded so `FaultInjectingHttpClient` can match
    // operation-typed rules against the G2 path the same way it matches V1.
    #[cfg(feature = "fault_injection")]
    {
        use crate::models::cosmos_headers::fault_injection_header_names::{
            FAULT_INJECTION_OPERATION, FAULT_INJECTION_REGION,
        };
        for name in [FAULT_INJECTION_OPERATION, FAULT_INJECTION_REGION] {
            let header = HeaderName::from_static(name);
            if let Some(value) = request.headers.get_optional_str(&header) {
                headers.insert(header, HeaderValue::from(value.to_owned()));
            }
        }
    }

    headers
}

/// Returns the operation name expected by the proxy's operation-type routing header.
fn proxy_operation_type_name(op: OperationType) -> &'static str {
    match op {
        OperationType::Create => "Create",
        OperationType::Read => "Read",
        OperationType::ReadFeed => "ReadFeed",
        OperationType::Replace => "Replace",
        OperationType::Delete => "Delete",
        OperationType::Upsert => "Upsert",
        OperationType::Query => "Query",
        OperationType::SqlQuery => "SqlQuery",
        OperationType::QueryPlan => "QueryPlan",
        OperationType::Batch => "Batch",
        OperationType::Head => "Head",
        OperationType::HeadFeed => "HeadFeed",
        OperationType::Execute => "ExecuteJavaScript",
        OperationType::Patch => "Patch",
        // Distributed transactions never reach Gateway 2.0 dispatch (rejected
        // by `is_operation_supported_by_gateway_v2`); they route through the
        // standard gateway coordinator.
        #[cfg(feature = "unstable_dtx")]
        OperationType::CommitDistributedTransaction | OperationType::ReadDistributedTransaction => {
            unreachable!("distributed transaction operations must not reach Gateway 2.0 dispatch")
        }
    }
}

/// Returns the resource name expected by the proxy's resource-type routing header.
fn proxy_resource_type_name(rt: ResourceType) -> &'static str {
    match rt {
        ResourceType::DatabaseAccount => "DatabaseAccount",
        ResourceType::Database => "Database",
        ResourceType::DocumentCollection => "DocumentCollection",
        ResourceType::Document => "Document",
        ResourceType::StoredProcedure => "StoredProcedure",
        ResourceType::Trigger => "Trigger",
        ResourceType::UserDefinedFunction => "UserDefinedFunction",
        ResourceType::PartitionKeyRange => "PartitionKeyRange",
        ResourceType::Offer => "Offer",
        #[cfg(feature = "unstable_dtx")]
        ResourceType::DistributedTransactionBatch => {
            unreachable!("distributed transaction batch must not reach Gateway 2.0 dispatch")
        }
    }
}
