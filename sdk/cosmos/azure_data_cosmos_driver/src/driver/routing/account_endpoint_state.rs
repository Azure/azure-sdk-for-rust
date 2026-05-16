// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Account-level endpoint routing state.

use std::{collections::HashMap, sync::Arc, time::Instant};

use url::Url;

use super::{CosmosEndpoint, UnavailableReason};

/// Immutable account-level endpoint routing state.
//
// `pub` (rather than `pub(crate)`) so that `crate::testing` can surface
// this type for memory benchmarks under the `__internal_testing` feature
// flag. The enclosing `routing` module is `pub(crate)` and
// `account_endpoint_state` is a private `mod`, so external consumers still
// cannot reach this via `crate::driver::routing::*`; it remains accessible
// only through the `crate::testing::*` re-exports.
#[derive(Clone, Debug)]
pub struct AccountEndpointState {
    /// Monotonically increasing generation for stale index detection.
    pub generation: u64,
    /// Ordered preferred read endpoints.
    pub preferred_read_endpoints: Arc<[CosmosEndpoint]>,
    /// Ordered preferred write endpoints.
    pub preferred_write_endpoints: Arc<[CosmosEndpoint]>,
    /// Endpoints marked temporarily unavailable, keyed by their primary URL.
    //
    // Field-level `pub(crate)` because the value type `UnavailableReason`
    // is itself crate-private; benchmarks consuming `AccountEndpointState`
    // through `crate::testing` only need read access to the endpoint lists,
    // not this internal bookkeeping map.
    pub(crate) unavailable_endpoints: HashMap<Url, (Instant, UnavailableReason)>,
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
}
