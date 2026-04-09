// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Factory decorator that wraps created HTTP clients with fault injection.

use std::sync::Arc;

use super::http_client::FaultClient;
use super::rule::FaultInjectionRule;
use crate::driver::transport::cosmos_transport_client::TransportClient;
use crate::driver::transport::http_client_factory::{HttpClientConfig, HttpClientFactory};
use crate::options::ConnectionPoolOptions;

/// An [`HttpClientFactory`] decorator that wraps clients with fault injection.
///
/// When `create` is called, this factory delegates to the inner factory to build
/// a real HTTP client, then wraps it in a [`FaultClient`] that
/// evaluates the configured rules on every request.
#[derive(Debug)]
pub(crate) struct FaultInjectingHttpClientFactory {
    inner: Arc<dyn HttpClientFactory>,
    rules: Arc<Vec<Arc<FaultInjectionRule>>>,
}

impl FaultInjectingHttpClientFactory {
    /// Creates a new factory that wraps clients from `inner` with fault injection rules.
    pub(crate) fn new(
        inner: Arc<dyn HttpClientFactory>,
        rules: Vec<Arc<FaultInjectionRule>>,
    ) -> Self {
        Self {
            inner,
            rules: Arc::new(rules),
        }
    }
}

impl HttpClientFactory for FaultInjectingHttpClientFactory {
    fn build(
        &self,
        connection_pool: &ConnectionPoolOptions,
        config: HttpClientConfig,
    ) -> azure_core::Result<Arc<dyn TransportClient>> {
        let real_client = self.inner.build(connection_pool, config)?;
        let rules = (*self.rules).clone();
        Ok(Arc::new(FaultClient::new(real_client, rules)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::transport::cosmos_transport_client::{
        HttpRequest, HttpResponse, TransportClient, TransportError,
    };
    use crate::fault_injection::{
        FaultInjectionErrorType, FaultInjectionResultBuilder, FaultInjectionRuleBuilder,
    };
    use std::sync::atomic::{AtomicU32, Ordering};

    /// A mock factory that creates mock transport clients.
    #[derive(Debug)]
    struct MockHttpClientFactory;

    impl HttpClientFactory for MockHttpClientFactory {
        fn build(
            &self,
            _connection_pool: &ConnectionPoolOptions,
            _config: HttpClientConfig,
        ) -> azure_core::Result<Arc<dyn TransportClient>> {
            Ok(Arc::new(MockTransportClient {
                call_count: AtomicU32::new(0),
            }))
        }
    }

    #[derive(Debug)]
    struct MockTransportClient {
        call_count: AtomicU32,
    }

    #[async_trait::async_trait]
    impl TransportClient for MockTransportClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            Ok(HttpResponse {
                status: 200,
                headers: azure_core::http::headers::Headers::new(),
                body: vec![],
            })
        }
    }

    #[test]
    fn factory_creates_fault_injecting_client() {
        let inner = Arc::new(MockHttpClientFactory);
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .build();
        let rule = Arc::new(FaultInjectionRuleBuilder::new("test-rule", error).build());

        let pool = ConnectionPoolOptions::default();
        let factory = FaultInjectingHttpClientFactory::new(inner, vec![rule]);
        let client = factory.build(
            &pool,
            HttpClientConfig::metadata(&pool, crate::diagnostics::TransportHttpVersion::Http11),
        );
        assert!(
            client.is_ok(),
            "factory should create a client successfully"
        );
    }
}
