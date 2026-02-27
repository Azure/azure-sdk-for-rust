// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod account_endpoint;
mod account_reference;
mod availability_strategy;
pub mod clients;
mod connection_string;
pub mod constants;
mod credential;
mod feed;
pub mod options;
mod partition_key;
pub(crate) mod pipeline;
pub mod query;
pub(crate) mod resource_context;
pub(crate) mod utils;

pub mod models;
pub mod transactional_batch;

#[doc(inline)]
pub use clients::CosmosClient;
#[doc(inline)]
pub use clients::CosmosClientBuilder;

pub use account_endpoint::CosmosAccountEndpoint;
pub use account_reference::CosmosAccountReference;
pub use availability_strategy::{AvailabilityStrategy, CrossRegionHedgingStrategy};
pub use connection_string::*;
pub use credential::CosmosCredential;
pub use models::CosmosResponse;
pub use options::*;
pub use partition_key::*;
pub use query::Query;
pub use transactional_batch::{
    BatchDeleteOptions, BatchReadOptions, BatchReplaceOptions, BatchUpsertOptions,
    TransactionalBatch, TransactionalBatchOperationResult, TransactionalBatchResponse,
};

pub use feed::{FeedItemIterator, FeedPage, FeedPageIterator};
mod background_task_manager;
mod cosmos_request;
#[cfg(feature = "fault_injection")]
pub mod fault_injection;
mod handler;
mod hash;
mod murmur_hash;
mod operation_context;
mod region_proximity;
pub mod regions;
mod request_context;
mod retry_policies;
mod routing;
mod serde;
