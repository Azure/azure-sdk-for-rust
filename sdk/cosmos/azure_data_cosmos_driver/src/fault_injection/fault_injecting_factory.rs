// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Factory decorator that wraps created HTTP clients with fault injection.

use std::sync::Arc;

use azure_core::http::HttpClient;

use super::fault_injecting_client::FaultInjectingHttpClient;
use super::rule::FaultInjectionRule;
use crate::driver::transport::http_client_factory::{HttpClientConfig, HttpClientFactory};
use crate::options::ConnectionPoolOptions;

/// An [`HttpClientFactory`] decorator that wraps clients with fault injection.
///
/// When `create` is called, this factory delegates to the inner factory to build
/// a real HTTP client, then wraps it in a [`FaultInjectingHttpClient`] that
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
    ) -> azure_core::Result<Arc<dyn HttpClient>> {
        let real_client = self.inner.build(connection_pool, config)?;
        let rules = (*self.rules).clone();
        Ok(Arc::new(FaultInjectingHttpClient::new(real_client, rules)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fault_injection::{
        FaultInjectionErrorType, FaultInjectionResultBuilder, FaultInjectionRuleBuilder,
    };
    use azure_core::http::{AsyncRawResponse, Request};
    use std::sync::atomic::{AtomicU32, Ordering};

    /// A mock factory that creates mock HTTP clients.
    #[derive(Debug)]
    struct MockHttpClientFactory;

    impl HttpClientFactory for MockHttpClientFactory {
        fn build(
            &self,
            _connection_pool: &ConnectionPoolOptions,
            _config: HttpClientConfig,
        ) -> azure_core::Result<Arc<dyn HttpClient>> {
            Ok(Arc::new(MockHttpClient {
                call_count: AtomicU32::new(0),
            }))
        }
    }

    #[derive(Debug)]
    struct MockHttpClient {
        call_count: AtomicU32,
    }

    #[async_trait::async_trait]
    impl HttpClient for MockHttpClient {
        async fn execute_request(
            &self,
            _request: &Request,
        ) -> azure_core::Result<AsyncRawResponse> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            Ok(AsyncRawResponse::from_bytes(
                azure_core::http::StatusCode::Ok,
                azure_core::http::headers::Headers::new(),
                vec![],
            ))
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
        let client = factory.build(&pool, HttpClientConfig::metadata(&pool));
        assert!(
            client.is_ok(),
            "factory should create a client successfully"
        );
    }
}
