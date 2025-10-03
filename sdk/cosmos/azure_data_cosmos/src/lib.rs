// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod cache;
pub mod clients;
mod connection;
mod connection_string;
pub mod constants;
mod feed;
mod location_cache;
pub mod models;
mod options;
mod partition_key;
pub mod query;
mod resource_context;
mod types;
mod utils;

pub use types::ResourceId;

#[doc(inline)]
pub use clients::CosmosClient;

pub use connection_string::*;
pub use options::*;
pub use partition_key::*;
pub use query::Query;

pub use feed::{FeedPage, FeedPager};
