// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Factory decorator that wraps created HTTP clients with fault injection.

use std::sync::{Arc, Mutex};

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
#[allow(dead_code)]
pub(crate) struct FaultInjectingHttpClientFactory {
    inner: Arc<dyn HttpClientFactory>,
    rules: Arc<Mutex<Vec<Arc<FaultInjectionRule>>>>,
}

impl FaultInjectingHttpClientFactory {
    /// Creates a new factory that wraps clients from `inner` with fault injection rules.
    #[allow(dead_code)]
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
