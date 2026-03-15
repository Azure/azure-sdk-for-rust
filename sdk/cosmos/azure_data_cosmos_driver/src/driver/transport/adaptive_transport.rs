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
/// `Gateway` is an unsharded HTTP/1.1 transport used when the gateway does not
/// support HTTP/2. `ShardedGateway` is a per-endpoint sharded HTTP/2 transport
/// used when HTTP/2 has been confirmed via the initialization probe.
/// `ShardedGateway20` is reserved for Gateway 2.0 thin-client requests and
/// always uses HTTP/2 prior knowledge.
#[derive(Clone)]
pub(crate) enum AdaptiveTransport {
    /// Unsharded HTTP/1.1 gateway transport (TCP keepalive, no HTTP/2 sharding).
    Gateway(Arc<dyn HttpClient>),
    /// Per-endpoint HTTP/2 sharded gateway transport (HTTP/2 keepalive, no TCP keepalive).
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
            HttpVersionPolicy::Http2Only => Self::ShardedGateway(Arc::new(
                ShardedHttpTransport::new(connection_pool.clone(), client_factory, config),
            )),
        })
    }

    /// Creates a Gateway 2.0 transport (always HTTP/2 sharded).
    pub(crate) fn gateway20(
        connection_pool: &ConnectionPoolOptions,
        client_factory: Arc<dyn HttpClientFactory>,
        config: HttpClientConfig,
    ) -> Self {
        Self::ShardedGateway20(Arc::new(ShardedHttpTransport::new(
            connection_pool.clone(),
            client_factory,
            config,
        )))
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

    /// Returns the shard ID that would be selected for the given request,
    /// without actually dispatching the request. Used to capture shard identity
    /// before a timeout race so that diagnostics can report which shard was
    /// targeted even when the transport future is cancelled.
    pub(crate) fn pre_select_shard(
        &self,
        request: &Request,
        excluded_shard_id: Option<u64>,
    ) -> Option<u64> {
        match self {
            Self::Gateway(_) => None,
            Self::ShardedGateway(transport) | Self::ShardedGateway20(transport) => {
                transport.pre_select_shard_id(request, excluded_shard_id)
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
