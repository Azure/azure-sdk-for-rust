// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Endpoint and routing index primitives.

use std::sync::Arc;

use crate::{driver::transport::EndpointKey, options::Region};
use url::Url;

/// The reference-counted inner data for a [`CosmosEndpoint`].
#[derive(Debug, PartialEq, Eq, Hash)]
struct CosmosEndpointData {
    region: Option<Region>,
    gateway_url: Url,
    gateway20_url: Option<Url>,
    /// Pre-computed `host:port` key for the gateway URL.
    ///
    /// Stored as `Arc<str>` so cloning the [`EndpointKey`] (once per transport
    /// pipeline invocation) is a cheap atomic refcount increment rather than a
    /// heap allocation.
    endpoint_key: EndpointKey,
}

/// A Cosmos DB service endpoint.
///
/// Cloning is cheap — the URL and region data are reference-counted.
#[derive(Clone, Debug)]
pub(crate) struct CosmosEndpoint(Arc<CosmosEndpointData>);

impl PartialEq for CosmosEndpoint {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0
    }
}

impl Eq for CosmosEndpoint {}

impl std::hash::Hash for CosmosEndpoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl CosmosEndpoint {
    /// Creates a global endpoint.
    pub fn global(url: Url) -> Self {
        let endpoint_key = EndpointKey::try_from(&url)
            .expect("CosmosEndpoint URL must have a valid host and port");
        Self(Arc::new(CosmosEndpointData {
            region: None,
            gateway_url: url,
            gateway20_url: None,
            endpoint_key,
        }))
    }

    /// Creates a regional endpoint.
    pub fn regional(region: Region, url: Url) -> Self {
        let endpoint_key = EndpointKey::try_from(&url)
            .expect("CosmosEndpoint URL must have a valid host and port");
        Self(Arc::new(CosmosEndpointData {
            region: Some(region),
            gateway_url: url,
            gateway20_url: None,
            endpoint_key,
        }))
    }

    /// Creates a regional endpoint with an optional Gateway 2.0 URL.
    pub fn regional_with_gateway20(region: Region, gateway_url: Url, gateway20_url: Url) -> Self {
        let endpoint_key = EndpointKey::try_from(&gateway_url)
            .expect("CosmosEndpoint URL must have a valid host and port");
        Self(Arc::new(CosmosEndpointData {
            region: Some(region),
            gateway_url,
            gateway20_url: Some(gateway20_url),
            endpoint_key,
        }))
    }

    /// Returns the region, if this is a regional endpoint.
    pub fn region(&self) -> Option<&Region> {
        self.0.region.as_ref()
    }

    /// Returns the standard gateway URL for this endpoint.
    pub fn url(&self) -> &Url {
        &self.0.gateway_url
    }

    /// Returns the pre-computed `host:port` endpoint key for the gateway URL.
    ///
    /// Cloning is cheap — the key is backed by `Arc<str>`.
    pub(crate) fn endpoint_key(&self) -> EndpointKey {
        self.0.endpoint_key.clone()
    }

    /// Returns the Gateway 2.0 URL for this endpoint, if available.
    #[cfg(test)]
    pub fn gateway20_url(&self) -> Option<&Url> {
        self.0.gateway20_url.as_ref()
    }

    /// Returns `true` when Gateway 2.0 should be used for this attempt.
    pub(crate) fn uses_gateway20(&self, prefer_gateway20: bool) -> bool {
        prefer_gateway20 && self.0.gateway20_url.is_some()
    }

    /// Returns the concrete URL selected for this attempt.
    pub(crate) fn selected_url(&self, prefer_gateway20: bool) -> &Url {
        if prefer_gateway20 {
            if let Some(url) = &self.0.gateway20_url {
                return url;
            }
        }

        &self.0.gateway_url
    }
}

/// Type-safe index into preferred endpoint lists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LocationIndex {
    index: usize,
    generation: u64,
}

impl LocationIndex {
    /// Creates an initial index at position 0.
    pub fn initial(generation: u64) -> Self {
        Self {
            index: 0,
            generation,
        }
    }

    /// Advances to the next index, wrapping within list bounds.
    pub fn next(self, list_len: usize) -> Self {
        debug_assert!(list_len > 0, "endpoint list should never be empty");
        if list_len == 0 {
            return self;
        }

        Self {
            index: (self.index + 1) % list_len,
            generation: self.generation,
        }
    }

    /// Advances to the next index for the provided generation.
    ///
    /// When the generation changes, stale indices are rebased to the first
    /// endpoint in the refreshed snapshot before advancing.
    pub fn next_for_generation(self, list_len: usize, generation: u64) -> Self {
        debug_assert!(list_len > 0, "endpoint list should never be empty");
        if list_len == 0 {
            return self;
        }

        if self.generation == generation {
            return self.next(list_len);
        }

        let base_index = 0;

        Self {
            index: (base_index + 1) % list_len,
            generation,
        }
    }

    /// Returns the numeric index.
    pub fn index(self) -> usize {
        self.index
    }

    /// Returns true when this index matches the provided generation.
    pub fn is_current(self, generation: u64) -> bool {
        self.generation == generation
    }
}

/// Why an endpoint was marked unavailable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum UnavailableReason {
    WriteForbidden,
    ServiceUnavailable,
    InternalServerError,
    TransportError,
}

#[cfg(test)]
mod tests {
    use super::LocationIndex;

    #[test]
    fn next_for_generation_rebases_stale_index_before_advancing() {
        let stale = LocationIndex::initial(3).next(3).next(3);

        let next = stale.next_for_generation(3, 4);

        assert_eq!(next.index(), 1);
        assert!(next.is_current(4));
    }
}
