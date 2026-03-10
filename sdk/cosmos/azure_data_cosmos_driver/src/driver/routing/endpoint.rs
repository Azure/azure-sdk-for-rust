// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Endpoint and routing index primitives.

use crate::options::Region;
use url::Url;

/// A Cosmos DB service endpoint.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct CosmosEndpoint {
    region: Option<Region>,
    url: Url,
    kind: EndpointKind,
}

impl CosmosEndpoint {
    /// Creates a global endpoint.
    pub fn global(url: Url) -> Self {
        Self {
            region: None,
            url,
            kind: EndpointKind::Global,
        }
    }

    /// Creates a regional endpoint.
    pub fn regional(region: Region, url: Url) -> Self {
        Self {
            region: Some(region),
            url,
            kind: EndpointKind::Regional,
        }
    }

    /// Returns the region, if this is a regional endpoint.
    pub fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    /// Returns the endpoint URL.
    pub fn url(&self) -> &Url {
        &self.url
    }
}

/// Endpoint classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum EndpointKind {
    /// Global endpoint (`{account}.documents.azure.com`).
    Global,
    /// Regional endpoint (`{account}-{region}.documents.azure.com`).
    Regional,
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
