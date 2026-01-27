// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration options for the Cosmos DB driver.
//!
//! This module contains types for configuring driver instances and individual operations.
//! Options follow a three-level hierarchy: Environment → Driver → Operation.

mod connection_pool;
mod driver_options;
mod env_parsing;
mod read_consistency;

pub use connection_pool::{ConnectionPoolOptions, ConnectionPoolOptionsBuilder};
pub use driver_options::DriverOptions;
pub use read_consistency::ReadConsistencyStrategy;
