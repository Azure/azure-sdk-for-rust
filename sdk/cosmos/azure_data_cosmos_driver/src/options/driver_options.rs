// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::ClientOptions;
use std::time::Duration;

use crate::options::ReadConsistencyStrategy;

use super::ConnectionPoolOptions;

/// Configuration options for a Cosmos DB driver instance.
///
/// These options control driver-wide behavior including connection pooling,
/// default consistency levels, and HTTP pipeline configuration.
#[derive(Clone, Debug)]
pub struct DriverOptions {
    /// Core HTTP client options from azure_core.
    pub client_options: ClientOptions,

    /// Connection pool configuration for managing TCP connections.
    pub connection_pool: ConnectionPoolOptions,

    /// Default request timeout for operations (can be overridden per-operation).
    pub default_timeout: Duration,

    /// Default read consistency strategy for read operations (can be overridden per-operation).
    pub default_read_consistency_strategy: Option<ReadConsistencyStrategy>,
}

impl Default for DriverOptions {
    fn default() -> Self {
        Self {
            client_options: ClientOptions::default(),
            connection_pool: ConnectionPoolOptions::default(),
            default_timeout: Duration::from_secs(60),
            default_read_consistency_strategy: None,
        }
    }
}
