// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

// =========================================================================
// Public API
// =========================================================================

pub use account_endpoint::AccountEndpoint;
pub use account_reference::AccountReference;
#[doc(inline)]
pub use clients::{ContainerClient, CosmosClient, CosmosClientBuilder, DatabaseClient};
#[cfg(feature = "preview_dtx")]
pub use clients::{DistributedReadTransaction, DistributedWriteTransaction};
pub use credential::CosmosCredential;
pub use error::{CosmosError, CosmosStatus, Result, SubStatusCode};
pub use feed::{FeedScope, Query};
pub use models::{PartitionKey, TransactionalBatch};
pub use options::RoutingStrategy;
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
mod runtime;
mod session_helpers;

// =========================================================================
// Crate-internal re-exports
// =========================================================================

/// Internal alias for the driver's `CosmosError`. Used at error-construction
/// sites inside this crate so they can call the driver's
/// `CosmosError::builder()` directly and then `.into()` the result into the
/// public [`CosmosError`] newtype. Not exposed in the public API.
pub(crate) use azure_data_cosmos_driver::error::CosmosError as DriverCosmosError;
