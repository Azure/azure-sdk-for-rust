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
mod feed_range;
pub mod options;
mod partition_key;
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
pub use error::{CosmosError, CosmosResult};
pub use models::{BatchResponse, CosmosDiagnosticsContext, ItemResponse, ResourceResponse};
pub use options::*;
pub use partition_key::*;
pub use query::Query;
pub use routing_strategy::RoutingStrategy;
pub use transactional_batch::{
    BatchDeleteOptions, BatchReadOptions, BatchReplaceOptions, BatchUpsertOptions,
    TransactionalBatch, TransactionalBatchOperationResult, TransactionalBatchResponse,
};

pub use feed::{FeedItemIterator, FeedPage, FeedPageIterator, QueryFeedPage};
pub use feed_range::FeedRange;
mod driver_bridge;
#[cfg(feature = "fault_injection")]
pub mod fault_injection;
mod hash;
mod murmur_hash;
mod region_proximity;
pub mod regions;
mod routing_strategy;
