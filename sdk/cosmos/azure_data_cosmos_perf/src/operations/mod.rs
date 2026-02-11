// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation trait and shared types for perf test operations.
//!
//! To add a new operation:
//! 1. Create a new file in this directory implementing [`Operation`].
//! 2. Register it in [`create_operations`].
//! 3. Add a CLI flag in `config.rs` to enable/disable it.

mod query_items;
mod read_item;
mod upsert_item;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::config::Config;
pub use crate::operations::query_items::QueryItemsOperation;
pub use crate::operations::read_item::ReadItemOperation;
pub use crate::operations::upsert_item::UpsertItemOperation;

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
pub fn create_operations(config: &Config, seed_count: usize) -> Vec<Arc<dyn Operation>> {
    let mut ops: Vec<Arc<dyn Operation>> = Vec::new();

    if !config.no_reads {
        ops.push(Arc::new(ReadItemOperation::new(seed_count)));
    }
    if !config.no_queries {
        ops.push(Arc::new(QueryItemsOperation::new(seed_count)));
    }
    if !config.no_upserts {
        ops.push(Arc::new(UpsertItemOperation::new(seed_count)));
    }

    ops
}
