// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-local HTTP client factory abstraction.

use std::{fmt, sync::Arc};

use super::cosmos_transport_client::TransportClient;

use crate::diagnostics::{TransportHttpVersion, TransportKind};
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
    pub(crate) allow_invalid_cert: bool,
    pub(crate) http2_keep_alive_while_idle: bool,
    /// The transport kind this HTTP client serves, when it is bound to a
    /// dataplane transport. Metadata clients (account discovery, etc.) leave
    /// this `None` because they are not gateway/Gateway-2.0-specific.
    ///
    /// This is consumed by the fault-injection layer so rules can scope
    /// themselves to a specific transport (`with_transport_kind`). The field
    /// has no readers when the `fault_injection` feature is disabled, so the
    /// dead-code warning is silenced only for that build configuration —
    /// when the feature is on, the field is read in
    /// `fault_injection::fault_injecting_factory::FaultInjectingHttpClientFactory::build`.
    #[cfg_attr(not(feature = "fault_injection"), allow(dead_code))]
    pub(crate) transport_kind: Option<TransportKind>,
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
            allow_invalid_cert: false,
            http2_keep_alive_while_idle: negotiated_version.is_http2(),
            transport_kind: None,
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
            allow_invalid_cert: false,
            http2_keep_alive_while_idle: negotiated_version.is_http2(),
            transport_kind: Some(TransportKind::Gateway),
        }
    }

    /// Config for Gateway 2.0 requests (always HTTP/2).
    pub(crate) fn dataplane_gateway_v2(connection_pool: &ConnectionPoolOptions) -> Self {
        Self {
            version_policy: HttpVersionPolicy::Http2Only,
            request_timeout: connection_pool.max_dataplane_request_timeout(),
            allow_invalid_cert: false,
            http2_keep_alive_while_idle: true,
            transport_kind: Some(TransportKind::GatewayV2),
        }
    }

    pub(crate) fn with_allow_invalid_cert(mut self) -> Self {
        self.allow_invalid_cert = true;
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
    fn dataplane_gateway_v2_always_uses_http2_only() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        assert_eq!(
            HttpClientConfig::dataplane_gateway_v2(&pool).version_policy,
            HttpVersionPolicy::Http2Only
        );
    }

    #[test]
    fn with_allow_invalid_cert_sets_flag() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        let config = HttpClientConfig::metadata(&pool, TransportHttpVersion::Http2);
        assert!(!config.allow_invalid_cert);
        assert!(config.with_allow_invalid_cert().allow_invalid_cert);
    }

    #[cfg(feature = "rustls")]
    #[test]
    fn default_factory_builds_client_with_default_tls_backend() {
        // The default backend is `TlsBackend::Rustls`; building the client must
        // succeed (i.e. `tls_backend_rustls()` is wired up under the `rustls`
        // feature).
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        assert_eq!(pool.tls_backend(), crate::options::TlsBackend::Rustls);
        let config = HttpClientConfig::metadata(&pool, TransportHttpVersion::Http2);
        let factory = DefaultHttpClientFactory::new();
        assert!(
            factory.build(&pool, config).is_ok(),
            "building the reqwest client with the default TLS backend should succeed"
        );
    }
}

pub trait HttpClientFactory: fmt::Debug + Send + Sync {
    fn build(
        &self,
        connection_pool: &ConnectionPoolOptions,
        config: HttpClientConfig,
    ) -> crate::error::Result<Arc<dyn TransportClient>>;
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
    ) -> crate::error::Result<Arc<dyn TransportClient>> {
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

        if config.allow_invalid_cert {
            #[cfg(feature = "__tls")]
            {
                builder = builder.danger_accept_invalid_certs(true);
            }
        }

        // Enforce the selected TLS backend on the reqwest transport. The driver
        // does not otherwise expose the transport, so this is the supported way
        // to assert a specific backend. The whole block compiles in only under
        // the `rustls` feature: `tls_backend_rustls()` (and the `tls_backend()`
        // accessor it reads) exist only then. With a different TLS feature the
        // backend is not driver-selectable and reqwest's own default applies.
        #[cfg(feature = "rustls")]
        {
            builder = match connection_pool.tls_backend() {
                crate::options::TlsBackend::Rustls => builder.tls_backend_rustls(),
            };
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
            // HTTP client construction is caller-controlled configuration
            // (TLS / pool sizing / version pinning), so surface it as a typed
            // configuration error.
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED)
                .with_message("failed to create HTTP client")
                .with_source(error)
                .build()
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
    ) -> crate::error::Result<Arc<dyn TransportClient>> {
        Err(crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::CLIENT_REQWEST_FEATURE_REQUIRED)
            .with_message(
                "azure_data_cosmos_driver requires the `reqwest` feature to construct the default transport",
            )
            .build()
            .into())
    }
}
