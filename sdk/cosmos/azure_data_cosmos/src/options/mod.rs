// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-request options types for Cosmos DB SDK operations.

// =========================================================================
// Public API
// =========================================================================

#[doc(inline)]
pub use azure_data_cosmos_driver::models::{
    MaxItemCountHint, Precondition, SessionToken, ThroughputControlGroupName,
};
#[cfg(feature = "preview_patch")]
#[doc(inline)]
pub use azure_data_cosmos_driver::options::PatchStrategy;
#[doc(inline)]
pub use azure_data_cosmos_driver::options::{
    AvailabilityStrategy, BinaryEncodingOptions, ConnectionPoolOptions,
    ConnectionPoolOptionsBuilder, ContentResponseOnWrite, DiagnosticsOptions,
    DiagnosticsOptionsBuilder, DiagnosticsVerbosity, EndToEndOperationLatencyPolicy,
    ExcludedRegions, HedgeThreshold, HedgingStrategy, OperationOptions, OperationOptionsBuilder,
    OperationOptionsView, PartitionFailoverOptions, PartitionFailoverOptionsBuilder, PriorityLevel,
    QueryPlanMode, ReadConsistencyStrategy, Region, ServerCertificateValidation,
    ThrottlingRetryOptions, ThrottlingRetryOptionsBuilder, ThrottlingRetryOptionsView,
    ThroughputControlGroupOptions, ThroughputControlOptions, ThroughputControlOptionsBuilder,
    ThroughputControlOptionsView, TlsBackend, UserAgentSuffix,
};
pub use batch::{
    BatchDeleteOptions, BatchOptions, BatchReadOptions, BatchReplaceOptions, BatchUpsertOptions,
};
pub use change_feed::{ChangeFeedMode, ChangeFeedOptions, ChangeFeedStartFrom};
pub use client::CosmosClientOptions;
pub use consistency::ConsistencyLevel;
pub use container::{ContainerClientOptions, ReadContainerOptions};
#[cfg(feature = "control_plane")]
pub use container::{
    CreateContainerOptions, DeleteContainerOptions, QueryContainersOptions, ReplaceContainerOptions,
};
#[cfg(feature = "control_plane")]
pub use database::{
    CreateDatabaseOptions, DeleteDatabaseOptions, QueryDatabasesOptions, ReadDatabaseOptions,
};
pub use feed::{FeedOptions, QueryOptions};
pub use feed_ranges::ReadFeedRangesOptions;
#[cfg(feature = "preview_patch")]
pub use item::PatchItemOptions;
pub use item::{ItemReadOptions, ItemWriteOptions};
pub use routing_strategy::RoutingStrategy;
#[cfg(feature = "control_plane")]
pub use throughput::ThroughputOptions;

// =========================================================================
// Internal modules
// =========================================================================

mod batch;
mod change_feed;
mod client;
mod consistency;
mod container;
#[cfg(feature = "control_plane")]
mod database;
mod feed;
mod feed_ranges;
mod item;
mod routing_strategy;
#[cfg(feature = "control_plane")]
mod throughput;
