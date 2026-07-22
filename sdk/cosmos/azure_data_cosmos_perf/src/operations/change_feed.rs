// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bounded per-feed-range change feed operation.

use std::{
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
    time::Duration,
};

use async_trait::async_trait;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::feed::FeedScope;
use azure_data_cosmos::options::ChangeFeedStartFrom;
use azure_data_cosmos::CosmosStatus;
use azure_data_cosmos_driver::error::CosmosError as DriverCosmosError;
use futures::StreamExt;

use super::{extract_backend_duration, FeedRangeCache, Operation, OperationResult, PerfItem};

/// Reads a bounded number of pages from one physical feed range.
pub struct ChangeFeedOperation {
    cache: FeedRangeCache,
    cursor: AtomicUsize,
    max_pages: usize,
}

impl ChangeFeedOperation {
    pub fn new(cache: FeedRangeCache, max_pages: usize) -> Self {
        Self {
            cache,
            cursor: AtomicUsize::new(0),
            max_pages,
        }
    }
}

#[async_trait]
impl Operation for ChangeFeedOperation {
    fn name(&self) -> &'static str {
        "ChangeFeed"
    }

    async fn execute(
        &self,
        container: &ContainerClient,
        capture_diagnostics: bool,
    ) -> azure_data_cosmos::Result<OperationResult> {
        let snapshot = {
            let guard = self.cache.read().expect("feed-range cache lock poisoned");
            Arc::clone(&guard)
        };
        if snapshot.is_empty() {
            return Err(DriverCosmosError::builder()
                .with_status(CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("feed-range cache is empty")
                .build()
                .into());
        }

        let idx = self.cursor.fetch_add(1, Ordering::Relaxed) % snapshot.len();
        let range = snapshot[idx].clone();
        let iterator = Box::pin(container.query_change_feed::<PerfItem>(
            FeedScope::range(range),
            ChangeFeedStartFrom::Beginning,
            None,
        ))
        .await?;
        let mut stream = Box::pin(iterator.take(self.max_pages));

        let mut backend_total: Option<Duration> = None;
        let mut diagnostics = Vec::new();
        while let Some(result) = stream.next().await {
            let page = result?;
            if let Some(duration) = extract_backend_duration(page.headers()) {
                backend_total = Some(backend_total.unwrap_or_default() + duration);
            }
            if capture_diagnostics {
                diagnostics.push(page.diagnostics());
            }
        }

        Ok(OperationResult::paged(backend_total, diagnostics))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use azure_data_cosmos::feed::FeedRange;

    use super::*;

    fn cache_with_len(len: usize) -> FeedRangeCache {
        let ranges: Vec<FeedRange> = (0..len).map(|_| FeedRange::full()).collect();
        Arc::new(RwLock::new(Arc::new(ranges)))
    }

    #[test]
    fn cursor_round_robins_across_ranges() {
        let operation = ChangeFeedOperation::new(cache_with_len(3), 4);
        let picks: Vec<usize> = (0..6)
            .map(|_| operation.cursor.fetch_add(1, Ordering::Relaxed) % 3)
            .collect();
        assert_eq!(picks, vec![0, 1, 2, 0, 1, 2]);
    }
}
