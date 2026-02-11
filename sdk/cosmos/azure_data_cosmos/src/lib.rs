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

#[doc(inline)]
pub use clients::CosmosClient;

pub use connection_string::*;
pub use models::CosmosResponse;
pub use options::*;
pub use partition_key::*;
pub use query::Query;

pub use feed::{FeedItemIterator, FeedPage, FeedPageIterator};
mod background_task_manager;
mod cosmos_request;
#[cfg(feature = "fault_injection")]
pub mod fault_injection;
mod handler;
mod hash;
mod murmur_hash;
mod operation_context;
pub mod regions;
mod request_context;
mod retry_policies;
mod routing;
mod serde;

#[cfg(not(target_arch = "wasm32"))]
mod conditional_send {
    /// Conditionally implements [`Send`] based on the `target_arch`.
    ///
    /// This implementation requires `Send`.
    pub trait ConditionalSend: Send {}

    impl<T> ConditionalSend for T where T: Send {}
}

#[cfg(target_arch = "wasm32")]
mod conditional_send {
    /// Conditionally implements [`Send`] based on the `target_arch`.
    ///
    /// This implementation does not require `Send`.
    pub trait ConditionalSend {}

    impl<T> ConditionalSend for T {}
}
