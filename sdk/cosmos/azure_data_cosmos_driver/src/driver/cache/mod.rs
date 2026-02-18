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

mod account_metadata_cache;
mod async_cache;
mod async_lazy;
mod container_cache;

pub(crate) use account_metadata_cache::AccountMetadataCache;
pub(crate) use async_cache::AsyncCache;
#[allow(unused)]
pub(crate) use async_lazy::AsyncLazy;
pub(crate) use container_cache::ContainerCache;
