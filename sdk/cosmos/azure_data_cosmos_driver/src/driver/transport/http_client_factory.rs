// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-local HTTP client factory abstraction.

use std::{fmt, sync::Arc};

use azure_core::http::HttpClient;

use crate::options::ConnectionPoolOptions;

/// HTTP protocol policy required by a transport.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HttpVersionPolicy {
    Http1Only,
    Http2Preferred,
    Http2Only,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct HttpClientConfig {
    pub(crate) version_policy: HttpVersionPolicy,
    pub(crate) request_timeout: std::time::Duration,
    pub(crate) for_emulator: bool,
}

impl HttpClientConfig {
    pub(crate) fn metadata(connection_pool: &ConnectionPoolOptions) -> Self {
        Self {
            version_policy: if connection_pool.is_http2_allowed() {
                HttpVersionPolicy::Http2Preferred
            } else {
                HttpVersionPolicy::Http1Only
            },
            request_timeout: connection_pool.max_metadata_request_timeout(),
            for_emulator: false,
        }
    }

    pub(crate) fn dataplane_gateway(connection_pool: &ConnectionPoolOptions) -> Self {
        Self {
            version_policy: if connection_pool.is_http2_allowed() {
                HttpVersionPolicy::Http2Preferred
            } else {
                HttpVersionPolicy::Http1Only
            },
            request_timeout: connection_pool.max_dataplane_request_timeout(),
            for_emulator: false,
        }
    }

    pub(crate) fn dataplane_gateway20(connection_pool: &ConnectionPoolOptions) -> Self {
        Self {
            version_policy: HttpVersionPolicy::Http2Only,
            request_timeout: connection_pool.max_dataplane_request_timeout(),
            for_emulator: false,
        }
    }

    pub(crate) fn for_emulator(mut self) -> Self {
        self.for_emulator = true;
        self
    }
}

pub(crate) trait HttpClientFactory: fmt::Debug + Send + Sync {
    fn build(
        &self,
        connection_pool: &ConnectionPoolOptions,
        config: HttpClientConfig,
    ) -> azure_core::Result<Arc<dyn HttpClient>>;
}

#[derive(Debug)]
pub(crate) struct DefaultHttpClientFactory;

impl DefaultHttpClientFactory {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[cfg(feature = "reqwest")]
impl HttpClientFactory for DefaultHttpClientFactory {
    fn build(
        &self,
        connection_pool: &ConnectionPoolOptions,
        config: HttpClientConfig,
    ) -> azure_core::Result<Arc<dyn HttpClient>> {
        let mut builder = reqwest::Client::builder();

        builder = builder.pool_max_idle_per_host(connection_pool.max_idle_connections_per_endpoint());

        if let Some(idle_timeout) = connection_pool.idle_connection_timeout() {
            builder = builder.pool_idle_timeout(idle_timeout);
        }

        builder = builder.connect_timeout(connection_pool.max_connect_timeout());
        builder = builder.timeout(config.request_timeout);

        if !connection_pool.is_proxy_allowed() {
            builder = builder.no_proxy();
        }

        if let Some(local_addr) = connection_pool.local_address() {
            builder = builder.local_address(local_addr);
        }

        if config.for_emulator {
            builder = builder.danger_accept_invalid_certs(true);
        }

        builder = match config.version_policy {
            HttpVersionPolicy::Http1Only => builder.http1_only(),
            HttpVersionPolicy::Http2Preferred => builder,
            HttpVersionPolicy::Http2Only => builder.http2_prior_knowledge(),
        };

        let client = builder.build().map_err(|error| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("Failed to create HTTP client: {error}"),
            )
        })?;
        Ok(Arc::new(client))
    }
}

#[cfg(not(feature = "reqwest"))]
impl HttpClientFactory for DefaultHttpClientFactory {
    fn build(
        &self,
        _connection_pool: &ConnectionPoolOptions,
        _config: HttpClientConfig,
    ) -> azure_core::Result<Arc<dyn HttpClient>> {
        Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "azure_data_cosmos_driver requires the `reqwest` feature to construct the default transport",
        ))
    }
}
