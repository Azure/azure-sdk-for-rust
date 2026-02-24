// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation trait and shared types for perf test operations.
//!
//! To add a new operation:
//! 1. Create a new file in this directory implementing [`Operation`].
//! 2. Register it in [`create_operations`].
//! 3. Add a CLI flag in `config.rs` to enable/disable it.

mod create_item;
mod query_items;
mod read_item;
mod upsert_item;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::options::ItemOptions;
use azure_data_cosmos::regions::RegionName;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::config::{Config, ExcludeRegionsScope};
pub use crate::operations::create_item::CreateItemOperation;
pub use crate::operations::query_items::QueryItemsOperation;
pub use crate::operations::read_item::ReadItemOperation;
pub use crate::operations::upsert_item::UpsertItemOperation;
use crate::seed::SharedItems;

/// A single executable perf test operation.
///
/// Implementations are expected to be stateless or use interior mutability.
/// They will be called concurrently from multiple tasks.
#[async_trait]
pub trait Operation: Send + Sync {
    /// Returns the display name of this operation (e.g., "ReadItem").
    fn name(&self) -> &'static str;

    /// Executes one instance of the operation.
    async fn execute(&self, container: &ContainerClient) -> azure_core::Result<()>;
}

/// The item type used for seeding, reading, querying, and upserting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfItem {
    pub id: String,
    pub partition_key: String,
    pub value: u64,
    pub payload: String,
}

/// Creates the list of enabled operations based on CLI configuration.
pub fn create_operations(
    config: &Config,
    seeded_items: Arc<SharedItems>,
) -> Vec<Arc<dyn Operation>> {
    let mut ops: Vec<Arc<dyn Operation>> = Vec::new();

    let (read_options, write_options) = build_item_options(config);

    if !config.no_reads {
        ops.push(Arc::new(ReadItemOperation::new(
            seeded_items.clone(),
            read_options.clone(),
        )));
    }
    if !config.no_queries {
        ops.push(Arc::new(QueryItemsOperation::new(seeded_items.clone())));
    }
    if !config.no_upserts {
        ops.push(Arc::new(UpsertItemOperation::new(
            seeded_items.clone(),
            write_options.clone(),
        )));
    }
    if !config.no_creates {
        ops.push(Arc::new(CreateItemOperation::new(
            seeded_items,
            write_options,
        )));
    }

    ops
}

/// Builds per-operation `ItemOptions` for reads and writes based on excluded regions config.
fn build_item_options(config: &Config) -> (Option<ItemOptions>, Option<ItemOptions>) {
    if config.excluded_regions.is_empty() {
        return (None, None);
    }

    let regions: Vec<RegionName> = config
        .excluded_regions
        .iter()
        .map(|r| r.clone().into())
        .collect();

    let opts = || Some(ItemOptions::default().with_excluded_regions(regions.clone()));

    match config.exclude_regions_for {
        ExcludeRegionsScope::Reads => (opts(), None),
        ExcludeRegionsScope::Writes => (None, opts()),
        ExcludeRegionsScope::Both => (opts(), opts()),
    }
}
