// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Transport-level observation hooks for test harnesses.
//!
//! **Unsupported internal API.** This module is available only under the
//! `__internal_transport_observer` feature. It exists so functional test
//! frameworks can collect wire-level evidence (for example, that hedged
//! requests were actually in flight concurrently) without relying on
//! diagnostics alone. It has no stability guarantees.
//!
//! A [`TransportObserver`] installed on a driver sees every HTTP exchange the
//! driver dispatches after bootstrap, including exchanges whose outcome was
//! produced by fault injection. The observer wraps the outermost HTTP client,
//! below transport sharding and Gateway 2.0 framing, so injected delays count
//! as in-flight time and every wire request is reported separately.
//!
//! The transport pipeline also attaches the logical request it is sending to
//! each [`HttpRequest`], so an [`HttpExchange`] reports the operation, routing,
//! and consistency even when Gateway 2.0 moves those headers into the RNTBD
//! payload. Gateway 2.0 responses are decoded the same way before they are
//! reported.

use std::{
    fmt,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

use async_trait::async_trait;
use azure_core::http::{
    headers::{HeaderName, Headers},
    Method, Url,
};
use bytes::Bytes;

use crate::{
    diagnostics::ExecutionContext,
    fault_injection::FaultOperationType,
    models::{DefaultConsistencyLevel, EffectivePartitionKey, OperationType, ResourceType},
    options::{ConnectionPoolOptions, ReadConsistencyStrategy, Region},
    pipeline::transport::{
        cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError},
        http_client_factory::{HttpClientConfig, HttpClientFactory},
        request::PreparedRequest,
        types::{TransportMode, TransportRequest},
        unwrap_response_for_gateway_v2,
    },
};

const ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
const SUB_STATUS: HeaderName = HeaderName::from_static("x-ms-substatus");

static NEXT_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Receives start and completion events for HTTP exchanges.
///
/// Implementations must be cheap and non-blocking: callbacks run inline on the
/// request path.
pub trait TransportObserver: Send + Sync + fmt::Debug {
    /// Called immediately before a request is handed to the transport.
    fn on_start(&self, exchange: &HttpExchange);

    /// Called when the exchange finishes, fails, or is cancelled.
    ///
    /// Every [`on_start`](Self::on_start) call is followed by exactly one
    /// `on_complete` call for the same [`HttpExchange::sequence`].
    fn on_complete(&self, exchange: &HttpExchange, outcome: &HttpOutcome);
}

/// The logical request behind one transport attempt.
///
/// The transport pipeline stores this on [`HttpRequest`] so the observer can
/// report it alongside the wire request.
#[derive(Clone, Debug)]
pub(crate) struct RequestContext {
    operation_type: OperationType,
    resource_type: ResourceType,
    transport_mode: TransportMode,
    execution_context: ExecutionContext,
    region: Option<Region>,
    effective_partition_key: Option<EffectivePartitionKey>,
    consistency_level: DefaultConsistencyLevel,
    read_consistency_strategy: ReadConsistencyStrategy,
    headers: Headers,
    body: Option<Bytes>,
}

/// Attaches the logical `request` to the HTTP request prepared for one attempt.
pub(super) fn attach_context(
    mut prepared: PreparedRequest,
    request: &TransportRequest,
    execution_context: ExecutionContext,
) -> PreparedRequest {
    prepared.http_request.observer_context = Some(Arc::new(RequestContext {
        operation_type: request.operation_type,
        resource_type: request.auth_context.resource_type,
        transport_mode: request.transport_mode,
        execution_context,
        region: request.endpoint.region().cloned(),
        effective_partition_key: request.effective_partition_key.clone(),
        consistency_level: request.effective_consistency,
        read_consistency_strategy: request.read_consistency_strategy,
        headers: request.headers.clone(),
        body: request.body.clone(),
    }));
    prepared
}

/// Describes one HTTP request dispatched by the driver.
///
/// Accessors for logical request data return `None` for requests sent outside
/// the transport pipeline, such as account-metadata refreshes and connectivity
/// probes.
#[derive(Clone, Debug)]
pub struct HttpExchange {
    sequence: u64,
    started_at: Instant,
    method: Method,
    url: Url,
    activity_id: Option<String>,
    body: Option<Bytes>,
    context: Option<Arc<RequestContext>>,
}

