// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Fault injection framework for testing Cosmos DB driver behavior under error conditions.
//!
//! This module provides a fault injection framework that intercepts HTTP requests at the
//! transport layer, below the retry policy. When a fault is injected, it triggers the same
//! retry and failover behavior as a real service error. This enables testing of:
//!
//! - Error handling for various HTTP status codes (503, 500, 429, 408, etc.)
//! - Retry logic and backoff behavior
//! - Regional failover scenarios
//! - Operation-specific error handling
//! - Shard eviction under consecutive failures
//! - Scale-up/down under fault injection
//!
//! # Core Components
//!
//! - [`FaultInjectionCondition`] — Defines when a fault should be applied, filtering by
//!   operation type, region, or container ID.
//! - [`FaultInjectionResult`] — Defines what error to inject, including error type, delay,
//!   and probability.
//! - [`FaultInjectionRule`] — Combines a condition with a result and additional controls
//!   like duration, start delay, and hit limit.
//!
//! # Rule Evaluation
//!
//! Rules are evaluated in the order they were added. The first matching rule is applied.
//! All specified conditions in a [`FaultInjectionCondition`] must match (AND logic):
//! if no conditions are specified, the rule matches all requests.

mod condition;
pub(crate) mod fault_injecting_client;
pub(crate) mod fault_injecting_factory;
mod result;
mod rule;

use std::fmt;
use std::str::FromStr;

use azure_core::http::headers::HeaderName;

use crate::models::{OperationType, ResourceType};

pub use condition::{FaultInjectionCondition, FaultInjectionConditionBuilder};
pub use result::{CustomResponse, FaultInjectionResult, FaultInjectionResultBuilder};
pub use rule::{FaultInjectionRule, FaultInjectionRuleBuilder};

/// Header name for the fault injection operation type.
///
/// This custom header is attached to outgoing requests so that the
/// [`FaultInjectingHttpClient`](fault_injecting_client::FaultInjectingHttpClient) can match
/// rules against the operation type without inspecting the request URL or body.
pub(crate) static FAULT_INJECTION_OPERATION: HeaderName =
    HeaderName::from_static("x-ms-fault-injection-operation");

/// Header name for the fault injection container ID.
///
/// Attached to outgoing requests to allow container-scoped rule matching.
#[allow(dead_code)]
pub(crate) static FAULT_INJECTION_CONTAINER_ID: HeaderName =
    HeaderName::from_static("x-ms-fault-injection-container-id");

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
        operation_type: OperationType,
        resource_type: ResourceType,
    ) -> Option<Self> {
        match (operation_type, resource_type) {
            (OperationType::Read, ResourceType::Document) => Some(FaultOperationType::ReadItem),
            (OperationType::Query, ResourceType::Document) => Some(FaultOperationType::QueryItem),
            (OperationType::Create, ResourceType::Document) => Some(FaultOperationType::CreateItem),
            (OperationType::Upsert, ResourceType::Document) => Some(FaultOperationType::UpsertItem),
            (OperationType::Replace, ResourceType::Document) => {
                Some(FaultOperationType::ReplaceItem)
            }
            (OperationType::Delete, ResourceType::Document) => Some(FaultOperationType::DeleteItem),
            // Note: Patch is not yet in the driver's OperationType but may be added later.
            (OperationType::Batch, ResourceType::Document) => Some(FaultOperationType::BatchItem),
            (OperationType::ReadFeed, ResourceType::Document) => {
                Some(FaultOperationType::ChangeFeedItem)
            }
            (OperationType::Read, ResourceType::DocumentCollection) => {
                Some(FaultOperationType::MetadataReadContainer)
            }
            (OperationType::Read, ResourceType::DatabaseAccount) => {
                Some(FaultOperationType::MetadataReadDatabaseAccount)
            }
            (OperationType::QueryPlan, ResourceType::Document) => {
                Some(FaultOperationType::MetadataQueryPlan)
            }
            (OperationType::ReadFeed, ResourceType::PartitionKeyRange) => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_operation_and_resource_read_item() {
        assert_eq!(
            FaultOperationType::from_operation_and_resource(
                OperationType::Read,
                ResourceType::Document
            ),
            Some(FaultOperationType::ReadItem)
        );
    }

    #[test]
    fn from_operation_and_resource_query_item() {
        assert_eq!(
            FaultOperationType::from_operation_and_resource(
                OperationType::Query,
                ResourceType::Document
            ),
            Some(FaultOperationType::QueryItem)
        );
    }

    #[test]
    fn from_operation_and_resource_create_item() {
        assert_eq!(
            FaultOperationType::from_operation_and_resource(
                OperationType::Create,
                ResourceType::Document
            ),
            Some(FaultOperationType::CreateItem)
        );
    }

    #[test]
    fn from_operation_and_resource_metadata_read_container() {
        assert_eq!(
            FaultOperationType::from_operation_and_resource(
                OperationType::Read,
                ResourceType::DocumentCollection
            ),
            Some(FaultOperationType::MetadataReadContainer)
        );
    }

    #[test]
    fn from_operation_and_resource_metadata_read_database_account() {
        assert_eq!(
            FaultOperationType::from_operation_and_resource(
                OperationType::Read,
                ResourceType::DatabaseAccount
            ),
            Some(FaultOperationType::MetadataReadDatabaseAccount)
        );
    }

    #[test]
    fn from_operation_and_resource_unknown_combination() {
        assert_eq!(
            FaultOperationType::from_operation_and_resource(
                OperationType::Delete,
                ResourceType::Database
            ),
            None
        );
    }

    #[test]
    fn round_trip_from_str() {
        for op in [
            FaultOperationType::ReadItem,
            FaultOperationType::QueryItem,
            FaultOperationType::CreateItem,
            FaultOperationType::UpsertItem,
            FaultOperationType::ReplaceItem,
            FaultOperationType::DeleteItem,
            FaultOperationType::PatchItem,
            FaultOperationType::BatchItem,
            FaultOperationType::ChangeFeedItem,
            FaultOperationType::MetadataReadContainer,
            FaultOperationType::MetadataReadDatabaseAccount,
            FaultOperationType::MetadataQueryPlan,
            FaultOperationType::MetadataPartitionKeyRanges,
        ] {
            let s = op.as_str();
            let parsed: FaultOperationType = s.parse().unwrap();
            assert_eq!(parsed, op, "round-trip failed for {s}");
        }
    }

    #[test]
    fn from_str_unknown() {
        assert!("UnknownOp".parse::<FaultOperationType>().is_err());
    }
}
