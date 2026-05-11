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
mod feed;
pub mod options;
pub(crate) mod pipeline;
pub mod query;
pub(crate) mod resource_context;
mod session_helpers;
pub(crate) mod utils;

pub mod models;
pub mod transactional_batch;

#[doc(inline)]
pub use clients::CosmosClient;
#[doc(inline)]
pub use clients::CosmosClientBuilder;

pub use account_endpoint::CosmosAccountEndpoint;
pub use account_reference::CosmosAccountReference;
#[doc(inline)]
pub use azure_data_cosmos_driver::models::ContinuationToken;
pub use azure_data_cosmos_driver::models::FeedRange;
#[doc(inline)]
pub use azure_data_cosmos_driver::models::PartitionKey;
#[doc(inline)]
pub use azure_data_cosmos_driver::models::PartitionKeyValue;
pub use clients::ThroughputPoller;
pub use connection_string::*;
pub use credential::CosmosCredential;
#[doc(inline)]
pub use hash::EffectivePartitionKey;
pub use models::{BatchResponse, CosmosDiagnostics, ItemResponse, ResourceResponse};
pub use options::*;
pub use query::Query;
pub use routing_strategy::RoutingStrategy;
pub use transactional_batch::{
    BatchDeleteOptions, BatchReadOptions, BatchReplaceOptions, BatchUpsertOptions,
    TransactionalBatch, TransactionalBatchOperationResult, TransactionalBatchResponse,
};

pub use feed::{FeedItemIterator, FeedPage, FeedPageIterator, QueryFeedPage};
mod background_task_manager;
mod cosmos_request;
mod driver_bridge;
#[cfg(feature = "fault_injection")]
pub mod fault_injection;
mod handler;
mod hash;
mod operation_context;
mod region_proximity;
pub mod regions;
mod request_context;
mod retry_policies;
mod routing;
mod routing_strategy;
mod serde;
