// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod clients;
mod connection_string;
pub mod constants;
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

pub use connection_string::*;
pub use models::CosmosResponse;
pub use options::*;
pub use partition_key::*;
pub use query::Query;
pub use transactional_batch::{
    TransactionalBatch, TransactionalBatchOperationResult, TransactionalBatchResponse,
};

pub use feed::{FeedPage, FeedPager};
mod cosmos_request;
mod handler;
mod operation_context;
pub mod regions;
mod request_context;
mod retry_policies;
mod routing;
mod serde;
