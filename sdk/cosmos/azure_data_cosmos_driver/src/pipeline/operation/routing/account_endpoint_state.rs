// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Account-level endpoint routing state.

use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use url::Url;

use super::{CosmosEndpoint, UnavailableReason};

/// Immutable account-level endpoint routing state.
#[derive(Clone, Debug)]
pub(crate) struct AccountEndpointState {
    /// Monotonically increasing generation for stale index detection.
    pub generation: u64,
    /// Ordered preferred read endpoints.
    pub preferred_read_endpoints: Arc<[CosmosEndpoint]>,
    /// Ordered preferred write endpoints.
    pub preferred_write_endpoints: Arc<[CosmosEndpoint]>,
    /// Write endpoints in the order returned by account metadata.
    ///
    /// Distributed Transactions route through the DTC in the account's
    /// current write region and must ignore caller regional preferences.
    pub account_write_endpoints: Arc<[CosmosEndpoint]>,
    /// Endpoints marked temporarily unavailable, keyed by their primary URL.
    pub unavailable_endpoints: HashMap<Url, (Instant, UnavailableReason)>,
    /// Whether account supports multiple write locations.
    pub multiple_write_locations_enabled: bool,
    /// Fallback endpoint when no preferred endpoint is available.
    pub default_endpoint: CosmosEndpoint,
}

impl AccountEndpointState {
    /// Creates a minimal single-endpoint state.
    pub fn single(default_endpoint: CosmosEndpoint) -> Self {
        Self {
            generation: 0,
            preferred_read_endpoints: vec![default_endpoint.clone()].into(),
            preferred_write_endpoints: vec![default_endpoint.clone()].into(),
            account_write_endpoints: vec![default_endpoint.clone()].into(),
            unavailable_endpoints: HashMap::new(),
            multiple_write_locations_enabled: false,
            default_endpoint,
        }
    }

    /// Returns preferred endpoints for the given operation kind.
    pub fn preferred_endpoints(&self, read_only: bool) -> &[CosmosEndpoint] {
        if read_only {
            &self.preferred_read_endpoints
        } else {
            &self.preferred_write_endpoints
        }
    }

    /// Checks account-wide failure evidence, allowing ordinary reads through a write-forbidden mark.
    pub(crate) fn is_region_available(
        &self,
        endpoint: &CosmosEndpoint,
        now: Instant,
        unavailability_ttl: Duration,
        for_read: bool,
    ) -> bool {
        !self
            .unavailable_endpoints
            .get(endpoint.url())
            .is_some_and(|(marked_at, reason)| {
                !(for_read && matches!(reason, UnavailableReason::WriteForbidden))
                    && now.saturating_duration_since(*marked_at) < unavailability_ttl
            })
    }

    /// Checks regional membership while allowing endpoints without a regional identity.
    pub(crate) fn contains_region(&self, endpoint: &CosmosEndpoint) -> bool {
        endpoint.region().is_none_or(|region| {
            self.preferred_read_endpoints
                .iter()
                .chain(self.preferred_write_endpoints.iter())
                .any(|known| known.region() == Some(region))
        })
    }
}