impl HttpExchange {
    fn new(request: &HttpRequest) -> Self {
        let context = request.observer_context.clone();
        let headers = context.as_ref().map_or(&request.headers, |c| &c.headers);
        Self {
            sequence: NEXT_SEQUENCE.fetch_add(1, Ordering::Relaxed),
            started_at: Instant::now(),
            method: request.method,
            url: request.url.clone(),
            activity_id: headers.get_optional_str(&ACTIVITY_ID).map(str::to_owned),
            body: match &context {
                Some(c) => c.body.clone(),
                None => request.body.clone(),
            },
            context,
        }
    }

    /// Process-wide monotonically increasing sequence number.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Monotonic time at which the request was handed to the transport.
    pub fn started_at(&self) -> Instant {
        self.started_at
    }

    /// The HTTP method on the wire.
    pub fn method(&self) -> Method {
        self.method
    }

    /// The request URL on the wire.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// The logical operation, using the fault-injection operation names
    /// (for example, `ReadItem` or `QueryItem`), when known.
    pub fn operation(&self) -> Option<&'static str> {
        let c = self.context.as_deref()?;
        FaultOperationType::from_operation_and_resource(&c.operation_type, &c.resource_type)
            .map(|op| op.as_str())
    }

    /// The operation type.
    pub fn operation_type(&self) -> Option<OperationType> {
        self.context.as_deref().map(|c| c.operation_type)
    }

    /// The type of resource the operation targets.
    pub fn resource_type(&self) -> Option<ResourceType> {
        self.context.as_deref().map(|c| c.resource_type)
    }

    /// The transport mode the request was routed to: `Gateway` or `GatewayV2`.
    pub fn transport_mode(&self) -> Option<&'static str> {
        self.context.as_deref().map(|c| match c.transport_mode {
            TransportMode::Gateway => "Gateway",
            TransportMode::GatewayV2 => "GatewayV2",
        })
    }

    /// Why the request was sent, for example an initial attempt or a retry.
    pub fn execution_context(&self) -> Option<ExecutionContext> {
        self.context.as_deref().map(|c| c.execution_context)
    }

    /// The region the request was routed to, when known.
    pub fn region(&self) -> Option<&str> {
        self.context.as_deref()?.region.as_ref().map(Region::as_str)
    }

    /// The effective partition key the request targets, when it targets one.
    pub fn effective_partition_key(&self) -> Option<&EffectivePartitionKey> {
        self.context.as_deref()?.effective_partition_key.as_ref()
    }

    /// The effective consistency level, for example `Session`.
    pub fn consistency_level(&self) -> Option<&'static str> {
        self.context.as_deref().map(|c| match c.consistency_level {
            DefaultConsistencyLevel::Strong => "Strong",
            DefaultConsistencyLevel::BoundedStaleness => "BoundedStaleness",
            DefaultConsistencyLevel::Session => "Session",
            DefaultConsistencyLevel::ConsistentPrefix => "ConsistentPrefix",
            DefaultConsistencyLevel::Eventual => "Eventual",
        })
    }

    /// The read consistency strategy the caller requested.
    pub fn read_consistency_strategy(&self) -> Option<ReadConsistencyStrategy> {
        self.context.as_deref().map(|c| c.read_consistency_strategy)
    }

    /// The logical request headers, before signing and before Gateway 2.0
    /// moves them into the RNTBD payload.
    pub fn headers(&self) -> Option<&Headers> {
        self.context.as_deref().map(|c| &c.headers)
    }

    /// The request activity ID, when present.
    pub fn activity_id(&self) -> Option<&str> {
        self.activity_id.as_deref()
    }

    /// The logical request body, when the request has one.
    ///
    /// For Gateway 2.0 this is the body before RNTBD framing.
    pub fn body(&self) -> Option<&Bytes> {
        self.body.as_ref()
    }

    fn is_gateway_v2(&self) -> bool {
        self.context
            .as_deref()
            .is_some_and(|c| c.transport_mode == TransportMode::GatewayV2)
    }
}

/// How an HTTP exchange finished.
#[derive(Clone, Debug)]
pub struct HttpOutcome {
    completed_at: Instant,
    kind: HttpOutcomeKind,
    body: Option<Bytes>,
}

/// The terminal state of an HTTP exchange.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum HttpOutcomeKind {
    /// A response was received (possibly synthesized by fault injection).
    ///
    /// For Gateway 2.0, the status and substatus are decoded from the RNTBD
    /// response.
    Response {
        /// HTTP status code.
        status: u16,
        /// Cosmos DB substatus code, when present.
        sub_status: Option<u32>,
    },
    /// The transport failed without a response.
    TransportError,
    /// The request future was dropped before completing, for example because
    /// a hedge partner won.
    Cancelled,
}

