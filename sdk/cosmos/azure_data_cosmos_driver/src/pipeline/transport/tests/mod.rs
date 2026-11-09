// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use async_trait::async_trait;

use super::{Clock, TransportPipelineConfig, TransportPipelineServices};
use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, ExecutionContext, PipelineKind, RequestSentStatus,
        TransportSecurity,
    },
    driver::error_chain_summary as format_transport_error_details_cosmos,
    models::{
        ActivityId, CosmosStatus, Credential, DefaultConsistencyLevel, OperationType, ResourceType,
    },
    options::DiagnosticsOptions,
    pipeline::{
        operation::routing::CosmosEndpoint,
        transport::{
            adaptive_transport::AdaptiveTransport,
            cosmos_headers::NO_RETRY_449,
            cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError},
            http_client_factory::{HttpClientConfig, HttpClientFactory},
            recovery::{
                should_retry_connectivity_failure, ThrottleRetryPolicy,
                MAX_LOCAL_CONNECTIVITY_RETRIES, METADATA_MAX_PER_RETRY_DELAY,
            },
            types::{TransportMode, TransportOutcome, TransportRequest, TransportResult},
            EndpointKey,
        },
    },
};

struct PipelineTestContext<'a> {
    transport: &'a AdaptiveTransport,
    clock: Clock,
    credential: &'a Credential,
    allow_sent_transport_retry: bool,
    user_agent: &'a azure_core::http::headers::HeaderValue,
    client_id: &'a azure_core::http::headers::HeaderValue,
    pipeline_type: PipelineKind,
    transport_security: TransportSecurity,
    endpoint_key: EndpointKey,
    account_name: Option<String>,
    collection_rid: Option<String>,
    max_throttle_attempts: u32,
    max_throttle_wait_time: Duration,
    max_throttle_per_retry_delay: Duration,
}

async fn run_pipeline(
    request: TransportRequest,
    context: &PipelineTestContext<'_>,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult {
    super::execute(
        request,
        &TransportPipelineConfig {
            allow_sent_transport_retry: context.allow_sent_transport_retry,
            user_agent: context.user_agent,
            client_id: context.client_id,
            pipeline_type: context.pipeline_type,
            transport_security: context.transport_security,
            endpoint_key: context.endpoint_key.clone(),
            account_name: context.account_name.clone(),
            collection_rid: context.collection_rid.clone(),
            throttle_policy: ThrottleRetryPolicy::with_limits(
                context.max_throttle_attempts,
                context.max_throttle_wait_time,
                context.max_throttle_per_retry_delay,
            ),
        },
        &TransportPipelineServices {
            transport: context.transport,
            credential: context.credential,
            clock: context.clock.clone(),
        },
        diagnostics,
    )
    .await
}

static TEST_CLIENT_ID: azure_core::http::headers::HeaderValue =
    azure_core::http::headers::HeaderValue::from_static("00000000-0000-4000-8000-000000000000");
static TEST_CLIENT_ID_HEADER: azure_core::http::headers::HeaderName =
    azure_core::http::headers::HeaderName::from_static("x-ms-client-id");

fn connectivity_failure(request_sent: RequestSentStatus) -> TransportResult {
    TransportResult {
        outcome: TransportOutcome::TransportError {
            status: crate::error::status_codes::TRANSPORT_IO_FAILED,
            error: crate::error::CosmosError::builder()
                .with_status(crate::error::status_codes::TRANSPORT_IO_FAILED)
                .with_message("connection reset")
                .build(),
            request_sent,
        },
    }
}

#[derive(Debug)]
struct HangingTransportClient;

#[derive(Debug)]
struct SuccessfulTransportClient;

#[async_trait]
impl TransportClient for SuccessfulTransportClient {
    async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        Ok(HttpResponse {
            status: 200,
            headers: azure_core::http::headers::Headers::new(),
            body: Vec::new(),
        })
    }
}

#[async_trait]
impl TransportClient for HangingTransportClient {
    async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        futures::future::pending().await
    }
}

