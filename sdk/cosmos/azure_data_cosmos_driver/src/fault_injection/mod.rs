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
//! # Core Components
//!
//! - [`FaultInjectionCondition`] — Defines when a fault should be applied, filtering by
//!   operation type, region, or container ID.
//! - [`FaultInjectionResult`] — Defines what error to inject, including error type, delay,
//!   and probability.
//! - [`FaultInjectionRule`] — Combines a condition with a result and additional controls
//!   like timing windows (`start_time`/`end_time`), `hit_limit`, and `probability`.
//! - [`FaultClient`] — An [`HttpClient`](azure_core::http::HttpClient)
//!   implementation that evaluates rules and injects faults.
//! - `FaultInjectingHttpClientFactory` — An `HttpClientFactory`
//!   decorator that wraps created clients with fault injection.

mod condition;
mod fault_injecting_factory;
mod http_client;
mod result;
mod rule;

use std::fmt;
use std::str::FromStr;

use crate::models::{OperationType, ResourceType};

pub use condition::{FaultInjectionCondition, FaultInjectionConditionBuilder};
pub(crate) use fault_injecting_factory::FaultInjectingHttpClientFactory;
pub use http_client::FaultClient;
pub use http_client::FaultInjectionEvaluation;
pub use result::{
    CustomResponse, CustomResponseBuilder, FaultInjectionResult, FaultInjectionResultBuilder,
};
pub use rule::{FaultInjectionRule, FaultInjectionRuleBuilder};

/// Represents different server error types that can be injected for fault testing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
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
    /// Produces an `ErrorKind::Connection` error, not an HTTP response error.
    ConnectionError,
    /// Simulates a response timeout (request sent but no response received).
    /// Produces an `ErrorKind::Io` error, not an HTTP response error.
    ResponseTimeout,
}

/// The type of operation to which the fault injection applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
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
            (OperationType::Read, ResourceType::Document) => Some(FaultOperationType::ReadItem),
            (OperationType::Query, ResourceType::Document) => Some(FaultOperationType::QueryItem),
            (OperationType::Create, ResourceType::Document) => Some(FaultOperationType::CreateItem),
            (OperationType::Upsert, ResourceType::Document) => Some(FaultOperationType::UpsertItem),
            (OperationType::Replace, ResourceType::Document) => {
                Some(FaultOperationType::ReplaceItem)
            }
            (OperationType::Delete, ResourceType::Document) => Some(FaultOperationType::DeleteItem),
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
            // PatchItem will be mapped when OperationType::Patch is added to the driver.
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
    type Err = azure_core::Error;

    /// Parses a string into a `FaultOperationType`.
    ///
    /// Returns an error if the string is not a recognized operation type.
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
            _ => Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!("unknown fault operation type: {s}"),
            )),
        }
    }
}

impl fmt::Display for FaultInjectionErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InternalServerError => write!(f, "InternalServerError"),
            Self::TooManyRequests => write!(f, "TooManyRequests"),
            Self::ReadSessionNotAvailable => write!(f, "ReadSessionNotAvailable"),
            Self::Timeout => write!(f, "Timeout"),
            Self::ServiceUnavailable => write!(f, "ServiceUnavailable"),
            Self::PartitionIsGone => write!(f, "PartitionIsGone"),
            Self::WriteForbidden => write!(f, "WriteForbidden"),
            Self::DatabaseAccountNotFound => write!(f, "DatabaseAccountNotFound"),
            Self::ConnectionError => write!(f, "ConnectionError"),
            Self::ResponseTimeout => write!(f, "ResponseTimeout"),
        }
    }
}

impl FromStr for FaultInjectionErrorType {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "InternalServerError" => Ok(Self::InternalServerError),
            "TooManyRequests" => Ok(Self::TooManyRequests),
            "ReadSessionNotAvailable" => Ok(Self::ReadSessionNotAvailable),
            "Timeout" => Ok(Self::Timeout),
            "ServiceUnavailable" => Ok(Self::ServiceUnavailable),
            "PartitionIsGone" => Ok(Self::PartitionIsGone),
            "WriteForbidden" => Ok(Self::WriteForbidden),
            "DatabaseAccountNotFound" => Ok(Self::DatabaseAccountNotFound),
            "ConnectionError" => Ok(Self::ConnectionError),
            "ResponseTimeout" => Ok(Self::ResponseTimeout),
            _ => Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!("unknown fault injection error type: {s}"),
            )),
        }
    }
}
