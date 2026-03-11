// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Adaptive HTTP transport layer.
//!
//! Provides [`AdaptiveTransport`] for transport-aware dispatch and
//! [`TransportContext`] which bundles a transport with optional
//! Gateway 2.0 endpoint overrides.

use std::{collections::HashMap, fmt, sync::Arc};

use azure_core::http::{AsyncRawResponse, HttpClient, Request};
use tracing::warn;
use url::Url;

use crate::{
    diagnostics::TransportKind, driver::cache::AccountProperties, options::Region,
};

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

    fn transport_kind(&self) -> &'static str {
        match self {
            Self::Gateway(_) => "Gateway",
            Self::Gateway20(_) => "Gateway20",
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
    pub(crate) async fn send(&self, request: &Request) -> azure_core::Result<AsyncRawResponse> {
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
            .field("kind", &self.transport_kind())
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
    let mut map = HashMap::new();
    let entries = properties
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
        );

    for (region, url) in entries {
        if let Some(existing) = map.get(&region) {
            if *existing != url {
                warn!(
                    %region,
                    existing_url = %existing,
                    new_url = %url,
                    "Duplicate thin-client region with conflicting URL; keeping first entry"
                );
                continue;
            }
        }
        map.insert(region, url);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::transport::tests::{
        account_properties_with_duplicate_region_conflicting_url,
        account_properties_with_duplicate_region_same_url,
        account_properties_with_partially_malformed_thin_client,
        account_properties_with_thin_client,
    };

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

    #[test]
    fn ignores_malformed_thin_client_endpoints() {
        let properties = account_properties_with_partially_malformed_thin_client();
        let overrides = thin_client_endpoint_overrides(&properties);

        assert_eq!(overrides.len(), 1);
        assert!(!overrides.contains_key(&Region::new("westus2")));
        assert_eq!(
            overrides[&Region::new("eastus")].as_str(),
            "https://test-eastus-thin.documents.azure.com:444/"
        );
    }

    #[test]
    fn duplicate_region_same_url_deduplicates() {
        let properties = account_properties_with_duplicate_region_same_url();
        let overrides = thin_client_endpoint_overrides(&properties);

        // westus2 appears in both read and write with the same URL — should deduplicate to 1 entry.
        assert_eq!(overrides.len(), 1);
        assert_eq!(
            overrides[&Region::new("westus2")].as_str(),
            "https://test-westus2-thin.documents.azure.com:444/"
        );
    }

    #[test]
    fn duplicate_region_conflicting_url_keeps_first() {
        let properties = account_properties_with_duplicate_region_conflicting_url();
        let overrides = thin_client_endpoint_overrides(&properties);

        // westus2 appears in both read and write with different URLs —
        // should keep the first (readable) entry.
        assert_eq!(overrides.len(), 1);
        assert_eq!(
            overrides[&Region::new("westus2")].as_str(),
            "https://test-westus2-read-thin.documents.azure.com:444/"
        );
    }
}