impl HttpOutcome {
    /// Monotonic time at which the exchange finished.
    pub fn completed_at(&self) -> Instant {
        self.completed_at
    }

    /// The terminal state.
    pub fn kind(&self) -> &HttpOutcomeKind {
        &self.kind
    }

    /// The response body, when a response arrived.
    ///
    /// For Gateway 2.0 this is the body decoded from the RNTBD response.
    pub fn body(&self) -> Option<&Bytes> {
        self.body.as_ref()
    }
}

/// Factory decorator that wraps every created client with an observer.
#[derive(Debug)]
pub(crate) struct ObservingHttpClientFactory {
    inner: Arc<dyn HttpClientFactory>,
    observer: Arc<dyn TransportObserver>,
}

impl ObservingHttpClientFactory {
    /// Wraps `inner`.
    pub(crate) fn new(
        inner: Arc<dyn HttpClientFactory>,
        observer: Arc<dyn TransportObserver>,
    ) -> Self {
        Self { inner, observer }
    }
}

impl HttpClientFactory for ObservingHttpClientFactory {
    fn build(
        &self,
        connection_pool: &ConnectionPoolOptions,
        config: HttpClientConfig,
    ) -> crate::error::Result<Arc<dyn TransportClient>> {
        Ok(Arc::new(ObservingClient {
            inner: self.inner.build(connection_pool, config)?,
            observer: Arc::clone(&self.observer),
        }))
    }
}

#[derive(Debug)]
struct ObservingClient {
    inner: Arc<dyn TransportClient>,
    observer: Arc<dyn TransportObserver>,
}

struct CompletionGuard<'a> {
    observer: &'a dyn TransportObserver,
    exchange: HttpExchange,
    done: bool,
}

impl CompletionGuard<'_> {
    fn complete(mut self, kind: HttpOutcomeKind, body: Option<Bytes>) {
        self.done = true;
        self.observer.on_complete(
            &self.exchange,
            &HttpOutcome {
                completed_at: Instant::now(),
                kind,
                body,
            },
        );
    }
}

impl Drop for CompletionGuard<'_> {
    fn drop(&mut self) {
        if !self.done {
            self.observer.on_complete(
                &self.exchange,
                &HttpOutcome {
                    completed_at: Instant::now(),
                    kind: HttpOutcomeKind::Cancelled,
                    body: None,
                },
            );
        }
    }
}

fn response_outcome(response: &HttpResponse) -> (HttpOutcomeKind, Option<Bytes>) {
    (
        HttpOutcomeKind::Response {
            status: response.status,
            sub_status: response
                .headers
                .get_optional_str(&SUB_STATUS)
                .and_then(|v| v.parse().ok()),
        },
        Some(Bytes::copy_from_slice(&response.body)),
    )
}

#[async_trait]
impl TransportClient for ObservingClient {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let exchange = HttpExchange::new(request);
        self.observer.on_start(&exchange);
        let gateway_v2 = exchange.is_gateway_v2();
        let guard = CompletionGuard {
            observer: self.observer.as_ref(),
            exchange,
            done: false,
        };

        let result = self.inner.send(request).await;

