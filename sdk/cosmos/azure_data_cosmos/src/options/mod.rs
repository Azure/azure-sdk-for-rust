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
#[doc(inline)]
pub use azure_data_cosmos_driver::options::{
    AvailabilityStrategy, ConnectionPoolOptions, ConnectionPoolOptionsBuilder,
    ContentResponseOnWrite, EndToEndOperationLatencyPolicy, ExcludedRegions, HedgeThreshold,
    HedgingStrategy, OperationOptions, OperationOptionsBuilder, OperationOptionsView,
    PartitionFailoverOptions, PartitionFailoverOptionsBuilder, PriorityLevel,
    ReadConsistencyStrategy, Region, ServerCertificateValidation, ThrottlingRetryOptions,
    ThrottlingRetryOptionsBuilder, ThrottlingRetryOptionsView, ThroughputControlGroupOptions,
    ThroughputControlOptions, ThroughputControlOptionsBuilder, ThroughputControlOptionsView,
    TlsBackend, UserAgentSuffix,
};
pub use batch::{
    BatchDeleteOptions, BatchOptions, BatchReadOptions, BatchReplaceOptions, BatchUpsertOptions,
};
pub use change_feed::{ChangeFeedMode, ChangeFeedOptions, ChangeFeedStartFrom};
pub use client::CosmosClientOptions;
pub use consistency::ConsistencyLevel;
pub use container::{
    CreateContainerOptions, DeleteContainerOptions, QueryContainersOptions, ReadContainerOptions,
    ReplaceContainerOptions,
};
pub use database::{
    CreateDatabaseOptions, DeleteDatabaseOptions, QueryDatabasesOptions, ReadDatabaseOptions,
};
pub use feed::{FeedOptions, QueryOptions};
pub use feed_ranges::ReadFeedRangesOptions;
pub use item::{ItemReadOptions, ItemWriteOptions, PatchItemOptions};
pub use routing_strategy::RoutingStrategy;
pub use throughput::ThroughputOptions;

// =========================================================================
// Internal modules
// =========================================================================

mod batch;
mod change_feed;
mod client;
mod consistency;
mod container;
mod database;
mod feed;
mod feed_ranges;
mod item;
mod routing_strategy;
mod throughput;
