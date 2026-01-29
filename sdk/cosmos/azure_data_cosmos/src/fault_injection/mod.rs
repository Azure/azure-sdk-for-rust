// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fault injection utilities for testing Cosmos DB client behavior under error conditions.

#[cfg(feature = "fault_injection")]
mod fault_http_client;

#[cfg(feature = "fault_injection")]
mod fault_injection_client_builder;

#[cfg(feature = "fault_injection")]
mod fault_injection_condition;

#[cfg(feature = "fault_injection")]
mod fault_injection_result;

#[cfg(feature = "fault_injection")]
mod fault_injection_rule;

// Re-export all public items from the submodules
#[cfg(feature = "fault_injection")]
pub use fault_injection_client_builder::FaultInjectionClientBuilder;

#[cfg(feature = "fault_injection")]
pub use fault_injection_condition::{
    FaultInjectionCondition, FaultInjectionConditionBuilder, FaultOperationType,
};

#[cfg(feature = "fault_injection")]
pub use fault_injection_result::{
    FaultInjectionResult, FaultInjectionServerError, FaultInjectionServerErrorBuilder,
    FaultInjectionServerErrorType,
};

#[cfg(feature = "fault_injection")]
pub use fault_injection_rule::{FaultInjectionRule, FaultInjectionRuleBuilder};
