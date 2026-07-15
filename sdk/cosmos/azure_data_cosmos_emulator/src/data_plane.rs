// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{io, sync::Arc};

use axum::{
    body::{to_bytes, Body},
    extract::{Request, State},
    http::{HeaderName, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    routing::any,
    Json, Router,
};
use azure_core::http::{Method, Request as CosmosRequest};
use azure_data_cosmos_driver::in_memory_emulator::InMemoryEmulatorHttpClient;
use serde_json::json;
use tokio::net::TcpListener;
use url::Url;

use crate::config::GatewayBinding;

const MAX_REQUEST_BODY_SIZE: usize = 16 * 1024 * 1024;

#[derive(Clone)]
struct GatewayState {
    emulator: Arc<InMemoryEmulatorHttpClient>,
    base_url: Url,
}

pub(crate) async fn serve(
    listener: TcpListener,
    binding: GatewayBinding,
    emulator: Arc<InMemoryEmulatorHttpClient>,
) -> io::Result<()> {
    tracing::info!(
        region = binding.region_name,
        endpoint = %binding.gateway_url,
        "Cosmos gateway listener ready"
    );
    let router = router(emulator, binding.gateway_url);
    axum::serve(listener, router).await
}

pub(crate) fn router(emulator: Arc<InMemoryEmulatorHttpClient>, base_url: Url) -> Router {
    Router::new()
        .fallback(any(dispatch))
        .with_state(GatewayState { emulator, base_url })
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
    let cosmos_request = into_cosmos_request(request, &state.base_url).await?;
    let response = state
        .emulator
        .execute_request(&cosmos_request)
        .await
        .map_err(internal_error)?;
    into_http_response(response).await
}

pub(crate) async fn into_cosmos_request(
    request: Request,
    base_url: &Url,
) -> Result<CosmosRequest, (StatusCode, String)> {
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
    let url = base_url
        .join(path_and_query.trim_start_matches('/'))
        .map_err(internal_error)?;
    let bytes = to_bytes(body, MAX_REQUEST_BODY_SIZE)
        .await
        .map_err(|error| (StatusCode::PAYLOAD_TOO_LARGE, error.to_string()))?;

    let mut cosmos_request = CosmosRequest::new(url, method);
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
    Ok(cosmos_request)
}

pub(crate) async fn into_http_response(
    response: azure_core::http::AsyncRawResponse,
) -> Result<Response<Body>, (StatusCode, String)> {
    let response = response
        .try_into_raw_response()
        .await
        .map_err(internal_error)?;

    let mut builder = Response::builder().status(u16::from(response.status()));
    for (name, value) in response.headers().iter() {
        let name = HeaderName::from_bytes(name.as_str().as_bytes()).map_err(internal_error)?;
        let value = HeaderValue::from_str(value.as_str()).map_err(internal_error)?;
        builder = builder.header(name, value);
    }
    builder
        .body(Body::from(response.body().as_ref().to_vec()))
        .map_err(internal_error)
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

        let response = execute(GatewayState { emulator, base_url }, request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
