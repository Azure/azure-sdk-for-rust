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
use azure_data_cosmos::models::DiagnosticsContext;
use azure_data_cosmos::options::{
    ExcludedRegions, ItemReadOptions, ItemWriteOptions, OperationOptions,
};
use azure_data_cosmos::regions::Region;
use azure_data_cosmos::ResponseHeaders;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

use crate::config::{Config, ExcludeRegionsScope};
pub use crate::operations::create_item::CreateItemOperation;
pub use crate::operations::query_items::QueryItemsOperation;
pub use crate::operations::read_item::ReadItemOperation;
pub use crate::operations::upsert_item::UpsertItemOperation;
use crate::seed::SharedItems;

/// Extracts the server-reported request duration from a Cosmos response.
///
/// Returns `None` when the header is missing (e.g., on responses served
/// from cache or when the gateway omitted the diagnostic header) or when
/// the value is infinite or negative.
pub(crate) fn extract_backend_duration(headers: &ResponseHeaders) -> Option<Duration> {
    headers
        .server_duration_ms()
        .map(|ms| Duration::from_secs_f64(ms / 1000.0))
}

/// Outcome of a successfully-executed perf operation.
///
/// Carries both the optional server-reported backend duration and the
/// finalized diagnostics context so the runner can route shard
/// observations into the perf binary's [`ShardObserver`].
pub struct OperationOutcome {
    pub backend_duration: Option<Duration>,
    pub diagnostics: Option<Arc<DiagnosticsContext>>,
}

impl OperationOutcome {
    pub fn new(backend_duration: Option<Duration>, diagnostics: Arc<DiagnosticsContext>) -> Self {
        Self {
            backend_duration,
            diagnostics: Some(diagnostics),
        }
    }
}

/// A single executable perf test operation.
///
/// Implementations are expected to be stateless or use interior mutability.
/// They will be called concurrently from multiple tasks.
#[async_trait]
pub trait Operation: Send + Sync {
    /// Returns the display name of this operation (e.g., "ReadItem").
    fn name(&self) -> &'static str;

    /// Executes one instance of the operation.
    ///
    /// Returns an [`OperationOutcome`] carrying the optional server-reported
    /// backend duration (from `x-ms-request-duration-ms`) and the finalized
    /// diagnostics context for the operation. Diagnostics is `None` only
    /// when no page was observed (e.g., empty query stream).
    async fn execute(
        &self,
        container: &ContainerClient,
    ) -> azure_data_cosmos::Result<OperationOutcome>;
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

/// Builds per-operation options for reads and writes based on excluded regions config.
fn build_item_options(config: &Config) -> (Option<ItemReadOptions>, Option<ItemWriteOptions>) {
    if config.excluded_regions.is_empty() {
        return (None, None);
    }

    let regions: Vec<Region> = config
        .excluded_regions
        .iter()
        .map(|r| r.clone().into())
        .collect();

    let mut operation = OperationOptions::default();
    operation.excluded_regions = Some(ExcludedRegions::from_iter(regions));

    let read_opts = || Some(ItemReadOptions::default().with_operation_options(operation.clone()));
    let write_opts = || Some(ItemWriteOptions::default().with_operation_options(operation.clone()));

    match config.exclude_regions_for {
        ExcludeRegionsScope::Reads => (read_opts(), None),
        ExcludeRegionsScope::Writes => (None, write_opts()),
        ExcludeRegionsScope::Both => (read_opts(), write_opts()),
    }
}
