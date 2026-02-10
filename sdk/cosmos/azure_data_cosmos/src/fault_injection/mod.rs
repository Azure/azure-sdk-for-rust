// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fault injection framework for testing Cosmos DB client behavior under error conditions.
//!
//! This module provides a fault injection framework that intercepts HTTP requests at the
//! transport layer, below the retry policy. When a fault is injected, it triggers the same
//! retry and failover behavior as a real service error. This enables testing of:
//!
//! - Error handling for various HTTP status codes (503, 500, 429, 408, etc.)
//! - Retry logic and backoff behavior
//! - Regional failover scenarios
//! - Operation-specific error handling
//!
//! # Enabling Fault Injection
//!
//! Fault injection requires the `fault_injection` feature flag:
//!
//! ```toml
//! [dependencies]
//! azure_data_cosmos = { version = "0.31", features = ["fault_injection"] }
//! ```
//!
//! # Core Components
//!
//! - [`FaultInjectionClientBuilder`] — Entry point for configuring fault injection. Wraps the
//!   default HTTP transport with a fault-injecting HTTP client and sets
//!   `fault_injection_enabled` on [`CosmosClientOptions`](crate::CosmosClientOptions).
//! - [`FaultInjectionCondition`] — Defines when a fault should be applied, filtering by
//!   operation type, region, or container ID.
//! - [`FaultInjectionResult`] — Defines what error to inject, including error type, delay,
//!   and probability.
//! - [`FaultInjectionRule`] — Combines a condition with a result and additional controls
//!   like duration, start delay, and hit limit.
//!
//! # Usage
//!
//! ```rust,no_run
//! use azure_data_cosmos::fault_injection::{
//!     FaultInjectionClientBuilder, FaultInjectionConditionBuilder,
//!     FaultInjectionErrorType, FaultInjectionResultBuilder,
//!     FaultInjectionRuleBuilder, FaultOperationType,
//! };
//! use azure_data_cosmos::CosmosClientOptions;
//! use std::sync::Arc;
//! use std::time::{Duration, Instant};
//!
//! // 1. Define what error to inject
//! let result = FaultInjectionResultBuilder::new()
//!     .with_error(FaultInjectionErrorType::ServiceUnavailable)
//!     .with_delay(Duration::from_millis(100))
//!     .with_probability(1.0)
//!     .build();
//!
//! // 2. Define when to inject it
//! let condition = FaultInjectionConditionBuilder::new()
//!     .with_operation_type(FaultOperationType::ReadItem)
//!     .with_region("West US".into())
//!     .build();
//!
//! // 3. Create a rule with timing constraints
//! let rule = Arc::new(FaultInjectionRuleBuilder::new("region-failover-test", result)
//!     .with_condition(condition)
//!     .with_hit_limit(5)
//!     .with_end_time(Instant::now() + Duration::from_secs(30))
//!     .build());
//!
//! // 4. Inject into client options
//! let options = FaultInjectionClientBuilder::new()
//!     .with_rule(rule)
//!     .inject(CosmosClientOptions::default());
//! ```
//!
//! # Rule Evaluation
//!
//! Rules are evaluated in the order they were added. The first matching rule is applied.
//! All specified conditions in a [`FaultInjectionCondition`] must match (AND logic):
//! if no conditions are specified, the rule matches all requests.
//!

mod fault_http_client;
mod fault_injection_client_builder;
mod fault_injection_condition;
mod fault_injection_result;
mod fault_injection_rule;

pub use fault_injection_client_builder::FaultInjectionClientBuilder;
pub use fault_injection_condition::{
    FaultInjectionCondition, FaultInjectionConditionBuilder, FaultOperationType,
};
pub use fault_injection_result::{
    FaultInjectionErrorType, FaultInjectionResult, FaultInjectionResultBuilder,
};
pub use fault_injection_rule::{FaultInjectionRule, FaultInjectionRuleBuilder};
