// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{io, sync::Arc};

#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};

use axum::{
    body::Body,
    extract::{Request, State},
    http::{Response, StatusCode, Version},
    response::IntoResponse,
    routing::any,
    Json, Router,
};
use azure_data_cosmos_driver::in_memory_emulator::InMemoryEmulatorHttpClient;
use serde_json::json;
use tokio::net::TcpListener;
use url::Url;

use crate::{data_plane, metrics::HostMetrics};

#[derive(Clone)]
struct GatewayV2State {
    emulator: Arc<InMemoryEmulatorHttpClient>,
    base_url: Url,
    metrics: Arc<HostMetrics>,
    #[cfg(test)]
    request_count: Option<Arc<AtomicUsize>>,
    #[cfg(test)]
    probe_count: Option<Arc<AtomicUsize>>,
}

pub(crate) async fn serve(
    listener: TcpListener,
    region_name: String,
    base_url: Url,
    emulator: Arc<InMemoryEmulatorHttpClient>,
    metrics: Arc<HostMetrics>,
) -> io::Result<()> {
    tracing::info!(
        region = region_name,
        endpoint = %base_url,
        "Cosmos Gateway 2.0 listener ready"
    );
    axum::serve(listener, router(emulator, base_url, metrics)).await
}

fn router(
    emulator: Arc<InMemoryEmulatorHttpClient>,
    base_url: Url,
    metrics: Arc<HostMetrics>,
) -> Router {
    Router::new()
        .fallback(any(dispatch))
        .with_state(GatewayV2State {
            emulator,
            base_url,
            metrics,
            #[cfg(test)]
            request_count: None,
            #[cfg(test)]
            probe_count: None,
        })
}

async fn dispatch(State(state): State<GatewayV2State>, request: Request) -> Response<Body> {
    match execute(state, request).await {
        Ok(response) => response,
        Err((status, message)) => {
            tracing::error!(status = %status, error = %message, "Gateway 2.0 request failed");
            (status, Json(json!({ "error": message }))).into_response()
        }
    }
}

