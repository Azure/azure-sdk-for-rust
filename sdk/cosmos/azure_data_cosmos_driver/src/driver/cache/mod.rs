// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cache infrastructure for Cosmos DB driver.
//!
//! This module provides partition key range caching via
//! [`ContainerRoutingMap`], which maps effective partition keys to their
//! owning physical partition key ranges.

mod account_metadata_cache;
mod async_cache;
mod async_lazy;
mod container_cache;
#[allow(dead_code)]
mod container_routing_map;
#[allow(dead_code)]
mod partition_key_range_cache;

pub(crate) use account_metadata_cache::{AccountMetadataCache, AccountProperties, AccountRegion};
pub(crate) use async_cache::AsyncCache;
pub(crate) use async_lazy::AsyncLazy;
pub(crate) use container_cache::ContainerCache;
pub use container_routing_map::ContainerRoutingMap;
pub(crate) use partition_key_range_cache::{
    parse_pk_ranges_response, PartitionKeyRangeCache, PkRangeFetchResult,
};
