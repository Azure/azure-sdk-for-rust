// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Adaptive HTTP transport layer.
//!
//! Provides [`AdaptiveTransport`] for protocol-aware dispatch and
//! [`TransportContext`] which bundles a transport with optional
//! Gateway 2.0 endpoint overrides.

use std::{collections::HashMap, fmt, sync::Arc};

use azure_core::http::{AsyncRawResponse, HttpClient, Request};
use tracing::warn;
use url::Url;

use crate::{driver::cache::AccountProperties, options::Region};

use super::http_client_factory::HttpVersionPolicy;

/// Transport strategy selected for a request pipeline.
///
/// Both metadata and standard dataplane transports use `Http2Preferred`,
/// which negotiates HTTP/2 via ALPN and falls back to HTTP/1.1 if the
/// server (e.g. RoutingGateway) does not support it. Gateway 2.0 uses
/// `Http2Only` with `http2_prior_knowledge()` (no fallback).
///
/// In Step 6, both variants will transition to wrapping
/// `ShardedHttpTransport` instead of a plain `Arc<dyn HttpClient>` —
/// see spec §6 "HTTP/2 connection sharding".
#[derive(Clone)]
pub(crate) enum AdaptiveTransport {
    /// HTTP/2-preferred transport with automatic HTTP/1.1 fallback.
    /// Used for both RoutingGateway (falls back to HTTP/1.1) and
    /// ComputeGateway (negotiates HTTP/2 via ALPN).
    Http2Preferred(Arc<dyn HttpClient>),
    /// HTTP/2-only transport, no HTTP/1.1 fallback (Gateway 2.0).
    Http2Only(Arc<dyn HttpClient>),
}

impl AdaptiveTransport {
    pub(crate) fn from_policy(policy: HttpVersionPolicy, client: Arc<dyn HttpClient>) -> Self {
        match policy {
            HttpVersionPolicy::Http2Preferred => Self::Http2Preferred(client),
            HttpVersionPolicy::Http2Only => Self::Http2Only(client),
        }
    }

    pub(crate) fn version_policy(&self) -> HttpVersionPolicy {
        match self {
            Self::Http2Preferred(_) => HttpVersionPolicy::Http2Preferred,
            Self::Http2Only(_) => HttpVersionPolicy::Http2Only,
        }
    }

    /// Sends an HTTP request through the underlying transport.
    ///
    // TODO(Step 6): When sharding is added, `Http2Preferred` and `Http2Only`
    // variants will dispatch through `ShardedHttpTransport` instead of
    // delegating directly to the `HttpClient`.
    pub(crate) async fn send(&self, request: &Request) -> azure_core::Result<AsyncRawResponse> {
        self.client().execute_request(request).await
    }

    fn client(&self) -> &Arc<dyn HttpClient> {
        match self {
            Self::Http2Preferred(client) | Self::Http2Only(client) => client,
        }
    }
}

impl fmt::Debug for AdaptiveTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AdaptiveTransport")
            .field("version_policy", &self.version_policy())
            .finish_non_exhaustive()
    }
}

/// Bundles an [`AdaptiveTransport`] with optional Gateway 2.0 endpoint
/// overrides, determined once per operation and threaded through the pipeline.
#[derive(Clone, Debug)]
pub(crate) struct TransportContext {
    /// The adaptive transport to use for HTTP requests.
    pub transport: AdaptiveTransport,
    /// Gateway 2.0 (thin client) endpoint overrides, keyed by region.
    /// Present only when Gateway 2.0 is enabled and the account has
    /// thin-client endpoints. Contains **both** read and write locations
    /// merged into a single map so that retries across read/write
    /// endpoint lists do not miss regions.
    pub thin_client_overrides: Option<Arc<HashMap<Region, Url>>>,
    /// Whether Gateway 2.0 was selected for this transport.
    #[allow(dead_code)] // Used for diagnostics and future routing decisions.
    pub is_gateway20: bool,
}

/// Builds a region-to-endpoint map for Gateway 2.0 (thin client) routing.
///
/// Merges **both** read and write thin-client locations into a single map.
/// This is necessary because session retries on single-write accounts can
/// reroute reads to write endpoints — if only read locations were included,
/// the write-region URL override would be missing, causing requests to be
/// sent to the wrong gateway endpoint while using HTTP/2-only transport.
pub(crate) fn thin_client_endpoint_overrides(
    properties: &AccountProperties,
) -> HashMap<Region, Url> {
    properties
        .thin_client_readable_locations
        .iter()
        .chain(properties.thin_client_writable_locations.iter())
        .filter_map(
            |region| match Url::parse(&region.database_account_endpoint) {
                Ok(url) => Some((region.name.clone(), url)),
                Err(err) => {
                    warn!(
                        region = %region.name,
                        endpoint = %region.database_account_endpoint,
                        error = %err,
                        "Ignoring malformed thin-client endpoint URL from AccountProperties"
                    );
                    None
                }
            },
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::transport::tests::account_properties_with_thin_client;

    #[test]
    fn merges_read_and_write_thin_client_overrides() {
        let properties = account_properties_with_thin_client();
        let overrides = thin_client_endpoint_overrides(&properties);

        // Both read and write locations should be in one map.
        assert_eq!(overrides.len(), 2);
        assert_eq!(
            overrides[&Region::new("westus2")].as_str(),
            "https://test-westus2-thin.documents.azure.com:444/"
        );
        assert_eq!(
            overrides[&Region::new("eastus")].as_str(),
            "https://test-eastus-thin.documents.azure.com:444/"
        );
    }
}
