// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! `InMemoryEmulatorHttpClient` — dispatches requests against an in-memory
//! Cosmos DB store. Used as a [`TransportClient`] implementation by the
//! driver and called directly by integration tests.

use std::sync::Arc;

use async_trait::async_trait;
use azure_core::http::{AsyncRawResponse, Request};
use azure_core::Bytes;

use super::config::VirtualAccountConfig;
use super::dispatch::{parse_request, resolve_region};
use super::observer::RequestObserver;
use super::operations::handle_operation;
use super::store::EmulatorStore;
use crate::driver::transport::cosmos_transport_client::{
    HttpRequest as DriverHttpRequest, HttpResponse as DriverHttpResponse, TransportClient,
    TransportError,
};
use crate::driver::transport::http_client_factory::{HttpClientConfig, HttpClientFactory};
use crate::models::CosmosStatus;
use crate::options::ConnectionPoolOptions;

/// An HTTP client that intercepts all requests and serves them from an in-memory store.
///
/// Implements [`azure_core::http::HttpClient`], replacing the real HTTP transport
/// at the bottom of the pipeline stack. The full operation pipeline (endpoint resolution,
/// session routing, retry, failover, diagnostics) executes normally above this layer.
///
/// # Tokio runtime requirement
///
/// All emulator entry points that schedule background work — point writes that
/// trigger non-immediate replication, [`EmulatorStore::split_partition`],
/// [`EmulatorStore::merge_partitions`], the deferred-replication retry path —
/// call `tokio::spawn` and therefore **must run inside a Tokio runtime**.
/// Calling them from a non-Tokio thread will panic. Use `#[tokio::test]` /
/// `tokio::runtime::Runtime::block_on` or only call them from code already
/// running inside a Tokio reactor.
pub struct InMemoryEmulatorHttpClient {
    store: Arc<EmulatorStore>,
    request_observer: Option<Arc<dyn RequestObserver>>,
}

impl InMemoryEmulatorHttpClient {
    /// Creates a new emulator HTTP client with the given virtual account configuration.
    pub fn new(config: VirtualAccountConfig) -> Self {
        Self {
            store: EmulatorStore::new(config),
            request_observer: None,
        }
    }

    /// Returns a handle to the underlying emulator store for test hooks and provisioning.
    pub fn store(&self) -> Arc<EmulatorStore> {
        Arc::clone(&self.store)
    }

    /// Attaches a [`RequestObserver`] that is invoked for every request the
    /// emulator handles, before the request is routed.
    ///
    /// Intended for tests that need to assert on outgoing request shape
    /// (e.g. that the configured `User-Agent` suffix actually reaches the
    /// wire). Without an observer the dispatch path pays no overhead.
    ///
    /// Replaces any previously-attached observer.
    pub fn with_request_observer(mut self, observer: Arc<dyn RequestObserver>) -> Self {
        self.request_observer = Some(observer);
        self
    }

    /// Creates a `CosmosDriverRuntimeBuilder` pre-configured to use this emulator
    /// as the HTTP transport for all requests.
    ///
    /// This enables end-to-end testing through the full driver pipeline
    /// (endpoint resolution, session routing, retry, failover, diagnostics)
    /// with all HTTP I/O replaced by the in-memory store.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> azure_data_cosmos_driver::error::Result<()> {
    /// use azure_data_cosmos_driver::in_memory_emulator::*;
    /// use azure_data_cosmos_driver::models::AccountReference;
    /// use azure_data_cosmos_driver::options::DriverOptions;
    /// use url::Url;
    ///
    /// let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(
    ///     VirtualAccountConfig::new(vec![
    ///         VirtualRegion::new("East US", Url::parse("https://eastus.emulator.local").unwrap()),
    ///     ])?,
    /// ));
    ///
    /// let runtime = emulator.runtime_builder().build().await?;
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://eastus.emulator.local").unwrap(),
    ///     "emulator-key",
    /// );
    /// let driver = runtime.create_driver(DriverOptions::builder(account).build()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn runtime_builder(self: &Arc<Self>) -> crate::driver::CosmosDriverRuntimeBuilder {
        let factory = Arc::new(EmulatorHttpClientFactory {
            client: Arc::clone(self),
        });
        crate::driver::CosmosDriverRuntimeBuilder::new().with_http_client_factory(factory)
    }

    /// Like [`Self::runtime_builder`] but composes the emulator factory with
    /// a `FaultInjectingHttpClientFactory` so the supplied
    /// [`FaultInjectionRule`](crate::fault_injection::FaultInjectionRule)s
    /// evaluate on every outbound request before reaching the emulator.
    ///
    /// Used by hedging integration tests to inject region-targeted delays
    /// and error statuses without standing up a real network harness.
    /// Rules are evaluated lowest-index first; see
    /// [`crate::fault_injection`] for the rule-construction surface.
    #[cfg(feature = "fault_injection")]
    pub fn runtime_builder_with_fault_rules(
        self: &Arc<Self>,
        rules: Vec<Arc<crate::fault_injection::FaultInjectionRule>>,
    ) -> crate::driver::CosmosDriverRuntimeBuilder {
        let emulator_factory = Arc::new(EmulatorHttpClientFactory {
            client: Arc::clone(self),
        });
        let fault_factory = Arc::new(
            crate::fault_injection::FaultInjectingHttpClientFactory::new(emulator_factory, rules),
        );
        crate::driver::CosmosDriverRuntimeBuilder::new().with_http_client_factory(fault_factory)
    }
}

