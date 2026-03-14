// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Adaptive HTTP transport layer.

use std::{fmt, sync::Arc};

use azure_core::http::{AsyncRawResponse, HttpClient, Request};

use crate::diagnostics::TransportKind;

use super::{
    http_client_factory::{HttpClientConfig, HttpClientFactory, HttpVersionPolicy},
    sharded_transport::{ShardedHttpTransport, TransportDispatch},
};
use crate::options::ConnectionPoolOptions;

/// Transport strategy selected for a request pipeline.
///
/// `Gateway` covers the standard metadata and dataplane gateway path. The
/// underlying reqwest client may be configured as HTTP/1.1-only or
/// HTTP/2-preferred depending on `ConnectionPoolOptions::is_http2_allowed()`.
/// `Gateway20` is reserved for thin-client Gateway 2.0 requests and always
/// uses HTTP/2 prior knowledge.
///
/// In Step 6, both variants will transition to wrapping
/// `ShardedHttpTransport` instead of a plain `Arc<dyn HttpClient>` —
/// see spec §6 "HTTP/2 connection sharding".
#[derive(Clone)]
pub(crate) enum AdaptiveTransport {
    /// Standard gateway transport for metadata and non-Gateway-2.0 dataplane requests.
    Gateway(Arc<dyn HttpClient>),
    /// Standard gateway transport with per-endpoint HTTP/2 sharding.
    ShardedGateway(Arc<ShardedHttpTransport>),
    /// Gateway 2.0 transport with per-endpoint HTTP/2 sharding.
    ShardedGateway20(Arc<ShardedHttpTransport>),
}

impl AdaptiveTransport {
    pub(crate) fn from_config(
        connection_pool: &ConnectionPoolOptions,
        client_factory: Arc<dyn HttpClientFactory>,
        config: HttpClientConfig,
    ) -> azure_core::Result<Self> {
        Ok(match config.version_policy {
            HttpVersionPolicy::Http11Only => {
                Self::Gateway(client_factory.build(connection_pool, config)?)
            }
            HttpVersionPolicy::Http2Preferred => Self::ShardedGateway(Arc::new(
                ShardedHttpTransport::new(connection_pool.clone(), client_factory, config),
            )),
            HttpVersionPolicy::Http2Only => Self::ShardedGateway20(Arc::new(
                ShardedHttpTransport::new(connection_pool.clone(), client_factory, config),
            )),
        })
    }

    /// Returns the [`TransportKind`] for diagnostics reporting.
    pub(crate) fn diagnostics_kind(&self) -> TransportKind {
        match self {
            Self::Gateway(_) | Self::ShardedGateway(_) => TransportKind::Gateway,
            Self::ShardedGateway20(_) => TransportKind::Gateway20,
        }
    }

    /// Sends an HTTP request through the underlying transport.
    pub(crate) async fn send(&self, request: &Request) -> azure_core::Result<AsyncRawResponse> {
        self.send_with_dispatch(request, None).await.result
    }

    pub(crate) async fn send_with_dispatch(
        &self,
        request: &Request,
        excluded_shard_id: Option<u64>,
    ) -> TransportDispatch {
        match self {
            Self::Gateway(client) => TransportDispatch {
                result: client.execute_request(request).await,
                shard_id: None,
                shard_diagnostics: None,
            },
            Self::ShardedGateway(transport) | Self::ShardedGateway20(transport) => {
                transport.send(request, excluded_shard_id).await
            }
        }
    }

    pub(crate) fn can_retry_on_different_shard(
        &self,
        request: &Request,
        excluded_shard_id: u64,
    ) -> bool {
        match self {
            Self::Gateway(_) => false,
            Self::ShardedGateway(transport) | Self::ShardedGateway20(transport) => {
                transport.can_retry_on_different_shard(request, excluded_shard_id)
            }
        }
    }
}

impl fmt::Debug for AdaptiveTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdaptiveTransport")
            .field("kind", &self.diagnostics_kind().as_ref())
            .finish_non_exhaustive()
    }
}
