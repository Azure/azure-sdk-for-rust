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
//! produced by fault injection. The observer wraps the outermost transport
//! client, so injected delays count as in-flight time.

use std::{
    fmt,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

use async_trait::async_trait;
use azure_core::http::{headers::HeaderName, Method, Url};
use bytes::Bytes;

use crate::{
    pipeline::transport::{
        cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError},
        http_client_factory::{HttpClientConfig, HttpClientFactory},
    },
    models::cosmos_headers::fault_injection_header_names::{
        FAULT_INJECTION_OPERATION, FAULT_INJECTION_REGION,
    },
    options::ConnectionPoolOptions,
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

/// Describes one HTTP request dispatched by the driver.
#[derive(Clone, Debug)]
pub struct HttpExchange {
    sequence: u64,
    started_at: Instant,
    method: Method,
    url: Url,
    operation: Option<String>,
    region: Option<String>,
    activity_id: Option<String>,
    body: Option<Bytes>,
}

impl HttpExchange {
    /// Process-wide monotonically increasing sequence number.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Monotonic time at which the request was handed to the transport.
    pub fn started_at(&self) -> Instant {
        self.started_at
    }

    /// The HTTP method.
    pub fn method(&self) -> Method {
        self.method
    }

    /// The request URL.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// The logical operation, using the fault-injection operation names
    /// (for example, `ReadItem` or `QueryItem`), when known.
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// The region the request was routed to, when known.
    pub fn region(&self) -> Option<&str> {
        self.region.as_deref()
    }

    /// The request activity ID, when present.
    pub fn activity_id(&self) -> Option<&str> {
        self.activity_id.as_deref()
    }

    /// The request body, when the request has one.
    pub fn body(&self) -> Option<&Bytes> {
        self.body.as_ref()
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
    pub fn body(&self) -> Option<&Bytes> {
        self.body.as_ref()
    }
}

/// Factory decorator that wraps every created client with an observer.
#[derive(Debug)]
pub(crate) struct ObservingHttpClientFactory {
    inner: Arc<dyn HttpClientFactory>,
    observer: Arc<dyn TransportObserver>,
    strip_tags: bool,
}

impl ObservingHttpClientFactory {
    /// Wraps `inner`.
    ///
    /// Set `strip_tags` when no fault-injecting layer sits below this one, so
    /// internal routing tags never reach the network.
    pub(crate) fn new(
        inner: Arc<dyn HttpClientFactory>,
        observer: Arc<dyn TransportObserver>,
        strip_tags: bool,
    ) -> Self {
        Self {
            inner,
            observer,
            strip_tags,
        }
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
            strip_tags: self.strip_tags,
        }))
    }
}

#[derive(Debug)]
struct ObservingClient {
    inner: Arc<dyn TransportClient>,
    observer: Arc<dyn TransportObserver>,
    strip_tags: bool,
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

#[async_trait]
impl TransportClient for ObservingClient {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let header = |name: &HeaderName| request.headers.get_optional_str(name).map(str::to_owned);
        let exchange = HttpExchange {
            sequence: NEXT_SEQUENCE.fetch_add(1, Ordering::Relaxed),
            started_at: Instant::now(),
            method: request.method,
            url: request.url.clone(),
            operation: header(&HeaderName::from_static(FAULT_INJECTION_OPERATION)),
            region: header(&HeaderName::from_static(FAULT_INJECTION_REGION)),
            activity_id: header(&ACTIVITY_ID),
            body: request.body.clone(),
        };
        self.observer.on_start(&exchange);
        let guard = CompletionGuard {
            observer: self.observer.as_ref(),
            exchange,
            done: false,
        };

        let result = if self.strip_tags {
            let mut headers = request.headers.clone();
            headers.remove(FAULT_INJECTION_OPERATION);
            headers.remove(FAULT_INJECTION_REGION);
            let stripped = HttpRequest {
                url: request.url.clone(),
                method: request.method,
                headers,
                body: request.body.clone(),
                timeout: request.timeout,
                evaluation_collector: request.evaluation_collector.clone(),
            };
            self.inner.send(&stripped).await
        } else {
            self.inner.send(request).await
        };

