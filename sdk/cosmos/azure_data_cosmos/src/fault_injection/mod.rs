// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fault injection framework for testing Cosmos DB client behavior under error conditions.
//!
//! Provides types that allow injecting simulated faults into the Cosmos DB client transport layer.
//! This enables testing of application resilience, retry logic, and failover handling without
//! needing to induce real faults in the service.
//! This module is a pure re-export facade over the driver's fault-injection
//! primitives — every type is re-exported directly from
//! [`azure_data_cosmos_driver::fault_injection`]. Build
//! [`FaultInjectionRule`]s with [`FaultInjectionRuleBuilder`] and pass the
//! `Vec<Arc<FaultInjectionRule>>` to
//! [`CosmosClientBuilder::with_fault_injection_rules`](crate::CosmosClientBuilder::with_fault_injection_rules),
//! which threads them through to the per-driver options; the driver's own
//! fault-injection transport client evaluates the rules on every in-flight
//! request.
//!
//! The fault injection framework enables testing of:
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
//! azure_data_cosmos = { version = "...", features = ["fault_injection"] }
//! ```
//!
//! # Core Components
//!
//! - [`FaultInjectionRule`] — Combines a condition with a result and
//!   additional controls like duration, start delay, and hit limit. Build
//!   with [`FaultInjectionRuleBuilder`]; pass a `Vec<Arc<FaultInjectionRule>>`
//!   to [`CosmosClientBuilder::with_fault_injection_rules`](crate::CosmosClientBuilder::with_fault_injection_rules).
//! - [`FaultInjectionCondition`] — Defines when a fault should be applied,
//!   filtering by operation type, region, container ID, or transport kind.
//! - [`FaultInjectionResult`] — Defines what error to inject, including
//!   error type, delay, and probability.
//!
//! # Usage
//!
//! ```rust,no_run
//! use azure_data_cosmos::fault_injection::{
//!     FaultInjectionConditionBuilder, FaultInjectionErrorType,
//!     FaultInjectionResultBuilder, FaultInjectionRuleBuilder, FaultOperationType,
//! };
//! use azure_data_cosmos::CosmosClientBuilder;
//! use azure_data_cosmos::AccountReference;
//! use azure_core::credentials::Secret;
//! use std::sync::Arc;
//! use std::time::{Duration, Instant};
//!
//! # async fn doc() {
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
//! // 4. Create the client with fault injection — pass the rules directly,
//! //    no SDK-side wrapper builder.
//! let client = CosmosClientBuilder::new()
//!     .with_fault_injection_rules(vec![rule])
//!     .unwrap()
//!     .build(
//!         AccountReference::with_authentication_key(
//!             "https://myaccount.documents.azure.com/".parse().unwrap(),
//!             Secret::new("my_account_key"),
//!         ),
//!         azure_data_cosmos::RoutingStrategy::ProximityTo("East US".into()),
//!     )
//!     .await
//!     .unwrap();
//! # }
//! ```
//!
//! # Rule Evaluation
//!
//! Rules are evaluated in the order they were added. The first matching rule is applied.
//! All specified conditions in a [`FaultInjectionCondition`] must match (AND logic):
//! if no conditions are specified, the rule matches all requests.

#[doc(inline)]
pub use azure_data_cosmos_driver::fault_injection::{
    CustomResponse, CustomResponseBuilder, FaultInjectionCondition, FaultInjectionConditionBuilder,
    FaultInjectionErrorType, FaultInjectionResult, FaultInjectionResultBuilder, FaultInjectionRule,
    FaultInjectionRuleBuilder, FaultOperationType,
};

/// Re-export of the driver's [`TransportKind`] enum so SDK consumers can
/// scope fault-injection rules to a specific transport (Gateway 1.x vs
/// Gateway 2.0) without depending on the driver crate directly.
pub use azure_data_cosmos_driver::diagnostics::TransportKind;
