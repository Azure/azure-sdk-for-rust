// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Adaptive HTTP transport layer.

use std::{fmt, sync::Arc};

use azure_core::http::{AsyncRawResponse, HttpClient, Request};

use crate::diagnostics::TransportKind;

use super::http_client_factory::HttpVersionPolicy;

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
    /// Gateway 2.0 transport for thin-client dataplane requests.
    Gateway20(Arc<dyn HttpClient>),
}

impl AdaptiveTransport {
    pub(crate) fn from_policy(policy: HttpVersionPolicy, client: Arc<dyn HttpClient>) -> Self {
        match policy {
            HttpVersionPolicy::Http11Only | HttpVersionPolicy::Http2Preferred => {
                Self::Gateway(client)
            }
            HttpVersionPolicy::Http2Only => Self::Gateway20(client),
        }
    }

    /// Returns the [`TransportKind`] for diagnostics reporting.
    pub(crate) fn diagnostics_kind(&self) -> TransportKind {
        match self {
            Self::Gateway(_) => TransportKind::Gateway,
            Self::Gateway20(_) => TransportKind::Gateway20,
        }
    }

    /// Sends an HTTP request through the underlying transport.
    ///
    // TODO(Step 6): When sharding is added, `Gateway` and `Gateway20`
    // variants will dispatch through `ShardedHttpTransport` instead of
    // delegating directly to the `HttpClient`.
    #[tracing::instrument(level = tracing::Level::TRACE, skip_all, err, fields(
        method = %request.method(),
        url = %request.url(),
    ))]
    pub(crate) async fn send(&self, request: &Request) -> azure_core::Result<AsyncRawResponse> {
        tracing::trace!("sending request");
        self.client().execute_request(request).await
    }

    fn client(&self) -> &Arc<dyn HttpClient> {
        match self {
            Self::Gateway(client) | Self::Gateway20(client) => client,
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