        let (kind, body) = match &result {
            Ok(response) => (
                HttpOutcomeKind::Response {
                    status: response.status,
                    sub_status: response
                        .headers
                        .get_optional_str(&SUB_STATUS)
                        .and_then(|v| v.parse().ok()),
                },
                Some(Bytes::copy_from_slice(&response.body)),
            ),
            Err(_) => (HttpOutcomeKind::TransportError, None),
        };
        guard.complete(kind, body);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::headers::Headers;
    use std::{sync::Mutex, time::Duration};

    #[derive(Debug, Default)]
    struct Recorder(
        Mutex<Vec<(u64, Option<HttpOutcomeKind>)>>,
        Mutex<Vec<Option<Bytes>>>,
    );

    impl TransportObserver for Recorder {
        fn on_start(&self, exchange: &HttpExchange) {
            self.0.lock().unwrap().push((exchange.sequence(), None));
        }
        fn on_complete(&self, exchange: &HttpExchange, outcome: &HttpOutcome) {
            let mut events = self.0.lock().unwrap();
            let entry = events
                .iter_mut()
                .find(|(seq, _)| *seq == exchange.sequence())
                .unwrap();
            entry.1 = Some(outcome.kind().clone());
            let mut bodies = self.1.lock().unwrap();
            bodies.push(exchange.body().cloned());
            bodies.push(outcome.body().cloned());
        }
    }

    #[derive(Debug)]
    struct SlowClient(Mutex<Option<Headers>>);

    #[async_trait]
    impl TransportClient for SlowClient {
        async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            *self.0.lock().unwrap() = Some(request.headers.clone());
            tokio::time::sleep(Duration::from_millis(20)).await;
            Ok(HttpResponse {
                status: 429,
                headers: {
                    let mut h = Headers::new();
                    h.insert(SUB_STATUS, "3200");
                    h
                },
                body: b"{\"id\":\"1\"}".to_vec(),
            })
        }
    }

    fn request() -> HttpRequest {
        let mut headers = Headers::new();
        headers.insert(FAULT_INJECTION_OPERATION, "ReadItem");
        headers.insert(FAULT_INJECTION_REGION, "East US");
        HttpRequest {
            url: Url::parse("https://example.documents.azure.com/dbs/db").unwrap(),
            method: Method::Get,
            headers,
            body: Some(Bytes::from_static(b"{}")),
            timeout: None,
            evaluation_collector: None,
        }
    }

    #[tokio::test]
    async fn reports_response_and_strips_tags() {
        let recorder = Arc::new(Recorder::default());
        let inner = Arc::new(SlowClient(Mutex::new(None)));
        let client = ObservingClient {
            inner: inner.clone(),
            observer: recorder.clone(),
            strip_tags: true,
        };
        client.send(&request()).await.unwrap();

        let events = recorder.0.lock().unwrap();
        assert_eq!(
            events[0].1,
            Some(HttpOutcomeKind::Response {
                status: 429,
                sub_status: Some(3200)
            })
        );
        assert_eq!(
            *recorder.1.lock().unwrap(),
            [
                Some(Bytes::from_static(b"{}")),
                Some(Bytes::from_static(b"{\"id\":\"1\"}"))
            ]
        );
        let forwarded = inner.0.lock().unwrap().clone().unwrap();
        assert!(forwarded
            .get_optional_str(&HeaderName::from_static(FAULT_INJECTION_OPERATION))
            .is_none());
    }

    #[tokio::test]
    async fn reports_cancellation_when_dropped() {
        let recorder = Arc::new(Recorder::default());
        let client = ObservingClient {
            inner: Arc::new(SlowClient(Mutex::new(None))),
            observer: recorder.clone(),
            strip_tags: false,
        };
        let request = request();
        let result = tokio::time::timeout(Duration::from_millis(1), client.send(&request)).await;
        assert!(result.is_err());
        assert_eq!(
            recorder.0.lock().unwrap()[0].1,
            Some(HttpOutcomeKind::Cancelled)
        );
    }
}
