// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-feed-range query operation.
//!
//! Each `execute()` runs `SELECT * FROM c` against ONE `FeedRange`,
//! round-robin'd from a cache shared with
//! [`FeedRangeRefresher`](super::feed_range_refresher::FeedRangeRefresher),
//! and drains every page so the harness exercises continuation-token
//! handling across multi-page responses per feed range.
//! The harness's existing worker pool provides concurrency — N workers
//! each issue one query at a time, naturally matching the concurrency
//! of every other operation.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::{query::FeedScope, CosmosStatus, FeedRange, Query};
use azure_data_cosmos_driver::error::CosmosError as DriverCosmosError;
use futures::StreamExt;

use super::{extract_backend_duration, Operation};

/// Shared, swappable feed-range cache.
///
/// The outer `RwLock` lets the background refresher atomically swap
/// the snapshot; the inner `Arc<Vec<FeedRange>>` lets readers clone
/// out cheaply while holding the read lock only briefly (no await).
pub type FeedRangeCache = Arc<RwLock<Arc<Vec<FeedRange>>>>;

pub struct FeedRangeQueryOperation {
    cache: FeedRangeCache,
    cursor: AtomicUsize,
}

impl FeedRangeQueryOperation {
    pub fn new(cache: FeedRangeCache) -> Self {
        Self {
            cache,
            cursor: AtomicUsize::new(0),
        }
    }
}

#[async_trait]
impl Operation for FeedRangeQueryOperation {
    fn name(&self) -> &'static str {
        "FeedRangeQuery"
    }

    async fn execute(
        &self,
        container: &ContainerClient,
    ) -> azure_data_cosmos::Result<Option<Duration>> {
        // Snapshot the current ranges; release the lock immediately.
        let snapshot = {
            let guard = self.cache.read().expect("feed-range cache lock poisoned");
            Arc::clone(&guard)
        };

        if snapshot.is_empty() {
            // Should be impossible after a successful seed; surface as a
            // typed error rather than panicking so the worker records it.
            return Err(DriverCosmosError::builder()
                .with_status(CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("feed-range cache is empty")
                .build()
                .into());
        }

        let idx = self.cursor.fetch_add(1, Ordering::Relaxed) % snapshot.len();
        let fr = snapshot[idx].clone();

        let query = Query::from("SELECT * FROM c");
        // Box::pin keeps this future off the worker's stack — clippy's
        // `large_futures` (-D in CI) fires on the un-pinned form (~16 KB).
        let mut stream = Box::pin(
            container
                .query_items::<super::PerfItem>(query, FeedScope::range(fr), None)
                .await?
                .into_pages(),
        );

        // Sum backend durations across pages so a multi-page response
        // reports the total server processing time, matching the
        // client-observed elapsed which wraps the entire stream drain.
        let mut backend_total: Option<Duration> = None;
        while let Some(result) = stream.next().await {
            let page = result?;
            if let Some(d) = extract_backend_duration(page.headers()) {
                backend_total = Some(backend_total.unwrap_or_default() + d);
            }
        }

        Ok(backend_total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cache_with_len(n: usize) -> FeedRangeCache {
        let ranges: Vec<FeedRange> = (0..n).map(|_| FeedRange::full()).collect();
        Arc::new(RwLock::new(Arc::new(ranges)))
    }

    #[test]
    fn cursor_round_robins_and_wraps() {
        let op = FeedRangeQueryOperation::new(cache_with_len(3));
        // 6 fetches over a 3-element cache should visit each index twice
        // (0,1,2,0,1,2 modulo 3).
        let picks: Vec<usize> = (0..6)
            .map(|_| op.cursor.fetch_add(1, Ordering::Relaxed) % 3)
            .collect();
        assert_eq!(picks, vec![0, 1, 2, 0, 1, 2]);
    }

    #[test]
    fn empty_cache_does_not_panic_on_snapshot() {
        // Construct against a zero-length cache and confirm snapshot
        // is empty (execute() would surface this as an error instead
        // of dividing by zero).
        let op = FeedRangeQueryOperation::new(cache_with_len(0));
        let guard = op.cache.read().unwrap();
        assert!(guard.is_empty());
    }
}