#[derive(Debug, Default)]
struct ClientIdRecordingRetryTransport {
    attempts: std::sync::atomic::AtomicUsize,
    client_ids: Mutex<Vec<Option<String>>>,
}

#[async_trait]
impl TransportClient for ClientIdRecordingRetryTransport {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        self.client_ids.lock().unwrap().push(
            request
                .headers
                .get_optional_str(&TEST_CLIENT_ID_HEADER)
                .map(str::to_owned),
        );

        if self
            .attempts
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            == 0
        {
            let mut headers = azure_core::http::headers::Headers::new();
            headers.insert("x-ms-retry-after-ms", "0");
            Ok(HttpResponse {
                status: 429,
                headers,
                body: Vec::new(),
            })
        } else {
            Ok(HttpResponse {
                status: 200,
                headers: azure_core::http::headers::Headers::new(),
                body: Vec::new(),
            })
        }
    }
}

/// Always returns an HTTP 429 response and counts how many times it was
/// invoked. Used by the end-to-end `transport::execute` tests that
/// need to drive the throttle-retry loop without standing up a real
/// service.
#[derive(Debug)]
struct AlwaysThrottlesTransportClient {
    request_count: Arc<std::sync::atomic::AtomicUsize>,
}

#[async_trait]
impl TransportClient for AlwaysThrottlesTransportClient {
    async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        self.request_count
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(HttpResponse {
            status: 429,
            headers: azure_core::http::headers::Headers::new(),
            body: vec![],
        })
    }
}

/// Always returns HTTP 429 with `x-ms-retry-after-ms: 0`, counting each
/// invocation. The zero retry-after forces every throttle delay to `0`, so
/// the *attempt count* (not the cumulative-wait budget) is the sole limiter
/// — keeping the large class-default budgets fast and their wire counts
/// deterministic, unlike `AlwaysThrottlesTransportClient` (whose exponential
/// fallback only stays cheap for small attempt counts).
#[derive(Debug)]
struct AlwaysThrottlesZeroDelayClient {
    request_count: Arc<std::sync::atomic::AtomicUsize>,
}

#[async_trait]
impl TransportClient for AlwaysThrottlesZeroDelayClient {
    async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        self.request_count
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut headers = azure_core::http::headers::Headers::new();
        headers.insert(
            azure_core::http::headers::HeaderName::from_static("x-ms-retry-after-ms"),
            azure_core::http::headers::HeaderValue::from_static("0"),
        );
        Ok(HttpResponse {
            status: 429,
            headers,
            body: vec![],
        })
    }
}

#[derive(Debug)]
struct ScriptedTransportClient {
    status: CosmosStatus,
    message: &'static str,
}

#[async_trait]
impl TransportClient for ScriptedTransportClient {
    async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        Err(TransportError::new(
            crate::error::CosmosError::builder()
                .with_status(self.status)
                .with_message(self.message)
                .build(),
            crate::diagnostics::RequestSentStatus::Unknown,
        ))
    }
}

#[derive(Debug)]
struct ScriptedFactory {
    clients: Mutex<Vec<Arc<dyn TransportClient>>>,
}

impl ScriptedFactory {
    fn new(clients: Vec<Arc<dyn TransportClient>>) -> Self {
        Self {
            clients: Mutex::new(clients.into_iter().rev().collect()),
        }
    }
}

impl HttpClientFactory for ScriptedFactory {
    fn build(
        &self,
        _connection_pool: &crate::options::ConnectionPoolOptions,
        _config: HttpClientConfig,
    ) -> crate::error::Result<Arc<dyn TransportClient>> {
        self.clients.lock().unwrap().pop().ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("no scripted client available")
                .build()
        })
    }
}

