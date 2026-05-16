// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! **Unsupported internal API — no stability guarantees.**
//!
//! This module is only available when the `__internal_testing` feature flag is
//! enabled. It is intended exclusively for use in benchmarks and test harnesses
//! that need to replace the reqwest transport layer with an in-memory mock.
//!
//! Breaking changes may be made to this module at any time without a semver
//! bump. Do **not** depend on this module in production code.
//!
//! # Usage
//!
//! Implement [`HttpClientFactory`] to provide custom [`TransportClient`]
//! instances, then pass your factory to
//! [`CosmosDriverRuntimeBuilder::with_mock_http_client_factory`](crate::CosmosDriverRuntimeBuilder::with_mock_http_client_factory).

pub use crate::driver::transport::{
    cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError},
    http_client_factory::{HttpClientConfig, HttpClientFactory},
};
pub use crate::options::ConnectionPoolOptions;

// Internal data structures exposed for memory / size benchmarks.
//
// These types are crate-private in production builds; the `__internal_testing`
// feature flag re-exports them here so that benchmarks (and any future
// memory-profiling tooling) can construct them, query their `size_of`,
// and exercise their hot paths without going through the full driver
// pipeline. The underlying modules remain `pub(crate)`, so these
// re-exports are the only way external callers can name the types.
pub use crate::driver::cache::{
    ContainerRoutingMap, PartitionKeyRangeCache, PkRangeFetchResult, RoutingMapError,
};

// PPCB / PPAF routing-state primitives exposed for memory benchmarks.
//
// `PartitionEndpointState` is the per-driver structure that holds the PPCB
// `circuit_breaker_overrides` map and the PPAF `failover_overrides` map.
// `PartitionFailoverEntry` is one entry inside those maps. Together they
// capture the steady-state heap footprint of partition-level failover.
// `CosmosEndpoint` is the regional-endpoint value type that lives inside
// `failed_endpoints` / `current_endpoint`.
pub use crate::driver::routing::endpoint::CosmosEndpoint;
pub use crate::driver::routing::partition_endpoint_state::{
    FailedEndpoints, HealthStatus, PartitionEndpointState, PartitionFailoverConfig,
    PartitionFailoverEntry,
};
pub use crate::driver::routing::partition_key_range_id::PartitionKeyRangeId;

// Routing state-store primitives exposed for end-to-end memory benchmarks
// that need to layer synthetic PPCB state on top of a real `CosmosDriver`
// instance. The store is reachable from a real driver via
// [`crate::driver::CosmosDriver::location_state_store`], also feature-gated.
pub use crate::driver::routing::account_endpoint_state::AccountEndpointState;
pub use crate::driver::routing::location_state_store::LocationStateStore;