impl std::fmt::Debug for InMemoryEmulatorHttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InMemoryEmulatorHttpClient")
            .field("store", &self.store)
            .finish()
    }
}

impl InMemoryEmulatorHttpClient {
    /// Dispatches a request against the in-memory store and returns the
    /// emulated response. Inherent method (no longer implements
    /// `azure_core::HttpClient`) so the entire emulator pipeline can
    /// surface typed [`crate::error::CosmosError`] values directly.
    pub async fn execute_request(
        &self,
        request: &Request,
    ) -> crate::error::Result<AsyncRawResponse> {
        // Notify any attached observer first so tests can assert on the
        // outgoing request shape (headers, URL, method) before the emulator
        // mutates state. The fast path when no observer is attached is a
        // single Option check.
        if let Some(observer) = &self.request_observer {
            observer.on_request(request);
        }

        let parsed = parse_request(request);

        // Resolve region from URL
        let region_name = match resolve_region(request.url(), self.store.config()) {
            Some(r) => r,
            None => {
                return Err(crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::new(azure_core::http::StatusCode::BadRequest))
                    .with_message(format!(
                        "in-memory emulator: request URL host '{}' does not match any configured region",
                        request.url().host_str().unwrap_or("<none>"),
                    ))
                    .build());
            }
        };

        // Extract request body
        let body_bytes: Vec<u8> = Bytes::from(request.body()).to_vec();

        let response = handle_operation(
            &self.store,
            region_name,
            &parsed,
            request.headers(),
            &body_bytes,
        )
        .await;

        Ok(response)
    }
}

/// An `HttpClientFactory` that always returns the same emulator `HttpClient`.
///
/// This factory ignores connection pool and config settings since the emulator
/// doesn't use real network connections.
#[derive(Debug)]
struct EmulatorHttpClientFactory {
    client: Arc<InMemoryEmulatorHttpClient>,
}

impl HttpClientFactory for EmulatorHttpClientFactory {
    fn build(
        &self,
        _connection_pool: &ConnectionPoolOptions,
        _config: HttpClientConfig,
    ) -> crate::error::Result<Arc<dyn TransportClient>> {
        Ok(Arc::new(EmulatorTransportClient {
            emulator: Arc::clone(&self.client),
        }))
    }
}

/// Adapter that implements the driver's [`TransportClient`] trait by
/// delegating to the in-memory emulator's request handling.
#[derive(Debug)]
struct EmulatorTransportClient {
    emulator: Arc<InMemoryEmulatorHttpClient>,
}

#[async_trait]
impl TransportClient for EmulatorTransportClient {
    async fn send(
        &self,
        request: &DriverHttpRequest,
    ) -> Result<DriverHttpResponse, TransportError> {
        use azure_core::http::Request;

        // Convert the driver's HttpRequest to an azure_core Request
        let method = request.method;
        let mut core_request = Request::new(request.url.clone(), method);
        for (name, value) in request.headers.iter() {
            core_request
                .headers_mut()
                .insert(name.clone(), value.clone());
        }
        if let Some(body) = &request.body {
            core_request.set_body(body.to_vec());
        }

        // Execute through the emulator
        let async_response = self
            .emulator
            .execute_request(&core_request)
            .await
            .map_err(|e| TransportError::new(e, crate::diagnostics::RequestSentStatus::Unknown))?;

        // Collect the buffered response
        let raw = async_response.try_into_raw_response().await.map_err(|e| {
            let cosmos_err = crate::error::CosmosError::builder()
                .with_status(CosmosStatus::TRANSPORT_BODY_READ_FAILED)
                .with_message(e.to_string())
                .with_source(e)
                .build();
            TransportError::new(cosmos_err, crate::diagnostics::RequestSentStatus::Sent)
        })?;

        let status = u16::from(raw.status());
        let headers = raw.headers().clone();
        let body: &[u8] = raw.body().as_ref();

        Ok(DriverHttpResponse {
            status,
            headers,
            body: body.to_vec(),
        })
    }
}
