// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! **Unsupported internal API — no stability guarantees.**
//!
//! This module is only available when the `__internal_mocking` feature flag is
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
