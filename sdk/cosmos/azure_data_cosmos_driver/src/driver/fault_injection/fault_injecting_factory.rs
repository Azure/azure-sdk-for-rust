// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Factory that wraps each produced `HttpClient` with fault injection.
//!
//! The `FaultInjectingHttpClientFactory` wraps a real `HttpClientFactory` and produces
//! `FaultInjectingHttpClient` instances. Because the factory is called once per shard
//! creation, each shard gets its own fault-injecting wrapper while sharing a single rule
//! set — the `ShardedHttpTransport` is completely unaware that interception is happening.

use std::sync::{Arc, Mutex};

use azure_core::http::HttpClient;

use super::fault_injecting_client::FaultInjectingHttpClient;
use super::rule::FaultInjectionRule;
use crate::driver::transport::http_client_factory::{HttpClientConfig, HttpClientFactory};
use crate::options::ConnectionPoolOptions;

/// Factory that wraps each produced `HttpClient` with fault injection.
///
/// Implements [`HttpClientFactory`] by delegating to an inner factory and wrapping
/// each produced client with a [`FaultInjectingHttpClient`] that evaluates the shared
/// rule set before delegating to the real client.
#[derive(Debug)]
pub(crate) struct FaultInjectingHttpClientFactory {
    inner: Arc<dyn HttpClientFactory>,
    rules: Arc<Mutex<Vec<Arc<FaultInjectionRule>>>>,
}

impl FaultInjectingHttpClientFactory {
    /// Creates a new factory wrapping the given inner factory with the specified rules.
    pub(crate) fn new(
        inner: Arc<dyn HttpClientFactory>,
        rules: Vec<Arc<FaultInjectionRule>>,
    ) -> Self {
        Self {
            inner,
            rules: Arc::new(Mutex::new(rules)),
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
        let rules = self.rules.lock().unwrap().clone();
        Ok(Arc::new(FaultInjectingHttpClient::new(real_client, rules)))
    }
}

#[cfg(test)]
mod tests {
    use super::FaultInjectingHttpClientFactory;
    use crate::driver::fault_injection::{
        FaultInjectionErrorType, FaultInjectionResultBuilder, FaultInjectionRuleBuilder,
    };
    use crate::driver::transport::http_client_factory::{
        HttpClientConfig, HttpClientFactory, HttpVersionPolicy,
    };
    use crate::options::ConnectionPoolOptionsBuilder;
    use azure_core::http::{Method, Request, Url};
    use std::sync::Arc;

    #[tokio::test]
    async fn factory_wraps_produced_clients() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        let inner: Arc<dyn HttpClientFactory> = Arc::new(
            crate::driver::transport::http_client_factory::DefaultHttpClientFactory::new(),
        );

        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::InternalServerError)
            .build();
        let rule = Arc::new(FaultInjectionRuleBuilder::new("test-rule", error).build());

        let factory = FaultInjectingHttpClientFactory::new(inner, vec![rule]);

        let config = HttpClientConfig {
            version_policy: HttpVersionPolicy::Http11Only,
            request_timeout: std::time::Duration::from_secs(30),
            for_emulator: false,
        };

        let client = factory.build(&pool, config).unwrap();

        // The produced client should inject faults
        let request = Request::new(
            Url::parse("https://test.cosmos.azure.com/dbs/testdb").unwrap(),
            Method::Get,
        );

        let err = client.execute_request(&request).await.unwrap_err();
        assert_eq!(
            err.http_status(),
            Some(azure_core::http::StatusCode::InternalServerError)
        );
        assert!(
            matches!(
                err.kind(),
                azure_core::error::ErrorKind::HttpResponse { .. }
            ),
            "expected HttpResponse error kind, got {:?}",
            err.kind()
        );
    }
}