async fn execute(
    state: GatewayV2State,
    request: Request,
) -> Result<Response<Body>, (StatusCode, String)> {
    if request.version() != Version::HTTP_2 {
        return Err((
            StatusCode::HTTP_VERSION_NOT_SUPPORTED,
            "Gateway 2.0 requires HTTP/2".to_owned(),
        ));
    }
    if request.method() != axum::http::Method::POST {
        return Err((
            StatusCode::METHOD_NOT_ALLOWED,
            "Gateway 2.0 requires POST".to_owned(),
        ));
    }
    if request.uri().path() == "/connectivity-probe" {
        state.metrics.record_connectivity_probe();
        #[cfg(test)]
        if let Some(probe_count) = &state.probe_count {
            probe_count.fetch_add(1, Ordering::SeqCst);
        }
        return Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .map_err(data_plane::internal_error);
    }
    let request = data_plane::into_cosmos_request(request, &state.base_url).await?;
    let response = state
        .emulator
        .execute_gateway_v2_request(&request)
        .await
        .map_err(|error| {
            let status = StatusCode::from_u16(u16::from(error.status().status_code()))
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            (status, error.to_string())
        })?;
    let response = data_plane::into_http_response(response).await?;
    state.metrics.record_gateway20_request();
    #[cfg(test)]
    if let Some(request_count) = &state.request_count {
        request_count.fetch_add(1, Ordering::SeqCst);
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::{headers::HeaderValue, Method, Request as CosmosRequest};
    use azure_data_cosmos_driver::{
        driver::CosmosDriverRuntime,
        in_memory_emulator::{ContainerConfig, VirtualAccountConfig, VirtualRegion},
        models::{
            AccountReference, CosmosOperation, ItemReference, PartitionKey, PartitionKeyDefinition,
        },
        options::{DriverOptions, OperationOptions},
    };

    #[tokio::test]
    async fn connectivity_probe_requires_http2_and_returns_ok() {
        let base_url = Url::parse("http://127.0.0.1:18444/").unwrap();
        let region = VirtualRegion::new("East US", Url::parse("http://127.0.0.1:18081/").unwrap())
            .with_gateway_v2_url(base_url.clone());
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(
            VirtualAccountConfig::new(vec![region]).unwrap(),
        ));
        let state = GatewayV2State {
            emulator,
            base_url,
            metrics: Arc::new(HostMetrics::default()),
            request_count: None,
            probe_count: None,
        };

        let mut request = Request::builder()
            .method("POST")
            .uri("/connectivity-probe")
            .body(Body::empty())
            .unwrap();
        *request.version_mut() = Version::HTTP_2;
        let response = execute(state.clone(), request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let request = Request::builder()
            .method("POST")
            .uri("/connectivity-probe")
            .body(Body::empty())
            .unwrap();
        let error = execute(state, request).await.unwrap_err();
        assert_eq!(error.0, StatusCode::HTTP_VERSION_NOT_SUPPORTED);
    }

    #[tokio::test]
    async fn data_requests_require_post() {
        let base_url = Url::parse("http://127.0.0.1:18444/").unwrap();
        let region = VirtualRegion::new("East US", Url::parse("http://127.0.0.1:18081/").unwrap())
            .with_gateway_v2_url(base_url.clone());
        let state = GatewayV2State {
            emulator: Arc::new(InMemoryEmulatorHttpClient::new(
                VirtualAccountConfig::new(vec![region]).unwrap(),
            )),
            base_url,
            metrics: Arc::new(HostMetrics::default()),
            request_count: None,
            probe_count: None,
        };

        for method in ["GET", "PUT", "DELETE"] {
            let mut request = Request::builder()
                .method(method)
                .uri("/")
                .body(Body::empty())
                .unwrap();
            *request.version_mut() = Version::HTTP_2;
            let error = execute(state.clone(), request).await.unwrap_err();
            assert_eq!(error.0, StatusCode::METHOD_NOT_ALLOWED);
        }
    }

    #[tokio::test]
    async fn real_driver_uses_hosted_gateway20_over_h2c() {
        let gateway_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let thin_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let gateway_url = Url::parse(&format!(
            "http://{}/",
            gateway_listener.local_addr().unwrap()
        ))
        .unwrap();
        let thin_url =
            Url::parse(&format!("http://{}/", thin_listener.local_addr().unwrap())).unwrap();
        let region = VirtualRegion::new("East US", gateway_url.clone())
            .with_gateway_v2_url(thin_url.clone());
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(
            VirtualAccountConfig::new(vec![region]).unwrap(),
        ));
        let store = emulator.store();
        store.create_database("db");
        let partition_key: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
            "paths": ["/pk"], "kind": "Hash", "version": 2
        }))
        .unwrap();
        store.create_container_with_config(
            "db",
            "coll",
            partition_key,
            ContainerConfig::new()
                .with_partition_count(1)
                .build()
                .unwrap(),
        );
        let mut seed = CosmosRequest::new(
            gateway_url.join("dbs/db/colls/coll/docs").unwrap(),
            Method::Post,
        );
        seed.headers_mut().insert(
            "x-ms-documentdb-partitionkey",
            HeaderValue::from_static(r#"["pk1"]"#),
        );
        seed.set_body(
            serde_json::to_vec(&serde_json::json!({
                "id": "item1", "pk": "pk1", "value": 42
            }))
            .unwrap(),
        );
        assert!(emulator
            .execute_request(&seed)
            .await
            .unwrap()
            .status()
            .is_success());

        let gateway_router = data_plane::router(emulator.clone(), gateway_url.clone());
        let gateway_task =
            tokio::spawn(async move { axum::serve(gateway_listener, gateway_router).await });
        let request_count = Arc::new(AtomicUsize::new(0));
        let probe_count = Arc::new(AtomicUsize::new(0));
        let thin_state = GatewayV2State {
            emulator,
            base_url: thin_url,
            metrics: Arc::new(HostMetrics::default()),
            request_count: Some(request_count.clone()),
            probe_count: Some(probe_count.clone()),
        };
        let thin_task = tokio::spawn(async move {
            axum::serve(
                thin_listener,
                Router::new().fallback(any(dispatch)).with_state(thin_state),
            )
            .await
        });

        let runtime = CosmosDriverRuntime::builder().build().await.unwrap();
        let account = AccountReference::with_master_key(
            gateway_url,
            "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==",
        );
        let driver = runtime
            .create_driver(DriverOptions::builder(account).build())
            .await
            .unwrap();
        let container = driver.resolve_container("db", "coll").await.unwrap();
        let item =
            ItemReference::from_name(&container, PartitionKey::from("pk1"), "item1".to_owned());
        let response = driver
            .execute_singleton_operation(
                CosmosOperation::read_item(item),
                OperationOptions::default(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status().status_code(),
            azure_core::http::StatusCode::Ok
        );
        assert!(
            probe_count.load(Ordering::SeqCst) > 0,
            "the driver must probe the advertised Gateway 2.0 endpoint"
        );
        assert!(
            request_count.load(Ordering::SeqCst) > 0,
            "the point read must reach the Gateway 2.0 listener"
        );

        gateway_task.abort();
        thin_task.abort();
    }
}
