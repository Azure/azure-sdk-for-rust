// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Adaptive HTTP transport variants for Step 5.

use std::{collections::HashMap, fmt, sync::Arc};

use azure_core::http::{AsyncRawResponse, HttpClient, Request};
use url::Url;

use crate::{driver::cache::AccountProperties, options::Region};

use super::http_client_factory::HttpVersionPolicy;

/// Transport strategy selected for a request pipeline.
#[derive(Clone)]
pub(crate) enum AdaptiveTransport {
    /// HTTP/1.1-only transport.
    Http1(Arc<dyn HttpClient>),
    /// HTTP/2-preferred transport that still accepts HTTP/1.1 fallback.
    Http2Preferred(Arc<dyn HttpClient>),
    /// HTTP/2-only transport.
    Http2Only(Arc<dyn HttpClient>),
}

impl AdaptiveTransport {
    pub(crate) fn from_policy(
        policy: HttpVersionPolicy,
        client: Arc<dyn HttpClient>,
    ) -> Self {
        match policy {
            HttpVersionPolicy::Http1Only => Self::Http1(client),
            HttpVersionPolicy::Http2Preferred => Self::Http2Preferred(client),
            HttpVersionPolicy::Http2Only => Self::Http2Only(client),
        }
    }

    pub(crate) fn version_policy(&self) -> HttpVersionPolicy {
        match self {
            Self::Http1(_) => HttpVersionPolicy::Http1Only,
            Self::Http2Preferred(_) => HttpVersionPolicy::Http2Preferred,
            Self::Http2Only(_) => HttpVersionPolicy::Http2Only,
        }
    }

    pub(crate) async fn send(&self, request: &Request) -> azure_core::Result<AsyncRawResponse> {
        self.client().execute_request(request).await
    }

    fn client(&self) -> &Arc<dyn HttpClient> {
        match self {
            Self::Http1(client) | Self::Http2Preferred(client) | Self::Http2Only(client) => {
                client
            }
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

/// Builds a region-to-endpoint map for Gateway 2.0 (thin client) routing.
pub(crate) fn thin_client_endpoint_overrides(
    properties: &AccountProperties,
    read_only: bool,
) -> HashMap<Region, Url> {
    let regions = if read_only {
        &properties.thin_client_readable_locations
    } else {
        &properties.thin_client_writable_locations
    };

    regions
        .iter()
        .filter_map(|region| {
            Url::parse(&region.database_account_endpoint)
                .ok()
                .map(|url| (region.name.clone(), url))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_thin_client_read_and_write_overrides() {
        let properties: AccountProperties = serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [],
            "readableLocations": [],
            "enableMultipleWriteLocations": false,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}",
            "thinClientReadableLocations": [
                {
                    "name": "westus2",
                    "databaseAccountEndpoint": "https://test-westus2-thin.documents.azure.com:444/"
                }
            ],
            "thinClientWritableLocations": [
                {
                    "name": "eastus",
                    "databaseAccountEndpoint": "https://test-eastus-thin.documents.azure.com:444/"
                }
            ]
        }))
        .unwrap();

        let read_overrides = thin_client_endpoint_overrides(&properties, true);
        let write_overrides = thin_client_endpoint_overrides(&properties, false);

        assert_eq!(read_overrides.len(), 1);
        assert_eq!(
            read_overrides[&Region::new("westus2")].as_str(),
            "https://test-westus2-thin.documents.azure.com:444/"
        );
        assert_eq!(write_overrides.len(), 1);
        assert_eq!(
            write_overrides[&Region::new("eastus")].as_str(),
            "https://test-eastus-thin.documents.azure.com:444/"
        );
    }
}
