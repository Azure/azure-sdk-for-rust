// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation trait and shared types for perf test operations.
//!
//! To add a new operation:
//! 1. Create a new file in this directory implementing [`Operation`].
//! 2. Register it in [`create_operations`].
//! 3. Add a CLI flag in `config.rs` to enable/disable it.

mod create_item;
mod feed_range_query;
mod feed_range_refresher;
mod query_items;
mod read_item;
mod upsert_item;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::ResponseHeaders;
use azure_data_cosmos::options::Region;
use azure_data_cosmos::options::{
    ExcludedRegions, ItemReadOptions, ItemWriteOptions, OperationOptions,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::config::{Config, ExcludeRegionsScope};
pub use crate::operations::create_item::CreateItemOperation;
pub use crate::operations::feed_range_query::{FeedRangeCache, FeedRangeQueryOperation};
pub use crate::operations::feed_range_refresher::{FeedRangeRefresher, READ_FEED_RANGES_STAT};
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
    /// Returns `Ok(Some(d))` when the server reported a processing duration
    /// via the `x-ms-request-duration-ms` response header (this is the
    /// backend latency surfaced separately from the client-observed
    /// wall-clock latency). Returns `Ok(None)` when no backend duration
    /// could be observed (multi-page query streams may aggregate, see
    /// individual implementations).
    async fn execute(
        &self,
        container: &ContainerClient,
    ) -> azure_data_cosmos::Result<Option<Duration>>;
}

/// The item type used for seeding, reading, querying, and upserting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfItem {
    pub id: String,
    pub partition_key: String,
    pub value: u64,
    pub payload: String,
}

/// Bundle returned by [`create_operations`]: the per-worker operations plus
/// (optionally) a background feed-range refresher.
///
/// `feed_range_refresher` is `None` when the feed-range query op is
/// disabled (`--no-feed-range-queries`) or when the refresh interval is 0.
pub struct OperationsBundle {
    pub ops: Vec<Arc<dyn Operation>>,
    pub feed_range_refresher: Option<FeedRangeRefresher>,
}

/// Creates the list of enabled operations based on CLI configuration.
///
/// Asynchronous because the feed-range query op requires an initial
/// `read_feed_ranges` to seed its shared cache.
pub async fn create_operations(
    config: &Config,
    container: &ContainerClient,
    seeded_items: Arc<SharedItems>,
) -> azure_data_cosmos::Result<OperationsBundle> {
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

    let feed_range_refresher = if config.no_feed_range_queries {
        None
    } else {
        let initial = container.read_feed_ranges(None).await?;
        if initial.is_empty() {
            return Err(azure_data_cosmos_driver::error::CosmosError::builder()
                .with_status(azure_data_cosmos::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("read_feed_ranges returned empty list during seed")
                .build()
                .into());
        }
        let cache: FeedRangeCache = Arc::new(RwLock::new(Arc::new(initial)));
        ops.push(Arc::new(FeedRangeQueryOperation::new(cache.clone())));

        if config.feed_range_refresh_secs > 0 {
            Some(FeedRangeRefresher::new(
                container.clone(),
                cache,
                Duration::from_secs(config.feed_range_refresh_secs),
            ))
        } else {
            None
        }
    };

    Ok(OperationsBundle {
        ops,
        feed_range_refresher,
    })
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
