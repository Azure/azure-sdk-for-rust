// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fault injection for testing Cosmos DB client error handling.
//!
//! Inject simulated transport faults to test retry and failover behavior
//! without changing the service. Build
//! [`FaultInjectionRule`]s with [`FaultInjectionRuleBuilder`] and pass the
//! `Vec<Arc<FaultInjectionRule>>` to
//! [`CosmosClientBuilder::with_fault_injection_rules()`](crate::CosmosClientBuilder::with_fault_injection_rules).
//! The types in this module are re-exported from
//! [`azure_data_cosmos_driver::fault_injection`].
//!
//! # Enable fault injection
//!
//! Enable the `fault_injection` Cargo feature. The example below also requires
//! `key_auth` to authenticate with an account key:
//!
//! ```toml
//! [dependencies]
//! azure_data_cosmos = { version = "0.39", features = ["fault_injection", "key_auth"] }
//! ```
//!
//! # Core components
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
//! # Examples
//!
//! ```rust,no_run
//! # #[cfg(feature = "key_auth")]
//! # async fn example() {
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
//! // 4. Create the client with fault injection.
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
//! # let _ = client;
//! # }
//! # fn main() {}
//! ```
//!
//! # Rule evaluation
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

/// Transport kind used to scope fault-injection rules to Gateway 1.x or
/// Gateway 2.0 without depending directly on the driver crate.
pub use azure_data_cosmos_driver::diagnostics::TransportKind;
