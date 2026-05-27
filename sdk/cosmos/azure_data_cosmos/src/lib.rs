// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod account_endpoint;
mod account_reference;
pub mod clients;
mod connection_string;
pub mod constants;
mod credential;
mod error;
mod feed;
pub mod options;
pub mod query;
mod session_helpers;

pub mod models;
pub mod transactional_batch;

#[doc(inline)]
pub use clients::CosmosClient;
#[doc(inline)]
pub use clients::CosmosClientBuilder;

pub use account_endpoint::CosmosAccountEndpoint;
pub use account_reference::CosmosAccountReference;
pub use clients::ThroughputPoller;
pub use connection_string::*;
pub use credential::CosmosCredential;
pub use error::{
    CosmosError, CosmosErrorBuilder, CosmosStatus, CosmosStatusKind, Result, SubStatusCode,
};
pub use models::{
    BatchResponse, DiagnosticsContext, IncrValue, ItemResponse, PatchOp, PatchSpec,
    ResourceResponse, ResponseBody, ResponseHeaders,
};
pub use options::*;
pub use query::Query;
pub use routing_strategy::RoutingStrategy;
pub use transactional_batch::{
    BatchDeleteOptions, BatchReadOptions, BatchReplaceOptions, BatchUpsertOptions,
    TransactionalBatch, TransactionalBatchOperationResult, TransactionalBatchResponse,
};

// Driver re-exports
#[doc(inline)]
pub use azure_data_cosmos_driver::models::{
    ContinuationToken, EffectivePartitionKey, FeedRange, PartitionKey, PartitionKeyValue,
};

pub use feed::{FeedItemIterator, FeedPage, FeedPageIterator, QueryFeedPage};
mod driver_bridge;
#[cfg(feature = "fault_injection")]
pub mod fault_injection;
mod region_proximity;
pub mod regions;
mod routing_strategy;
