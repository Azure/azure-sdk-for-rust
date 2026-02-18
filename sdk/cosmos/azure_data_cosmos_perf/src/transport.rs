// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Custom HTTP transport for perf testing.
//!
//! Builds a `reqwest::Client` with connection pool settings matching the Cosmos DB
//! driver's defaults from PR #3592 (`azure_data_cosmos_driver`). This replaces the
//! default azure_core transport which uses a bare `reqwest::ClientBuilder::new().build()`.

use std::{sync::Arc, time::Duration};

use azure_core::http::Transport;

/// Default connection pool settings from the Cosmos DB driver.
const MAX_IDLE_CONNECTIONS_PER_HOST: usize = 1_000;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(6);

/// Creates a custom HTTP transport with the Cosmos DB driver's connection pool defaults.
pub fn create_transport() -> Result<Transport, Box<dyn std::error::Error>> {
    let client = reqwest::ClientBuilder::new()
        .pool_max_idle_per_host(MAX_IDLE_CONNECTIONS_PER_HOST)
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .build()?;

    Ok(Transport::new(Arc::new(client)))
}
