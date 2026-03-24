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
//! - [`FaultInjectionClientBuilder`] — Entry point for configuring fault injection. Pass the
//!   configured builder to [`CosmosClientBuilder::with_fault_injection()`](crate::CosmosClientBuilder::with_fault_injection)
//!   to enable fault injection and wrap the HTTP transport with a fault-injecting client.
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
//!     .build(CosmosAccountReference::with_master_key(
//!         "https://myaccount.documents.azure.com/".parse().unwrap(),
//!         Secret::new("my_account_key"),
//!     ))
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
//!

mod client_builder;
mod condition;
mod http_client;
mod result;
mod rule;

use std::fmt;
use std::str::FromStr;

use crate::operation_context::OperationType;
use crate::resource_context::ResourceType;

pub use client_builder::FaultInjectionClientBuilder;
pub use condition::{FaultInjectionCondition, FaultInjectionConditionBuilder};
pub use result::{CustomResponse, FaultInjectionResult, FaultInjectionResultBuilder};
pub use rule::{FaultInjectionRule, FaultInjectionRuleBuilder};

/// Represents different server error types that can be injected for fault testing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultInjectionErrorType {
    /// 500 from server.
    InternalServerError,
    /// 429 from server.
    TooManyRequests,
    /// 404-1002 from server.
    ReadSessionNotAvailable,
    /// 408 from server.
    Timeout,
    /// Simulate service unavailable (503).
    ServiceUnavailable,
    /// 410-1002 from server.
    PartitionIsGone,
    /// 403-3 Forbidden from server.
    WriteForbidden,
    /// 403-1008 Forbidden from server.
    DatabaseAccountNotFound,
    /// Simulates a connection failure (e.g., connection refused, DNS failure).
    /// Produces an `ErrorKind::Io` error, not an HTTP response error.
    ConnectionError,
    /// Simulates a response timeout (request sent but no response received).
    /// Produces an `ErrorKind::Io` error, not an HTTP response error.
    ResponseTimeout,
}

/// The type of operation to which the fault injection applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultOperationType {
    /// Read items.
    ReadItem,
    /// Query items.
    QueryItem,
    /// Create item.
    CreateItem,
    /// Upsert item.
    UpsertItem,
    /// Replace item.
    ReplaceItem,
    /// Delete item.
    DeleteItem,
    /// Patch item.
    PatchItem,
    /// Batch item.
    BatchItem,
    /// Read change feed items.
    ChangeFeedItem,
    /// Read container request.
    MetadataReadContainer,
    /// Read database account request.
    MetadataReadDatabaseAccount,
    /// Query query plan request.
    MetadataQueryPlan,
    /// Partition key ranges request.
    MetadataPartitionKeyRanges,
}

impl FaultOperationType {
    /// Returns the string representation of this operation type.
    pub fn as_str(&self) -> &'static str {
        match self {
            FaultOperationType::ReadItem => "ReadItem",
            FaultOperationType::QueryItem => "QueryItem",
            FaultOperationType::CreateItem => "CreateItem",
            FaultOperationType::UpsertItem => "UpsertItem",
            FaultOperationType::ReplaceItem => "ReplaceItem",
            FaultOperationType::DeleteItem => "DeleteItem",
            FaultOperationType::PatchItem => "PatchItem",
            FaultOperationType::BatchItem => "BatchItem",
            FaultOperationType::ChangeFeedItem => "ChangeFeedItem",
            FaultOperationType::MetadataReadContainer => "MetadataReadContainer",
            FaultOperationType::MetadataReadDatabaseAccount => "MetadataReadDatabaseAccount",
            FaultOperationType::MetadataQueryPlan => "MetadataQueryPlan",
            FaultOperationType::MetadataPartitionKeyRanges => "MetadataPartitionKeyRanges",
        }
    }

    /// Converts an operation type and resource type pair into a fault injection operation type.
    ///
    /// Returns `None` if the combination does not map to a known fault operation type.
    pub fn from_operation_and_resource(
        operation_type: &OperationType,
        resource_type: &ResourceType,
    ) -> Option<Self> {
        match (operation_type, resource_type) {
            (OperationType::Read, ResourceType::Documents) => Some(FaultOperationType::ReadItem),
            (OperationType::Query, ResourceType::Documents) => Some(FaultOperationType::QueryItem),
            (OperationType::Create, ResourceType::Documents) => {
                Some(FaultOperationType::CreateItem)
            }
            (OperationType::Upsert, ResourceType::Documents) => {
                Some(FaultOperationType::UpsertItem)
            }
            (OperationType::Replace, ResourceType::Documents) => {
                Some(FaultOperationType::ReplaceItem)
            }
            (OperationType::Delete, ResourceType::Documents) => {
                Some(FaultOperationType::DeleteItem)
            }
            (OperationType::Patch, ResourceType::Documents) => Some(FaultOperationType::PatchItem),
            (OperationType::Batch, ResourceType::Documents) => Some(FaultOperationType::BatchItem),
            (OperationType::ReadFeed, ResourceType::Documents) => {
                Some(FaultOperationType::ChangeFeedItem)
            }
            (OperationType::Read, ResourceType::Containers) => {
                Some(FaultOperationType::MetadataReadContainer)
            }
            (OperationType::Read, ResourceType::DatabaseAccount) => {
                Some(FaultOperationType::MetadataReadDatabaseAccount)
            }
            (OperationType::QueryPlan, ResourceType::Documents) => {
                Some(FaultOperationType::MetadataQueryPlan)
            }
            (OperationType::ReadFeed, ResourceType::PartitionKeyRanges) => {
                Some(FaultOperationType::MetadataPartitionKeyRanges)
            }
            _ => None,
        }
    }
}

impl fmt::Display for FaultOperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for FaultOperationType {
    type Err = ();

    /// Parses a string into a `FaultOperationType`.
    ///
    /// Returns `Err(())` if the string is not a recognized operation type.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ReadItem" => Ok(FaultOperationType::ReadItem),
            "QueryItem" => Ok(FaultOperationType::QueryItem),
            "CreateItem" => Ok(FaultOperationType::CreateItem),
            "UpsertItem" => Ok(FaultOperationType::UpsertItem),
            "ReplaceItem" => Ok(FaultOperationType::ReplaceItem),
            "DeleteItem" => Ok(FaultOperationType::DeleteItem),
            "PatchItem" => Ok(FaultOperationType::PatchItem),
            "BatchItem" => Ok(FaultOperationType::BatchItem),
            "ChangeFeedItem" => Ok(FaultOperationType::ChangeFeedItem),
            "MetadataReadContainer" => Ok(FaultOperationType::MetadataReadContainer),
            "MetadataReadDatabaseAccount" => Ok(FaultOperationType::MetadataReadDatabaseAccount),
            "MetadataQueryPlan" => Ok(FaultOperationType::MetadataQueryPlan),
            "MetadataPartitionKeyRanges" => Ok(FaultOperationType::MetadataPartitionKeyRanges),
            _ => Err(()),
        }
    }
}
