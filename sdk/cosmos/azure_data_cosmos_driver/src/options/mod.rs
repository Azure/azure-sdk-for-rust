// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration options for the Cosmos DB driver.
//!
//! This module contains types for configuring driver instances and individual operations.
//! Options follow a four-level hierarchy with layered resolution:
//!
//! **Environment → Runtime → Account (Driver) → Operation** (lowest to highest priority)
//!
//! [`RuntimeOptions`] is the central option group that participates in all layers.
//! It uses `#[derive(CosmosOptions)]` to generate:
//! - [`RuntimeOptionsView`] — snapshot view for resolving options across layers
//! - [`RuntimeOptionsBuilder`] — fluent builder for constructing options
//! - `from_env()` — environment variable loading
//!
//! [`ConnectionPoolOptions`] and [`DiagnosticsOptions`] are captured once at
//! initialization time and do not participate in per-operation layered resolution.

mod connection_pool;
mod cross_layer_operation_options;
mod diagnostics_options;
mod driver_options;
mod env_parsing;
mod identity;
mod operation_options;
mod policies;
mod priority;
mod read_consistency;
mod region;
mod retry_options;
mod runtime_options;
mod session_retry_options;
mod throughput_control;

pub use connection_pool::{ConnectionPoolOptions, ConnectionPoolOptionsBuilder};
pub use cross_layer_operation_options::{
    CrossLayerOperationOptions, CrossLayerOperationOptionsBuilder, CrossLayerOperationOptionsView,
};
pub use diagnostics_options::{
    DiagnosticsOptions, DiagnosticsOptionsBuilder, DiagnosticsVerbosity,
};
pub use driver_options::{DriverOptions, DriverOptionsBuilder};
pub(crate) use env_parsing::parse_duration_millis_from_env;
pub use identity::{CorrelationId, UserAgentSuffix, WorkloadId};
pub use operation_options::OperationOptions;
pub use policies::{
    ContentResponseOnWrite, EmulatorServerCertValidation, EndToEndOperationLatencyPolicy,
    ExcludedRegions,
};
pub use priority::PriorityLevel;
pub use read_consistency::ReadConsistencyStrategy;
pub use region::Region;
pub use retry_options::{RetryOptions, RetryOptionsBuilder, RetryOptionsView};
pub use runtime_options::{RuntimeOptions, RuntimeOptionsBuilder, RuntimeOptionsView};
pub use session_retry_options::{
    SessionRetryOptions, SessionRetryOptionsBuilder, SessionRetryOptionsView,
};
pub use throughput_control::{
    ThroughputControlGroupKey, ThroughputControlGroupOptions,
    ThroughputControlGroupRegistrationError, ThroughputControlGroupRegistry,
    ThroughputControlGroupSnapshot, ThroughputTarget,
};
