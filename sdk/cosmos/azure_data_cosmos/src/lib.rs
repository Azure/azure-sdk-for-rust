// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod clients;
mod connection_string;
pub mod constants;
mod feed;
mod options;
mod partition_key;
pub(crate) mod pipeline;
pub mod query;
pub(crate) mod resource_context;
pub(crate) mod utils;

pub mod models;

#[doc(inline)]
pub use clients::CosmosClient;

pub use connection_string::*;
pub use options::*;
pub use partition_key::*;
pub use query::Query;

pub use feed::{FeedPage, FeedPager};
mod handler;
mod retry_policies;
pub mod routing;
