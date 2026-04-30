// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fault injection framework for testing Cosmos DB client behavior under error conditions.
//!
//! This module wraps the driver's fault-injection primitives — every type
//! except [`FaultInjectionClientBuilder`] is re-exported directly from
//! [`azure_data_cosmos_driver::fault_injection`]. The SDK only owns the
//! [`FaultInjectionClientBuilder`] (which produces an [`azure_core::http::Transport`]
//! that the SDK pipeline plugs in) and a small adapter for translating SDK-side
//! `OperationType` / `ResourceType` pairs into the driver's
//! [`FaultOperationType`].
//!
//! Below the transport layer, fault injection intercepts HTTP requests and
//! triggers the same retry and failover behavior as a real service error.
//! It enables testing of:
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
//! - [`FaultInjectionClientBuilder`] — Entry point for configuring fault injection. Pass the
//!   configured builder to [`CosmosClientBuilder::with_fault_injection()`](crate::CosmosClientBuilder::with_fault_injection)
//!   to enable fault injection and wrap the HTTP transport with a fault-injecting client.
//! - [`FaultInjectionCondition`] — Defines when a fault should be applied, filtering by
//!   operation type, region, container ID, or transport kind.
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
//! use azure_data_cosmos::CosmosClientBuilder;
//! use azure_data_cosmos::CosmosAccountReference;
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
//! // 4. Create the fault injection builder
//! let fault_builder = FaultInjectionClientBuilder::new()
//!     .with_rule(rule);
//!
//! // 5. Create the client with fault injection
//! let client = CosmosClientBuilder::new()
//!     .with_fault_injection(fault_builder)
//!     .build(
//!         CosmosAccountReference::with_master_key(
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

mod client_builder;
mod http_client;

pub use client_builder::FaultInjectionClientBuilder;

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

use crate::operation_context::OperationType as SdkOperationType;
use crate::resource_context::ResourceType as SdkResourceType;

/// Maps an SDK-side `(OperationType, ResourceType)` pair to the driver's
/// [`FaultOperationType`].
///
/// This mirrors `FaultOperationType::from_operation_and_resource` on the
/// driver, but takes SDK enums directly so SDK callers don't need to convert
/// to driver enums first. Returns `None` if the combination doesn't map to a
/// known fault operation type.
pub(crate) fn fault_operation_for_sdk(
    operation_type: &SdkOperationType,
    resource_type: &SdkResourceType,
) -> Option<FaultOperationType> {
    match (operation_type, resource_type) {
        (SdkOperationType::Read, SdkResourceType::Documents) => Some(FaultOperationType::ReadItem),
        (SdkOperationType::Query, SdkResourceType::Documents) => {
            Some(FaultOperationType::QueryItem)
        }
        (SdkOperationType::Create, SdkResourceType::Documents) => {
            Some(FaultOperationType::CreateItem)
        }
        (SdkOperationType::Upsert, SdkResourceType::Documents) => {
            Some(FaultOperationType::UpsertItem)
        }
        (SdkOperationType::Replace, SdkResourceType::Documents) => {
            Some(FaultOperationType::ReplaceItem)
        }
        (SdkOperationType::Delete, SdkResourceType::Documents) => {
            Some(FaultOperationType::DeleteItem)
        }
        (SdkOperationType::Patch, SdkResourceType::Documents) => {
            Some(FaultOperationType::PatchItem)
        }
        (SdkOperationType::Batch, SdkResourceType::Documents) => {
            Some(FaultOperationType::BatchItem)
        }
        (SdkOperationType::ReadFeed, SdkResourceType::Documents) => {
            Some(FaultOperationType::ChangeFeedItem)
        }
        (SdkOperationType::Read, SdkResourceType::Containers) => {
            Some(FaultOperationType::MetadataReadContainer)
        }
        (SdkOperationType::Read, SdkResourceType::DatabaseAccount) => {
            Some(FaultOperationType::MetadataReadDatabaseAccount)
        }
        (SdkOperationType::QueryPlan, SdkResourceType::Documents) => {
            Some(FaultOperationType::MetadataQueryPlan)
        }
        (SdkOperationType::ReadFeed, SdkResourceType::PartitionKeyRanges) => {
            Some(FaultOperationType::MetadataPartitionKeyRanges)
        }
        _ => None,
    }
}
