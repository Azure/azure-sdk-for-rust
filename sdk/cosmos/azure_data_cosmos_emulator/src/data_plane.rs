// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{io, str::FromStr, sync::Arc};

use axum::{
    body::{to_bytes, Body},
    extract::{Request, State},
    http::{HeaderName, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    routing::any,
    Json, Router,
};
use azure_core::http::{Method, Request as CosmosRequest};
use azure_data_cosmos_driver::{
    in_memory_emulator::InMemoryEmulatorHttpClient, options::ReadConsistencyStrategy,
};
use serde_json::json;
use tokio::net::TcpListener;
use url::Url;

use crate::{config::GatewayBinding, metrics::HostMetrics};

const MAX_REQUEST_BODY_SIZE: usize = 16 * 1024 * 1024;

#[derive(Clone)]
struct GatewayState {
    emulator: Arc<InMemoryEmulatorHttpClient>,
    base_url: Url,
    metrics: Arc<HostMetrics>,
}

pub(crate) async fn serve(
    listener: TcpListener,
    binding: GatewayBinding,
    emulator: Arc<InMemoryEmulatorHttpClient>,
    metrics: Arc<HostMetrics>,
) -> io::Result<()> {
    tracing::info!(
        region = binding.region_name,
        endpoint = %binding.gateway_url,
        "Cosmos gateway listener ready"
    );
    let router = router_with_metrics(emulator, binding.gateway_url, metrics);
    axum::serve(listener, router).await
}

#[cfg(test)]
pub(crate) fn router(emulator: Arc<InMemoryEmulatorHttpClient>, base_url: Url) -> Router {
    router_with_metrics(emulator, base_url, Arc::new(HostMetrics::default()))
}

fn router_with_metrics(
    emulator: Arc<InMemoryEmulatorHttpClient>,
    base_url: Url,
    metrics: Arc<HostMetrics>,
) -> Router {
    Router::new()
        .fallback(any(dispatch))
        .with_state(GatewayState {
            emulator,
            base_url,
            metrics,
        })
}

async fn dispatch(State(state): State<GatewayState>, request: Request) -> Response<Body> {
    match execute(state, request).await {
        Ok(response) => response,
        Err((status, message)) => (status, Json(json!({ "error": message }))).into_response(),
    }
}

async fn execute(
    state: GatewayState,
    request: Request,
) -> Result<Response<Body>, (StatusCode, String)> {
    let (cosmos_request, audit) = into_cosmos_request_audited(request, &state.base_url).await?;
    let response = state
        .emulator
        .execute_request(&cosmos_request)
        .await
        .map_err(internal_error)?;
    // Health counters describe requests for which the host constructed a
    // complete HTTP response. Decode, dispatch, and response-conversion
    // failures are deliberately excluded on both gateway paths.
    let (response, binary_response_payload) = into_http_response_audited(response).await?;
    state
        .metrics
        .record_binary_request(audit.binary_negotiated, audit.binary_request_payload);
    state
        .metrics
        .record_binary_response(binary_response_payload);
    state
        .metrics
        .record_read_consistency_strategy(audit.read_consistency_strategy);
    state.metrics.record_gateway_request();
    Ok(response)
}

pub(crate) async fn into_cosmos_request(
    request: Request,
    base_url: &Url,
) -> Result<CosmosRequest, (StatusCode, String)> {
    into_cosmos_request_audited(request, base_url)
        .await
        .map(|(request, _)| request)
}

struct GatewayRequestAudit {
    binary_negotiated: bool,
    binary_request_payload: bool,
    read_consistency_strategy: Option<ReadConsistencyStrategy>,
}

async fn into_cosmos_request_audited(
    request: Request,
    base_url: &Url,
) -> Result<(CosmosRequest, GatewayRequestAudit), (StatusCode, String)> {
    let (parts, body) = request.into_parts();
    let method = parts
        .method
        .as_str()
        .parse::<Method>()
        .map_err(|error| (StatusCode::METHOD_NOT_ALLOWED, error.to_string()))?;
    let path_and_query = parts
        .uri
        .path_and_query()
        .map(|value| value.as_str())
        .unwrap_or("/");
    if path_and_query.starts_with("//") {
        return Err((
            StatusCode::BAD_REQUEST,
            "request paths must contain exactly one leading slash".to_owned(),
        ));
    }
    let relative_path = path_and_query.strip_prefix('/').unwrap_or(path_and_query);
    let url = base_url.join(relative_path).map_err(internal_error)?;
    let bytes = to_bytes(body, MAX_REQUEST_BODY_SIZE)
        .await
        .map_err(|error| (StatusCode::PAYLOAD_TOO_LARGE, error.to_string()))?;
    let audit = GatewayRequestAudit {
        binary_negotiated: parts
            .headers
            .get("x-ms-cosmos-supported-serialization-formats")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| {
                value
                    .split(',')
                    .any(|format| format.trim().eq_ignore_ascii_case("CosmosBinary"))
            }),
        binary_request_payload: bytes.first() == Some(&0x80),
        read_consistency_strategy: parts
            .headers
            .get("x-ms-cosmos-read-consistency-strategy")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| ReadConsistencyStrategy::from_str(value).ok()),
    };

    let mut cosmos_request = CosmosRequest::new(url, method);
    // `azure_core::http::headers::Headers` is backed by a map keyed on header
    // name, so a repeated header name here collapses to its last value —
    // matching `into_http_response`'s output below, which can only ever
    // iterate a `Headers` map and therefore can never observe (or forward)
    // more than one value per name either. Both directions of this bridge
    // are consistently single-valued-per-header-name.
    for (name, value) in &parts.headers {
        let value = value
            .to_str()
            .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?;
        cosmos_request
            .headers_mut()
            .insert(name.as_str().to_owned(), value.to_owned());
    }
    if !bytes.is_empty() {
        cosmos_request.set_body(bytes.to_vec());
    }
    Ok((cosmos_request, audit))
}

