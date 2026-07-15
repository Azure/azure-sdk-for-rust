// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration options for the Cosmos DB driver.
//!
//! This module contains types for configuring driver instances and individual operations.
//! Options follow a three-level hierarchy: Runtime → Driver → Operation.

mod connection_pool;
mod dedicated_gateway;
mod diagnostics_options;
mod diagnostics_thresholds;
mod driver_options;
mod env_parsing;
mod identity;
mod operation_options;
mod policies;
mod priority;
mod read_consistency;
mod region;
mod runtime_options;
mod throughput_control;
mod triggers;

pub use connection_pool::{ConnectionPoolOptions, ConnectionPoolOptionsBuilder};
pub use dedicated_gateway::DedicatedGatewayOptions;
pub use diagnostics_options::{
    DiagnosticsOptions, DiagnosticsOptionsBuilder, DiagnosticsVerbosity,
};
pub use diagnostics_thresholds::DiagnosticsThresholds;
pub use driver_options::{DriverOptions, DriverOptionsBuilder};
pub use identity::{CorrelationId, UserAgentSuffix, WorkloadId};
pub use operation_options::OperationOptions;
pub use policies::{
    ContentResponseOnWrite, EmulatorServerCertValidation, EndToEndOperationLatencyPolicy,
    ExcludedRegions, QuotaInfoEnabled, ScriptLoggingEnabled,
};
pub use priority::PriorityLevel;
pub use read_consistency::ReadConsistencyStrategy;
pub use region::Region;
pub use runtime_options::{RuntimeOptions, RuntimeOptionsBuilder, SharedRuntimeOptions};
pub use throughput_control::{
    ThroughputControlGroupKey, ThroughputControlGroupOptions,
    ThroughputControlGroupRegistrationError, ThroughputControlGroupRegistry,
    ThroughputControlGroupSnapshot, ThroughputTarget,
};
pub use triggers::TriggerOptions;