fn scripted_transport(
    status_a: CosmosStatus,
    message_a: &'static str,
    status_b: CosmosStatus,
    message_b: &'static str,
) -> AdaptiveTransport {
    let pool = crate::options::ConnectionPoolOptions::builder()
        .with_max_http2_streams_per_client(1)
        .with_http2_fan_out_threshold_percent(100)
        .with_min_http2_connections_per_endpoint(2)
        .with_max_http2_connections_per_endpoint(2)
        .build()
        .unwrap();
    let factory = Arc::new(ScriptedFactory::new(vec![
        Arc::new(ScriptedTransportClient {
            status: status_a,
            message: message_a,
        }),
        Arc::new(ScriptedTransportClient {
            status: status_b,
            message: message_b,
        }),
    ]));

    AdaptiveTransport::from_config(
        &pool,
        factory,
        HttpClientConfig::dataplane_gateway(&pool, crate::diagnostics::TransportHttpVersion::Http2),
    )
    .unwrap()
}

fn test_endpoint_key() -> EndpointKey {
    EndpointKey::try_from(&url::Url::parse("https://test.documents.azure.com:443/").unwrap())
        .unwrap()
}

pub(super) fn test_request(deadline: Option<Instant>) -> TransportRequest {
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    TransportRequest {
        method: azure_core::http::Method::Get,
        endpoint: endpoint.clone(),
        transport_mode: TransportMode::Gateway,
        operation_type: OperationType::Read,
        effective_partition_key: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        url: endpoint.url().clone(),
        headers: azure_core::http::headers::Headers::new(),
        #[cfg(feature = "unstable_dtx")]
        resource_type: ResourceType::Database,
        body: None,
        auth_context: crate::pipeline::transport::AuthorizationContext::new(
            azure_core::http::Method::Get,
            ResourceType::Database,
            "",
        ),
        execution_context: ExecutionContext::Initial,
        routing_fallback: None,
        deadline: deadline.into(),
    }
}

/// Builds a minimal RNTBD response frame carrying `http_status` and a
/// `SubStatus` token, used to emulate a thin-client error envelope body.
fn gateway_v2_error_frame(http_status: u32, sub_status: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&0_u32.to_le_bytes()); // total_len placeholder
    bytes.extend_from_slice(&http_status.to_le_bytes());
    bytes.extend_from_slice(&[0_u8; 16]); // nil activity id
    bytes.extend_from_slice(&0x001C_u16.to_le_bytes()); // SubStatus token id
    bytes.push(0x02); // ULong token type
    bytes.extend_from_slice(&sub_status.to_le_bytes());
    let total_len = u32::try_from(bytes.len()).unwrap();
    bytes[0..4].copy_from_slice(&total_len.to_le_bytes());
    bytes
}

async fn finalize_gateway_v2_response(
    status: azure_core::http::StatusCode,
    body: Vec<u8>,
) -> TransportResult {
    let client = AdaptiveTransport::Gateway(Arc::new(GatewayV2MockTransportClient::new(vec![
        HttpResponse {
            status: status.into(),
            headers: azure_core::http::headers::Headers::new(),
            body,
        },
    ])));
    let request = gateway_v2_transport_request(TransportMode::GatewayV2);
    let credential = Credential::from(azure_core::credentials::Secret::new("dGVzdA=="));
    let user_agent = azure_core::http::headers::HeaderValue::from_static("test-agent");
    let context = gateway_v2_context(
        &client,
        request.endpoint.endpoint_key(),
        Some("account".to_owned()),
        &credential,
        &user_agent,
    );
    run_pipeline(request, &context, &mut gateway_v2_diagnostics()).await
}

#[derive(Debug)]
struct GatewayV2MockTransportClient {
    responses: Mutex<VecDeque<HttpResponse>>,
    requests: Mutex<Vec<HttpRequest>>,
}

impl GatewayV2MockTransportClient {
    fn new(responses: Vec<HttpResponse>) -> Self {
        Self {
            responses: Mutex::new(responses.into()),
            requests: Mutex::new(Vec::new()),
        }
    }

    fn requests(&self) -> Vec<HttpRequest> {
        self.requests.lock().unwrap().clone()
    }
}

#[async_trait]
impl TransportClient for GatewayV2MockTransportClient {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push(request.clone());
        self.responses.lock().unwrap().pop_front().ok_or_else(|| {
            TransportError::new(
                crate::error::CosmosError::builder()
                    .with_status(crate::error::status_codes::TRANSPORT_IO_FAILED)
                    .with_message("no response queued")
                    .build(),
                RequestSentStatus::Unknown,
            )
        })
    }
}

