// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-local HTTP client factory abstraction.

use std::{fmt, sync::Arc};

use super::cosmos_transport_client::TransportClient;

use crate::diagnostics::TransportHttpVersion;
use crate::options::ConnectionPoolOptions;

/// HTTP protocol policy required by a transport.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HttpVersionPolicy {
    /// Use HTTP/1.1 only. TCP keepalive is configured; no HTTP/2 keepalive.
    Http11Only,
    /// HTTP/2 only (`http2_prior_knowledge`), no HTTP/1.1 fallback.
    /// HTTP/2 keepalive is configured; TCP keepalive is not.
    Http2Only,
}

#[derive(Clone, Copy, Debug)]
pub struct HttpClientConfig {
    pub(crate) version_policy: HttpVersionPolicy,
    pub(crate) request_timeout: std::time::Duration,
    pub(crate) for_emulator: bool,
    pub(crate) http2_keep_alive_while_idle: bool,
}

impl HttpClientConfig {
    /// Config for metadata requests using the negotiated HTTP version.
    pub(crate) fn metadata(
        connection_pool: &ConnectionPoolOptions,
        negotiated_version: TransportHttpVersion,
    ) -> Self {
        Self {
            version_policy: match negotiated_version {
                TransportHttpVersion::Http2 => HttpVersionPolicy::Http2Only,
                TransportHttpVersion::Http11 => HttpVersionPolicy::Http11Only,
            },
            request_timeout: connection_pool.max_metadata_request_timeout(),
            for_emulator: false,
            http2_keep_alive_while_idle: negotiated_version.is_http2(),
        }
    }

    /// Config for dataplane gateway requests using the negotiated HTTP version.
    pub(crate) fn dataplane_gateway(
        connection_pool: &ConnectionPoolOptions,
        negotiated_version: TransportHttpVersion,
    ) -> Self {
        Self {
            version_policy: match negotiated_version {
                TransportHttpVersion::Http2 => HttpVersionPolicy::Http2Only,
                TransportHttpVersion::Http11 => HttpVersionPolicy::Http11Only,
            },
            request_timeout: connection_pool.max_dataplane_request_timeout(),
            for_emulator: false,
            http2_keep_alive_while_idle: negotiated_version.is_http2(),
        }
    }

    /// Config for Gateway 2.0 requests (always HTTP/2).
    pub(crate) fn dataplane_gateway20(connection_pool: &ConnectionPoolOptions) -> Self {
        Self {
            version_policy: HttpVersionPolicy::Http2Only,
            request_timeout: connection_pool.max_dataplane_request_timeout(),
            for_emulator: false,
            http2_keep_alive_while_idle: true,
        }
    }

    pub(crate) fn for_emulator(mut self) -> Self {
        self.for_emulator = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::ConnectionPoolOptionsBuilder;

    #[test]
    fn metadata_http11_version() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        assert_eq!(
            HttpClientConfig::metadata(&pool, TransportHttpVersion::Http11).version_policy,
            HttpVersionPolicy::Http11Only
        );
    }

    #[test]
    fn metadata_http2_version() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        assert_eq!(
            HttpClientConfig::metadata(&pool, TransportHttpVersion::Http2).version_policy,
            HttpVersionPolicy::Http2Only
        );
    }

    #[test]
    fn dataplane_gateway_http11_version() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        assert_eq!(
            HttpClientConfig::dataplane_gateway(&pool, TransportHttpVersion::Http11).version_policy,
            HttpVersionPolicy::Http11Only
        );
    }

    #[test]
    fn dataplane_gateway_http2_version() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        assert_eq!(
            HttpClientConfig::dataplane_gateway(&pool, TransportHttpVersion::Http2).version_policy,
            HttpVersionPolicy::Http2Only
        );
    }

    #[test]
    fn dataplane_gateway20_always_uses_http2_only() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        assert_eq!(
            HttpClientConfig::dataplane_gateway20(&pool).version_policy,
            HttpVersionPolicy::Http2Only
        );
    }

    #[test]
    fn for_emulator_sets_emulator_flag() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        let config = HttpClientConfig::metadata(&pool, TransportHttpVersion::Http2);
        assert!(!config.for_emulator);
        assert!(config.for_emulator().for_emulator);
    }
}

pub trait HttpClientFactory: fmt::Debug + Send + Sync {
    fn build(
        &self,
        connection_pool: &ConnectionPoolOptions,
        config: HttpClientConfig,
    ) -> azure_core::Result<Arc<dyn TransportClient>>;
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
    ) -> azure_core::Result<Arc<dyn TransportClient>> {
        let mut builder = reqwest::Client::builder();

        builder =
            builder.pool_max_idle_per_host(connection_pool.max_idle_connections_per_endpoint());

        if let Some(idle_timeout) = connection_pool.idle_connection_timeout() {
            builder = builder.pool_idle_timeout(idle_timeout);
        }

        builder = builder.connect_timeout(connection_pool.max_connect_timeout());
        builder = builder.timeout(config.request_timeout);

        if !connection_pool.proxy_allowed() {
            builder = builder.no_proxy();
        }

        if let Some(local_addr) = connection_pool.local_address() {
            builder = builder.local_address(local_addr);
        }

        if config.for_emulator {
            builder = builder.danger_accept_invalid_certs(true);
        }

        builder = match config.version_policy {
            HttpVersionPolicy::Http11Only => {
                // HTTP/1.1: TCP keepalive for connection liveness detection.
                builder = builder.tcp_keepalive(connection_pool.tcp_keepalive_time());
                if let Some(interval) = connection_pool.tcp_keepalive_interval() {
                    builder = builder.tcp_keepalive_interval(interval);
                }
                if let Some(retries) = connection_pool.tcp_keepalive_retries() {
                    builder = builder.tcp_keepalive_retries(retries);
                }
                builder.http1_only()
            }
            HttpVersionPolicy::Http2Only => {
                // HTTP/2: application-level keepalive via PING frames.
                // TCP keepalive is not needed — HTTP/2 PING frames serve that role.
                builder
                    .http2_keep_alive_interval(connection_pool.http2_keep_alive_interval())
                    .http2_keep_alive_timeout(connection_pool.http2_keep_alive_timeout())
                    .http2_keep_alive_while_idle(config.http2_keep_alive_while_idle)
                    .http2_prior_knowledge()
            }
        };

        let client = builder.build().map_err(|error| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("Failed to create HTTP client: {error}"),
            )
        })?;
        Ok(Arc::new(
            super::reqwest_transport_client::ReqwestTransportClient::new(client),
        ))
    }
}

#[cfg(not(feature = "reqwest"))]
impl HttpClientFactory for DefaultHttpClientFactory {
    fn build(
        &self,
        _connection_pool: &ConnectionPoolOptions,
        _config: HttpClientConfig,
    ) -> azure_core::Result<Arc<dyn TransportClient>> {
        Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "azure_data_cosmos_driver requires the `reqwest` feature to construct the default transport",
        ))
    }
}
