// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
//!
//! ## Unstable OpenTelemetry
//!
//! Enable the off-by-default `unstable_opentelemetry` feature to emit metrics and
//! distributed traces through the OpenTelemetry diagnostics handlers. This
//! integration is unstable because the `opentelemetry` crate is still in
//! preview. Diagnostics contexts and non-OpenTelemetry handlers remain available
//! without the feature.

// =========================================================================
// Public API
// =========================================================================

pub use account_endpoint::AccountEndpoint;
pub use account_reference::AccountReference;
#[doc(inline)]
pub use clients::{ContainerClient, CosmosClient, CosmosClientBuilder, DatabaseClient};
#[cfg(feature = "unstable_dtx")]
pub use clients::{DistributedReadTransaction, DistributedWriteTransaction};
pub use credential::CosmosCredential;
pub use error::{CosmosError, CosmosStatus, Result, SubStatusCode};
pub use feed::{FeedScope, Query};
pub use models::{PartitionKey, TransactionalBatch};
pub use options::RoutingStrategy;
pub use resource_identity::{ResourceId, ResourceIdentity};
pub use runtime::{CosmosRuntime, CosmosRuntimeBuilder};

// =========================================================================
// Public modules
// =========================================================================

pub mod clients;
pub mod diagnostics;
pub mod error;
#[cfg(feature = "fault_injection")]
pub mod fault_injection;
pub mod feed;
pub mod models;
pub mod options;

// =========================================================================
// Internal modules
// =========================================================================

mod account_endpoint;
mod account_reference;
mod constants;
mod credential;
mod driver_bridge;
mod region_proximity;
mod resource_identity;
mod runtime;
mod session_helpers;