const GATEWAY_V2_ACTIVITY_ID: &str = "00112233-4455-6677-8899-aabbccddeeff";

fn gateway_v2_transport_request(transport_mode: TransportMode) -> TransportRequest {
    let endpoint = CosmosEndpoint::global(
        url::Url::parse("https://test-thin.documents.azure.com:444/").unwrap(),
    );
    let mut headers = azure_core::http::headers::Headers::new();
    headers.insert("x-ms-activity-id", GATEWAY_V2_ACTIVITY_ID);
    TransportRequest {
        method: azure_core::http::Method::Get,
        endpoint: endpoint.clone(),
        transport_mode,
        operation_type: OperationType::Read,
        effective_partition_key: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        url: endpoint.url().clone(),
        headers,
        #[cfg(feature = "unstable_dtx")]
        resource_type: ResourceType::Document,
        body: None,
        auth_context: crate::pipeline::transport::AuthorizationContext::new(
            azure_core::http::Method::Get,
            ResourceType::Document,
            "dbs/db1/colls/coll1/docs/doc1",
        ),
        execution_context: ExecutionContext::Initial,
        routing_fallback: None,
        deadline: None.into(),
    }
}

fn gateway_v2_context<'a>(
    client: &'a AdaptiveTransport,
    endpoint_key: EndpointKey,
    account_name: Option<String>,
    credential: &'a Credential,
    user_agent: &'a azure_core::http::headers::HeaderValue,
) -> PipelineTestContext<'a> {
    PipelineTestContext {
        transport: client,
        clock: Clock::default(),
        allow_sent_transport_retry: false,
        credential,
        user_agent,
        client_id: &TEST_CLIENT_ID,
        pipeline_type: PipelineKind::DataPlane,
        transport_security: TransportSecurity::Secure,
        endpoint_key,
        account_name,
        collection_rid: None,
        max_throttle_attempts: 9,
        max_throttle_wait_time: Duration::from_secs(30),
        max_throttle_per_retry_delay: METADATA_MAX_PER_RETRY_DELAY,
    }
}

fn gateway_v2_diagnostics() -> DiagnosticsContextBuilder {
    DiagnosticsContextBuilder::new(
        ActivityId::from_string(GATEWAY_V2_ACTIVITY_ID.to_owned()),
        Arc::new(DiagnosticsOptions::default()),
    )
}

fn gateway_v2_response(
    status: u32,
    write_tokens: impl FnOnce(&mut Vec<u8>),
    body: &[u8],
) -> HttpResponse {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&status.to_le_bytes());
    write_gateway_v2_uuid(
        &mut bytes,
        uuid::Uuid::parse_str(GATEWAY_V2_ACTIVITY_ID).unwrap(),
    );
    write_tokens(&mut bytes);
    if !body.is_empty() {
        // PayloadPresent = true (id 0x0000, type Byte).
        bytes.extend_from_slice(&0x0000_u16.to_le_bytes());
        bytes.push(0x00);
        bytes.push(1);
    }
    let total_len = u32::try_from(bytes.len()).unwrap();
    bytes[0..4].copy_from_slice(&total_len.to_le_bytes());
    if !body.is_empty() {
        bytes.extend_from_slice(&(body.len() as u32).to_le_bytes());
        bytes.extend_from_slice(body);
    }
    HttpResponse {
        status: 200,
        headers: azure_core::http::headers::Headers::new(),
        body: bytes,
    }
}

fn write_gateway_v2_u32_token(bytes: &mut Vec<u8>, id: u16, value: u32) {
    bytes.extend_from_slice(&id.to_le_bytes());
    bytes.push(0x02);
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn write_gateway_v2_uuid(bytes: &mut Vec<u8>, value: uuid::Uuid) {
    let value = value.as_u128();
    bytes.extend_from_slice(&((value >> 64) as u64).to_le_bytes());
    bytes.extend_from_slice(&(value as u64).to_le_bytes());
}

mod connectivity;
mod deadlines;
mod diagnostics;
mod gateway_v2;
mod throttling;
