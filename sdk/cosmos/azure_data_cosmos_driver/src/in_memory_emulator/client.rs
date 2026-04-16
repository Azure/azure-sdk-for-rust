// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! `InMemoryEmulatorHttpClient` — implements `azure_core::http::HttpClient`.

use std::sync::Arc;

use async_trait::async_trait;
use azure_core::http::{AsyncRawResponse, HttpClient, Request};
use azure_core::Bytes;

use super::config::VirtualAccountConfig;
use super::dispatch::{parse_request, resolve_region};
use super::operations::handle_operation;
use super::store::EmulatorStore;
use crate::driver::transport::cosmos_transport_client::{
    HttpRequest as DriverHttpRequest, HttpResponse as DriverHttpResponse, TransportClient,
    TransportError,
};
use crate::driver::transport::http_client_factory::{HttpClientConfig, HttpClientFactory};
use crate::options::ConnectionPoolOptions;

/// An HTTP client that intercepts all requests and serves them from an in-memory store.
///
/// Implements [`azure_core::http::HttpClient`], replacing the real HTTP transport
/// at the bottom of the pipeline stack. The full operation pipeline (endpoint resolution,
/// session routing, retry, failover, diagnostics) executes normally above this layer.
pub struct InMemoryEmulatorHttpClient {
    store: Arc<EmulatorStore>,
}

impl InMemoryEmulatorHttpClient {
    /// Creates a new emulator HTTP client with the given virtual account configuration.
    pub fn new(config: VirtualAccountConfig) -> Self {
        Self {
            store: EmulatorStore::new(config),
        }
    }

    /// Returns a handle to the underlying emulator store for test hooks and provisioning.
    pub fn store(&self) -> Arc<EmulatorStore> {
        Arc::clone(&self.store)
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
    /// # async fn example() -> azure_core::Result<()> {
    /// use azure_data_cosmos_driver::in_memory_emulator::*;
    /// use azure_data_cosmos_driver::models::AccountReference;
    /// use url::Url;
    ///
    /// let emulator = InMemoryEmulatorHttpClient::new(
    ///     VirtualAccountConfig::new(vec![
    ///         VirtualRegion::new("East US", Url::parse("https://eastus.emulator.local").unwrap()),
    ///     ])
    /// );
    ///
    /// let runtime = emulator.runtime_builder().build().await?;
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://eastus.emulator.local").unwrap(),
    ///     "emulator-key",
    /// );
    /// let driver = runtime.get_or_create_driver(account, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn runtime_builder(&self) -> crate::driver::CosmosDriverRuntimeBuilder {
        let factory = Arc::new(EmulatorHttpClientFactory {
            client: Arc::new(Self {
                store: Arc::clone(&self.store),
            }),
        });
        crate::driver::CosmosDriverRuntimeBuilder::new().with_http_client_factory(factory)
    }
}

impl std::fmt::Debug for InMemoryEmulatorHttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InMemoryEmulatorHttpClient")
            .field("store", &self.store)
            .finish()
    }
}

#[async_trait]
impl HttpClient for InMemoryEmulatorHttpClient {
    async fn execute_request(&self, request: &Request) -> azure_core::Result<AsyncRawResponse> {
        let parsed = parse_request(request);

        // Resolve region from URL
        let region_name = resolve_region(request.url(), self.store.config()).unwrap_or_else(|| {
            // If no region matches, use the first configured region as default
            self.store.config().regions()[0].name()
        });

        // Extract request body
        let body_bytes: Vec<u8> = Bytes::from(request.body()).to_vec();

        let response = handle_operation(&self.store, region_name, &parsed, &body_bytes).await;

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
    ) -> azure_core::Result<Arc<dyn TransportClient>> {
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
            TransportError::new(
                azure_core::Error::new(azure_core::error::ErrorKind::Io, e),
                crate::diagnostics::RequestSentStatus::Sent,
            )
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
