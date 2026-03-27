// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration options for the Cosmos DB driver.
//!
//! This module contains types for configuring driver instances and individual operations.
//! Options follow a four-level hierarchy with layered resolution:
//!
//! **Environment → Runtime → Account (Driver) → Operation** (lowest to highest priority)
//!
//! Option groups use `#[derive(CosmosOptions)]` to generate View, Builder, Default,
//! and `from_env()` constructors:
//!
//! - [`OperationOptions`] — cross-layer options for service requests (consistency,
//!   excluded regions, content response on write)
//! - [`RetryOptions`] / [`SessionRetryOptions`] — retry behavior configuration
//! - [`RuntimeOptions`] — internal infrastructure options (failover, timeouts, session capture)
//!
//! [`ConnectionPoolOptions`] and [`DiagnosticsOptions`] are captured once at
//! initialization time and do not participate in per-operation layered resolution.

mod connection_pool;
mod diagnostics_options;
mod driver_options;
mod env_parsing;
mod identity;
mod operation_options;
mod policies;
mod priority;
mod read_consistency;
mod region;
mod request_options;
mod retry_options;
mod runtime_options;
mod session_retry_options;
mod throughput_control;

pub use connection_pool::{ConnectionPoolOptions, ConnectionPoolOptionsBuilder};
pub use diagnostics_options::{
    DiagnosticsOptions, DiagnosticsOptionsBuilder, DiagnosticsVerbosity,
};
pub use driver_options::{DriverOptions, DriverOptionsBuilder};
pub(crate) use env_parsing::parse_duration_millis_from_env;
pub use identity::{CorrelationId, UserAgentSuffix, WorkloadId};
pub use operation_options::{OperationOptions, OperationOptionsBuilder, OperationOptionsView};
pub use policies::{
    ContentResponseOnWrite, EmulatorServerCertValidation, EndToEndOperationLatencyPolicy,
    ExcludedRegions,
};
pub use priority::PriorityLevel;
pub use read_consistency::ReadConsistencyStrategy;
pub use region::Region;
pub use request_options::RequestOptions;
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
