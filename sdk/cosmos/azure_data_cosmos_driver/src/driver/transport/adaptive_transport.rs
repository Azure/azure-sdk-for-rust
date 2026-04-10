// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Adaptive HTTP transport layer.

use std::{fmt, sync::Arc};

use crate::diagnostics::{TransportHttpVersion, TransportKind};

use super::{
    cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError},
    http_client_factory::{HttpClientConfig, HttpClientFactory, HttpVersionPolicy},
    sharded_transport::{EndpointKey, ShardedHttpTransport, TransportDispatch},
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
    Gateway(Arc<dyn TransportClient>),
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

    /// Creates an unsharded transport wrapping a single HTTP client.
    ///
    /// Used for lightweight one-shot transports (e.g., the bootstrap probe)
    /// where the overhead of sharding and background health sweeps is
    /// unnecessary.
    pub(crate) fn unsharded(
        connection_pool: &ConnectionPoolOptions,
        client_factory: Arc<dyn HttpClientFactory>,
        config: HttpClientConfig,
    ) -> azure_core::Result<Self> {
        Ok(Self::Gateway(
            client_factory.build(connection_pool, config)?,
        ))
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

    /// Returns the HTTP protocol version used by this transport.
    pub(crate) fn diagnostics_http_version(&self) -> TransportHttpVersion {
        match self {
            Self::Gateway(_) => TransportHttpVersion::Http11,
            Self::ShardedGateway(_) | Self::ShardedGateway20(_) => TransportHttpVersion::Http2,
        }
    }

    /// Sends an HTTP request through the underlying transport.
    pub(crate) async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        match self {
            Self::Gateway(client) => client.send(request).await,
            Self::ShardedGateway(transport) | Self::ShardedGateway20(transport) => {
                let endpoint_key = EndpointKey::try_from(&request.url).map_err(|e| {
                    TransportError::new(e, crate::diagnostics::RequestSentStatus::NotSent)
                })?;
                transport
                    .send(request, None, &endpoint_key, None)
                    .await
                    .result
            }
        }
    }

    pub(crate) async fn send_with_dispatch(
        &self,
        request: &HttpRequest,
        excluded_shard_id: Option<u64>,
        endpoint_key: &EndpointKey,
        preferred_shard_id: Option<u64>,
    ) -> TransportDispatch {
        match self {
            Self::Gateway(client) => TransportDispatch {
                result: client.send(request).await,
                shard_id: None,
                shard_diagnostics: None,
            },
            Self::ShardedGateway(transport) | Self::ShardedGateway20(transport) => {
                transport
                    .send(request, excluded_shard_id, endpoint_key, preferred_shard_id)
                    .await
            }
        }
    }

    pub(crate) fn can_retry_on_different_shard(
        &self,
        excluded_shard_id: u64,
        endpoint_key: &EndpointKey,
    ) -> bool {
        match self {
            Self::Gateway(_) => false,
            Self::ShardedGateway(transport) | Self::ShardedGateway20(transport) => {
                transport.can_retry_on_different_shard(excluded_shard_id, endpoint_key)
            }
        }
    }

    /// Returns the shard ID that would be selected for the given request,
    /// without actually dispatching the request. Used to capture shard identity
    /// before a timeout race so that diagnostics can report which shard was
    /// targeted even when the transport future is cancelled.
    pub(crate) fn pre_select_shard(
        &self,
        excluded_shard_id: Option<u64>,
        endpoint_key: &EndpointKey,
    ) -> Option<u64> {
        match self {
            Self::Gateway(_) => None,
            Self::ShardedGateway(transport) | Self::ShardedGateway20(transport) => {
                transport.pre_select_shard_id(excluded_shard_id, endpoint_key)
            }
        }
    }
}

impl fmt::Debug for AdaptiveTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdaptiveTransport")
            .field("kind", &self.diagnostics_kind().as_ref())
            .field("http_version", &self.diagnostics_http_version().as_ref())
            .finish_non_exhaustive()
    }
}