        let (kind, body) = match &result {
            Ok(response) if gateway_v2 => match unwrap_response_for_gateway_v2(response.clone()) {
                Ok(decoded) => response_outcome(&decoded),
                Err(_) => response_outcome(response),
            },
            Ok(response) => response_outcome(response),
            Err(_) => (HttpOutcomeKind::TransportError, None),
        };
        guard.complete(kind, body);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Mutex, time::Duration};

    #[derive(Debug, Default)]
    struct Recorder(Mutex<Vec<(HttpExchange, Option<HttpOutcome>)>>);

    impl TransportObserver for Recorder {
        fn on_start(&self, exchange: &HttpExchange) {
            self.0.lock().unwrap().push((exchange.clone(), None));
        }
        fn on_complete(&self, exchange: &HttpExchange, outcome: &HttpOutcome) {
            let mut events = self.0.lock().unwrap();
            let entry = events
                .iter_mut()
                .find(|(e, _)| e.sequence() == exchange.sequence())
                .unwrap();
            entry.1 = Some(outcome.clone());
        }
    }

    #[derive(Debug)]
    struct SlowClient(HttpResponse);

    #[async_trait]
    impl TransportClient for SlowClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            tokio::time::sleep(Duration::from_millis(20)).await;
            Ok(self.0.clone())
        }
    }

    fn throttled() -> HttpResponse {
        let mut headers = Headers::new();
        headers.insert(SUB_STATUS, "3200");
        HttpResponse {
            status: 429,
            headers,
            body: b"{\"id\":\"1\"}".to_vec(),
        }
    }

    fn context(transport_mode: TransportMode) -> RequestContext {
        let mut headers = Headers::new();
        headers.insert(ACTIVITY_ID, "logical-activity");
        RequestContext {
            operation_type: OperationType::Read,
            resource_type: ResourceType::Document,
            transport_mode,
            execution_context: ExecutionContext::Hedging,
            region: Some(Region::new("eastus")),
            effective_partition_key: None,
            consistency_level: DefaultConsistencyLevel::Session,
            read_consistency_strategy: ReadConsistencyStrategy::Default,
            headers,
            body: Some(Bytes::from_static(b"{}")),
        }
    }

    fn request(context: Option<RequestContext>) -> HttpRequest {
        HttpRequest {
            url: Url::parse("https://example.documents.azure.com/dbs/db").unwrap(),
            method: Method::Get,
            headers: Headers::new(),
            body: Some(Bytes::from_static(b"rntbd-frame")),
            timeout: None,
            evaluation_collector: None,
            observer_context: context.map(Arc::new),
        }
    }

    fn observe(response: HttpResponse) -> (Arc<Recorder>, ObservingClient) {
        let recorder = Arc::new(Recorder::default());
        let client = ObservingClient {
            inner: Arc::new(SlowClient(response)),
            observer: recorder.clone(),
        };
        (recorder, client)
    }

    #[tokio::test]
    async fn reports_logical_request_and_response() {
        let (recorder, client) = observe(throttled());
        client
            .send(&request(Some(context(TransportMode::Gateway))))
            .await
            .unwrap();

        let events = recorder.0.lock().unwrap();
        let (exchange, outcome) = &events[0];
        assert_eq!(exchange.operation(), Some("ReadItem"));
        assert_eq!(exchange.resource_type(), Some(ResourceType::Document));
        assert_eq!(exchange.transport_mode(), Some("Gateway"));
        assert_eq!(
            exchange.execution_context(),
            Some(ExecutionContext::Hedging)
        );
        assert_eq!(exchange.region(), Some("eastus"));
        assert_eq!(exchange.consistency_level(), Some("Session"));
        assert_eq!(exchange.activity_id(), Some("logical-activity"));
        assert_eq!(exchange.body(), Some(&Bytes::from_static(b"{}")));
        let outcome = outcome.as_ref().unwrap();
        assert_eq!(
            outcome.kind(),
            &HttpOutcomeKind::Response {
                status: 429,
                sub_status: Some(3200)
            }
        );
        assert_eq!(outcome.body(), Some(&Bytes::from_static(b"{\"id\":\"1\"}")));
    }

    #[tokio::test]
    async fn decodes_gateway_v2_response() {
        let mut frame = Vec::with_capacity(24);
        frame.extend_from_slice(&24_u32.to_le_bytes());
        frame.extend_from_slice(&404_u32.to_le_bytes());
        frame.extend_from_slice(&[0; 16]);
        let (recorder, client) = observe(HttpResponse {
            status: 200,
            headers: Headers::new(),
            body: frame,
        });
        client
            .send(&request(Some(context(TransportMode::GatewayV2))))
            .await
            .unwrap();

        let events = recorder.0.lock().unwrap();
        assert_eq!(events[0].0.transport_mode(), Some("GatewayV2"));
        assert_eq!(
            events[0].1.as_ref().unwrap().kind(),
            &HttpOutcomeKind::Response {
                status: 404,
                sub_status: None
            }
        );
    }

    #[tokio::test]
    async fn reports_wire_request_without_context() {
        let (recorder, client) = observe(throttled());
        client.send(&request(None)).await.unwrap();

        let events = recorder.0.lock().unwrap();
        let exchange = &events[0].0;
        assert_eq!(exchange.operation(), None);
        assert_eq!(exchange.region(), None);
        assert_eq!(exchange.body(), Some(&Bytes::from_static(b"rntbd-frame")));
    }

    #[tokio::test]
    async fn reports_cancellation_when_dropped() {
        let (recorder, client) = observe(throttled());
        let request = request(None);
        let result = tokio::time::timeout(Duration::from_millis(1), client.send(&request)).await;
        assert!(result.is_err());
        assert_eq!(
            recorder.0.lock().unwrap()[0].1.as_ref().unwrap().kind(),
            &HttpOutcomeKind::Cancelled
        );
    }
}
