// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cache infrastructure for Cosmos DB driver.
//!
//! This module provides async caching primitives with single-pending-I/O semantics:
//!
//! - [`AsyncLazy`] - Single-value lazy initialization
//! - [`AsyncCache`] - Key-value cache with per-key lazy initialization
//! - [`AccountMetadataCache`] - Cache for account metadata (regions, capabilities)
//! - [`ContainerCache`] - Cache for container metadata (partition key, indexing)
//! - [`PartitionKeyRangeCache`] - Cache for partition key ranges (routing)

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