pub(crate) async fn into_http_response(
    response: azure_core::http::AsyncRawResponse,
) -> Result<Response<Body>, (StatusCode, String)> {
    into_http_response_audited(response)
        .await
        .map(|(response, _)| response)
}

async fn into_http_response_audited(
    response: azure_core::http::AsyncRawResponse,
) -> Result<(Response<Body>, bool), (StatusCode, String)> {
    let response = response
        .try_into_raw_response()
        .await
        .map_err(internal_error)?;
    let binary_response_payload = response.body().first() == Some(&0x80);

    let mut builder = Response::builder().status(u16::from(response.status()));
    for (name, value) in response.headers().iter() {
        let name = HeaderName::from_bytes(name.as_str().as_bytes()).map_err(internal_error)?;
        let value = HeaderValue::from_str(value.as_str()).map_err(internal_error)?;
        builder = builder.header(name, value);
    }
    let response = builder
        .body(Body::from(response.body().as_ref().to_vec()))
        .map_err(internal_error)?;
    Ok((response, binary_response_payload))
}

pub(crate) fn internal_error(error: impl std::fmt::Display) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_data_cosmos_driver::in_memory_emulator::{VirtualAccountConfig, VirtualRegion};

    #[tokio::test]
    async fn account_read_flows_through_http_bridge() {
        let base_url = Url::parse("http://127.0.0.1:18081/").unwrap();
        let config =
            VirtualAccountConfig::new(vec![VirtualRegion::new("East US", base_url.clone())])
                .unwrap();
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
        let request = Request::builder().uri("/").body(Body::empty()).unwrap();

        let response = execute(
            GatewayState {
                emulator,
                base_url,
                metrics: Arc::new(HostMetrics::default()),
            },
            request,
        )
        .await
        .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn rejects_multi_slash_paths_without_replacing_authority() {
        let base_url = Url::parse("http://127.0.0.1:18081/").unwrap();
        let request = Request::builder()
            .uri("//attacker.example/dbs")
            .body(Body::empty())
            .unwrap();

        let error = into_cosmos_request(request, &base_url).await.unwrap_err();
        assert_eq!(error.0, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn decode_failures_do_not_increment_completed_operation_counters() {
        let base_url = Url::parse("http://127.0.0.1:18081/").unwrap();
        let config =
            VirtualAccountConfig::new(vec![VirtualRegion::new("East US", base_url.clone())])
                .unwrap();
        let metrics = Arc::new(HostMetrics::default());
        let request = Request::builder()
            .uri("//invalid.example/dbs")
            .header(
                "x-ms-cosmos-supported-serialization-formats",
                "CosmosBinary",
            )
            .header("x-ms-cosmos-read-consistency-strategy", "Session")
            .body(Body::from(vec![0x80]))
            .unwrap();

        let response = dispatch(
            State(GatewayState {
                emulator: Arc::new(InMemoryEmulatorHttpClient::new(config)),
                base_url,
                metrics: Arc::clone(&metrics),
            }),
            request,
        )
        .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(metrics.gateway_requests(), 0);
        assert_eq!(metrics.binary_negotiated_requests(), 0);
        assert_eq!(metrics.binary_payload_requests(), 0);
        assert_eq!(metrics.wire_session_consistency_requests(), 0);
    }
}
